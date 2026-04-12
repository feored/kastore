use std::io::Read;
use std::io::Write;

use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;

use crate::Error;
use crate::SaveVersion;
use crate::internal::error::ParseSection;
use crate::internal::reader::Reader;
use crate::internal::writer::Writer;
use crate::model::header::game_type::GameType;
use crate::model::header::map_info::{
    Difficulty, GameVersion, LossConditionData, LossConditionKind, MapInfo, VictoryConditionData,
    VictoryConditionKind, WorldDate,
};
use crate::model::header::player::{PlayerColorsSet, PlayerSlotInfo, Race};
use crate::model::header::supported_language::SupportedLanguage;
use crate::model::save_game::BodyCompressionHeader;
use crate::version::{MapInfoRevision, VersionProfile};

pub(crate) const SAVE_FILE_MAGIC_NUMBER: u16 = 0xFF03;
pub(crate) const REQUIRES_POL: u16 = 0x4000;
const COMPRESSION_FORMAT_VERSION_0: u16 = 0;
const RESERVED_BYTES_0: u16 = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FileVersion {
    save_version: SaveVersion,
    save_version_offset: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct FileParts {
    pub(crate) requires_pol: bool,
    /// Summary metadata from the outer save header (`HeaderSAV` / `Maps::FileInfo`).
    pub(crate) file_info: MapInfo,
    pub(crate) game_type: GameType,
    pub(crate) body_compression_header: BodyCompressionHeader,
    /// Decompressed gameplay body bytes from the save file's compressed body chunk.
    pub(crate) body: Vec<u8>,
}

fn expect_save_file_magic(reader: &mut Reader<'_>) -> std::result::Result<(), Error> {
    let magic_offset = reader.position();
    let magic_number = reader.read_u16_be("save file magic number")?;

    if magic_number != SAVE_FILE_MAGIC_NUMBER {
        return Err(reader.unexpected_value(
            "save file magic number",
            magic_offset,
            "0xFF03",
            format!("0x{magic_number:04X}"),
        ));
    }

    Ok(())
}

fn encode_save_file_magic(writer: &mut Writer) {
    writer.write_u16_be(SAVE_FILE_MAGIC_NUMBER);
}

fn decode_version(reader: &mut Reader<'_>) -> std::result::Result<FileVersion, Error> {
    let _save_version_string = reader.read_save_string("save version string")?;
    let save_version_offset = reader.position();
    let save_version = SaveVersion::from_u16(reader.read_u16_be("save version")?);

    Ok(FileVersion {
        save_version,
        save_version_offset,
    })
}

fn encode_version(writer: &mut Writer, version: SaveVersion) {
    let version_string = version.as_u16().to_string();
    writer.write_u32_be(version_string.len() as u32);
    writer.write_bytes(version_string.as_bytes());
    writer.write_u16_be(version.as_u16());
}

pub(crate) fn detect_save_version(bytes: &[u8]) -> std::result::Result<SaveVersion, Error> {
    let mut reader = Reader::with_context(bytes, ParseSection::Container);
    expect_save_file_magic(&mut reader)?;
    Ok(decode_version(&mut reader)?.save_version)
}

pub(crate) fn decode_file(
    bytes: &[u8],
    profile: VersionProfile,
) -> std::result::Result<FileParts, Error> {
    let mut reader = Reader::with_context(bytes, ParseSection::Container);
    expect_save_file_magic(&mut reader)?;
    let version = decode_version(&mut reader)?;

    if version.save_version != profile.save_version {
        return Err(reader.invalid_value(
            "save version",
            version.save_version_offset,
            "detected version does not match selected profile",
        ));
    }

    reader.set_section(ParseSection::Header);
    let requires_pol = (reader.read_u16_be("flags")? & REQUIRES_POL) != 0;
    let file_info = decode_map_info(&mut reader, profile.map_info_revision)?;
    let game_type = GameType::from_i32(reader.read_i32_be("game type")?);
    let (body_compression_header, body) = decode_body_bytes(&mut reader)?;

    Ok(FileParts {
        requires_pol,
        file_info,
        game_type,
        body_compression_header,
        body,
    })
}

pub(crate) fn encode_file(
    parts: &FileParts,
    profile: VersionProfile,
) -> std::result::Result<Vec<u8>, Error> {
    let mut writer = Writer::new();
    encode_save_file_magic(&mut writer);
    encode_version(&mut writer, profile.save_version);
    writer.write_u16_be(if parts.requires_pol { REQUIRES_POL } else { 0 });
    encode_map_info(&mut writer, &parts.file_info, profile.map_info_revision)?;
    writer.write_i32_be(parts.game_type.to_i32());
    let _body_compression_header = encode_body_bytes(&mut writer, &parts.body)?;
    Ok(writer.into_bytes())
}

fn decode_map_info(
    reader: &mut Reader<'_>,
    revision: MapInfoRevision,
) -> std::result::Result<MapInfo, Error> {
    reader.set_section(ParseSection::MapInfo);

    let filename = reader.read_save_string("map filename")?;
    let name = reader.read_save_string("map name")?;
    let description = reader.read_save_string("map description")?;
    let width = reader.read_u16_be("map width")?;
    let height = reader.read_u16_be("map height")?;
    let difficulty = Difficulty::from_byte(reader.read_u8("map difficulty")?);
    let player_entry_count = reader.read_u8("player entry count")?;
    let mut player_slots = Vec::with_capacity(usize::from(player_entry_count));

    for _ in 0..player_entry_count {
        player_slots.push(PlayerSlotInfo {
            race: Race::from_byte(reader.read_u8("player race")?),
            allies: PlayerColorsSet::from_bits(reader.read_u8("player allies")?),
        });
    }

    let kingdom_colors = PlayerColorsSet::from_bits(reader.read_u8("kingdom colors")?);
    let colors_available_for_humans =
        PlayerColorsSet::from_bits(reader.read_u8("colors available for humans")?);
    let colors_available_for_comp =
        PlayerColorsSet::from_bits(reader.read_u8("colors available for computer")?);
    let colors_of_random_races =
        PlayerColorsSet::from_bits(reader.read_u8("colors of random races")?);
    let victory_condition = VictoryConditionData {
        kind: VictoryConditionKind::from_byte(reader.read_u8("victory condition type")?),
        comp_also_wins: reader.read_byte_as_bool("computer also wins")?,
        allow_normal_victory: reader.read_byte_as_bool("allow normal victory")?,
        params: [
            reader.read_u16_be("victory condition param 0")?,
            reader.read_u16_be("victory condition param 1")?,
        ],
    };
    let loss_condition = LossConditionData {
        kind: LossConditionKind::from_byte(reader.read_u8("loss condition type")?),
        params: [
            reader.read_u16_be("loss condition param 0")?,
            reader.read_u16_be("loss condition param 1")?,
        ],
    };
    let timestamp = reader.read_u32_be("timestamp")?;
    let start_with_hero_in_first_castle =
        reader.read_byte_as_bool("start with hero in first castle")?;
    let version = GameVersion::from_u32(reader.read_u32_be("game version")?);
    let world_date = WorldDate {
        day: reader.read_u32_be("world date day")?,
        week: reader.read_u32_be("world date week")?,
        month: reader.read_u32_be("world date month")?,
    };
    let main_language = SupportedLanguage::from(reader.read_u8("main language")?);
    let creator_notes = match revision {
        MapInfoRevision::V10024 => None,
        MapInfoRevision::V10033 => Some(reader.read_save_string("creator notes")?),
    };

    Ok(MapInfo {
        filename,
        name,
        description,
        width,
        height,
        difficulty,
        player_slots,
        kingdom_colors,
        colors_available_for_humans,
        colors_available_for_comp,
        colors_of_random_races,
        victory_condition,
        loss_condition,
        timestamp,
        start_with_hero_in_first_castle,
        version,
        world_date,
        main_language,
        creator_notes,
    })
}

fn encode_map_info(
    writer: &mut Writer,
    file_info: &MapInfo,
    revision: MapInfoRevision,
) -> std::result::Result<(), Error> {
    writer.write_save_string(&file_info.filename);
    writer.write_save_string(&file_info.name);
    writer.write_save_string(&file_info.description);
    writer.write_u16_be(file_info.width);
    writer.write_u16_be(file_info.height);
    writer.write_u8(file_info.difficulty.to_byte());
    let player_slot_count =
        u8::try_from(file_info.player_slots.len()).map_err(|_| Error::InvalidModel {
            field: "player slots",
            message: "player slot count must fit in u8",
        })?;
    writer.write_u8(player_slot_count);

    for slot in &file_info.player_slots {
        writer.write_u8(slot.race.to_byte());
        writer.write_u8(slot.allies.bits());
    }

    writer.write_u8(file_info.kingdom_colors.bits());
    writer.write_u8(file_info.colors_available_for_humans.bits());
    writer.write_u8(file_info.colors_available_for_comp.bits());
    writer.write_u8(file_info.colors_of_random_races.bits());
    writer.write_u8(file_info.victory_condition.kind.to_byte());
    writer.write_byte_from_bool(file_info.victory_condition.comp_also_wins);
    writer.write_byte_from_bool(file_info.victory_condition.allow_normal_victory);
    writer.write_u16_be(file_info.victory_condition.params[0]);
    writer.write_u16_be(file_info.victory_condition.params[1]);
    writer.write_u8(file_info.loss_condition.kind.to_byte());
    writer.write_u16_be(file_info.loss_condition.params[0]);
    writer.write_u16_be(file_info.loss_condition.params[1]);
    writer.write_u32_be(file_info.timestamp);
    writer.write_byte_from_bool(file_info.start_with_hero_in_first_castle);
    writer.write_u32_be(file_info.version.to_u32());
    writer.write_u32_be(file_info.world_date.day);
    writer.write_u32_be(file_info.world_date.week);
    writer.write_u32_be(file_info.world_date.month);
    writer.write_u8(u8::from(file_info.main_language));

    if revision == MapInfoRevision::V10033 {
        match &file_info.creator_notes {
            Some(creator_notes) => writer.write_save_string(creator_notes),
            None => writer.write_u32_be(0),
        }
    }

    Ok(())
}

fn decode_body_bytes(
    reader: &mut Reader<'_>,
) -> std::result::Result<(BodyCompressionHeader, Vec<u8>), Error> {
    reader.set_section(ParseSection::Body);

    let raw_size_offset = reader.position();
    let raw_size = reader.read_u32_be("body raw size")?;
    let raw_size = usize::try_from(raw_size).map_err(|_| {
        reader.invalid_value(
            "body raw size",
            raw_size_offset,
            "body raw size does not fit in usize",
        )
    })?;

    let zip_size_offset = reader.position();
    let zip_size = reader.read_u32_be("body zip size")?;
    let zip_size = usize::try_from(zip_size).map_err(|_| {
        reader.invalid_value(
            "body zip size",
            zip_size_offset,
            "body zip size does not fit in usize",
        )
    })?;

    if zip_size == 0 {
        return Err(reader.invalid_value(
            "body zip size",
            zip_size_offset,
            "body zip size must be non-zero",
        ));
    }

    let compression_version_offset = reader.position();
    let compression_version = reader.read_u16_be("body compression format version")?;
    if compression_version != COMPRESSION_FORMAT_VERSION_0 {
        return Err(reader.unexpected_value(
            "body compression format version",
            compression_version_offset,
            "0",
            compression_version.to_string(),
        ));
    }

    let reserved_offset = reader.position();
    let reserved = reader.read_u16_be("body unused")?;
    if reserved != RESERVED_BYTES_0 {
        return Err(reader.unexpected_value(
            "body unused",
            reserved_offset,
            "0",
            reserved.to_string(),
        ));
    }

    let zlib_bytes_offset = reader.position();
    let zlib_bytes = reader.read_bytes(zip_size, "body zlib bytes")?;
    let mut decoder = ZlibDecoder::new(zlib_bytes);
    let mut body = Vec::with_capacity(raw_size);
    decoder.read_to_end(&mut body).map_err(|_| {
        reader.invalid_value(
            "body zlib bytes",
            zlib_bytes_offset,
            "zlib decompression failed",
        )
    })?;

    if body.len() != raw_size {
        return Err(reader.invalid_value(
            "body decompressed size",
            raw_size_offset,
            "decompressed body size does not match raw size",
        ));
    }

    Ok((
        BodyCompressionHeader {
            raw_size: raw_size as u32,
            zip_size: zip_size as u32,
            compression_format_version: compression_version,
            reserved,
        },
        body,
    ))
}

fn encode_body_bytes(
    writer: &mut Writer,
    body: &[u8],
) -> std::result::Result<BodyCompressionHeader, Error> {
    if body.is_empty() {
        return Err(Error::InvalidModel {
            field: "body",
            message: "body must not be empty",
        });
    }

    let raw_size = u32::try_from(body.len()).map_err(|_| Error::InvalidModel {
        field: "body",
        message: "body length must fit in u32",
    })?;

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(body).map_err(|_| Error::InvalidModel {
        field: "body",
        message: "body compression failed",
    })?;
    let zlib_bytes = encoder.finish().map_err(|_| Error::InvalidModel {
        field: "body",
        message: "body compression failed",
    })?;

    let zip_size = u32::try_from(zlib_bytes.len()).map_err(|_| Error::InvalidModel {
        field: "body",
        message: "compressed body length must fit in u32",
    })?;

    let info = BodyCompressionHeader {
        raw_size,
        zip_size,
        compression_format_version: COMPRESSION_FORMAT_VERSION_0,
        reserved: RESERVED_BYTES_0,
    };

    writer.write_u32_be(info.raw_size);
    writer.write_u32_be(info.zip_size);
    writer.write_u16_be(info.compression_format_version);
    writer.write_u16_be(info.reserved);
    writer.write_bytes(&zlib_bytes);

    Ok(info)
}

#[cfg(test)]
mod tests;
