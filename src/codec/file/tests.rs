use std::io::Write;

use flate2::Compression;
use flate2::write::ZlibEncoder;

use crate::model::header::game_type::GameType;
use crate::model::header::map_info::{
    GameVersion, LossConditionData, LossConditionKind, VictoryConditionData, VictoryConditionKind,
};
use crate::model::header::player::{
    PlayerColor, PlayerColorsSet, PlayerSlotInfo, PlayerSlotView, Race,
};
use crate::model::header::supported_language::SupportedLanguage;
use crate::version::{SaveVersion, profile_for};
use crate::{ParseError, ParseErrorKind, ParseSection};

use super::{decode_file, encode_file};

fn push_string(bytes: &mut Vec<u8>, value: &[u8]) {
    bytes.extend_from_slice(&(value.len() as u32).to_be_bytes());
    bytes.extend_from_slice(value);
}

fn minimal_file_bytes(
    save_version: SaveVersion,
    version_string: &[u8],
    creator_notes: Option<&[u8]>,
    game_type: i32,
    body: &[u8],
) -> Vec<u8> {
    minimal_file_bytes_with_body_offset(
        save_version,
        version_string,
        creator_notes,
        game_type,
        body,
    )
    .0
}

fn minimal_file_bytes_with_body_offset(
    save_version: SaveVersion,
    version_string: &[u8],
    creator_notes: Option<&[u8]>,
    game_type: i32,
    body: &[u8],
) -> (Vec<u8>, usize) {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&[0xFF, 0x03]);
    push_string(&mut bytes, version_string);
    bytes.extend_from_slice(&save_version.as_u16().to_be_bytes());
    bytes.extend_from_slice(&0u16.to_be_bytes());
    push_string(&mut bytes, b"");
    push_string(&mut bytes, b"");
    push_string(&mut bytes, b"");
    bytes.extend_from_slice(&[0x00, 0x00]); // width
    bytes.extend_from_slice(&[0x00, 0x00]); // height
    bytes.extend_from_slice(&[
        0x00, // difficulty
        0x00, // player entry count
        0x00, // kingdom colors
        0x00, // colors available for humans
        0x00, // colors available for computer
        0x00, // colors of random races
        0x00, // victory condition type
        0x00, // computer also wins
        0x00, // allow normal victory
    ]);
    bytes.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // victory params
    bytes.push(0x00); // loss condition type
    bytes.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // loss params
    bytes.extend_from_slice(&0u32.to_be_bytes());
    bytes.push(0x00); // start with hero in first castle
    bytes.extend_from_slice(&0u32.to_be_bytes());
    bytes.extend_from_slice(&0u32.to_be_bytes());
    bytes.extend_from_slice(&0u32.to_be_bytes());
    bytes.extend_from_slice(&0u32.to_be_bytes());
    bytes.push(0x00); // main language

    if let Some(creator_notes) = creator_notes {
        push_string(&mut bytes, creator_notes);
    }

    bytes.extend_from_slice(&game_type.to_be_bytes());
    let body_offset = bytes.len();
    push_body_chunk(&mut bytes, body);

    (bytes, body_offset)
}

fn push_body_chunk(bytes: &mut Vec<u8>, body: &[u8]) {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(body).unwrap();
    let compressed = encoder.finish().unwrap();

    bytes.extend_from_slice(&(body.len() as u32).to_be_bytes());
    bytes.extend_from_slice(&(compressed.len() as u32).to_be_bytes());
    bytes.extend_from_slice(&0u16.to_be_bytes());
    bytes.extend_from_slice(&0u16.to_be_bytes());
    bytes.extend_from_slice(&compressed);
}

fn decode(bytes: &[u8], save_version: SaveVersion) -> Result<super::FileParts, crate::Error> {
    let profile = profile_for(save_version).expect("supported test version");
    decode_file(bytes, profile)
}

fn decode_then_encode(bytes: &[u8], save_version: SaveVersion) -> Result<Vec<u8>, crate::Error> {
    let parts = decode(bytes, save_version)?;
    let profile = profile_for(save_version).expect("supported test version");
    encode_file(&parts, profile)
}

