pub const CLSID_AudioFrameNativeFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16a0a3b9_9f65_4102_9367_2cda3a4f372a);
pub const CLSID_VideoFrameNativeFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd194386a_04e3_4814_8100_b2b0ae6d78c7);
#[doc = "*Required features: `\"Win32_System_WinRT_Media\"`*"]
#[repr(transparent)]
pub struct IAudioFrameNative(::windows::core::IUnknown);
impl IAudioFrameNative {
    pub unsafe fn GetData<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetData)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IAudioFrameNative> for ::windows::core::IUnknown {
    fn from(value: IAudioFrameNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAudioFrameNative> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAudioFrameNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioFrameNative> for ::windows::core::IUnknown {
    fn from(value: &IAudioFrameNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IAudioFrameNative> for ::windows::core::IInspectable {
    fn from(value: IAudioFrameNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAudioFrameNative> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IAudioFrameNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioFrameNative> for ::windows::core::IInspectable {
    fn from(value: &IAudioFrameNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IAudioFrameNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20be1e2e_930f_4746_9335_3c332f255093);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameNative_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Media\"`*"]
#[repr(transparent)]
pub struct IAudioFrameNativeFactory(::windows::core::IUnknown);
impl IAudioFrameNativeFactory {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn CreateFromMFSample<'a, P0, P1, T>(&self, data: P0, forcereadonly: P1) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Media::MediaFoundation::IMFSample>>,
        P1: ::std::convert::Into<super::super::super::Foundation::BOOL>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).CreateFromMFSample)(::windows::core::Interface::as_raw(self), data.into().abi(), forcereadonly.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IAudioFrameNativeFactory> for ::windows::core::IUnknown {
    fn from(value: IAudioFrameNativeFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAudioFrameNativeFactory> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAudioFrameNativeFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioFrameNativeFactory> for ::windows::core::IUnknown {
    fn from(value: &IAudioFrameNativeFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IAudioFrameNativeFactory> for ::windows::core::IInspectable {
    fn from(value: IAudioFrameNativeFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAudioFrameNativeFactory> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IAudioFrameNativeFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioFrameNativeFactory> for ::windows::core::IInspectable {
    fn from(value: &IAudioFrameNativeFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IAudioFrameNativeFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7bd67cf8_bf7d_43e6_af8d_b170ee0c0110);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameNativeFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
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
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetData)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetDevice)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IVideoFrameNative> for ::windows::core::IUnknown {
    fn from(value: IVideoFrameNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVideoFrameNative> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVideoFrameNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVideoFrameNative> for ::windows::core::IUnknown {
    fn from(value: &IVideoFrameNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IVideoFrameNative> for ::windows::core::IInspectable {
    fn from(value: IVideoFrameNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVideoFrameNative> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IVideoFrameNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVideoFrameNative> for ::windows::core::IInspectable {
    fn from(value: &IVideoFrameNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IVideoFrameNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26ba702b_314a_4620_aaf6_7a51aa58fa18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoFrameNative_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Media\"`*"]
#[repr(transparent)]
pub struct IVideoFrameNativeFactory(::windows::core::IUnknown);
impl IVideoFrameNativeFactory {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub unsafe fn CreateFromMFSample<'a, P0, P1, P2, T>(&self, data: P0, subtype: *const ::windows::core::GUID, width: u32, height: u32, forcereadonly: P1, mindisplayaperture: *const super::super::super::Media::MediaFoundation::MFVideoArea, device: P2) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Media::MediaFoundation::IMFSample>>,
        P1: ::std::convert::Into<super::super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Media::MediaFoundation::IMFDXGIDeviceManager>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).CreateFromMFSample)(::windows::core::Interface::as_raw(self), data.into().abi(), ::core::mem::transmute(subtype), width, height, forcereadonly.into(), ::core::mem::transmute(mindisplayaperture), device.into().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IVideoFrameNativeFactory> for ::windows::core::IUnknown {
    fn from(value: IVideoFrameNativeFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVideoFrameNativeFactory> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVideoFrameNativeFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVideoFrameNativeFactory> for ::windows::core::IUnknown {
    fn from(value: &IVideoFrameNativeFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IVideoFrameNativeFactory> for ::windows::core::IInspectable {
    fn from(value: IVideoFrameNativeFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVideoFrameNativeFactory> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IVideoFrameNativeFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVideoFrameNativeFactory> for ::windows::core::IInspectable {
    fn from(value: &IVideoFrameNativeFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IVideoFrameNativeFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69e3693e_8e1e_4e63_ac4c_7fdc21d9731d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoFrameNativeFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub CreateFromMFSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, subtype: *const ::windows::core::GUID, width: u32, height: u32, forcereadonly: super::super::super::Foundation::BOOL, mindisplayaperture: *const super::super::super::Media::MediaFoundation::MFVideoArea, device: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation")))]
    CreateFromMFSample: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
