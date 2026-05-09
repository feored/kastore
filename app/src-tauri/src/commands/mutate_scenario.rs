use kastore::{set_duplicated_map_info_text, DuplicatedMapInfoTextField, SaveString};
use tauri::State;

use crate::bridge::scenario_dto;
use crate::document::ManagedSaveState;
use crate::dto::{ScenarioMutationDto, ScenarioMutationResultDto};

#[tauri::command]
pub fn mutate_scenario(
    mutation: ScenarioMutationDto,
    state: State<'_, ManagedSaveState>,
) -> Result<ScenarioMutationResultDto, String> {
    let mut state = state.lock()?;
    let save_state = state
        .as_mut()
        .ok_or_else(|| "No save file is currently open.".to_string())?;

    let (field, text) = match mutation {
        ScenarioMutationDto::SetName { text } => (DuplicatedMapInfoTextField::Name, text),
        ScenarioMutationDto::SetFileName { text } => (DuplicatedMapInfoTextField::Filename, text),
        ScenarioMutationDto::SetDescription { text } => {
            (DuplicatedMapInfoTextField::Description, text)
        }
    };

    set_duplicated_map_info_text(&mut save_state.save, field, SaveString::from(text));
    save_state.mark_changed();

    Ok(ScenarioMutationResultDto {
        scenario: scenario_dto(&save_state.save),
        dirty: save_state.dirty,
        revision: save_state.revision,
    })
}
