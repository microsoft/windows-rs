#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

pub mod Test {
    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    pub struct Point {
        pub X: i32,
        pub Y: i32,
    }
}