#[test]
fn decode_file_rejects_invalid_magic() {
    let bytes = [0x00, 0x00, 0x12, 0x34];
    let error = decode(&bytes, SaveVersion::FORMAT_VERSION_1111_RELEASE).unwrap_err();

    assert_eq!(
        error,
        crate::Error::Parse(ParseError {
            section: ParseSection::Container,
            field: "save file magic number",
            offset: 0,
            kind: ParseErrorKind::UnexpectedValue {
                expected: "0xFF03",
                actual: "0x0000".to_string(),
            },
        })
    );
}

#[test]
fn decode_file_returns_error_for_truncated_map_filename() {
    let bytes = [
        0xFF, 0x03, // magic
        0x00, 0x00, 0x00, 0x05, // version string length
        b'1', b'0', b'0', b'3', b'2', // version string
        0x27, 0x30, // version number 10032
        0x00, 0x00, // flags
    ];

    let error = decode(&bytes, SaveVersion::FORMAT_VERSION_1111_RELEASE).unwrap_err();

    assert_eq!(
        error,
        crate::Error::Parse(ParseError {
            section: ParseSection::MapInfo,
            field: "map filename",
            offset: 15,
            kind: ParseErrorKind::Truncated {
                needed: 4,
                remaining: 0,
            },
        })
    );
}

#[test]
fn decode_file_allows_non_utf8_string_bytes_and_ignores_version_string() {
    let mut bytes = vec![
        0xFF, 0x03, // magic
        0x00, 0x00, 0x00, 0x05, // version string length
        b'o', b'o', b'p', b's', b'!', // version string
        0x27, 0x30, // version number 10032
        0x00, 0x00, // flags
        0x00, 0x00, 0x00, 0x02, // filename length
        0xFF, 0xFE, // filename bytes
        0x00, 0x00, 0x00, 0x03, // name length
        b'A', 0x00, b'B', // name bytes with embedded NUL
        0x00, 0x00, 0x00, 0x00, // description length
        0x00, 0x00, // width
        0x00, 0x00, // height
        0x00, // difficulty
        0x02, // player entry count
        0x01, // player 0 race
        0x05, // player 0 allies
        0x20, // player 1 race
        0x06, // player 1 allies
        0x11, // kingdom colors
        0x01, // colors available for humans
        0x10, // colors available for computer
        0x04, // colors of random races
        0x05, // victory condition type
        0x01, // computer also wins
        0x00, // allow normal victory
        0x00, 0x14, // victory condition param 0
        0x12, 0x34, // victory condition param 1
        0x02, // loss condition type
        0xAB, 0xCD, // loss condition param 0
        0x00, 0x09, // loss condition param 1
        0xDE, 0xAD, 0xBE, 0xEF, // timestamp
        0x01, // start with hero in first castle
        0x00, 0x00, 0x00, 0x07, // game version
        0x00, 0x00, 0x00, 0x03, // world date day
        0x00, 0x00, 0x00, 0x04, // world date week
        0x00, 0x00, 0x00, 0x05, // world date month
        0x02, // main language
        0x00, 0x00, 0x00, 0x05, // game type
    ];
    push_body_chunk(&mut bytes, &[0xDE, 0xAD, 0xBE, 0xEF]);

    let file = decode(&bytes, SaveVersion::FORMAT_VERSION_1111_RELEASE).unwrap();

    assert_eq!(file.file_info.filename.as_bytes(), &[0xFF, 0xFE]);
    assert_eq!(file.file_info.name.as_bytes(), b"A\0B");
    assert_eq!(file.file_info.player_slots.len(), 2);
    assert_eq!(
        file.file_info.player_slots[0],
        PlayerSlotInfo {
            race: Race::Knight,
            allies: PlayerColorsSet::from_bits(0x05),
        }
    );
    assert_eq!(
        file.file_info.player_slots[1],
        PlayerSlotInfo {
            race: Race::Necromancer,
            allies: PlayerColorsSet::from_bits(0x06),
        }
    );
    assert_eq!(
        file.file_info.player_slot(0),
        Some(PlayerSlotView {
            slot_index: 0,
            color: Some(PlayerColor::Blue),
            race: Race::Knight,
            allies: PlayerColorsSet::from_bits(0x05),
        })
    );
    assert_eq!(
        file.file_info.player_slot(1),
        Some(PlayerSlotView {
            slot_index: 1,
            color: Some(PlayerColor::Green),
            race: Race::Necromancer,
            allies: PlayerColorsSet::from_bits(0x06),
        })
    );
    assert_eq!(
        file.file_info.kingdom_colors,
        PlayerColorsSet::from_bits(0x11)
    );
    assert_eq!(
        file.file_info.victory_condition,
        VictoryConditionData {
            kind: VictoryConditionKind::CollectEnoughGold,
            comp_also_wins: true,
            allow_normal_victory: false,
            params: [0x0014, 0x1234],
        }
    );
    assert_eq!(
        file.file_info.loss_condition,
        LossConditionData {
            kind: LossConditionKind::LossHero,
            params: [0xABCD, 0x0009],
        }
    );
    assert_eq!(file.file_info.timestamp, 0xDEADBEEF);
    assert_eq!(file.file_info.version, GameVersion::Unknown(7));
    assert_eq!(file.file_info.main_language, SupportedLanguage::POLISH);
    assert_eq!(file.file_info.creator_notes, None);
    assert_eq!(file.game_type, GameType::from_i32(0x0000_0005));
    assert_eq!(file.body, vec![0xDE, 0xAD, 0xBE, 0xEF]);
}

