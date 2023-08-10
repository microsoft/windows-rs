#![allow(non_upper_case_globals)]

macro_rules! flags {
    ($name:ident, $size:ty) => {
        #[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
        pub struct $name(pub $size);
        impl $name {
            pub fn contains(&self, contains: Self) -> bool {
                *self & contains == contains
            }
        }
        impl std::ops::BitOr for $name {
            type Output = Self;
            fn bitor(self, other: Self) -> Self {
                Self(self.0 | other.0)
            }
        }
        impl std::ops::BitAnd for $name {
            type Output = Self;
            fn bitand(self, other: Self) -> Self {
                Self(self.0 & other.0)
            }
        }
        impl std::ops::BitOrAssign for $name {
            fn bitor_assign(&mut self, other: Self) {
                self.0.bitor_assign(other.0)
            }
        }
        impl std::ops::BitAndAssign for $name {
            fn bitand_assign(&mut self, other: Self) {
                self.0.bitand_assign(other.0)
            }
        }
        impl std::ops::Not for $name {
            type Output = Self;
            fn not(self) -> Self {
                Self(self.0.not())
            }
        }
    };
}

flags!(FieldAttributes, u16);
impl FieldAttributes {
    pub const Private: Self = Self(0x1);
    pub const Public: Self = Self(0x6);
    pub const Literal: Self = Self(0x40);
    pub const Static: Self = Self(0x10);
    pub const SpecialName: Self = Self(0x200);
    pub const RTSpecialName: Self = Self(0x400);
    pub const HasDefault: Self = Self(0x8000);
}

flags!(MethodAttributes, u16);
impl MethodAttributes {
    pub const Abstract: Self = Self(0x400);
    pub const HideBySig: Self = Self(0x80);
    pub const NewSlot: Self = Self(0x100);
    pub const Public: Self = Self(0x6);
    pub const SpecialName: Self = Self(0x800);
    pub const Virtual: Self = Self(0x40);
}

flags!(MethodImplAttributes, u16);
impl MethodImplAttributes {
    pub const PreserveSig: Self = Self(0x80);
}

// These are not really ECMA-335 attributes but instead the flags found in the method signature.
flags!(MethodCallAttributes, u8);
impl MethodCallAttributes {
    pub const HASTHIS: Self = Self(0x20);
    pub const VARARG: Self = Self(0x05);
}

flags!(ParamAttributes, u16);
impl ParamAttributes {
    pub const In: Self = Self(0x1);
    pub const Out: Self = Self(0x2);
    pub const Optional: Self = Self(0x10);
}

flags!(PInvokeAttributes, usize);
impl PInvokeAttributes {
    pub const SupportsLastError: Self = Self(0x40);
    pub const CallConvPlatformapi: Self = Self(0x100);
    pub const CallConvCdecl: Self = Self(0x200);
    pub const CallConvStdcall: Self = Self(0x300);
    pub const CallConvThiscall: Self = Self(0x400);
    pub const CallConvFastcall: Self = Self(0x500);
}

flags!(TypeAttributes, u32);
impl TypeAttributes {
    pub const Public: Self = Self(0x1);
    pub const ExplicitLayout: Self = Self(0x10);
    pub const Abstract: Self = Self(0x80);
    pub const Sealed: Self = Self(0x100);
    pub const WindowsRuntime: Self = Self(0x4000);
    pub const Interface: Self = Self(0x20);
    pub const SequentialLayout: Self = Self(0x8);
    pub const Import: Self = Self(0x1000);
}

flags!(AssemblyFlags, u32);
impl AssemblyFlags {
    pub const WindowsRuntime: Self = Self(0x200);
}
