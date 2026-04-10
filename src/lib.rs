//! Read and write fheroes2 save files.

pub mod codec;
pub(crate) mod internal;
pub mod model;
pub mod version;

pub use codec::{load, save, save_as};
pub use internal::error::{Error, ParseError, ParseErrorKind, ParseSection};
pub use internal::save_string::SaveString;
pub use model::SaveGame;
pub use version::{SaveVersion, VersionProfile};
