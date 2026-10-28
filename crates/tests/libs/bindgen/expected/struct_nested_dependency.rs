pub mod Consumer {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Value {
        pub first: super::Producer::Owner_Inner,
        pub second: [super::Producer::Other_Inner_Deep; 2],
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
    pub struct Other_Inner {
        pub value: Dependency,
    }
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct Other_Inner_Deep {
        pub value: u16,
    }
    #[repr(C)]
    #[cfg(target_arch = "x86")]
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct Owner {
        pub tag: u32,
        pub named: Owner_Owner_0,
        pub anonymous: Owner_0,
        pub tail: *mut Owner_1,
    }
    #[repr(C)]
    #[cfg(target_arch = "x86")]
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct Owner_0 {
        pub value: u16,
    }
    #[repr(C)]
    #[cfg(target_arch = "x86")]
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct Owner_1 {
        pub value: u32,
    }
    #[repr(C)]
    #[cfg(target_arch = "x86")]
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct Owner_Inner {
        pub value: u32,
        pub owner: *mut Owner,
    }
    #[repr(C)]
    #[cfg(target_arch = "x86")]
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct Owner_Owner_0 {
        pub value: u16,
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
        pub named: Owner_Owner_0,
        pub anonymous: Owner_0,
        pub tail: *mut Owner_1,
    }
    #[repr(C)]
    #[cfg(any(
        target_arch = "aarch64",
        target_arch = "arm64ec",
        target_arch = "x86_64"
    ))]
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct Owner_0 {
        pub value: u16,
    }
    #[repr(C)]
    #[cfg(any(
        target_arch = "aarch64",
        target_arch = "arm64ec",
        target_arch = "x86_64"
    ))]
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct Owner_1 {
        pub value: u32,
    }
    #[repr(C)]
    #[cfg(any(
        target_arch = "aarch64",
        target_arch = "arm64ec",
        target_arch = "x86_64"
    ))]
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct Owner_Inner {
        pub value: u64,
        pub owner: *mut Owner,
    }
    #[repr(C)]
    #[cfg(any(
        target_arch = "aarch64",
        target_arch = "arm64ec",
        target_arch = "x86_64"
    ))]
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct Owner_Owner_0 {
        pub value: u16,
    }
}
