#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
pub const CLSID_AudioFrameNativeFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 379626425, data2: 40805, data3: 16642, data4: [147, 103, 44, 218, 58, 79, 55, 42] };
pub const CLSID_VideoFrameNativeFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3516151914,
    data2: 1251,
    data3: 18452,
    data4: [129, 0, 178, 176, 174, 109, 120, 199],
};
#[repr(transparent)]
pub struct IAudioFrameNative(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioFrameNative {}
impl ::core::clone::Clone for IAudioFrameNative {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioFrameNativeFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioFrameNativeFactory {}
impl ::core::clone::Clone for IAudioFrameNativeFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoFrameNative(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoFrameNative {}
impl ::core::clone::Clone for IVideoFrameNative {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoFrameNativeFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoFrameNativeFactory {}
impl ::core::clone::Clone for IVideoFrameNativeFactory {
    fn clone(&self) -> Self {
        *self
    }
}
