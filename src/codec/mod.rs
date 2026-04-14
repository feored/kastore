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
    let (world, world_prefix_len) = body::world::decode_prefix(&parts.body)?;
    let settings_bytes = parts
        .body
        .get(world_prefix_len..)
        .ok_or(Error::InvalidModel {
            field: "body",
            message: "world prefix length exceeds body length",
        })?;
    let (settings, _) = body::settings::decode_prefix(settings_bytes, profile.map_info_revision)?;

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
        settings,
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
    let body = encode_body(save_game, profile.map_info_revision)?;

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

fn encode_body(
    save_game: &SaveGame,
    map_info_revision: crate::version::MapInfoRevision,
) -> std::result::Result<Vec<u8>, Error> {
    let mut encoded_prefix = body::world::encode(&save_game.world)?;

    if save_game.body.is_empty() {
        let mut writer = crate::internal::writer::Writer::new();
        writer.write_bytes(&encoded_prefix);
        body::settings::encode(&mut writer, &save_game.settings, map_info_revision)?;
        return Ok(writer.into_bytes());
    }

    // Preserve any trailing body sections that still live in the opaque byte tail.
    let (_, original_world_prefix_len) = body::world::decode_prefix(&save_game.body)?;
    let original_settings_bytes =
        save_game
            .body
            .get(original_world_prefix_len..)
            .ok_or_else(|| Error::InvalidModel {
                field: "body",
                message: "world prefix length exceeds original body length",
            })?;
    let (_, original_settings_prefix_len) =
        body::settings::decode_prefix(original_settings_bytes, map_info_revision)?;
    let original_prefix_len = original_world_prefix_len + original_settings_prefix_len;

    let mut writer = crate::internal::writer::Writer::new();
    writer.write_bytes(&encoded_prefix);
    body::settings::encode(&mut writer, &save_game.settings, map_info_revision)?;
    encoded_prefix = writer.into_bytes();

    let suffix = save_game
        .body
        .get(original_prefix_len..)
        .ok_or_else(|| Error::InvalidModel {
            field: "body",
            message: "typed body prefix length exceeds original body length",
        })?;

    let mut body = Vec::with_capacity(encoded_prefix.len() + suffix.len());
    body.extend_from_slice(&encoded_prefix);
    body.extend_from_slice(suffix);
    Ok(body)
}

#[cfg(test)]
mod tests;
