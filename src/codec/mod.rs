use crate::Error;
use crate::model::{SaveGame, SaveHeader};
use crate::version::{SaveVersion, profile_for};

pub fn load(bytes: &[u8]) -> std::result::Result<SaveGame, Error> {
    let save_version = crate::container::detect_save_version(bytes)?;
    let Some(profile) = profile_for(save_version) else {
        return Err(Error::UnsupportedSaveVersion {
            version: save_version.as_u16(),
        });
    };
    let parts = crate::container::decode_container(bytes, profile)?;

    Ok(SaveGame {
        source_version: save_version,
        header: SaveHeader {
            requires_pol: parts.requires_pol,
            map_info: parts.map_info,
            game_type: parts.game_type,
        },
        payload: parts.payload,
    })
}

pub fn save(save_game: &SaveGame) -> std::result::Result<Vec<u8>, Error> {
    save_as(save_game, save_game.source_version)
}

pub fn save_as(save_game: &SaveGame, target: SaveVersion) -> std::result::Result<Vec<u8>, Error> {
    let Some(profile) = profile_for(target) else {
        return Err(Error::UnsupportedSaveVersion {
            version: target.as_u16(),
        });
    };

    if save_game.source_version != target {
        return Err(Error::NotImplemented {
            feature: "save version conversion",
        });
    }

    crate::container::encode_container(
        &crate::container::ContainerParts {
            requires_pol: save_game.header.requires_pol,
            map_info: save_game.header.map_info.clone(),
            game_type: save_game.header.game_type,
            payload: save_game.payload.clone(),
        },
        profile,
    )
}

#[cfg(test)]
mod tests;
