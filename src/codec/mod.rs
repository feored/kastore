pub(crate) mod body;
mod file;
mod parse;
mod world_date;

use crate::Error;
use crate::model::header::SaveHeader;
use crate::model::save_game::SaveGame;
use crate::version::{SaveVersion, profile_for};
use parse::ParseContext;
pub use parse::{Diagnostic, DiagnosticKind, LoadOptions, ParseMode, ParseReport, Severity};

/// Decode a supported fheroes2 save file.
pub fn load(bytes: &[u8]) -> std::result::Result<SaveGame, Error> {
    Ok(load_with_options(bytes, &LoadOptions::strict())?.value)
}

/// Decode a supported fheroes2 save file and return parse diagnostics.
pub fn load_with_options(
    bytes: &[u8],
    options: &LoadOptions,
) -> std::result::Result<ParseReport<SaveGame>, Error> {
    let mut parse_context = ParseContext::new(options.parse_mode);
    let save_version = file::detect_save_version(bytes)?;
    let Some(profile) = profile_for(save_version) else {
        return Err(Error::UnsupportedSaveVersion {
            version: save_version.as_u16(),
        });
    };
    let parts = file::decode_file(bytes, profile, &mut parse_context)?;
    let sections = body::decode_sections(
        &parts.body,
        profile.map_info_revision,
        parts.game_type,
        &mut parse_context,
    )?;

    Ok(parse_context.finish(SaveGame {
        source_version: save_version,
        header: SaveHeader {
            requires_pol: parts.requires_pol,
            file_info: parts.file_info,
            game_type: parts.game_type,
        },
        compression_header: parts.body_compression_header,
        world: sections.world,
        settings: sections.settings,
        game_over_result: sections.game_over_result,
        campaign_save_data: sections.campaign_save_data,
    }))
}

/// Encode a save using its original save format version.
pub fn save(save_game: &SaveGame) -> std::result::Result<Vec<u8>, Error> {
    save_as(save_game, save_game.source_version)
}

/// Encode a save using a specific save format version.
///
/// Conversion is supported between currently supported save profiles.
pub fn save_as(save_game: &SaveGame, target: SaveVersion) -> std::result::Result<Vec<u8>, Error> {
    let Some(target_profile) = profile_for(target) else {
        return Err(Error::UnsupportedSaveVersion {
            version: target.as_u16(),
        });
    };

    let Some(source_profile) = profile_for(save_game.source_version) else {
        return Err(Error::UnsupportedSaveVersion {
            version: save_game.source_version.as_u16(),
        });
    };
    let _source_map_info_revision = source_profile.map_info_revision;
    let body = encode_body(save_game, target_profile.map_info_revision)?;

    file::encode_file(
        &file::FileParts {
            requires_pol: save_game.header.requires_pol,
            file_info: save_game.header.file_info.clone(),
            game_type: save_game.header.game_type,
            body_compression_header: save_game.compression_header,
            body,
        },
        target_profile,
    )
}

fn encode_body(
    save_game: &SaveGame,
    target_map_info_revision: crate::version::MapInfoRevision,
) -> std::result::Result<Vec<u8>, Error> {
    body::encode_sections(
        &save_game.world,
        &save_game.settings,
        &save_game.game_over_result,
        save_game.campaign_save_data.as_ref(),
        target_map_info_revision,
        save_game.header.game_type,
    )
}

#[cfg(test)]
mod tests;
