use std::fmt::Display;

use crate::model::world::heroes::army::Troop;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CampaignSaveData {
    pub current_scenario_info: ScenarioInfoId,
    pub current_scenario_bonus_id: i32,
    pub finished_maps: Vec<ScenarioInfoId>,
    pub bonuses_for_finished_maps: Vec<i32>,
    pub days_passed_per_finished_map: Vec<u32>,
    pub obtained_campaign_awards: Vec<i32>,
    pub carry_over_troops: Vec<Troop>,
    pub difficulty: i32,
    pub min_difficulty: i32,
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ScenarioInfoId {
    pub campaign_id: i32,
    pub scenario_id: i32,
}

impl Display for CampaignSaveData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "  current scenario: campaign={} scenario={}",
            self.current_scenario_info.campaign_id, self.current_scenario_info.scenario_id
        )?;
        writeln!(
            f,
            "  current scenario bonus id: {}",
            self.current_scenario_bonus_id
        )?;
        writeln!(f, "  finished maps: {}", self.finished_maps.len())?;
        writeln!(
            f,
            "  finished map bonuses: {}",
            self.bonuses_for_finished_maps.len()
        )?;
        writeln!(
            f,
            "  finished map days entries: {}",
            self.days_passed_per_finished_map.len()
        )?;
        writeln!(
            f,
            "  obtained campaign awards: {}",
            self.obtained_campaign_awards.len()
        )?;
        writeln!(f, "  carry-over troops: {}", self.carry_over_troops.len())?;
        writeln!(f, "  difficulty: {}", self.difficulty)?;
        writeln!(f, "  min difficulty: {}", self.min_difficulty)
    }
}
