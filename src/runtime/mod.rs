#[macro_use] mod macros;

mod array;
mod delay_load;
mod factory_cache;
mod guid;
mod heap;
mod hstring;
mod param;
mod ref_count;
mod sha1;
mod waiter;
mod weak;
mod weak_ref_count;
mod error;
mod hresult;
mod activation_factory;
mod inspectable;
mod unknown;
pub(crate) mod bindings;
mod abi;
mod compose;
mod handle;
mod interface;
mod into_param;
mod runtime_name;
mod runtime_type;
mod to_impl;

pub use array::*;
pub use delay_load::*;
pub use factory_cache::*;
pub use guid::*;
pub use heap::*;
pub use hstring::*;
pub use param::*;
pub use ref_count::*;
pub use sha1::*;
pub use waiter::*;
pub use weak::*;
pub use weak_ref_count::*;
pub use error::*;
pub use hresult::*;
pub use activation_factory::*;
pub use inspectable::*;
pub use unknown::*;
pub use abi::*;
pub use compose::*;
pub use handle::*;
pub use interface::*;
pub use into_param::*;
pub use runtime_name::*;
pub use runtime_type::*;
pub use to_impl::*;

// A [`Result`] type that provides Windows error information.
#[must_use]
pub type Result<T> = std::result::Result<T, Error>;

pub use bindings::Windows::Win32::System::Com::IAgileObject;

// TODO: rather than hiding, consider just removing
#[doc(hidden)]
pub type RawPtr = *mut std::ffi::c_void;

#[cfg(feature = "macros")]
pub use windows_macros::{build, generate, implement, include_bindings};

// TODO: remove this
#[cfg(feature = "macros")]
pub use windows_reader::workspace_dir;
