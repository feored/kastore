use std::fmt::Display;

use crate::version::SaveVersion;

use super::{SaveHeader, World};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BodyCompressionHeader {
    pub raw_size: u32,
    pub zip_size: u32,
    pub compression_format_version: u16,
    pub reserved: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SaveGame {
    pub source_version: SaveVersion,
    pub header: SaveHeader,
    /// Metadata from the compressed body wrapper in the save file.
    pub compression_header: BodyCompressionHeader,
    /// Decompressed gameplay body bytes from the save file's compressed body chunk.
    pub body: Vec<u8>,
    /// Decoded body world data. Encoding still uses `body` until model-driven body writing exists.
    pub world: World,
}

impl Display for SaveGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "save version: {}", self.source_version)?;
        writeln!(f, "requires_pol: {}", self.header.requires_pol)?;
        write!(f, "{}", self.header.file_info)?;
        writeln!(f, "game type: {}", self.header.game_type)?;
        writeln!(f, "body bytes (decompressed): {}", self.body.len())?;
        writeln!(
            f,
            "body wrapper raw size: {}",
            self.compression_header.raw_size
        )?;
        writeln!(
            f,
            "body wrapper zip size: {}",
            self.compression_header.zip_size
        )?;
        writeln!(
            f,
            "body wrapper compression format version: {}",
            self.compression_header.compression_format_version
        )?;
        writeln!(
            f,
            "body wrapper reserved: {}",
            self.compression_header.reserved
        )?;
        writeln!(
            f,
            "body has end marker 0xFF03: {}",
            self.body.ends_with(&[0xFF, 0x03])
        )?;
        write!(f, "{}", self.world)
    }
}
