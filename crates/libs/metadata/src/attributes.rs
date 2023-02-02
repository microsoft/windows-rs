use super::*;

flags!(FieldAttributes, u16);
impl FieldAttributes {
    pub const PRIVATE: Self = Self(0x1);
    pub const PUBLIC: Self = Self(0x6);
    pub const LITERAL: Self = Self(0x40);
    pub const STATIC: Self = Self(0x10);
    pub const SPECIAL: Self = Self(0x200);
    pub const RUNTIME_SPECIAL: Self = Self(0x400);
    pub const HAS_DEFAULT: Self = Self(0x8000);
}

flags!(MethodAttributes, u16);
impl MethodAttributes {
    pub const ABSTRACT: Self = Self(0x400);
    pub const HIDE_BY_SIG: Self = Self(0x80);
    pub const NEW_SLOT: Self = Self(0x100);
    pub const PUBLIC: Self = Self(0x6);
    pub const SPECIAL: Self = Self(0x800);
    pub const VIRTUAL: Self = Self(0x40);
}

flags!(MethodImplAttributes, usize);
impl MethodImplAttributes {
    pub const PRESERVE_SIG: Self = Self(0x80);
}

flags!(ParamAttributes, u16);
impl ParamAttributes {
    pub const INPUT: Self = Self(0x1);
    pub const OUTPUT: Self = Self(0x2);
    pub const OPTIONAL: Self = Self(0x10);
}

flags!(PInvokeAttributes, usize);
impl PInvokeAttributes {
    pub const LAST_ERROR: Self = Self(0x40);
    pub const CONV_PLATFORM: Self = Self(0x100);
    pub const CONV_CDECL: Self = Self(0x200);
    pub const CONV_STDCALL: Self = Self(0x300);
    pub const CONV_THISCALL: Self = Self(0x400);
    pub const CONV_FASTCALL: Self = Self(0x500);
}

flags!(TypeAttributes, u32);
impl TypeAttributes {
    pub const PUBLIC: Self = Self(0x1);
    pub const EXPLICIT_LAYOUT: Self = Self(0x10);
    pub const ABSTRACT: Self = Self(0x80);
    pub const SEALED: Self = Self(0x100);
    pub const WINRT: Self = Self(0x4000);
    pub const INTERFACE: Self = Self(0x20);
    pub const SEQUENTIAL_LAYOUT: Self = Self(0x8);
}
