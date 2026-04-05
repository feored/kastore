use crate::model::PlayerColorsSet;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Difficulty {
    #[default]
    Easy,
    Normal,
    Hard,
    Expert,
    Impossible,
    Unknown(u8),
}

impl From<u8> for Difficulty {
    fn from(value: u8) -> Self {
        match value {
            0 => Difficulty::Easy,
            1 => Difficulty::Normal,
            2 => Difficulty::Hard,
            3 => Difficulty::Expert,
            4 => Difficulty::Impossible,
            other => Difficulty::Unknown(other),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MapInfo {
    pub filename: String,
    pub name: String,
    pub description: String,
    pub width: u16,
    pub height: u16,
    pub difficulty: Difficulty,
    pub kingdom_colors: PlayerColorsSet,
    pub colors_available_for_humans: PlayerColorsSet,
    pub colors_available_for_comp: PlayerColorsSet,
    pub colors_of_random_races: PlayerColorsSet,
}
