use super::*;
use crate::internal::writer::Writer;
use crate::model::{
    DirectionSet, Hero, LayerType, ObjectPart, PlayerColor, PlayerColorsSet, Tile,
};

fn sample_tile() -> Tile {
    Tile {
        index: -7,
        terrain_image_index: 0x1234,
        terrain_flags: 0xA5,
        tile_passability_directions: DirectionSet::from_bits(
            DirectionSet::TOP.bits() | DirectionSet::RIGHT.bits(),
        ),
        main_object_part: ObjectPart {
            layer_type: LayerType::TerrainLayer,
            uid: 0x0102_0304,
            icn_type: 45,
            icn_index: 7,
        },
        main_object_type: 0x00A3,
        fog_colors: PlayerColorsSet::from_bits(0x21),
        metadata: vec![0xDEAD_BEEF, 5],
        occupant_hero_id: 9,
        is_tile_marked_as_road: true,
        ground_object_parts: vec![
            ObjectPart {
                layer_type: LayerType::ObjectLayer,
                uid: 0x1111_1111,
                icn_type: 12,
                icn_index: 1,
            },
            ObjectPart {
                layer_type: LayerType::BackgroundLayer,
                uid: 0x2222_2222,
                icn_type: 29,
                icn_index: 8,
            },
        ],
        top_object_parts: vec![ObjectPart {
            layer_type: LayerType::ShadowLayer,
            uid: 0x3333_3333,
            icn_type: 14,
            icn_index: 2,
        }],
        boat_owner_color: PlayerColor::Red,
    }
}

fn world_bytes_with_placeholder_heroes(width: i32, height: i32, tiles: &[Tile]) -> Vec<u8> {
    let mut writer = Writer::new();
    writer.write_i32_be(width);
    writer.write_i32_be(height);
    writer.write_u32_be(u32::try_from(tiles.len()).unwrap());
    for tile in tiles {
        tile::encode(&mut writer, tile).unwrap();
    }
    writer.write_u32_be(SERIALIZED_HERO_SLOTS);
    for _ in 0..SERIALIZED_HERO_SLOTS {
        encode_placeholder_hero(&mut writer);
    }
    writer.into_bytes()
}

#[test]
fn decode_world_reads_tiles_and_filters_placeholder_heroes() {
    let tile = sample_tile();
    let bytes = world_bytes_with_placeholder_heroes(2, 1, std::slice::from_ref(&tile));

    let world = decode(&bytes).unwrap();

    assert_eq!(world.width, 2);
    assert_eq!(world.height, 1);
    assert_eq!(world.tiles, vec![tile]);
    assert!(world.heroes.is_empty());
}

#[test]
fn encode_world_rejects_non_empty_semantic_heroes() {
    let world = World {
        width: 0,
        height: 0,
        tiles: Vec::new(),
        heroes: vec![Hero::default()],
    };

    assert_eq!(
        encode(&world),
        Err(Error::NotImplemented {
            feature: "world hero encoding",
        })
    );
}

#[test]
fn encode_world_round_trips_empty_semantic_world() {
    let world = World {
        width: 0,
        height: 0,
        tiles: Vec::new(),
        heroes: Vec::new(),
    };

    let encoded = encode(&world).unwrap();
    let decoded = decode(&encoded).unwrap();

    assert_eq!(decoded, world);
}
