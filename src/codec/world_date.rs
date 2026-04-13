use crate::Error;
use crate::internal::reader::Reader;
use crate::internal::writer::Writer;
use crate::model::header::map_info::WorldDate;

pub(crate) fn decode_world_date(reader: &mut Reader<'_>) -> std::result::Result<WorldDate, Error> {
    Ok(WorldDate {
        day: reader.read_u32_be("world date day")?,
        week: reader.read_u32_be("world date week")?,
        month: reader.read_u32_be("world date month")?,
    })
}

pub(crate) fn encode_world_date(writer: &mut Writer, world_date: WorldDate) {
    writer.write_u32_be(world_date.day);
    writer.write_u32_be(world_date.week);
    writer.write_u32_be(world_date.month);
}
