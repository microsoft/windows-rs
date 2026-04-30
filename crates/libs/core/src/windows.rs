mod array;
pub use array::*;

mod resources;
pub use resources::*;

impl Param<PCWSTR> for &BSTR {
    unsafe fn param(self) -> ParamValue<PCWSTR> {
        ParamValue::Owned(PCWSTR(self.as_ptr()))
    }
}

impl TypeKind for BSTR {
    type TypeKind = CloneType;
}
