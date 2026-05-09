use std::error::Error;
use std::fmt::{Display, Formatter};

use super::save_game::SaveGame;
use crate::SaveString;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeaderSyncError {
    InvalidWorldWidth(i32),
    InvalidWorldHeight(i32),
}

impl Display for HeaderSyncError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HeaderSyncError::InvalidWorldWidth(width) => {
                write!(f, "world width {width} cannot be written to map info")
            }
            HeaderSyncError::InvalidWorldHeight(height) => {
                write!(f, "world height {height} cannot be written to map info")
            }
        }
    }
}

impl Error for HeaderSyncError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DuplicatedMapInfoTextField {
    Filename,
    Name,
    Description,
}

pub fn set_duplicated_map_info_text(
    save: &mut SaveGame,
    field: DuplicatedMapInfoTextField,
    value: SaveString,
) {
    match field {
        DuplicatedMapInfoTextField::Filename => {
            save.header.file_info.filename = value.clone();
            save.settings.current_map_info.filename = value;
        }
        DuplicatedMapInfoTextField::Name => {
            save.header.file_info.name = value.clone();
            save.settings.current_map_info.name = value;
        }
        DuplicatedMapInfoTextField::Description => {
            save.header.file_info.description = value.clone();
            save.settings.current_map_info.description = value;
        }
    }
}

pub fn sync_header_metadata_from_runtime(save: &mut SaveGame) -> Result<(), HeaderSyncError> {
    let width = u16::try_from(save.world.width)
        .map_err(|_| HeaderSyncError::InvalidWorldWidth(save.world.width))?;
    let height = u16::try_from(save.world.height)
        .map_err(|_| HeaderSyncError::InvalidWorldHeight(save.world.height))?;

    save.header.file_info.width = width;
    save.header.file_info.height = height;
    save.header.file_info.world_date = save.world.world_date;
    save.header.file_info.difficulty = save.settings.game_difficulty;
    save.header.game_type = save.settings.game_type;

    save.settings.current_map_info.width = width;
    save.settings.current_map_info.height = height;
    save.settings.current_map_info.world_date = save.world.world_date;
    save.settings.current_map_info.difficulty = save.settings.game_difficulty;

    Ok(())
}
