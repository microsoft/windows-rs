mod agile_reference;
mod array;
mod as_impl;
mod com_interface;
mod error;
mod event;
mod guid;
mod hresult;
mod inspectable;
mod interface;
mod param;
mod runtime_name;
mod runtime_type;
mod scoped_interface;
mod strings;
mod r#type;
mod unknown;
mod weak;

use super::*;
pub use agile_reference::*;
pub use array::*;
pub use as_impl::*;
pub use com_interface::*;
pub use error::*;
pub use event::*;
pub use guid::*;
pub use hresult::*;
pub use inspectable::*;
pub use interface::*;
pub use param::*;
pub use r#type::*;
pub use runtime_name::*;
pub use runtime_type::*;
pub use scoped_interface::*;
pub use strings::*;
pub use unknown::*;
pub use weak::*;

/// A specialized [`Result`] type that provides Windows error information.
pub type Result<T> = std::result::Result<T, Error>;

#[doc(hidden)]
#[cfg(feature = "implement")]
pub use windows_implement::implement;

#[doc(hidden)]
#[cfg(feature = "implement")]
pub use windows_interface::interface;

/// Attempts to load the factory object for the given WinRT class.
/// This can be used to access COM interfaces implemented on a Windows Runtime class factory.
pub fn factory<C: RuntimeName, I: ComInterface>() -> Result<I> {
    imp::factory::<C, I>()
}