#[test]
fn decode_file_rejects_invalid_body_compression_version() {
    let (mut bytes, body_offset) = minimal_file_bytes_with_body_offset(
        SaveVersion::FORMAT_VERSION_1111_RELEASE,
        b"10032",
        None,
        0x0000_0002,
        &[0xFF, 0x03],
    );
    let compression_version_offset = body_offset + 8;
    bytes[compression_version_offset] = 0x00;
    bytes[compression_version_offset + 1] = 0x01;

    let error = decode(&bytes, SaveVersion::FORMAT_VERSION_1111_RELEASE).unwrap_err();

    assert_eq!(
        error,
        crate::Error::Parse(ParseError {
            section: ParseSection::Body,
            field: "body compression format version",
            offset: compression_version_offset,
            kind: ParseErrorKind::UnexpectedValue {
                expected: "0",
                actual: "1".to_string(),
            },
        })
    );
}

#[test]
fn decode_file_rejects_body_size_mismatch() {
    let (mut bytes, body_offset) = minimal_file_bytes_with_body_offset(
        SaveVersion::FORMAT_VERSION_1111_RELEASE,
        b"10032",
        None,
        0x0000_0002,
        &[0xAA, 0xBB, 0xCC],
    );
    bytes[body_offset..body_offset + 4].copy_from_slice(&5u32.to_be_bytes());

    let error = decode(&bytes, SaveVersion::FORMAT_VERSION_1111_RELEASE).unwrap_err();

    assert_eq!(
        error,
        crate::Error::Parse(ParseError {
            section: ParseSection::Body,
            field: "body decompressed size",
            offset: body_offset,
            kind: ParseErrorKind::InvalidValue {
                message: "decompressed body size does not match raw size",
            },
        })
    );
}

#[test]
fn encode_file_round_trips_v10033_creator_notes() {
    let bytes = minimal_file_bytes(
        SaveVersion::FORMAT_VERSION_1150_RELEASE,
        b"10033",
        Some(&[0xFF, 0x00, b'A']),
        0x0000_0002,
        &[0xFF, 0x03],
    );

    let encoded = decode_then_encode(&bytes, SaveVersion::FORMAT_VERSION_1150_RELEASE).unwrap();
    let original = decode(&bytes, SaveVersion::FORMAT_VERSION_1150_RELEASE).unwrap();
    let round_tripped = decode(&encoded, SaveVersion::FORMAT_VERSION_1150_RELEASE).unwrap();

    assert_eq!(round_tripped, original);
}
