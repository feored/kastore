use std::collections::BTreeMap;

use super::*;
use crate::SaveString;
use crate::internal::writer::Writer;
use crate::model::header::map_info::WorldDate;
use crate::model::header::player::{PlayerColor, PlayerColorsSet, Race};
use crate::model::header::supported_language::SupportedLanguage;
use crate::model::world::captured_objects::CapturedObject;
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
use crate::model::world::kingdoms::{
    KINGDOM_SLOT_COUNT, Kingdom, KingdomModeSet, PUZZLE_REVEALED_TILES_COUNT,
};
use crate::model::world::map_objects::{
    LocalizedString, MapEvent, MapObject, MapObjectBase, MapSign, MapSphinx,
};
use crate::model::world::tile::direction::DirectionSet;
use crate::model::world::tile::{LayerType, ObjectPart, Tile};
use crate::model::world::timed_events::TimedEvent;
use crate::model::world::ultimate_artifact::UltimateArtifact;
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
    kingdoms::encode(&mut writer, &vec![Kingdom::default(); KINGDOM_SLOT_COUNT]).unwrap();
    writer.write_u32_be(0);
    writer.write_u32_be(0);
    writer.write_u32_be(0);
    super::ultimate_artifact::encode(&mut writer, &UltimateArtifact::default()).unwrap();
    crate::codec::world_date::encode_world_date(&mut writer, WorldDate::default());
    writer.write_i32_be(HeroID::Unknown(0).to_i32());
    writer.write_i32_be(HeroID::Unknown(0).to_i32());
    writer.write_u32_be(0);
    writer.write_u32_be(0);
    writer.into_bytes()
}

fn castle_index(width: i32, height: i32, castle: &Castle) -> i32 {
    super::validation::castle_index_from_map_position(width, height, castle).unwrap()
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
    assert_eq!(world.kingdoms, vec![Kingdom::default(); KINGDOM_SLOT_COUNT]);
    assert!(world.custom_rumors.is_empty());
    assert!(world.timed_events.is_empty());
    assert!(world.captured_objects.is_empty());
    assert_eq!(world.ultimate_artifact, UltimateArtifact::default());
    assert_eq!(world.world_date, WorldDate::default());
    assert_eq!(world.hero_id_as_win_condition, HeroID::Unknown(0));
    assert_eq!(world.hero_id_as_lose_condition, HeroID::Unknown(0));
    assert!(world.map_objects.is_empty());
    assert_eq!(world.seed, 0);
}

#[test]
fn encode_world_round_trips_empty_semantic_world() {
    let world = World::default();

    let encoded = encode(&world).unwrap();
    let decoded = decode(&encoded).unwrap();

    assert_eq!(decoded, world);
}

#[test]
fn world_display_includes_world_extras() {
    let mut world = World::default();
    world.custom_rumors = vec![SaveString::from("Hidden treasure in the marshes")];
    world.timed_events = vec![sample_timed_event()];
    world.captured_objects = BTreeMap::from([(
        4,
        CapturedObject {
            object_type: 54,
            color: PlayerColor::Blue,
            guardians: Troop {
                monster: MonsterType::Griffin,
                count: 27,
            },
        },
    )]);
    world.map_objects = sample_map_objects();

    let display = world.to_string();

    assert!(display.contains("custom_rumors:"));
    assert!(display.contains("Hidden treasure in the marshes"));
    assert!(display.contains("timed_events:"));
    assert!(display.contains("Weekly Bonus"));
    assert!(display.contains("1 captured objects"));
    assert!(display.contains("map_objects:"));
    assert!(display.contains("Sign uid=41"));
}

