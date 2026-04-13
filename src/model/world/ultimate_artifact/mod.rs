use std::fmt::Display;

use crate::model::world::heroes::artifact::Artifact;
use crate::model::world::{Point, heroes::artifact::ArtifactID};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UltimateArtifact {
    pub artifact: Artifact,
    pub index: i32,
    pub is_found: bool,
    pub offset: Point,
}

impl UltimateArtifact {
    pub fn is_meaningful(&self) -> bool {
        self.index >= 0
            || self.is_found
            || !matches!(self.artifact.id, ArtifactID::Unknown(0))
            || self.artifact.ext != 0
            || self.offset != Point::default()
    }
}

impl Default for UltimateArtifact {
    fn default() -> Self {
        Self {
            artifact: Artifact::default(),
            index: -1,
            is_found: false,
            offset: Point::default(),
        }
    }
}

impl Display for UltimateArtifact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "artifact={}, index={}, found={}, offset=({}, {})",
            self.artifact.id, self.index, self.is_found, self.offset.x, self.offset.y
        )?;

        if self.artifact.ext != 0 {
            write!(f, ", ext={}", self.artifact.ext)?;
        }

        Ok(())
    }
}
