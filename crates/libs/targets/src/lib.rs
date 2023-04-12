/*!
Learn more about Rust for Windows here: <https://github.com/microsoft/windows-rs>
*/

#![no_std]

#[cfg(all(windows_raw_dylib, target_arch = "x86"))]
#[macro_export]
#[doc(hidden)]
macro_rules! link {
    ($library:literal $abi:literal $($link_name:literal)? $(#[$($doc:tt)*])* fn $name:ident($($arg:ident: $argty:ty),*)$(->$ret:ty)?) => (
        #[link(name = $library, kind = "raw-dylib", modifiers = "+verbatim", import_name_type = "undecorated")]
        extern $abi {
            $(#[$($doc)*])*
            $(#[link_name=$link_name])?
            pub fn $name($($arg: $argty),*) $(->$ret)?;
        }
    )
}

#[cfg(all(windows_raw_dylib, not(target_arch = "x86")))]
#[macro_export]
#[doc(hidden)]
macro_rules! link {
    ($library:literal $abi:literal $($link_name:literal)? $(#[$($doc:tt)*])* fn $name:ident($($arg:ident: $argty:ty),*)$(->$ret:ty)?) => (
        #[link(name = $library, kind = "raw-dylib", modifiers = "+verbatim")]
        extern "system" {
            $(#[$($doc)*])*
            $(#[link_name=$link_name])?
            pub fn $name($($arg: $argty),*) $(->$ret)?;
        }
    )
}

#[cfg(all(windows, not(windows_raw_dylib)))]
#[macro_export]
#[doc(hidden)]
macro_rules! link {
    ($library:literal $abi:literal $($link_name:literal)? $(#[$($doc:tt)*])* fn $name:ident($($arg:ident: $argty:ty),*)$(->$ret:ty)?) => (
        #[link(name = "windows.0.48.0")]
        extern $abi {
            $(#[$($doc)*])*
            $(#[link_name=$link_name])?
            pub fn $name($($arg: $argty),*) $(->$ret)?;
        }
    )
}

#[cfg(all(not(windows), not(windows_raw_dylib)))]
#[macro_export]
#[doc(hidden)]
macro_rules! link {
    ($library:literal $abi:literal $($link_name:literal)? $(#[$($doc:tt)*])* fn $name:ident($($arg:ident: $argty:ty),*)$(->$ret:ty)?) => (
        extern $abi {
            $(#[$($doc)*])*
            pub fn $name($($arg: $argty),*) $(->$ret)?;
        }
    )
}
