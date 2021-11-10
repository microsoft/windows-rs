#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const CLSID_SoftwareBitmapNativeFactory: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x84e65691_8602_4a84_be46_708be9cd4b74);
#[doc = "*Required features: `Win32_System_WinRT_Graphics_Imaging`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISoftwareBitmapNative(pub ::windows::runtime::IUnknown);
impl ISoftwareBitmapNative {
    #[doc = "*Required features: `Win32_System_WinRT_Graphics_Imaging`*"]
    pub unsafe fn GetData<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISoftwareBitmapNative {
    type Vtable = ISoftwareBitmapNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x94bc8415_04ea_4b2e_af13_4de95aa898eb);
}
impl ::core::convert::From<ISoftwareBitmapNative> for ::windows::runtime::IUnknown {
    fn from(value: ISoftwareBitmapNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISoftwareBitmapNative> for ::windows::runtime::IUnknown {
    fn from(value: &ISoftwareBitmapNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISoftwareBitmapNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISoftwareBitmapNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT_Graphics_Imaging`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISoftwareBitmapNativeFactory(pub ::windows::runtime::IUnknown);
impl ISoftwareBitmapNativeFactory {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging"))]
    #[doc = "*Required features: `Win32_System_WinRT_Graphics_Imaging`, `Win32_Foundation`, `Win32_Graphics_Imaging`*"]
    pub unsafe fn CreateFromWICBitmap<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Graphics::Imaging::IWICBitmap>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::BOOL>, T: ::windows::runtime::Interface>(&self, data: Param0, forcereadonly: Param1) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), data.into_param().abi(), forcereadonly.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    #[doc = "*Required features: `Win32_System_WinRT_Graphics_Imaging`, `Win32_Foundation`, `Win32_Media_MediaFoundation`*"]
    pub unsafe fn CreateFromMF2DBuffer2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Media::MediaFoundation::IMF2DBuffer2>, Param4: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::BOOL>, T: ::windows::runtime::Interface>(
        &self,
        data: Param0,
        subtype: *const ::windows::runtime::GUID,
        width: u32,
        height: u32,
        forcereadonly: Param4,
        mindisplayaperture: *const super::super::super::super::Media::MediaFoundation::MFVideoArea,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(
            ::core::mem::transmute_copy(self),
            data.into_param().abi(),
            ::core::mem::transmute(subtype),
            ::core::mem::transmute(width),
            ::core::mem::transmute(height),
            forcereadonly.into_param().abi(),
            ::core::mem::transmute(mindisplayaperture),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISoftwareBitmapNativeFactory {
    type Vtable = ISoftwareBitmapNativeFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc3c181ec_2914_4791_af02_02d224a10b43);
}
impl ::core::convert::From<ISoftwareBitmapNativeFactory> for ::windows::runtime::IUnknown {
    fn from(value: ISoftwareBitmapNativeFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISoftwareBitmapNativeFactory> for ::windows::runtime::IUnknown {
    fn from(value: &ISoftwareBitmapNativeFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISoftwareBitmapNativeFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISoftwareBitmapNativeFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapNativeFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr, forcereadonly: super::super::super::super::Foundation::BOOL, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr, subtype: *const ::windows::runtime::GUID, width: u32, height: u32, forcereadonly: super::super::super::super::Foundation::BOOL, mindisplayaperture: *const super::super::super::super::Media::MediaFoundation::MFVideoArea, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation")))] usize,
);
