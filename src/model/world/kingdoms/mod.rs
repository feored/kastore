use std::fmt::Display;

use crate::model::header::player::PlayerColor;
use crate::model::world::heroes::id::HeroID;
use crate::model::world::{Funds, IndexObject};

pub const KINGDOM_SLOT_COUNT: usize = 7;

pub const PUZZLE_REVEALED_TILES_COUNT: usize = 48;
pub const PUZZLE_ZONE_COUNTS: [usize; 4] = [24, 16, 4, 4];

const PUZZLE_ZONE2_START: u8 = PUZZLE_ZONE_COUNTS[0] as u8;
const PUZZLE_ZONE3_START: u8 = PUZZLE_ZONE2_START + PUZZLE_ZONE_COUNTS[1] as u8;
const PUZZLE_ZONE4_START: u8 = PUZZLE_ZONE3_START + PUZZLE_ZONE_COUNTS[2] as u8;

const DEFAULT_PUZZLE_ZONE1_ORDER: [u8; 24] = puzzle_zone_order(0);
const DEFAULT_PUZZLE_ZONE2_ORDER: [u8; 16] = puzzle_zone_order(PUZZLE_ZONE2_START);
const DEFAULT_PUZZLE_ZONE3_ORDER: [u8; 4] = puzzle_zone_order(PUZZLE_ZONE3_START);
const DEFAULT_PUZZLE_ZONE4_ORDER: [u8; 4] = puzzle_zone_order(PUZZLE_ZONE4_START);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Kingdom {
    pub mode: KingdomModeSet,
    pub color: PlayerColor,
    pub funds: Funds,
    pub lost_town_days: u32,
    pub castle_indexes: Vec<i32>,
    pub hero_ids: Vec<HeroID>,
    pub recruits: KingdomRecruits,
    pub visited_objects: Vec<IndexObject>,
    pub puzzle: KingdomPuzzle,
    pub visited_tents_colors: i32,
    pub top_castle_in_kingdom_view: i32,
    pub top_hero_in_kingdom_view: i32,
}

impl Default for Kingdom {
    fn default() -> Self {
        Self {
            mode: KingdomModeSet::default(),
            color: PlayerColor::None,
            funds: Funds::default(),
            lost_town_days: 0,
            castle_indexes: Vec::new(),
            hero_ids: Vec::new(),
            recruits: KingdomRecruits::default(),
            visited_objects: Vec::new(),
            puzzle: KingdomPuzzle::default(),
            visited_tents_colors: 0,
            top_castle_in_kingdom_view: -1,
            top_hero_in_kingdom_view: -1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KingdomRecruit {
    pub hero_id: HeroID,
    pub surrender_day: u32,
}

impl Default for KingdomRecruit {
    fn default() -> Self {
        Self {
            hero_id: HeroID::Unknown(0),
            surrender_day: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct KingdomRecruits {
    pub first: KingdomRecruit,
    pub second: KingdomRecruit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KingdomPuzzle {
    /// 48 ASCII bytes ('0' or '1') serialized as a length-prefixed byte blob.
    pub revealed_tiles: Vec<u8>,
    pub zone1_order: Vec<u8>,
    pub zone2_order: Vec<u8>,
    pub zone3_order: Vec<u8>,
    pub zone4_order: Vec<u8>,
}

impl Default for KingdomPuzzle {
    fn default() -> Self {
        Self {
            revealed_tiles: vec![b'0'; PUZZLE_REVEALED_TILES_COUNT],
            zone1_order: DEFAULT_PUZZLE_ZONE1_ORDER.to_vec(),
            zone2_order: DEFAULT_PUZZLE_ZONE2_ORDER.to_vec(),
            zone3_order: DEFAULT_PUZZLE_ZONE3_ORDER.to_vec(),
            zone4_order: DEFAULT_PUZZLE_ZONE4_ORDER.to_vec(),
        }
    }
}

const fn puzzle_zone_order<const N: usize>(start: u8) -> [u8; N] {
    let mut order = [0; N];
    let mut index = 0;
    while index < N {
        order[index] = start + index as u8;
        index += 1;
    }
    order
}

// fheroes2 kingdom mode bitset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct KingdomModeSet(u32);

impl KingdomModeSet {
    pub const IDENTIFYHERO: Self = Self(0x0000_0002);
    pub const KINGDOM_OVERVIEW_CASTLE_SELECTION: Self = Self(0x0000_0008);

    /// Build from raw kingdom mode bits.
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Return the raw mode bits.
    pub const fn bits(self) -> u32 {
        self.0
    }
}

impl Display for Kingdom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut recruit_parts = Vec::new();
        for recruit in [self.recruits.first, self.recruits.second] {
            if recruit.hero_id.to_i32() == 0 {
                continue;
            }

            if recruit.surrender_day == 0 {
                recruit_parts.push(recruit.hero_id.to_string());
            } else {
                recruit_parts.push(format!(
                    "{} (day {})",
                    recruit.hero_id, recruit.surrender_day
                ));
            }
        }

        write!(
            f,
            "{} [castles={}, heroes={}, recruits={}, funds={}wood/{}mercury/{}ore/{}sulfur/{}crystal/{}gems/{} gold, visited={}",
            self.color,
            self.castle_indexes.len(),
            self.hero_ids.len(),
            if recruit_parts.is_empty() {
                "none".to_string()
            } else {
                recruit_parts.join(", ")
            },
            self.funds.wood,
            self.funds.mercury,
            self.funds.ore,
            self.funds.sulfur,
            self.funds.crystal,
            self.funds.gems,
            self.funds.gold,
            self.visited_objects.len()
        )?;

        if self.lost_town_days != 0 {
            write!(f, ", lost_town_days={}", self.lost_town_days)?;
        }

        if self.visited_tents_colors != 0 {
            write!(f, ", tents=0x{:08X}", self.visited_tents_colors as u32)?;
        }

        write!(f, "]")
    }
}
