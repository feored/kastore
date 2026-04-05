use std::fmt::Display;

pub const MAX_PLAYERS: usize = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Race {
    #[default]
    None,
    Knight,
    Barbarian,
    Sorceress,
    Warlock,
    Wizard,
    Necromancer,
    Multi,
    Random,
    Unknown(u8),
}

impl From<u8> for Race {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Race::None,
            0x01 => Race::Knight,
            0x02 => Race::Barbarian,
            0x04 => Race::Sorceress,
            0x08 => Race::Warlock,
            0x10 => Race::Wizard,
            0x20 => Race::Necromancer,
            0x40 => Race::Multi,
            0x80 => Race::Random,
            other => Race::Unknown(other),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlayerColor {
    #[default]
    None,
    Blue,
    Green,
    Red,
    Yellow,
    Orange,
    Purple,
    Unused,
    Unknown(u8),
}

impl From<u8> for PlayerColor {
    fn from(value: u8) -> Self {
        PlayerColor::from_bits(value)
    }
}

impl PlayerColor {
    pub const fn from_bits(value: u8) -> Self {
        match value {
            0x00 => PlayerColor::None,
            0x01 => PlayerColor::Blue,
            0x02 => PlayerColor::Green,
            0x04 => PlayerColor::Red,
            0x08 => PlayerColor::Yellow,
            0x10 => PlayerColor::Orange,
            0x20 => PlayerColor::Purple,
            0x80 => PlayerColor::Unused,
            other => PlayerColor::Unknown(other),
        }
    }

    pub const fn bits(self) -> u8 {
        match self {
            PlayerColor::None => 0x00,
            PlayerColor::Blue => 0x01,
            PlayerColor::Green => 0x02,
            PlayerColor::Red => 0x04,
            PlayerColor::Yellow => 0x08,
            PlayerColor::Orange => 0x10,
            PlayerColor::Purple => 0x20,
            PlayerColor::Unused => 0x80,
            PlayerColor::Unknown(bits) => bits,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PlayerColorsSet(u8);

impl PlayerColorsSet {
    pub const EMPTY: Self = Self(0);

    pub const fn from_bits(bits: u8) -> Self {
        Self(bits)
    }

    pub const fn bits(self) -> u8 {
        self.0
    }

    pub const fn from_color(color: PlayerColor) -> Self {
        Self(color.bits())
    }

    pub const fn contains(self, color: PlayerColor) -> bool {
        (self.0 & color.bits()) != 0
    }

    pub fn insert(&mut self, color: PlayerColor) {
        self.0 |= color.bits();
    }

    pub fn remove(&mut self, color: PlayerColor) {
        self.0 &= !(color.bits());
    }
}

impl Display for PlayerColorsSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut colors = Vec::new();
        for &color in [
            PlayerColor::Blue,
            PlayerColor::Green,
            PlayerColor::Red,
            PlayerColor::Yellow,
            PlayerColor::Orange,
            PlayerColor::Purple,
            PlayerColor::Unused,
        ]
        .iter()
        {
            if self.contains(color) {
                colors.push(format!("{:?}", color));
            }
        }
        write!(f, "{{{}}}", colors.join(", "))
    }
}
