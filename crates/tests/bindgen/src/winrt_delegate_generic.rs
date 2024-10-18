#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EventHandler<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown>
    for EventHandler<T>
{
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable>
    for EventHandler<T>
{
}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for EventHandler<T> {
    type Vtable = EventHandler_Vtbl;
    const IID: windows_core::GUID =
        windows_core::GUID::from_u128(0x9de1c535_6ae1_11e0_84e1_18a905bcc53f);
}
impl<T: windows_core::RuntimeType + 'static> core::ops::Deref for EventHandler<T> {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl<T: windows_core::RuntimeType + 'static> EventHandler<T> {
    pub fn Invoke(&self) {}
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for EventHandler<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct EventHandler_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
