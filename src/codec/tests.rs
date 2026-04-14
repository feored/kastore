use std::fs;

use crate::model::header::player::{PlayerColor, PlayerSlotInfo};
use crate::version::{VersionProfile, profile_for};
use crate::{
    DiagnosticKind, Error, LoadOptions, ParseReport, ParseSection, SaveGame, SaveString,
    SaveVersion, Severity, load, load_with_options, save, save_as,
};

use super::file;
use super::parse::ParseContext;

const FIXTURE_PATH: &str = "tests/saves/10032/Guardian_War_0009.sav";

// Fixture helpers

fn fixture_bytes() -> Vec<u8> {
    fs::read(FIXTURE_PATH).expect("fixture save file should be readable")
}

fn load_fixture() -> SaveGame {
    strict_load(&fixture_bytes()).expect("fixture save should decode in strict mode")
}

fn strict_load(bytes: &[u8]) -> Result<SaveGame, Error> {
    load(bytes)
}

fn strict_load_with_options(bytes: &[u8]) -> Result<ParseReport<SaveGame>, Error> {
    load_with_options(bytes, &LoadOptions::strict())
}

fn permissive_load(bytes: &[u8]) -> Result<ParseReport<SaveGame>, Error> {
    load_with_options(bytes, &LoadOptions::permissive())
}

// Byte mutation helpers for diagnostics policy tests.

fn decode_file_parts(bytes: &[u8]) -> (VersionProfile, file::FileParts) {
    let save_version = file::detect_save_version(bytes)
        .expect("test bytes should contain a detectable save version");
    let profile =
        profile_for(save_version).expect("test bytes should use a supported save profile");
    let mut parse_context = ParseContext::new(LoadOptions::strict().parse_mode);
    let parts = file::decode_file(bytes, profile, &mut parse_context)
        .expect("test bytes should decode into file parts in strict mode");

    (profile, parts)
}

fn bytes_with_reserved_body_bytes(bytes: &[u8], reserved: u16) -> (Vec<u8>, usize) {
    let (_, parts) = decode_file_parts(bytes);
    let reserved_offset = bytes.len() - parts.body_compression_header.zip_size as usize - 2;
    let mut mutated = bytes.to_vec();
    mutated[reserved_offset..reserved_offset + 2].copy_from_slice(&reserved.to_be_bytes());

    (mutated, reserved_offset)
}

fn bytes_with_trailing_body_bytes(bytes: &[u8], trailing: &[u8]) -> Vec<u8> {
    let (profile, mut parts) = decode_file_parts(bytes);
    parts.body.extend_from_slice(trailing);

    file::encode_file(&parts, profile)
        .expect("mutated file parts should encode after appending trailing body bytes")
}

fn assert_warning(
    report: &ParseReport<SaveGame>,
    kind: DiagnosticKind,
    section: ParseSection,
    field: Option<&'static str>,
    offset: Option<usize>,
) {
    assert!(
        report.diagnostics.iter().any(|diagnostic| {
            diagnostic.severity == Severity::Warning
                && diagnostic.kind == kind
                && diagnostic.section == section
                && (field.is_none() || diagnostic.field == field)
                && (offset.is_none() || diagnostic.offset == offset)
        }),
        "expected warning {:?} in {:?} for field {:?} at offset {:?}",
        kind,
        section,
        field,
        offset
    );
}

// 1. Round-trip / persistence tests

#[test]
fn save_round_trips_loaded_fixture() {
    let save_game = load_fixture();
    let encoded = save(&save_game).expect("save should encode successfully");
    let reloaded = strict_load(&encoded).expect("encoded save should reload in strict mode");

    assert_eq!(
        save_game.source_version,
        SaveVersion::FORMAT_VERSION_1111_RELEASE
    );
    assert_eq!(reloaded.source_version, save_game.source_version);
    assert_eq!(reloaded.header, save_game.header);
    assert_eq!(reloaded.world, save_game.world);
    assert_eq!(reloaded.settings, save_game.settings);
    assert_eq!(reloaded.game_over_result, save_game.game_over_result);
    assert_eq!(reloaded.compression_header.compression_format_version, 0);
    assert_eq!(reloaded.compression_header.reserved, 0);
    assert!(reloaded.compression_header.zip_size > 0);
}

