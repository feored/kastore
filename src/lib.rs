//! Read and write fheroes2 save files.

pub mod codec;
pub(crate) mod internal;
pub mod model;
pub mod version;

pub use codec::{
	Diagnostic, DiagnosticKind, LoadOptions, ParseMode, ParseReport, Severity, load,
	load_with_options, save, save_as,
};
pub use internal::error::{Error, ParseError, ParseErrorKind, ParseSection};
pub use internal::save_string::SaveString;
pub use version::{SaveVersion, VersionProfile};

pub use model::header::SaveHeader;
pub use model::header::map_info::MapInfo;
pub use model::save_game::SaveGame;
pub use model::settings::Settings;
pub use model::world::World;
pub use model::world::castles::Castle;
pub use model::world::heroes::Hero;
pub use model::world::kingdoms::Kingdom;
