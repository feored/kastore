mod captured_objects;
mod castles;
mod custom_rumors;
mod heroes;
mod kingdoms;
mod tile;
mod timed_events;
mod ultimate_artifact;
mod validation;

use crate::Error;
use crate::codec::world_date::{decode_world_date, encode_world_date};
use crate::internal::error::ParseSection;
use crate::internal::reader::Reader;
use crate::internal::writer::Writer;
use crate::model::world::World;
use crate::model::world::heroes::id::HeroID;
use crate::model::world::tile::Tile;
use validation::validate_kingdoms;

pub(crate) fn decode(bytes: &[u8]) -> std::result::Result<World, Error> {
    Ok(decode_with_remaining_offset(bytes)?.0)
}

pub(crate) fn decode_with_remaining_offset(
    bytes: &[u8],
) -> std::result::Result<(World, usize), Error> {
    let mut reader = Reader::with_context(bytes, ParseSection::World);
    let width: i32 = reader.read_i32_be("world width")?;
    let height: i32 = reader.read_i32_be("world height")?;
    let tiles_count: u32 = reader.read_u32_be("world tiles count")?;
    let mut tiles: Vec<Tile> = Vec::new();
    for _ in 0..tiles_count {
        tiles.push(tile::decode(&mut reader)?);
    }
    let heroes = heroes::decode(&mut reader)?;
    let castles = castles::decode(&mut reader)?;
    let kingdoms_offset = reader.position();
    let kingdoms = kingdoms::decode(&mut reader)?;
    let custom_rumors = custom_rumors::decode(&mut reader)?;
    let timed_events = timed_events::decode(&mut reader)?;
    let captured_objects = captured_objects::decode(&mut reader)?;
    let ultimate_artifact = ultimate_artifact::decode(&mut reader)?;
    let world_date = decode_world_date(&mut reader)?;
    let hero_id_as_win_condition =
        HeroID::from_i32(reader.read_i32_be("hero id as win condition")?);
    let hero_id_as_lose_condition =
        HeroID::from_i32(reader.read_i32_be("hero id as loss condition")?);
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
    };
    validate_kingdoms(&world)
        .map_err(|issue| reader.invalid_value(issue.field, kingdoms_offset, issue.message))?;

    Ok((world, reader.position()))
}

pub(crate) fn encode(world: &World) -> std::result::Result<Vec<u8>, Error> {
    let mut writer = Writer::new();
    writer.write_i32_be(world.width);
    writer.write_i32_be(world.height);
    writer.write_u32_be(
        u32::try_from(world.tiles.len()).map_err(|_| Error::InvalidModel {
            field: "world tiles",
            message: "tile count must fit in u32",
        })?,
    );

    for tile in &world.tiles {
        tile::encode(&mut writer, tile)?;
    }

    heroes::encode(&mut writer, &world.heroes)?;
    castles::encode(&mut writer, &world.castles)?;
    validate_kingdoms(world).map_err(|issue| Error::InvalidModel {
        field: issue.field,
        message: issue.message,
    })?;
    kingdoms::encode(&mut writer, &world.kingdoms)?;
    custom_rumors::encode(&mut writer, &world.custom_rumors)?;
    timed_events::encode(&mut writer, &world.timed_events)?;
    captured_objects::encode(&mut writer, &world.captured_objects)?;
    ultimate_artifact::encode(&mut writer, &world.ultimate_artifact)?;
    encode_world_date(&mut writer, world.world_date);
    writer.write_i32_be(world.hero_id_as_win_condition.to_i32());
    writer.write_i32_be(world.hero_id_as_lose_condition.to_i32());

    Ok(writer.into_bytes())
}

#[cfg(test)]
mod tests;
