use std::fmt::Display;

use crate::model::header::player::PlayerColor;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Army {
    pub troops: Vec<Troop>,
    pub spread_combat_formation: bool,
    pub player_color: PlayerColor,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Troop {
    pub monster: MonsterType,
    pub count: u32,
}

/// fheroes2 monster id.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonsterType {
    Unknown(i32),
    Peasant,
    Archer,
    Ranger,
    Pikeman,
    VeteranPikeman,
    Swordsman,
    MasterSwordsman,
    Cavalry,
    Champion,
    Paladin,
    Crusader,
    Goblin,
    Orc,
    OrcChief,
    Wolf,
    Ogre,
    OgreLord,
    Troll,
    WarTroll,
    Cyclops,
    Sprite,
    Dwarf,
    BattleDwarf,
    Elf,
    GrandElf,
    Druid,
    GreaterDruid,
    Unicorn,
    Phoenix,
    Centaur,
    Gargoyle,
    Griffin,
    Minotaur,
    MinotaurKing,
    Hydra,
    GreenDragon,
    RedDragon,
    BlackDragon,
    Halfling,
    Boar,
    IronGolem,
    SteelGolem,
    Roc,
    Mage,
    Archmage,
    Giant,
    Titan,
    Skeleton,
    Zombie,
    MutantZombie,
    Mummy,
    RoyalMummy,
    Vampire,
    VampireLord,
    Lich,
    PowerLich,
    BoneDragon,
    Rogue,
    Nomad,
    Ghost,
    Genie,
    Medusa,
    EarthElement,
    AirElement,
    FireElement,
    WaterElement,
    RandomMonster,
    RandomMonsterLevel1,
    RandomMonsterLevel2,
    RandomMonsterLevel3,
    RandomMonsterLevel4,
}

impl MonsterType {
    pub const fn from_i32(value: i32) -> Self {
        match value {
            1 => MonsterType::Peasant,
            2 => MonsterType::Archer,
            3 => MonsterType::Ranger,
            4 => MonsterType::Pikeman,
            5 => MonsterType::VeteranPikeman,
            6 => MonsterType::Swordsman,
            7 => MonsterType::MasterSwordsman,
            8 => MonsterType::Cavalry,
            9 => MonsterType::Champion,
            10 => MonsterType::Paladin,
            11 => MonsterType::Crusader,
            12 => MonsterType::Goblin,
            13 => MonsterType::Orc,
            14 => MonsterType::OrcChief,
            15 => MonsterType::Wolf,
            16 => MonsterType::Ogre,
            17 => MonsterType::OgreLord,
            18 => MonsterType::Troll,
            19 => MonsterType::WarTroll,
            20 => MonsterType::Cyclops,
            21 => MonsterType::Sprite,
            22 => MonsterType::Dwarf,
            23 => MonsterType::BattleDwarf,
            24 => MonsterType::Elf,
            25 => MonsterType::GrandElf,
            26 => MonsterType::Druid,
            27 => MonsterType::GreaterDruid,
            28 => MonsterType::Unicorn,
            29 => MonsterType::Phoenix,
            30 => MonsterType::Centaur,
            31 => MonsterType::Gargoyle,
            32 => MonsterType::Griffin,
            33 => MonsterType::Minotaur,
            34 => MonsterType::MinotaurKing,
            35 => MonsterType::Hydra,
            36 => MonsterType::GreenDragon,
            37 => MonsterType::RedDragon,
            38 => MonsterType::BlackDragon,
            39 => MonsterType::Halfling,
            40 => MonsterType::Boar,
            41 => MonsterType::IronGolem,
            42 => MonsterType::SteelGolem,
            43 => MonsterType::Roc,
            44 => MonsterType::Mage,
            45 => MonsterType::Archmage,
            46 => MonsterType::Giant,
            47 => MonsterType::Titan,
            48 => MonsterType::Skeleton,
            49 => MonsterType::Zombie,
            50 => MonsterType::MutantZombie,
            51 => MonsterType::Mummy,
            52 => MonsterType::RoyalMummy,
            53 => MonsterType::Vampire,
            54 => MonsterType::VampireLord,
            55 => MonsterType::Lich,
            56 => MonsterType::PowerLich,
            57 => MonsterType::BoneDragon,
            58 => MonsterType::Rogue,
            59 => MonsterType::Nomad,
            60 => MonsterType::Ghost,
            61 => MonsterType::Genie,
            62 => MonsterType::Medusa,
            63 => MonsterType::EarthElement,
            64 => MonsterType::AirElement,
            65 => MonsterType::FireElement,
            66 => MonsterType::WaterElement,
            67 => MonsterType::RandomMonster,
            68 => MonsterType::RandomMonsterLevel1,
            69 => MonsterType::RandomMonsterLevel2,
            70 => MonsterType::RandomMonsterLevel3,
            71 => MonsterType::RandomMonsterLevel4,
            other => MonsterType::Unknown(other),
        }
    }

