mod r10032;

use crate::SaveVersion;
use crate::Error;
use crate::version::ContainerRevision;

pub(crate) fn decode_container(
    revision: ContainerRevision,
    bytes: &[u8],
) -> std::result::Result<SaveContainer, Error> {
    match revision {
        ContainerRevision::R10032 => r10032::decode(bytes),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SaveContainer {
    pub save_version: SaveVersion,
    pub payload: Vec<u8>,
}

#[cfg(test)]
mod tests;
