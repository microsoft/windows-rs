pub mod Test {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct Enum(pub i32);
    pub const First: Enum = Enum(0);
    pub const Second: Enum = Enum(1);
    pub const Third: Enum = Enum(2);
    pub mod Inner {
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
        pub struct Enum(pub i32);
        pub const First: Enum = Enum(0);
        pub const Second: Enum = Enum(1);
    }
}
