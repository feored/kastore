use std::fmt::Display;

use crate::SaveString;
use crate::model::header::player::PlayerColorsSet;
use crate::model::header::supported_language::SupportedLanguage;
use crate::model::world::heroes::artifact::{Artifact, ArtifactID};
use crate::model::world::heroes::skills::{SecondarySkill, Skill, SkillLevel};
use crate::model::world::{Funds, MapPosition};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MapObjectBase {
    pub map_position: MapPosition,
    pub uid: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MapObject {
    Event(MapEvent),
    Sphinx(MapSphinx),
    Sign(MapSign),
}

impl MapObject {
    pub const EVENT_TYPE: u16 = 147;
    pub const SPHINX_TYPE: u16 = 207;
    pub const SIGN_TYPE: u16 = 130;

    pub fn base(&self) -> &MapObjectBase {
        match self {
            MapObject::Event(event) => &event.base,
            MapObject::Sphinx(sphinx) => &sphinx.base,
            MapObject::Sign(sign) => &sign.base,
        }
    }

    pub const fn object_type(&self) -> u16 {
        match self {
            MapObject::Event(_) => Self::EVENT_TYPE,
            MapObject::Sphinx(_) => Self::SPHINX_TYPE,
            MapObject::Sign(_) => Self::SIGN_TYPE,
        }
    }
}

impl Display for MapObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapObject::Event(event) => write!(f, "{event}"),
            MapObject::Sphinx(sphinx) => write!(f, "{sphinx}"),
            MapObject::Sign(sign) => write!(f, "{sign}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapEvent {
    pub base: MapObjectBase,
    pub resources: Funds,
    pub artifact: Artifact,
    pub is_computer_player_allowed: bool,
    pub is_single_time_event: bool,
    pub colors: PlayerColorsSet,
    pub message: SaveString,
    pub secondary_skill: SecondarySkill,
    pub experience: i32,
}

impl Default for MapEvent {
    fn default() -> Self {
        Self {
            base: MapObjectBase::default(),
            resources: Funds::default(),
            artifact: Artifact::default(),
            is_computer_player_allowed: false,
            is_single_time_event: true,
            colors: PlayerColorsSet::default(),
            message: SaveString::default(),
            secondary_skill: SecondarySkill {
                id: Skill::Unknown(0),
                level: SkillLevel::Unknown(0),
            },
            experience: 0,
        }
    }
}

impl Display for MapEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Event uid={}, pos=({}, {}), colors={}",
            self.base.uid, self.base.map_position.x, self.base.map_position.y, self.colors
        )?;

        let rewards = brief_rewards(
            &self.resources,
            &self.artifact,
            &self.secondary_skill,
            self.experience,
        );
        if !rewards.is_empty() {
            write!(f, ", rewards={rewards}")?;
        }

        if !self.message.is_empty() {
            write!(f, ", message={}", self.message.brief(56))?;
        }

        if self.is_computer_player_allowed {
            write!(f, ", ai=true")?;
        }

        if self.is_single_time_event {
            write!(f, ", once=true")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MapSphinx {
    pub base: MapObjectBase,
    pub resources: Funds,
    pub artifact: Artifact,
    pub answers: Vec<SaveString>,
    pub riddle: SaveString,
    pub valid: bool,
    pub is_truncated_answer: bool,
}

impl Display for MapSphinx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Sphinx uid={}, pos=({}, {}), answers={}, valid={}, truncated={}",
            self.base.uid,
            self.base.map_position.x,
            self.base.map_position.y,
            self.answers.len(),
            self.valid,
            self.is_truncated_answer
        )?;

        let rewards = brief_rewards(
            &self.resources,
            &self.artifact,
            &SecondarySkill {
                id: Skill::Unknown(0),
                level: SkillLevel::Unknown(0),
            },
            0,
        );
        if !rewards.is_empty() {
            write!(f, ", rewards={rewards}")?;
        }

        if !self.riddle.is_empty() {
            write!(f, ", riddle={}", self.riddle.brief(56))?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LocalizedString {
    pub text: SaveString,
    pub language: Option<SupportedLanguage>,
}

impl Display for LocalizedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text.brief(56))?;

        if let Some(language) = self.language {
            write!(f, " ({language})")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MapSign {
    pub base: MapObjectBase,
    pub message: LocalizedString,
}

impl Display for MapSign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Sign uid={}, pos=({}, {}), message={}",
            self.base.uid, self.base.map_position.x, self.base.map_position.y, self.message
        )
    }
}

fn brief_rewards(
    funds: &Funds,
    artifact: &Artifact,
    secondary_skill: &SecondarySkill,
    experience: i32,
) -> String {
    let mut parts = Vec::new();

    let funds = funds.brief();
    if !funds.is_empty() {
        parts.push(funds);
    }

    if !matches!(artifact.id, ArtifactID::Unknown(0)) || artifact.ext != 0 {
        parts.push(format!("artifact={}", artifact.id));
    }

    if !matches!(secondary_skill.id, Skill::Unknown(0))
        || !matches!(secondary_skill.level, SkillLevel::Unknown(0))
    {
        parts.push(format!(
            "skill={:?} {:?}",
            secondary_skill.id, secondary_skill.level
        ));
    }

    if experience != 0 {
        parts.push(format!("exp={experience}"));
    }

    parts.join(", ")
}
