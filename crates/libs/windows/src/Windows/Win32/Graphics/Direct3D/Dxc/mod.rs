#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub const CLSID_DxcAssembler: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd728db68_f903_4f80_94cd_dccf76ec7151);
pub const CLSID_DxcCompiler: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73e22d93_e6ce_47f3_b5bf_f0664f39c1b0);
pub const CLSID_DxcCompilerArgs: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e56ae82_224d_470f_a1a1_fe3016ee9f9d);
pub const CLSID_DxcContainerBuilder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94134294_411f_4574_b4d0_8741e25240d2);
pub const CLSID_DxcContainerReflection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9f54489_55b8_400c_ba3a_1675e4728b91);
pub const CLSID_DxcDiaDataSource: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd1f6b73_2ab0_484d_8edc_ebe7a43ca09f);
pub const CLSID_DxcLibrary: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6245d6af_66e0_48fd_80b4_4d271796748c);
pub const CLSID_DxcLinker: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef6a8087_b0ea_4d56_9e45_d07e1a8b7806);
pub const CLSID_DxcOptimizer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae2cd79f_cc22_453f_9b6b_b124e7a5204c);
pub const CLSID_DxcPdbUtils: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54621dfb_f2ce_457e_ae8c_ec355faeec7c);
pub const CLSID_DxcValidator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ca3e215_f728_4cf3_8cdd_88af917587a1);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_ALL_RESOURCES_BOUND: &'static str = "-all_resources_bound";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_AVOID_FLOW_CONTROL: &'static str = "-Gfa";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_DEBUG: &'static str = "-Zi";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_DEBUG_NAME_FOR_BINARY: &'static str = "-Zsb";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_DEBUG_NAME_FOR_SOURCE: &'static str = "-Zss";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_ENABLE_BACKWARDS_COMPATIBILITY: &'static str = "-Gec";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_ENABLE_STRICTNESS: &'static str = "-Ges";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_IEEE_STRICTNESS: &'static str = "-Gis";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_OPTIMIZATION_LEVEL0: &'static str = "-O0";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_OPTIMIZATION_LEVEL1: &'static str = "-O1";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_OPTIMIZATION_LEVEL2: &'static str = "-O2";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_OPTIMIZATION_LEVEL3: &'static str = "-O3";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_PACK_MATRIX_COLUMN_MAJOR: &'static str = "-Zpc";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_PACK_MATRIX_ROW_MAJOR: &'static str = "-Zpr";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_PREFER_FLOW_CONTROL: &'static str = "-Gfp";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_RESOURCES_MAY_ALIAS: &'static str = "-res_may_alias";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_SKIP_OPTIMIZATIONS: &'static str = "-Od";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_SKIP_VALIDATION: &'static str = "-Vd";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_ARG_WARNINGS_ARE_ERRORS: &'static str = "-WX";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXC_CP(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_CP_ACP: DXC_CP = DXC_CP(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_CP_UTF16: DXC_CP = DXC_CP(1200u32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_CP_UTF8: DXC_CP = DXC_CP(65001u32);
impl ::core::marker::Copy for DXC_CP {}
impl ::core::clone::Clone for DXC_CP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXC_CP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXC_CP {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXC_CP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXC_CP").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_EXTRA_OUTPUT_NAME_STDERR: &'static str = "*stderr*";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_EXTRA_OUTPUT_NAME_STDOUT: &'static str = "*stdout*";
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_HASHFLAG_INCLUDES_SOURCE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DXC_OUT_KIND(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_NONE: DXC_OUT_KIND = DXC_OUT_KIND(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_OBJECT: DXC_OUT_KIND = DXC_OUT_KIND(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_ERRORS: DXC_OUT_KIND = DXC_OUT_KIND(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_PDB: DXC_OUT_KIND = DXC_OUT_KIND(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_SHADER_HASH: DXC_OUT_KIND = DXC_OUT_KIND(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_DISASSEMBLY: DXC_OUT_KIND = DXC_OUT_KIND(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_HLSL: DXC_OUT_KIND = DXC_OUT_KIND(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_TEXT: DXC_OUT_KIND = DXC_OUT_KIND(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_REFLECTION: DXC_OUT_KIND = DXC_OUT_KIND(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_ROOT_SIGNATURE: DXC_OUT_KIND = DXC_OUT_KIND(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_EXTRA_OUTPUTS: DXC_OUT_KIND = DXC_OUT_KIND(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DXC_OUT_FORCE_DWORD: DXC_OUT_KIND = DXC_OUT_KIND(-1i32);
impl ::core::marker::Copy for DXC_OUT_KIND {}
impl ::core::clone::Clone for DXC_OUT_KIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DXC_OUT_KIND {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DXC_OUT_KIND {
    type Abi = Self;
}
impl ::core::fmt::Debug for DXC_OUT_KIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXC_OUT_KIND").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub struct DxcArgPair {
    pub pName: ::windows::core::PCWSTR,
    pub pValue: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for DxcArgPair {}
impl ::core::clone::Clone for DxcArgPair {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DxcArgPair {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DxcArgPair").field("pName", &self.pName).field("pValue", &self.pValue).finish()
    }
}
unsafe impl ::windows::core::Abi for DxcArgPair {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DxcArgPair {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DxcArgPair>()) == 0 }
    }
}
impl ::core::cmp::Eq for DxcArgPair {}
impl ::core::default::Default for DxcArgPair {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub struct DxcBuffer {
    pub Ptr: *const ::core::ffi::c_void,
    pub Size: usize,
    pub Encoding: u32,
}
impl ::core::marker::Copy for DxcBuffer {}
impl ::core::clone::Clone for DxcBuffer {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DxcBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DxcBuffer").field("Ptr", &self.Ptr).field("Size", &self.Size).field("Encoding", &self.Encoding).finish()
    }
}
unsafe impl ::windows::core::Abi for DxcBuffer {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DxcBuffer {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DxcBuffer>()) == 0 }
    }
}
impl ::core::cmp::Eq for DxcBuffer {}
impl ::core::default::Default for DxcBuffer {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[inline]
pub unsafe fn DxcCreateInstance(rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DxcCreateInstance(rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DxcCreateInstance(::core::mem::transmute(rclsid), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn DxcCreateInstance2<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IMalloc>>(pmalloc: Param0, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DxcCreateInstance2(pmalloc: ::windows::core::RawPtr, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DxcCreateInstance2(pmalloc.into_param().abi(), ::core::mem::transmute(rclsid), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub type DxcCreateInstance2Proc = ::core::option::Option<unsafe extern "system" fn(pmalloc: ::core::option::Option<super::super::super::System::Com::IMalloc>, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub type DxcCreateInstanceProc = ::core::option::Option<unsafe extern "system" fn(rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub struct DxcDefine {
    pub Name: ::windows::core::PCWSTR,
    pub Value: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for DxcDefine {}
impl ::core::clone::Clone for DxcDefine {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DxcDefine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DxcDefine").field("Name", &self.Name).field("Value", &self.Value).finish()
    }
}
unsafe impl ::windows::core::Abi for DxcDefine {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DxcDefine {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DxcDefine>()) == 0 }
    }
}
impl ::core::cmp::Eq for DxcDefine {}
impl ::core::default::Default for DxcDefine {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
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
impl ::core::fmt::Debug for DxcShaderHash {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DxcShaderHash").field("Flags", &self.Flags).field("HashDigest", &self.HashDigest).finish()
    }
}
unsafe impl ::windows::core::Abi for DxcShaderHash {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DxcShaderHash {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DxcShaderHash>()) == 0 }
    }
}
impl ::core::cmp::Eq for DxcShaderHash {}
impl ::core::default::Default for DxcShaderHash {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DxcValidatorFlags_Default: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DxcValidatorFlags_InPlaceEdit: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DxcValidatorFlags_ModuleOnly: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DxcValidatorFlags_RootSignatureOnly: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DxcValidatorFlags_ValidMask: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DxcVersionInfoFlags_Debug: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DxcVersionInfoFlags_Internal: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
pub const DxcVersionInfoFlags_None: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcAssembler(::windows::core::IUnknown);
impl IDxcAssembler {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn AssembleToContainer<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, pshader: Param0) -> ::windows::core::Result<IDxcOperationResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AssembleToContainer)(::core::mem::transmute_copy(self), pshader.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDxcOperationResult>(result__)
    }
}
impl ::core::convert::From<IDxcAssembler> for ::windows::core::IUnknown {
    fn from(value: IDxcAssembler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcAssembler> for ::windows::core::IUnknown {
    fn from(value: &IDxcAssembler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcAssembler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcAssembler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcAssembler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcAssembler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcAssembler {}
impl ::core::fmt::Debug for IDxcAssembler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcAssembler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDxcAssembler {
    type Vtable = IDxcAssembler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x091f7a26_1c1f_4948_904b_e6e3a8a771d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcAssembler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub AssembleToContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshader: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcBlob(::windows::core::IUnknown);
impl IDxcBlob {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetBufferPointer(&self) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetBufferPointer)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetBufferSize(&self) -> usize {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetBufferSize)(::core::mem::transmute_copy(self)))
    }
}
impl ::core::convert::From<IDxcBlob> for ::windows::core::IUnknown {
    fn from(value: IDxcBlob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcBlob> for ::windows::core::IUnknown {
    fn from(value: &IDxcBlob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcBlob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcBlob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcBlob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcBlob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcBlob {}
impl ::core::fmt::Debug for IDxcBlob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcBlob").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDxcBlob {
    type Vtable = IDxcBlob_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ba5fb08_5195_40e2_ac58_0d989c3a0102);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlob_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetBufferPointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
    pub GetBufferSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcBlobEncoding(::windows::core::IUnknown);
impl IDxcBlobEncoding {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetBufferPointer(&self) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetBufferPointer)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetBufferSize(&self) -> usize {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetBufferSize)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEncoding(&self, pknown: *mut super::super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetEncoding)(::core::mem::transmute_copy(self), ::core::mem::transmute(pknown), ::core::mem::transmute(pcodepage)).ok()
    }
}
impl ::core::convert::From<IDxcBlobEncoding> for ::windows::core::IUnknown {
    fn from(value: IDxcBlobEncoding) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcBlobEncoding> for ::windows::core::IUnknown {
    fn from(value: &IDxcBlobEncoding) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcBlobEncoding {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcBlobEncoding {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDxcBlobEncoding> for IDxcBlob {
    fn from(value: IDxcBlobEncoding) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcBlobEncoding> for IDxcBlob {
    fn from(value: &IDxcBlobEncoding) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDxcBlob> for IDxcBlobEncoding {
    fn into_param(self) -> ::windows::core::Param<'a, IDxcBlob> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDxcBlob> for &'a IDxcBlobEncoding {
    fn into_param(self) -> ::windows::core::Param<'a, IDxcBlob> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcBlobEncoding {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcBlobEncoding {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcBlobEncoding {}
impl ::core::fmt::Debug for IDxcBlobEncoding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcBlobEncoding").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDxcBlobEncoding {
    type Vtable = IDxcBlobEncoding_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7241d424_2646_4191_97c0_98e96e42fc68);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlobEncoding_Vtbl {
    pub base: IDxcBlob_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pknown: *mut super::super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEncoding: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcBlobUtf16(::windows::core::IUnknown);
impl IDxcBlobUtf16 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetBufferPointer(&self) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.base.GetBufferPointer)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetBufferSize(&self) -> usize {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.base.GetBufferSize)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEncoding(&self, pknown: *mut super::super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetEncoding)(::core::mem::transmute_copy(self), ::core::mem::transmute(pknown), ::core::mem::transmute(pcodepage)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetStringPointer(&self) -> ::windows::core::PWSTR {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetStringPointer)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetStringLength(&self) -> usize {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetStringLength)(::core::mem::transmute_copy(self)))
    }
}
impl ::core::convert::From<IDxcBlobUtf16> for ::windows::core::IUnknown {
    fn from(value: IDxcBlobUtf16) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcBlobUtf16> for ::windows::core::IUnknown {
    fn from(value: &IDxcBlobUtf16) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcBlobUtf16 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcBlobUtf16 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDxcBlobUtf16> for IDxcBlob {
    fn from(value: IDxcBlobUtf16) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcBlobUtf16> for IDxcBlob {
    fn from(value: &IDxcBlobUtf16) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDxcBlob> for IDxcBlobUtf16 {
    fn into_param(self) -> ::windows::core::Param<'a, IDxcBlob> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDxcBlob> for &'a IDxcBlobUtf16 {
    fn into_param(self) -> ::windows::core::Param<'a, IDxcBlob> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDxcBlobUtf16> for IDxcBlobEncoding {
    fn from(value: IDxcBlobUtf16) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcBlobUtf16> for IDxcBlobEncoding {
    fn from(value: &IDxcBlobUtf16) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDxcBlobEncoding> for IDxcBlobUtf16 {
    fn into_param(self) -> ::windows::core::Param<'a, IDxcBlobEncoding> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDxcBlobEncoding> for &'a IDxcBlobUtf16 {
    fn into_param(self) -> ::windows::core::Param<'a, IDxcBlobEncoding> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcBlobUtf16 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcBlobUtf16 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcBlobUtf16 {}
impl ::core::fmt::Debug for IDxcBlobUtf16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcBlobUtf16").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDxcBlobUtf16 {
    type Vtable = IDxcBlobUtf16_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3f84eab_0faa_497e_a39c_ee6ed60b2d84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlobUtf16_Vtbl {
    pub base: IDxcBlobEncoding_Vtbl,
    pub GetStringPointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::PWSTR,
    pub GetStringLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcBlobUtf8(::windows::core::IUnknown);
impl IDxcBlobUtf8 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetBufferPointer(&self) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.base.GetBufferPointer)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetBufferSize(&self) -> usize {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.base.GetBufferSize)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEncoding(&self, pknown: *mut super::super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetEncoding)(::core::mem::transmute_copy(self), ::core::mem::transmute(pknown), ::core::mem::transmute(pcodepage)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetStringPointer(&self) -> ::windows::core::PSTR {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetStringPointer)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetStringLength(&self) -> usize {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetStringLength)(::core::mem::transmute_copy(self)))
    }
}
impl ::core::convert::From<IDxcBlobUtf8> for ::windows::core::IUnknown {
    fn from(value: IDxcBlobUtf8) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcBlobUtf8> for ::windows::core::IUnknown {
    fn from(value: &IDxcBlobUtf8) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcBlobUtf8 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcBlobUtf8 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDxcBlobUtf8> for IDxcBlob {
    fn from(value: IDxcBlobUtf8) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcBlobUtf8> for IDxcBlob {
    fn from(value: &IDxcBlobUtf8) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDxcBlob> for IDxcBlobUtf8 {
    fn into_param(self) -> ::windows::core::Param<'a, IDxcBlob> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDxcBlob> for &'a IDxcBlobUtf8 {
    fn into_param(self) -> ::windows::core::Param<'a, IDxcBlob> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDxcBlobUtf8> for IDxcBlobEncoding {
    fn from(value: IDxcBlobUtf8) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcBlobUtf8> for IDxcBlobEncoding {
    fn from(value: &IDxcBlobUtf8) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDxcBlobEncoding> for IDxcBlobUtf8 {
    fn into_param(self) -> ::windows::core::Param<'a, IDxcBlobEncoding> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDxcBlobEncoding> for &'a IDxcBlobUtf8 {
    fn into_param(self) -> ::windows::core::Param<'a, IDxcBlobEncoding> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcBlobUtf8 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcBlobUtf8 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcBlobUtf8 {}
impl ::core::fmt::Debug for IDxcBlobUtf8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcBlobUtf8").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDxcBlobUtf8 {
    type Vtable = IDxcBlobUtf8_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3da636c9_ba71_4024_a301_30cbf125305b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlobUtf8_Vtbl {
    pub base: IDxcBlobEncoding_Vtbl,
    pub GetStringPointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::PSTR,
    pub GetStringLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcCompiler(::windows::core::IUnknown);
impl IDxcCompiler {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn Compile<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param8: ::windows::core::IntoParam<'a, IDxcIncludeHandler>>(&self, psource: Param0, psourcename: Param1, pentrypoint: Param2, ptargetprofile: Param3, parguments: &[::windows::core::PWSTR], pdefines: &[DxcDefine], pincludehandler: Param8) -> ::windows::core::Result<IDxcOperationResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Compile)(::core::mem::transmute_copy(self), psource.into_param().abi(), psourcename.into_param().abi(), pentrypoint.into_param().abi(), ptargetprofile.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(parguments)), parguments.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pdefines)), pdefines.len() as _, pincludehandler.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDxcOperationResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn Preprocess<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param6: ::windows::core::IntoParam<'a, IDxcIncludeHandler>>(&self, psource: Param0, psourcename: Param1, parguments: &[::windows::core::PWSTR], pdefines: &[DxcDefine], pincludehandler: Param6) -> ::windows::core::Result<IDxcOperationResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Preprocess)(::core::mem::transmute_copy(self), psource.into_param().abi(), psourcename.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(parguments)), parguments.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pdefines)), pdefines.len() as _, pincludehandler.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDxcOperationResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn Disassemble<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, psource: Param0) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Disassemble)(::core::mem::transmute_copy(self), psource.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDxcBlobEncoding>(result__)
    }
}
impl ::core::convert::From<IDxcCompiler> for ::windows::core::IUnknown {
    fn from(value: IDxcCompiler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcCompiler> for ::windows::core::IUnknown {
    fn from(value: &IDxcCompiler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcCompiler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcCompiler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcCompiler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcCompiler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcCompiler {}
impl ::core::fmt::Debug for IDxcCompiler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcCompiler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDxcCompiler {
    type Vtable = IDxcCompiler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c210bf3_011f_4422_8d70_6f9acb8db617);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompiler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Compile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psource: ::windows::core::RawPtr, psourcename: ::windows::core::PCWSTR, pentrypoint: ::windows::core::PCWSTR, ptargetprofile: ::windows::core::PCWSTR, parguments: *const ::windows::core::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Preprocess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psource: ::windows::core::RawPtr, psourcename: ::windows::core::PCWSTR, parguments: *const ::windows::core::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Disassemble: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psource: ::windows::core::RawPtr, ppdisassembly: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcCompiler2(::windows::core::IUnknown);
impl IDxcCompiler2 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn Compile<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param8: ::windows::core::IntoParam<'a, IDxcIncludeHandler>>(&self, psource: Param0, psourcename: Param1, pentrypoint: Param2, ptargetprofile: Param3, parguments: &[::windows::core::PWSTR], pdefines: &[DxcDefine], pincludehandler: Param8) -> ::windows::core::Result<IDxcOperationResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Compile)(::core::mem::transmute_copy(self), psource.into_param().abi(), psourcename.into_param().abi(), pentrypoint.into_param().abi(), ptargetprofile.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(parguments)), parguments.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pdefines)), pdefines.len() as _, pincludehandler.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDxcOperationResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn Preprocess<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param6: ::windows::core::IntoParam<'a, IDxcIncludeHandler>>(&self, psource: Param0, psourcename: Param1, parguments: &[::windows::core::PWSTR], pdefines: &[DxcDefine], pincludehandler: Param6) -> ::windows::core::Result<IDxcOperationResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Preprocess)(::core::mem::transmute_copy(self), psource.into_param().abi(), psourcename.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(parguments)), parguments.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pdefines)), pdefines.len() as _, pincludehandler.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDxcOperationResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn Disassemble<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, psource: Param0) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Disassemble)(::core::mem::transmute_copy(self), psource.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn CompileWithDebug<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param8: ::windows::core::IntoParam<'a, IDxcIncludeHandler>>(&self, psource: Param0, psourcename: Param1, pentrypoint: Param2, ptargetprofile: Param3, parguments: &[::windows::core::PWSTR], pdefines: &[DxcDefine], pincludehandler: Param8, ppresult: *mut ::core::option::Option<IDxcOperationResult>, ppdebugblobname: *mut ::windows::core::PWSTR, ppdebugblob: *mut ::core::option::Option<IDxcBlob>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CompileWithDebug)(
            ::core::mem::transmute_copy(self),
            psource.into_param().abi(),
            psourcename.into_param().abi(),
            pentrypoint.into_param().abi(),
            ptargetprofile.into_param().abi(),
            ::core::mem::transmute(::windows::core::as_ptr_or_null(parguments)),
            parguments.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pdefines)),
            pdefines.len() as _,
            pincludehandler.into_param().abi(),
            ::core::mem::transmute(ppresult),
            ::core::mem::transmute(ppdebugblobname),
            ::core::mem::transmute(ppdebugblob),
        )
        .ok()
    }
}
impl ::core::convert::From<IDxcCompiler2> for ::windows::core::IUnknown {
    fn from(value: IDxcCompiler2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcCompiler2> for ::windows::core::IUnknown {
    fn from(value: &IDxcCompiler2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcCompiler2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcCompiler2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDxcCompiler2> for IDxcCompiler {
    fn from(value: IDxcCompiler2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcCompiler2> for IDxcCompiler {
    fn from(value: &IDxcCompiler2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDxcCompiler> for IDxcCompiler2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDxcCompiler> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDxcCompiler> for &'a IDxcCompiler2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDxcCompiler> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcCompiler2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcCompiler2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcCompiler2 {}
impl ::core::fmt::Debug for IDxcCompiler2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcCompiler2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDxcCompiler2 {
    type Vtable = IDxcCompiler2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa005a9d9_b8bb_4594_b5c9_0e633bec4d37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompiler2_Vtbl {
    pub base: IDxcCompiler_Vtbl,
    pub CompileWithDebug: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psource: ::windows::core::RawPtr, psourcename: ::windows::core::PCWSTR, pentrypoint: ::windows::core::PCWSTR, ptargetprofile: ::windows::core::PCWSTR, parguments: *const ::windows::core::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr, ppdebugblobname: *mut ::windows::core::PWSTR, ppdebugblob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcCompiler3(::windows::core::IUnknown);
impl IDxcCompiler3 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn Compile<'a, Param3: ::windows::core::IntoParam<'a, IDxcIncludeHandler>>(&self, psource: *const DxcBuffer, parguments: &[::windows::core::PWSTR], pincludehandler: Param3, riid: *const ::windows::core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Compile)(::core::mem::transmute_copy(self), ::core::mem::transmute(psource), ::core::mem::transmute(::windows::core::as_ptr_or_null(parguments)), parguments.len() as _, pincludehandler.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppresult)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn Disassemble(&self, pobject: *const DxcBuffer, riid: *const ::windows::core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Disassemble)(::core::mem::transmute_copy(self), ::core::mem::transmute(pobject), ::core::mem::transmute(riid), ::core::mem::transmute(ppresult)).ok()
    }
}
impl ::core::convert::From<IDxcCompiler3> for ::windows::core::IUnknown {
    fn from(value: IDxcCompiler3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcCompiler3> for ::windows::core::IUnknown {
    fn from(value: &IDxcCompiler3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcCompiler3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcCompiler3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcCompiler3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcCompiler3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcCompiler3 {}
impl ::core::fmt::Debug for IDxcCompiler3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcCompiler3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDxcCompiler3 {
    type Vtable = IDxcCompiler3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x228b4687_5a6a_4730_900c_9702b2203f54);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompiler3_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Compile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psource: *const DxcBuffer, parguments: *const ::windows::core::PWSTR, argcount: u32, pincludehandler: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Disassemble: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobject: *const DxcBuffer, riid: *const ::windows::core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcCompilerArgs(::windows::core::IUnknown);
impl IDxcCompilerArgs {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetArguments(&self) -> *mut ::windows::core::PWSTR {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetArguments)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetCount)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn AddArguments(&self, parguments: &[::windows::core::PWSTR]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddArguments)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(parguments)), parguments.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn AddArgumentsUTF8(&self, parguments: &[::windows::core::PSTR]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddArgumentsUTF8)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(parguments)), parguments.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn AddDefines(&self, pdefines: &[DxcDefine]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddDefines)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pdefines)), pdefines.len() as _).ok()
    }
}
impl ::core::convert::From<IDxcCompilerArgs> for ::windows::core::IUnknown {
    fn from(value: IDxcCompilerArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcCompilerArgs> for ::windows::core::IUnknown {
    fn from(value: &IDxcCompilerArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcCompilerArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcCompilerArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcCompilerArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcCompilerArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcCompilerArgs {}
impl ::core::fmt::Debug for IDxcCompilerArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcCompilerArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDxcCompilerArgs {
    type Vtable = IDxcCompilerArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73effe2a_70dc_45f8_9690_eff64c02429d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompilerArgs_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> *mut ::windows::core::PWSTR,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub AddArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parguments: *const ::windows::core::PWSTR, argcount: u32) -> ::windows::core::HRESULT,
    pub AddArgumentsUTF8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parguments: *const ::windows::core::PSTR, argcount: u32) -> ::windows::core::HRESULT,
    pub AddDefines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdefines: *const DxcDefine, definecount: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcContainerBuilder(::windows::core::IUnknown);
impl IDxcContainerBuilder {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn Load<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, pdxilcontainerheader: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Load)(::core::mem::transmute_copy(self), pdxilcontainerheader.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn AddPart<'a, Param1: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, fourcc: u32, psource: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddPart)(::core::mem::transmute_copy(self), ::core::mem::transmute(fourcc), psource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn RemovePart(&self, fourcc: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemovePart)(::core::mem::transmute_copy(self), ::core::mem::transmute(fourcc)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn SerializeContainer(&self) -> ::windows::core::Result<IDxcOperationResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SerializeContainer)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDxcOperationResult>(result__)
    }
}
impl ::core::convert::From<IDxcContainerBuilder> for ::windows::core::IUnknown {
    fn from(value: IDxcContainerBuilder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcContainerBuilder> for ::windows::core::IUnknown {
    fn from(value: &IDxcContainerBuilder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcContainerBuilder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcContainerBuilder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcContainerBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcContainerBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcContainerBuilder {}
impl ::core::fmt::Debug for IDxcContainerBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcContainerBuilder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDxcContainerBuilder {
    type Vtable = IDxcContainerBuilder_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x334b1f50_2292_4b35_99a1_25588d8c17fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcContainerBuilder_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdxilcontainerheader: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AddPart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fourcc: u32, psource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemovePart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fourcc: u32) -> ::windows::core::HRESULT,
    pub SerializeContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcContainerReflection(::windows::core::IUnknown);
impl IDxcContainerReflection {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn Load<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, pcontainer: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Load)(::core::mem::transmute_copy(self), pcontainer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetPartCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPartCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetPartKind(&self, idx: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPartKind)(::core::mem::transmute_copy(self), ::core::mem::transmute(idx), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetPartContent(&self, idx: u32) -> ::windows::core::Result<IDxcBlob> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPartContent)(::core::mem::transmute_copy(self), ::core::mem::transmute(idx), ::core::mem::transmute(&mut result__)).from_abi::<IDxcBlob>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn FindFirstPartKind(&self, kind: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FindFirstPartKind)(::core::mem::transmute_copy(self), ::core::mem::transmute(kind), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetPartReflection(&self, idx: u32, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPartReflection)(::core::mem::transmute_copy(self), ::core::mem::transmute(idx), ::core::mem::transmute(iid), ::core::mem::transmute(ppvobject)).ok()
    }
}
impl ::core::convert::From<IDxcContainerReflection> for ::windows::core::IUnknown {
    fn from(value: IDxcContainerReflection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcContainerReflection> for ::windows::core::IUnknown {
    fn from(value: &IDxcContainerReflection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcContainerReflection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcContainerReflection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcContainerReflection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcContainerReflection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcContainerReflection {}
impl ::core::fmt::Debug for IDxcContainerReflection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcContainerReflection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDxcContainerReflection {
    type Vtable = IDxcContainerReflection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2c21b26_8350_4bdc_976a_331ce6f4c54c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcContainerReflection_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontainer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetPartCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presult: *mut u32) -> ::windows::core::HRESULT,
    pub GetPartKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idx: u32, presult: *mut u32) -> ::windows::core::HRESULT,
    pub GetPartContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idx: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub FindFirstPartKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: u32, presult: *mut u32) -> ::windows::core::HRESULT,
    pub GetPartReflection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idx: u32, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcExtraOutputs(::windows::core::IUnknown);
impl IDxcExtraOutputs {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetOutputCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetOutputCount)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetOutput(&self, uindex: u32, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputtype: *mut ::core::option::Option<IDxcBlobUtf16>, ppoutputname: *mut ::core::option::Option<IDxcBlobUtf16>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex), ::core::mem::transmute(iid), ::core::mem::transmute(ppvobject), ::core::mem::transmute(ppoutputtype), ::core::mem::transmute(ppoutputname)).ok()
    }
}
impl ::core::convert::From<IDxcExtraOutputs> for ::windows::core::IUnknown {
    fn from(value: IDxcExtraOutputs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcExtraOutputs> for ::windows::core::IUnknown {
    fn from(value: &IDxcExtraOutputs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcExtraOutputs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcExtraOutputs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcExtraOutputs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcExtraOutputs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcExtraOutputs {}
impl ::core::fmt::Debug for IDxcExtraOutputs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcExtraOutputs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDxcExtraOutputs {
    type Vtable = IDxcExtraOutputs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x319b37a2_a5c2_494a_a5de_4801b2faf989);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcExtraOutputs_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetOutputCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputtype: *mut ::windows::core::RawPtr, ppoutputname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcIncludeHandler(::windows::core::IUnknown);
impl IDxcIncludeHandler {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn LoadSource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pfilename: Param0) -> ::windows::core::Result<IDxcBlob> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LoadSource)(::core::mem::transmute_copy(self), pfilename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDxcBlob>(result__)
    }
}
impl ::core::convert::From<IDxcIncludeHandler> for ::windows::core::IUnknown {
    fn from(value: IDxcIncludeHandler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcIncludeHandler> for ::windows::core::IUnknown {
    fn from(value: &IDxcIncludeHandler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcIncludeHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcIncludeHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcIncludeHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcIncludeHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcIncludeHandler {}
impl ::core::fmt::Debug for IDxcIncludeHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcIncludeHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDxcIncludeHandler {
    type Vtable = IDxcIncludeHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f61fc7d_950d_467f_b3e3_3c02fb49187c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcIncludeHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub LoadSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilename: ::windows::core::PCWSTR, ppincludesource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcLibrary(::windows::core::IUnknown);
impl IDxcLibrary {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetMalloc<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IMalloc>>(&self, pmalloc: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMalloc)(::core::mem::transmute_copy(self), pmalloc.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn CreateBlobFromBlob<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0, offset: u32, length: u32) -> ::windows::core::Result<IDxcBlob> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateBlobFromBlob)(::core::mem::transmute_copy(self), pblob.into_param().abi(), ::core::mem::transmute(offset), ::core::mem::transmute(length), ::core::mem::transmute(&mut result__)).from_abi::<IDxcBlob>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn CreateBlobFromFile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pfilename: Param0, codepage: *const DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateBlobFromFile)(::core::mem::transmute_copy(self), pfilename.into_param().abi(), ::core::mem::transmute(codepage), ::core::mem::transmute(&mut result__)).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn CreateBlobWithEncodingFromPinned(&self, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateBlobWithEncodingFromPinned)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptext), ::core::mem::transmute(size), ::core::mem::transmute(codepage), ::core::mem::transmute(&mut result__)).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn CreateBlobWithEncodingOnHeapCopy(&self, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateBlobWithEncodingOnHeapCopy)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptext), ::core::mem::transmute(size), ::core::mem::transmute(codepage), ::core::mem::transmute(&mut result__)).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateBlobWithEncodingOnMalloc<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IMalloc>>(&self, ptext: *const ::core::ffi::c_void, pimalloc: Param1, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateBlobWithEncodingOnMalloc)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptext), pimalloc.into_param().abi(), ::core::mem::transmute(size), ::core::mem::transmute(codepage), ::core::mem::transmute(&mut result__)).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn CreateIncludeHandler(&self) -> ::windows::core::Result<IDxcIncludeHandler> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateIncludeHandler)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDxcIncludeHandler>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateStreamFromBlobReadOnly<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0) -> ::windows::core::Result<super::super::super::System::Com::IStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateStreamFromBlobReadOnly)(::core::mem::transmute_copy(self), pblob.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetBlobAsUtf8<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetBlobAsUtf8)(::core::mem::transmute_copy(self), pblob.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetBlobAsUtf16<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetBlobAsUtf16)(::core::mem::transmute_copy(self), pblob.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDxcBlobEncoding>(result__)
    }
}
impl ::core::convert::From<IDxcLibrary> for ::windows::core::IUnknown {
    fn from(value: IDxcLibrary) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcLibrary> for ::windows::core::IUnknown {
    fn from(value: &IDxcLibrary) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcLibrary {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcLibrary {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcLibrary {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcLibrary {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcLibrary {}
impl ::core::fmt::Debug for IDxcLibrary {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcLibrary").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDxcLibrary {
    type Vtable = IDxcLibrary_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5204dc7_d18c_4c3c_bdfb_851673980fe7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcLibrary_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub SetMalloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmalloc: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetMalloc: usize,
    pub CreateBlobFromBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, offset: u32, length: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateBlobFromFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilename: ::windows::core::PCWSTR, codepage: *const DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateBlobWithEncodingFromPinned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateBlobWithEncodingOnHeapCopy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateBlobWithEncodingOnMalloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptext: *const ::core::ffi::c_void, pimalloc: ::windows::core::RawPtr, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateBlobWithEncodingOnMalloc: usize,
    pub CreateIncludeHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateStreamFromBlobReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateStreamFromBlobReadOnly: usize,
    pub GetBlobAsUtf8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetBlobAsUtf16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcLinker(::windows::core::IUnknown);
impl IDxcLinker {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn RegisterLibrary<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, plibname: Param0, plib: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterLibrary)(::core::mem::transmute_copy(self), plibname.into_param().abi(), plib.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn Link<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pentryname: Param0, ptargetprofile: Param1, plibnames: &[::windows::core::PWSTR], parguments: &[::windows::core::PWSTR]) -> ::windows::core::Result<IDxcOperationResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Link)(::core::mem::transmute_copy(self), pentryname.into_param().abi(), ptargetprofile.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(plibnames)), plibnames.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(parguments)), parguments.len() as _, ::core::mem::transmute(&mut result__)).from_abi::<IDxcOperationResult>(result__)
    }
}
impl ::core::convert::From<IDxcLinker> for ::windows::core::IUnknown {
    fn from(value: IDxcLinker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcLinker> for ::windows::core::IUnknown {
    fn from(value: &IDxcLinker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcLinker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcLinker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcLinker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcLinker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcLinker {}
impl ::core::fmt::Debug for IDxcLinker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcLinker").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDxcLinker {
    type Vtable = IDxcLinker_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1b5be2a_62dd_4327_a1c2_42ac1e1e78e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcLinker_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub RegisterLibrary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plibname: ::windows::core::PCWSTR, plib: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Link: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pentryname: ::windows::core::PCWSTR, ptargetprofile: ::windows::core::PCWSTR, plibnames: *const ::windows::core::PWSTR, libcount: u32, parguments: *const ::windows::core::PWSTR, argcount: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcOperationResult(::windows::core::IUnknown);
impl IDxcOperationResult {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::HRESULT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetResult(&self) -> ::windows::core::Result<IDxcBlob> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetResult)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDxcBlob>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetErrorBuffer(&self) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetErrorBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDxcBlobEncoding>(result__)
    }
}
impl ::core::convert::From<IDxcOperationResult> for ::windows::core::IUnknown {
    fn from(value: IDxcOperationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcOperationResult> for ::windows::core::IUnknown {
    fn from(value: &IDxcOperationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcOperationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcOperationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcOperationResult {}
impl ::core::fmt::Debug for IDxcOperationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcOperationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDxcOperationResult {
    type Vtable = IDxcOperationResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcedb484a_d4e9_445a_b991_ca21ca157dc2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcOperationResult_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub GetResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetErrorBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pperrors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcOptimizer(::windows::core::IUnknown);
impl IDxcOptimizer {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetAvailablePassCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAvailablePassCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetAvailablePass(&self, index: u32) -> ::windows::core::Result<IDxcOptimizerPass> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAvailablePass)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<IDxcOptimizerPass>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn RunOptimizer<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0, ppoptions: &[::windows::core::PWSTR], poutputmodule: *mut ::core::option::Option<IDxcBlob>, ppoutputtext: *mut ::core::option::Option<IDxcBlobEncoding>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RunOptimizer)(::core::mem::transmute_copy(self), pblob.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(ppoptions)), ppoptions.len() as _, ::core::mem::transmute(poutputmodule), ::core::mem::transmute(ppoutputtext)).ok()
    }
}
impl ::core::convert::From<IDxcOptimizer> for ::windows::core::IUnknown {
    fn from(value: IDxcOptimizer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcOptimizer> for ::windows::core::IUnknown {
    fn from(value: &IDxcOptimizer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcOptimizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcOptimizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcOptimizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcOptimizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcOptimizer {}
impl ::core::fmt::Debug for IDxcOptimizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcOptimizer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDxcOptimizer {
    type Vtable = IDxcOptimizer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25740e2e_9cba_401b_9119_4fb42f39f270);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcOptimizer_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetAvailablePassCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetAvailablePass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RunOptimizer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, ppoptions: *const ::windows::core::PWSTR, optioncount: u32, poutputmodule: *mut ::windows::core::RawPtr, ppoutputtext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcOptimizerPass(::windows::core::IUnknown);
impl IDxcOptimizerPass {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetOptionName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOptionName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDescription)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetOptionArgCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOptionArgCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetOptionArgName(&self, argindex: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOptionArgName)(::core::mem::transmute_copy(self), ::core::mem::transmute(argindex), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetOptionArgDescription(&self, argindex: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOptionArgDescription)(::core::mem::transmute_copy(self), ::core::mem::transmute(argindex), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IDxcOptimizerPass> for ::windows::core::IUnknown {
    fn from(value: IDxcOptimizerPass) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcOptimizerPass> for ::windows::core::IUnknown {
    fn from(value: &IDxcOptimizerPass) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcOptimizerPass {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcOptimizerPass {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcOptimizerPass {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcOptimizerPass {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcOptimizerPass {}
impl ::core::fmt::Debug for IDxcOptimizerPass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcOptimizerPass").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDxcOptimizerPass {
    type Vtable = IDxcOptimizerPass_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae2cd79f_cc22_453f_9b6b_b124e7a5204c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcOptimizerPass_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetOptionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetOptionArgCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetOptionArgName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, argindex: u32, ppresult: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetOptionArgDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, argindex: u32, ppresult: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcPdbUtils(::windows::core::IUnknown);
impl IDxcPdbUtils {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn Load<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, ppdbordxil: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Load)(::core::mem::transmute_copy(self), ppdbordxil.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetSourceCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSourceCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetSource(&self, uindex: u32) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSource)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex), ::core::mem::transmute(&mut result__)).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSourceName(&self, uindex: u32) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSourceName)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetFlagCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFlagCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFlag(&self, uindex: u32) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFlag)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetArgCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetArgCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetArg(&self, uindex: u32) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetArg)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetArgPairCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetArgPairCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetArgPair(&self, uindex: u32, pname: *mut super::super::super::Foundation::BSTR, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetArgPair)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex), ::core::mem::transmute(pname), ::core::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetDefineCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDefineCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDefine(&self, uindex: u32) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDefine)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTargetProfile(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTargetProfile)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEntryPoint(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetEntryPoint)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMainFileName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMainFileName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetHash(&self) -> ::windows::core::Result<IDxcBlob> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetHash)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDxcBlob>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsFullPDB(&self) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).IsFullPDB)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetFullPDB(&self) -> ::windows::core::Result<IDxcBlob> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFullPDB)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDxcBlob>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetVersionInfo(&self) -> ::windows::core::Result<IDxcVersionInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetVersionInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDxcVersionInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn SetCompiler<'a, Param0: ::windows::core::IntoParam<'a, IDxcCompiler3>>(&self, pcompiler: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCompiler)(::core::mem::transmute_copy(self), pcompiler.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn CompileForFullPDB(&self) -> ::windows::core::Result<IDxcResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CompileForFullPDB)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDxcResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn OverrideArgs(&self, pargpairs: *const DxcArgPair, unumargpairs: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OverrideArgs)(::core::mem::transmute_copy(self), ::core::mem::transmute(pargpairs), ::core::mem::transmute(unumargpairs)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn OverrideRootSignature<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, prootsignature: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OverrideRootSignature)(::core::mem::transmute_copy(self), prootsignature.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDxcPdbUtils> for ::windows::core::IUnknown {
    fn from(value: IDxcPdbUtils) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcPdbUtils> for ::windows::core::IUnknown {
    fn from(value: &IDxcPdbUtils) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcPdbUtils {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcPdbUtils {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcPdbUtils {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcPdbUtils {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcPdbUtils {}
impl ::core::fmt::Debug for IDxcPdbUtils {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcPdbUtils").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDxcPdbUtils {
    type Vtable = IDxcPdbUtils_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6c9647e_9d6a_4c3b_b94c_524b5a6c343d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcPdbUtils_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdbordxil: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetSourceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSourceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSourceName: usize,
    pub GetFlagCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFlag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFlag: usize,
    pub GetArgCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetArg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetArg: usize,
    pub GetArgPairCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetArgPair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, pname: *mut super::super::super::Foundation::BSTR, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetArgPair: usize,
    pub GetDefineCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDefine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDefine: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTargetProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTargetProfile: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEntryPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEntryPoint: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMainFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMainFileName: usize,
    pub GetHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presult: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsFullPDB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsFullPDB: usize,
    pub GetFullPDB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfullpdb: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetVersionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppversioninfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetCompiler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcompiler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CompileForFullPDB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub OverrideArgs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pargpairs: *const DxcArgPair, unumargpairs: u32) -> ::windows::core::HRESULT,
    pub OverrideRootSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prootsignature: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcResult(::windows::core::IUnknown);
impl IDxcResult {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::HRESULT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetResult(&self) -> ::windows::core::Result<IDxcBlob> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetResult)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDxcBlob>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetErrorBuffer(&self) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetErrorBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasOutput(&self, dxcoutkind: DXC_OUT_KIND) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).HasOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(dxcoutkind)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetOutput(&self, dxcoutkind: DXC_OUT_KIND, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputname: *mut ::core::option::Option<IDxcBlobUtf16>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(dxcoutkind), ::core::mem::transmute(iid), ::core::mem::transmute(ppvobject), ::core::mem::transmute(ppoutputname)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetNumOutputs(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetNumOutputs)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetOutputByIndex(&self, index: u32) -> DXC_OUT_KIND {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetOutputByIndex)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn PrimaryOutput(&self) -> DXC_OUT_KIND {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).PrimaryOutput)(::core::mem::transmute_copy(self)))
    }
}
impl ::core::convert::From<IDxcResult> for ::windows::core::IUnknown {
    fn from(value: IDxcResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcResult> for ::windows::core::IUnknown {
    fn from(value: &IDxcResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDxcResult> for IDxcOperationResult {
    fn from(value: IDxcResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcResult> for IDxcOperationResult {
    fn from(value: &IDxcResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDxcOperationResult> for IDxcResult {
    fn into_param(self) -> ::windows::core::Param<'a, IDxcOperationResult> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDxcOperationResult> for &'a IDxcResult {
    fn into_param(self) -> ::windows::core::Param<'a, IDxcOperationResult> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcResult {}
impl ::core::fmt::Debug for IDxcResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDxcResult {
    type Vtable = IDxcResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58346cda_dde7_4497_9461_6f87af5e0659);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcResult_Vtbl {
    pub base: IDxcOperationResult_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub HasOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dxcoutkind: DXC_OUT_KIND) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasOutput: usize,
    pub GetOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dxcoutkind: DXC_OUT_KIND, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetNumOutputs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetOutputByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> DXC_OUT_KIND,
    pub PrimaryOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DXC_OUT_KIND,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcUtils(::windows::core::IUnknown);
impl IDxcUtils {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn CreateBlobFromBlob<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0, offset: u32, length: u32) -> ::windows::core::Result<IDxcBlob> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateBlobFromBlob)(::core::mem::transmute_copy(self), pblob.into_param().abi(), ::core::mem::transmute(offset), ::core::mem::transmute(length), ::core::mem::transmute(&mut result__)).from_abi::<IDxcBlob>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn CreateBlobFromPinned(&self, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateBlobFromPinned)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(size), ::core::mem::transmute(codepage), ::core::mem::transmute(&mut result__)).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MoveToBlob<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IMalloc>>(&self, pdata: *const ::core::ffi::c_void, pimalloc: Param1, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MoveToBlob)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), pimalloc.into_param().abi(), ::core::mem::transmute(size), ::core::mem::transmute(codepage), ::core::mem::transmute(&mut result__)).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn CreateBlob(&self, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateBlob)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(size), ::core::mem::transmute(codepage), ::core::mem::transmute(&mut result__)).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn LoadFile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pfilename: Param0, pcodepage: *const DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LoadFile)(::core::mem::transmute_copy(self), pfilename.into_param().abi(), ::core::mem::transmute(pcodepage), ::core::mem::transmute(&mut result__)).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateReadOnlyStreamFromBlob<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0) -> ::windows::core::Result<super::super::super::System::Com::IStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateReadOnlyStreamFromBlob)(::core::mem::transmute_copy(self), pblob.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn CreateDefaultIncludeHandler(&self) -> ::windows::core::Result<IDxcIncludeHandler> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateDefaultIncludeHandler)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDxcIncludeHandler>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetBlobAsUtf8<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0) -> ::windows::core::Result<IDxcBlobUtf8> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetBlobAsUtf8)(::core::mem::transmute_copy(self), pblob.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDxcBlobUtf8>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetBlobAsUtf16<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0) -> ::windows::core::Result<IDxcBlobUtf16> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetBlobAsUtf16)(::core::mem::transmute_copy(self), pblob.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDxcBlobUtf16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetDxilContainerPart(&self, pshader: *const DxcBuffer, dxcpart: u32, pppartdata: *mut *mut ::core::ffi::c_void, ppartsizeinbytes: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDxilContainerPart)(::core::mem::transmute_copy(self), ::core::mem::transmute(pshader), ::core::mem::transmute(dxcpart), ::core::mem::transmute(pppartdata), ::core::mem::transmute(ppartsizeinbytes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn CreateReflection(&self, pdata: *const DxcBuffer, iid: *const ::windows::core::GUID, ppvreflection: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateReflection)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(iid), ::core::mem::transmute(ppvreflection)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn BuildArguments<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, psourcename: Param0, pentrypoint: Param1, ptargetprofile: Param2, parguments: &[::windows::core::PWSTR], pdefines: &[DxcDefine]) -> ::windows::core::Result<IDxcCompilerArgs> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).BuildArguments)(::core::mem::transmute_copy(self), psourcename.into_param().abi(), pentrypoint.into_param().abi(), ptargetprofile.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(parguments)), parguments.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pdefines)), pdefines.len() as _, ::core::mem::transmute(&mut result__)).from_abi::<IDxcCompilerArgs>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetPDBContents<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, ppdbblob: Param0, pphash: *mut ::core::option::Option<IDxcBlob>, ppcontainer: *mut ::core::option::Option<IDxcBlob>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPDBContents)(::core::mem::transmute_copy(self), ppdbblob.into_param().abi(), ::core::mem::transmute(pphash), ::core::mem::transmute(ppcontainer)).ok()
    }
}
impl ::core::convert::From<IDxcUtils> for ::windows::core::IUnknown {
    fn from(value: IDxcUtils) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcUtils> for ::windows::core::IUnknown {
    fn from(value: &IDxcUtils) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcUtils {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcUtils {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcUtils {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcUtils {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcUtils {}
impl ::core::fmt::Debug for IDxcUtils {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcUtils").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDxcUtils {
    type Vtable = IDxcUtils_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4605c4cb_2019_492a_ada4_65f20bb7d67f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcUtils_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub CreateBlobFromBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, offset: u32, length: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateBlobFromPinned: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub MoveToBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, pimalloc: ::windows::core::RawPtr, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    MoveToBlob: usize,
    pub CreateBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub LoadFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilename: ::windows::core::PCWSTR, pcodepage: *const DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateReadOnlyStreamFromBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateReadOnlyStreamFromBlob: usize,
    pub CreateDefaultIncludeHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetBlobAsUtf8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetBlobAsUtf16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: ::windows::core::RawPtr, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetDxilContainerPart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshader: *const DxcBuffer, dxcpart: u32, pppartdata: *mut *mut ::core::ffi::c_void, ppartsizeinbytes: *mut u32) -> ::windows::core::HRESULT,
    pub CreateReflection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *const DxcBuffer, iid: *const ::windows::core::GUID, ppvreflection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BuildArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psourcename: ::windows::core::PCWSTR, pentrypoint: ::windows::core::PCWSTR, ptargetprofile: ::windows::core::PCWSTR, parguments: *const ::windows::core::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, ppargs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetPDBContents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdbblob: ::windows::core::RawPtr, pphash: *mut ::windows::core::RawPtr, ppcontainer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcValidator(::windows::core::IUnknown);
impl IDxcValidator {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn Validate<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, pshader: Param0, flags: u32) -> ::windows::core::Result<IDxcOperationResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Validate)(::core::mem::transmute_copy(self), pshader.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(&mut result__)).from_abi::<IDxcOperationResult>(result__)
    }
}
impl ::core::convert::From<IDxcValidator> for ::windows::core::IUnknown {
    fn from(value: IDxcValidator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcValidator> for ::windows::core::IUnknown {
    fn from(value: &IDxcValidator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcValidator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcValidator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcValidator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcValidator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcValidator {}
impl ::core::fmt::Debug for IDxcValidator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcValidator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDxcValidator {
    type Vtable = IDxcValidator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6e82bd2_1fd7_4826_9811_2857e797f49a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcValidator_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Validate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshader: ::windows::core::RawPtr, flags: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcValidator2(::windows::core::IUnknown);
impl IDxcValidator2 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn Validate<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, pshader: Param0, flags: u32) -> ::windows::core::Result<IDxcOperationResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Validate)(::core::mem::transmute_copy(self), pshader.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(&mut result__)).from_abi::<IDxcOperationResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn ValidateWithDebug<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, pshader: Param0, flags: u32, poptdebugbitcode: *const DxcBuffer) -> ::windows::core::Result<IDxcOperationResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ValidateWithDebug)(::core::mem::transmute_copy(self), pshader.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(poptdebugbitcode), ::core::mem::transmute(&mut result__)).from_abi::<IDxcOperationResult>(result__)
    }
}
impl ::core::convert::From<IDxcValidator2> for ::windows::core::IUnknown {
    fn from(value: IDxcValidator2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcValidator2> for ::windows::core::IUnknown {
    fn from(value: &IDxcValidator2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcValidator2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcValidator2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDxcValidator2> for IDxcValidator {
    fn from(value: IDxcValidator2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcValidator2> for IDxcValidator {
    fn from(value: &IDxcValidator2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDxcValidator> for IDxcValidator2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDxcValidator> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDxcValidator> for &'a IDxcValidator2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDxcValidator> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcValidator2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcValidator2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcValidator2 {}
impl ::core::fmt::Debug for IDxcValidator2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcValidator2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDxcValidator2 {
    type Vtable = IDxcValidator2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x458e1fd1_b1b2_4750_a6e1_9c10f03bed92);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcValidator2_Vtbl {
    pub base: IDxcValidator_Vtbl,
    pub ValidateWithDebug: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pshader: ::windows::core::RawPtr, flags: u32, poptdebugbitcode: *const DxcBuffer, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcVersionInfo(::windows::core::IUnknown);
impl IDxcVersionInfo {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetVersion(&self, pmajor: *mut u32, pminor: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmajor), ::core::mem::transmute(pminor)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IDxcVersionInfo> for ::windows::core::IUnknown {
    fn from(value: IDxcVersionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcVersionInfo> for ::windows::core::IUnknown {
    fn from(value: &IDxcVersionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcVersionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcVersionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcVersionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcVersionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcVersionInfo {}
impl ::core::fmt::Debug for IDxcVersionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcVersionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDxcVersionInfo {
    type Vtable = IDxcVersionInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb04f5b50_2059_4f12_a8ff_a1e0cde1cc7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcVersionInfo_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmajor: *mut u32, pminor: *mut u32) -> ::windows::core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcVersionInfo2(::windows::core::IUnknown);
impl IDxcVersionInfo2 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetVersion(&self, pmajor: *mut u32, pminor: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmajor), ::core::mem::transmute(pminor)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetCommitInfo(&self, pcommitcount: *mut u32, pcommithash: *mut *mut i8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCommitInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcommitcount), ::core::mem::transmute(pcommithash)).ok()
    }
}
impl ::core::convert::From<IDxcVersionInfo2> for ::windows::core::IUnknown {
    fn from(value: IDxcVersionInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcVersionInfo2> for ::windows::core::IUnknown {
    fn from(value: &IDxcVersionInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcVersionInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcVersionInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDxcVersionInfo2> for IDxcVersionInfo {
    fn from(value: IDxcVersionInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcVersionInfo2> for IDxcVersionInfo {
    fn from(value: &IDxcVersionInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDxcVersionInfo> for IDxcVersionInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDxcVersionInfo> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDxcVersionInfo> for &'a IDxcVersionInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDxcVersionInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcVersionInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcVersionInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcVersionInfo2 {}
impl ::core::fmt::Debug for IDxcVersionInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcVersionInfo2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDxcVersionInfo2 {
    type Vtable = IDxcVersionInfo2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb6904c4_42f0_4b62_9c46_983af7da7c83);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcVersionInfo2_Vtbl {
    pub base: IDxcVersionInfo_Vtbl,
    pub GetCommitInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcommitcount: *mut u32, pcommithash: *mut *mut i8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
#[repr(transparent)]
pub struct IDxcVersionInfo3(::windows::core::IUnknown);
impl IDxcVersionInfo3 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D_Dxc\"`*"]
    pub unsafe fn GetCustomVersionString(&self) -> ::windows::core::Result<*mut i8> {
        let mut result__: *mut i8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCustomVersionString)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut i8>(result__)
    }
}
impl ::core::convert::From<IDxcVersionInfo3> for ::windows::core::IUnknown {
    fn from(value: IDxcVersionInfo3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDxcVersionInfo3> for ::windows::core::IUnknown {
    fn from(value: &IDxcVersionInfo3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcVersionInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcVersionInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDxcVersionInfo3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDxcVersionInfo3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDxcVersionInfo3 {}
impl ::core::fmt::Debug for IDxcVersionInfo3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDxcVersionInfo3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDxcVersionInfo3 {
    type Vtable = IDxcVersionInfo3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e13e843_9d25_473c_9ad2_03b2d0b44b1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcVersionInfo3_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCustomVersionString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pversionstring: *mut *mut i8) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
