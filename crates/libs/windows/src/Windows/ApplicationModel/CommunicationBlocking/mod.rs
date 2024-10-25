windows_core::imp::define_interface!(ICommunicationBlockingAccessManagerStatics, ICommunicationBlockingAccessManagerStatics_Vtbl, 0x1c969998_9d2a_5db7_edd5_0ce407fc2595);
impl windows_core::RuntimeType for ICommunicationBlockingAccessManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICommunicationBlockingAccessManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsBlockingActive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsBlockedNumberAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ShowBlockNumbersUI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ShowBlockNumbersUI: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ShowUnblockNumbersUI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ShowUnblockNumbersUI: usize,
    pub ShowBlockedCallsUI: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowBlockedMessagesUI: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICommunicationBlockingAppManagerStatics, ICommunicationBlockingAppManagerStatics_Vtbl, 0x77db58ec_14a6_4baa_942a_6a673d999bf2);
impl windows_core::RuntimeType for ICommunicationBlockingAppManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICommunicationBlockingAppManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsCurrentAppActiveBlockingApp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ShowCommunicationBlockingSettingsUI: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICommunicationBlockingAppManagerStatics2, ICommunicationBlockingAppManagerStatics2_Vtbl, 0x14a68edd_ed88_457a_a364_a3634d6f166d);
impl windows_core::RuntimeType for ICommunicationBlockingAppManagerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICommunicationBlockingAppManagerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestSetAsActiveBlockingAppAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub struct CommunicationBlockingAccessManager;
impl CommunicationBlockingAccessManager {}
impl windows_core::RuntimeName for CommunicationBlockingAccessManager {
    const NAME: &'static str = "Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingAccessManager";
}
pub struct CommunicationBlockingAppManager;
impl CommunicationBlockingAppManager {}
impl windows_core::RuntimeName for CommunicationBlockingAppManager {
    const NAME: &'static str = "Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingAppManager";
}
