#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SaveVersion {
    #[default]
    V10032,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContainerRevision {
    R10032,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorldRevision {
    V1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SettingsRevision {
    V1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameOverRevision {
    V1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CampaignRevision {
    V1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VersionProfile {
    pub save_version: SaveVersion,
    pub container_revision: ContainerRevision,
    pub world_revision: WorldRevision,
    pub settings_revision: SettingsRevision,
    pub game_over_revision: GameOverRevision,
    pub campaign_revision: Option<CampaignRevision>,
}

pub const LATEST_PROFILE: VersionProfile = VersionProfile {
    save_version: SaveVersion::V10032,
    container_revision: ContainerRevision::R10032,
    world_revision: WorldRevision::V1,
    settings_revision: SettingsRevision::V1,
    game_over_revision: GameOverRevision::V1,
    campaign_revision: Some(CampaignRevision::V1),
};

pub const fn profile_for(save_version: SaveVersion) -> VersionProfile {
    match save_version {
        SaveVersion::V10032 => LATEST_PROFILE,
    }
}
