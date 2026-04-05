use crate::internal::error::Error;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Reader<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Reader<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    pub(crate) fn position(&self) -> usize {
        self.offset
    }

    pub(crate) fn remaining(&self) -> &'a [u8] {
        &self.bytes[self.offset..]
    }

    pub(crate) fn read_u8(&mut self, field_name: &'static str) -> std::result::Result<u8, Error> {
        let bytes = self.read_bytes(1, field_name)?;
        Ok(bytes[0])
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

    pub(crate) fn read_bytes(
        &mut self,
        len: usize,
        field_name: &'static str,
    ) -> std::result::Result<&'a [u8], Error> {
        let end = self
            .offset
            .checked_add(len)
            .ok_or(Error::InvalidContainer(field_name))?;
        let bytes = self
            .bytes
            .get(self.offset..end)
            .ok_or(Error::InvalidContainer(field_name))?;
        self.offset = end;
        Ok(bytes)
    }

    pub(crate) fn read_string(
        &mut self,
        field_name: &'static str,
    ) -> std::result::Result<String, Error> {
        let len = self.read_u32_be(field_name)?;
        let len = usize::try_from(len).map_err(|_| Error::InvalidContainer(field_name))?;
        let bytes = self.read_bytes(len, field_name)?;
        String::from_utf8(bytes.to_vec()).map_err(|_| Error::InvalidContainer(field_name))
    }
}
