use std::fmt::{Display, Formatter};
use std::str::Utf8Error;

/// Length-prefixed raw string bytes from the save file.
///
/// fheroes2 strings are byte containers; UTF-8 is not guaranteed.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SaveString(Vec<u8>);

impl SaveString {
    /// Build from raw bytes.
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    /// Build from UTF-8 text bytes.
    pub fn from_utf8(s: &str) -> Self {
        Self::from_bytes(s.as_bytes().to_vec())
    }

    /// Return the raw bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Consume and return the raw bytes.
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }

    /// Interpret the bytes as UTF-8.
    pub fn as_utf8(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(&self.0)
    }

    /// Decode as UTF-8, replacing invalid bytes.
    pub fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(&self.0).into_owned()
    }

    /// Return whether there are no bytes.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Return a debug-formatted, single-line excerpt truncated to `max_chars` characters.
    pub(crate) fn brief(&self, max_chars: usize) -> String {
        let single_line = self.to_string_lossy().replace(['\r', '\n'], " ");
        let total_chars = single_line.chars().count();
        let mut shortened: String = single_line.chars().take(max_chars).collect();
        if total_chars > max_chars {
            shortened.push_str("...");
        }
        format!("{shortened:?}")
    }
}

impl Display for SaveString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string_lossy())
    }
}

impl From<Vec<u8>> for SaveString {
    fn from(value: Vec<u8>) -> Self {
        Self::from_bytes(value)
    }
}

impl From<String> for SaveString {
    fn from(value: String) -> Self {
        Self::from_bytes(value.into_bytes())
    }
}

impl From<&str> for SaveString {
    fn from(value: &str) -> Self {
        Self::from_utf8(value)
    }
}

impl PartialEq<&str> for SaveString {
    fn eq(&self, other: &&str) -> bool {
        self.0 == other.as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::SaveString;

    #[test]
    fn as_utf8_rejects_invalid_utf8() {
        let value = SaveString::from_bytes(vec![0xFF, 0xFE]);

        assert!(value.as_utf8().is_err());
    }

    #[test]
    fn to_string_lossy_returns_owned_string() {
        let value = SaveString::from_bytes(vec![b'A', 0xFF, b'B']);

        assert_eq!(value.to_string_lossy(), "A\u{FFFD}B");
    }
}
