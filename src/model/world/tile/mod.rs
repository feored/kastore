mod direction;

pub use direction::DirectionSet;

use crate::model::{PlayerColor, PlayerColorsSet};

/// Serialized fheroes2 tile record.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Tile {
    /// Tile index in map order.
    pub index: i32,
    /// Raw terrain image index.
    pub terrain_image_index: u16,
    /// Raw terrain flag byte.
    pub terrain_flags: u8,
    /// Direction/passability bitset.
    pub tile_passability_directions: DirectionSet,
    /// Main object part on this tile.
    pub main_object_part: ObjectPart,
    /// Raw main object type id.
    pub main_object_type: u16,
    /// Player colors that have fog on this tile.
    pub fog_colors: PlayerColorsSet,
    /// Counted metadata values. Meaning depends on the object type.
    pub metadata: Vec<u32>,
    /// Raw cached occupant hero id (`uint8_t` in fheroes2).
    pub occupant_hero_id: u8,
    /// Whether this tile is marked as road.
    pub is_tile_marked_as_road: bool,
    /// Ground object parts on this tile.
    pub ground_object_parts: Vec<ObjectPart>,
    /// Top object parts on this tile.
    pub top_object_parts: Vec<ObjectPart>,
    /// Boat owner color stored on this tile.
    pub boat_owner_color: PlayerColor,
}

/// fheroes2 object layer id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LayerType {
    #[default]
    ObjectLayer,
    BackgroundLayer,
    ShadowLayer,
    TerrainLayer,
    /// Preserved raw value not known by this crate.
    UnknownLayer(u8),
}

impl LayerType {
    /// Build from the raw save byte.
    pub const fn from_byte(value: u8) -> Self {
        match value {
            0 => LayerType::ObjectLayer,
            1 => LayerType::BackgroundLayer,
            2 => LayerType::ShadowLayer,
            3 => LayerType::TerrainLayer,
            other => LayerType::UnknownLayer(other),
        }
    }

    /// Return the raw save byte.
    pub const fn to_byte(self) -> u8 {
        match self {
            LayerType::ObjectLayer => 0,
            LayerType::BackgroundLayer => 1,
            LayerType::ShadowLayer => 2,
            LayerType::TerrainLayer => 3,
            LayerType::UnknownLayer(other) => other,
        }
    }
}

/// One serialized object part.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ObjectPart {
    /// Layer this part belongs to.
    pub layer_type: LayerType,
    /// Unique object part id from fheroes2.
    pub uid: u32,
    /// Raw ICN sprite sheet id.
    pub icn_type: u8,
    /// Raw index inside the ICN sprite sheet.
    pub icn_index: u8,
}
