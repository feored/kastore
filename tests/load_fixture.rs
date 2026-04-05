use std::fs;

use kastore::{load, Error};

#[test]
fn load_supported_fixtures_reaches_decode_boundary() {
    let fixtures = [
        "tests/saves/10032/Guardian_War_0009.sav",
        "tests/saves/10032/Good_5_Complete.savc",
        "tests/saves/10032/Evil_1_0018.savc",
    ];

    for fixture in fixtures {
        let bytes = fs::read(fixture).unwrap();
        let error = load(&bytes).unwrap_err();

        assert_eq!(error, Error::NotImplemented("save decode"));
    }
}