    pub const fn to_i32(self) -> i32 {
        match self {
            MonsterType::Unknown(value) => value,
            MonsterType::Peasant => 1,
            MonsterType::Archer => 2,
            MonsterType::Ranger => 3,
            MonsterType::Pikeman => 4,
            MonsterType::VeteranPikeman => 5,
            MonsterType::Swordsman => 6,
            MonsterType::MasterSwordsman => 7,
            MonsterType::Cavalry => 8,
            MonsterType::Champion => 9,
            MonsterType::Paladin => 10,
            MonsterType::Crusader => 11,
            MonsterType::Goblin => 12,
            MonsterType::Orc => 13,
            MonsterType::OrcChief => 14,
            MonsterType::Wolf => 15,
            MonsterType::Ogre => 16,
            MonsterType::OgreLord => 17,
            MonsterType::Troll => 18,
            MonsterType::WarTroll => 19,
            MonsterType::Cyclops => 20,
            MonsterType::Sprite => 21,
            MonsterType::Dwarf => 22,
            MonsterType::BattleDwarf => 23,
            MonsterType::Elf => 24,
            MonsterType::GrandElf => 25,
            MonsterType::Druid => 26,
            MonsterType::GreaterDruid => 27,
            MonsterType::Unicorn => 28,
            MonsterType::Phoenix => 29,
            MonsterType::Centaur => 30,
            MonsterType::Gargoyle => 31,
            MonsterType::Griffin => 32,
            MonsterType::Minotaur => 33,
            MonsterType::MinotaurKing => 34,
            MonsterType::Hydra => 35,
            MonsterType::GreenDragon => 36,
            MonsterType::RedDragon => 37,
            MonsterType::BlackDragon => 38,
            MonsterType::Halfling => 39,
            MonsterType::Boar => 40,
            MonsterType::IronGolem => 41,
            MonsterType::SteelGolem => 42,
            MonsterType::Roc => 43,
            MonsterType::Mage => 44,
            MonsterType::Archmage => 45,
            MonsterType::Giant => 46,
            MonsterType::Titan => 47,
            MonsterType::Skeleton => 48,
            MonsterType::Zombie => 49,
            MonsterType::MutantZombie => 50,
            MonsterType::Mummy => 51,
            MonsterType::RoyalMummy => 52,
            MonsterType::Vampire => 53,
            MonsterType::VampireLord => 54,
            MonsterType::Lich => 55,
            MonsterType::PowerLich => 56,
            MonsterType::BoneDragon => 57,
            MonsterType::Rogue => 58,
            MonsterType::Nomad => 59,
            MonsterType::Ghost => 60,
            MonsterType::Genie => 61,
            MonsterType::Medusa => 62,
            MonsterType::EarthElement => 63,
            MonsterType::AirElement => 64,
            MonsterType::FireElement => 65,
            MonsterType::WaterElement => 66,
            MonsterType::RandomMonster => 67,
            MonsterType::RandomMonsterLevel1 => 68,
            MonsterType::RandomMonsterLevel2 => 69,
            MonsterType::RandomMonsterLevel3 => 70,
            MonsterType::RandomMonsterLevel4 => 71,
        }
    }
}

impl Default for MonsterType {
    fn default() -> Self {
        MonsterType::Unknown(0)
    }
}

impl Display for MonsterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MonsterType::Unknown(value) => write!(f, "Unknown monster {value}"),
            known => write!(f, "{known:?} ({})", known.to_i32()),
        }
    }
}

impl Display for Troop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} x{}", self.monster, self.count)
    }
}

impl Display for Army {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let troops: Vec<String> = self
            .troops
            .iter()
            .filter(|troop| troop.count > 0)
            .map(ToString::to_string)
            .collect();

        let formation = if self.spread_combat_formation {
            "spread"
        } else {
            "grouped"
        };

        if troops.is_empty() {
            write!(f, "{} formation, empty", formation)
        } else {
            write!(f, "{} formation, {}", formation, troops.join(", "))
        }
    }
}
