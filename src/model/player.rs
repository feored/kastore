use std::fmt::Display;

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

impl Race {
    pub const fn from_byte(value: u8) -> Self {
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

    pub const fn to_byte(self) -> u8 {
        match self {
            Race::None => 0x00,
            Race::Knight => 0x01,
            Race::Barbarian => 0x02,
            Race::Sorceress => 0x04,
            Race::Warlock => 0x08,
            Race::Wizard => 0x10,
            Race::Necromancer => 0x20,
            Race::Multi => 0x40,
            Race::Random => 0x80,
            Race::Unknown(value) => value,
        }
    }
}

impl Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Race::None => f.write_str("Neutral"),
            Race::Knight => f.write_str("Knight"),
            Race::Barbarian => f.write_str("Barbarian"),
            Race::Sorceress => f.write_str("Sorceress"),
            Race::Warlock => f.write_str("Warlock"),
            Race::Wizard => f.write_str("Wizard"),
            Race::Necromancer => f.write_str("Necromancer"),
            Race::Multi => f.write_str("Multi"),
            Race::Random => f.write_str("Random"),
            Race::Unknown(value) => write!(f, "Unknown race 0x{value:02X}"),
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
    pub const fn from_index(index: u8) -> Option<Self> {
        match index {
            0 => Some(PlayerColor::Blue),
            1 => Some(PlayerColor::Green),
            2 => Some(PlayerColor::Red),
            3 => Some(PlayerColor::Yellow),
            4 => Some(PlayerColor::Orange),
            5 => Some(PlayerColor::Purple),
            _ => None,
        }
    }

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

impl Display for PlayerColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerColor::None => f.write_str("None"),
            PlayerColor::Blue => f.write_str("Blue"),
            PlayerColor::Green => f.write_str("Green"),
            PlayerColor::Red => f.write_str("Red"),
            PlayerColor::Yellow => f.write_str("Yellow"),
            PlayerColor::Orange => f.write_str("Orange"),
            PlayerColor::Purple => f.write_str("Purple"),
            PlayerColor::Unused => f.write_str("Unused"),
            PlayerColor::Unknown(bits) => write!(f, "Unknown color 0x{bits:02X}"),
        }
    }
}

/// Serialized per-slot payload. Slot identity is derived from position in `MapInfo::player_slots`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PlayerSlotInfo {
    pub race: Race,
    pub allies: PlayerColorsSet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerSlotView {
    pub slot_index: usize,
    pub color: Option<PlayerColor>,
    pub race: Race,
    pub allies: PlayerColorsSet,
}

impl PlayerSlotView {
    pub fn from_stored(slot_index: usize, slot: PlayerSlotInfo) -> Self {
        Self {
            slot_index,
            color: u8::try_from(slot_index)
                .ok()
                .and_then(PlayerColor::from_index),
            race: slot.race,
            allies: slot.allies,
        }
    }
}

impl Display for PlayerSlotView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Some(color) => {
                write!(
                    f,
                    "{color} slot {}: {} race, allies {}",
                    self.slot_index, self.race, self.allies
                )
            }
            None => {
                write!(
                    f,
                    "Slot {}: {} race, allies {}",
                    self.slot_index, self.race, self.allies
                )
            }
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
                colors.push(color.to_string());
            }
        }
        write!(f, "{{{}}}", colors.join(", "))
    }
}
