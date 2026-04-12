use super::*;
use crate::SaveString;
use crate::internal::writer::Writer;
use crate::model::header::player::{PlayerColor, PlayerColorsSet, Race};
use crate::model::world::castles::buildings::{CastleBuilding, CastleBuildingSet, CastleDwellings};
use crate::model::world::castles::{Castle, CastleModeSet, MageGuild};
use crate::model::world::heroes::army::{Army, MonsterType, Troop};
use crate::model::world::heroes::artifact::{Artifact, ArtifactID};
use crate::model::world::heroes::id::HeroID;
use crate::model::world::heroes::modes::HeroModeSet;
use crate::model::world::heroes::path::{Direction, Path, RouteStep};
use crate::model::world::heroes::skills::{SecondarySkill, Skill, SkillLevel};
use crate::model::world::heroes::spells::Spell;
use crate::model::world::heroes::{Hero, HeroBase, PrimarySkills};
use crate::model::world::tile::direction::DirectionSet;
use crate::model::world::tile::{LayerType, ObjectPart, Tile};
use crate::model::world::{IndexObject, MapPosition, Point, World};

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
    heroes::encode(&mut writer, &[]).unwrap();
    castles::encode(&mut writer, &[]).unwrap();
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
    assert!(world.castles.is_empty());
}

#[test]
fn encode_world_round_trips_empty_semantic_world() {
    let world = World {
        width: 0,
        height: 0,
        tiles: Vec::new(),
        heroes: Vec::new(),
        castles: Vec::new(),
    };

    let encoded = encode(&world).unwrap();
    let decoded = decode(&encoded).unwrap();

    assert_eq!(decoded, world);
}

#[test]
fn encode_world_round_trips_semantic_heroes_in_slot_order() {
    let kastore = sample_hero(HeroID::Kastore, "Kastore", PlayerColor::Blue, Race::Warlock);
    let solmyr = sample_hero(HeroID::Solmyr, "Solmyr", PlayerColor::None, Race::Wizard);
    let world = World {
        width: 3,
        height: 2,
        tiles: vec![sample_tile()],
        heroes: vec![solmyr.clone(), kastore.clone()],
        castles: Vec::new(),
    };

    let encoded = encode(&world).unwrap();
    let decoded = decode(&encoded).unwrap();

    assert_eq!(
        decoded,
        World {
            width: 3,
            height: 2,
            tiles: vec![sample_tile()],
            heroes: vec![kastore, solmyr],
            castles: Vec::new(),
        }
    );
}

#[test]
fn encode_world_round_trips_semantic_castles() {
    let castle = sample_castle();
    let world = World {
        width: 3,
        height: 2,
        tiles: vec![sample_tile()],
        heroes: Vec::new(),
        castles: vec![castle.clone()],
    };

    let encoded = encode(&world).unwrap();
    let decoded = decode(&encoded).unwrap();

    assert_eq!(decoded, world);
}

#[test]
fn encode_world_rejects_duplicate_hero_ids() {
    let world = World {
        width: 0,
        height: 0,
        tiles: Vec::new(),
        heroes: vec![
            sample_hero(HeroID::Kastore, "Kastore", PlayerColor::Blue, Race::Warlock),
            sample_hero(
                HeroID::Kastore,
                "Other Kastore",
                PlayerColor::Red,
                Race::Warlock,
            ),
        ],
        castles: Vec::new(),
    };

    assert_eq!(
        encode(&world),
        Err(Error::InvalidModel {
            field: "world heroes",
            message: "hero ids must be unique",
        })
    );
}

