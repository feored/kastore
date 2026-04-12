use std::fmt::Display;

pub mod army;
pub mod artifact;
pub mod id;
pub mod modes;
pub mod path;
pub mod skills;
pub mod spells;

use crate::{
    internal::save_string::SaveString,
    model::header::player::{PlayerColor, Race},
    model::world::{IndexObject, MapPosition, Point},
};

use self::army::Army;
use self::artifact::Artifact;
use self::id::HeroID;
use self::modes::HeroModeSet;
use self::path::{Direction, Path};
use self::skills::SecondarySkill;
use self::spells::Spell;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Hero {
    pub base: HeroBase,
    pub name: SaveString,
    pub color_base: PlayerColor,
    pub experience: u32,
    pub secondary_skills: Vec<SecondarySkill>,
    pub army: Army,
    pub id: HeroID,
    pub portrait: i32,
    pub race: Race,
    pub object_type_under_hero: u16,
    pub path: Path,
    pub direction: Direction,
    pub sprite_index: i32,
    pub patrol_center: Point,
    pub patrol_distance: u32,
    pub visited_objects: Vec<IndexObject>,
    pub last_ground_region: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HeroBase {
    pub primary_skills: PrimarySkills,
    pub map_position: MapPosition,
    pub modes: HeroModeSet,
    pub spell_points: u32,
    pub move_points: u32,
    pub spell_book: Vec<Spell>,
    pub bag_artifacts: Vec<Artifact>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PrimarySkills {
    pub attack: i32,
    pub defense: i32,
    pub knowledge: i32,
    pub power: i32,
}

impl Display for PrimarySkills {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A{} D{} K{} P{}",
            self.attack, self.defense, self.knowledge, self.power
        )
    }
}

impl Hero {
    pub fn is_placeholder(&self) -> bool {
        matches!(self.id, HeroID::Unknown(_))
    }

    pub fn is_debug(&self) -> bool {
        matches!(self.id, HeroID::Debug)
    }

    pub fn is_jailed(&self) -> bool {
        self.base.modes.contains(HeroModeSet::JAIL)
    }

    pub fn is_recruit_offer(&self) -> bool {
        self.base.modes.contains(HeroModeSet::RECRUIT)
    }

    pub fn is_available_for_hire(&self) -> bool {
        self.is_meaningful() && self.color_base == PlayerColor::None && !self.is_jailed()
    }

    pub fn is_active(&self) -> bool {
        self.is_meaningful()
            && matches!(
                self.color_base,
                PlayerColor::Blue
                    | PlayerColor::Green
                    | PlayerColor::Red
                    | PlayerColor::Yellow
                    | PlayerColor::Orange
                    | PlayerColor::Purple
            )
            && !self.is_jailed()
    }

    pub fn is_in_play(&self) -> bool {
        self.is_meaningful() && !self.is_available_for_hire() && !self.is_recruit_offer()
    }

    pub fn is_meaningful(&self) -> bool {
        !self.is_placeholder() && !self.is_debug()
    }
}

impl Display for Hero {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = if self.name.is_empty() {
            self.id.to_string()
        } else {
            self.name.to_string_lossy()
        };
        let status = if self.is_jailed() {
            "jailed"
        } else if self.is_active() {
            "active"
        } else if self.is_in_play() {
            "in-play"
        } else if self.is_available_for_hire() {
            "for-hire"
        } else {
            "inactive"
        };
        let status = if self.is_recruit_offer() {
            format!("{status}, recruit-offer")
        } else {
            status.to_string()
        };

        write!(
            f,
            "{} [{}] status={}, color={}, race={}, pos=({}, {}), xp={}, skills={}, mana={}, move={}, army={}, path={}, visited={}, ground_region={}",
            name,
            self.id,
            status,
            self.color_base,
            self.race,
            self.base.map_position.x,
            self.base.map_position.y,
            self.experience,
            self.base.primary_skills,
            self.base.spell_points,
            self.base.move_points,
            self.army,
            self.path,
            self.visited_objects.len(),
            self.last_ground_region
        )
    }
}
