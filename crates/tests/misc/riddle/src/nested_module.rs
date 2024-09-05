#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub mod NestedModule {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct NestedType {
        pub field: f32,
    }
    impl windows_core::TypeKind for NestedType {
        type TypeKind = windows_core::CopyType;
    }
    impl windows_core::RuntimeType for NestedType {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::from_slice(b"struct(Test.NestedModule.NestedType;f4)");
    }
    impl Default for NestedType {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TestType {
    pub field: i32,
}
impl windows_core::TypeKind for TestType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TestType {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Test.TestType;i4)");
}
impl Default for TestType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
