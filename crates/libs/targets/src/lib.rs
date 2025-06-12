#![doc = include_str!("../readme.md")]
#![no_std]

/// Defines an external function to import.
#[cfg(all(windows_raw_dylib, target_arch = "x86"))]
#[macro_export]
macro_rules! link {
    // Avoid future-compat lint if bindings are still using the old "cdecl" form of the bindings.
    ($library:literal "cdecl" $($link_name:literal)? fn $($function:tt)*) => (
        $crate::link!($library "C" $($link_name)? fn $($function)*);
    );
    ($library:literal $abi:literal $($link_name:literal)? fn $($function:tt)*) => (
        #[link(name = $library, kind = "raw-dylib", modifiers = "+verbatim", import_name_type = "undecorated")]
        extern $abi {
            $(#[link_name=$link_name])?
            pub fn $($function)*;
        }
    )
}

/// Defines an external function to import.
#[cfg(all(windows_raw_dylib, not(target_arch = "x86")))]
#[macro_export]
macro_rules! link {
    // Avoid future-compat lint if bindings are still using the old "cdecl" form of the bindings.
    ($library:literal "cdecl" $($link_name:literal)? fn $($function:tt)*) => (
        $crate::link!($library "C" $($link_name)? fn $($function)*);
    );
    ($library:literal $abi:literal $($link_name:literal)? fn $($function:tt)*) => (
        #[link(name = $library, kind = "raw-dylib", modifiers = "+verbatim")]
        extern $abi {
            $(#[link_name=$link_name])?
            pub fn $($function)*;
        }
    )
}

/// Defines an external function to import.
#[cfg(all(windows, not(windows_raw_dylib)))]
#[macro_export]
macro_rules! link {
    // Avoid future-compat lint if bindings are still using the old "cdecl" form of the bindings.
    ($library:literal "cdecl" $($link_name:literal)? fn $($function:tt)*) => (
        $crate::link!($library "C" $($link_name)? fn $($function)*);
    );
    ($library:literal $abi:literal $($link_name:literal)? fn $($function:tt)*) => (
        #[link(name = "windows.0.53.0")]
        extern $abi {
            $(#[link_name=$link_name])?
            pub fn $($function)*;
        }
    )
}

/// Defines an external function to import.
#[cfg(all(not(windows), not(windows_raw_dylib)))]
#[macro_export]
macro_rules! link {
    // Avoid future-compat lint if bindings are still using the old "cdecl" form of the bindings.
    ($library:literal "cdecl" $($link_name:literal)? fn $($function:tt)*) => (
        $crate::link!($library "C" $($link_name)? fn $($function)*);
    );
    ($library:literal $abi:literal $($link_name:literal)? fn $($function:tt)*) => (
        extern $abi {
            pub fn $($function)*;
        }
    )
}
