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
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for EventHandler<T> {
    type Vtable = EventHandler_Vtbl<T>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_u128(0x9de1c535_6ae1_11e0_84e1_18a905bcc53f);
}
impl<T: windows_core::RuntimeType + 'static> EventHandler<T> {
    pub fn Invoke<P0, P1>(&self, sender: P0, args: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<T>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Invoke)(
                windows_core::Interface::as_raw(this),
                sender.param().abi(),
                args.param().abi(),
            )
            .ok()
        }
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for EventHandler<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct EventHandler_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        windows_core::AbiType<T>,
    ) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
}
