#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    pub fn DxcCreateInstance(rclsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn DxcCreateInstance2(pmalloc: super::super::super::System::Com::IMalloc, rclsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
pub const CLSID_DxcAssembler: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3609779048,
    data2: 63747,
    data3: 20352,
    data4: [148, 205, 220, 207, 118, 236, 113, 81],
};
pub const CLSID_DxcCompiler: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1944202643,
    data2: 59086,
    data3: 18419,
    data4: [181, 191, 240, 102, 79, 57, 193, 176],
};
pub const CLSID_DxcCompilerArgs: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1045868162,
    data2: 8781,
    data3: 18191,
    data4: [161, 161, 254, 48, 22, 238, 159, 157],
};
pub const CLSID_DxcContainerBuilder: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2484290196,
    data2: 16671,
    data3: 17780,
    data4: [180, 208, 135, 65, 226, 82, 64, 210],
};
pub const CLSID_DxcContainerReflection: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3119858825,
    data2: 21944,
    data3: 16396,
    data4: [186, 58, 22, 117, 228, 114, 139, 145],
};
pub const CLSID_DxcDiaDataSource: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3441388403,
    data2: 10928,
    data3: 18509,
    data4: [142, 220, 235, 231, 164, 60, 160, 159],
};
pub const CLSID_DxcLibrary: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1648744111,
    data2: 26336,
    data3: 18685,
    data4: [128, 180, 77, 39, 23, 150, 116, 140],
};
pub const CLSID_DxcLinker: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4016734343, data2: 45290, data3: 19798, data4: [158, 69, 208, 126, 26, 139, 120, 6] };
pub const CLSID_DxcOptimizer: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2922174367,
    data2: 52258,
    data3: 17727,
    data4: [155, 107, 177, 36, 231, 165, 32, 76],
};
pub const CLSID_DxcPdbUtils: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1415716347,
    data2: 62158,
    data3: 17790,
    data4: [174, 140, 236, 53, 95, 174, 236, 124],
};
pub const CLSID_DxcValidator: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2359550485,
    data2: 63272,
    data3: 19699,
    data4: [140, 221, 136, 175, 145, 117, 135, 161],
};
pub const DXC_CP_ACP: u32 = 0u32;
pub const DXC_CP_UTF16: u32 = 1200u32;
pub const DXC_CP_UTF8: u32 = 65001u32;
pub const DXC_HASHFLAG_INCLUDES_SOURCE: u32 = 1u32;
pub const DXC_OUT_NONE: i32 = 0i32;
pub const DXC_OUT_OBJECT: i32 = 1i32;
pub const DXC_OUT_ERRORS: i32 = 2i32;
pub const DXC_OUT_PDB: i32 = 3i32;
pub const DXC_OUT_SHADER_HASH: i32 = 4i32;
pub const DXC_OUT_DISASSEMBLY: i32 = 5i32;
pub const DXC_OUT_HLSL: i32 = 6i32;
pub const DXC_OUT_TEXT: i32 = 7i32;
pub const DXC_OUT_REFLECTION: i32 = 8i32;
pub const DXC_OUT_ROOT_SIGNATURE: i32 = 9i32;
pub const DXC_OUT_EXTRA_OUTPUTS: i32 = 10i32;
pub const DXC_OUT_FORCE_DWORD: i32 = -1i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DxcArgPair {
    pub pName: super::super::super::Foundation::PWSTR,
    pub pValue: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DxcArgPair {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DxcArgPair {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DxcBuffer {
    pub Ptr: *mut ::core::ffi::c_void,
    pub Size: usize,
    pub Encoding: u32,
}
impl ::core::marker::Copy for DxcBuffer {}
impl ::core::clone::Clone for DxcBuffer {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
pub type DxcCreateInstance2Proc = unsafe extern "system" fn(pmalloc: super::super::super::System::Com::IMalloc, rclsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type DxcCreateInstanceProc = unsafe extern "system" fn(rclsid: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DxcDefine {
    pub Name: super::super::super::Foundation::PWSTR,
    pub Value: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DxcDefine {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DxcDefine {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DxcShaderHash {
    pub Flags: u32,
    pub HashDigest: [u8; 16],
}
impl ::core::marker::Copy for DxcShaderHash {}
impl ::core::clone::Clone for DxcShaderHash {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DxcValidatorFlags_Default: u32 = 0u32;
pub const DxcValidatorFlags_InPlaceEdit: u32 = 1u32;
pub const DxcValidatorFlags_ModuleOnly: u32 = 4u32;
pub const DxcValidatorFlags_RootSignatureOnly: u32 = 2u32;
pub const DxcValidatorFlags_ValidMask: u32 = 7u32;
pub const DxcVersionInfoFlags_Debug: u32 = 1u32;
pub const DxcVersionInfoFlags_Internal: u32 = 2u32;
pub const DxcVersionInfoFlags_None: u32 = 0u32;
#[repr(transparent)]
pub struct IDxcAssembler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDxcAssembler {}
impl ::core::clone::Clone for IDxcAssembler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDxcBlob(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDxcBlob {}
impl ::core::clone::Clone for IDxcBlob {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDxcBlobEncoding(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDxcBlobEncoding {}
impl ::core::clone::Clone for IDxcBlobEncoding {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDxcBlobUtf16(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDxcBlobUtf16 {}
impl ::core::clone::Clone for IDxcBlobUtf16 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDxcBlobUtf8(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDxcBlobUtf8 {}
impl ::core::clone::Clone for IDxcBlobUtf8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDxcCompiler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDxcCompiler {}
impl ::core::clone::Clone for IDxcCompiler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDxcCompiler2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDxcCompiler2 {}
impl ::core::clone::Clone for IDxcCompiler2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDxcCompiler3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDxcCompiler3 {}
impl ::core::clone::Clone for IDxcCompiler3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDxcCompilerArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDxcCompilerArgs {}
impl ::core::clone::Clone for IDxcCompilerArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDxcContainerBuilder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDxcContainerBuilder {}
impl ::core::clone::Clone for IDxcContainerBuilder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDxcContainerReflection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDxcContainerReflection {}
impl ::core::clone::Clone for IDxcContainerReflection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDxcExtraOutputs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDxcExtraOutputs {}
impl ::core::clone::Clone for IDxcExtraOutputs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDxcIncludeHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDxcIncludeHandler {}
impl ::core::clone::Clone for IDxcIncludeHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDxcLibrary(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDxcLibrary {}
impl ::core::clone::Clone for IDxcLibrary {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDxcLinker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDxcLinker {}
impl ::core::clone::Clone for IDxcLinker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDxcOperationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDxcOperationResult {}
impl ::core::clone::Clone for IDxcOperationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDxcOptimizer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDxcOptimizer {}
impl ::core::clone::Clone for IDxcOptimizer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDxcOptimizerPass(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDxcOptimizerPass {}
impl ::core::clone::Clone for IDxcOptimizerPass {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDxcPdbUtils(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDxcPdbUtils {}
impl ::core::clone::Clone for IDxcPdbUtils {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDxcResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDxcResult {}
impl ::core::clone::Clone for IDxcResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDxcUtils(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDxcUtils {}
impl ::core::clone::Clone for IDxcUtils {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDxcValidator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDxcValidator {}
impl ::core::clone::Clone for IDxcValidator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDxcValidator2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDxcValidator2 {}
impl ::core::clone::Clone for IDxcValidator2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDxcVersionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDxcVersionInfo {}
impl ::core::clone::Clone for IDxcVersionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDxcVersionInfo2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDxcVersionInfo2 {}
impl ::core::clone::Clone for IDxcVersionInfo2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDxcVersionInfo3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDxcVersionInfo3 {}
impl ::core::clone::Clone for IDxcVersionInfo3 {
    fn clone(&self) -> Self {
        *self
    }
}
