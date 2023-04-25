#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct ISoftwareBitmapNative(::windows_core::IUnknown);
impl ISoftwareBitmapNative {
    pub unsafe fn GetData<T>(&self) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetData)(::windows_core::Interface::as_raw(self), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(ISoftwareBitmapNative, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
unsafe impl ::windows_core::Interface for ISoftwareBitmapNative {
    type Vtable = ISoftwareBitmapNative_Vtbl;
}
impl ::core::clone::Clone for ISoftwareBitmapNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISoftwareBitmapNative {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x94bc8415_04ea_4b2e_af13_4de95aa898eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapNative_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct ISoftwareBitmapNativeFactory(::windows_core::IUnknown);
impl ISoftwareBitmapNativeFactory {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateFromWICBitmap<P0, P1, T>(&self, data: P0, forcereadonly: P1) -> ::windows_core::Result<T>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::Graphics::Imaging::IWICBitmap>,
        P1: ::windows_core::IntoParam<super::super::super::super::Foundation::BOOL>,
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).CreateFromWICBitmap)(::windows_core::Interface::as_raw(self), data.into_param().abi(), forcereadonly.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn CreateFromMF2DBuffer2<P0, P1, T>(&self, data: P0, subtype: *const ::windows_core::GUID, width: u32, height: u32, forcereadonly: P1, mindisplayaperture: ::core::option::Option<*const super::super::super::super::Media::MediaFoundation::MFVideoArea>) -> ::windows_core::Result<T>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::Media::MediaFoundation::IMF2DBuffer2>,
        P1: ::windows_core::IntoParam<super::super::super::super::Foundation::BOOL>,
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).CreateFromMF2DBuffer2)(::windows_core::Interface::as_raw(self), data.into_param().abi(), subtype, width, height, forcereadonly.into_param().abi(), ::core::mem::transmute(mindisplayaperture.unwrap_or(::std::ptr::null())), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(ISoftwareBitmapNativeFactory, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
unsafe impl ::windows_core::Interface for ISoftwareBitmapNativeFactory {
    type Vtable = ISoftwareBitmapNativeFactory_Vtbl;
}
impl ::core::clone::Clone for ISoftwareBitmapNativeFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISoftwareBitmapNativeFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc3c181ec_2914_4791_af02_02d224a10b43);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapNativeFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging"))]
    pub CreateFromWICBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, forcereadonly: super::super::super::super::Foundation::BOOL, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging")))]
    CreateFromWICBitmap: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub CreateFromMF2DBuffer2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, subtype: *const ::windows_core::GUID, width: u32, height: u32, forcereadonly: super::super::super::super::Foundation::BOOL, mindisplayaperture: *const super::super::super::super::Media::MediaFoundation::MFVideoArea, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation")))]
    CreateFromMF2DBuffer2: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Imaging\"`*"]
pub const CLSID_SoftwareBitmapNativeFactory: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x84e65691_8602_4a84_be46_708be9cd4b74);
#[cfg(feature = "implement")]
::core::include!("impl.rs");
