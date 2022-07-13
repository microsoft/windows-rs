pub const CLSID_SoftwareBitmapNativeFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84e65691_8602_4a84_be46_708be9cd4b74);
#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct ISoftwareBitmapNative(::windows::core::IUnknown);
impl ISoftwareBitmapNative {
    pub unsafe fn GetData<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetData)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<ISoftwareBitmapNative> for ::windows::core::IUnknown {
    fn from(value: ISoftwareBitmapNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISoftwareBitmapNative> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISoftwareBitmapNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISoftwareBitmapNative> for ::windows::core::IUnknown {
    fn from(value: &ISoftwareBitmapNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ISoftwareBitmapNative> for ::windows::core::IInspectable {
    fn from(value: ISoftwareBitmapNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISoftwareBitmapNative> for &'a ::windows::core::IInspectable {
    fn from(value: &'a ISoftwareBitmapNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISoftwareBitmapNative> for ::windows::core::IInspectable {
    fn from(value: &ISoftwareBitmapNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
impl ::core::fmt::Debug for ISoftwareBitmapNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISoftwareBitmapNative").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISoftwareBitmapNative {
    type Vtable = ISoftwareBitmapNative_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94bc8415_04ea_4b2e_af13_4de95aa898eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapNative_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct ISoftwareBitmapNativeFactory(::windows::core::IUnknown);
impl ISoftwareBitmapNativeFactory {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateFromWICBitmap<'a, P0, P1, T>(&self, data: P0, forcereadonly: P1) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::super::Graphics::Imaging::IWICBitmap>>,
        P1: ::std::convert::Into<super::super::super::super::Foundation::BOOL>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).CreateFromWICBitmap)(::windows::core::Interface::as_raw(self), data.into().abi(), forcereadonly.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn CreateFromMF2DBuffer2<'a, P0, P1, T>(&self, data: P0, subtype: *const ::windows::core::GUID, width: u32, height: u32, forcereadonly: P1, mindisplayaperture: *const super::super::super::super::Media::MediaFoundation::MFVideoArea) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::super::Media::MediaFoundation::IMF2DBuffer2>>,
        P1: ::std::convert::Into<super::super::super::super::Foundation::BOOL>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).CreateFromMF2DBuffer2)(::windows::core::Interface::as_raw(self), data.into().abi(), ::core::mem::transmute(subtype), width, height, forcereadonly.into(), ::core::mem::transmute(mindisplayaperture), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<ISoftwareBitmapNativeFactory> for ::windows::core::IUnknown {
    fn from(value: ISoftwareBitmapNativeFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISoftwareBitmapNativeFactory> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISoftwareBitmapNativeFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISoftwareBitmapNativeFactory> for ::windows::core::IUnknown {
    fn from(value: &ISoftwareBitmapNativeFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ISoftwareBitmapNativeFactory> for ::windows::core::IInspectable {
    fn from(value: ISoftwareBitmapNativeFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISoftwareBitmapNativeFactory> for &'a ::windows::core::IInspectable {
    fn from(value: &'a ISoftwareBitmapNativeFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISoftwareBitmapNativeFactory> for ::windows::core::IInspectable {
    fn from(value: &ISoftwareBitmapNativeFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
impl ::core::fmt::Debug for ISoftwareBitmapNativeFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISoftwareBitmapNativeFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISoftwareBitmapNativeFactory {
    type Vtable = ISoftwareBitmapNativeFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3c181ec_2914_4791_af02_02d224a10b43);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapNativeFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging"))]
    pub CreateFromWICBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, forcereadonly: super::super::super::super::Foundation::BOOL, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging")))]
    CreateFromWICBitmap: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub CreateFromMF2DBuffer2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, subtype: *const ::windows::core::GUID, width: u32, height: u32, forcereadonly: super::super::super::super::Foundation::BOOL, mindisplayaperture: *const super::super::super::super::Media::MediaFoundation::MFVideoArea, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation")))]
    CreateFromMF2DBuffer2: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
