#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Foundation`, `Win32_Graphics_Direct3D`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
    pub fn D3D10CompileEffectFromMemory(pdata: *const ::core::ffi::c_void, datalength: usize, psrcfilename: super::super::Foundation::PSTR, pdefines: *const super::Direct3D::D3D_SHADER_MACRO, pinclude: ::windows::runtime::RawPtr, hlslflags: u32, fxflags: u32, ppcompiledeffect: *mut ::windows::runtime::RawPtr, pperrors: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Foundation`, `Win32_Graphics_Direct3D`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
    pub fn D3D10CompileShader(psrcdata: super::super::Foundation::PSTR, srcdatasize: usize, pfilename: super::super::Foundation::PSTR, pdefines: *const super::Direct3D::D3D_SHADER_MACRO, pinclude: ::windows::runtime::RawPtr, pfunctionname: super::super::Foundation::PSTR, pprofile: super::super::Foundation::PSTR, flags: u32, ppshader: *mut ::windows::runtime::RawPtr, pperrormsgs: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Graphics_Direct3D`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub fn D3D10CreateBlob(numbytes: usize, ppbuffer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Foundation`, `Win32_Graphics_Dxgi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
    pub fn D3D10CreateDevice(padapter: ::windows::runtime::RawPtr, drivertype: D3D10_DRIVER_TYPE, software: super::super::Foundation::HINSTANCE, flags: u32, sdkversion: u32, ppdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Foundation`, `Win32_Graphics_Dxgi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
    pub fn D3D10CreateDevice1(padapter: ::windows::runtime::RawPtr, drivertype: D3D10_DRIVER_TYPE, software: super::super::Foundation::HINSTANCE, flags: u32, hardwarelevel: D3D10_FEATURE_LEVEL1, sdkversion: u32, ppdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Foundation`, `Win32_Graphics_Dxgi`, `Win32_Graphics_Dxgi_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common"))]
    pub fn D3D10CreateDeviceAndSwapChain(padapter: ::windows::runtime::RawPtr, drivertype: D3D10_DRIVER_TYPE, software: super::super::Foundation::HINSTANCE, flags: u32, sdkversion: u32, pswapchaindesc: *const super::Dxgi::DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::windows::runtime::RawPtr, ppdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Foundation`, `Win32_Graphics_Dxgi`, `Win32_Graphics_Dxgi_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common"))]
    pub fn D3D10CreateDeviceAndSwapChain1(padapter: ::windows::runtime::RawPtr, drivertype: D3D10_DRIVER_TYPE, software: super::super::Foundation::HINSTANCE, flags: u32, hardwarelevel: D3D10_FEATURE_LEVEL1, sdkversion: u32, pswapchaindesc: *const super::Dxgi::DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::windows::runtime::RawPtr, ppdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`*"]
    pub fn D3D10CreateEffectFromMemory(pdata: *const ::core::ffi::c_void, datalength: usize, fxflags: u32, pdevice: ::windows::runtime::RawPtr, peffectpool: ::windows::runtime::RawPtr, ppeffect: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`*"]
    pub fn D3D10CreateEffectPoolFromMemory(pdata: *const ::core::ffi::c_void, datalength: usize, fxflags: u32, pdevice: ::windows::runtime::RawPtr, ppeffectpool: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`*"]
    pub fn D3D10CreateStateBlock(pdevice: ::windows::runtime::RawPtr, pstateblockmask: *const D3D10_STATE_BLOCK_MASK, ppstateblock: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Foundation`, `Win32_Graphics_Direct3D`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
    pub fn D3D10DisassembleEffect(peffect: ::windows::runtime::RawPtr, enablecolorcode: super::super::Foundation::BOOL, ppdisassembly: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Foundation`, `Win32_Graphics_Direct3D`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
    pub fn D3D10DisassembleShader(pshader: *const ::core::ffi::c_void, bytecodelength: usize, enablecolorcode: super::super::Foundation::BOOL, pcomments: super::super::Foundation::PSTR, ppdisassembly: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3D10GetGeometryShaderProfile(pdevice: ::windows::runtime::RawPtr) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Graphics_Direct3D`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub fn D3D10GetInputAndOutputSignatureBlob(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppsignatureblob: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Graphics_Direct3D`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub fn D3D10GetInputSignatureBlob(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppsignatureblob: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Graphics_Direct3D`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub fn D3D10GetOutputSignatureBlob(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppsignatureblob: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3D10GetPixelShaderProfile(pdevice: ::windows::runtime::RawPtr) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Graphics_Direct3D`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub fn D3D10GetShaderDebugInfo(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppdebuginfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3D10GetVertexShaderProfile(pdevice: ::windows::runtime::RawPtr) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Foundation`, `Win32_Graphics_Direct3D`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
    pub fn D3D10PreprocessShader(psrcdata: super::super::Foundation::PSTR, srcdatasize: usize, pfilename: super::super::Foundation::PSTR, pdefines: *const super::Direct3D::D3D_SHADER_MACRO, pinclude: ::windows::runtime::RawPtr, ppshadertext: *mut ::windows::runtime::RawPtr, pperrormsgs: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`*"]
    pub fn D3D10ReflectShader(pshaderbytecode: *const ::core::ffi::c_void, bytecodelength: usize, ppreflector: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`*"]
    pub fn D3D10StateBlockMaskDifference(pa: *const D3D10_STATE_BLOCK_MASK, pb: *const D3D10_STATE_BLOCK_MASK, presult: *mut D3D10_STATE_BLOCK_MASK) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`*"]
    pub fn D3D10StateBlockMaskDisableAll(pmask: *mut D3D10_STATE_BLOCK_MASK) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`*"]
    pub fn D3D10StateBlockMaskDisableCapture(pmask: *mut D3D10_STATE_BLOCK_MASK, statetype: D3D10_DEVICE_STATE_TYPES, rangestart: u32, rangelength: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`*"]
    pub fn D3D10StateBlockMaskEnableAll(pmask: *mut D3D10_STATE_BLOCK_MASK) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`*"]
    pub fn D3D10StateBlockMaskEnableCapture(pmask: *mut D3D10_STATE_BLOCK_MASK, statetype: D3D10_DEVICE_STATE_TYPES, rangestart: u32, rangelength: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3D10StateBlockMaskGetSetting(pmask: *const D3D10_STATE_BLOCK_MASK, statetype: D3D10_DEVICE_STATE_TYPES, entry: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`*"]
    pub fn D3D10StateBlockMaskIntersect(pa: *const D3D10_STATE_BLOCK_MASK, pb: *const D3D10_STATE_BLOCK_MASK, presult: *mut D3D10_STATE_BLOCK_MASK) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D10`*"]
    pub fn D3D10StateBlockMaskUnion(pa: *const D3D10_STATE_BLOCK_MASK, pb: *const D3D10_STATE_BLOCK_MASK, presult: *mut D3D10_STATE_BLOCK_MASK) -> ::windows::runtime::HRESULT;
}
