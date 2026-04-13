use crate::Error;
use crate::internal::reader::Reader;
use crate::internal::writer::Writer;
use crate::model::world::Point;
use crate::model::world::heroes::artifact::{Artifact, ArtifactID};
use crate::model::world::ultimate_artifact::UltimateArtifact;

pub(super) fn decode(reader: &mut Reader<'_>) -> std::result::Result<UltimateArtifact, Error> {
    let artifact_id: ArtifactID = ArtifactID::from_i32(reader.read_i32_be("ultimate artifact id")?);
    let artifact_ext: i32 = reader.read_i32_be("ultimate artifact ext")?;
    let artifact = Artifact {
        id: artifact_id,
        ext: artifact_ext,
    };
    let index: i32 = reader.read_i32_be("ultimate artifact index")?;
    let is_found: bool = reader.read_byte_as_bool("ultimate artifact is found")?;
    let offset: Point = Point {
        x: reader.read_i32_be("ultimate artifact offset x")?,
        y: reader.read_i32_be("ultimate artifact offset y")?,
    };

    Ok(UltimateArtifact {
        artifact,
        index,
        is_found,
        offset,
    })
}

pub(super) fn encode(
    writer: &mut Writer,
    ultimate_artifact: &UltimateArtifact,
) -> std::result::Result<(), Error> {
    writer.write_i32_be(ultimate_artifact.artifact.id.to_i32());
    writer.write_i32_be(ultimate_artifact.artifact.ext);
    writer.write_i32_be(ultimate_artifact.index);
    writer.write_byte_from_bool(ultimate_artifact.is_found);
    writer.write_i32_be(ultimate_artifact.offset.x);
    writer.write_i32_be(ultimate_artifact.offset.y);
    Ok(())
}
