mod heroes;
mod tile;

use crate::Error;
use crate::internal::error::ParseSection;
use crate::internal::reader::Reader;
use crate::internal::writer::Writer;
use crate::model::{Tile, World};

const SERIALIZED_HERO_SLOTS: u32 = 73;

pub(crate) fn decode(bytes: &[u8]) -> std::result::Result<World, Error> {
    let mut reader = Reader::with_context(bytes, ParseSection::World);
    let width: i32 = reader.read_i32_be("world width")?;
    let height: i32 = reader.read_i32_be("world height")?;
    let tiles_count: u32 = reader.read_u32_be("world tiles count")?;
    let mut tiles: Vec<Tile> = Vec::new();
    for _ in 0..tiles_count {
        tiles.push(tile::decode(&mut reader)?);
    }
    let heroes = heroes::decode(&mut reader)?;
    Ok(World {
        width,
        height,
        tiles,
        heroes,
    })
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

    if !world.heroes.is_empty() {
        return Err(Error::NotImplemented {
            feature: "world hero encoding",
        });
    }

    writer.write_u32_be(SERIALIZED_HERO_SLOTS);
    for _ in 0..SERIALIZED_HERO_SLOTS {
        encode_placeholder_hero(&mut writer);
    }

    Ok(writer.into_bytes())
}

fn encode_placeholder_hero(writer: &mut Writer) {
    writer.write_i32_be(0);
    writer.write_i32_be(0);
    writer.write_i32_be(0);
    writer.write_i32_be(0);
    writer.write_u16_be(0);
    writer.write_u16_be(0);
    writer.write_u32_be(0);
    writer.write_u32_be(0);
    writer.write_u32_be(0);
    writer.write_u32_be(0);
    writer.write_u32_be(0);
    writer.write_u32_be(0);
    writer.write_u8(0);
    writer.write_u32_be(0);
    writer.write_u32_be(0);
    writer.write_u32_be(5);
    for _ in 0..5 {
        writer.write_i32_be(0);
        writer.write_u32_be(0);
    }
    writer.write_u8(0);
    writer.write_u8(0);
    writer.write_i32_be(0);
    writer.write_i32_be(0);
    writer.write_i32_be(0);
    writer.write_u16_be(0);
    writer.write_u8(0);
    writer.write_u32_be(0);
    writer.write_i32_be(0);
    writer.write_i32_be(0);
    writer.write_i32_be(0);
    writer.write_u32_be(0);
    writer.write_u32_be(0);
    writer.write_u32_be(0);
    writer.write_u32_be(0);
}

#[cfg(test)]
mod tests;
