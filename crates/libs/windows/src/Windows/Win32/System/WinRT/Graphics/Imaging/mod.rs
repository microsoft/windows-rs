#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub const CLSID_SoftwareBitmapNativeFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84e65691_8602_4a84_be46_708be9cd4b74);
#[doc = "*Required features: 'Win32_System_WinRT_Graphics_Imaging'*"]
#[repr(transparent)]
pub struct ISoftwareBitmapNative(::windows::core::IUnknown);
impl ISoftwareBitmapNative {
    #[doc = "*Required features: 'Win32_System_WinRT_Graphics_Imaging'*"]
    pub unsafe fn GetData<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<ISoftwareBitmapNative> for ::windows::core::IInspectable {
    fn from(value: ISoftwareBitmapNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISoftwareBitmapNative> for ::windows::core::IInspectable {
    fn from(value: &ISoftwareBitmapNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISoftwareBitmapNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ISoftwareBitmapNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISoftwareBitmapNative> for ::windows::core::IUnknown {
    fn from(value: ISoftwareBitmapNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISoftwareBitmapNative> for ::windows::core::IUnknown {
    fn from(value: &ISoftwareBitmapNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISoftwareBitmapNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISoftwareBitmapNative {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISoftwareBitmapNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISoftwareBitmapNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISoftwareBitmapNative {}
unsafe impl ::windows::core::Interface for ISoftwareBitmapNative {
    type Vtable = ISoftwareBitmapNativeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94bc8415_04ea_4b2e_af13_4de95aa898eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapNativeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_WinRT_Graphics_Imaging'*"]
#[repr(transparent)]
pub struct ISoftwareBitmapNativeFactory(::windows::core::IUnknown);
impl ISoftwareBitmapNativeFactory {
    #[doc = "*Required features: 'Win32_System_WinRT_Graphics_Imaging', 'Win32_Foundation', 'Win32_Graphics_Imaging'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateFromWICBitmap<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Graphics::Imaging::IWICBitmap>, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::BOOL>, T: ::windows::core::Interface>(&self, data: Param0, forcereadonly: Param1) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), data.into_param().abi(), forcereadonly.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Graphics_Imaging', 'Win32_Foundation', 'Win32_Media_MediaFoundation'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn CreateFromMF2DBuffer2<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Media::MediaFoundation::IMF2DBuffer2>, Param4: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::BOOL>, T: ::windows::core::Interface>(&self, data: Param0, subtype: *const ::windows::core::GUID, width: u32, height: u32, forcereadonly: Param4, mindisplayaperture: *const super::super::super::super::Media::MediaFoundation::MFVideoArea) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), data.into_param().abi(), ::core::mem::transmute(subtype), ::core::mem::transmute(width), ::core::mem::transmute(height), forcereadonly.into_param().abi(), ::core::mem::transmute(mindisplayaperture), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<ISoftwareBitmapNativeFactory> for ::windows::core::IInspectable {
    fn from(value: ISoftwareBitmapNativeFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISoftwareBitmapNativeFactory> for ::windows::core::IInspectable {
    fn from(value: &ISoftwareBitmapNativeFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISoftwareBitmapNativeFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ISoftwareBitmapNativeFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISoftwareBitmapNativeFactory> for ::windows::core::IUnknown {
    fn from(value: ISoftwareBitmapNativeFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISoftwareBitmapNativeFactory> for ::windows::core::IUnknown {
    fn from(value: &ISoftwareBitmapNativeFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISoftwareBitmapNativeFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISoftwareBitmapNativeFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISoftwareBitmapNativeFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISoftwareBitmapNativeFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISoftwareBitmapNativeFactory {}
unsafe impl ::windows::core::Interface for ISoftwareBitmapNativeFactory {
    type Vtable = ISoftwareBitmapNativeFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3c181ec_2914_4791_af02_02d224a10b43);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapNativeFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, forcereadonly: super::super::super::super::Foundation::BOOL, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, subtype: *const ::windows::core::GUID, width: u32, height: u32, forcereadonly: super::super::super::super::Foundation::BOOL, mindisplayaperture: *const super::super::super::super::Media::MediaFoundation::MFVideoArea, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation")))] usize,
);
