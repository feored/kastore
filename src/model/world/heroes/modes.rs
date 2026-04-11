use std::fmt::Display;

/// fheroes2 hero mode bitset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct HeroModeSet(u32);

impl HeroModeSet {
    pub const EMPTY: Self = Self(0x0000_0000);
    pub const SHIPMASTER: Self = Self(0x0000_0001);
    pub const SPELLCASTED: Self = Self(0x0000_0004);
    pub const ENABLEMOVE: Self = Self(0x0000_0008);
    pub const RECRUIT: Self = Self(0x0000_0040);
    pub const JAIL: Self = Self(0x0000_0080);
    pub const ACTION: Self = Self(0x0000_0100);
    pub const SAVEMP: Self = Self(0x0000_0200);
    pub const SLEEPER: Self = Self(0x0000_0400);
    pub const CUSTOM: Self = Self(0x0000_1000);
    pub const NOTDISMISS: Self = Self(0x0000_2000);
    pub const VISIONS: Self = Self(0x0000_4000);
    pub const PATROL: Self = Self(0x0000_8000);

    /// Build from raw hero mode bits.
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

impl Display for HeroModeSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == Self::EMPTY.0 {
            return f.write_str("None (0x00000000)");
        }

        let mut parts = Vec::new();
        for &(flag, name) in [
            (Self::SHIPMASTER, "Shipmaster"),
            (Self::SPELLCASTED, "Spellcasted"),
            (Self::ENABLEMOVE, "EnableMove"),
            (Self::RECRUIT, "Recruit"),
            (Self::JAIL, "Jail"),
            (Self::ACTION, "Action"),
            (Self::SAVEMP, "SaveMp"),
            (Self::SLEEPER, "Sleeper"),
            (Self::CUSTOM, "Custom"),
            (Self::NOTDISMISS, "NotDismiss"),
            (Self::VISIONS, "Visions"),
            (Self::PATROL, "Patrol"),
        ]
        .iter()
        {
            if self.contains(flag) {
                parts.push(name);
            }
        }

        write!(f, "{} (0x{:08X})", parts.join(" | "), self.0)
    }
}
