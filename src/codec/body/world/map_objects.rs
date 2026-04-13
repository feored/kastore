use std::collections::BTreeMap;

use crate::Error;
use crate::SaveString;
use crate::internal::reader::Reader;
use crate::internal::writer::Writer;
use crate::model::header::player::PlayerColorsSet;
use crate::model::header::supported_language::SupportedLanguage;
use crate::model::world::MapPosition;
use crate::model::world::heroes::artifact::{Artifact, ArtifactID};
use crate::model::world::heroes::skills::{SecondarySkill, Skill, SkillLevel};
use crate::model::world::map_objects::{
    LocalizedString, MapEvent, MapObject, MapObjectBase, MapSign, MapSphinx,
};

pub(super) fn decode(
    reader: &mut Reader<'_>,
) -> std::result::Result<BTreeMap<u32, MapObject>, Error> {
    let count = reader.read_u32_be("map objects count")?;
    let mut map_objects = BTreeMap::new();

    for _ in 0..count {
        let uid_offset = reader.position();
        let uid = reader.read_u32_be("map object uid")?;
        let object_type = reader.read_u16_be("map object type")?;
        let object = match object_type {
            MapObject::EVENT_TYPE => MapObject::Event(decode_event(reader)?),
            MapObject::SPHINX_TYPE => MapObject::Sphinx(decode_sphinx(reader)?),
            MapObject::SIGN_TYPE => MapObject::Sign(decode_sign(reader)?),
            _ => {
                return Err(reader.invalid_value(
                    "map object type",
                    uid_offset + 4,
                    "unsupported map object type",
                ));
            }
        };

        if object.base().uid != uid {
            return Err(reader.invalid_value(
                "map object uid",
                uid_offset,
                "map object key uid must match object body uid",
            ));
        }

        if map_objects.insert(uid, object).is_some() {
            return Err(reader.invalid_value(
                "map object uid",
                uid_offset,
                "duplicate map object uid",
            ));
        }
    }

    Ok(map_objects)
}

pub(super) fn encode(
    writer: &mut Writer,
    map_objects: &BTreeMap<u32, MapObject>,
) -> std::result::Result<(), Error> {
    writer.write_u32_be(
        u32::try_from(map_objects.len()).map_err(|_| Error::InvalidModel {
            field: "world map objects",
            message: "map object count must fit in u32",
        })?,
    );

    for (uid, object) in map_objects {
        if object.base().uid != *uid {
            return Err(Error::InvalidModel {
                field: "world map objects",
                message: "map object key uid must match object body uid",
            });
        }

        writer.write_u32_be(*uid);
        writer.write_u16_be(object.object_type());

        match object {
            MapObject::Event(event) => encode_event(writer, event)?,
            MapObject::Sphinx(sphinx) => encode_sphinx(writer, sphinx)?,
            MapObject::Sign(sign) => encode_sign(writer, sign)?,
        };
    }

    Ok(())
}

fn decode_event(reader: &mut Reader<'_>) -> std::result::Result<MapEvent, Error> {
    Ok(MapEvent {
        base: decode_base_object(reader)?,
        resources: super::decode_funds(reader)?,
        artifact: decode_artifact(reader)?,
        is_computer_player_allowed: reader
            .read_byte_as_bool("map event is computer player allowed")?,
        is_single_time_event: reader.read_byte_as_bool("map event is single time event")?,
        colors: PlayerColorsSet::from_bits(reader.read_u8("map event colors")?),
        message: reader.read_save_string("map event message")?,
        secondary_skill: SecondarySkill {
            id: Skill::from_i32(reader.read_i32_be("map event secondary skill")?),
            level: SkillLevel::from_i32(reader.read_i32_be("map event secondary skill level")?),
        },
        experience: reader.read_i32_be("map event experience")?,
    })
}

fn encode_event(writer: &mut Writer, event: &MapEvent) -> std::result::Result<(), Error> {
    encode_base_object(writer, &event.base);
    super::encode_funds(writer, &event.resources);
    encode_artifact(writer, &event.artifact);
    writer.write_byte_from_bool(event.is_computer_player_allowed);
    writer.write_byte_from_bool(event.is_single_time_event);
    writer.write_u8(event.colors.bits());
    writer.write_save_string(&event.message);
    writer.write_i32_be(event.secondary_skill.id.to_i32());
    writer.write_i32_be(event.secondary_skill.level.to_i32());
    writer.write_i32_be(event.experience);

    Ok(())
}

