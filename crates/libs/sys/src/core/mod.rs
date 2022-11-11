mod literals;

#[doc(hidden)]
pub use literals::*;

pub type HRESULT = i32;
pub type HSTRING = *mut ::core::ffi::c_void;
pub type IUnknown = *mut ::core::ffi::c_void;
pub type IInspectable = *mut ::core::ffi::c_void;
pub type PSTR = *mut u8;
pub type PWSTR = *mut u16;
pub type PCSTR = *const u8;
pub type PCWSTR = *const u16;
pub type BSTR = *const u16;

#[repr(C)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl ::core::marker::Copy for GUID {}

impl ::core::clone::Clone for GUID {
    fn clone(&self) -> Self {
        *self
    }
}

impl GUID {
    pub const fn from_u128(uuid: u128) -> Self {
        Self { data1: (uuid >> 96) as u32, data2: (uuid >> 80 & 0xffff) as u16, data3: (uuid >> 64 & 0xffff) as u16, data4: (uuid as u64).to_be_bytes() }
    }
}

#[doc(hidden)]
pub unsafe fn delay_load<T>(library: PCSTR, function: PCSTR) -> Option<T> {
    let library = LoadLibraryA(library);

    if library == 0 {
        return None;
    }

    let address = GetProcAddress(library, function);

    if !address.is_null() {
        return Some(core::mem::transmute_copy(&address));
    }

    FreeLibrary(library);
    None
}

#[cfg(all(windows_raw_dylib, target_arch = "x86"))]
#[link(name = "kernel32.dll", kind = "raw-dylib", modifiers = "+verbatim", import_name_type = "undecorated")]
extern "system" {
    fn GetProcAddress(library: isize, name: PCSTR) -> *const core::ffi::c_void;
    fn LoadLibraryA(name: PCSTR) -> isize;
    fn FreeLibrary(library: isize) -> i32;
}

#[cfg(all(windows_raw_dylib, not(target_arch = "x86")))]
#[link(name = "kernel32.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    fn GetProcAddress(library: isize, name: PCSTR) -> *const core::ffi::c_void;
    fn LoadLibraryA(name: PCSTR) -> isize;
    fn FreeLibrary(library: isize) -> i32;
}

#[cfg(not(windows_raw_dylib))]
#[link(name = "windows")]
extern "system" {
    fn GetProcAddress(library: isize, name: PCSTR) -> *const core::ffi::c_void;
    fn LoadLibraryA(name: PCSTR) -> isize;
    fn FreeLibrary(library: isize) -> i32;
}


#[cfg(all(windows_raw_dylib,  not(windows_delay_load), target_arch = "x86"))]
#[macro_export]
macro_rules! link {
    ($library:literal, $abi:literal fn $name:ident($($arg:ident: $argty:ty),*)->$ret:ty) => (
        #[link(name = $library, kind = "raw-dylib", modifiers = "+verbatim", import_name_type = "undecorated")]
        extern $abi {
            pub fn $name($($arg: $argty),*) -> $ret;
        }
    )
}

#[cfg(all(windows_raw_dylib,  not(windows_delay_load), not(target_arch = "x86")))]
#[macro_export]
macro_rules! link {
    ($library:literal, $abi:literal fn $name:ident($($arg:ident: $argty:ty),*)->$ret:ty) => (
        #[link(name = $library, kind = "raw-dylib", modifiers = "+verbatim")]
        extern "system" {
            pub fn $name($($arg: $argty),*) -> $ret;
        }
    )
}

#[cfg(all(not(windows_raw_dylib), not(windows_delay_load), target_arch = "x86"))]
#[macro_export]
macro_rules! link {
    ($library:literal, $abi:literal fn $name:ident($($arg:ident: $argty:ty),*)->$ret:ty) => (
        #[link(name = "windows")]
        extern $abi {
            pub fn $name($($arg: $argty),*) -> $ret;
        }
    )
}

#[cfg(all(not(windows_raw_dylib), not(windows_delay_load), not(target_arch = "x86")))]
#[macro_export]
macro_rules! link {
    ($library:literal, $abi:literal fn $name:ident($($arg:ident: $argty:ty),*)->$ret:ty) => (
        #[link(name = "windows")]
        extern "system" {
            pub fn $name($($arg: $argty),*) -> $ret;
        }

    )
}

#[cfg(windows_delay_load)]
#[macro_export]
macro_rules! link {
    ($library:literal, $abi:literal fn $name:ident($($arg:ident: $argty:ty),*)->$ret:ty) => (
        #[cfg(target_arch = "x86")]
        pub type $name = ::core::option::Option<unsafe extern $abi fn($($argty),*) -> $ret>;
        #[cfg(not(target_arch = "x86"))]
        pub type $name = ::core::option::Option<unsafe extern "system" fn($($argty),*) -> $ret>;
        pub unsafe fn $name($($arg: $argty),*) -> $ret {
            static mut CACHE: $name = None;
            unsafe {
                if CACHE.is_none() {
                    let name = ::core::concat!(::core::stringify!($name), "\0");
                    CACHE = $crate::core::delay_load($crate::core::s!($library), name.as_ptr());
                }
                CACHE.unwrap()($($arg),*)
            }
        }
    )
}

#[doc(hidden)]
pub use crate::link;
