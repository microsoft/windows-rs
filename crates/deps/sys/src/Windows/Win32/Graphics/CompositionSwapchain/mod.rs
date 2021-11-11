#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub fn CreatePresentationFactory(d3ddevice: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, presentationfactory: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
}
