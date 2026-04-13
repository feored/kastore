use crate::Error;
use crate::internal::reader::Reader;
use crate::internal::writer::Writer;
use crate::model::header::player::PlayerColorsSet;
use crate::model::world::timed_events::TimedEvent;

pub(super) fn decode(reader: &mut Reader<'_>) -> std::result::Result<Vec<TimedEvent>, Error> {
    let count = reader.read_u32_be("timed events count")?;
    let mut timed_events = Vec::with_capacity(usize::try_from(count).unwrap_or(0));
    for _ in 0..count {
        timed_events.push(TimedEvent {
            resources: super::decode_funds(reader)?,
            is_applicable_for_ai_players: reader
                .read_byte_as_bool("timed event applies to ai players")?,
            first_occurrence_day: reader.read_u32_be("timed event first occurrence day")?,
            repeat_period_in_days: reader.read_u32_be("timed event repeat period in days")?,
            colors: PlayerColorsSet::from_bits(reader.read_u8("timed event colors")?),
            message: reader.read_save_string("timed event message")?,
            title: reader.read_save_string("timed event title")?,
        });
    }

    Ok(timed_events)
}

pub(super) fn encode(
    writer: &mut Writer,
    timed_events: &[TimedEvent],
) -> std::result::Result<(), Error> {
    writer.write_u32_be(
        u32::try_from(timed_events.len()).map_err(|_| Error::InvalidModel {
            field: "world timed events",
            message: "timed event count must fit in u32",
        })?,
    );
    for timed_event in timed_events {
        super::encode_funds(writer, &timed_event.resources);
        writer.write_byte_from_bool(timed_event.is_applicable_for_ai_players);
        writer.write_u32_be(timed_event.first_occurrence_day);
        writer.write_u32_be(timed_event.repeat_period_in_days);
        writer.write_u8(timed_event.colors.bits());
        writer.write_save_string(&timed_event.message);
        writer.write_save_string(&timed_event.title);
    }

    Ok(())
}
