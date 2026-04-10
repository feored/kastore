//! Public save model types.

mod header;
mod save_game;
mod world;

pub use header::{
    Difficulty, GameType, GameVersion, LossConditionData, LossConditionKind, MapInfo, PlayerColor,
    PlayerColorsSet, PlayerSlotInfo, PlayerSlotView, Race, SaveHeader, SupportedLanguage,
    VictoryConditionData, VictoryConditionKind, WorldDate,
};
pub use save_game::{BodyCompressionHeader, SaveGame};
pub use world::{DirectionSet, LayerType, ObjectPart, Tile, World};
