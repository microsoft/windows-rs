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

use bindings::*;
use dialog::*;
use filter::*;
use std::ffi::OsString;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};
use windows_core::*;
#[cfg(feature = "reactor")]
use windows_reactor::{Component, ComponentContext};

pub use folder::FolderPicker;
pub use location::PickerLocation;
pub use open::OpenFilePicker;
pub use save::SaveFilePicker;
pub use windows_core::{GUID, Result};

#[cfg(test)]
mod tests {
    use super::*;
    use windows_core::HRESULT;

    const E_INVALIDARG: HRESULT = HRESULT(0x8007_0057_u32 as i32);

    fn assert_invalid_owner<T>(result: Result<T>) {
        assert_eq!(result.err().unwrap().code(), E_INVALIDARG);
    }

    #[test]
    fn raw_entry_points_reject_null_owner_before_opening_a_dialog() {
        let owner = core::ptr::null_mut();

        assert_invalid_owner(OpenFilePicker::new().show_for_hwnd(owner));
        assert_invalid_owner(OpenFilePicker::new().show_multiple_for_hwnd(owner));
        assert_invalid_owner(FolderPicker::new().show_for_hwnd(owner));
        assert_invalid_owner(FolderPicker::new().show_multiple_for_hwnd(owner));
        assert_invalid_owner(SaveFilePicker::new().show_for_hwnd(owner));
    }
}
