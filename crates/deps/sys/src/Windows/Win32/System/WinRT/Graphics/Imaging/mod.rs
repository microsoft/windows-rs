#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub const CLSID_SoftwareBitmapNativeFactory: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2229687953,
    data2: 34306,
    data3: 19076,
    data4: [190, 70, 112, 139, 233, 205, 75, 116],
};
pub struct ISoftwareBitmapNative(pub *mut ::core::ffi::c_void);
pub struct ISoftwareBitmapNativeFactory(pub *mut ::core::ffi::c_void);
