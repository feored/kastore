use crate::internal::save_string::SaveString;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Writer {
    bytes: Vec<u8>,
    offset: usize,
}

impl Writer {
    pub(crate) fn new() -> Self {
        Self {
            bytes: Vec::new(),
            offset: 0,
        }
    }

    pub(crate) fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    pub(crate) fn write_u8(&mut self, value: u8) {
        self.bytes.push(value);
        self.offset += 1;
    }

    pub(crate) fn write_u16_be(&mut self, value: u16) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
        self.offset += 2;
    }

    pub(crate) fn write_u32_be(&mut self, value: u32) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
        self.offset += 4;
    }

    pub(crate) fn write_i32_be(&mut self, value: i32) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
        self.offset += 4;
    }

    pub(crate) fn write_bytes(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
        self.offset += bytes.len();
    }

    pub(crate) fn write_save_string(&mut self, value: &SaveString) {
        let as_bytes = value.as_bytes();
        self.write_u32_be(as_bytes.len() as u32);
        self.write_bytes(as_bytes);
    }

    pub(crate) fn write_byte_from_bool(&mut self, value: bool) {
        self.write_u8(if value { 1 } else { 0 });
    }
}
