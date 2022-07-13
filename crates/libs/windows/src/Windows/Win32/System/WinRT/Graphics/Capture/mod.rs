#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Capture\"`*"]
#[repr(transparent)]
pub struct IGraphicsCaptureItemInterop(::windows::core::IUnknown);
impl IGraphicsCaptureItemInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateForWindow<'a, P0, T>(&self, window: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).CreateForWindow)(::windows::core::Interface::as_raw(self), window.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateForMonitor<'a, P0, T>(&self, monitor: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::super::super::Graphics::Gdi::HMONITOR>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).CreateForMonitor)(::windows::core::Interface::as_raw(self), monitor.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IGraphicsCaptureItemInterop> for ::windows::core::IUnknown {
    fn from(value: IGraphicsCaptureItemInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IGraphicsCaptureItemInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IGraphicsCaptureItemInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGraphicsCaptureItemInterop> for ::windows::core::IUnknown {
    fn from(value: &IGraphicsCaptureItemInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IGraphicsCaptureItemInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGraphicsCaptureItemInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGraphicsCaptureItemInterop {}
impl ::core::fmt::Debug for IGraphicsCaptureItemInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGraphicsCaptureItemInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IGraphicsCaptureItemInterop {
    type Vtable = IGraphicsCaptureItemInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3628e81b_3cac_4c60_b7f4_23ce0e0c3356);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureItemInterop_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: super::super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateForWindow: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CreateForMonitor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, monitor: super::super::super::super::Graphics::Gdi::HMONITOR, riid: *const ::windows::core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CreateForMonitor: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
