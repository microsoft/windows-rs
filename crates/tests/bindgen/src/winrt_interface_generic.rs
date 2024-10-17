#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IAsyncOperation<TResult>(windows_core::IUnknown, core::marker::PhantomData<TResult>)
where
    TResult: windows_core::RuntimeType + 'static;
impl<TResult: windows_core::RuntimeType + 'static>
    windows_core::imp::CanInto<windows_core::IUnknown> for IAsyncOperation<TResult>
{
}
impl<TResult: windows_core::RuntimeType + 'static>
    windows_core::imp::CanInto<windows_core::IInspectable> for IAsyncOperation<TResult>
{
}
unsafe impl<TResult: windows_core::RuntimeType + 'static> windows_core::Interface
    for IAsyncOperation<TResult>
{
    type Vtable = IAsyncOperation_Vtbl;
    const IID: windows_core::GUID =
        windows_core::GUID::from_u128(0x9fc2b0bb_e446_44e2_aa61_9cab8f636af2);
}
impl<TResult: windows_core::RuntimeType + 'static> core::ops::Deref for IAsyncOperation<TResult> {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl<TResult: windows_core::RuntimeType + 'static> IAsyncOperation<TResult> {
    pub fn SetCompleted(&self) {}
    pub fn Completed(&self) {}
    pub fn GetResults(&self) {}
}
impl<TResult: windows_core::RuntimeType + 'static> windows_core::RuntimeType
    for IAsyncOperation<TResult>
{
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAsyncOperation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