fn decode_sphinx(reader: &mut Reader<'_>) -> std::result::Result<MapSphinx, Error> {
    let base = decode_base_object(reader)?;
    let resources = super::decode_funds(reader)?;
    let artifact = decode_artifact(reader)?;
    let answers_count = reader.read_u32_be("map sphinx answers count")?;
    let mut answers = Vec::with_capacity(usize::try_from(answers_count).unwrap_or(0));
    for _ in 0..answers_count {
        answers.push(reader.read_save_string("map sphinx answer")?);
    }

    Ok(MapSphinx {
        base,
        resources,
        artifact,
        answers,
        riddle: reader.read_save_string("map sphinx riddle")?,
        valid: reader.read_byte_as_bool("map sphinx valid")?,
        is_truncated_answer: reader.read_byte_as_bool("map sphinx is truncated answer")?,
    })
}

fn encode_sphinx(writer: &mut Writer, sphinx: &MapSphinx) -> std::result::Result<(), Error> {
    encode_base_object(writer, &sphinx.base);
    super::encode_funds(writer, &sphinx.resources);
    encode_artifact(writer, &sphinx.artifact);
    writer.write_u32_be(
        u32::try_from(sphinx.answers.len()).map_err(|_| Error::InvalidModel {
            field: "world map sphinx answers",
            message: "map sphinx answer count must fit in u32",
        })?,
    );
    for answer in &sphinx.answers {
        writer.write_save_string(answer);
    }
    writer.write_save_string(&sphinx.riddle);
    writer.write_byte_from_bool(sphinx.valid);
    writer.write_byte_from_bool(sphinx.is_truncated_answer);

    Ok(())
}

fn decode_sign(reader: &mut Reader<'_>) -> std::result::Result<MapSign, Error> {
    Ok(MapSign {
        base: decode_base_object(reader)?,
        message: decode_localized_string(reader)?,
    })
}

fn encode_sign(writer: &mut Writer, sign: &MapSign) -> std::result::Result<(), Error> {
    encode_base_object(writer, &sign.base);
    encode_localized_string(writer, &sign.message);

    Ok(())
}

fn decode_base_object(reader: &mut Reader<'_>) -> std::result::Result<MapObjectBase, Error> {
    Ok(MapObjectBase {
        map_position: MapPosition {
            x: reader.read_i16_be("map object position x")?,
            y: reader.read_i16_be("map object position y")?,
        },
        uid: reader.read_u32_be("map object body uid")?,
    })
}

fn encode_base_object(writer: &mut Writer, base: &MapObjectBase) {
    writer.write_i16_be(base.map_position.x);
    writer.write_i16_be(base.map_position.y);
    writer.write_u32_be(base.uid);
}

fn decode_artifact(reader: &mut Reader<'_>) -> std::result::Result<Artifact, Error> {
    Ok(Artifact {
        id: ArtifactID::from_i32(reader.read_i32_be("map object artifact id")?),
        ext: reader.read_i32_be("map object artifact ext")?,
    })
}

fn encode_artifact(writer: &mut Writer, artifact: &Artifact) {
    writer.write_i32_be(artifact.id.to_i32());
    writer.write_i32_be(artifact.ext);
}

fn decode_localized_string(reader: &mut Reader<'_>) -> std::result::Result<LocalizedString, Error> {
    let text: SaveString = reader.read_save_string("localized string text")?;
    let has_language = reader.read_byte_as_bool("localized string has language")?;
    let language = if has_language {
        Some(SupportedLanguage::from(
            reader.read_u8("localized string language")?,
        ))
    } else {
        None
    };

    Ok(LocalizedString { text, language })
}

fn encode_localized_string(writer: &mut Writer, value: &LocalizedString) {
    writer.write_save_string(&value.text);
    writer.write_byte_from_bool(value.language.is_some());
    if let Some(language) = value.language {
        writer.write_u8(language.as_u8());
    }
}
