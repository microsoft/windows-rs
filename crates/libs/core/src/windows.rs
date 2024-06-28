mod agile_reference;
pub use agile_reference::*;

mod array;
pub use array::*;

#[cfg(feature = "std")]
mod event;
#[cfg(feature = "std")]
pub use event::*;

mod handles;
pub use handles::*;

mod variant;
pub use variant::*;

/// Attempts to load the factory object for the given WinRT class.
/// This can be used to access COM interfaces implemented on a Windows Runtime class factory.
pub fn factory<C: RuntimeName, I: Interface>() -> Result<I> {
    imp::factory::<C, I>()
}
