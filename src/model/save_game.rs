use std::fmt::Display;

use crate::model::{GameType, MapInfo};
use crate::version::SaveVersion;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SaveHeader {
    pub requires_pol: bool,
    pub map_info: MapInfo,
    pub game_type: GameType,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SaveGame {
    pub source_version: SaveVersion,
    pub header: SaveHeader,
    pub payload: Vec<u8>,
}

impl Display for SaveGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "save version: {}", self.source_version)?;
        writeln!(f, "game type: {}", self.header.game_type)?;
        writeln!(f, "requires_pol: {}", self.header.requires_pol)?;
        writeln!(f, "payload bytes: {}", self.payload.len())?;
        write!(f, "{}", self.header.map_info)
    }
}
