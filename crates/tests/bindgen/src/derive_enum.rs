#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd)]
pub struct AsyncStatus(pub i32);
impl AsyncStatus {
    pub const Canceled: Self = Self(2i32);
    pub const Completed: Self = Self(1i32);
    pub const Error: Self = Self(3i32);
    pub const Started: Self = Self(0i32);
}
impl windows_core::TypeKind for AsyncStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for AsyncStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.AsyncStatus;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CollectionChange(pub i32);
impl CollectionChange {
    pub const Reset: Self = Self(0i32);
    pub const ItemInserted: Self = Self(1i32);
    pub const ItemRemoved: Self = Self(2i32);
    pub const ItemChanged: Self = Self(3i32);
}
impl windows_core::TypeKind for CollectionChange {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for CollectionChange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.Foundation.Collections.CollectionChange;i4)",
    );
}
