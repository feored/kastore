use crate::Error;
use crate::codec::body::world::heroes::{
    decode_army, decode_hero_base, encode_army, encode_hero_base,
};
use crate::internal::reader::Reader;
use crate::internal::writer::Writer;
use crate::model::CastleDwellings;
use crate::model::MageGuild;
use crate::model::Spell;
use crate::model::{
    Army, Castle, CastleBuildingSet, CastleModeSet, HeroBase, MapPosition, PlayerColor, Race,
};

pub(super) fn decode(reader: &mut Reader<'_>) -> std::result::Result<Vec<Castle>, Error> {
    let count = reader.read_u32_be("castles count")?;
    let mut castles = Vec::with_capacity(usize::try_from(count).unwrap_or(0));
    for _ in 0..count {
        castles.push(decode_castle(reader)?);
    }
    Ok(castles)
}

pub(super) fn encode(writer: &mut Writer, castles: &[Castle]) -> std::result::Result<(), Error> {
    writer.write_u32_be(
        u32::try_from(castles.len()).map_err(|_| Error::InvalidModel {
            field: "world castles",
            message: "castle count must fit in u32",
        })?,
    );

    for castle in castles {
        encode_castle(writer, castle)?;
    }

    Ok(())
}

fn decode_castle(reader: &mut Reader<'_>) -> std::result::Result<Castle, Error> {
    let map_position = MapPosition {
        x: reader.read_i16_be("castle map position x")?,
        y: reader.read_i16_be("castle map position y")?,
    };
    let castle_mode: CastleModeSet = CastleModeSet::from_bits(reader.read_u32_be("castle mode")?);
    let race = Race::from_i32(reader.read_i32_be("castle race")?);
    let constructed_buildings =
        CastleBuildingSet::from_mask(reader.read_u32_be("castle constructed buildings")?);
    let disabled_buildings =
        CastleBuildingSet::from_mask(reader.read_u32_be("castle disabled buildings")?);
    let captain: HeroBase = decode_hero_base(reader)?;
    let color_base = PlayerColor::from_bits(reader.read_u8("castle color base")?);
    let name = reader.read_save_string("castle name")?;
    let mage_guild_normal_spells_count =
        reader.read_u32_be("castle mage guild normal spells count")?;
    let mut mage_guild_normal_spells =
        Vec::with_capacity(usize::try_from(mage_guild_normal_spells_count).unwrap_or(0));
    for _ in 0..mage_guild_normal_spells_count {
        mage_guild_normal_spells.push(Spell::from_i32(
            reader.read_i32_be("castle mage guild normal spell")?,
        ));
    }
    let mage_guild_library_spells_count =
        reader.read_u32_be("castle mage guild library spells count")?;
    let mut mage_guild_library_spells =
        Vec::with_capacity(usize::try_from(mage_guild_library_spells_count).unwrap_or(0));
    for _ in 0..mage_guild_library_spells_count {
        mage_guild_library_spells.push(Spell::from_i32(
            reader.read_i32_be("castle mage guild library spell")?,
        ));
    }
    let mage_guild_spells = MageGuild {
        spells: mage_guild_normal_spells,
        library_spells: mage_guild_library_spells,
    };

    let dwellings_count_position = reader.position();
    let dwellings_count = reader.read_u32_be("castle dwellings count")?;
    if dwellings_count != CastleDwellings::COUNT as u32 {
        return Err(reader.invalid_value(
            "castle dwellings count",
            dwellings_count_position,
            "unexpected dwellings size, expected 6",
        ));
    }
    let dwellings = CastleDwellings::from_counts([
        reader.read_u32_be("castle dwelling tier 1")?,
        reader.read_u32_be("castle dwelling tier 2")?,
        reader.read_u32_be("castle dwelling tier 3")?,
        reader.read_u32_be("castle dwelling tier 4")?,
        reader.read_u32_be("castle dwelling tier 5")?,
        reader.read_u32_be("castle dwelling tier 6")?,
    ]);
    let army: Army = decode_army(reader)?;
    Ok(Castle {
        map_position,
        mode: castle_mode,
        race,
        constructed_buildings,
        disabled_buildings,
        color_base,
        captain,
        name,
        mage_guild_spells,
        dwellings,
        army,
    })
}

fn encode_castle(writer: &mut Writer, castle: &Castle) -> std::result::Result<(), Error> {
    writer.write_i16_be(castle.map_position.x);
    writer.write_i16_be(castle.map_position.y);
    writer.write_u32_be(castle.mode.bits());
    writer.write_i32_be(castle.race.to_i32());
    writer.write_u32_be(castle.constructed_buildings.to_mask());
    writer.write_u32_be(castle.disabled_buildings.to_mask());
    encode_hero_base(writer, &castle.captain)?;
    writer.write_u8(castle.color_base.bits());
    writer.write_save_string(&castle.name);
    writer.write_u32_be(
        u32::try_from(castle.mage_guild_spells.spells.len()).map_err(|_| Error::InvalidModel {
            field: "castle mage guild spells",
            message: "spell count must fit in u32",
        })?,
    );
    for spell in &castle.mage_guild_spells.spells {
        writer.write_i32_be(spell.to_i32());
    }
    writer.write_u32_be(
        u32::try_from(castle.mage_guild_spells.library_spells.len()).map_err(|_| {
            Error::InvalidModel {
                field: "castle mage guild library spells",
                message: "spell count must fit in u32",
            }
        })?,
    );
    for spell in &castle.mage_guild_spells.library_spells {
        writer.write_i32_be(spell.to_i32());
    }
    writer.write_u32_be(CastleDwellings::COUNT as u32);
    for count in castle.dwellings.counts() {
        writer.write_u32_be(count);
    }
    encode_army(writer, &castle.army)?;

    Ok(())
}
