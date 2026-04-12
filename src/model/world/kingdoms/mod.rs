use crate::model::header::player::PlayerColor;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Kingdom {
    pub mode: KingdomModeSet,
    pub color: PlayerColor,
}

// fheroes2 kingdom mode bitset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct KingdomModeSet(u32);

impl KingdomModeSet {
    pub const IDENTIFYHERO: Self = Self(0x0000_0002);
    pub const KINGDOM_OVERVIEW_CASTLE_SELECTION: Self = Self(0x0000_0008);

    /// Build from raw hero mode bits.
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Return the raw mode bits.
    pub const fn bits(self) -> u32 {
        self.0
    }
}
