use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DirectionSet(u16);

impl DirectionSet {
    pub const UNKNOWN: Self = Self(0x0000);
    pub const TOP_LEFT: Self = Self(0x0001);
    pub const TOP: Self = Self(0x0002);
    pub const TOP_RIGHT: Self = Self(0x0004);
    pub const RIGHT: Self = Self(0x0008);
    pub const BOTTOM_RIGHT: Self = Self(0x0010);
    pub const BOTTOM: Self = Self(0x0020);
    pub const BOTTOM_LEFT: Self = Self(0x0040);
    pub const LEFT: Self = Self(0x0080);
    pub const CENTER: Self = Self(0x0100);

    pub const fn from_bits(bits: u16) -> Self {
        Self(bits)
    }

    pub const fn bits(self) -> u16 {
        self.0
    }

    pub const fn contains(self, directions: Self) -> bool {
        (self.0 & directions.0) == directions.0
    }

    pub fn insert(&mut self, directions: Self) {
        self.0 |= directions.0;
    }

    pub fn remove(&mut self, directions: Self) {
        self.0 &= !directions.0;
    }
}

impl Display for DirectionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == Self::UNKNOWN.0 {
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
