use std::fmt::Display;

use crate::model::header::player::PlayerColorsSet;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GameOverResult {
    pub active_colors: PlayerColorsSet,
    pub result: GameOverResultSet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GameOverResultSet(u32);

impl GameOverResultSet {
    pub const EMPTY: Self = Self(0x0000_0000);
    pub const WINS_ALL: Self = GameOverResultSet(0x0000_0001);
    pub const WINS_TOWN: Self = GameOverResultSet(0x0000_0002);
    pub const WINS_HERO: Self = GameOverResultSet(0x0000_0004);
    pub const WINS_ARTIFACT: Self = GameOverResultSet(0x0000_0008);
    pub const WINS_SIDE: Self = GameOverResultSet(0x0000_0010);
    pub const WINS_GOLD: Self = GameOverResultSet(0x0000_0020);
    pub const LOSS_ALL: Self = GameOverResultSet(0x0000_0100);
    pub const LOSS_TOWN: Self = GameOverResultSet(0x0000_0200);
    pub const LOSS_HERO: Self = GameOverResultSet(0x0000_0400);
    pub const LOSS_TIME: Self = GameOverResultSet(0x0000_0800);
    pub const LOSS_ENEMY_WINS_TOWN: Self = GameOverResultSet(0x0001_0000);
    pub const LOSS_ENEMY_WINS_ARTIFACT: Self = GameOverResultSet(0x0002_0000);
    pub const LOSS_ENEMY_WINS_GOLD: Self = GameOverResultSet(0x0004_0000);

    /// Build from raw game over result mode bits.
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Return the raw mode bits.
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Return whether all bits in `flags` are set.
    pub const fn contains(self, flags: Self) -> bool {
        (self.0 & flags.0) == flags.0
    }

    /// Set all bits in `flags`.
    pub fn insert(&mut self, flags: Self) {
        self.0 |= flags.0;
    }

    /// Clear all bits in `flags`.
    pub fn remove(&mut self, flags: Self) {
        self.0 &= !flags.0;
    }
}

impl Display for GameOverResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "  active colors: {}", self.active_colors)?;
        writeln!(f, "  result: {:#010x}", self.result.bits())
    }
}
