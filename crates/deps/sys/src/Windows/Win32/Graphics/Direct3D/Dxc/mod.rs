#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Dxc`*"]
    pub fn DxcCreateInstance(rclsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Dxc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn DxcCreateInstance2(pmalloc: super::super::super::System::Com::IMalloc, rclsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
pub const CLSID_DxcAssembler: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3609779048,
    data2: 63747,
    data3: 20352,
    data4: [148, 205, 220, 207, 118, 236, 113, 81],
};
pub const CLSID_DxcCompiler: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1944202643,
    data2: 59086,
    data3: 18419,
    data4: [181, 191, 240, 102, 79, 57, 193, 176],
};
pub const CLSID_DxcCompilerArgs: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1045868162,
    data2: 8781,
    data3: 18191,
    data4: [161, 161, 254, 48, 22, 238, 159, 157],
};
pub const CLSID_DxcContainerBuilder: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2484290196,
    data2: 16671,
    data3: 17780,
    data4: [180, 208, 135, 65, 226, 82, 64, 210],
};
pub const CLSID_DxcContainerReflection: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3119858825,
    data2: 21944,
    data3: 16396,
    data4: [186, 58, 22, 117, 228, 114, 139, 145],
};
pub const CLSID_DxcDiaDataSource: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3441388403,
    data2: 10928,
    data3: 18509,
    data4: [142, 220, 235, 231, 164, 60, 160, 159],
};
pub const CLSID_DxcLibrary: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1648744111,
    data2: 26336,
    data3: 18685,
    data4: [128, 180, 77, 39, 23, 150, 116, 140],
};
pub const CLSID_DxcLinker: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4016734343, data2: 45290, data3: 19798, data4: [158, 69, 208, 126, 26, 139, 120, 6] };
pub const CLSID_DxcOptimizer: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2922174367,
    data2: 52258,
    data3: 17727,
    data4: [155, 107, 177, 36, 231, 165, 32, 76],
};
pub const CLSID_DxcPdbUtils: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1415716347,
    data2: 62158,
    data3: 17790,
    data4: [174, 140, 236, 53, 95, 174, 236, 124],
};
pub const CLSID_DxcValidator: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2359550485,
    data2: 63272,
    data3: 19699,
    data4: [140, 221, 136, 175, 145, 117, 135, 161],
};
pub struct DXC_CP(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D_Dxc`*"]
pub const DXC_HASHFLAG_INCLUDES_SOURCE: u32 = 1u32;
pub struct DXC_OUT_KIND(i32);
pub struct DxcArgPair(i32);
pub struct DxcBuffer(i32);
pub struct DxcCreateInstance2Proc(i32);
pub struct DxcCreateInstanceProc(i32);
pub struct DxcDefine(i32);
pub struct DxcShaderHash(i32);
#[doc = "*Required features: `Win32_Graphics_Direct3D_Dxc`*"]
pub const DxcValidatorFlags_Default: u32 = 0u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Dxc`*"]
pub const DxcValidatorFlags_InPlaceEdit: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Dxc`*"]
pub const DxcValidatorFlags_ModuleOnly: u32 = 4u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Dxc`*"]
pub const DxcValidatorFlags_RootSignatureOnly: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Dxc`*"]
pub const DxcValidatorFlags_ValidMask: u32 = 7u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Dxc`*"]
pub const DxcVersionInfoFlags_Debug: u32 = 1u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Dxc`*"]
pub const DxcVersionInfoFlags_Internal: u32 = 2u32;
#[doc = "*Required features: `Win32_Graphics_Direct3D_Dxc`*"]
pub const DxcVersionInfoFlags_None: u32 = 0u32;
pub struct IDxcAssembler(i32);
pub struct IDxcBlob(i32);
pub struct IDxcBlobEncoding(i32);
pub struct IDxcBlobUtf16(i32);
pub struct IDxcBlobUtf8(i32);
pub struct IDxcCompiler(i32);
pub struct IDxcCompiler2(i32);
pub struct IDxcCompiler3(i32);
pub struct IDxcCompilerArgs(i32);
pub struct IDxcContainerBuilder(i32);
pub struct IDxcContainerReflection(i32);
pub struct IDxcExtraOutputs(i32);
pub struct IDxcIncludeHandler(i32);
pub struct IDxcLibrary(i32);
pub struct IDxcLinker(i32);
pub struct IDxcOperationResult(i32);
pub struct IDxcOptimizer(i32);
pub struct IDxcOptimizerPass(i32);
pub struct IDxcPdbUtils(i32);
pub struct IDxcResult(i32);
pub struct IDxcUtils(i32);
pub struct IDxcValidator(i32);
pub struct IDxcValidator2(i32);
pub struct IDxcVersionInfo(i32);
pub struct IDxcVersionInfo2(i32);
pub struct IDxcVersionInfo3(i32);
