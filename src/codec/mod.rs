pub(crate) mod body;
mod file;
mod world_date;

use crate::Error;
use crate::model::header::SaveHeader;
use crate::model::save_game::SaveGame;
use crate::version::{SaveVersion, profile_for};

/// Decode a supported fheroes2 save file.
pub fn load(bytes: &[u8]) -> std::result::Result<SaveGame, Error> {
    let save_version = file::detect_save_version(bytes)?;
    let Some(profile) = profile_for(save_version) else {
        return Err(Error::UnsupportedSaveVersion {
            version: save_version.as_u16(),
        });
    };
    let parts = file::decode_file(bytes, profile)?;
    let world = body::world::decode(&parts.body)?;

    Ok(SaveGame {
        source_version: save_version,
        header: SaveHeader {
            requires_pol: parts.requires_pol,
            file_info: parts.file_info,
            game_type: parts.game_type,
        },
        compression_header: parts.body_compression_header,
        body: parts.body,
        world,
    })
}

/// Encode a save using its original save format version.
///
/// Body sections not covered by the typed model are preserved through
/// `SaveGame::body`.
pub fn save(save_game: &SaveGame) -> std::result::Result<Vec<u8>, Error> {
    save_as(save_game, save_game.source_version)
}

/// Encode a save using a specific save format version.
///
/// Cross-version conversion is not implemented yet.
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
    let body = encode_body(save_game)?;

    file::encode_file(
        &file::FileParts {
            requires_pol: save_game.header.requires_pol,
            file_info: save_game.header.file_info.clone(),
            game_type: save_game.header.game_type,
            body_compression_header: save_game.compression_header,
            body,
        },
        profile,
    )
}

fn encode_body(save_game: &SaveGame) -> std::result::Result<Vec<u8>, Error> {
    if save_game.body.is_empty() {
        return body::world::encode(&save_game.world);
    }

    // Preserve any trailing body sections that still live in the opaque byte tail.
    let (_, original_world_prefix_len) = body::world::decode_prefix(&save_game.body)?;
    let encoded_world = body::world::encode(&save_game.world)?;
    let suffix = save_game
        .body
        .get(original_world_prefix_len..)
        .ok_or_else(|| Error::InvalidModel {
            field: "body",
            message: "world prefix length exceeds original body length",
        })?;

    let mut body = Vec::with_capacity(encoded_world.len() + suffix.len());
    body.extend_from_slice(&encoded_world);
    body.extend_from_slice(suffix);
    Ok(body)
}

#[cfg(test)]
mod tests;
