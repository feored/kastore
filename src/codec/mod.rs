use crate::model::SaveGame;
use crate::version::{profile_for, SaveVersion};
use crate::Error;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameType {
    Standard,
    Campaign,
    Hotseat,
}

#[allow(dead_code)]
impl GameType {
    fn extension(&self) -> &str {
        match self {
            GameType::Standard => "sav",
            GameType::Campaign => "savc",
            GameType::Hotseat => "savh",
        }
    }

    fn from_extension(extension: &str) -> Option<GameType> {
        match extension {
            "sav" => Some(GameType::Standard),
            "savc" => Some(GameType::Campaign),
            "savh" => Some(GameType::Hotseat),
            _ => None,
        }
    }
}

pub fn load(bytes: &[u8]) -> std::result::Result<SaveGame, Error> {
    let profile = profile_for(SaveVersion::V10032);
    let _container = crate::container::decode_container(profile.container_revision, bytes)?;
    Err(Error::NotImplemented("save decode"))
}

pub fn save(save_game: &SaveGame) -> std::result::Result<Vec<u8>, Error> {
    save_as(save_game, SaveVersion::V10032)
}

pub fn save_as(save_game: &SaveGame, target: SaveVersion) -> std::result::Result<Vec<u8>, Error> {
    let _ = (save_game, profile_for(target));
    Err(Error::NotImplemented("save encode"))
}

#[cfg(test)]
mod tests;
