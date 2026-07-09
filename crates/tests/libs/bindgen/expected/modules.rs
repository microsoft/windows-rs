pub mod Test {
    pub type Enum = i32;
    pub const First: Enum = 0;
    pub const Second: Enum = 1;
    pub const Third: Enum = 2;
    pub mod Inner {
        pub type Enum = i32;
        pub const First: Enum = 0;
        pub const Second: Enum = 1;
    }
}
