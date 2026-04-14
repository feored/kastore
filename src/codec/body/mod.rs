use crate::Error;
use crate::codec::parse::{DiagnosticKind, ParseContext};
use crate::internal::error::ParseSection;
use crate::internal::reader::Reader;
use crate::model::campaign_save_data::CampaignSaveData;
use crate::model::game_over_result::GameOverResult;
use crate::model::header::game_type::GameType;
use crate::model::settings::Settings;
use crate::model::world::World;
use crate::version::MapInfoRevision;

pub(crate) mod campaign_save_data;
pub(crate) mod game_over_result;
pub(crate) mod settings;
pub(crate) mod world;

const BODY_END_MARKER: u16 = 0xFF03;

pub(crate) struct DecodedSections {
    pub(crate) world: World,
    pub(crate) settings: Settings,
    pub(crate) game_over_result: GameOverResult,
    pub(crate) campaign_save_data: Option<CampaignSaveData>,
}

pub(crate) fn decode_sections(
    body_bytes: &[u8],
    map_info_revision: MapInfoRevision,
    game_type: GameType,
    parse_context: &mut ParseContext,
) -> std::result::Result<DecodedSections, Error> {
    let mut reader = Reader::with_context(body_bytes, ParseSection::Body);

    let world = world::decode(&mut reader)?;
    let settings = settings::decode(&mut reader, map_info_revision)?;
    let game_over_result = game_over_result::decode(&mut reader)?;

    let campaign_save_data = if game_type.contains(GameType::CAMPAIGN) {
        Some(campaign_save_data::decode(&mut reader)?)
    } else {
        None
    };

    let end_marker_offset = reader.position();
    let end_marker = reader.read_u16_be("body end marker")?;
    if end_marker != BODY_END_MARKER {
        return Err(reader.unexpected_value(
            "body end marker",
            end_marker_offset,
            "0xFF03",
            format!("0x{end_marker:04X}"),
        ));
    }

    if reader.position() != body_bytes.len() {
        parse_context.warn(
            DiagnosticKind::TrailingBytes,
            ParseSection::Body,
            Some("body"),
            Some(reader.position()),
            "unexpected trailing bytes after body end marker",
            Some(reader.invalid_value(
                "body",
                reader.position(),
                "unexpected trailing bytes after body end marker",
            )),
        )?;
    }

    Ok(DecodedSections {
        world,
        settings,
        game_over_result,
        campaign_save_data,
    })
}

pub(crate) fn encode_sections(
    world_model: &World,
    settings_model: &Settings,
    game_over_result_model: &GameOverResult,
    campaign_save_data_model: Option<&CampaignSaveData>,
    map_info_revision: MapInfoRevision,
    game_type: GameType,
) -> std::result::Result<Vec<u8>, Error> {
    let mut writer = crate::internal::writer::Writer::new();
    world::encode_into(&mut writer, world_model)?;
    settings::encode(&mut writer, settings_model, map_info_revision)?;
    game_over_result::encode(&mut writer, game_over_result_model);

    if game_type.contains(GameType::CAMPAIGN) {
        let campaign_save_data_model = campaign_save_data_model.ok_or(Error::InvalidModel {
            field: "campaign save data",
            message: "campaign game type requires campaign save data",
        })?;
        campaign_save_data::encode(&mut writer, campaign_save_data_model)?;
    }

    writer.write_u16_be(BODY_END_MARKER);

    Ok(writer.into_bytes())
}
