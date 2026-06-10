#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd)]
pub struct AsyncStatus(pub i32);
impl AsyncStatus {
    pub const Canceled: Self = Self(2);
    pub const Completed: Self = Self(1);
    pub const Error: Self = Self(3);
    pub const Started: Self = Self(0);
}
impl windows_core::TypeKind for AsyncStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for AsyncStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.AsyncStatus;i4)");
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.AsyncStatus");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CollectionChange(pub i32);
impl CollectionChange {
    pub const Reset: Self = Self(0);
    pub const ItemInserted: Self = Self(1);
    pub const ItemRemoved: Self = Self(2);
    pub const ItemChanged: Self = Self(3);
}
impl windows_core::TypeKind for CollectionChange {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for CollectionChange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Windows.Foundation.Collections.CollectionChange;i4)",
    );
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Windows.Foundation.Collections.CollectionChange",
    );
}
