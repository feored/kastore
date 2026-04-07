use crate::SaveString;
use crate::internal::error::ParseSection;
use crate::internal::reader::Reader;
use crate::internal::writer::Writer;
use crate::model::{
    Difficulty, GameVersion, LossConditionData, LossConditionKind, MapInfo, PlayerColorsSet,
    PlayerSlotInfo, Race, SupportedLanguage, VictoryConditionData, VictoryConditionKind, WorldDate,
};
use crate::version::MapInfoRevision;

pub(crate) fn decode(
    reader: &mut Reader<'_>,
    revision: MapInfoRevision,
) -> std::result::Result<MapInfo, crate::Error> {
    reader.set_section(ParseSection::MapInfo);

    let filename = SaveString::from_bytes(reader.read_string_bytes("map filename")?);
    let name = SaveString::from_bytes(reader.read_string_bytes("map name")?);
    let description = SaveString::from_bytes(reader.read_string_bytes("map description")?);
    let width = reader.read_u16_be("map width")?;
    let height = reader.read_u16_be("map height")?;
    let difficulty = Difficulty::from_byte(reader.read_u8("map difficulty")?);
    let player_entry_count = reader.read_u8("player entry count")?;
    let mut player_slots = Vec::with_capacity(usize::from(player_entry_count));

    for _ in 0..player_entry_count {
        player_slots.push(PlayerSlotInfo {
            race: Race::from_byte(reader.read_u8("player race")?),
            allies: PlayerColorsSet::from_bits(reader.read_u8("player allies")?),
        });
    }

    let kingdom_colors = PlayerColorsSet::from_bits(reader.read_u8("kingdom colors")?);
    let colors_available_for_humans =
        PlayerColorsSet::from_bits(reader.read_u8("colors available for humans")?);
    let colors_available_for_comp =
        PlayerColorsSet::from_bits(reader.read_u8("colors available for computer")?);
    let colors_of_random_races =
        PlayerColorsSet::from_bits(reader.read_u8("colors of random races")?);
    let victory_condition = VictoryConditionData {
        kind: VictoryConditionKind::from_byte(reader.read_u8("victory condition type")?),
        comp_also_wins: reader.read_byte_as_bool("computer also wins")?,
        allow_normal_victory: reader.read_byte_as_bool("allow normal victory")?,
        params: [
            reader.read_u16_be("victory condition param 0")?,
            reader.read_u16_be("victory condition param 1")?,
        ],
    };
    let loss_condition = LossConditionData {
        kind: LossConditionKind::from_byte(reader.read_u8("loss condition type")?),
        params: [
            reader.read_u16_be("loss condition param 0")?,
            reader.read_u16_be("loss condition param 1")?,
        ],
    };
    let timestamp = reader.read_u32_be("timestamp")?;
    let start_with_hero_in_first_castle =
        reader.read_byte_as_bool("start with hero in first castle")?;
    let version = GameVersion::from_u32(reader.read_u32_be("game version")?);
    let world_date = WorldDate {
        day: reader.read_u32_be("world date day")?,
        week: reader.read_u32_be("world date week")?,
        month: reader.read_u32_be("world date month")?,
    };
    let main_language = SupportedLanguage::from(reader.read_u8("main language")?);
    let creator_notes = match revision {
        MapInfoRevision::V10024 => None,
        MapInfoRevision::V10033 => Some(SaveString::from_bytes(
            reader.read_string_bytes("creator notes")?,
        )),
    };

    Ok(MapInfo {
        filename,
        name,
        description,
        width,
        height,
        difficulty,
        player_slots,
        kingdom_colors,
        colors_available_for_humans,
        colors_available_for_comp,
        colors_of_random_races,
        victory_condition,
        loss_condition,
        timestamp,
        start_with_hero_in_first_castle,
        version,
        world_date,
        main_language,
        creator_notes,
    })
}

pub(crate) fn encode(
    writer: &mut Writer,
    file_info: &MapInfo,
    revision: MapInfoRevision,
) -> std::result::Result<(), crate::Error> {
    writer.write_save_string(&file_info.filename);
    writer.write_save_string(&file_info.name);
    writer.write_save_string(&file_info.description);
    writer.write_u16_be(file_info.width);
    writer.write_u16_be(file_info.height);
    writer.write_u8(file_info.difficulty.to_byte());
    let player_slot_count =
        u8::try_from(file_info.player_slots.len()).map_err(|_| crate::Error::InvalidModel {
            field: "player slots",
            message: "player slot count must fit in u8",
        })?;
    writer.write_u8(player_slot_count);

    for slot in &file_info.player_slots {
        writer.write_u8(slot.race.to_byte());
        writer.write_u8(slot.allies.bits());
    }

    writer.write_u8(file_info.kingdom_colors.bits());
    writer.write_u8(file_info.colors_available_for_humans.bits());
    writer.write_u8(file_info.colors_available_for_comp.bits());
    writer.write_u8(file_info.colors_of_random_races.bits());
    writer.write_u8(file_info.victory_condition.kind.to_byte());
    writer.write_byte_from_bool(file_info.victory_condition.comp_also_wins);
    writer.write_byte_from_bool(file_info.victory_condition.allow_normal_victory);
    writer.write_u16_be(file_info.victory_condition.params[0]);
    writer.write_u16_be(file_info.victory_condition.params[1]);
    writer.write_u8(file_info.loss_condition.kind.to_byte());
    writer.write_u16_be(file_info.loss_condition.params[0]);
    writer.write_u16_be(file_info.loss_condition.params[1]);
    writer.write_u32_be(file_info.timestamp);
    writer.write_byte_from_bool(file_info.start_with_hero_in_first_castle);
    writer.write_u32_be(file_info.version.to_u32());
    writer.write_u32_be(file_info.world_date.day);
    writer.write_u32_be(file_info.world_date.week);
    writer.write_u32_be(file_info.world_date.month);
    writer.write_u8(u8::from(file_info.main_language));

    if revision == MapInfoRevision::V10033 {
        match &file_info.creator_notes {
            Some(creator_notes) => writer.write_save_string(creator_notes),
            None => writer.write_u32_be(0),
        }
    }

    Ok(())
}
