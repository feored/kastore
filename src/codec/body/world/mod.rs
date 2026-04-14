mod captured_objects;
mod castles;
mod custom_rumors;
mod heroes;
mod kingdoms;
mod map_objects;
mod tile;
mod timed_events;
mod ultimate_artifact;
mod validation;

use crate::Error;
use crate::codec::world_date::{decode_world_date, encode_world_date};
use crate::internal::error::ParseSection;
use crate::internal::reader::Reader;
use crate::internal::writer::Writer;
use crate::model::world::Funds;
use crate::model::world::World;
use crate::model::world::heroes::id::HeroID;
use crate::model::world::tile::Tile;
use validation::validate_kingdoms;

pub(crate) fn decode(reader: &mut Reader<'_>) -> std::result::Result<World, Error> {
    reader.set_section(ParseSection::World);
    let width: i32 = reader.read_i32_be("world width")?;
    let height: i32 = reader.read_i32_be("world height")?;
    let tiles_count: u32 = reader.read_u32_be("world tiles count")?;
    let mut tiles: Vec<Tile> = Vec::new();
    for _ in 0..tiles_count {
        tiles.push(tile::decode(reader)?);
    }
    let heroes = heroes::decode(reader)?;
    let castles = castles::decode(reader)?;
    let kingdoms_offset = reader.position();
    let kingdoms = kingdoms::decode(reader)?;
    let custom_rumors = custom_rumors::decode(reader)?;
    let timed_events = timed_events::decode(reader)?;
    let captured_objects = captured_objects::decode(reader)?;
    let ultimate_artifact = ultimate_artifact::decode(reader)?;
    let world_date = decode_world_date(reader)?;
    let hero_id_as_win_condition =
        HeroID::from_i32(reader.read_i32_be("hero id as win condition")?);
    let hero_id_as_lose_condition =
        HeroID::from_i32(reader.read_i32_be("hero id as loss condition")?);
    let map_objects = map_objects::decode(reader)?;
    let seed = reader.read_u32_be("world seed")?;
    let world = World {
        width,
        height,
        tiles,
        heroes,
        castles,
        kingdoms,
        custom_rumors,
        timed_events,
        captured_objects,
        ultimate_artifact,
        world_date,
        hero_id_as_win_condition,
        hero_id_as_lose_condition,
        map_objects,
        seed,
    };
    validate_kingdoms(&world)
        .map_err(|issue| reader.invalid_value(issue.field, kingdoms_offset, issue.message))?;

    Ok(world)
}

pub(crate) fn encode_into(writer: &mut Writer, world: &World) -> std::result::Result<(), Error> {
    writer.write_i32_be(world.width);
    writer.write_i32_be(world.height);
    writer.write_u32_be(
        u32::try_from(world.tiles.len()).map_err(|_| Error::InvalidModel {
            field: "world tiles",
            message: "tile count must fit in u32",
        })?,
    );

    for tile in &world.tiles {
        tile::encode(writer, tile)?;
    }

    heroes::encode(writer, &world.heroes)?;
    castles::encode(writer, &world.castles)?;
    validate_kingdoms(world).map_err(|issue| Error::InvalidModel {
        field: issue.field,
        message: issue.message,
    })?;
    kingdoms::encode(writer, &world.kingdoms)?;
    custom_rumors::encode(writer, &world.custom_rumors)?;
    timed_events::encode(writer, &world.timed_events)?;
    captured_objects::encode(writer, &world.captured_objects)?;
    ultimate_artifact::encode(writer, &world.ultimate_artifact)?;
    encode_world_date(writer, world.world_date);
    writer.write_i32_be(world.hero_id_as_win_condition.to_i32());
    writer.write_i32_be(world.hero_id_as_lose_condition.to_i32());
    map_objects::encode(writer, &world.map_objects)?;
    writer.write_u32_be(world.seed);

    Ok(())
}

pub(super) fn decode_funds(reader: &mut Reader<'_>) -> std::result::Result<Funds, Error> {
    Ok(Funds {
        wood: reader.read_i32_be("funds wood")?,
        mercury: reader.read_i32_be("funds mercury")?,
        ore: reader.read_i32_be("funds ore")?,
        sulfur: reader.read_i32_be("funds sulfur")?,
        crystal: reader.read_i32_be("funds crystal")?,
        gems: reader.read_i32_be("funds gems")?,
        gold: reader.read_i32_be("funds gold")?,
    })
}

pub(super) fn encode_funds(writer: &mut Writer, funds: &Funds) {
    writer.write_i32_be(funds.wood);
    writer.write_i32_be(funds.mercury);
    writer.write_i32_be(funds.ore);
    writer.write_i32_be(funds.sulfur);
    writer.write_i32_be(funds.crystal);
    writer.write_i32_be(funds.gems);
    writer.write_i32_be(funds.gold);
}

#[cfg(test)]
mod tests;
