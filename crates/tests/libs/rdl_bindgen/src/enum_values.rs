#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

pub mod Test {
    #[repr(transparent)]
    #[derive(Clone, Copy)]
    pub struct Status(pub i32);
    impl Status {
        pub const Active: Self = Self(1i32);
        pub const Inactive: Self = Self(2i32);
        pub const Pending: Self = Self(3i32);
    }
}
