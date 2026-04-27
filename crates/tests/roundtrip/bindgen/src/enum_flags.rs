pub mod Test {
    pub const Execute: Options = Options(4u32);
    pub const None: Options = Options(0u32);
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct Options(pub u32);
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
    pub const Read: Options = Options(1u32);
    pub const Write: Options = Options(2u32);
}
