use std::fmt::Display;

/// Single hero movement direction value.
///
/// This is distinct from the tile-layer `DirectionSet` bitmask.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Direction {
    #[default]
    Unknown,
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    Center,
    Raw(i32),
}

impl Direction {
    pub const fn from_i32(value: i32) -> Self {
        match value {
            0x0000 => Direction::Unknown,
            0x0001 => Direction::TopLeft,
            0x0002 => Direction::Top,
            0x0004 => Direction::TopRight,
            0x0008 => Direction::Right,
            0x0010 => Direction::BottomRight,
            0x0020 => Direction::Bottom,
            0x0040 => Direction::BottomLeft,
            0x0080 => Direction::Left,
            0x0100 => Direction::Center,
            other => Direction::Raw(other),
        }
    }

    pub const fn to_i32(self) -> i32 {
        match self {
            Direction::Unknown => 0x0000,
            Direction::TopLeft => 0x0001,
            Direction::Top => 0x0002,
            Direction::TopRight => 0x0004,
            Direction::Right => 0x0008,
            Direction::BottomRight => 0x0010,
            Direction::Bottom => 0x0020,
            Direction::BottomLeft => 0x0040,
            Direction::Left => 0x0080,
            Direction::Center => 0x0100,
            Direction::Raw(value) => value,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Unknown => f.write_str("Unknown (0x0000)"),
            Direction::Raw(value) => write!(f, "Raw direction 0x{value:04X}"),
            known => write!(f, "{known:?} (0x{:04X})", known.to_i32()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Path {
    pub hidden: bool,
    pub steps: Vec<RouteStep>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RouteStep {
    pub from_index: i32,
    pub direction: Direction,
    pub movement_cost: u32,
}

impl Display for RouteStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -> {} (cost {})",
            self.from_index, self.direction, self.movement_cost
        )
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let visibility = if self.hidden { "hidden" } else { "shown" };
        write!(f, "{visibility}, {} steps", self.steps.len())
    }
}
