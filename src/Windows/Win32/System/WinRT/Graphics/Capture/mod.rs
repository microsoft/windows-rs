#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_WinRT_Graphics_Capture`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGraphicsCaptureItemInterop(pub ::windows::runtime::IUnknown);
impl IGraphicsCaptureItemInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT_Graphics_Capture`, `Win32_Foundation`*"]
    pub unsafe fn CreateForWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::HWND>, T: ::windows::runtime::Interface>(&self, window: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), window.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_System_WinRT_Graphics_Capture`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn CreateForMonitor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Graphics::Gdi::HMONITOR>, T: ::windows::runtime::Interface>(&self, monitor: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), monitor.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGraphicsCaptureItemInterop {
    type Vtable = IGraphicsCaptureItemInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3628e81b_3cac_4c60_b7f4_23ce0e0c3356);
}
impl ::core::convert::From<IGraphicsCaptureItemInterop> for ::windows::runtime::IUnknown {
    fn from(value: IGraphicsCaptureItemInterop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGraphicsCaptureItemInterop> for ::windows::runtime::IUnknown {
    fn from(value: &IGraphicsCaptureItemInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGraphicsCaptureItemInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGraphicsCaptureItemInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureItemInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, window: super::super::super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, monitor: super::super::super::super::Graphics::Gdi::HMONITOR, riid: *const ::windows::runtime::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
);
