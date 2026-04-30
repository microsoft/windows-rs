mod array;
pub use array::*;

mod resources;
pub use resources::*;

/// Attempts to load the factory object for the given WinRT class.
/// This can be used to access COM interfaces implemented on a Windows Runtime class factory.
pub fn factory<C: RuntimeName, I: Interface>() -> Result<I> {
    imp::load_factory::<C, I>()
}

impl Param<PCWSTR> for &BSTR {
    unsafe fn param(self) -> ParamValue<PCWSTR> {
        ParamValue::Owned(PCWSTR(self.as_ptr()))
    }
}

impl TypeKind for BSTR {
    type TypeKind = CloneType;
}
