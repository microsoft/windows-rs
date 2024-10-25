#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IRetailModeStatics, IRetailModeStatics_Vtbl, 0xd7ded029_fdda_43e7_93fb_e53ab6e89ec3);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IRetailModeStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IRetailModeStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub RetailModeEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RetailModeEnabled: usize,
}
#[cfg(feature = "deprecated")]
pub struct RetailMode;
#[cfg(feature = "deprecated")]
impl RetailMode {}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for RetailMode {
    const NAME: &'static str = "Windows.Phone.System.Profile.RetailMode";
}
