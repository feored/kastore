pub mod game_type;
pub mod map_info;
pub mod player;
pub mod supported_language;

/// Outer save header.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SaveHeader {
    /// Whether the save requires Price of Loyalty assets.
    pub requires_pol: bool,
    /// Summary metadata from the outer save header (`HeaderSAV` / `Maps::FileInfo`).
    /// Some of these values are also stored inside the body and may be overridden
    /// when the save is fully loaded by the game.
    pub file_info: map_info::MapInfo,
    /// fheroes2 game type bitfield.
    pub game_type: game_type::GameType,
}
