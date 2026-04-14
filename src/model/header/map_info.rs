use std::fmt::Display;

use crate::SaveString;

use super::player::{PlayerColorsSet, PlayerSlotInfo, PlayerSlotView};
use super::supported_language::SupportedLanguage;

/// Map version marker stored in `Maps::FileInfo`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameVersion {
    #[default]
    SuccessionWars,
    PriceOfLoyalty,
    Resurrection,
    /// Preserved raw value not known by this crate.
    Unknown(u32),
}

impl GameVersion {
    /// Build from the raw save value.
    pub const fn from_u32(value: u32) -> Self {
        match value {
            0 => GameVersion::SuccessionWars,
            1 => GameVersion::PriceOfLoyalty,
            2 => GameVersion::Resurrection,
            other => GameVersion::Unknown(other),
        }
    }

    /// Return the raw save value.
    pub const fn to_u32(self) -> u32 {
        match self {
            GameVersion::SuccessionWars => 0,
            GameVersion::PriceOfLoyalty => 1,
            GameVersion::Resurrection => 2,
            GameVersion::Unknown(other) => other,
        }
    }
}

/// In-game date stored in the save header.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WorldDate {
    /// Day within the current week.
    pub day: u32,
    /// Week within the current month.
    pub week: u32,
    /// Month number.
    pub month: u32,
}

/// Game difficulty stored as a fheroes2 byte value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Difficulty {
    #[default]
    Easy,
    Normal,
    Hard,
    Expert,
    Impossible,
    /// Preserved raw value not known by this crate.
    Unknown(i32),
}

impl Difficulty {
    /// Build from the raw save byte.
    pub const fn from_byte(value: u8) -> Self {
        match value {
            0 => Difficulty::Easy,
            1 => Difficulty::Normal,
            2 => Difficulty::Hard,
            3 => Difficulty::Expert,
            4 => Difficulty::Impossible,
            other => Difficulty::Unknown(other as i32),
        }
    }

    /// Return the raw save byte.
    pub const fn to_byte(self) -> u8 {
        match self {
            Difficulty::Easy => 0,
            Difficulty::Normal => 1,
            Difficulty::Hard => 2,
            Difficulty::Expert => 3,
            Difficulty::Impossible => 4,
            Difficulty::Unknown(other) => other as u8,
        }
    }

    /// Build from the raw i32 wire value used in the settings body.
    pub const fn from_i32(value: i32) -> Self {
        match value {
            0 => Difficulty::Easy,
            1 => Difficulty::Normal,
            2 => Difficulty::Hard,
            3 => Difficulty::Expert,
            4 => Difficulty::Impossible,
            other => Difficulty::Unknown(other),
        }
    }

    /// Return the raw i32 wire value used in the settings body.
    pub const fn to_i32(self) -> i32 {
        match self {
            Difficulty::Easy => 0,
            Difficulty::Normal => 1,
            Difficulty::Hard => 2,
            Difficulty::Expert => 3,
            Difficulty::Impossible => 4,
            Difficulty::Unknown(other) => other,
        }
    }
}

/// Victory condition kind stored in `Maps::FileInfo`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VictoryConditionKind {
    #[default]
    DefeatEveryone,
    CaptureTown,
    KillHero,
    ObtainArtifact,
    DefeatOtherSide,
    CollectEnoughGold,
    /// Preserved raw value not known by this crate.
    Unknown(u8),
}

impl VictoryConditionKind {
    /// Build from the raw save byte.
    pub const fn from_byte(value: u8) -> Self {
        match value {
            0 => VictoryConditionKind::DefeatEveryone,
            1 => VictoryConditionKind::CaptureTown,
            2 => VictoryConditionKind::KillHero,
            3 => VictoryConditionKind::ObtainArtifact,
            4 => VictoryConditionKind::DefeatOtherSide,
            5 => VictoryConditionKind::CollectEnoughGold,
            other => VictoryConditionKind::Unknown(other),
        }
    }

    /// Return the raw save byte.
    pub const fn to_byte(self) -> u8 {
        match self {
            VictoryConditionKind::DefeatEveryone => 0,
            VictoryConditionKind::CaptureTown => 1,
            VictoryConditionKind::KillHero => 2,
            VictoryConditionKind::ObtainArtifact => 3,
            VictoryConditionKind::DefeatOtherSide => 4,
            VictoryConditionKind::CollectEnoughGold => 5,
            VictoryConditionKind::Unknown(other) => other,
        }
    }
}

/// Victory condition data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VictoryConditionData {
    /// Victory condition selector.
    pub kind: VictoryConditionKind,
    /// Whether computer players may also satisfy this condition.
    pub comp_also_wins: bool,
    /// Whether normal defeat-all victory remains enabled.
    pub allow_normal_victory: bool,
    /// Raw condition parameters. Their meaning depends on `kind`.
    pub params: [u16; 2],
}

