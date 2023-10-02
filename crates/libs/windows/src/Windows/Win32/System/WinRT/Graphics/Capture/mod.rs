#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGraphicsCaptureItemInterop(::windows_core::IUnknown);
impl IGraphicsCaptureItemInterop {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateForWindow<P0, T>(&self, window: P0) -> ::windows_core::Result<T>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::Foundation::HWND>,
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).CreateForWindow)(::windows_core::Interface::as_raw(self), window.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateForMonitor<P0, T>(&self, monitor: P0) -> ::windows_core::Result<T>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::Graphics::Gdi::HMONITOR>,
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).CreateForMonitor)(::windows_core::Interface::as_raw(self), monitor.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IGraphicsCaptureItemInterop, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGraphicsCaptureItemInterop {
    type Vtable = IGraphicsCaptureItemInterop_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGraphicsCaptureItemInterop {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3628e81b_3cac_4c60_b7f4_23ce0e0c3356);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureItemInterop_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: super::super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateForWindow: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CreateForMonitor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, monitor: super::super::super::super::Graphics::Gdi::HMONITOR, riid: *const ::windows_core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CreateForMonitor: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
