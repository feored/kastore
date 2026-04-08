use std::fmt::Display;

use crate::model::{GameType, MapInfo};
use crate::version::SaveVersion;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PayloadCompressionHeader {
    pub raw_size: u32,
    pub zip_size: u32,
    pub compression_format_version: u16,
    pub reserved: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SaveHeader {
    pub requires_pol: bool,
    /// Summary metadata from the outer save header (`HeaderSAV` / `Maps::FileInfo`).
    /// Some of these values are also stored inside the payload and may be overridden
    /// when the save is fully loaded by the game.
    pub file_info: MapInfo,
    pub game_type: GameType,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SaveGame {
    pub source_version: SaveVersion,
    pub header: SaveHeader,
    /// Metadata from the compressed payload wrapper in the save container.
    pub payload_compression_header: PayloadCompressionHeader,
    /// Decompressed gameplay payload bytes from the save's compressed payload chunk.
    pub payload: Vec<u8>,
}

impl Display for SaveGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "save version: {}", self.source_version)?;
        writeln!(f, "requires_pol: {}", self.header.requires_pol)?;
        write!(f, "{}", self.header.file_info)?;
        writeln!(f, "game type: {}", self.header.game_type)?;
        writeln!(f, "payload bytes (decompressed): {}", self.payload.len())?;
        writeln!(
            f,
            "payload wrapper raw size: {}",
            self.payload_compression_header.raw_size
        )?;
        writeln!(
            f,
            "payload wrapper zip size: {}",
            self.payload_compression_header.zip_size
        )?;
        writeln!(
            f,
            "payload wrapper compression format version: {}",
            self.payload_compression_header
                .compression_format_version
        )?;
        writeln!(
            f,
            "payload wrapper reserved: {}",
            self.payload_compression_header.reserved
        )?;
        writeln!(
            f,
            "payload has end marker 0xFF03: {}",
            self.payload.ends_with(&[0xFF, 0x03])
        )
    }
}
