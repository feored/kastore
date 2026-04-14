use crate::Error;
use crate::codec::file::{decode_map_info, encode_map_info};
use crate::internal::error::ParseSection;
use crate::internal::reader::Reader;
use crate::internal::writer::Writer;
use crate::model::header::game_type::GameType;
use crate::model::header::map_info::Difficulty;
use crate::model::header::player::{PlayerColor, PlayerColorsSet, Race};
use crate::model::settings::{
    Settings, SettingsAiPersonality, SettingsFocus, SettingsFocusKind, SettingsHandicapStatus,
    SettingsPlayer, SettingsPlayerControl, SettingsPlayers,
};
use crate::version::MapInfoRevision;

pub(crate) fn decode(
    reader: &mut Reader<'_>,
    map_info_revision: MapInfoRevision,
) -> std::result::Result<Settings, Error> {
    reader.set_section(ParseSection::Settings);
    let loaded_file_language = reader.read_save_string("settings loaded file language")?;
    let current_map_info = decode_map_info(reader, map_info_revision)?;
    reader.set_section(ParseSection::Settings);

    let game_difficulty = Difficulty::from_i32(reader.read_i32_be("settings game difficulty")?);
    let game_type = GameType::from_i32(reader.read_i32_be("settings game type")?);

    let colors = PlayerColorsSet::from_bits(reader.read_u8("settings players colors")?);
    let current_player_color =
        PlayerColor::from_bits(reader.read_u8("settings players current player color")?);
    let expected_players_count = usize::try_from(colors.bits().count_ones()).unwrap_or(0);

    let mut entries = Vec::with_capacity(expected_players_count);
    for _ in 0..expected_players_count {
        entries.push(SettingsPlayer {
            modes: reader.read_u32_be("settings player modes")?,
            control: SettingsPlayerControl::from_i32(
                reader.read_i32_be("settings player control")?,
            ),
            color: PlayerColor::from_bits(reader.read_u8("settings player color")?),
            race: Race::from_i32(reader.read_i32_be("settings player race")?),
            friends: PlayerColorsSet::from_bits(reader.read_u8("settings player friends")?),
            name: reader.read_save_string("settings player name")?,
            focus: SettingsFocus {
                kind: SettingsFocusKind::from_i32(
                    reader.read_i32_be("settings player focus kind")?,
                ),
                tile_index: reader.read_i32_be("settings player focus tile index")?,
            },
            ai_personality: SettingsAiPersonality::from_i32(
                reader.read_i32_be("settings player ai personality")?,
            ),
            handicap_status: SettingsHandicapStatus::from_u8(
                reader.read_u8("settings player handicap status")?,
            ),
        });
    }

    Ok(Settings {
        loaded_file_language,
        current_map_info,
        game_difficulty,
        game_type,
        players: SettingsPlayers {
            colors,
            current_player_color,
            entries,
        },
    })
}

pub(crate) fn encode(
    writer: &mut Writer,
    settings: &Settings,
    map_info_revision: MapInfoRevision,
) -> std::result::Result<(), Error> {
    writer.write_save_string(&settings.loaded_file_language);
    encode_map_info(writer, &settings.current_map_info, map_info_revision)?;
    writer.write_i32_be(settings.game_difficulty.to_i32());
    writer.write_i32_be(settings.game_type.to_i32());

    let expected_players_count =
        usize::try_from(settings.players.colors.bits().count_ones()).unwrap_or(0);
    if settings.players.entries.len() != expected_players_count {
        return Err(Error::InvalidModel {
            field: "settings players",
            message: "player entry count must match colors bit count",
        });
    }

    writer.write_u8(settings.players.colors.bits());
    writer.write_u8(settings.players.current_player_color.bits());
    for player in &settings.players.entries {
        writer.write_u32_be(player.modes);
        writer.write_i32_be(player.control.to_i32());
        writer.write_u8(player.color.bits());
        writer.write_i32_be(player.race.to_i32());
        writer.write_u8(player.friends.bits());
        writer.write_save_string(&player.name);
        writer.write_i32_be(player.focus.kind.to_i32());
        writer.write_i32_be(player.focus.tile_index);
        writer.write_i32_be(player.ai_personality.to_i32());
        writer.write_u8(player.handicap_status.to_u8());
    }

    Ok(())
}
