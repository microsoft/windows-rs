pub mod Consumer {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Value {
        pub first: super::Producer::Owner_0,
        pub second: [super::Producer::Other_0_0; 2],
    }
    impl Default for Value {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }
}
pub mod Producer {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct Dependency {
        pub value: i32,
    }
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct Other {
        pub tag: u32,
    }
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct Other_0 {
        pub value: Dependency,
    }
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct Other_0_0 {
        pub value: u16,
    }
    #[repr(C)]
    #[cfg(target_arch = "x86")]
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct Owner {
        pub tag: u32,
    }
    #[repr(C)]
    #[cfg(target_arch = "x86")]
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct Owner_0 {
        pub value: u32,
        pub owner: *mut Owner,
    }
    #[repr(C)]
    #[cfg(any(
        target_arch = "aarch64",
        target_arch = "arm64ec",
        target_arch = "x86_64"
    ))]
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct Owner {
        pub tag: u32,
    }
    #[repr(C)]
    #[cfg(any(
        target_arch = "aarch64",
        target_arch = "arm64ec",
        target_arch = "x86_64"
    ))]
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct Owner_0 {
        pub value: u64,
        pub owner: *mut Owner,
    }
}