#[test]
fn encode_world_round_trips_semantic_heroes_in_slot_order() {
    let kastore = sample_hero(HeroID::Kastore, "Kastore", PlayerColor::Blue, Race::Warlock);
    let solmyr = sample_hero(HeroID::Solmyr, "Solmyr", PlayerColor::None, Race::Wizard);
    let kingdoms = sample_kingdoms(3, 2, std::slice::from_ref(&kastore), &[]);
    let world = World {
        width: 3,
        height: 2,
        tiles: vec![sample_tile()],
        heroes: vec![solmyr.clone(), kastore.clone()],
        castles: Vec::new(),
        kingdoms,
        custom_rumors: Vec::new(),
        timed_events: Vec::new(),
        captured_objects: BTreeMap::new(),
        ultimate_artifact: UltimateArtifact::default(),
        world_date: WorldDate::default(),
        hero_id_as_win_condition: HeroID::Unknown(0),
        hero_id_as_lose_condition: HeroID::Unknown(0),
        map_objects: BTreeMap::new(),
        seed: 0,
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
            kingdoms: sample_kingdoms(
                3,
                2,
                &[sample_hero(
                    HeroID::Kastore,
                    "Kastore",
                    PlayerColor::Blue,
                    Race::Warlock,
                )],
                &[],
            ),
            custom_rumors: Vec::new(),
            timed_events: Vec::new(),
            captured_objects: BTreeMap::new(),
            ultimate_artifact: UltimateArtifact::default(),
            world_date: WorldDate::default(),
            hero_id_as_win_condition: HeroID::Unknown(0),
            hero_id_as_lose_condition: HeroID::Unknown(0),
            map_objects: BTreeMap::new(),
            seed: 0,
        }
    );
}

#[test]
fn encode_world_round_trips_semantic_castles() {
    let castle = sample_castle();
    let kingdoms = sample_kingdoms(3, 2, &[], std::slice::from_ref(&castle));
    let world = World {
        width: 3,
        height: 2,
        tiles: vec![sample_tile()],
        heroes: Vec::new(),
        castles: vec![castle.clone()],
        kingdoms,
        custom_rumors: Vec::new(),
        timed_events: Vec::new(),
        captured_objects: BTreeMap::new(),
        ultimate_artifact: UltimateArtifact::default(),
        world_date: WorldDate::default(),
        hero_id_as_win_condition: HeroID::Unknown(0),
        hero_id_as_lose_condition: HeroID::Unknown(0),
        map_objects: BTreeMap::new(),
        seed: 0,
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
        kingdoms: vec![Kingdom::default(); KINGDOM_SLOT_COUNT],
        custom_rumors: Vec::new(),
        timed_events: Vec::new(),
        captured_objects: BTreeMap::new(),
        ultimate_artifact: UltimateArtifact::default(),
        world_date: WorldDate::default(),
        hero_id_as_win_condition: HeroID::Unknown(0),
        hero_id_as_lose_condition: HeroID::Unknown(0),
        map_objects: BTreeMap::new(),
        seed: 0,
    };

    assert_eq!(
        encode(&world),
        Err(Error::InvalidModel {
            field: "world heroes",
            message: "hero ids must be unique",
        })
    );
}

#[test]
fn encode_world_rejects_kingdom_hero_color_mismatch() {
    let hero = sample_hero(HeroID::Kastore, "Kastore", PlayerColor::Blue, Race::Warlock);
    let mut kingdoms = vec![Kingdom::default(); KINGDOM_SLOT_COUNT];
    kingdoms[2].color = PlayerColor::Red;
    kingdoms[2].hero_ids.push(hero.id);
    let world = World {
        width: 3,
        height: 2,
        tiles: vec![sample_tile()],
        heroes: vec![hero],
        castles: Vec::new(),
        kingdoms,
        custom_rumors: Vec::new(),
        timed_events: Vec::new(),
        captured_objects: BTreeMap::new(),
        ultimate_artifact: UltimateArtifact::default(),
        world_date: WorldDate::default(),
        hero_id_as_win_condition: HeroID::Unknown(0),
        hero_id_as_lose_condition: HeroID::Unknown(0),
        map_objects: BTreeMap::new(),
        seed: 0,
    };

    assert_eq!(
        encode(&world),
        Err(Error::InvalidModel {
            field: "kingdom heroes",
            message: "kingdom hero references must match the referenced hero color",
        })
    );
}

