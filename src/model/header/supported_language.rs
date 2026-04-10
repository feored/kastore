use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SupportedLanguage(u8);

impl SupportedLanguage {
    pub const ENGLISH: Self = Self(0);
    pub const FRENCH: Self = Self(1);
    pub const POLISH: Self = Self(2);
    pub const GERMAN: Self = Self(3);
    pub const RUSSIAN: Self = Self(4);
    pub const ITALIAN: Self = Self(5);
    pub const CZECH: Self = Self(6);
    pub const SPANISH: Self = Self(7);
    pub const BELARUSIAN: Self = Self(8);
    pub const BULGARIAN: Self = Self(9);
    pub const DANISH: Self = Self(10);
    pub const DUTCH: Self = Self(11);
    pub const GREEK: Self = Self(12);
    pub const HUNGARIAN: Self = Self(13);
    pub const NORWEGIAN: Self = Self(14);
    pub const PORTUGUESE: Self = Self(15);
    pub const ROMANIAN: Self = Self(16);
    pub const SLOVAK: Self = Self(17);
    pub const SWEDISH: Self = Self(18);
    pub const TURKISH: Self = Self(19);
    pub const UKRAINIAN: Self = Self(20);
    pub const VIETNAMESE: Self = Self(21);

    pub const fn from_u8(value: u8) -> Self {
        Self(value)
    }

    pub const fn as_u8(self) -> u8 {
        self.0
    }

    pub const fn name(self) -> Option<&'static str> {
        match self.0 {
            0 => Some("English"),
            1 => Some("French"),
            2 => Some("Polish"),
            3 => Some("German"),
            4 => Some("Russian"),
            5 => Some("Italian"),
            6 => Some("Czech"),
            7 => Some("Spanish"),
            8 => Some("Belarusian"),
            9 => Some("Bulgarian"),
            10 => Some("Danish"),
            11 => Some("Dutch"),
            12 => Some("Greek"),
            13 => Some("Hungarian"),
            14 => Some("Norwegian"),
            15 => Some("Portuguese"),
            16 => Some("Romanian"),
            17 => Some("Slovak"),
            18 => Some("Swedish"),
            19 => Some("Turkish"),
            20 => Some("Ukrainian"),
            21 => Some("Vietnamese"),
            _ => None,
        }
    }
}

impl From<u8> for SupportedLanguage {
    fn from(value: u8) -> Self {
        Self::from_u8(value)
    }
}

impl From<SupportedLanguage> for u8 {
    fn from(value: SupportedLanguage) -> Self {
        value.as_u8()
    }
}

impl Display for SupportedLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.name() {
            Some(name) => f.write_str(name),
            None => write!(f, "Unknown language {}", self.0),
        }
    }
}
