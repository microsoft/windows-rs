#![doc = include_str!("../readme.md")]
#![no_std]

/// Defines an external function to import.
///
/// Expands to an `extern` block declaring the function as well as a `pub type`
/// alias matching the function's signature. The type alias has the same name
/// as the function, lives in the type namespace, and is useful for storing or
/// passing around the function pointer (for example, after resolving the
/// symbol at runtime via `GetProcAddress`).
#[cfg(all(windows, target_arch = "x86"))]
#[macro_export]
macro_rules! link {
    ($library:literal $abi:literal $($link_name:literal)? fn $name:ident($($params:tt)*) $(-> $ret:ty)?) => (
        #[link(name = $library, kind = "raw-dylib", modifiers = "+verbatim", import_name_type = "undecorated")]
        extern $abi {
            $(#[link_name=$link_name])?
            pub fn $name($($params)*) $(-> $ret)?;
        }
        #[allow(non_camel_case_types)]
        pub type $name = unsafe extern $abi fn($($params)*) $(-> $ret)?;
    )
}

/// Defines an external function to import.
///
/// Expands to an `extern` block declaring the function as well as a `pub type`
/// alias matching the function's signature. The type alias has the same name
/// as the function, lives in the type namespace, and is useful for storing or
/// passing around the function pointer (for example, after resolving the
/// symbol at runtime via `GetProcAddress`).
#[cfg(all(windows, not(target_arch = "x86")))]
#[macro_export]
macro_rules! link {
    ($library:literal $abi:literal $($link_name:literal)? fn $name:ident($($params:tt)*) $(-> $ret:ty)?) => (
        #[link(name = $library, kind = "raw-dylib", modifiers = "+verbatim")]
        extern $abi {
            $(#[link_name=$link_name])?
            pub fn $name($($params)*) $(-> $ret)?;
        }
        #[allow(non_camel_case_types)]
        pub type $name = unsafe extern $abi fn($($params)*) $(-> $ret)?;
    )
}

/// Defines an external function to import.
///
/// Expands to an `extern` block declaring the function as well as a `pub type`
/// alias matching the function's signature. The type alias has the same name
/// as the function, lives in the type namespace, and is useful for storing or
/// passing around the function pointer (for example, after resolving the
/// symbol at runtime via `GetProcAddress`).
#[cfg(not(windows))]
#[macro_export]
macro_rules! link {
    ($library:literal $abi:literal $($link_name:literal)? fn $name:ident($($params:tt)*) $(-> $ret:ty)?) => (
        extern $abi {
            pub fn $name($($params)*) $(-> $ret)?;
        }
        #[allow(non_camel_case_types)]
        pub type $name = unsafe extern $abi fn($($params)*) $(-> $ret)?;
    )
}
