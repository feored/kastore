use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    UnsupportedSaveVersion {
        version: u16,
    },
    Parse(ParseError),
    InvalidModel {
        field: &'static str,
        message: &'static str,
    },
    NotImplemented {
        feature: &'static str,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseSection {
    Container,
    Header,
    MapInfo,
    Body,
    World,
    Settings,
    GameOver,
    Campaign,
}

impl Display for ParseSection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseSection::Container => f.write_str("container"),
            ParseSection::Header => f.write_str("header"),
            ParseSection::MapInfo => f.write_str("map_info"),
            ParseSection::Body => f.write_str("body"),
            ParseSection::World => f.write_str("world"),
            ParseSection::Settings => f.write_str("settings"),
            ParseSection::GameOver => f.write_str("game_over"),
            ParseSection::Campaign => f.write_str("campaign"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub section: ParseSection,
    pub field: &'static str,
    pub offset: usize,
    pub kind: ParseErrorKind,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "parse error in {} field \"{}\" at offset {}",
            self.section, self.field, self.offset
        )?;

        write!(f, ": {}", self.kind)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErrorKind {
    Truncated {
        needed: usize,
        remaining: usize,
    },
    UnexpectedValue {
        expected: &'static str,
        actual: String,
    },
    InvalidValue {
        message: &'static str,
    },
    Unsupported {
        message: &'static str,
    },
}

impl Display for ParseErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErrorKind::Truncated { needed, remaining } => {
                write!(
                    f,
                    "truncated input: needed {needed} bytes, {remaining} remaining"
                )
            }
            ParseErrorKind::UnexpectedValue { expected, actual } => {
                write!(f, "unexpected value: expected {expected}, got {actual}")
            }
            ParseErrorKind::InvalidValue { message } => {
                write!(f, "invalid value: {message}")
            }
            ParseErrorKind::Unsupported { message } => write!(f, "unsupported: {message}"),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnsupportedSaveVersion { version } => {
                write!(f, "unsupported save version: {version}")
            }
            Error::Parse(error) => error.fmt(f),
            Error::InvalidModel { field, message } => {
                write!(f, "invalid model field \"{field}\": {message}")
            }
            Error::NotImplemented { feature } => write!(f, "not implemented: {feature}"),
        }
    }
}

impl std::error::Error for Error {}