#[test]
fn save_persists_world_hero_edits_into_reloaded_save() {
    let mut save_game = load_fixture();
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
    let encoded = save(&save_game).expect("save should encode successfully");
    let reloaded = strict_load(&encoded).expect("encoded save should reload in strict mode");
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
    let mut save_game = load_fixture();
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
    let encoded = save(&save_game).expect("save should encode successfully");
    let reloaded = strict_load(&encoded).expect("encoded save should reload in strict mode");
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
    let mut save_game = load_fixture();
    let (tile_index, captured_object) = save_game
        .world
        .captured_objects
        .iter_mut()
        .next()
        .expect("fixture should contain at least one captured object");
    captured_object.guardians.count = captured_object.guardians.count.saturating_add(9);

    let edited_tile_index = *tile_index;
    let edited_captured_object = captured_object.clone();
    let encoded = save(&save_game).expect("save should encode successfully");
    let reloaded = strict_load(&encoded).expect("encoded save should reload in strict mode");
    let reloaded_captured_object = reloaded
        .world
        .captured_objects
        .get(&edited_tile_index)
        .expect("edited captured object should still exist after reload");

    assert_eq!(reloaded_captured_object, &edited_captured_object);
}

// 2. Display / validation tests

#[test]
fn save_display_includes_decoded_castles() {
    let save_game = load_fixture();
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
    let mut save_game = SaveGame {
        source_version: SaveVersion::FORMAT_VERSION_1111_RELEASE,
        ..SaveGame::default()
    };
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

// 3. Version conversion tests

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
fn save_as_converts_between_supported_versions() {
    let save_game = load_fixture();
    let encoded = save_as(&save_game, SaveVersion::FORMAT_VERSION_1150_RELEASE)
        .expect("save conversion should encode successfully");
    let reloaded = strict_load(&encoded).expect("converted save should reload in strict mode");

    assert_eq!(
        reloaded.source_version,
        SaveVersion::FORMAT_VERSION_1150_RELEASE
    );
    assert_eq!(reloaded.world, save_game.world);
    assert_eq!(reloaded.game_over_result, save_game.game_over_result);
    assert_eq!(
        reloaded.settings.players, save_game.settings.players,
        "settings players should survive conversion unchanged"
    );
}

// 4. Diagnostics / permissive parsing tests

#[test]
fn load_with_options_allows_reserved_body_bytes_in_permissive_mode() {
    let fixture = fixture_bytes();
    let (bytes, reserved_offset) = bytes_with_reserved_body_bytes(&fixture, 1);

    let strict_error = strict_load(&bytes).unwrap_err();
    let strict_options_error = strict_load_with_options(&bytes).unwrap_err();
    let report = permissive_load(&bytes)
        .expect("permissive load should preserve reserved body wrapper bytes");

    assert_eq!(strict_error, strict_options_error);
    assert_eq!(report.value.compression_header.reserved, 1);
    assert_warning(
        &report,
        DiagnosticKind::UnexpectedReservedValue,
        ParseSection::Body,
        Some("body unused"),
        Some(reserved_offset),
    );
}

#[test]
fn load_with_options_allows_trailing_body_bytes_in_permissive_mode() {
    let fixture = fixture_bytes();
    let bytes_with_trailing_body = bytes_with_trailing_body_bytes(&fixture, &[0xAA, 0xBB, 0xCC]);

    let strict_error = strict_load(&bytes_with_trailing_body).unwrap_err();
    let strict_options_error = strict_load_with_options(&bytes_with_trailing_body).unwrap_err();
    let report = permissive_load(&bytes_with_trailing_body)
        .expect("permissive load should preserve trailing bytes after the body end marker");

    assert_eq!(strict_error, strict_options_error);
    assert!(matches!(strict_error, Error::Parse(_)));
    assert_warning(
        &report,
        DiagnosticKind::TrailingBytes,
        ParseSection::Body,
        Some("body"),
        None,
    );
}
