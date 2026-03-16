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
    pub struct S(pub u8);
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub union U {
        pub value: u8,
    }
    impl Default for U {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }
}
