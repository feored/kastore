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
fn decode_container_parses_real_fixture_header() {
    let bytes = fs::read("tests/saves/10032/Guardian_War_0009.sav").unwrap();

    let container = decode_container(ContainerRevision::R10032, &bytes).unwrap();

    assert_eq!(container.save_version, SaveVersion::V10032);
    assert!(!container.payload.is_empty());
    assert_eq!(&container.payload[..3], &[0x40, 0x00, 0x00]);
}
