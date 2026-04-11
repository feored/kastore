mod heroes;
mod index_object;
mod tile;

use std::fmt::Display;

pub use heroes::{
    Army, Artifact, ArtifactID, Direction, Hero, HeroBase, HeroID, HeroModeSet, MonsterType, Path,
    PrimarySkills, RouteStep, SecondarySkill, Skill, SkillLevel, Spell, Troop,
};
pub use index_object::IndexObject;
pub use tile::{DirectionSet, LayerType, ObjectPart, Tile};

/// Decoded fheroes2 `World` section.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct World {
    /// World width in tiles.
    pub width: i32,
    /// World height in tiles.
    pub height: i32,
    /// Tile records in fheroes2 serialization order.
    pub tiles: Vec<Tile>,
    /// Decoded non-placeholder hero roster.
    pub heroes: Vec<Hero>,
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let active_heroes = self.heroes.iter().filter(|hero| hero.is_active()).count();
        let in_play_heroes: Vec<&Hero> = self.heroes.iter().filter(|hero| hero.is_in_play()).collect();
        let hireable_heroes = self
            .heroes
            .iter()
            .filter(|hero| hero.is_available_for_hire())
            .count();
        let jailed_heroes = self.heroes.iter().filter(|hero| hero.is_jailed()).count();

        writeln!(
            f,
            "world: {}x{}, {} tiles, {} heroes ({} active, {} in play, {} for hire, {} jailed)",
            self.width,
            self.height,
            self.tiles.len(),
            self.heroes.len(),
            active_heroes,
            in_play_heroes.len(),
            hireable_heroes,
            jailed_heroes
        )?;

        if in_play_heroes.is_empty() {
            return Ok(());
        }

        writeln!(f, "heroes:")?;
        for hero in in_play_heroes {
            writeln!(f, "  - {hero}")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MapPosition {
    pub x: i16,
    pub y: i16,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
