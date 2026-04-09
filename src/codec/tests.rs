use std::fs;

use crate::{Error, SaveGame, SaveVersion, load, save, save_as};

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
    assert_eq!(reloaded.body, save_game.body);
    assert_eq!(
        reloaded.compression_header.raw_size as usize,
        reloaded.body.len()
    );
    assert_eq!(reloaded.compression_header.compression_format_version, 0);
    assert_eq!(reloaded.compression_header.reserved, 0);
    assert!(reloaded.compression_header.zip_size > 0);
}

#[test]
fn save_rejects_too_many_player_slots_without_panicking() {
    let mut save_game = SaveGame::default();
    save_game.source_version = SaveVersion::FORMAT_VERSION_1111_RELEASE;
    save_game.header.file_info.player_slots = vec![crate::model::PlayerSlotInfo::default(); 256];

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
