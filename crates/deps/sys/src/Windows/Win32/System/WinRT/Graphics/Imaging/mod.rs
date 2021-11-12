#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
pub const CLSID_SoftwareBitmapNativeFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2229687953,
    data2: 34306,
    data3: 19076,
    data4: [190, 70, 112, 139, 233, 205, 75, 116],
};
#[repr(transparent)]
pub struct ISoftwareBitmapNative(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISoftwareBitmapNativeFactory(pub *mut ::core::ffi::c_void);
