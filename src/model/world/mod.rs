pub mod captured_objects;
pub mod castles;
pub mod heroes;
pub mod kingdoms;
pub mod tile;
pub mod timed_events;
pub mod ultimate_artifact;

use std::collections::BTreeMap;
use std::fmt::Display;

use crate::SaveString;
use crate::model::header::map_info::WorldDate;
use crate::model::header::player::PlayerColor;
use crate::model::world::captured_objects::CapturedObject;
use crate::model::world::heroes::id::HeroID;
use crate::model::world::kingdoms::KINGDOM_SLOT_COUNT;
use crate::model::world::timed_events::TimedEvent;
use crate::model::world::ultimate_artifact::UltimateArtifact;

/// Decoded fheroes2 `World` section.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct World {
    /// World width in tiles.
    pub width: i32,
    /// World height in tiles.
    pub height: i32,
    /// Tile records in fheroes2 serialization order.
    pub tiles: Vec<tile::Tile>,
    /// Decoded non-placeholder hero roster.
    pub heroes: Vec<heroes::Hero>,
    /// Decoded castle table.
    pub castles: Vec<castles::Castle>,
    /// Decoded fixed kingdom table.
    pub kingdoms: Vec<kingdoms::Kingdom>,
    /// Decoded custom rumors table.
    pub custom_rumors: Vec<SaveString>,
    /// Decoded timed event list.
    pub timed_events: Vec<TimedEvent>,
    /// Decoded captured object map keyed by tile index.
    pub captured_objects: BTreeMap<i32, CapturedObject>,
    /// Decoded ultimate artifact state.
    pub ultimate_artifact: UltimateArtifact,
    pub world_date: WorldDate,
    pub hero_id_as_win_condition: HeroID,
    pub hero_id_as_lose_condition: HeroID,
}

impl Default for World {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            tiles: Vec::new(),
            heroes: Vec::new(),
            castles: Vec::new(),
            kingdoms: vec![kingdoms::Kingdom::default(); KINGDOM_SLOT_COUNT],
            custom_rumors: Vec::new(),
            timed_events: Vec::new(),
            captured_objects: BTreeMap::new(),
            ultimate_artifact: UltimateArtifact::default(),
            world_date: WorldDate::default(),
            hero_id_as_win_condition: HeroID::Unknown(0),
            hero_id_as_lose_condition: HeroID::Unknown(0),
        }
    }
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let active_heroes = self.heroes.iter().filter(|hero| hero.is_active()).count();
        let in_play_heroes: Vec<&heroes::Hero> = self
            .heroes
            .iter()
            .filter(|hero| hero.is_in_play())
            .collect();
        let hireable_heroes = self
            .heroes
            .iter()
            .filter(|hero| hero.is_available_for_hire())
            .count();
        let jailed_heroes = self.heroes.iter().filter(|hero| hero.is_jailed()).count();

        writeln!(
            f,
            "world: {}x{}, {} tiles, {} kingdoms, {} castles, {} heroes ({} active, {} in play, {} for hire, {} jailed)",
            self.width,
            self.height,
            self.tiles.len(),
            self.kingdoms.len(),
            self.castles.len(),
            self.heroes.len(),
            active_heroes,
            in_play_heroes.len(),
            hireable_heroes,
            jailed_heroes
        )?;
        writeln!(
            f,
            "world extras: {} custom rumors, {} timed events, {} captured objects",
            self.custom_rumors.len(),
            self.timed_events.len(),
            self.captured_objects.len()
        )?;

        let visible_kingdoms: Vec<&kingdoms::Kingdom> = self
            .kingdoms
            .iter()
            .filter(|kingdom| {
                kingdom.color != PlayerColor::None
                    || !kingdom.hero_ids.is_empty()
                    || !kingdom.castle_indexes.is_empty()
            })
            .collect();

        if !visible_kingdoms.is_empty() {
            writeln!(f, "kingdoms:")?;
            for kingdom in visible_kingdoms {
                writeln!(f, "  - {kingdom}")?;
            }
        }

        if !self.custom_rumors.is_empty() {
            writeln!(f, "custom_rumors:")?;
            for rumor in &self.custom_rumors {
                writeln!(f, "  - {}", brief_save_string(rumor, 96))?;
            }
        }

        if !self.timed_events.is_empty() {
            writeln!(f, "timed_events:")?;
            for timed_event in &self.timed_events {
                writeln!(f, "  - {timed_event}")?;
            }
        }

        if self.ultimate_artifact.is_meaningful() {
            writeln!(f, "ultimate_artifact: {}", self.ultimate_artifact)?;
        }

        if !self.castles.is_empty() {
            writeln!(f, "castles:")?;
            for castle in &self.castles {
                writeln!(f, "  - {castle}")?;
            }
        }

        if !in_play_heroes.is_empty() {
            writeln!(f, "heroes:")?;
            for hero in in_play_heroes {
                writeln!(f, "  - {hero}")?;
            }
        }

        Ok(())
    }
}

fn brief_save_string(value: &SaveString, max_chars: usize) -> String {
    let single_line = value.to_string_lossy().replace(['\r', '\n'], " ");
    let total_chars = single_line.chars().count();
    let mut shortened: String = single_line.chars().take(max_chars).collect();
    if total_chars > max_chars {
        shortened.push_str("...");
    }

    format!("{shortened:?}")
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

/// Reference to a world object by tile index and object type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct IndexObject {
    /// Absolute tile index in map order.
    pub tile_index: i32,
    /// Raw `MP2::MapObjectType` value.
    pub object_type: u16,
}

/// Reference to a world object by tile index and object type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Funds {
    pub wood: i32,
    pub mercury: i32,
    pub ore: i32,
    pub sulfur: i32,
    pub crystal: i32,
    pub gems: i32,
    pub gold: i32,
}
