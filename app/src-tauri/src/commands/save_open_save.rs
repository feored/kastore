use kastore::{save, sync_header_metadata_from_runtime};
use tauri::State;

use crate::bridge::opened_save_dto;
use crate::document::ManagedSaveState;
use crate::dto::OpenedSaveDto;
use crate::save::write_save_file;

#[tauri::command]
pub fn save_open_save(state: State<'_, ManagedSaveState>) -> Result<OpenedSaveDto, String> {
    let mut state = state.lock()?;
    let save_state = state
        .as_mut()
        .ok_or_else(|| "No save file is currently open.".to_string())?;

    sync_header_metadata_from_runtime(&mut save_state.save)
        .map_err(|error| format!("Could not sync duplicated scenario metadata: {error}"))?;

    let bytes =
        save(&save_state.save).map_err(|error| format!("Could not encode save: {error}"))?;

    write_save_file(&save_state.path, &bytes)?;
    save_state.dirty = false;

    Ok(opened_save_dto(
        &save_state.path,
        &save_state.save,
        Vec::new(),
        save_state.dirty,
        save_state.revision,
    ))
}
