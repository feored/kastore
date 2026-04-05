use crate::model::MapInfo;
use crate::version::SaveVersion;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SaveHeader {
    pub requires_pol: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SaveGame {
    pub source_version: SaveVersion,
    pub header: SaveHeader,
    pub map_info: MapInfo,
}
