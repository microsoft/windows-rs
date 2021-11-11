#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_WinRT_Graphics_Capture`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGraphicsCaptureItemInterop(pub ::windows::core::IUnknown);
impl IGraphicsCaptureItemInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT_Graphics_Capture`, `Win32_Foundation`*"]
    pub unsafe fn CreateForWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::HWND>, T: ::windows::core::Interface>(&self, window: Param0) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), window.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_System_WinRT_Graphics_Capture`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn CreateForMonitor<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Graphics::Gdi::HMONITOR>, T: ::windows::core::Interface>(&self, monitor: Param0) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), monitor.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::core::Interface for IGraphicsCaptureItemInterop {
    type Vtable = IGraphicsCaptureItemInterop_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3628e81b_3cac_4c60_b7f4_23ce0e0c3356);
}
impl ::core::convert::From<IGraphicsCaptureItemInterop> for ::windows::core::IUnknown {
    fn from(value: IGraphicsCaptureItemInterop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGraphicsCaptureItemInterop> for ::windows::core::IUnknown {
    fn from(value: &IGraphicsCaptureItemInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGraphicsCaptureItemInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGraphicsCaptureItemInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureItemInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, window: super::super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, monitor: super::super::super::super::Graphics::Gdi::HMONITOR, riid: *const ::windows::core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
);
