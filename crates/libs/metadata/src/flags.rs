macro_rules! flags {
    ($name:ident, $size:ty) => {
        #[derive(Default, Copy, Clone, PartialEq, Eq)]
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
    pub const PRIVATE: Self = Self(0x1);
    pub const PUBLIC: Self = Self(0x6);
    pub const LITERAL: Self = Self(0x40);
    pub const STATIC: Self = Self(0x10);
    pub const SPECIAL: Self = Self(0x200);
    pub const RUNTIME_SPECIAL: Self = Self(0x400);
    pub const HAS_DEFAULT: Self = Self(0x8000);
}

flags!(MethodAttributes, usize);
impl MethodAttributes {
    pub const SPECIAL: Self = Self(0x800);
}

flags!(MethodImplAttributes, usize);
impl MethodImplAttributes {
    pub const PRESERVE_SIG: Self = Self(0x80);
}

flags!(ParamAttributes, usize);
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
}
