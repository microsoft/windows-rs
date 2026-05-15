windows_core::imp::define_interface!(INotificationActivationCallback, INotificationActivationCallback_Vtbl, 0x53e31837_6600_4a81_9395_75cffe746f94);
windows_core::imp::interface_hierarchy!(INotificationActivationCallback, windows_core::IUnknown);
impl INotificationActivationCallback {
    pub unsafe fn Activate<P0, P1>(&self, appusermodelid: P0, invokedargs: P1, data: &[NOTIFICATION_USER_INPUT_DATA]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), appusermodelid.param().abi(), invokedargs.param().abi(), core::mem::transmute(data.as_ptr()), data.len().try_into().unwrap()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INotificationActivationCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *const NOTIFICATION_USER_INPUT_DATA, u32) -> windows_core::HRESULT,
}
pub trait INotificationActivationCallback_Impl {
    fn Activate(&self, appusermodelid: &windows_core::PCWSTR, invokedargs: &windows_core::PCWSTR, data: *const NOTIFICATION_USER_INPUT_DATA, count: u32) -> windows_core::Result<()>;
}
impl INotificationActivationCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> Self
    where
        <Identity as windows_core::IUnknownImpl>::Impl: INotificationActivationCallback_Impl,
    {
        unsafe extern "system" fn Activate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, appusermodelid: windows_core::PCWSTR, invokedargs: windows_core::PCWSTR, data: *const NOTIFICATION_USER_INPUT_DATA, count: u32) -> windows_core::HRESULT
        where
            <Identity as windows_core::IUnknownImpl>::Impl: INotificationActivationCallback_Impl,
        {
            unsafe {
                let this__outer__: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                let this: &<Identity as windows_core::IUnknownImpl>::Impl = <Identity as windows_core::IUnknownImpl>::get_impl(this__outer__);
                INotificationActivationCallback_Impl::Activate(this, core::mem::transmute(&appusermodelid), core::mem::transmute(&invokedargs), core::mem::transmute_copy(&data), core::mem::transmute_copy(&count)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Activate: Activate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INotificationActivationCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for INotificationActivationCallback {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NOTIFICATION_USER_INPUT_DATA {
    pub Key: windows_core::PCWSTR,
    pub Value: windows_core::PCWSTR,
}
