#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_core::imp::define_interface!(
    IKnownContactFieldStatics,
    IKnownContactFieldStatics_Vtbl,
    0x2e0e1b12_d627_4fca_bad4_1faf168c7d14
);
impl windows_core::RuntimeType for IKnownContactFieldStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownContactFieldStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Email: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub PhoneNumber: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Location: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub InstantMessage: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    ConvertNameToType: usize,
    ConvertTypeToName: usize,
}
#[deprecated(
    note = "KnownContactField may be altered or unavailable for releases after Windows 8.1. Instead, use ContactAddress, ContactPhone, ContactConnectedServiceAccount or ContactEmail."
)]
pub struct KnownContactField;
#[allow(deprecated)]
impl KnownContactField {
    #[deprecated(
        note = "IKnownContactFieldStatics may be altered or unavailable for releases after Windows 8.1. Instead, use ContactAddress, ContactPhone, ContactConnectedServiceAccount or ContactEmail."
    )]
    pub fn Email() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownContactFieldStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Email)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        })
    }
    #[deprecated(
        note = "IKnownContactFieldStatics may be altered or unavailable for releases after Windows 8.1. Instead, use ContactAddress, ContactPhone, ContactConnectedServiceAccount or ContactEmail."
    )]
    pub fn PhoneNumber() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownContactFieldStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhoneNumber)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        })
    }
    #[deprecated(
        note = "IKnownContactFieldStatics may be altered or unavailable for releases after Windows 8.1. Instead, use ContactAddress, ContactPhone, ContactConnectedServiceAccount or ContactEmail."
    )]
    pub fn Location() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownContactFieldStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Location)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        })
    }
    #[deprecated(
        note = "IKnownContactFieldStatics may be altered or unavailable for releases after Windows 8.1. Instead, use ContactAddress, ContactPhone, ContactConnectedServiceAccount or ContactEmail."
    )]
    pub fn InstantMessage() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownContactFieldStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstantMessage)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        })
    }
    fn IKnownContactFieldStatics<
        R,
        F: FnOnce(&IKnownContactFieldStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            KnownContactField,
            IKnownContactFieldStatics,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[allow(deprecated)]
impl windows_core::RuntimeName for KnownContactField {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.KnownContactField";
}
