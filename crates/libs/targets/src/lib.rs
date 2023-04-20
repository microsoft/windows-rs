/*!
Learn more about Rust for Windows here: <https://github.com/microsoft/windows-rs>
*/

#![no_std]

#[cfg(all(windows_raw_dylib, target_arch = "x86"))]
#[macro_export]
#[doc(hidden)]
macro_rules! link {
    ($library:literal $abi:literal $($link_name:literal)? $(#[$doc:meta])? $vis:vis fn $($function:tt)*) => (
        #[link(name = $library, kind = "raw-dylib", modifiers = "+verbatim", import_name_type = "undecorated")]
        extern $abi {
            $(#[$doc])?
            $(#[link_name=$link_name])?
            $vis fn $($function)*;
        }
    )
}

#[cfg(all(windows_raw_dylib, not(target_arch = "x86")))]
#[macro_export]
#[doc(hidden)]
macro_rules! link {
    ($library:literal $abi:literal $($link_name:literal)? $(#[$doc:meta])? $vis:vis fn $($function:tt)*) => (
        #[link(name = $library, kind = "raw-dylib", modifiers = "+verbatim")]
        extern "C" {
            $(#[$doc])?
            $(#[link_name=$link_name])?
            $vis fn $($function)*;
        }
    )
}

#[cfg(all(windows, not(windows_raw_dylib)))]
#[macro_export]
#[doc(hidden)]
macro_rules! link {
    ($library:literal $abi:literal $($link_name:literal)? $(#[$doc:meta])? $vis:vis fn $($function:tt)*) => (
        #[link(name = "windows.0.48.0")]
        extern $abi {
            $(#[$doc])?
            $(#[link_name=$link_name])?
            $vis fn $($function)*;
        }
    )
}

#[cfg(all(not(windows), not(windows_raw_dylib)))]
#[macro_export]
#[doc(hidden)]
macro_rules! link {
    ($library:literal $abi:literal $($link_name:literal)? $(#[$doc:meta])? $vis:vis fn $($function:tt)*) => (
        extern $abi {
            $(#[$doc])?
            $vis fn $($function)*;
        }
    )
}
