#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

pub mod Test {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct Options(pub u32);
    impl Options {
        pub const None: Self = Self(0u32);
        pub const Read: Self = Self(1u32);
        pub const Write: Self = Self(2u32);
        pub const Execute: Self = Self(4u32);
    }
    impl windows_core::TypeKind for Options {
        type TypeKind = windows_core::CopyType;
    }
    impl windows_core::RuntimeType for Options {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::from_slice(b"enum(Test.Options;u4)");
    }
    impl Options {
        pub const fn contains(&self, other: Self) -> bool {
            self.0 & other.0 == other.0
        }
    }
    impl core::ops::BitOr for Options {
        type Output = Self;
        fn bitor(self, other: Self) -> Self {
            Self(self.0 | other.0)
        }
    }
    impl core::ops::BitAnd for Options {
        type Output = Self;
        fn bitand(self, other: Self) -> Self {
            Self(self.0 & other.0)
        }
    }
    impl core::ops::BitOrAssign for Options {
        fn bitor_assign(&mut self, other: Self) {
            self.0.bitor_assign(other.0)
        }
    }
    impl core::ops::BitAndAssign for Options {
        fn bitand_assign(&mut self, other: Self) {
            self.0.bitand_assign(other.0)
        }
    }
    impl core::ops::Not for Options {
        type Output = Self;
        fn not(self) -> Self {
            Self(self.0.not())
        }
    }
}