#[test]
fn encode_world_round_trips_semantic_kingdom_details() {
    let hero = sample_hero(HeroID::Kastore, "Kastore", PlayerColor::Blue, Race::Warlock);
    let castle = sample_castle();
    let mut kingdoms = sample_kingdoms(
        3,
        2,
        std::slice::from_ref(&hero),
        std::slice::from_ref(&castle),
    );
    kingdoms[0].mode = KingdomModeSet::from_bits(KingdomModeSet::IDENTIFYHERO.bits());
    kingdoms[0].recruits.first.hero_id = hero.id;
    kingdoms[0].recruits.first.surrender_day = 7;
    kingdoms[0].visited_objects = vec![
        IndexObject {
            tile_index: 5,
            object_type: 77,
        },
        IndexObject {
            tile_index: 6,
            object_type: 88,
        },
    ];
    kingdoms[0].puzzle.revealed_tiles = "01".repeat(PUZZLE_REVEALED_TILES_COUNT / 2).into_bytes();
    kingdoms[0].puzzle.zone1_order.reverse();
    kingdoms[0].puzzle.zone2_order.reverse();
    kingdoms[0].visited_tents_colors = 1 << 8;
    kingdoms[0].top_castle_in_kingdom_view = 3;
    kingdoms[0].top_hero_in_kingdom_view = 4;
    let custom_rumors = vec![
        SaveString::from("A rumor from the tavern"),
        SaveString::from("A second rumor"),
    ];
    let timed_events = vec![sample_timed_event()];
    let captured_objects = BTreeMap::from([(
        4,
        CapturedObject {
            object_type: 54,
            color: PlayerColor::Blue,
            guardians: Troop {
                monster: MonsterType::Griffin,
                count: 27,
            },
        },
    )]);
    let map_objects = sample_map_objects();

    let world = World {
        width: 3,
        height: 2,
        tiles: vec![sample_tile()],
        heroes: vec![hero],
        castles: vec![castle],
        kingdoms,
        custom_rumors,
        timed_events,
        captured_objects,
        ultimate_artifact: UltimateArtifact::default(),
        world_date: WorldDate::default(),
        hero_id_as_win_condition: HeroID::Unknown(0),
        hero_id_as_lose_condition: HeroID::Unknown(0),
        map_objects,
        seed: 0xDEAD_BEEF,
    };

    let encoded = encode(&world).unwrap();
    let decoded = decode(&encoded).unwrap();

    assert_eq!(decoded, world);
}

#[test]
fn encode_world_rejects_invalid_kingdom_puzzle_revealed_tiles() {
    let mut world = World::default();
    world.kingdoms[0].puzzle.revealed_tiles = vec![b'0'; PUZZLE_REVEALED_TILES_COUNT - 1];

    assert_eq!(
        encode(&world),
        Err(Error::InvalidModel {
            field: "kingdom puzzle revealed tiles",
            message: "revealed tiles must be 48 ASCII '0'/'1' bytes",
        })
    );
}

#[test]
fn encode_world_rejects_invalid_kingdom_puzzle_zone_size() {
    let mut world = World::default();
    world.kingdoms[0].puzzle.zone1_order.pop();

    assert_eq!(
        encode(&world),
        Err(Error::InvalidModel {
            field: "kingdom puzzle zone1",
            message: "zone must contain exactly 24 tiles",
        })
    );
}

#[test]
fn encode_world_rejects_kingdom_slot_color_mismatch() {
    let mut world = World::default();
    world.kingdoms[1].color = PlayerColor::Blue;

    assert_eq!(
        encode(&world),
        Err(Error::InvalidModel {
            field: "kingdom colors",
            message: "kingdom slot colors must match fheroes2 slot order or be None for inactive slots",
        })
    );
}

