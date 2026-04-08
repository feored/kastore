use crate::internal::error::{Error, ParseError, ParseErrorKind, ParseSection};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Reader<'a> {
    bytes: &'a [u8],
    offset: usize,
    section: ParseSection,
}

impl<'a> Reader<'a> {
    pub(crate) fn with_context(bytes: &'a [u8], section: ParseSection) -> Self {
        Self {
            bytes,
            offset: 0,
            section,
        }
    }

    pub(crate) fn set_section(&mut self, section: ParseSection) {
        self.section = section;
    }

    pub(crate) fn position(&self) -> usize {
        self.offset
    }

    pub(crate) fn read_u8(&mut self, field_name: &'static str) -> std::result::Result<u8, Error> {
        let bytes = self.read_bytes(1, field_name)?;
        Ok(bytes[0])
    }

    pub(crate) fn read_byte_as_bool(
        &mut self,
        field_name: &'static str,
    ) -> std::result::Result<bool, Error> {
        Ok(self.read_u8(field_name)? != 0)
    }

    pub(crate) fn read_u16_be(
        &mut self,
        field_name: &'static str,
    ) -> std::result::Result<u16, Error> {
        let bytes = self.read_bytes(2, field_name)?;
        Ok(u16::from_be_bytes([bytes[0], bytes[1]]))
    }

    pub(crate) fn read_u32_be(
        &mut self,
        field_name: &'static str,
    ) -> std::result::Result<u32, Error> {
        let bytes = self.read_bytes(4, field_name)?;
        Ok(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    pub(crate) fn read_i32_be(
        &mut self,
        field_name: &'static str,
    ) -> std::result::Result<i32, Error> {
        let bytes = self.read_bytes(4, field_name)?;
        Ok(i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    pub(crate) fn read_bytes(
        &mut self,
        len: usize,
        field_name: &'static str,
    ) -> std::result::Result<&'a [u8], Error> {
        let start = self.offset;
        let remaining = self.bytes.len().saturating_sub(start);
        let end = self
            .offset
            .checked_add(len)
            .ok_or_else(|| self.truncated(field_name, start, len, remaining))?;
        let bytes = self
            .bytes
            .get(self.offset..end)
            .ok_or_else(|| self.truncated(field_name, start, len, remaining))?;
        self.offset = end;
        Ok(bytes)
    }

    pub(crate) fn read_string_bytes(
        &mut self,
        field_name: &'static str,
    ) -> std::result::Result<Vec<u8>, Error> {
        let length_offset = self.offset;
        let len = self.read_u32_be(field_name)?;
        let len = usize::try_from(len).map_err(|_| {
            self.invalid_value(
                field_name,
                length_offset,
                "string length does not fit in usize",
            )
        })?;
        Ok(self.read_bytes(len, field_name)?.to_vec())
    }

    pub(crate) fn unexpected_value(
        &self,
        field_name: &'static str,
        offset: usize,
        expected: &'static str,
        actual: impl Into<String>,
    ) -> Error {
        self.error_at(
            field_name,
            offset,
            ParseErrorKind::UnexpectedValue {
                expected,
                actual: actual.into(),
            },
        )
    }

    pub(crate) fn invalid_value(
        &self,
        field_name: &'static str,
        offset: usize,
        message: &'static str,
    ) -> Error {
        self.error_at(field_name, offset, ParseErrorKind::InvalidValue { message })
    }

    fn truncated(
        &self,
        field_name: &'static str,
        offset: usize,
        needed: usize,
        remaining: usize,
    ) -> Error {
        self.error_at(
            field_name,
            offset,
            ParseErrorKind::Truncated { needed, remaining },
        )
    }

    fn error_at(&self, field_name: &'static str, offset: usize, kind: ParseErrorKind) -> Error {
        Error::Parse(ParseError {
            section: self.section,
            field: field_name,
            offset,
            kind,
        })
    }
}
