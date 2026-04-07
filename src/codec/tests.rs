use std::fs;

use crate::{SaveVersion, load, save};

#[test]
fn save_round_trips_loaded_fixture() {
    let bytes = fs::read("tests/saves/10032/Guardian_War_0009.sav").unwrap();

    let save_game = load(&bytes).unwrap();
    let encoded = save(&save_game).unwrap();
    let display = save_game.to_string();

    assert_eq!(
        save_game.source_version,
        SaveVersion::FORMAT_VERSION_1111_RELEASE
    );
    assert!(display.contains("save version: 10032"));
    assert!(display.contains("game type: Standard"));
    assert!(display.contains("map name: Guardian War"));
    assert_eq!(encoded, bytes);
}
