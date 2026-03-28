pub use windows_strings::*;

impl RuntimeType for HSTRING {
    const SIGNATURE: imp::ConstBuffer = imp::ConstBuffer::from_slice(b"string");
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
