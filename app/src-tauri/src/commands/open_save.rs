use std::path::Path;

use tauri::State;

use crate::bridge::{opened_save_dto, parser_diagnostics};
use crate::document::{ManagedSaveState, SaveState};
use crate::dto::OpenedSaveDto;
use crate::save::open_save_file;

#[tauri::command]
pub fn open_save(
    path: String,
    state: State<'_, ManagedSaveState>,
) -> Result<OpenedSaveDto, String> {
    let path = Path::new(&path);
    let report = open_save_file(path)?;
    let diagnostics = parser_diagnostics(&report.diagnostics);
    let save_state = SaveState::opened(path.to_path_buf(), report.value);
    let opened_save = opened_save_dto(
        path,
        &save_state.save,
        diagnostics,
        save_state.dirty,
        save_state.revision,
    );

    let mut state = state.lock()?;
    *state = Some(save_state);

    Ok(opened_save)
}
