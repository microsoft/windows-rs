#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const CLSID_SoftwareBitmapNativeFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84e65691_8602_4a84_be46_708be9cd4b74);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISoftwareBitmapNative(pub ::windows::core::IUnknown);
impl ISoftwareBitmapNative {
    pub unsafe fn GetData<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::core::Interface for ISoftwareBitmapNative {
    type Vtable = ISoftwareBitmapNative_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94bc8415_04ea_4b2e_af13_4de95aa898eb);
}
impl ::core::convert::From<ISoftwareBitmapNative> for ::windows::core::IUnknown {
    fn from(value: ISoftwareBitmapNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISoftwareBitmapNative> for ::windows::core::IUnknown {
    fn from(value: &ISoftwareBitmapNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISoftwareBitmapNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISoftwareBitmapNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapNative_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISoftwareBitmapNativeFactory(pub ::windows::core::IUnknown);
impl ISoftwareBitmapNativeFactory {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateFromWICBitmap<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Graphics::Imaging::IWICBitmap>, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::BOOL>, T: ::windows::core::Interface>(&self, data: Param0, forcereadonly: Param1) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), data.into_param().abi(), forcereadonly.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn CreateFromMF2DBuffer2<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Media::MediaFoundation::IMF2DBuffer2>, Param4: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::BOOL>, T: ::windows::core::Interface>(&self, data: Param0, subtype: *const ::windows::core::GUID, width: u32, height: u32, forcereadonly: Param4, mindisplayaperture: *const super::super::super::super::Media::MediaFoundation::MFVideoArea) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), data.into_param().abi(), ::core::mem::transmute(subtype), ::core::mem::transmute(width), ::core::mem::transmute(height), forcereadonly.into_param().abi(), ::core::mem::transmute(mindisplayaperture), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::core::Interface for ISoftwareBitmapNativeFactory {
    type Vtable = ISoftwareBitmapNativeFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3c181ec_2914_4791_af02_02d224a10b43);
}
impl ::core::convert::From<ISoftwareBitmapNativeFactory> for ::windows::core::IUnknown {
    fn from(value: ISoftwareBitmapNativeFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISoftwareBitmapNativeFactory> for ::windows::core::IUnknown {
    fn from(value: &ISoftwareBitmapNativeFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISoftwareBitmapNativeFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISoftwareBitmapNativeFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapNativeFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, data: ::windows::core::RawPtr, forcereadonly: super::super::super::super::Foundation::BOOL, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, data: ::windows::core::RawPtr, subtype: *const ::windows::core::GUID, width: u32, height: u32, forcereadonly: super::super::super::super::Foundation::BOOL, mindisplayaperture: *const super::super::super::super::Media::MediaFoundation::MFVideoArea, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation")))] usize,
);
