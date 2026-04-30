#![doc = include_str!("../readme.md")]
#![doc(html_no_source)]
#![debugger_visualizer(natvis_file = "../windows-core.natvis")]
#![cfg_attr(all(not(feature = "std")), no_std)]
#![expect(
    non_snake_case,
    non_camel_case_types,
    dead_code,
    clippy::missing_transmute_annotations,
    clippy::upper_case_acronyms
)]

#[cfg(windows)]
include!("windows.rs");

extern crate self as windows_core;

extern crate alloc;

use alloc::boxed::Box;

#[doc(hidden)]
pub mod imp;

mod agile_reference;
mod as_impl;
mod com_object;
#[cfg(feature = "std")]
mod event;
mod guid;
mod in_ref;
mod inspectable;
mod interface;
mod out_param;
mod out_ref;
mod param;
mod param_value;
mod runtime_name;
mod runtime_type;
mod scoped_interface;
mod r#type;
mod unknown;
mod weak;

pub use agile_reference::*;
pub use as_impl::*;
pub use com_object::*;
#[cfg(feature = "std")]
pub use event::*;
pub use guid::*;
pub use in_ref::*;
pub use inspectable::*;
pub use interface::*;
pub use out_param::*;
pub use out_ref::*;
pub use param::*;
pub use param_value::*;
pub use r#type::*;
pub use runtime_name::*;
pub use runtime_type::*;
pub use scoped_interface::*;
pub use unknown::*;
pub use weak::*;
pub use windows_implement::implement;
pub use windows_interface::interface;
pub use windows_link::link;
pub use windows_result::*;
pub use windows_strings::*;

/// Attempts to load the factory object for the given WinRT class.
/// This can be used to access COM interfaces implemented on a Windows Runtime class factory.
#[cfg(windows)]
pub fn factory<C: RuntimeName, I: Interface>() -> Result<I> {
    imp::load_factory::<C, I>()
}

impl Param<PCWSTR> for &HSTRING {
    unsafe fn param(self) -> ParamValue<PCWSTR> {
        ParamValue::Owned(PCWSTR(self.as_ptr()))
    }
}

impl Param<PCWSTR> for PWSTR {
    unsafe fn param(self) -> ParamValue<PCWSTR> {
        ParamValue::Owned(PCWSTR(self.0))
    }
}

impl Param<PCSTR> for PSTR {
    unsafe fn param(self) -> ParamValue<PCSTR> {
        ParamValue::Owned(PCSTR(self.0))
    }
}

impl RuntimeType for HSTRING {
    const SIGNATURE: imp::ConstBuffer = imp::ConstBuffer::from_slice(b"string");
}

impl TypeKind for PWSTR {
    type TypeKind = CopyType;
}

impl TypeKind for PSTR {
    type TypeKind = CopyType;
}

impl TypeKind for PCWSTR {
    type TypeKind = CopyType;
}

impl TypeKind for PCSTR {
    type TypeKind = CopyType;
}

impl TypeKind for HSTRING {
    type TypeKind = CloneType;
}
