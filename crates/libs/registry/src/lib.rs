/*!
Learn more about Rust for Windows here: <https://github.com/microsoft/windows-rs>
*/

#![cfg(windows)]
#![no_std]

#[macro_use]
extern crate alloc;

use alloc::{string::String, vec::Vec};

mod bindings;
use bindings::*;

mod key;
pub use key::Key;

mod value;
pub use value::Value;

mod key_iterator;
pub use key_iterator::KeyIterator;

mod value_iterator;
pub use value_iterator::ValueIterator;

mod r#type;
pub use r#type::Type;

pub use windows_result::Result;
use windows_result::*;

pub use windows_strings::HSTRING;
use windows_strings::*;

/// The predefined `HKEY_CLASSES_ROOT` registry key.
pub const CLASSES_ROOT: &Key = &Key(HKEY_CLASSES_ROOT);

/// The predefined `HKEY_CURRENT_CONFIG` registry key.
pub const CURRENT_CONFIG: &Key = &Key(HKEY_CURRENT_CONFIG);

/// The predefined `HKEY_CURRENT_USER` registry key.
pub const CURRENT_USER: &Key = &Key(HKEY_CURRENT_USER);

/// The predefined `HKEY_LOCAL_MACHINE` registry key.
pub const LOCAL_MACHINE: &Key = &Key(HKEY_LOCAL_MACHINE);

/// The predefined `HKEY_USERS` registry key.
pub const USERS: &Key = &Key(HKEY_USERS);

fn pcwstr<T: AsRef<str>>(value: T) -> Vec<u16> {
    value
        .as_ref()
        .encode_utf16()
        .chain(core::iter::once(0))
        .collect()
}

fn trim(mut value: &[u16]) -> &[u16] {
    while value.last() == Some(&0) {
        value = &value[..value.len() - 1];
    }
    value
}

fn win32_error(result: u32) -> Result<()> {
    if result == 0 {
        Ok(())
    } else {
        Err(Error::from_hresult(HRESULT::from_win32(result)))
    }
}

fn invalid_data() -> Error {
    Error::from_hresult(HRESULT::from_win32(ERROR_INVALID_DATA))
}
