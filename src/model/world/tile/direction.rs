use std::fmt::Display;

/// fheroes2 direction bitset.
///
/// In tile records this is also the passability mask.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DirectionSet(u16);

impl DirectionSet {
    /// No direction bits set.
    pub const EMPTY: Self = Self(0x0000);
    pub const TOP_LEFT: Self = Self(0x0001);
    pub const TOP: Self = Self(0x0002);
    pub const TOP_RIGHT: Self = Self(0x0004);
    pub const RIGHT: Self = Self(0x0008);
    pub const BOTTOM_RIGHT: Self = Self(0x0010);
    pub const BOTTOM: Self = Self(0x0020);
    pub const BOTTOM_LEFT: Self = Self(0x0040);
    pub const LEFT: Self = Self(0x0080);
    pub const CENTER: Self = Self(0x0100);

    /// Build from the raw direction bitset.
    pub const fn from_bits(bits: u16) -> Self {
        Self(bits)
    }

    /// Return the raw direction bitset.
    pub const fn bits(self) -> u16 {
        self.0
    }

    /// Return whether all bits in `directions` are set.
    pub const fn contains(self, directions: Self) -> bool {
        (self.0 & directions.0) == directions.0
    }

    /// Set all bits in `directions`.
    pub fn insert(&mut self, directions: Self) {
        self.0 |= directions.0;
    }

    /// Clear all bits in `directions`.
    pub fn remove(&mut self, directions: Self) {
        self.0 &= !directions.0;
    }
}

impl Display for DirectionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == Self::EMPTY.0 {
            return f.write_str("Blocked (0x0000)");
        }

        let mut parts = Vec::new();
        for &(direction, name) in [
            (Self::TOP_LEFT, "TopLeft"),
            (Self::TOP, "Top"),
            (Self::TOP_RIGHT, "TopRight"),
            (Self::RIGHT, "Right"),
            (Self::BOTTOM_RIGHT, "BottomRight"),
            (Self::BOTTOM, "Bottom"),
            (Self::BOTTOM_LEFT, "BottomLeft"),
            (Self::LEFT, "Left"),
            (Self::CENTER, "Center"),
        ]
        .iter()
        {
            if self.contains(direction) {
                parts.push(name);
            }
        }

        write!(f, "{} (0x{:04X})", parts.join(" | "), self.0)
    }
}
