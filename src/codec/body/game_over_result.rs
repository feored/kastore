use crate::Error;
use crate::internal::error::ParseSection;
use crate::internal::reader::Reader;
use crate::internal::writer::Writer;
use crate::model::game_over_result::{GameOverResult, GameOverResultSet};
use crate::model::header::player::PlayerColorsSet;

pub(crate) fn decode(
    reader: &mut Reader<'_>,
) -> std::result::Result<GameOverResult, Error> {
    reader.set_section(ParseSection::GameOver);
    let active_colors =
        PlayerColorsSet::from_bits(reader.read_u8("game over result active colors")?);
    let result = GameOverResultSet::from_bits(reader.read_u32_be("game over result")?);
    Ok(GameOverResult {
        active_colors,
        result,
    })
}

pub(crate) fn encode(writer: &mut Writer, game_over_result: &GameOverResult) {
    writer.write_u8(game_over_result.active_colors.bits());
    writer.write_u32_be(game_over_result.result.bits());
}
