use std::mem;

use bindings::Windows::Win32::{Com, SystemServices::PWSTR};

/// Copy a [`PWSTR`] from an input param to a [`String`].
pub fn string_from_pwstr(source: PWSTR) -> String {
    if source.is_null() {
        String::new()
    } else {
        let mut buffer = Vec::new();
        let mut pwz = source.0;

        unsafe {
            while *pwz != 0 {
                buffer.push(*pwz);
                pwz = pwz.add(1);
            }
        }

        String::from_utf16(&buffer).expect("string_from_pwstr")
    }
}

/// Copy a [`PWSTR`] allocated with [`Com::CoTaskMemAlloc`] from an input param to a [`String`]
/// and free the original buffer with [`Com::CoTaskMemFree`].
pub fn take_pwstr(source: PWSTR) -> String {
    let result = string_from_pwstr(source);

    if !source.is_null() {
        unsafe {
            Com::CoTaskMemFree(mem::transmute(source.0));
        }
    }

    result
}