/// Loss condition kind stored in `Maps::FileInfo`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LossConditionKind {
    #[default]
    LossEverything,
    LossTown,
    LossHero,
    LossOutOfTime,
    /// Preserved raw value not known by this crate.
    Unknown(u8),
}

impl LossConditionKind {
    /// Build from the raw save byte.
    pub const fn from_byte(value: u8) -> Self {
        match value {
            0 => LossConditionKind::LossEverything,
            1 => LossConditionKind::LossTown,
            2 => LossConditionKind::LossHero,
            3 => LossConditionKind::LossOutOfTime,
            other => LossConditionKind::Unknown(other),
        }
    }

    /// Return the raw save byte.
    pub const fn to_byte(self) -> u8 {
        match self {
            LossConditionKind::LossEverything => 0,
            LossConditionKind::LossTown => 1,
            LossConditionKind::LossHero => 2,
            LossConditionKind::LossOutOfTime => 3,
            LossConditionKind::Unknown(other) => other,
        }
    }
}

/// Loss condition data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LossConditionData {
    /// Loss condition selector.
    pub kind: LossConditionKind,
    /// Raw condition parameters. Their meaning depends on `kind`.
    pub params: [u16; 2],
}

/// Summary map metadata from the outer save header.
///
/// fheroes2 also stores map info inside the body settings. The outer copy is
/// useful for listing saves without decoding the full body.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MapInfo {
    /// Source map filename.
    pub filename: SaveString,
    /// Display name.
    pub name: SaveString,
    /// Map description.
    pub description: SaveString,
    /// Creator notes added by save format `10033`.
    pub creator_notes: Option<SaveString>,
    /// Map width in tiles.
    pub width: u16,
    /// Map height in tiles.
    pub height: u16,
    /// Starting difficulty.
    pub difficulty: Difficulty,
    /// Slot order matters: index 0 is blue, 1 is green, and so on.
    pub player_slots: Vec<PlayerSlotInfo>,
    /// Colors with active kingdoms.
    pub kingdom_colors: PlayerColorsSet,
    /// Colors selectable by humans.
    pub colors_available_for_humans: PlayerColorsSet,
    /// Colors selectable by computer players.
    pub colors_available_for_comp: PlayerColorsSet,
    /// Colors whose race is randomized.
    pub colors_of_random_races: PlayerColorsSet,
    /// Victory condition summary.
    pub victory_condition: VictoryConditionData,
    /// Loss condition summary.
    pub loss_condition: LossConditionData,
    /// Raw fheroes2 timestamp.
    pub timestamp: u32,
    /// Whether the first castle starts with a hero.
    pub start_with_hero_in_first_castle: bool,
    /// Map/game version marker.
    pub version: GameVersion,
    /// Current in-game date summary.
    pub world_date: WorldDate,
    /// Main language.
    pub main_language: SupportedLanguage,
}

impl MapInfo {
    /// Return a player slot with its color derived from its stored index.
    pub fn player_slot(&self, slot_index: u8) -> Option<PlayerSlotView> {
        self.player_slots
            .get(usize::from(slot_index))
            .copied()
            .map(|slot| PlayerSlotView::from_stored(usize::from(slot_index), slot))
    }
}

impl Display for MapInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "map filename: {}", self.filename)?;
        writeln!(f, "map name: {}", self.name)?;
        writeln!(f, "description: {}", self.description)?;
        writeln!(f, "width: {}", self.width)?;
        writeln!(f, "height: {}", self.height)?;
        writeln!(f, "difficulty: {:?}", self.difficulty)?;
        writeln!(f, "player slots: {}", self.player_slots.len())?;

        for (index, slot) in self.player_slots.iter().copied().enumerate() {
            let slot = PlayerSlotView::from_stored(index, slot);
            writeln!(f, "{slot}")?;
        }

        writeln!(f, "kingdom colors: {}", self.kingdom_colors)?;
        writeln!(
            f,
            "colors available for humans: {}",
            self.colors_available_for_humans
        )?;
        writeln!(
            f,
            "colors available for computer: {}",
            self.colors_available_for_comp
        )?;
        writeln!(f, "colors of random races: {}", self.colors_of_random_races)?;
        writeln!(f, "victory condition: {:?}", self.victory_condition)?;
        writeln!(f, "loss condition: {:?}", self.loss_condition)?;
        writeln!(f, "timestamp: {}", self.timestamp)?;
        writeln!(
            f,
            "start with hero in first castle: {}",
            self.start_with_hero_in_first_castle
        )?;
        writeln!(f, "version: {:?}", self.version)?;
        writeln!(
            f,
            "world date: day {}, week {}, month {}",
            self.world_date.day, self.world_date.week, self.world_date.month
        )?;
        writeln!(f, "main language: {}", self.main_language)?;
        match &self.creator_notes {
            Some(creator_notes) if creator_notes.is_empty() => {
                writeln!(f, "creator notes: <empty>")?
            }
            Some(creator_notes) => writeln!(f, "creator notes: {}", creator_notes)?,
            None => writeln!(f, "creator notes: <none>")?,
        }
        Ok(())
    }
}
