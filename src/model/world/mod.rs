pub mod castles;
pub mod heroes;
pub mod kingdoms;
pub mod tile;

use std::fmt::Display;

/// Decoded fheroes2 `World` section.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
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
            "world: {}x{}, {} tiles, {} castles, {} heroes ({} active, {} in play, {} for hire, {} jailed)",
            self.width,
            self.height,
            self.tiles.len(),
            self.castles.len(),
            self.heroes.len(),
            active_heroes,
            in_play_heroes.len(),
            hireable_heroes,
            jailed_heroes
        )?;

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