fn sample_castle() -> Castle {
    let mut mode = CastleModeSet::EMPTY;
    mode.insert(CastleModeSet::CUSTOM_ARMY);
    mode.insert(CastleModeSet::ALLOW_TO_BUILD_TODAY);

    let constructed_buildings = CastleBuildingSet::from_mask(
        CastleBuilding::Well.bits()
            | CastleBuilding::Marketplace.bits()
            | CastleBuilding::MageGuild2.bits()
            | CastleBuilding::Dwelling6.bits()
            | CastleBuilding::Upgrade7.bits(),
    );
    let disabled_buildings = CastleBuildingSet::from_mask(
        CastleBuilding::Shipyard.bits() | CastleBuilding::Shrine.bits(),
    );

    Castle {
        map_position: MapPosition { x: 44, y: 55 },
        mode,
        race: Race::Warlock,
        constructed_buildings,
        disabled_buildings,
        color_base: PlayerColor::Blue,
        captain: sample_hero(HeroID::Kastore, "Captain", PlayerColor::Blue, Race::Warlock).base,
        name: SaveString::from("Dungeon"),
        mage_guild_spells: MageGuild {
            spells: vec![Spell::Arrow, Spell::Teleport],
            library_spells: vec![Spell::DimensionDoor],
        },
        dwellings: CastleDwellings::from_counts([11, 22, 33, 44, 55, 66]),
        army: Army {
            troops: vec![
                Troop {
                    monster: MonsterType::Centaur,
                    count: 14,
                },
                Troop {
                    monster: MonsterType::Gargoyle,
                    count: 13,
                },
                Troop {
                    monster: MonsterType::Minotaur,
                    count: 12,
                },
                Troop {
                    monster: MonsterType::Hydra,
                    count: 11,
                },
                Troop {
                    monster: MonsterType::BlackDragon,
                    count: 10,
                },
            ],
            spread_combat_formation: true,
            player_color: PlayerColor::Blue,
        },
    }
}

fn sample_hero(id: HeroID, name: &str, color: PlayerColor, race: Race) -> Hero {
    let raw_id = id.to_i32();

    Hero {
        base: HeroBase {
            primary_skills: PrimarySkills {
                attack: raw_id,
                defense: raw_id + 1,
                knowledge: raw_id + 2,
                power: raw_id + 3,
            },
            map_position: MapPosition {
                x: raw_id as i16,
                y: (raw_id as i16) + 10,
            },
            modes: if color == PlayerColor::None {
                HeroModeSet::RECRUIT
            } else {
                HeroModeSet::ENABLEMOVE
            },
            spell_points: raw_id as u32 + 100,
            move_points: raw_id as u32 + 200,
            spell_book: vec![Spell::Arrow, Spell::DimensionDoor],
            bag_artifacts: vec![
                Artifact {
                    id: ArtifactID::MagicBook,
                    ext: 0,
                },
                Artifact {
                    id: ArtifactID::SpellScroll,
                    ext: 57,
                },
            ],
        },
        name: SaveString::from(name),
        color_base: color,
        experience: raw_id as u32 * 1000,
        secondary_skills: vec![
            SecondarySkill {
                id: Skill::Wisdom,
                level: SkillLevel::Advanced,
            },
            SecondarySkill {
                id: Skill::Logistics,
                level: SkillLevel::Expert,
            },
        ],
        army: Army {
            troops: vec![
                Troop {
                    monster: MonsterType::Peasant,
                    count: raw_id as u32 + 1,
                },
                Troop {
                    monster: MonsterType::Archer,
                    count: raw_id as u32 + 2,
                },
                Troop {
                    monster: MonsterType::Mage,
                    count: raw_id as u32 + 3,
                },
                Troop {
                    monster: MonsterType::Titan,
                    count: raw_id as u32 + 4,
                },
                Troop {
                    monster: MonsterType::BlackDragon,
                    count: raw_id as u32 + 5,
                },
            ],
            spread_combat_formation: color == PlayerColor::None,
            player_color: color,
        },
        id,
        portrait: raw_id,
        race,
        object_type_under_hero: 0x0123,
        path: Path {
            hidden: color != PlayerColor::None,
            steps: vec![RouteStep {
                from_index: raw_id * 10,
                direction: Direction::BottomRight,
                movement_cost: raw_id as u32 + 50,
            }],
        },
        direction: Direction::Left,
        sprite_index: raw_id + 7,
        patrol_center: Point {
            x: raw_id * 2,
            y: raw_id * 3,
        },
        patrol_distance: raw_id as u32 + 8,
        visited_objects: vec![IndexObject {
            tile_index: raw_id * 100,
            object_type: 0x0042,
        }],
        last_ground_region: raw_id as u32 + 9,
    }
}
