#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Direct3D12`, `Win32_Graphics_Direct3D`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub fn D3D12CreateDevice(padapter: ::windows::runtime::RawPtr, minimumfeaturelevel: super::Direct3D::D3D_FEATURE_LEVEL, riid: *const ::windows::runtime::GUID, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
    pub fn D3D12CreateRootSignatureDeserializer(psrcdata: *const ::core::ffi::c_void, srcdatasizeinbytes: usize, prootsignaturedeserializerinterface: *const ::windows::runtime::GUID, pprootsignaturedeserializer: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
    pub fn D3D12CreateVersionedRootSignatureDeserializer(psrcdata: *const ::core::ffi::c_void, srcdatasizeinbytes: usize, prootsignaturedeserializerinterface: *const ::windows::runtime::GUID, pprootsignaturedeserializer: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
    pub fn D3D12EnableExperimentalFeatures(numfeatures: u32, piids: *const ::windows::runtime::GUID, pconfigurationstructs: *const ::core::ffi::c_void, pconfigurationstructsizes: *const u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
    pub fn D3D12GetDebugInterface(riid: *const ::windows::runtime::GUID, ppvdebug: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D12`*"]
    pub fn D3D12GetInterface(rclsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppvdebug: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D12`, `Win32_Graphics_Direct3D`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub fn D3D12SerializeRootSignature(prootsignature: *const D3D12_ROOT_SIGNATURE_DESC, version: D3D_ROOT_SIGNATURE_VERSION, ppblob: *mut ::windows::runtime::RawPtr, pperrorblob: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D12`, `Win32_Graphics_Direct3D`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub fn D3D12SerializeVersionedRootSignature(prootsignature: *const D3D12_VERSIONED_ROOT_SIGNATURE_DESC, ppblob: *mut ::windows::runtime::RawPtr, pperrorblob: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
}
