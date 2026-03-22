mod bindings;
use windows_core::*;

use crate::bindings::Windows::Win32::System::LibraryLoader::{
    LoadLibraryExA, LOAD_LIBRARY_AS_IMAGE_RESOURCE,
};

#[test]
fn load_winmd() {
    unsafe {
        LoadLibraryExA(
            s!("../../../crates/libs/bindgen/default/Windows.winmd"),
            None,
            LOAD_LIBRARY_AS_IMAGE_RESOURCE,
        )
        .expect("failed to load winmd");
    }
}
