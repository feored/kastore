mod map_info;

use crate::Error;
use crate::SaveVersion;
use crate::internal::error::ParseSection;
use crate::internal::reader::Reader;
use crate::internal::writer::Writer;
use crate::model::{GameType, MapInfo};
use crate::version::VersionProfile;

pub(crate) const MAGIC_NUMBER: u16 = 0xFF03;
pub(crate) const REQUIRES_POL: u16 = 0x4000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ContainerVersion {
    save_version: SaveVersion,
    save_version_offset: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct ContainerParts {
    pub(crate) requires_pol: bool,
    pub(crate) map_info: MapInfo,
    pub(crate) game_type: GameType,
    pub(crate) payload: Vec<u8>,
}

fn expect_magic(reader: &mut Reader<'_>) -> std::result::Result<(), Error> {
    let magic_offset = reader.position();
    let magic_number = reader.read_u16_be("magic number")?;

    if magic_number != MAGIC_NUMBER {
        return Err(reader.unexpected_value(
            "magic number",
            magic_offset,
            "0xFF03",
            format!("0x{magic_number:04X}"),
        ));
    }

    Ok(())
}

fn encode_magic(writer: &mut crate::internal::writer::Writer) {
    writer.write_u16_be(MAGIC_NUMBER);
}

fn decode_version(reader: &mut Reader<'_>) -> std::result::Result<ContainerVersion, Error> {
    let _save_version_string = reader.read_string_bytes("save version string")?;
    let save_version_offset = reader.position();
    let save_version = SaveVersion::from_u16(reader.read_u16_be("save version")?);

    Ok(ContainerVersion {
        save_version,
        save_version_offset,
    })
}

fn encode_version(writer: &mut crate::internal::writer::Writer, version: SaveVersion) {
    let version_string = version.as_u16().to_string();
    writer.write_u32_be(version_string.len() as u32);
    writer.write_bytes(version_string.as_bytes());
    writer.write_u16_be(version.as_u16());
}

pub(crate) fn detect_save_version(bytes: &[u8]) -> std::result::Result<SaveVersion, Error> {
    let mut reader = Reader::with_context(bytes, ParseSection::Container);
    expect_magic(&mut reader)?;
    Ok(decode_version(&mut reader)?.save_version)
}

pub(crate) fn decode_container(
    bytes: &[u8],
    profile: VersionProfile,
) -> std::result::Result<ContainerParts, Error> {
    let mut reader = Reader::with_context(bytes, ParseSection::Container);
    expect_magic(&mut reader)?;
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
    let map_info = map_info::decode(&mut reader, profile.map_info_revision)?;
    let game_type = GameType::from_i32(reader.read_i32_be("game type")?);

    Ok(ContainerParts {
        requires_pol,
        map_info,
        game_type,
        payload: reader.remaining().to_vec(),
    })
}

pub(crate) fn encode_container(
    parts: &ContainerParts,
    profile: VersionProfile,
) -> std::result::Result<Vec<u8>, Error> {
    let mut writer = Writer::new();
    encode_magic(&mut writer);
    encode_version(&mut writer, profile.save_version);
    writer.write_u16_be(if parts.requires_pol { REQUIRES_POL } else { 0 });
    map_info::encode(&mut writer, &parts.map_info, profile.map_info_revision)?;
    writer.write_i32_be(parts.game_type.to_i32());
    writer.write_bytes(&parts.payload);
    Ok(writer.into_bytes())
}

#[cfg(test)]
mod tests;
