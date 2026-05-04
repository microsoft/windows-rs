#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

pub struct GuidHelper;
impl GuidHelper {
    pub fn CreateNewGuid() -> windows_core::Result<windows_core::GUID> {
        Self::IGuidHelperStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateNewGuid)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    pub fn Empty() -> windows_core::Result<windows_core::GUID> {
        Self::IGuidHelperStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Empty)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        })
    }
    pub fn Equals(
        target: windows_core::GUID,
        value: windows_core::GUID,
    ) -> windows_core::Result<bool> {
        Self::IGuidHelperStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Equals)(
                windows_core::Interface::as_raw(this),
                &target,
                &value,
                &mut result__,
            )
            .map(|| result__)
        })
    }
    fn IGuidHelperStatics<R, F: FnOnce(&IGuidHelperStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GuidHelper, IGuidHelperStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for GuidHelper {
    const NAME: &'static str = "Windows.Foundation.GuidHelper";
}
windows_core::imp::define_interface!(
    IGuidHelperStatics,
    IGuidHelperStatics_Vtbl,
    0x59c7966b_ae52_5283_ad7f_a1b9e9678add
);
impl windows_core::RuntimeType for IGuidHelperStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidHelperStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateNewGuid: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::GUID,
    ) -> windows_core::HRESULT,
    pub Empty: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::GUID,
    ) -> windows_core::HRESULT,
    pub Equals: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        &windows_core::GUID,
        &windows_core::GUID,
        *mut bool,
    ) -> windows_core::HRESULT,
}
