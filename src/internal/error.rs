use std::fmt::{Display, Formatter};

/// Error returned by save decoding and encoding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The numeric save format version is not supported.
    UnsupportedSaveVersion { version: u16 },
    /// A parse error with section, field, and offset.
    Parse(ParseError),
    /// A typed model value cannot be encoded.
    InvalidModel {
        field: &'static str,
        message: &'static str,
    },
    /// Requested behavior is known but not implemented.
    NotImplemented { feature: &'static str },
}

/// Top-level section where parsing failed.
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

/// Parse failure with the field and byte offset that failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    /// Save section being parsed.
    pub section: ParseSection,
    /// Field being parsed.
    pub field: &'static str,
    /// Byte offset within the current reader.
    pub offset: usize,
    /// Error details.
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

/// Specific parse failure kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErrorKind {
    /// Input ended before enough bytes were available.
    Truncated { needed: usize, remaining: usize },
    /// A required sentinel or marker had another value.
    UnexpectedValue {
        expected: &'static str,
        actual: String,
    },
    /// A value was present but invalid for this field.
    InvalidValue { message: &'static str },
    /// A valid value refers to unsupported behavior.
    Unsupported { message: &'static str },
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
