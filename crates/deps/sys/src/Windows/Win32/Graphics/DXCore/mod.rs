#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_DXCore`*"]
    pub fn DXCoreCreateAdapterFactory(riid: *const ::windows::runtime::GUID, ppvfactory: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
}
