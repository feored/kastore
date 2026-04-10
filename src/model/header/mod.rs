mod game_type;
mod map_info;
mod player;
mod supported_language;

pub use game_type::GameType;
pub use map_info::{
    Difficulty, GameVersion, LossConditionData, LossConditionKind, MapInfo, VictoryConditionData,
    VictoryConditionKind, WorldDate,
};
pub use player::{PlayerColor, PlayerColorsSet, PlayerSlotInfo, PlayerSlotView, Race};
pub use supported_language::SupportedLanguage;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SaveHeader {
    pub requires_pol: bool,
    /// Summary metadata from the outer save header (`HeaderSAV` / `Maps::FileInfo`).
    /// Some of these values are also stored inside the body and may be overridden
    /// when the save is fully loaded by the game.
    pub file_info: MapInfo,
    pub game_type: GameType,
}