#[test]
fn encode_world_rejects_missing_kingdom_hero_membership() {
    let hero = sample_hero(HeroID::Kastore, "Kastore", PlayerColor::Blue, Race::Warlock);
    let mut kingdoms = vec![Kingdom::default(); KINGDOM_SLOT_COUNT];
    kingdoms[0].color = PlayerColor::Blue;
    let world = World {
        width: 0,
        height: 0,
        tiles: Vec::new(),
        heroes: vec![hero],
        castles: Vec::new(),
        kingdoms,
        custom_rumors: Vec::new(),
        timed_events: Vec::new(),
        captured_objects: BTreeMap::new(),
        ultimate_artifact: UltimateArtifact::default(),
        world_date: WorldDate::default(),
        hero_id_as_win_condition: HeroID::Unknown(0),
        hero_id_as_lose_condition: HeroID::Unknown(0),
        map_objects: BTreeMap::new(),
        seed: 0,
    };

    assert_eq!(
        encode(&world),
        Err(Error::InvalidModel {
            field: "kingdom heroes",
            message: "every non-neutral hero must appear in exactly one kingdom hero list",
        })
    );
}

#[test]
fn encode_world_rejects_kingdom_castle_color_mismatch() {
    let castle = sample_castle();
    let mut kingdoms = vec![Kingdom::default(); KINGDOM_SLOT_COUNT];
    kingdoms[1].color = PlayerColor::Green;
    kingdoms[1].castle_indexes.push(castle_index(3, 2, &castle));
    let world = World {
        width: 3,
        height: 2,
        tiles: vec![sample_tile()],
        heroes: Vec::new(),
        castles: vec![castle],
        kingdoms,
        custom_rumors: Vec::new(),
        timed_events: Vec::new(),
        captured_objects: BTreeMap::new(),
        ultimate_artifact: UltimateArtifact::default(),
        world_date: WorldDate::default(),
        hero_id_as_win_condition: HeroID::Unknown(0),
        hero_id_as_lose_condition: HeroID::Unknown(0),
        map_objects: BTreeMap::new(),
        seed: 0,
    };

    assert_eq!(
        encode(&world),
        Err(Error::InvalidModel {
            field: "kingdom castles",
            message: "kingdom castle references must match the referenced castle color",
        })
    );
}

#[test]
fn encode_world_rejects_unknown_kingdom_castle_ref() {
    let mut kingdoms = vec![Kingdom::default(); KINGDOM_SLOT_COUNT];
    kingdoms[0].color = PlayerColor::Blue;
    kingdoms[0].castle_indexes.push(1234);
    let world = World {
        width: 3,
        height: 2,
        tiles: vec![sample_tile()],
        heroes: Vec::new(),
        castles: Vec::new(),
        kingdoms,
        custom_rumors: Vec::new(),
        timed_events: Vec::new(),
        captured_objects: BTreeMap::new(),
        ultimate_artifact: UltimateArtifact::default(),
        world_date: WorldDate::default(),
        hero_id_as_win_condition: HeroID::Unknown(0),
        hero_id_as_lose_condition: HeroID::Unknown(0),
        map_objects: BTreeMap::new(),
        seed: 0,
    };

    assert_eq!(
        encode(&world),
        Err(Error::InvalidModel {
            field: "kingdom castles",
            message: "kingdom castle references must point to decoded castles",
        })
    );
}

