use std::collections::BTreeMap;

use crate::Error;
use crate::internal::reader::Reader;
use crate::internal::writer::Writer;
use crate::model::header::player::PlayerColor;
use crate::model::world::captured_objects::CapturedObject;
use crate::model::world::heroes::army::{MonsterType, Troop};

use super::heroes::encode_troop;

pub(super) fn decode(
    reader: &mut Reader<'_>,
) -> std::result::Result<BTreeMap<i32, CapturedObject>, Error> {
    let count = reader.read_u32_be("captured objects count")?;
    let mut captured_objects = BTreeMap::new();
    for _ in 0..count {
        let tile_index_offset = reader.position();
        let tile_index = reader.read_i32_be("captured object tile index")?;
        let captured_object = CapturedObject {
            object_type: reader.read_u16_be("captured object type")?,
            color: PlayerColor::from_bits(reader.read_u8("captured object color")?),
            guardians: Troop {
                monster: MonsterType::from_i32(
                    reader.read_i32_be("captured object guardians monster")?,
                ),
                count: reader.read_u32_be("captured object guardians count")?,
            },
        };
        if captured_objects
            .insert(tile_index, captured_object)
            .is_some()
        {
            return Err(reader.invalid_value(
                "captured object tile index",
                tile_index_offset,
                "duplicate captured object tile index",
            ));
        }
    }

    Ok(captured_objects)
}

pub(super) fn encode(
    writer: &mut Writer,
    captured_objects: &BTreeMap<i32, CapturedObject>,
) -> std::result::Result<(), Error> {
    writer.write_u32_be(u32::try_from(captured_objects.len()).map_err(|_| {
        Error::InvalidModel {
            field: "world captured objects",
            message: "captured object count must fit in u32",
        }
    })?);
    for (tile_index, captured_object) in captured_objects {
        writer.write_i32_be(*tile_index);
        writer.write_u16_be(captured_object.object_type);
        writer.write_u8(captured_object.color.bits());
        encode_troop(writer, &captured_object.guardians);
    }

    Ok(())
}
