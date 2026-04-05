use super::SaveContainer;
use crate::Error;
use crate::internal::reader::Reader;
use crate::version::SaveVersion;

pub(crate) const MAGIC_NUMBER: u16 = 0xFF03;

pub(crate) fn decode(bytes: &[u8]) -> std::result::Result<SaveContainer, Error> {
    let mut reader = Reader::new(bytes);
    let magic_number = reader.read_u16_be("magic number")?;

    if magic_number != MAGIC_NUMBER {
        return Err(Error::InvalidContainer("unexpected magic number"));
    }

    let save_version_string = reader.read_string("save version string")?;
    let save_version_number = reader.read_u16_be("save version")?;
    if save_version_string != save_version_number.to_string() {
        return Err(Error::InvalidContainer("save version mismatch"));
    }

    let save_version = match save_version_number {
        10032 => SaveVersion::V10032,
        _ => return Err(Error::UnsupportedSaveVersion),
    };

    Ok(SaveContainer {
        save_version,
        payload: reader.remaining().to_vec(),
    })
}
