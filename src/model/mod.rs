mod game_type;
mod map_info;
mod player;
mod save_game;
mod supported_language;

pub use game_type::GameType;
pub use map_info::{
    Difficulty, GameVersion, LossConditionData, LossConditionKind, MapInfo, VictoryConditionData,
    VictoryConditionKind, WorldDate,
};
pub use player::{PlayerColor, PlayerColorsSet, PlayerSlotInfo, PlayerSlotView, Race};
pub use save_game::{BodyCompressionHeader, SaveGame, SaveHeader};
pub use supported_language::SupportedLanguage;
