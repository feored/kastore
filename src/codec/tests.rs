use super::GameType;

#[test]
fn game_type_extension_mapping_round_trips() {
    let game_type = GameType::Standard;
    assert_eq!(game_type.extension(), "sav");

    let game_type = GameType::Campaign;
    assert_eq!(game_type.extension(), "savc");

    let game_type = GameType::Hotseat;
    assert_eq!(game_type.extension(), "savh");

    let extension = "sav";
    assert_eq!(
        GameType::from_extension(extension),
        Some(GameType::Standard)
    );

    let extension = "savc";
    assert_eq!(
        GameType::from_extension(extension),
        Some(GameType::Campaign)
    );

    let extension = "savh";
    assert_eq!(GameType::from_extension(extension), Some(GameType::Hotseat));
}
