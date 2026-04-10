use std::fmt::Display;

/// fheroes2 save format version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SaveVersion(u16);

impl SaveVersion {
    pub const FORMAT_VERSION_1150_RELEASE: Self = Self(10033);
    pub const FORMAT_VERSION_1111_RELEASE: Self = Self(10032);
    pub const FORMAT_VERSION_1109_RELEASE: Self = Self(10031);
    pub const FORMAT_VERSION_1108_RELEASE: Self = Self(10030);
    pub const FORMAT_VERSION_PRE1_1108_RELEASE: Self = Self(10029);
    pub const FORMAT_VERSION_1107_RELEASE: Self = Self(10028);
    pub const FORMAT_VERSION_1106_RELEASE: Self = Self(10027);
    pub const FORMAT_VERSION_PPRE1_1106_RELEASE: Self = Self(10026);
    pub const FORMAT_VERSION_1104_RELEASE: Self = Self(10025);
    pub const FORMAT_VERSION_1103_RELEASE: Self = Self(10024);
    pub const FORMAT_VERSION_PRE2_1103_RELEASE: Self = Self(10023);
    pub const FORMAT_VERSION_PRE1_1103_RELEASE: Self = Self(10022);
    pub const FORMAT_VERSION_1101_RELEASE: Self = Self(10021);
    pub const FORMAT_VERSION_PRE1_1101_RELEASE: Self = Self(10020);
    pub const FORMAT_VERSION_1100_RELEASE: Self = Self(10019);
    pub const FORMAT_VERSION_PRE3_1100_RELEASE: Self = Self(10018);
    pub const FORMAT_VERSION_PRE2_1100_RELEASE: Self = Self(10017);
    pub const FORMAT_VERSION_PRE1_1100_RELEASE: Self = Self(10016);
    pub const FORMAT_VERSION_1010_RELEASE: Self = Self(10015);
    pub const FORMAT_VERSION_1009_RELEASE: Self = Self(10014);
    pub const FORMAT_VERSION_PRE2_1009_RELEASE: Self = Self(10013);
    pub const FORMAT_VERSION_PRE1_1009_RELEASE: Self = Self(10012);
    pub const FORMAT_VERSION_1007_RELEASE: Self = Self(10011);
    pub const FORMAT_VERSION_1005_RELEASE: Self = Self(10010);

    pub const LAST_SUPPORTED_FORMAT_VERSION: Self = Self::FORMAT_VERSION_1005_RELEASE;
    pub const CURRENT_FORMAT_VERSION: Self = Self::FORMAT_VERSION_1150_RELEASE;

    /// Build from the raw save version number.
    pub const fn from_u16(value: u16) -> Self {
        Self(value)
    }

    /// Return the raw save version number.
    pub const fn as_u16(self) -> u16 {
        self.0
    }
}

impl Default for SaveVersion {
    fn default() -> Self {
        Self::FORMAT_VERSION_1111_RELEASE
    }
}

impl From<u16> for SaveVersion {
    fn from(value: u16) -> Self {
        Self::from_u16(value)
    }
}

impl From<SaveVersion> for u16 {
    fn from(value: SaveVersion) -> Self {
        value.as_u16()
    }
}

impl Display for SaveVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_u16())
    }
}

/// Version-specific map-info layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MapInfoRevision {
    V10024,
    V10033,
}

/// Decoding profile for a supported save format version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VersionProfile {
    /// Save format version.
    pub save_version: SaveVersion,
    /// `Maps::FileInfo` layout revision.
    pub map_info_revision: MapInfoRevision,
}

/// Decoding profile for save format `10032`.
pub const PROFILE_10032: VersionProfile = VersionProfile {
    save_version: SaveVersion::FORMAT_VERSION_1111_RELEASE,
    map_info_revision: MapInfoRevision::V10024,
};

/// Decoding profile for the latest supported save format.
pub const LATEST_PROFILE: VersionProfile = VersionProfile {
    save_version: SaveVersion::FORMAT_VERSION_1150_RELEASE,
    map_info_revision: MapInfoRevision::V10033,
};

/// Return the decoding profile for a supported save format version.
pub const fn profile_for(save_version: SaveVersion) -> Option<VersionProfile> {
    if save_version.as_u16() == SaveVersion::FORMAT_VERSION_1111_RELEASE.as_u16() {
        Some(PROFILE_10032)
    } else if save_version.as_u16() == SaveVersion::FORMAT_VERSION_1150_RELEASE.as_u16() {
        Some(LATEST_PROFILE)
    } else {
        None
    }
}
