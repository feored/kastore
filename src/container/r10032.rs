use super::{ContainerHeader, DecodedContainer};
use crate::Error;
use crate::internal::reader::Reader;
use crate::model::{Difficulty, MapInfo, PlayerColorsSet};
use crate::version::SaveVersion;

pub(crate) const MAGIC_NUMBER: u16 = 0xFF03;
pub(crate) const REQUIRES_POL: u16 = 0x4000;

pub(crate) fn decode(bytes: &[u8]) -> std::result::Result<DecodedContainer, Error> {
    let mut reader = Reader::new(bytes);
    let magic_number = reader.read_u16_be("magic number")?;

    if magic_number != MAGIC_NUMBER {
        return Err(Error::InvalidContainer("unexpected magic number"));
    }

    let _save_version_string = reader.read_string("save version string")?;
    let save_version_number = reader.read_u16_be("save version")?;

    let save_version = match save_version_number {
        10032 => SaveVersion::V10032,
        _ => return Err(Error::UnsupportedSaveVersion),
    };

    let requires_pol = (reader.read_u16_be("flags")? & REQUIRES_POL) != 0;

    let map_info = MapInfo {
        filename: reader.read_string("map filename")?,
        name: reader.read_string("map name")?,
        description: reader.read_string("map description")?,
        width: reader.read_u16_be("map width")?,
        height: reader.read_u16_be("map height")?,
        difficulty: Difficulty::from(reader.read_u8("map difficulty")?),
        kingdom_colors: PlayerColorsSet::from_bits(reader.read_u8("kingdom colors")?),
        colors_available_for_humans: PlayerColorsSet::from_bits(
            reader.read_u8("colors available for humans")?,
        ),
        colors_available_for_comp: PlayerColorsSet::from_bits(
            reader.read_u8("colors available for computer")?,
        ),
        colors_of_random_races: PlayerColorsSet::from_bits(
            reader.read_u8("colors of random races")?,
        ),
    };

    Ok(DecodedContainer {
        save_version,
        header: ContainerHeader {
            requires_pol,
            map_info,
        },
        payload: reader.remaining().to_vec(),
    })
}
