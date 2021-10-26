/*!
The Rust language projection follows in the tradition established by [C++/WinRT](https://github.com/microsoft/cppwinrt)
of building language projections for Windows using standard languages and compilers, providing a natural and idiomatic
way for Rust developers to call Windows APIs. The [`windows`] crate lets you call any Windows API past, present, and
future using code generated on the fly directly from the metadata describing the API and right into your Rust package
where you can call them as if they were just another Rust module.

Learn more here: <https://github.com/microsoft/windows-rs>
*/

#![doc(html_no_source)]

extern crate self as windows;
mod Windows;
pub mod runtime;
pub use Windows::*;
