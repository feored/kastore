mod castles;
mod heroes;
mod tile;

use crate::Error;
use crate::internal::error::ParseSection;
use crate::internal::reader::Reader;
use crate::internal::writer::Writer;
use crate::model::world::World;
use crate::model::world::tile::Tile;

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
    Ok((
        World {
            width,
            height,
            tiles,
            heroes,
            castles,
        },
        reader.position(),
    ))
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

    Ok(writer.into_bytes())
}

#[cfg(test)]
mod tests;
