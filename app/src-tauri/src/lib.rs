mod bridge;
mod commands;
mod document;
pub mod dto;
mod save;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(document::ManagedSaveState::default())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::mutate_scenario::mutate_scenario,
            commands::open_save::open_save,
            commands::save_open_save::save_open_save,
            commands::validate_open_save::validate_open_save
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
