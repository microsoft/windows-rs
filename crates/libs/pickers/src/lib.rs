#![doc = include_str!("../readme.md")]

#[expect(non_snake_case, non_upper_case_globals, clippy::upper_case_acronyms)]
mod bindings;
mod dialog;
mod filter;
mod folder;
mod location;
mod open;
#[cfg(feature = "reactor")]
mod reactor;
mod save;

use bindings::*;
use dialog::*;
use filter::*;
use std::ffi::OsString;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf, Prefix};
use windows_core::*;
#[cfg(feature = "reactor")]
use windows_reactor::{Component, ComponentContext};

pub use folder::FolderPicker;
pub use location::PickerLocation;
pub use open::OpenFilePicker;
pub use save::SaveFilePicker;
pub use windows_core::{GUID, Result};
