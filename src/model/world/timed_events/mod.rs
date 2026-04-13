use std::fmt::Display;

use crate::SaveString;
use crate::model::header::player::PlayerColorsSet;
use crate::model::world::Funds;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TimedEvent {
    pub resources: Funds,
    pub is_applicable_for_ai_players: bool,
    pub first_occurrence_day: u32,
    pub repeat_period_in_days: u32,
    pub colors: PlayerColorsSet,
    pub message: SaveString,
    pub title: SaveString,
}

impl Display for TimedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let resources = self.resources.brief();
        write!(
            f,
            "title={}, day={}, repeat={}, ai={}, colors={}, resources={}",
            self.title.brief(48),
            self.first_occurrence_day,
            self.repeat_period_in_days,
            self.is_applicable_for_ai_players,
            self.colors,
            if resources.is_empty() {
                "none"
            } else {
                &resources
            }
        )?;

        if !self.message.is_empty() {
            write!(f, ", message={}", self.message.brief(72))?;
        }

        Ok(())
    }
}
