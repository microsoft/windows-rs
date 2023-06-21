#![cfg_attr(docsrs, doc = "This is a stub. The latest API documentation is here: <https://microsoft.github.io/windows-docs-rs/>")]
#![cfg_attr(docsrs, doc = "")]
/*!
Learn more about Rust for Windows here: <https://github.com/microsoft/windows-rs>
*/

#![doc(html_no_source)]
#![allow(non_snake_case, clashing_extern_declarations, non_upper_case_globals, non_camel_case_types, clippy::all)]

extern crate self as windows;
pub use Windows::*;
mod Windows;

pub mod core {
    pub use windows_core::*;

    #[doc(hidden)]
    #[cfg(feature = "implement")]
    pub use windows_implement::implement;

    #[doc(hidden)]
    #[cfg(feature = "implement")]
    pub use windows_interface::interface;
}