#[test]
fn encode_world_rejects_map_object_uid_mismatch() {
    let world = World {
        width: 0,
        height: 0,
        tiles: Vec::new(),
        heroes: Vec::new(),
        castles: Vec::new(),
        kingdoms: vec![Kingdom::default(); KINGDOM_SLOT_COUNT],
        custom_rumors: Vec::new(),
        timed_events: Vec::new(),
        captured_objects: BTreeMap::new(),
        ultimate_artifact: UltimateArtifact::default(),
        world_date: WorldDate::default(),
        hero_id_as_win_condition: HeroID::Unknown(0),
        hero_id_as_lose_condition: HeroID::Unknown(0),
        map_objects: BTreeMap::from([(
            41,
            MapObject::Sign(MapSign {
                base: MapObjectBase {
                    map_position: MapPosition { x: 2, y: 3 },
                    uid: 42,
                },
                message: LocalizedString {
                    text: SaveString::from("Beware the swamp."),
                    language: Some(SupportedLanguage::ENGLISH),
                },
            }),
        )]),
        seed: 0,
    };

    assert_eq!(
        encode(&world),
        Err(Error::InvalidModel {
            field: "world map objects",
            message: "map object key uid must match object body uid",
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
        map_position: MapPosition { x: 1, y: 1 },
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

fn sample_kingdoms(width: i32, height: i32, heroes: &[Hero], castles: &[Castle]) -> Vec<Kingdom> {
    let mut kingdoms = vec![Kingdom::default(); KINGDOM_SLOT_COUNT];
    kingdoms[0].color = PlayerColor::Blue;
    kingdoms[0].hero_ids = heroes
        .iter()
        .filter(|hero| hero.color_base == PlayerColor::Blue)
        .map(|hero| hero.id)
        .collect();
    kingdoms[0].castle_indexes = castles
        .iter()
        .filter(|castle| castle.color_base == PlayerColor::Blue)
        .map(|castle| castle_index(width, height, castle))
        .collect();
    kingdoms[0].funds.gold = 12_345;
    kingdoms[0].top_castle_in_kingdom_view = 0;
    kingdoms[0].top_hero_in_kingdom_view = 0;
    kingdoms
}

fn sample_timed_event() -> TimedEvent {
    TimedEvent {
        resources: crate::model::world::Funds {
            wood: 5,
            mercury: -1,
            ore: 7,
            sulfur: 0,
            crystal: 3,
            gems: 2,
            gold: 1_500,
        },
        is_applicable_for_ai_players: true,
        first_occurrence_day: 4,
        repeat_period_in_days: 7,
        colors: PlayerColorsSet::from_bits(PlayerColor::Blue.bits() | PlayerColor::Green.bits()),
        message: SaveString::from("The treasury grows."),
        title: SaveString::from("Weekly Bonus"),
    }
}

fn sample_map_objects() -> BTreeMap<u32, MapObject> {
    BTreeMap::from([
        (
            41,
            MapObject::Sign(MapSign {
                base: MapObjectBase {
                    map_position: MapPosition { x: 2, y: 3 },
                    uid: 41,
                },
                message: LocalizedString {
                    text: SaveString::from("Beware the swamp."),
                    language: Some(SupportedLanguage::ENGLISH),
                },
            }),
        ),
        (
            55,
            MapObject::Event(MapEvent {
                base: MapObjectBase {
                    map_position: MapPosition { x: 4, y: 5 },
                    uid: 55,
                },
                resources: crate::model::world::Funds {
                    wood: 3,
                    mercury: 0,
                    ore: 2,
                    sulfur: 0,
                    crystal: 0,
                    gems: 1,
                    gold: 750,
                },
                artifact: Artifact {
                    id: ArtifactID::MagicBook,
                    ext: 0,
                },
                is_computer_player_allowed: true,
                is_single_time_event: true,
                colors: PlayerColorsSet::from_bits(PlayerColor::Blue.bits()),
                message: SaveString::from("Blue only."),
                secondary_skill: SecondarySkill {
                    id: Skill::Wisdom,
                    level: SkillLevel::Basic,
                },
                experience: 250,
            }),
        ),
        (
            77,
            MapObject::Sphinx(MapSphinx {
                base: MapObjectBase {
                    map_position: MapPosition { x: 6, y: 7 },
                    uid: 77,
                },
                resources: crate::model::world::Funds {
                    wood: 0,
                    mercury: 0,
                    ore: 0,
                    sulfur: 0,
                    crystal: 0,
                    gems: 0,
                    gold: 1_500,
                },
                artifact: Artifact {
                    id: ArtifactID::GoldenGoose,
                    ext: 0,
                },
                answers: vec![SaveString::from("silence"), SaveString::from("shadow")],
                riddle: SaveString::from("What walks unseen?"),
                valid: true,
                is_truncated_answer: false,
            }),
        ),
    ])
}

fn sample_hero(id: HeroID, name: &str, color: PlayerColor, race: Race) -> Hero {
    let (
        primary_skills,
        map_position,
        spell_points,
        move_points,
        spell_book,
        bag_artifacts,
        experience,
        secondary_skills,
        troops,
        from_index,
        movement_cost,
        sprite_index,
        patrol_center,
        patrol_distance,
        visited_tile_index,
        last_ground_region,
    ) = match id {
        HeroID::Kastore => (
            PrimarySkills {
                attack: 3,
                defense: 2,
                knowledge: 4,
                power: 5,
            },
            MapPosition { x: 5, y: 8 },
            18,
            1_420,
            vec![Spell::Arrow, Spell::Teleport],
            vec![
                Artifact {
                    id: ArtifactID::MagicBook,
                    ext: 0,
                },
                Artifact {
                    id: ArtifactID::SpellScroll,
                    ext: 57,
                },
            ],
            12_750,
            vec![
                SecondarySkill {
                    id: Skill::Wisdom,
                    level: SkillLevel::Advanced,
                },
                SecondarySkill {
                    id: Skill::Logistics,
                    level: SkillLevel::Advanced,
                },
            ],
            vec![
                Troop {
                    monster: MonsterType::Centaur,
                    count: 28,
                },
                Troop {
                    monster: MonsterType::Gargoyle,
                    count: 18,
                },
                Troop {
                    monster: MonsterType::Griffin,
                    count: 12,
                },
                Troop {
                    monster: MonsterType::Minotaur,
                    count: 7,
                },
                Troop {
                    monster: MonsterType::BlackDragon,
                    count: 2,
                },
            ],
            118,
            130,
            40,
            Point { x: 6, y: 9 },
            3,
            211,
            9,
        ),
        HeroID::Solmyr => (
            PrimarySkills {
                attack: 2,
                defense: 3,
                knowledge: 8,
                power: 7,
            },
            MapPosition { x: 11, y: 4 },
            34,
            1_360,
            vec![Spell::Arrow, Spell::DimensionDoor],
            vec![
                Artifact {
                    id: ArtifactID::MagicBook,
                    ext: 0,
                },
                Artifact {
                    id: ArtifactID::SpellScroll,
                    ext: 44,
                },
            ],
            19_500,
            vec![
                SecondarySkill {
                    id: Skill::Wisdom,
                    level: SkillLevel::Expert,
                },
                SecondarySkill {
                    id: Skill::Mysticism,
                    level: SkillLevel::Advanced,
                },
            ],
            vec![
                Troop {
                    monster: MonsterType::Halfling,
                    count: 32,
                },
                Troop {
                    monster: MonsterType::Boar,
                    count: 18,
                },
                Troop {
                    monster: MonsterType::IronGolem,
                    count: 12,
                },
                Troop {
                    monster: MonsterType::Mage,
                    count: 6,
                },
                Troop {
                    monster: MonsterType::Titan,
                    count: 2,
                },
            ],
            64,
            120,
            68,
            Point { x: 10, y: 4 },
            2,
            305,
            12,
        ),
        other => panic!("missing sample hero fixture for {other:?}"),
    };

    Hero {
        base: HeroBase {
            primary_skills,
            map_position,
            modes: if color == PlayerColor::None {
                HeroModeSet::RECRUIT
            } else {
                HeroModeSet::ENABLEMOVE
            },
            spell_points,
            move_points,
            spell_book,
            bag_artifacts,
        },
        name: SaveString::from(name),
        color_base: color,
        experience,
        secondary_skills,
        army: Army {
            troops,
            spread_combat_formation: color == PlayerColor::None,
            player_color: color,
        },
        id,
        portrait: id.to_i32(),
        race,
        object_type_under_hero: 0x0123,
        path: Path {
            hidden: color != PlayerColor::None,
            steps: vec![RouteStep {
                from_index,
                direction: Direction::BottomRight,
                movement_cost,
            }],
        },
        direction: Direction::Left,
        sprite_index,
        patrol_center,
        patrol_distance,
        visited_objects: vec![IndexObject {
            tile_index: visited_tile_index,
            object_type: 0x0042,
        }],
        last_ground_region,
    }
}
