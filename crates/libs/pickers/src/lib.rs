#![doc = include_str!("../readme.md")]

#[expect(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    clippy::upper_case_acronyms
)]
mod bindings;
mod dialog;
mod filter;
mod folder;
mod location;
mod open;
#[cfg(feature = "reactor")]
mod reactor;
mod save;

pub use folder::FolderPicker;
pub use location::PickerLocation;
pub use open::OpenFilePicker;
pub use save::SaveFilePicker;
pub use windows_core::{GUID, Result};
