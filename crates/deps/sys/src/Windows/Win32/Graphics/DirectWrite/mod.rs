#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_DirectWrite`*"]
    pub fn DWriteCreateFactory(factorytype: DWRITE_FACTORY_TYPE, iid: *const ::windows::runtime::GUID, factory: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
}
