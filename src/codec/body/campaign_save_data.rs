use crate::Error;
use crate::internal::error::ParseSection;
use crate::internal::reader::Reader;
use crate::internal::writer::Writer;
use crate::model::campaign_save_data::{CampaignSaveData, ScenarioInfoId};
use crate::model::world::heroes::army::{MonsterType, Troop};

pub(crate) fn decode(
    reader: &mut Reader<'_>,
) -> std::result::Result<CampaignSaveData, Error> {
    reader.set_section(ParseSection::Campaign);
    let current_scenario_info = ScenarioInfoId {
        campaign_id: reader.read_i32_be("current scenario campaign id")?,
        scenario_id: reader.read_i32_be("current scenario scenario id")?,
    };
    let current_scenario_bonus_id = reader.read_i32_be("current scenario bonus id")?;

    let finished_maps_count_offset = reader.position();
    let finished_maps_count = reader.read_u32_be("finished maps")?;
    let finished_maps_count = usize::try_from(finished_maps_count).map_err(|_| {
        reader.invalid_value(
            "finished maps",
            finished_maps_count_offset,
            "vector count does not fit in usize",
        )
    })?;
    let mut finished_maps = Vec::with_capacity(finished_maps_count);
    for _ in 0..finished_maps_count {
        finished_maps.push(ScenarioInfoId {
            campaign_id: reader.read_i32_be("finished map campaign id")?,
            scenario_id: reader.read_i32_be("finished map scenario id")?,
        });
    }

    let bonuses_count_offset = reader.position();
    let bonuses_count = reader.read_u32_be("finished map bonuses")?;
    let bonuses_count = usize::try_from(bonuses_count).map_err(|_| {
        reader.invalid_value(
            "finished map bonuses",
            bonuses_count_offset,
            "vector count does not fit in usize",
        )
    })?;
    let mut bonuses_for_finished_maps = Vec::with_capacity(bonuses_count);
    for _ in 0..bonuses_count {
        bonuses_for_finished_maps.push(reader.read_i32_be("finished map bonus")?);
    }

    let days_count_offset = reader.position();
    let days_count = reader.read_u32_be("finished map days")?;
    let days_count = usize::try_from(days_count).map_err(|_| {
        reader.invalid_value(
            "finished map days",
            days_count_offset,
            "vector count does not fit in usize",
        )
    })?;
    let mut days_passed_per_finished_map = Vec::with_capacity(days_count);
    for _ in 0..days_count {
        days_passed_per_finished_map.push(reader.read_u32_be("finished map days entry")?);
    }

    let awards_count_offset = reader.position();
    let awards_count = reader.read_u32_be("obtained campaign awards")?;
    let awards_count = usize::try_from(awards_count).map_err(|_| {
        reader.invalid_value(
            "obtained campaign awards",
            awards_count_offset,
            "vector count does not fit in usize",
        )
    })?;
    let mut obtained_campaign_awards = Vec::with_capacity(awards_count);
    for _ in 0..awards_count {
        obtained_campaign_awards.push(reader.read_i32_be("obtained campaign award")?);
    }

    let troops_count_offset = reader.position();
    let troops_count = reader.read_u32_be("carry-over troops")?;
    let troops_count = usize::try_from(troops_count).map_err(|_| {
        reader.invalid_value(
            "carry-over troops",
            troops_count_offset,
            "vector count does not fit in usize",
        )
    })?;
    let mut carry_over_troops = Vec::with_capacity(troops_count);
    for _ in 0..troops_count {
        carry_over_troops.push(Troop {
            monster: MonsterType::from_i32(reader.read_i32_be("campaign troop monster")?),
            count: reader.read_u32_be("campaign troop count")?,
        });
    }

    let difficulty = reader.read_i32_be("campaign difficulty")?;
    let min_difficulty = reader.read_i32_be("campaign min difficulty")?;

    Ok(CampaignSaveData {
        current_scenario_info,
        current_scenario_bonus_id,
        finished_maps,
        bonuses_for_finished_maps,
        days_passed_per_finished_map,
        obtained_campaign_awards,
        carry_over_troops,
        difficulty,
        min_difficulty,
    })
}

pub(crate) fn encode(
    writer: &mut Writer,
    campaign_save_data: &CampaignSaveData,
) -> std::result::Result<(), Error> {
    if campaign_save_data.bonuses_for_finished_maps.len() != campaign_save_data.finished_maps.len()
    {
        return Err(Error::InvalidModel {
            field: "campaign bonuses_for_finished_maps",
            message: "bonuses_for_finished_maps length must match finished_maps length",
        });
    }
    if campaign_save_data.days_passed_per_finished_map.len()
        != campaign_save_data.finished_maps.len()
    {
        return Err(Error::InvalidModel {
            field: "campaign days_passed_per_finished_map",
            message: "days_passed_per_finished_map length must match finished_maps length",
        });
    }

    writer.write_i32_be(campaign_save_data.current_scenario_info.campaign_id);
    writer.write_i32_be(campaign_save_data.current_scenario_info.scenario_id);
    writer.write_i32_be(campaign_save_data.current_scenario_bonus_id);

    writer.write_u32_be(
        u32::try_from(campaign_save_data.finished_maps.len()).map_err(|_| Error::InvalidModel {
            field: "campaign finished maps",
            message: "vector count must fit in u32",
        })?,
    );
    for entry in &campaign_save_data.finished_maps {
        writer.write_i32_be(entry.campaign_id);
        writer.write_i32_be(entry.scenario_id);
    }

    writer.write_u32_be(
        u32::try_from(campaign_save_data.bonuses_for_finished_maps.len()).map_err(|_| {
            Error::InvalidModel {
                field: "campaign finished map bonuses",
                message: "vector count must fit in u32",
            }
        })?,
    );
    for bonus in &campaign_save_data.bonuses_for_finished_maps {
        writer.write_i32_be(*bonus);
    }

    writer.write_u32_be(
        u32::try_from(campaign_save_data.days_passed_per_finished_map.len()).map_err(|_| {
            Error::InvalidModel {
                field: "campaign finished map days",
                message: "vector count must fit in u32",
            }
        })?,
    );
    for days in &campaign_save_data.days_passed_per_finished_map {
        writer.write_u32_be(*days);
    }

    writer.write_u32_be(
        u32::try_from(campaign_save_data.obtained_campaign_awards.len()).map_err(|_| {
            Error::InvalidModel {
                field: "campaign obtained awards",
                message: "vector count must fit in u32",
            }
        })?,
    );
    for award in &campaign_save_data.obtained_campaign_awards {
        writer.write_i32_be(*award);
    }

    writer.write_u32_be(
        u32::try_from(campaign_save_data.carry_over_troops.len()).map_err(|_| {
            Error::InvalidModel {
                field: "campaign carry-over troops",
                message: "vector count must fit in u32",
            }
        })?,
    );
    for troop in &campaign_save_data.carry_over_troops {
        writer.write_i32_be(troop.monster.to_i32());
        writer.write_u32_be(troop.count);
    }

    writer.write_i32_be(campaign_save_data.difficulty);
    writer.write_i32_be(campaign_save_data.min_difficulty);

    Ok(())
}
