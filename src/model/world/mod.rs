mod direction;

use std::fmt::Display;

use super::{PlayerColor, PlayerColorsSet};

pub use direction::DirectionSet;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct World {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Tile>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Tile {
    pub index: i32,
    pub terrain_image_index: u16,
    pub terrain_flags: u8,
    pub tile_passability_directions: DirectionSet,
    pub main_object_part: ObjectPart,
    pub main_object_type: u16,
    pub fog_colors: PlayerColorsSet,
    pub metadata: Vec<u32>,
    pub occupant_hero_id: u8,
    pub is_tile_marked_as_road: bool,
    pub ground_object_parts: Vec<ObjectPart>,
    pub top_object_parts: Vec<ObjectPart>,
    pub boat_owner_color: PlayerColor,
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "world: {}x{}, {} tiles",
            self.width,
            self.height,
            self.tiles.len()
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LayerType {
    #[default]
    ObjectLayer,
    BackgroundLayer,
    ShadowLayer,
    TerrainLayer,
    UnknownLayer(u8),
}

impl LayerType {
    pub const fn from_byte(value: u8) -> Self {
        match value {
            0 => LayerType::ObjectLayer,
            1 => LayerType::BackgroundLayer,
            2 => LayerType::ShadowLayer,
            3 => LayerType::TerrainLayer,
            other => LayerType::UnknownLayer(other),
        }
    }

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ObjectPart {
    pub layer_type: LayerType,
    pub uid: u32,
    pub icn_type: u8,
    pub icn_index: u8,
}
