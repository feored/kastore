use crate::Error;
use crate::SaveString;
use crate::internal::reader::Reader;
use crate::model::{
    Army, Artifact, ArtifactID, Direction, Hero, HeroBase, HeroID, HeroModeSet, IndexObject,
    MapPosition, MonsterType, Path, PlayerColor, Point, PrimarySkills, Race, RouteStep,
    SecondarySkill, Skill, SkillLevel, Spell, Troop,
};

const EXPECTED_HEROES_COUNT: u32 = 73;

pub(super) fn decode(reader: &mut Reader<'_>) -> std::result::Result<Vec<Hero>, Error> {
    let count_offset = reader.position();
    let count = reader.read_u32_be("heroes count")?;
    if count != EXPECTED_HEROES_COUNT {
        return Err(reader.invalid_value(
            "heroes count",
            count_offset,
            "unexpected heroes table size, expected 73",
        ));
    }
    let mut heroes = Vec::with_capacity(usize::try_from(count).unwrap_or(0));
    for _ in 0..count {
        let hero = decode_hero(reader)?;
        if hero.is_meaningful() {
            heroes.push(hero);
        }
    }
    Ok(heroes)
}

fn decode_hero(reader: &mut Reader<'_>) -> std::result::Result<Hero, Error> {
    let base = decode_hero_base(reader)?;
    let name = reader.read_string_bytes("hero name")?;
    let color_base = PlayerColor::from_bits(reader.read_u8("hero color base")?);
    let experience = reader.read_u32_be("hero experience")?;
    let secondary_skills = decode_secondary_skills(reader)?;
    let army = decode_army(reader)?;
    let id = HeroID::from_i32(reader.read_i32_be("hero id")?);
    let portrait = reader.read_i32_be("hero portrait")?;
    let race = Race::from_i32(reader.read_i32_be("hero race")?);
    let object_type_under_hero = reader.read_u16_be("hero object type under hero")?;
    let path = decode_path(reader)?;
    let direction = Direction::from_i32(reader.read_i32_be("hero direction")?);
    let sprite_index = reader.read_i32_be("hero sprite index")?;
    let patrol_center = Point {
        x: reader.read_i32_be("hero patrol center x")?,
        y: reader.read_i32_be("hero patrol center y")?,
    };
    let patrol_distance = reader.read_u32_be("hero patrol distance")?;
    let visited_objects = decode_visited_objects(reader)?;
    let last_ground_region = reader.read_u32_be("hero last ground region")?;

    Ok(Hero {
        base,
        name: SaveString::from(name),
        color_base,
        experience,
        secondary_skills,
        army,
        id,
        portrait,
        race,
        object_type_under_hero,
        path,
        direction,
        sprite_index,
        patrol_center,
        patrol_distance,
        visited_objects,
        last_ground_region,
    })
}

fn decode_hero_base(reader: &mut Reader<'_>) -> std::result::Result<HeroBase, Error> {
    let primary_skills = PrimarySkills {
        attack: reader.read_i32_be("hero primary skill attack")?,
        defense: reader.read_i32_be("hero primary skill defense")?,
        knowledge: reader.read_i32_be("hero primary skill knowledge")?,
        power: reader.read_i32_be("hero primary skill power")?,
    };
    let map_position = MapPosition {
        x: reader.read_i16_be("hero map position x")?,
        y: reader.read_i16_be("hero map position y")?,
    };
    let modes = HeroModeSet::from_bits(reader.read_u32_be("hero modes")?);
    let spell_points = reader.read_u32_be("hero spell points")?;
    let move_points = reader.read_u32_be("hero move points")?;
    let spells_count = reader.read_u32_be("hero spell book count")?;
    let mut spell_book = Vec::with_capacity(usize::try_from(spells_count).unwrap_or(0));
    for _ in 0..spells_count {
        spell_book.push(Spell::from_i32(
            reader.read_i32_be("hero spell book spell")?,
        ));
    }
    let artifacts_count = reader.read_u32_be("hero bag artifacts count")?;
    let mut bag_artifacts = Vec::with_capacity(usize::try_from(artifacts_count).unwrap_or(0));
    for _ in 0..artifacts_count {
        let artifact: Artifact = Artifact {
            id: ArtifactID::from_i32(reader.read_i32_be("hero bag artifact")?),
            ext: reader.read_i32_be("hero bag artifact")?,
        };
        bag_artifacts.push(artifact);
    }
    Ok(HeroBase {
        primary_skills,
        map_position,
        modes,
        spell_points,
        move_points,
        spell_book,
        bag_artifacts,
    })
}

fn decode_secondary_skills(
    reader: &mut Reader<'_>,
) -> std::result::Result<Vec<SecondarySkill>, Error> {
    let secondary_skills_count_offset = reader.position();
    let secondary_skills_count = reader.read_u32_be("hero secondary skills count")?;
    if secondary_skills_count > 8 {
        return Err(reader.invalid_value(
            "hero secondary skills count",
            secondary_skills_count_offset,
            "too many hero secondary skills",
        ));
    }
    let mut secondary_skills =
        Vec::with_capacity(usize::try_from(secondary_skills_count).unwrap_or(0));
    for _ in 0..secondary_skills_count {
        secondary_skills.push(SecondarySkill {
            id: Skill::from_i32(reader.read_i32_be("hero secondary skill")?),
            level: SkillLevel::from_i32(reader.read_i32_be("hero secondary skill level")?),
        });
    }
    Ok(secondary_skills)
}

fn decode_army(reader: &mut Reader<'_>) -> std::result::Result<Army, Error> {
    let troops_count_offset = reader.position();
    let troops_count = reader.read_u32_be("hero army troops count")?;
    if troops_count != 5 {
        return Err(reader.invalid_value(
            "hero army troops count",
            troops_count_offset,
            "unexpected hero army troops count, expected 5",
        ));
    }
    let mut troops = Vec::with_capacity(usize::try_from(troops_count).unwrap_or(0));
    for _ in 0..troops_count {
        troops.push(Troop {
            monster: MonsterType::from_i32(reader.read_i32_be("hero army troop monster type")?),
            count: reader.read_u32_be("hero army troop count")?,
        });
    }
    let army: Army = Army {
        troops,
        spread_combat_formation: reader.read_byte_as_bool("spread combat formation")?,
        player_color: PlayerColor::from_bits(reader.read_u8("army player color")?),
    };
    Ok(army)
}

fn decode_path(reader: &mut Reader<'_>) -> std::result::Result<Path, Error> {
    let hidden = reader.read_byte_as_bool("hero path hidden")?;
    let steps_count = reader.read_u32_be("hero path steps count")?;
    let mut steps = Vec::with_capacity(usize::try_from(steps_count).unwrap_or(0));
    for _ in 0..steps_count {
        steps.push(RouteStep {
            from_index: reader.read_i32_be("hero path step from index")?,
            direction: Direction::from_i32(reader.read_i32_be("hero path step direction")?),
            movement_cost: reader.read_u32_be("hero path step movement cost")?,
        });
    }
    Ok(Path { hidden, steps })
}

fn decode_visited_objects(reader: &mut Reader<'_>) -> std::result::Result<Vec<IndexObject>, Error> {
    let visited_count = reader.read_u32_be("hero visited objects count")?;
    let mut visited_objects = Vec::with_capacity(usize::try_from(visited_count).unwrap_or(0));
    for _ in 0..visited_count {
        visited_objects.push(IndexObject {
            tile_index: reader.read_i32_be("hero visited object tile index")?,
            object_type: reader.read_u16_be("hero visited object type")?,
        });
    }
    Ok(visited_objects)
}
