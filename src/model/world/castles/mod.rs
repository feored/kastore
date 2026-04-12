use std::fmt::Display;

pub mod buildings;

use crate::internal::save_string::SaveString;
use crate::model::header::player::{PlayerColor, Race};
use crate::model::world::MapPosition;
use crate::model::world::heroes::HeroBase;
use crate::model::world::heroes::army::Army;
use crate::model::world::heroes::spells::Spell;

use self::buildings::{CastleBuildingSet, CastleDwellingTier, CastleDwellings};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Castle {
    pub map_position: MapPosition,
    pub mode: CastleModeSet,
    pub race: Race,
    pub constructed_buildings: CastleBuildingSet,
    pub disabled_buildings: CastleBuildingSet,
    pub color_base: PlayerColor,
    pub captain: HeroBase,
    pub name: SaveString,
    pub mage_guild_spells: MageGuild,
    pub dwellings: CastleDwellings,
    pub army: Army,
}

impl Castle {
    pub const fn can_build_today(&self) -> bool {
        self.mode.can_build()
    }

    pub const fn dwelling_count(&self, tier: CastleDwellingTier) -> u32 {
        self.dwellings.count(tier)
    }
}

impl Display for Castle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = if self.name.is_empty() {
            "Unnamed castle".to_string()
        } else {
            self.name.to_string_lossy()
        };

        write!(
            f,
            "{} [color={}, race={}, pos=({}, {}), can_build_today={}, dwellings={}, army={}]",
            name,
            self.color_base,
            self.race,
            self.map_position.x,
            self.map_position.y,
            self.can_build_today(),
            self.dwellings,
            self.army
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MageGuild {
    pub spells: Vec<Spell>,
    pub library_spells: Vec<Spell>,
}

/// fheroes2 castle mode bitset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CastleModeSet(u32);

impl CastleModeSet {
    pub const EMPTY: Self = Self(0x0000_0000);
    pub const UNUSED_ALLOW_CASTLE_CONSTRUCTION: Self = Self(0x0000_0002);
    pub const CUSTOM_ARMY: Self = Self(0x0000_0004);
    pub const ALLOW_TO_BUILD_TODAY: Self = Self(0x0000_0008);

    /// Build from raw castle mode bits.
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Return the raw mode bits.
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Return whether all bits in `flags` are set.
    pub const fn contains(self, flags: Self) -> bool {
        (self.0 & flags.0) == flags.0
    }

    pub const fn can_build(self) -> bool {
        self.contains(Self::ALLOW_TO_BUILD_TODAY)
    }

    /// Set all bits in `flags`.
    pub fn insert(&mut self, flags: Self) {
        self.0 |= flags.0;
    }

    /// Clear all bits in `flags`.
    pub fn remove(&mut self, flags: Self) {
        self.0 &= !flags.0;
    }
}

impl Display for CastleModeSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == Self::EMPTY.0 {
            return f.write_str("None (0x00000000)");
        }

        let mut parts = Vec::new();
        for &(flag, name) in [
            (
                Self::UNUSED_ALLOW_CASTLE_CONSTRUCTION,
                "UnusedAllowCastleConstruction",
            ),
            (Self::CUSTOM_ARMY, "CustomArmy"),
            (Self::ALLOW_TO_BUILD_TODAY, "AllowToBuildToday"),
        ]
        .iter()
        {
            if self.contains(flag) {
                parts.push(name);
            }
        }

        write!(f, "{} (0x{:08X})", parts.join(" | "), self.0)
    }
}
