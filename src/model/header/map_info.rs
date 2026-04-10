use std::fmt::Display;

use crate::SaveString;

use super::{PlayerColorsSet, PlayerSlotInfo, PlayerSlotView, SupportedLanguage};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameVersion {
    #[default]
    SuccessionWars,
    PriceOfLoyalty,
    Resurrection,
    Unknown(u32),
}

impl GameVersion {
    pub const fn from_u32(value: u32) -> Self {
        match value {
            0 => GameVersion::SuccessionWars,
            1 => GameVersion::PriceOfLoyalty,
            2 => GameVersion::Resurrection,
            other => GameVersion::Unknown(other),
        }
    }

    pub const fn to_u32(self) -> u32 {
        match self {
            GameVersion::SuccessionWars => 0,
            GameVersion::PriceOfLoyalty => 1,
            GameVersion::Resurrection => 2,
            GameVersion::Unknown(other) => other,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WorldDate {
    pub day: u32,
    pub week: u32,
    pub month: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Difficulty {
    #[default]
    Easy,
    Normal,
    Hard,
    Expert,
    Impossible,
    Unknown(u8),
}

impl Difficulty {
    pub const fn from_byte(value: u8) -> Self {
        match value {
            0 => Difficulty::Easy,
            1 => Difficulty::Normal,
            2 => Difficulty::Hard,
            3 => Difficulty::Expert,
            4 => Difficulty::Impossible,
            other => Difficulty::Unknown(other),
        }
    }

    pub const fn to_byte(self) -> u8 {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VictoryConditionKind {
    #[default]
    DefeatEveryone,
    CaptureTown,
    KillHero,
    ObtainArtifact,
    DefeatOtherSide,
    CollectEnoughGold,
    Unknown(u8),
}

impl VictoryConditionKind {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VictoryConditionData {
    pub kind: VictoryConditionKind,
    pub comp_also_wins: bool,
    pub allow_normal_victory: bool,
    pub params: [u16; 2],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LossConditionKind {
    #[default]
    LossEverything,
    LossTown,
    LossHero,
    LossOutOfTime,
    Unknown(u8),
}

impl LossConditionKind {
    pub const fn from_byte(value: u8) -> Self {
        match value {
            0 => LossConditionKind::LossEverything,
            1 => LossConditionKind::LossTown,
            2 => LossConditionKind::LossHero,
            3 => LossConditionKind::LossOutOfTime,
            other => LossConditionKind::Unknown(other),
        }
    }

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LossConditionData {
    pub kind: LossConditionKind,
    pub params: [u16; 2],
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MapInfo {
    pub filename: SaveString,
    pub name: SaveString,
    pub description: SaveString,
    pub creator_notes: Option<SaveString>,
    pub width: u16,
    pub height: u16,
    pub difficulty: Difficulty,
    /// Slot order matters: index 0 is blue, 1 is green, and so on.
    pub player_slots: Vec<PlayerSlotInfo>,
    pub kingdom_colors: PlayerColorsSet,
    pub colors_available_for_humans: PlayerColorsSet,
    pub colors_available_for_comp: PlayerColorsSet,
    pub colors_of_random_races: PlayerColorsSet,
    pub victory_condition: VictoryConditionData,
    pub loss_condition: LossConditionData,
    pub timestamp: u32,
    pub start_with_hero_in_first_castle: bool,
    pub version: GameVersion,
    pub world_date: WorldDate,
    pub main_language: SupportedLanguage,
}

impl MapInfo {
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
