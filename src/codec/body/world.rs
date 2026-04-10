use crate::internal::error::ParseSection;
use crate::internal::reader::Reader;
use crate::model::{
    DirectionSet, LayerType, ObjectPart, PlayerColor, PlayerColorsSet, Tile, World,
};

pub(crate) fn decode(bytes: &[u8]) -> std::result::Result<World, crate::Error> {
    let mut reader = Reader::with_context(bytes, ParseSection::World);
    decode_world(&mut reader)
}

fn decode_world(reader: &mut Reader<'_>) -> std::result::Result<World, crate::Error> {
    reader.set_section(ParseSection::World);
    let width: i32 = reader.read_i32_be("world width")?;
    let height: i32 = reader.read_i32_be("world height")?;
    let tiles_count: u32 = reader.read_u32_be("world tiles count")?;
    let mut tiles: Vec<Tile> = Vec::new();
    for _ in 0..tiles_count {
        tiles.push(decode_tile(reader)?);
    }
    Ok(World {
        width,
        height,
        tiles,
    })
}

fn decode_tile(reader: &mut Reader<'_>) -> std::result::Result<Tile, crate::Error> {
    let index: i32 = reader.read_i32_be("tile index")?;
    let terrain_image_index: u16 = reader.read_u16_be("tile terrain image index")?;
    let terrain_flags: u8 = reader.read_u8("tile terrain flags")?;
    let tile_passability_directions =
        DirectionSet::from_bits(reader.read_u16_be("tile passability directions")?);
    let main_object_part = decode_object_part(reader)?;
    let main_object_type: u16 = reader.read_u16_be("tile main object type")?;
    let fog_colors: PlayerColorsSet =
        PlayerColorsSet::from_bits(reader.read_u8("tile fog colors")?);
    let metadata_count = reader.read_u32_be("tile metadata count")?;
    let mut metadata = Vec::with_capacity(usize::try_from(metadata_count).unwrap());
    for _ in 0..metadata_count {
        metadata.push(reader.read_u32_be("tile metadata")?);
    }
    let occupant_hero_id: u8 = reader.read_u8("tile occupant hero id")?;
    let is_tile_marked_as_road: bool = reader.read_byte_as_bool("tile is marked as road")?;
    let ground_object_parts_count = reader.read_u32_be("tile ground object parts count")?;
    let mut ground_object_parts =
        Vec::with_capacity(usize::try_from(ground_object_parts_count).unwrap());
    for _ in 0..ground_object_parts_count {
        ground_object_parts.push(decode_object_part(reader)?);
    }
    let top_object_parts_count = reader.read_u32_be("tile top object parts count")?;
    let mut top_object_parts = Vec::with_capacity(usize::try_from(top_object_parts_count).unwrap());
    for _ in 0..top_object_parts_count {
        top_object_parts.push(decode_object_part(reader)?);
    }
    let boat_owner_color: PlayerColor =
        PlayerColor::from_bits(reader.read_u8("tile boat owner color")?);

    Ok(Tile {
        index,
        terrain_image_index,
        terrain_flags,
        tile_passability_directions,
        main_object_part,
        main_object_type,
        fog_colors,
        metadata,
        occupant_hero_id,
        is_tile_marked_as_road,
        ground_object_parts,
        top_object_parts,
        boat_owner_color,
    })
}

fn decode_object_part(reader: &mut Reader<'_>) -> std::result::Result<ObjectPart, crate::Error> {
    let layer_type = LayerType::from_byte(reader.read_u8("object part layer type")?);
    let uid = reader.read_u32_be("object part uid")?;
    let icn_type = reader.read_u8("object part icn type")?;
    let icn_index = reader.read_u8("object part icn index")?;

    Ok(ObjectPart {
        layer_type,
        uid,
        icn_type,
        icn_index,
    })
}
