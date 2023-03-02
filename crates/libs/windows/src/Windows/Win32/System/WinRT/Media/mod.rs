#[doc = "*Required features: `\"Win32_System_WinRT_Media\"`*"]
#[repr(transparent)]
pub struct IAudioFrameNative(::windows::core::IUnknown);
impl IAudioFrameNative {
    pub unsafe fn GetData<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).GetData)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IAudioFrameNative, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IAudioFrameNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioFrameNative {}
impl ::core::fmt::Debug for IAudioFrameNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioFrameNative").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioFrameNative {
    type Vtable = IAudioFrameNative_Vtbl;
}
impl ::core::clone::Clone for IAudioFrameNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioFrameNative {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20be1e2e_930f_4746_9335_3c332f255093);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameNative_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Media\"`*"]
#[repr(transparent)]
pub struct IAudioFrameNativeFactory(::windows::core::IUnknown);
impl IAudioFrameNativeFactory {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn CreateFromMFSample<P0, P1, T>(&self, data: P0, forcereadonly: P1) -> ::windows::core::Result<T>
    where
        P0: ::windows::core::IntoParam<super::super::super::Media::MediaFoundation::IMFSample>,
        P1: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).CreateFromMFSample)(::windows::core::Interface::as_raw(self), data.into_param().abi(), forcereadonly.into_param().abi(), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IAudioFrameNativeFactory, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IAudioFrameNativeFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioFrameNativeFactory {}
impl ::core::fmt::Debug for IAudioFrameNativeFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioFrameNativeFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioFrameNativeFactory {
    type Vtable = IAudioFrameNativeFactory_Vtbl;
}
impl ::core::clone::Clone for IAudioFrameNativeFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioFrameNativeFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7bd67cf8_bf7d_43e6_af8d_b170ee0c0110);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameNativeFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub CreateFromMFSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, forcereadonly: super::super::super::Foundation::BOOL, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation")))]
    CreateFromMFSample: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Media\"`*"]
#[repr(transparent)]
pub struct IVideoFrameNative(::windows::core::IUnknown);
impl IVideoFrameNative {
    pub unsafe fn GetData<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).GetData)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).GetDevice)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVideoFrameNative, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IVideoFrameNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVideoFrameNative {}
impl ::core::fmt::Debug for IVideoFrameNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVideoFrameNative").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVideoFrameNative {
    type Vtable = IVideoFrameNative_Vtbl;
}
impl ::core::clone::Clone for IVideoFrameNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVideoFrameNative {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26ba702b_314a_4620_aaf6_7a51aa58fa18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoFrameNative_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Media\"`*"]
#[repr(transparent)]
pub struct IVideoFrameNativeFactory(::windows::core::IUnknown);
impl IVideoFrameNativeFactory {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn CreateFromMFSample<P0, P1, P2, T>(&self, data: P0, subtype: *const ::windows::core::GUID, width: u32, height: u32, forcereadonly: P1, mindisplayaperture: ::core::option::Option<*const super::super::super::Media::MediaFoundation::MFVideoArea>, device: P2) -> ::windows::core::Result<T>
    where
        P0: ::windows::core::IntoParam<super::super::super::Media::MediaFoundation::IMFSample>,
        P1: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
        P2: ::windows::core::IntoParam<super::super::super::Media::MediaFoundation::IMFDXGIDeviceManager>,
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).CreateFromMFSample)(::windows::core::Interface::as_raw(self), data.into_param().abi(), subtype, width, height, forcereadonly.into_param().abi(), ::core::mem::transmute(mindisplayaperture.unwrap_or(::std::ptr::null())), device.into_param().abi(), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVideoFrameNativeFactory, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IVideoFrameNativeFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVideoFrameNativeFactory {}
impl ::core::fmt::Debug for IVideoFrameNativeFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVideoFrameNativeFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVideoFrameNativeFactory {
    type Vtable = IVideoFrameNativeFactory_Vtbl;
}
impl ::core::clone::Clone for IVideoFrameNativeFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVideoFrameNativeFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69e3693e_8e1e_4e63_ac4c_7fdc21d9731d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoFrameNativeFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub CreateFromMFSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, subtype: *const ::windows::core::GUID, width: u32, height: u32, forcereadonly: super::super::super::Foundation::BOOL, mindisplayaperture: *const super::super::super::Media::MediaFoundation::MFVideoArea, device: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation")))]
    CreateFromMFSample: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Media\"`*"]
pub const CLSID_AudioFrameNativeFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16a0a3b9_9f65_4102_9367_2cda3a4f372a);
#[doc = "*Required features: `\"Win32_System_WinRT_Media\"`*"]
pub const CLSID_VideoFrameNativeFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd194386a_04e3_4814_8100_b2b0ae6d78c7);
#[cfg(feature = "implement")]
::core::include!("impl.rs");
