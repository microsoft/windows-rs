#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_core::imp::define_interface!(
    IAsyncInfo,
    IAsyncInfo_Vtbl,
    0x00000036_0000_0000_c000_000000000046
);
windows_core::imp::interface_hierarchy!(
    IAsyncInfo,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl core::ops::Deref for IAsyncInfo {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl IAsyncInfo {
    pub fn Id(&self) {}
    pub fn Status(&self) {}
    pub fn ErrorCode(&self) {}
    pub fn Cancel(&self) {}
    pub fn Close(&self) {}
}
impl windows_core::RuntimeType for IAsyncInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAsyncInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
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
