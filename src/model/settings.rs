use std::fmt::Display;

use crate::SaveString;
use crate::model::header::game_type::GameType;
use crate::model::header::map_info::{Difficulty, MapInfo};
use crate::model::header::player::{PlayerColor, PlayerColorsSet, Race};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Settings {
    pub loaded_file_language: SaveString,
    pub current_map_info: MapInfo,
    pub game_difficulty: Difficulty,
    pub game_type: GameType,
    pub players: SettingsPlayers,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SettingsPlayers {
    pub colors: PlayerColorsSet,
    pub current_player_color: PlayerColor,
    pub entries: Vec<SettingsPlayer>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SettingsPlayer {
    pub modes: u32,
    pub control: SettingsPlayerControl,
    pub color: PlayerColor,
    pub race: Race,
    pub friends: PlayerColorsSet,
    pub name: SaveString,
    pub focus: SettingsFocus,
    pub ai_personality: SettingsAiPersonality,
    pub handicap_status: SettingsHandicapStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SettingsPlayerControl {
    #[default]
    None,
    Human,
    Ai,
    HumanAi,
    Unknown(i32),
}

impl SettingsPlayerControl {
    pub const fn from_i32(value: i32) -> Self {
        match value {
            0 => SettingsPlayerControl::None,
            1 => SettingsPlayerControl::Human,
            4 => SettingsPlayerControl::Ai,
            5 => SettingsPlayerControl::HumanAi,
            other => SettingsPlayerControl::Unknown(other),
        }
    }

    pub const fn to_i32(self) -> i32 {
        match self {
            SettingsPlayerControl::None => 0,
            SettingsPlayerControl::Human => 1,
            SettingsPlayerControl::Ai => 4,
            SettingsPlayerControl::HumanAi => 5,
            SettingsPlayerControl::Unknown(other) => other,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SettingsFocus {
    pub kind: SettingsFocusKind,
    pub tile_index: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SettingsFocusKind {
    #[default]
    None,
    Hero,
    Castle,
    Unknown(i32),
}

impl SettingsFocusKind {
    pub const fn from_i32(value: i32) -> Self {
        match value {
            0 => SettingsFocusKind::None,
            1 => SettingsFocusKind::Hero,
            2 => SettingsFocusKind::Castle,
            other => SettingsFocusKind::Unknown(other),
        }
    }

    pub const fn to_i32(self) -> i32 {
        match self {
            SettingsFocusKind::None => 0,
            SettingsFocusKind::Hero => 1,
            SettingsFocusKind::Castle => 2,
            SettingsFocusKind::Unknown(other) => other,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SettingsAiPersonality {
    #[default]
    None,
    Warrior,
    Builder,
    Explorer,
    Unknown(i32),
}

impl SettingsAiPersonality {
    pub const fn from_i32(value: i32) -> Self {
        match value {
            0 => SettingsAiPersonality::None,
            1 => SettingsAiPersonality::Warrior,
            2 => SettingsAiPersonality::Builder,
            3 => SettingsAiPersonality::Explorer,
            other => SettingsAiPersonality::Unknown(other),
        }
    }

    pub const fn to_i32(self) -> i32 {
        match self {
            SettingsAiPersonality::None => 0,
            SettingsAiPersonality::Warrior => 1,
            SettingsAiPersonality::Builder => 2,
            SettingsAiPersonality::Explorer => 3,
            SettingsAiPersonality::Unknown(other) => other,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SettingsHandicapStatus {
    #[default]
    None,
    Mild,
    Severe,
    Unknown(u8),
}

impl SettingsHandicapStatus {
    pub const fn from_u8(value: u8) -> Self {
        match value {
            0 => SettingsHandicapStatus::None,
            1 => SettingsHandicapStatus::Mild,
            2 => SettingsHandicapStatus::Severe,
            other => SettingsHandicapStatus::Unknown(other),
        }
    }

    pub const fn to_u8(self) -> u8 {
        match self {
            SettingsHandicapStatus::None => 0,
            SettingsHandicapStatus::Mild => 1,
            SettingsHandicapStatus::Severe => 2,
            SettingsHandicapStatus::Unknown(other) => other,
        }
    }
}

impl Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "loaded_file_language: {:?}",
            self.loaded_file_language.to_string_lossy()
        )?;
        writeln!(f, "game_difficulty: {:?}", self.game_difficulty)?;
        writeln!(f, "game_type: {}", self.game_type)?;
        writeln!(f, "players colors: {}", self.players.colors)?;
        writeln!(
            f,
            "players current color: {}",
            self.players.current_player_color
        )?;
        writeln!(f, "players entries: {}", self.players.entries.len())?;

        for player in &self.players.entries {
            writeln!(f, "  - {player}")?;
        }

        Ok(())
    }
}

impl Display for SettingsPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({:?}), control={:?}, allies={}, focus={:?}#{}, ai={:?}, handicap={:?}",
            if self.name.is_empty() {
                self.color.to_string()
            } else {
                self.name.to_string_lossy()
            },
            self.race,
            self.control,
            self.friends,
            self.focus.kind,
            self.focus.tile_index,
            self.ai_personality,
            self.handicap_status
        )
    }
}
