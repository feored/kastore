/// Reference to a world object by tile index and object type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct IndexObject {
    /// Absolute tile index in map order.
    pub tile_index: i32,
    /// Raw `MP2::MapObjectType` value.
    pub object_type: u16,
}
