use std::fmt::Display;

/// fheroes2 game type bitfield.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GameType(i32);

impl GameType {
    const KNOWN_BITS: i32 = Self::STANDARD.0
        | Self::CAMPAIGN.0
        | Self::HOTSEAT.0
        | Self::NETWORK.0
        | Self::BATTLE_ONLY.0
        | Self::LOAD_FILE.0;

    pub const MENU: Self = Self(0);
    pub const STANDARD: Self = Self(0x01);
    pub const CAMPAIGN: Self = Self(0x02);
    pub const HOTSEAT: Self = Self(0x04);
    pub const NETWORK: Self = Self(0x08);
    pub const BATTLE_ONLY: Self = Self(0x10);
    pub const LOAD_FILE: Self = Self(0x80);
    pub const MULTI: Self = Self::HOTSEAT;

    /// Build from the raw save value.
    pub const fn from_i32(value: i32) -> Self {
        Self(value)
    }

    /// Return the raw save value.
    pub const fn to_i32(self) -> i32 {
        self.0
    }

    /// Return whether all bits in `flags` are set.
    pub const fn contains(self, flags: Self) -> bool {
        (self.0 & flags.0) == flags.0
    }

    /// Return any bits not currently recognized by this crate.
    pub const fn unknown_bits(self) -> i32 {
        self.0 & !Self::KNOWN_BITS
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

impl Display for GameType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == 0 {
            return f.write_str("Menu (0x00000000)");
        }

        let mut parts = Vec::new();
        if self.contains(Self::STANDARD) {
            parts.push("Standard");
        }
        if self.contains(Self::CAMPAIGN) {
            parts.push("Campaign");
        }
        if self.contains(Self::HOTSEAT) {
            parts.push("Hotseat");
        }
        if self.contains(Self::NETWORK) {
            parts.push("Network");
        }
        if self.contains(Self::BATTLE_ONLY) {
            parts.push("BattleOnly");
        }
        if self.contains(Self::LOAD_FILE) {
            parts.push("LoadFile");
        }

        let unknown_bits = self.unknown_bits();
        if unknown_bits != 0 {
            return write!(
                f,
                "{} (0x{:08X}, unknown 0x{:08X})",
                parts.join(" | "),
                self.0,
                unknown_bits
            );
        }

        write!(f, "{} (0x{:08X})", parts.join(" | "), self.0)
    }
}
