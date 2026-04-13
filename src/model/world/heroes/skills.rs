#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SecondarySkill {
    pub id: Skill,
    pub level: SkillLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Skill {
    Unknown(i32),
    Pathfinding,
    Archery,
    Logistics,
    Scouting,
    Diplomacy,
    Navigation,
    Leadership,
    Wisdom,
    Mysticism,
    Luck,
    Ballistics,
    EagleEye,
    Necromancy,
    Estates,
}

impl Default for Skill {
    fn default() -> Self {
        Skill::Unknown(0)
    }
}

impl Skill {
    pub const fn from_i32(value: i32) -> Self {
        match value {
            1 => Skill::Pathfinding,
            2 => Skill::Archery,
            3 => Skill::Logistics,
            4 => Skill::Scouting,
            5 => Skill::Diplomacy,
            6 => Skill::Navigation,
            7 => Skill::Leadership,
            8 => Skill::Wisdom,
            9 => Skill::Mysticism,
            10 => Skill::Luck,
            11 => Skill::Ballistics,
            12 => Skill::EagleEye,
            13 => Skill::Necromancy,
            14 => Skill::Estates,
            other => Skill::Unknown(other),
        }
    }

    pub const fn to_i32(self) -> i32 {
        match self {
            Skill::Unknown(value) => value,
            Skill::Pathfinding => 1,
            Skill::Archery => 2,
            Skill::Logistics => 3,
            Skill::Scouting => 4,
            Skill::Diplomacy => 5,
            Skill::Navigation => 6,
            Skill::Leadership => 7,
            Skill::Wisdom => 8,
            Skill::Mysticism => 9,
            Skill::Luck => 10,
            Skill::Ballistics => 11,
            Skill::EagleEye => 12,
            Skill::Necromancy => 13,
            Skill::Estates => 14,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillLevel {
    Unknown(i32),
    Basic,
    Advanced,
    Expert,
}

impl Default for SkillLevel {
    fn default() -> Self {
        SkillLevel::Unknown(0)
    }
}

impl SkillLevel {
    pub const fn from_i32(value: i32) -> Self {
        match value {
            1 => SkillLevel::Basic,
            2 => SkillLevel::Advanced,
            3 => SkillLevel::Expert,
            other => SkillLevel::Unknown(other),
        }
    }

    pub const fn to_i32(self) -> i32 {
        match self {
            SkillLevel::Unknown(value) => value,
            SkillLevel::Basic => 1,
            SkillLevel::Advanced => 2,
            SkillLevel::Expert => 3,
        }
    }
}
