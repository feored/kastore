use std::fs;

use crate::model::header::player::{PlayerColor, PlayerSlotInfo};
use crate::{Error, SaveGame, SaveString, SaveVersion, load, save, save_as};

#[test]
fn save_round_trips_loaded_fixture() {
    let bytes = fs::read("tests/saves/10032/Guardian_War_0009.sav").unwrap();

    let save_game = load(&bytes).unwrap();
    let encoded = save(&save_game).unwrap();
    let reloaded = load(&encoded).unwrap();

    assert_eq!(
        save_game.source_version,
        SaveVersion::FORMAT_VERSION_1111_RELEASE
    );
    assert_eq!(reloaded.source_version, save_game.source_version);
    assert_eq!(reloaded.header, save_game.header);
    assert_eq!(reloaded.world, save_game.world);
    assert_eq!(reloaded.settings, save_game.settings);
    assert_eq!(
        reloaded.compression_header.raw_size as usize,
        reloaded.body.len()
    );
    assert_eq!(reloaded.compression_header.compression_format_version, 0);
    assert_eq!(reloaded.compression_header.reserved, 0);
    assert!(reloaded.compression_header.zip_size > 0);
}

#[test]
fn save_persists_world_hero_edits_into_reloaded_save() {
    let bytes = fs::read("tests/saves/10032/Guardian_War_0009.sav").unwrap();

    let mut save_game = load(&bytes).unwrap();
    let hero = save_game
        .world
        .heroes
        .iter_mut()
        .find(|hero| hero.color_base == PlayerColor::Blue && hero.is_in_play())
        .expect("fixture should contain an in-play Blue hero");
    hero.experience = hero.experience.saturating_add(9_999);
    hero.base.primary_skills.attack += 2;
    if let Some(first_troop) = hero.army.troops.first_mut() {
        first_troop.count = first_troop.count.saturating_add(11);
    }

    let edited_hero = hero.clone();
    let encoded = save(&save_game).unwrap();
    let reloaded = load(&encoded).unwrap();
    let reloaded_hero = reloaded
        .world
        .heroes
        .iter()
        .find(|hero| hero.id == edited_hero.id)
        .expect("edited hero should still exist after reload");

    assert_eq!(reloaded_hero, &edited_hero);
}

#[test]
fn save_persists_world_castle_edits_into_reloaded_save() {
    let bytes = fs::read("tests/saves/10032/Guardian_War_0009.sav").unwrap();

    let mut save_game = load(&bytes).unwrap();
    let castle = save_game
        .world
        .castles
        .first_mut()
        .expect("fixture should contain at least one castle");
    let original_name = castle.name.to_string_lossy();
    castle.name = SaveString::from(format!("{original_name} [kastore]"));
    castle.dwellings.tier_1 = castle.dwellings.tier_1.saturating_add(7);
    if let Some(first_troop) = castle.army.troops.first_mut() {
        first_troop.count = first_troop.count.saturating_add(3);
    }

    let edited_castle = castle.clone();
    let encoded = save(&save_game).unwrap();
    let reloaded = load(&encoded).unwrap();
    let reloaded_castle = reloaded
        .world
        .castles
        .iter()
        .find(|castle| castle.map_position == edited_castle.map_position)
        .expect("edited castle should still exist after reload");

    assert_eq!(reloaded_castle, &edited_castle);
}

#[test]
fn save_persists_world_captured_object_edits_into_reloaded_save() {
    let bytes = fs::read("tests/saves/10032/Guardian_War_0009.sav").unwrap();

    let mut save_game = load(&bytes).unwrap();
    let (tile_index, captured_object) = save_game
        .world
        .captured_objects
        .iter_mut()
        .next()
        .expect("fixture should contain at least one captured object");
    captured_object.guardians.count = captured_object.guardians.count.saturating_add(9);

    let edited_tile_index = *tile_index;
    let edited_captured_object = captured_object.clone();
    let encoded = save(&save_game).unwrap();
    let reloaded = load(&encoded).unwrap();
    let reloaded_captured_object = reloaded
        .world
        .captured_objects
        .get(&edited_tile_index)
        .expect("edited captured object should still exist after reload");

    assert_eq!(reloaded_captured_object, &edited_captured_object);
}

#[test]
fn save_display_includes_decoded_castles() {
    let bytes = fs::read("tests/saves/10032/Guardian_War_0009.sav").unwrap();

    let save_game = load(&bytes).unwrap();
    let display = save_game.to_string();

    assert!(display.contains("kingdoms:"));
    if let Some(first_kingdom) = save_game.world.kingdoms.iter().find(|kingdom| {
        kingdom.color != PlayerColor::None
            || !kingdom.hero_ids.is_empty()
            || !kingdom.castle_indexes.is_empty()
    }) {
        assert!(display.contains(&first_kingdom.to_string()));
    }

    assert!(display.contains("castles:"));
    if let Some(first_castle) = save_game.world.castles.first() {
        assert!(display.contains(&first_castle.to_string()));
    }
}

#[test]
fn save_rejects_too_many_player_slots_without_panicking() {
    let mut save_game = SaveGame::default();
    save_game.source_version = SaveVersion::FORMAT_VERSION_1111_RELEASE;
    save_game.header.file_info.player_slots = vec![PlayerSlotInfo::default(); 256];

    let display = save_game.to_string();
    let error = save(&save_game).unwrap_err();

    assert!(display.contains("player slots: 256"));
    assert!(display.contains("Slot 255: Neutral race"));
    assert_eq!(
        error,
        Error::InvalidModel {
            field: "player slots",
            message: "player slot count must fit in u8",
        }
    );
}

#[test]
fn save_as_rejects_unsupported_target_version() {
    let save_game = SaveGame {
        source_version: SaveVersion::FORMAT_VERSION_1111_RELEASE,
        ..SaveGame::default()
    };

    let error = save_as(&save_game, SaveVersion::from_u16(9999)).unwrap_err();

    assert_eq!(error, Error::UnsupportedSaveVersion { version: 9999 });
}

#[test]
fn save_as_rejects_version_conversion_for_now() {
    let save_game = SaveGame {
        source_version: SaveVersion::FORMAT_VERSION_1111_RELEASE,
        ..SaveGame::default()
    };

    let error = save_as(&save_game, SaveVersion::FORMAT_VERSION_1150_RELEASE).unwrap_err();

    assert_eq!(
        error,
        Error::NotImplemented {
            feature: "save version conversion",
        }
    );
}
