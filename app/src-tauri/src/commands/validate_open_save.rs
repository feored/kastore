use kastore::validate_save_game;
use tauri::State;

use crate::bridge::validation_result;
use crate::document::ManagedSaveState;
use crate::dto::ValidationResultDto;

#[tauri::command]
pub fn validate_open_save(
    state: State<'_, ManagedSaveState>,
) -> Result<ValidationResultDto, String> {
    let state = state.lock()?;
    let save_state = state
        .as_ref()
        .ok_or_else(|| "No save file is currently open.".to_string())?;

    let issue = validate_save_game(&save_state.save).err();

    Ok(validation_result(save_state.revision, issue))
}
