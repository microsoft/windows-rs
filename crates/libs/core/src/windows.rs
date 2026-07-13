mod array;
pub use array::*;

impl Param<PCWSTR> for &BSTR {
    unsafe fn param(self) -> ParamValue<PCWSTR> {
        ParamValue::Owned(PCWSTR(self.as_ptr()))
    }
}

impl TypeKind for BSTR {
    type TypeKind = CloneType;
}
