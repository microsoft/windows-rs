::windows_core::imp::com_interface!(IIsolatedEnvironmentInterop, IIsolatedEnvironmentInterop_Vtbl, 0x85713c2e_8e62_46c5_8de2_c647e1d54636);
::windows_core::imp::interface_hierarchy!(IIsolatedEnvironmentInterop, ::windows_core::IUnknown);
impl IIsolatedEnvironmentInterop {
    pub unsafe fn GetHostHwndInterop<P0>(&self, containerhwnd: P0) -> ::windows_core::Result<super::super::super::Foundation::HWND>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetHostHwndInterop)(::windows_core::Interface::as_raw(self), containerhwnd.into_param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IIsolatedEnvironmentInterop_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetHostHwndInterop: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::super::Foundation::HWND, *mut super::super::super::Foundation::HWND) -> ::windows_core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
