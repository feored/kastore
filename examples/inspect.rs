fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fixtures = [
        "tests/saves/10032/Guardian_War_0009.sav",
        "tests/saves/10032/Good_5_Complete.savc",
        "tests/saves/10032/Evil_1_0018.savc",
    ];

    for fixture in fixtures {
        let bytes = std::fs::read(fixture)?;
        let save_game = kastore::load(&bytes)?;

        println!("{fixture}");
        println!("  save_version: {:?}", save_game.source_version);
        println!("  requires_pol: {}", save_game.header.requires_pol);
        println!("  map filename: {}", save_game.map_info.filename);
        println!("  map name: {}", save_game.map_info.name);
        println!("  description: {}", save_game.map_info.description);
        println!("  width: {}", save_game.map_info.width);
        println!("  height: {}", save_game.map_info.height);
        println!("  difficulty: {:?}", save_game.map_info.difficulty);
        println!("  kingdom colors: {}", save_game.map_info.kingdom_colors);
        println!(
            "  colors available for humans: {}",
            save_game.map_info.colors_available_for_humans
        );
        println!(
            "  colors available for computer: {}",
            save_game.map_info.colors_available_for_comp
        );
        println!(
            "  colors of random_races: {}",
            save_game.map_info.colors_of_random_races
        );
    }

    Ok(())
}
