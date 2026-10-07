pub mod Control {
    pub mod Nested {
        #[repr(C)]
        #[derive(Clone, Copy, Default)]
        pub struct Value {
            pub value: u16,
        }
    }
}
pub mod Repro {
    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    pub struct Example {
        pub shared: super::Windows::Win32::System::Com::Shared,
        pub local: super::Windows::Win32::System::Com::Local,
        pub control: super::Control::Nested::Value,
    }
}
pub mod Windows {
    pub mod Win32 {
        #[repr(C)]
        #[derive(Clone, Copy, Default)]
        pub struct Root {
            pub shared: System::Com::Shared,
        }
        pub mod System {
            pub mod Com {
                #[repr(C)]
                #[derive(Clone, Copy, Default)]
                pub struct Local {
                    pub shared: Shared,
                    pub root: super::super::Root,
                    pub sibling: super::Storage::Shared,
                    pub child: Nested::Child,
                }
                #[repr(C)]
                #[derive(Clone, Copy, Default)]
                pub struct Shared {
                    pub value: i32,
                }
                pub mod Nested {
                    #[repr(C)]
                    #[derive(Clone, Copy, Default)]
                    pub struct Child {
                        pub parent: super::Shared,
                    }
                }
            }
            pub mod Storage {
                #[repr(C)]
                #[derive(Clone, Copy, Default)]
                pub struct Shared {
                    pub value: u64,
                }
            }
        }
    }
}
