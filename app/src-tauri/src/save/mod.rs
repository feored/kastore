use std::path::Path;

use kastore::{load_with_options, LoadOptions, ParseReport, SaveGame};

pub fn open_save_file(path: &Path) -> Result<ParseReport<SaveGame>, String> {
    let bytes = std::fs::read(path)
        .map_err(|error| format!("Could not read {}: {error}", path.display()))?;

    let report = load_with_options(&bytes, &LoadOptions::permissive())
        .map_err(|error| format!("Could not parse {}: {error}", path.display()))?;

    Ok(report)
}

pub fn write_save_file(path: &Path, bytes: &[u8]) -> Result<(), String> {
    if path.exists() {
        let backup_path = backup_path_for(path);
        std::fs::copy(path, &backup_path).map_err(|error| {
            format!("Could not create backup {}: {error}", backup_path.display())
        })?;
    }

    std::fs::write(path, bytes)
        .map_err(|error| format!("Could not write save {}: {error}", path.display()))?;

    Ok(())
}

fn backup_path_for(path: &Path) -> std::path::PathBuf {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| "save".to_string());

    path.with_file_name(format!("{file_name}.bak"))
}
