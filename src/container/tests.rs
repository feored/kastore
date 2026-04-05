use std::fs;

use crate::version::ContainerRevision;
use crate::version::SaveVersion;

use super::decode_container;

#[test]
fn decode_container_rejects_invalid_magic() {
    let bytes = [0x00, 0x00, 0x12, 0x34];

    let error = decode_container(ContainerRevision::R10032, &bytes).unwrap_err();

    assert_eq!(error, crate::Error::InvalidContainer("unexpected magic number"));
}

#[test]
fn decode_container_allows_mismatched_version_string() {
    let bytes = [
        0xFF, 0x03, // magic
        0x00, 0x00, 0x00, 0x04, // version string length
        b'o', b'o', b'p', b's', // version string
        0x27, 0x30, // version number 10032
        0x00, 0x00, // flags
        0x00, 0x00, 0x00, 0x00, // filename length
        0x00, 0x00, 0x00, 0x00, // name length
        0x00, 0x00, 0x00, 0x00, // description length
        0x00, 0x00, // width
        0x00, 0x00, // height
        0x00, // difficulty
    ];

    let container = decode_container(ContainerRevision::R10032, &bytes).unwrap();

    assert_eq!(container.save_version, SaveVersion::V10032);
    assert!(!container.header.requires_pol);
    assert_eq!(container.header.map_info.width, 0);
    assert_eq!(container.header.map_info.difficulty, crate::model::Difficulty::Easy);
}

#[test]
fn decode_container_returns_error_for_truncated_map_filename() {
    let bytes = [
        0xFF, 0x03, // magic
        0x00, 0x00, 0x00, 0x05, // version string length
        b'1', b'0', b'0', b'3', b'2', // version string
        0x27, 0x30, // version number 10032
        0x00, 0x00, // flags
    ];

    let error = decode_container(ContainerRevision::R10032, &bytes).unwrap_err();

    assert_eq!(error, crate::Error::InvalidContainer("map filename"));
}

#[test]
fn decode_container_parses_real_fixture_header() {
    let bytes = fs::read("tests/saves/10032/Guardian_War_0009.sav").unwrap();

    let container = decode_container(ContainerRevision::R10032, &bytes).unwrap();

    assert_eq!(container.save_version, SaveVersion::V10032);
    assert!(container.header.requires_pol);
    assert_eq!(container.header.map_info.filename, "GUARDWAR.MX2");
    assert!(container.header.map_info.name.contains("Guardian"));
    assert_eq!(container.header.map_info.width, 72);
    assert_eq!(container.header.map_info.height, 72);
    assert!(container
        .header
        .map_info
        .description
        .starts_with("You and your ally's families"));
}
