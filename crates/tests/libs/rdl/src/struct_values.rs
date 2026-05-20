pub mod Test {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct Color {
        pub R: u8,
        pub G: u8,
        pub B: u8,
    }
}
