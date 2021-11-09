#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const CLSID_AudioFrameNativeFactory: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(379626425, 40805, 16642, [147, 103, 44, 218, 58, 79, 55, 42]);
pub const CLSID_VideoFrameNativeFactory: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3516151914, 1251, 18452, [129, 0, 178, 176, 174, 109, 120, 199]);
#[doc = "*Required features: `Win32_System_WinRT_Media`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioFrameNative(pub ::windows::runtime::IUnknown);
impl IAudioFrameNative {
    #[doc = "*Required features: `Win32_System_WinRT_Media`*"]
    pub unsafe fn GetData<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAudioFrameNative {
    type Vtable = IAudioFrameNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(549330478, 37647, 18246, [147, 53, 60, 51, 47, 37, 80, 147]);
}
impl ::core::convert::From<IAudioFrameNative> for ::windows::runtime::IUnknown {
    fn from(value: IAudioFrameNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioFrameNative> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioFrameNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioFrameNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioFrameNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT_Media`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioFrameNativeFactory(pub ::windows::runtime::IUnknown);
impl IAudioFrameNativeFactory {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    #[doc = "*Required features: `Win32_System_WinRT_Media`, `Win32_Foundation`, `Win32_Media_MediaFoundation`*"]
    pub unsafe fn CreateFromMFSample<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Media::MediaFoundation::IMFSample>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>, T: ::windows::runtime::Interface>(&self, data: Param0, forcereadonly: Param1) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), data.into_param().abi(), forcereadonly.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAudioFrameNativeFactory {
    type Vtable = IAudioFrameNativeFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2077654264, 49021, 17382, [175, 141, 177, 112, 238, 12, 1, 16]);
}
impl ::core::convert::From<IAudioFrameNativeFactory> for ::windows::runtime::IUnknown {
    fn from(value: IAudioFrameNativeFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioFrameNativeFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioFrameNativeFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioFrameNativeFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioFrameNativeFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameNativeFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr, forcereadonly: super::super::super::Foundation::BOOL, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation")))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT_Media`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVideoFrameNative(pub ::windows::runtime::IUnknown);
impl IVideoFrameNative {
    #[doc = "*Required features: `Win32_System_WinRT_Media`*"]
    pub unsafe fn GetData<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_System_WinRT_Media`*"]
    pub unsafe fn GetDevice<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVideoFrameNative {
    type Vtable = IVideoFrameNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(649752619, 12618, 17952, [170, 246, 122, 81, 170, 88, 250, 24]);
}
impl ::core::convert::From<IVideoFrameNative> for ::windows::runtime::IUnknown {
    fn from(value: IVideoFrameNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVideoFrameNative> for ::windows::runtime::IUnknown {
    fn from(value: &IVideoFrameNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVideoFrameNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IVideoFrameNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoFrameNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT_Media`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVideoFrameNativeFactory(pub ::windows::runtime::IUnknown);
impl IVideoFrameNativeFactory {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    #[doc = "*Required features: `Win32_System_WinRT_Media`, `Win32_Foundation`, `Win32_Media_MediaFoundation`*"]
    pub unsafe fn CreateFromMFSample<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Media::MediaFoundation::IMFSample>, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>, Param6: ::windows::runtime::IntoParam<'a, super::super::super::Media::MediaFoundation::IMFDXGIDeviceManager>, T: ::windows::runtime::Interface>(
        &self,
        data: Param0,
        subtype: *const ::windows::runtime::GUID,
        width: u32,
        height: u32,
        forcereadonly: Param4,
        mindisplayaperture: *const super::super::super::Media::MediaFoundation::MFVideoArea,
        device: Param6,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            data.into_param().abi(),
            ::core::mem::transmute(subtype),
            ::core::mem::transmute(width),
            ::core::mem::transmute(height),
            forcereadonly.into_param().abi(),
            ::core::mem::transmute(mindisplayaperture),
            device.into_param().abi(),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVideoFrameNativeFactory {
    type Vtable = IVideoFrameNativeFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1776511294, 36382, 20067, [172, 76, 127, 220, 33, 217, 115, 29]);
}
impl ::core::convert::From<IVideoFrameNativeFactory> for ::windows::runtime::IUnknown {
    fn from(value: IVideoFrameNativeFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVideoFrameNativeFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IVideoFrameNativeFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVideoFrameNativeFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IVideoFrameNativeFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoFrameNativeFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr, subtype: *const ::windows::runtime::GUID, width: u32, height: u32, forcereadonly: super::super::super::Foundation::BOOL, mindisplayaperture: *const super::super::super::Media::MediaFoundation::MFVideoArea, device: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation")))] usize,
);
