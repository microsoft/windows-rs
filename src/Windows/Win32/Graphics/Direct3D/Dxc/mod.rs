#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DXC_CP(pub u32);
pub const DXC_CP_ACP: DXC_CP = DXC_CP(0u32);
pub const DXC_CP_UTF16: DXC_CP = DXC_CP(1200u32);
pub const DXC_CP_UTF8: DXC_CP = DXC_CP(65001u32);
impl ::core::convert::From<u32> for DXC_CP {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DXC_CP {
    type Abi = Self;
}
impl ::core::ops::BitOr for DXC_CP {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for DXC_CP {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for DXC_CP {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for DXC_CP {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for DXC_CP {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DXC_HASHFLAG_INCLUDES_SOURCE: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DXC_OUT_KIND(pub i32);
pub const DXC_OUT_NONE: DXC_OUT_KIND = DXC_OUT_KIND(0i32);
pub const DXC_OUT_OBJECT: DXC_OUT_KIND = DXC_OUT_KIND(1i32);
pub const DXC_OUT_ERRORS: DXC_OUT_KIND = DXC_OUT_KIND(2i32);
pub const DXC_OUT_PDB: DXC_OUT_KIND = DXC_OUT_KIND(3i32);
pub const DXC_OUT_SHADER_HASH: DXC_OUT_KIND = DXC_OUT_KIND(4i32);
pub const DXC_OUT_DISASSEMBLY: DXC_OUT_KIND = DXC_OUT_KIND(5i32);
pub const DXC_OUT_HLSL: DXC_OUT_KIND = DXC_OUT_KIND(6i32);
pub const DXC_OUT_TEXT: DXC_OUT_KIND = DXC_OUT_KIND(7i32);
pub const DXC_OUT_REFLECTION: DXC_OUT_KIND = DXC_OUT_KIND(8i32);
pub const DXC_OUT_ROOT_SIGNATURE: DXC_OUT_KIND = DXC_OUT_KIND(9i32);
pub const DXC_OUT_EXTRA_OUTPUTS: DXC_OUT_KIND = DXC_OUT_KIND(10i32);
pub const DXC_OUT_FORCE_DWORD: DXC_OUT_KIND = DXC_OUT_KIND(-1i32);
impl ::core::convert::From<i32> for DXC_OUT_KIND {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DXC_OUT_KIND {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DxcArgPair {
    pub pName: super::super::super::Foundation::PWSTR,
    pub pValue: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl DxcArgPair {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DxcArgPair {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DxcArgPair {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DxcArgPair").field("pName", &self.pName).field("pValue", &self.pValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DxcArgPair {
    fn eq(&self, other: &Self) -> bool {
        self.pName == other.pName && self.pValue == other.pValue
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DxcArgPair {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DxcArgPair {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DxcBuffer {
    pub Ptr: *mut ::core::ffi::c_void,
    pub Size: usize,
    pub Encoding: u32,
}
impl DxcBuffer {}
impl ::core::default::Default for DxcBuffer {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DxcBuffer {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DxcBuffer").field("Ptr", &self.Ptr).field("Size", &self.Size).field("Encoding", &self.Encoding).finish()
    }
}
impl ::core::cmp::PartialEq for DxcBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.Ptr == other.Ptr && self.Size == other.Size && self.Encoding == other.Encoding
    }
}
impl ::core::cmp::Eq for DxcBuffer {}
unsafe impl ::windows::core::Abi for DxcBuffer {
    type Abi = Self;
}
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
#[cfg(feature = "Win32_System_Com")]
pub type DxcCreateInstance2Proc = ::core::option::Option<unsafe extern "system" fn(pmalloc: ::windows::core::RawPtr, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
pub type DxcCreateInstanceProc = ::core::option::Option<unsafe extern "system" fn(rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DxcDefine {
    pub Name: super::super::super::Foundation::PWSTR,
    pub Value: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl DxcDefine {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DxcDefine {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DxcDefine {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DxcDefine").field("Name", &self.Name).field("Value", &self.Value).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DxcDefine {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Value == other.Value
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DxcDefine {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DxcDefine {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DxcShaderHash {
    pub Flags: u32,
    pub HashDigest: [u8; 16],
}
impl DxcShaderHash {}
impl ::core::default::Default for DxcShaderHash {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DxcShaderHash {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DxcShaderHash").field("Flags", &self.Flags).field("HashDigest", &self.HashDigest).finish()
    }
}
impl ::core::cmp::PartialEq for DxcShaderHash {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.HashDigest == other.HashDigest
    }
}
impl ::core::cmp::Eq for DxcShaderHash {}
unsafe impl ::windows::core::Abi for DxcShaderHash {
    type Abi = Self;
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDxcAssembler(pub ::windows::core::IUnknown);
impl IDxcAssembler {
    pub unsafe fn AssembleToContainer<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, pshader: Param0) -> ::windows::core::Result<IDxcOperationResult> {
        let mut result__: <IDxcOperationResult as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pshader.into_param().abi(), &mut result__).from_abi::<IDxcOperationResult>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDxcAssembler {
    type Vtable = IDxcAssembler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x091f7a26_1c1f_4948_904b_e6e3a8a771d5);
}
impl ::core::convert::From<IDxcAssembler> for ::windows::core::IUnknown {
    fn from(value: IDxcAssembler) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDxcAssembler> for ::windows::core::IUnknown {
    fn from(value: &IDxcAssembler) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcAssembler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcAssembler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcAssembler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pshader: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDxcBlob(pub ::windows::core::IUnknown);
impl IDxcBlob {
    pub unsafe fn GetBufferPointer(&self) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetBufferSize(&self) -> usize {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::core::Interface for IDxcBlob {
    type Vtable = IDxcBlob_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ba5fb08_5195_40e2_ac58_0d989c3a0102);
}
impl ::core::convert::From<IDxcBlob> for ::windows::core::IUnknown {
    fn from(value: IDxcBlob) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDxcBlob> for ::windows::core::IUnknown {
    fn from(value: &IDxcBlob) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcBlob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcBlob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlob_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> *mut ::core::ffi::c_void,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDxcBlobEncoding(pub ::windows::core::IUnknown);
impl IDxcBlobEncoding {
    pub unsafe fn GetBufferPointer(&self) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetBufferSize(&self) -> usize {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEncoding(&self, pknown: *mut super::super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pknown), ::core::mem::transmute(pcodepage)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDxcBlobEncoding {
    type Vtable = IDxcBlobEncoding_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7241d424_2646_4191_97c0_98e96e42fc68);
}
impl ::core::convert::From<IDxcBlobEncoding> for ::windows::core::IUnknown {
    fn from(value: IDxcBlobEncoding) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDxcBlobEncoding> for ::windows::core::IUnknown {
    fn from(value: &IDxcBlobEncoding) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcBlobEncoding {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcBlobEncoding {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDxcBlob> for &IDxcBlobEncoding {
    fn into_param(self) -> ::windows::core::Param<'a, IDxcBlob> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlobEncoding_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> *mut ::core::ffi::c_void,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pknown: *mut super::super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDxcBlobUtf16(pub ::windows::core::IUnknown);
impl IDxcBlobUtf16 {
    pub unsafe fn GetBufferPointer(&self) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetBufferSize(&self) -> usize {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEncoding(&self, pknown: *mut super::super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pknown), ::core::mem::transmute(pcodepage)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStringPointer(&self) -> super::super::super::Foundation::PWSTR {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetStringLength(&self) -> usize {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::core::Interface for IDxcBlobUtf16 {
    type Vtable = IDxcBlobUtf16_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3f84eab_0faa_497e_a39c_ee6ed60b2d84);
}
impl ::core::convert::From<IDxcBlobUtf16> for ::windows::core::IUnknown {
    fn from(value: IDxcBlobUtf16) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDxcBlobUtf16> for ::windows::core::IUnknown {
    fn from(value: &IDxcBlobUtf16) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcBlobUtf16 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcBlobUtf16 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDxcBlobEncoding> for &IDxcBlobUtf16 {
    fn into_param(self) -> ::windows::core::Param<'a, IDxcBlobEncoding> {
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
impl<'a> ::windows::core::IntoParam<'a, IDxcBlob> for &IDxcBlobUtf16 {
    fn into_param(self) -> ::windows::core::Param<'a, IDxcBlob> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlobUtf16_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> *mut ::core::ffi::c_void,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pknown: *mut super::super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> super::super::super::Foundation::PWSTR,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDxcBlobUtf8(pub ::windows::core::IUnknown);
impl IDxcBlobUtf8 {
    pub unsafe fn GetBufferPointer(&self) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetBufferSize(&self) -> usize {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEncoding(&self, pknown: *mut super::super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pknown), ::core::mem::transmute(pcodepage)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStringPointer(&self) -> super::super::super::Foundation::PSTR {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetStringLength(&self) -> usize {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::core::Interface for IDxcBlobUtf8 {
    type Vtable = IDxcBlobUtf8_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3da636c9_ba71_4024_a301_30cbf125305b);
}
impl ::core::convert::From<IDxcBlobUtf8> for ::windows::core::IUnknown {
    fn from(value: IDxcBlobUtf8) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDxcBlobUtf8> for ::windows::core::IUnknown {
    fn from(value: &IDxcBlobUtf8) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcBlobUtf8 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcBlobUtf8 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDxcBlobEncoding> for &IDxcBlobUtf8 {
    fn into_param(self) -> ::windows::core::Param<'a, IDxcBlobEncoding> {
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
impl<'a> ::windows::core::IntoParam<'a, IDxcBlob> for &IDxcBlobUtf8 {
    fn into_param(self) -> ::windows::core::Param<'a, IDxcBlob> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcBlobUtf8_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> *mut ::core::ffi::c_void,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pknown: *mut super::super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> super::super::super::Foundation::PSTR,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDxcCompiler(pub ::windows::core::IUnknown);
impl IDxcCompiler {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Compile<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param8: ::windows::core::IntoParam<'a, IDxcIncludeHandler>>(
        &self,
        psource: Param0,
        psourcename: Param1,
        pentrypoint: Param2,
        ptargetprofile: Param3,
        parguments: *const super::super::super::Foundation::PWSTR,
        argcount: u32,
        pdefines: *const DxcDefine,
        definecount: u32,
        pincludehandler: Param8,
    ) -> ::windows::core::Result<IDxcOperationResult> {
        let mut result__: <IDxcOperationResult as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(
            ::core::mem::transmute_copy(self),
            psource.into_param().abi(),
            psourcename.into_param().abi(),
            pentrypoint.into_param().abi(),
            ptargetprofile.into_param().abi(),
            ::core::mem::transmute(parguments),
            ::core::mem::transmute(argcount),
            ::core::mem::transmute(pdefines),
            ::core::mem::transmute(definecount),
            pincludehandler.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDxcOperationResult>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Preprocess<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param6: ::windows::core::IntoParam<'a, IDxcIncludeHandler>>(&self, psource: Param0, psourcename: Param1, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: Param6) -> ::windows::core::Result<IDxcOperationResult> {
        let mut result__: <IDxcOperationResult as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), psource.into_param().abi(), psourcename.into_param().abi(), ::core::mem::transmute(parguments), ::core::mem::transmute(argcount), ::core::mem::transmute(pdefines), ::core::mem::transmute(definecount), pincludehandler.into_param().abi(), &mut result__).from_abi::<IDxcOperationResult>(result__)
    }
    pub unsafe fn Disassemble<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, psource: Param0) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__: <IDxcBlobEncoding as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), psource.into_param().abi(), &mut result__).from_abi::<IDxcBlobEncoding>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDxcCompiler {
    type Vtable = IDxcCompiler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c210bf3_011f_4422_8d70_6f9acb8db617);
}
impl ::core::convert::From<IDxcCompiler> for ::windows::core::IUnknown {
    fn from(value: IDxcCompiler) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDxcCompiler> for ::windows::core::IUnknown {
    fn from(value: &IDxcCompiler) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcCompiler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcCompiler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompiler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, psource: ::windows::core::RawPtr, psourcename: super::super::super::Foundation::PWSTR, pentrypoint: super::super::super::Foundation::PWSTR, ptargetprofile: super::super::super::Foundation::PWSTR, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psource: ::windows::core::RawPtr, psourcename: super::super::super::Foundation::PWSTR, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psource: ::windows::core::RawPtr, ppdisassembly: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDxcCompiler2(pub ::windows::core::IUnknown);
impl IDxcCompiler2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Compile<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param8: ::windows::core::IntoParam<'a, IDxcIncludeHandler>>(
        &self,
        psource: Param0,
        psourcename: Param1,
        pentrypoint: Param2,
        ptargetprofile: Param3,
        parguments: *const super::super::super::Foundation::PWSTR,
        argcount: u32,
        pdefines: *const DxcDefine,
        definecount: u32,
        pincludehandler: Param8,
    ) -> ::windows::core::Result<IDxcOperationResult> {
        let mut result__: <IDxcOperationResult as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(
            ::core::mem::transmute_copy(self),
            psource.into_param().abi(),
            psourcename.into_param().abi(),
            pentrypoint.into_param().abi(),
            ptargetprofile.into_param().abi(),
            ::core::mem::transmute(parguments),
            ::core::mem::transmute(argcount),
            ::core::mem::transmute(pdefines),
            ::core::mem::transmute(definecount),
            pincludehandler.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDxcOperationResult>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Preprocess<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param6: ::windows::core::IntoParam<'a, IDxcIncludeHandler>>(&self, psource: Param0, psourcename: Param1, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: Param6) -> ::windows::core::Result<IDxcOperationResult> {
        let mut result__: <IDxcOperationResult as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), psource.into_param().abi(), psourcename.into_param().abi(), ::core::mem::transmute(parguments), ::core::mem::transmute(argcount), ::core::mem::transmute(pdefines), ::core::mem::transmute(definecount), pincludehandler.into_param().abi(), &mut result__).from_abi::<IDxcOperationResult>(result__)
    }
    pub unsafe fn Disassemble<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, psource: Param0) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__: <IDxcBlobEncoding as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), psource.into_param().abi(), &mut result__).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CompileWithDebug<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param8: ::windows::core::IntoParam<'a, IDxcIncludeHandler>>(
        &self,
        psource: Param0,
        psourcename: Param1,
        pentrypoint: Param2,
        ptargetprofile: Param3,
        parguments: *const super::super::super::Foundation::PWSTR,
        argcount: u32,
        pdefines: *const DxcDefine,
        definecount: u32,
        pincludehandler: Param8,
        ppresult: *mut ::core::option::Option<IDxcOperationResult>,
        ppdebugblobname: *mut super::super::super::Foundation::PWSTR,
        ppdebugblob: *mut ::core::option::Option<IDxcBlob>,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            psource.into_param().abi(),
            psourcename.into_param().abi(),
            pentrypoint.into_param().abi(),
            ptargetprofile.into_param().abi(),
            ::core::mem::transmute(parguments),
            ::core::mem::transmute(argcount),
            ::core::mem::transmute(pdefines),
            ::core::mem::transmute(definecount),
            pincludehandler.into_param().abi(),
            ::core::mem::transmute(ppresult),
            ::core::mem::transmute(ppdebugblobname),
            ::core::mem::transmute(ppdebugblob),
        )
        .ok()
    }
}
unsafe impl ::windows::core::Interface for IDxcCompiler2 {
    type Vtable = IDxcCompiler2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa005a9d9_b8bb_4594_b5c9_0e633bec4d37);
}
impl ::core::convert::From<IDxcCompiler2> for ::windows::core::IUnknown {
    fn from(value: IDxcCompiler2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDxcCompiler2> for ::windows::core::IUnknown {
    fn from(value: &IDxcCompiler2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcCompiler2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcCompiler2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDxcCompiler> for &IDxcCompiler2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDxcCompiler> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompiler2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, psource: ::windows::core::RawPtr, psourcename: super::super::super::Foundation::PWSTR, pentrypoint: super::super::super::Foundation::PWSTR, ptargetprofile: super::super::super::Foundation::PWSTR, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psource: ::windows::core::RawPtr, psourcename: super::super::super::Foundation::PWSTR, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psource: ::windows::core::RawPtr, ppdisassembly: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        psource: ::windows::core::RawPtr,
        psourcename: super::super::super::Foundation::PWSTR,
        pentrypoint: super::super::super::Foundation::PWSTR,
        ptargetprofile: super::super::super::Foundation::PWSTR,
        parguments: *const super::super::super::Foundation::PWSTR,
        argcount: u32,
        pdefines: *const DxcDefine,
        definecount: u32,
        pincludehandler: ::windows::core::RawPtr,
        ppresult: *mut ::windows::core::RawPtr,
        ppdebugblobname: *mut super::super::super::Foundation::PWSTR,
        ppdebugblob: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDxcCompiler3(pub ::windows::core::IUnknown);
impl IDxcCompiler3 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Compile<'a, Param3: ::windows::core::IntoParam<'a, IDxcIncludeHandler>>(&self, psource: *const DxcBuffer, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pincludehandler: Param3, riid: *const ::windows::core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(psource), ::core::mem::transmute(parguments), ::core::mem::transmute(argcount), pincludehandler.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppresult)).ok()
    }
    pub unsafe fn Disassemble(&self, pobject: *const DxcBuffer, riid: *const ::windows::core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pobject), ::core::mem::transmute(riid), ::core::mem::transmute(ppresult)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDxcCompiler3 {
    type Vtable = IDxcCompiler3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x228b4687_5a6a_4730_900c_9702b2203f54);
}
impl ::core::convert::From<IDxcCompiler3> for ::windows::core::IUnknown {
    fn from(value: IDxcCompiler3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDxcCompiler3> for ::windows::core::IUnknown {
    fn from(value: &IDxcCompiler3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcCompiler3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcCompiler3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompiler3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psource: *const DxcBuffer, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pincludehandler: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pobject: *const DxcBuffer, riid: *const ::windows::core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDxcCompilerArgs(pub ::windows::core::IUnknown);
impl IDxcCompilerArgs {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetArguments(&self) -> *mut super::super::super::Foundation::PWSTR {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddArguments(&self, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(parguments), ::core::mem::transmute(argcount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddArgumentsUTF8(&self, parguments: *const super::super::super::Foundation::PSTR, argcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(parguments), ::core::mem::transmute(argcount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDefines(&self, pdefines: *const DxcDefine, definecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdefines), ::core::mem::transmute(definecount)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDxcCompilerArgs {
    type Vtable = IDxcCompilerArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73effe2a_70dc_45f8_9690_eff64c02429d);
}
impl ::core::convert::From<IDxcCompilerArgs> for ::windows::core::IUnknown {
    fn from(value: IDxcCompilerArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDxcCompilerArgs> for ::windows::core::IUnknown {
    fn from(value: &IDxcCompilerArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcCompilerArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcCompilerArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcCompilerArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> *mut super::super::super::Foundation::PWSTR,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, parguments: *const super::super::super::Foundation::PSTR, argcount: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdefines: *const DxcDefine, definecount: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDxcContainerBuilder(pub ::windows::core::IUnknown);
impl IDxcContainerBuilder {
    pub unsafe fn Load<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, pdxilcontainerheader: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pdxilcontainerheader.into_param().abi()).ok()
    }
    pub unsafe fn AddPart<'a, Param1: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, fourcc: u32, psource: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(fourcc), psource.into_param().abi()).ok()
    }
    pub unsafe fn RemovePart(&self, fourcc: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(fourcc)).ok()
    }
    pub unsafe fn SerializeContainer(&self) -> ::windows::core::Result<IDxcOperationResult> {
        let mut result__: <IDxcOperationResult as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDxcOperationResult>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDxcContainerBuilder {
    type Vtable = IDxcContainerBuilder_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x334b1f50_2292_4b35_99a1_25588d8c17fe);
}
impl ::core::convert::From<IDxcContainerBuilder> for ::windows::core::IUnknown {
    fn from(value: IDxcContainerBuilder) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDxcContainerBuilder> for ::windows::core::IUnknown {
    fn from(value: &IDxcContainerBuilder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcContainerBuilder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcContainerBuilder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcContainerBuilder_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdxilcontainerheader: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fourcc: u32, psource: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fourcc: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDxcContainerReflection(pub ::windows::core::IUnknown);
impl IDxcContainerReflection {
    pub unsafe fn Load<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, pcontainer: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pcontainer.into_param().abi()).ok()
    }
    pub unsafe fn GetPartCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetPartKind(&self, idx: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(idx), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetPartContent(&self, idx: u32) -> ::windows::core::Result<IDxcBlob> {
        let mut result__: <IDxcBlob as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(idx), &mut result__).from_abi::<IDxcBlob>(result__)
    }
    pub unsafe fn FindFirstPartKind(&self, kind: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(kind), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetPartReflection(&self, idx: u32, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(idx), ::core::mem::transmute(iid), ::core::mem::transmute(ppvobject)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDxcContainerReflection {
    type Vtable = IDxcContainerReflection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2c21b26_8350_4bdc_976a_331ce6f4c54c);
}
impl ::core::convert::From<IDxcContainerReflection> for ::windows::core::IUnknown {
    fn from(value: IDxcContainerReflection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDxcContainerReflection> for ::windows::core::IUnknown {
    fn from(value: &IDxcContainerReflection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcContainerReflection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcContainerReflection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcContainerReflection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcontainer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, presult: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, idx: u32, presult: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, idx: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, kind: u32, presult: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, idx: u32, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDxcExtraOutputs(pub ::windows::core::IUnknown);
impl IDxcExtraOutputs {
    pub unsafe fn GetOutputCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetOutput(&self, uindex: u32, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputtype: *mut ::core::option::Option<IDxcBlobUtf16>, ppoutputname: *mut ::core::option::Option<IDxcBlobUtf16>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex), ::core::mem::transmute(iid), ::core::mem::transmute(ppvobject), ::core::mem::transmute(ppoutputtype), ::core::mem::transmute(ppoutputname)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDxcExtraOutputs {
    type Vtable = IDxcExtraOutputs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x319b37a2_a5c2_494a_a5de_4801b2faf989);
}
impl ::core::convert::From<IDxcExtraOutputs> for ::windows::core::IUnknown {
    fn from(value: IDxcExtraOutputs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDxcExtraOutputs> for ::windows::core::IUnknown {
    fn from(value: &IDxcExtraOutputs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcExtraOutputs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcExtraOutputs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcExtraOutputs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uindex: u32, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputtype: *mut ::windows::core::RawPtr, ppoutputname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDxcIncludeHandler(pub ::windows::core::IUnknown);
impl IDxcIncludeHandler {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LoadSource<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pfilename: Param0) -> ::windows::core::Result<IDxcBlob> {
        let mut result__: <IDxcBlob as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pfilename.into_param().abi(), &mut result__).from_abi::<IDxcBlob>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDxcIncludeHandler {
    type Vtable = IDxcIncludeHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f61fc7d_950d_467f_b3e3_3c02fb49187c);
}
impl ::core::convert::From<IDxcIncludeHandler> for ::windows::core::IUnknown {
    fn from(value: IDxcIncludeHandler) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDxcIncludeHandler> for ::windows::core::IUnknown {
    fn from(value: &IDxcIncludeHandler) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcIncludeHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcIncludeHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcIncludeHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfilename: super::super::super::Foundation::PWSTR, ppincludesource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDxcLibrary(pub ::windows::core::IUnknown);
impl IDxcLibrary {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetMalloc<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IMalloc>>(&self, pmalloc: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pmalloc.into_param().abi()).ok()
    }
    pub unsafe fn CreateBlobFromBlob<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0, offset: u32, length: u32) -> ::windows::core::Result<IDxcBlob> {
        let mut result__: <IDxcBlob as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pblob.into_param().abi(), ::core::mem::transmute(offset), ::core::mem::transmute(length), &mut result__).from_abi::<IDxcBlob>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateBlobFromFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pfilename: Param0, codepage: *const DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__: <IDxcBlobEncoding as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pfilename.into_param().abi(), ::core::mem::transmute(codepage), &mut result__).from_abi::<IDxcBlobEncoding>(result__)
    }
    pub unsafe fn CreateBlobWithEncodingFromPinned(&self, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__: <IDxcBlobEncoding as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptext), ::core::mem::transmute(size), ::core::mem::transmute(codepage), &mut result__).from_abi::<IDxcBlobEncoding>(result__)
    }
    pub unsafe fn CreateBlobWithEncodingOnHeapCopy(&self, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__: <IDxcBlobEncoding as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptext), ::core::mem::transmute(size), ::core::mem::transmute(codepage), &mut result__).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateBlobWithEncodingOnMalloc<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IMalloc>>(&self, ptext: *const ::core::ffi::c_void, pimalloc: Param1, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__: <IDxcBlobEncoding as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptext), pimalloc.into_param().abi(), ::core::mem::transmute(size), ::core::mem::transmute(codepage), &mut result__).from_abi::<IDxcBlobEncoding>(result__)
    }
    pub unsafe fn CreateIncludeHandler(&self) -> ::windows::core::Result<IDxcIncludeHandler> {
        let mut result__: <IDxcIncludeHandler as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDxcIncludeHandler>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateStreamFromBlobReadOnly<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0) -> ::windows::core::Result<super::super::super::System::Com::IStream> {
        let mut result__: <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pblob.into_param().abi(), &mut result__).from_abi::<super::super::super::System::Com::IStream>(result__)
    }
    pub unsafe fn GetBlobAsUtf8<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__: <IDxcBlobEncoding as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pblob.into_param().abi(), &mut result__).from_abi::<IDxcBlobEncoding>(result__)
    }
    pub unsafe fn GetBlobAsUtf16<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__: <IDxcBlobEncoding as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pblob.into_param().abi(), &mut result__).from_abi::<IDxcBlobEncoding>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDxcLibrary {
    type Vtable = IDxcLibrary_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5204dc7_d18c_4c3c_bdfb_851673980fe7);
}
impl ::core::convert::From<IDxcLibrary> for ::windows::core::IUnknown {
    fn from(value: IDxcLibrary) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDxcLibrary> for ::windows::core::IUnknown {
    fn from(value: &IDxcLibrary) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcLibrary {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcLibrary {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcLibrary_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmalloc: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pblob: ::windows::core::RawPtr, offset: u32, length: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfilename: super::super::super::Foundation::PWSTR, codepage: *const DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptext: *const ::core::ffi::c_void, pimalloc: ::windows::core::RawPtr, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pblob: ::windows::core::RawPtr, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pblob: ::windows::core::RawPtr, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pblob: ::windows::core::RawPtr, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDxcLinker(pub ::windows::core::IUnknown);
impl IDxcLinker {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterLibrary<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, plibname: Param0, plib: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), plibname.into_param().abi(), plib.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Link<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pentryname: Param0, ptargetprofile: Param1, plibnames: *const super::super::super::Foundation::PWSTR, libcount: u32, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32) -> ::windows::core::Result<IDxcOperationResult> {
        let mut result__: <IDxcOperationResult as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pentryname.into_param().abi(), ptargetprofile.into_param().abi(), ::core::mem::transmute(plibnames), ::core::mem::transmute(libcount), ::core::mem::transmute(parguments), ::core::mem::transmute(argcount), &mut result__).from_abi::<IDxcOperationResult>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDxcLinker {
    type Vtable = IDxcLinker_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1b5be2a_62dd_4327_a1c2_42ac1e1e78e6);
}
impl ::core::convert::From<IDxcLinker> for ::windows::core::IUnknown {
    fn from(value: IDxcLinker) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDxcLinker> for ::windows::core::IUnknown {
    fn from(value: &IDxcLinker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcLinker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcLinker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcLinker_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plibname: super::super::super::Foundation::PWSTR, plib: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pentryname: super::super::super::Foundation::PWSTR, ptargetprofile: super::super::super::Foundation::PWSTR, plibnames: *const super::super::super::Foundation::PWSTR, libcount: u32, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDxcOperationResult(pub ::windows::core::IUnknown);
impl IDxcOperationResult {
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__: <::windows::core::HRESULT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
    }
    pub unsafe fn GetResult(&self) -> ::windows::core::Result<IDxcBlob> {
        let mut result__: <IDxcBlob as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDxcBlob>(result__)
    }
    pub unsafe fn GetErrorBuffer(&self) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__: <IDxcBlobEncoding as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDxcBlobEncoding>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDxcOperationResult {
    type Vtable = IDxcOperationResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcedb484a_d4e9_445a_b991_ca21ca157dc2);
}
impl ::core::convert::From<IDxcOperationResult> for ::windows::core::IUnknown {
    fn from(value: IDxcOperationResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDxcOperationResult> for ::windows::core::IUnknown {
    fn from(value: &IDxcOperationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcOperationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstatus: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pperrors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDxcOptimizer(pub ::windows::core::IUnknown);
impl IDxcOptimizer {
    pub unsafe fn GetAvailablePassCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAvailablePass(&self, index: u32) -> ::windows::core::Result<IDxcOptimizerPass> {
        let mut result__: <IDxcOptimizerPass as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<IDxcOptimizerPass>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunOptimizer<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0, ppoptions: *const super::super::super::Foundation::PWSTR, optioncount: u32, poutputmodule: *mut ::core::option::Option<IDxcBlob>, ppoutputtext: *mut ::core::option::Option<IDxcBlobEncoding>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pblob.into_param().abi(), ::core::mem::transmute(ppoptions), ::core::mem::transmute(optioncount), ::core::mem::transmute(poutputmodule), ::core::mem::transmute(ppoutputtext)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDxcOptimizer {
    type Vtable = IDxcOptimizer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25740e2e_9cba_401b_9119_4fb42f39f270);
}
impl ::core::convert::From<IDxcOptimizer> for ::windows::core::IUnknown {
    fn from(value: IDxcOptimizer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDxcOptimizer> for ::windows::core::IUnknown {
    fn from(value: &IDxcOptimizer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcOptimizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcOptimizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcOptimizer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pblob: ::windows::core::RawPtr, ppoptions: *const super::super::super::Foundation::PWSTR, optioncount: u32, poutputmodule: *mut ::windows::core::RawPtr, ppoutputtext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDxcOptimizerPass(pub ::windows::core::IUnknown);
impl IDxcOptimizerPass {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOptionName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetOptionArgCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOptionArgName(&self, argindex: u32) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(argindex), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOptionArgDescription(&self, argindex: u32) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(argindex), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDxcOptimizerPass {
    type Vtable = IDxcOptimizerPass_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae2cd79f_cc22_453f_9b6b_b124e7a5204c);
}
impl ::core::convert::From<IDxcOptimizerPass> for ::windows::core::IUnknown {
    fn from(value: IDxcOptimizerPass) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDxcOptimizerPass> for ::windows::core::IUnknown {
    fn from(value: &IDxcOptimizerPass) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcOptimizerPass {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcOptimizerPass {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcOptimizerPass_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppresult: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppresult: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, argindex: u32, ppresult: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, argindex: u32, ppresult: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDxcPdbUtils(pub ::windows::core::IUnknown);
impl IDxcPdbUtils {
    pub unsafe fn Load<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, ppdbordxil: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ppdbordxil.into_param().abi()).ok()
    }
    pub unsafe fn GetSourceCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetSource(&self, uindex: u32) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__: <IDxcBlobEncoding as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex), &mut result__).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSourceName(&self, uindex: u32) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetFlagCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFlag(&self, uindex: u32) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetArgCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetArg(&self, uindex: u32) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetArgPairCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetArgPair(&self, uindex: u32, pname: *mut super::super::super::Foundation::BSTR, pvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex), ::core::mem::transmute(pname), ::core::mem::transmute(pvalue)).ok()
    }
    pub unsafe fn GetDefineCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDefine(&self, uindex: u32) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTargetProfile(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEntryPoint(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMainFileName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetHash(&self) -> ::windows::core::Result<IDxcBlob> {
        let mut result__: <IDxcBlob as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDxcBlob>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsFullPDB(&self) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetFullPDB(&self) -> ::windows::core::Result<IDxcBlob> {
        let mut result__: <IDxcBlob as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDxcBlob>(result__)
    }
    pub unsafe fn GetVersionInfo(&self) -> ::windows::core::Result<IDxcVersionInfo> {
        let mut result__: <IDxcVersionInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDxcVersionInfo>(result__)
    }
    pub unsafe fn SetCompiler<'a, Param0: ::windows::core::IntoParam<'a, IDxcCompiler3>>(&self, pcompiler: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), pcompiler.into_param().abi()).ok()
    }
    pub unsafe fn CompileForFullPDB(&self) -> ::windows::core::Result<IDxcResult> {
        let mut result__: <IDxcResult as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDxcResult>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OverrideArgs(&self, pargpairs: *const DxcArgPair, unumargpairs: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(pargpairs), ::core::mem::transmute(unumargpairs)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OverrideRootSignature<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, prootsignature: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), prootsignature.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IDxcPdbUtils {
    type Vtable = IDxcPdbUtils_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6c9647e_9d6a_4c3b_b94c_524b5a6c343d);
}
impl ::core::convert::From<IDxcPdbUtils> for ::windows::core::IUnknown {
    fn from(value: IDxcPdbUtils) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDxcPdbUtils> for ::windows::core::IUnknown {
    fn from(value: &IDxcPdbUtils) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcPdbUtils {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcPdbUtils {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcPdbUtils_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppdbordxil: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uindex: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uindex: u32, presult: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uindex: u32, presult: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uindex: u32, presult: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uindex: u32, pname: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pvalue: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uindex: u32, presult: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, presult: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, presult: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, presult: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, presult: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppfullpdb: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppversioninfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcompiler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pargpairs: *const DxcArgPair, unumargpairs: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, prootsignature: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDxcResult(pub ::windows::core::IUnknown);
impl IDxcResult {
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__: <::windows::core::HRESULT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
    }
    pub unsafe fn GetResult(&self) -> ::windows::core::Result<IDxcBlob> {
        let mut result__: <IDxcBlob as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDxcBlob>(result__)
    }
    pub unsafe fn GetErrorBuffer(&self) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__: <IDxcBlobEncoding as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasOutput(&self, dxcoutkind: DXC_OUT_KIND) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dxcoutkind)))
    }
    pub unsafe fn GetOutput(&self, dxcoutkind: DXC_OUT_KIND, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputname: *mut ::core::option::Option<IDxcBlobUtf16>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dxcoutkind), ::core::mem::transmute(iid), ::core::mem::transmute(ppvobject), ::core::mem::transmute(ppoutputname)).ok()
    }
    pub unsafe fn GetNumOutputs(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetOutputByIndex(&self, index: u32) -> DXC_OUT_KIND {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)))
    }
    pub unsafe fn PrimaryOutput(&self) -> DXC_OUT_KIND {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::core::Interface for IDxcResult {
    type Vtable = IDxcResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58346cda_dde7_4497_9461_6f87af5e0659);
}
impl ::core::convert::From<IDxcResult> for ::windows::core::IUnknown {
    fn from(value: IDxcResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDxcResult> for ::windows::core::IUnknown {
    fn from(value: &IDxcResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDxcOperationResult> for &IDxcResult {
    fn into_param(self) -> ::windows::core::Param<'a, IDxcOperationResult> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstatus: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pperrors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dxcoutkind: DXC_OUT_KIND) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dxcoutkind: DXC_OUT_KIND, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32) -> DXC_OUT_KIND,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> DXC_OUT_KIND,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDxcUtils(pub ::windows::core::IUnknown);
impl IDxcUtils {
    pub unsafe fn CreateBlobFromBlob<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0, offset: u32, length: u32) -> ::windows::core::Result<IDxcBlob> {
        let mut result__: <IDxcBlob as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pblob.into_param().abi(), ::core::mem::transmute(offset), ::core::mem::transmute(length), &mut result__).from_abi::<IDxcBlob>(result__)
    }
    pub unsafe fn CreateBlobFromPinned(&self, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__: <IDxcBlobEncoding as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(size), ::core::mem::transmute(codepage), &mut result__).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MoveToBlob<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IMalloc>>(&self, pdata: *const ::core::ffi::c_void, pimalloc: Param1, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__: <IDxcBlobEncoding as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), pimalloc.into_param().abi(), ::core::mem::transmute(size), ::core::mem::transmute(codepage), &mut result__).from_abi::<IDxcBlobEncoding>(result__)
    }
    pub unsafe fn CreateBlob(&self, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__: <IDxcBlobEncoding as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(size), ::core::mem::transmute(codepage), &mut result__).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LoadFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pfilename: Param0, pcodepage: *const DXC_CP) -> ::windows::core::Result<IDxcBlobEncoding> {
        let mut result__: <IDxcBlobEncoding as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pfilename.into_param().abi(), ::core::mem::transmute(pcodepage), &mut result__).from_abi::<IDxcBlobEncoding>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateReadOnlyStreamFromBlob<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0) -> ::windows::core::Result<super::super::super::System::Com::IStream> {
        let mut result__: <super::super::super::System::Com::IStream as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pblob.into_param().abi(), &mut result__).from_abi::<super::super::super::System::Com::IStream>(result__)
    }
    pub unsafe fn CreateDefaultIncludeHandler(&self) -> ::windows::core::Result<IDxcIncludeHandler> {
        let mut result__: <IDxcIncludeHandler as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDxcIncludeHandler>(result__)
    }
    pub unsafe fn GetBlobAsUtf8<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0) -> ::windows::core::Result<IDxcBlobUtf8> {
        let mut result__: <IDxcBlobUtf8 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pblob.into_param().abi(), &mut result__).from_abi::<IDxcBlobUtf8>(result__)
    }
    pub unsafe fn GetBlobAsUtf16<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, pblob: Param0) -> ::windows::core::Result<IDxcBlobUtf16> {
        let mut result__: <IDxcBlobUtf16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pblob.into_param().abi(), &mut result__).from_abi::<IDxcBlobUtf16>(result__)
    }
    pub unsafe fn GetDxilContainerPart(&self, pshader: *const DxcBuffer, dxcpart: u32, pppartdata: *mut *mut ::core::ffi::c_void, ppartsizeinbytes: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pshader), ::core::mem::transmute(dxcpart), ::core::mem::transmute(pppartdata), ::core::mem::transmute(ppartsizeinbytes)).ok()
    }
    pub unsafe fn CreateReflection(&self, pdata: *const DxcBuffer, iid: *const ::windows::core::GUID, ppvreflection: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdata), ::core::mem::transmute(iid), ::core::mem::transmute(ppvreflection)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BuildArguments<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(
        &self,
        psourcename: Param0,
        pentrypoint: Param1,
        ptargetprofile: Param2,
        parguments: *const super::super::super::Foundation::PWSTR,
        argcount: u32,
        pdefines: *const DxcDefine,
        definecount: u32,
    ) -> ::windows::core::Result<IDxcCompilerArgs> {
        let mut result__: <IDxcCompilerArgs as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), psourcename.into_param().abi(), pentrypoint.into_param().abi(), ptargetprofile.into_param().abi(), ::core::mem::transmute(parguments), ::core::mem::transmute(argcount), ::core::mem::transmute(pdefines), ::core::mem::transmute(definecount), &mut result__).from_abi::<IDxcCompilerArgs>(result__)
    }
    pub unsafe fn GetPDBContents<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, ppdbblob: Param0, pphash: *mut ::core::option::Option<IDxcBlob>, ppcontainer: *mut ::core::option::Option<IDxcBlob>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ppdbblob.into_param().abi(), ::core::mem::transmute(pphash), ::core::mem::transmute(ppcontainer)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDxcUtils {
    type Vtable = IDxcUtils_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4605c4cb_2019_492a_ada4_65f20bb7d67f);
}
impl ::core::convert::From<IDxcUtils> for ::windows::core::IUnknown {
    fn from(value: IDxcUtils) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDxcUtils> for ::windows::core::IUnknown {
    fn from(value: &IDxcUtils) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcUtils {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcUtils {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcUtils_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pblob: ::windows::core::RawPtr, offset: u32, length: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdata: *const ::core::ffi::c_void, pimalloc: ::windows::core::RawPtr, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfilename: super::super::super::Foundation::PWSTR, pcodepage: *const DXC_CP, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pblob: ::windows::core::RawPtr, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pblob: ::windows::core::RawPtr, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pblob: ::windows::core::RawPtr, pblobencoding: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pshader: *const DxcBuffer, dxcpart: u32, pppartdata: *mut *mut ::core::ffi::c_void, ppartsizeinbytes: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdata: *const DxcBuffer, iid: *const ::windows::core::GUID, ppvreflection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psourcename: super::super::super::Foundation::PWSTR, pentrypoint: super::super::super::Foundation::PWSTR, ptargetprofile: super::super::super::Foundation::PWSTR, parguments: *const super::super::super::Foundation::PWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, ppargs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppdbblob: ::windows::core::RawPtr, pphash: *mut ::windows::core::RawPtr, ppcontainer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDxcValidator(pub ::windows::core::IUnknown);
impl IDxcValidator {
    pub unsafe fn Validate<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, pshader: Param0, flags: u32) -> ::windows::core::Result<IDxcOperationResult> {
        let mut result__: <IDxcOperationResult as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pshader.into_param().abi(), ::core::mem::transmute(flags), &mut result__).from_abi::<IDxcOperationResult>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDxcValidator {
    type Vtable = IDxcValidator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6e82bd2_1fd7_4826_9811_2857e797f49a);
}
impl ::core::convert::From<IDxcValidator> for ::windows::core::IUnknown {
    fn from(value: IDxcValidator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDxcValidator> for ::windows::core::IUnknown {
    fn from(value: &IDxcValidator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcValidator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcValidator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcValidator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pshader: ::windows::core::RawPtr, flags: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDxcValidator2(pub ::windows::core::IUnknown);
impl IDxcValidator2 {
    pub unsafe fn Validate<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, pshader: Param0, flags: u32) -> ::windows::core::Result<IDxcOperationResult> {
        let mut result__: <IDxcOperationResult as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pshader.into_param().abi(), ::core::mem::transmute(flags), &mut result__).from_abi::<IDxcOperationResult>(result__)
    }
    pub unsafe fn ValidateWithDebug<'a, Param0: ::windows::core::IntoParam<'a, IDxcBlob>>(&self, pshader: Param0, flags: u32, poptdebugbitcode: *const DxcBuffer) -> ::windows::core::Result<IDxcOperationResult> {
        let mut result__: <IDxcOperationResult as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pshader.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(poptdebugbitcode), &mut result__).from_abi::<IDxcOperationResult>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDxcValidator2 {
    type Vtable = IDxcValidator2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x458e1fd1_b1b2_4750_a6e1_9c10f03bed92);
}
impl ::core::convert::From<IDxcValidator2> for ::windows::core::IUnknown {
    fn from(value: IDxcValidator2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDxcValidator2> for ::windows::core::IUnknown {
    fn from(value: &IDxcValidator2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcValidator2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcValidator2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDxcValidator> for &IDxcValidator2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDxcValidator> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcValidator2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pshader: ::windows::core::RawPtr, flags: u32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pshader: ::windows::core::RawPtr, flags: u32, poptdebugbitcode: *const DxcBuffer, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDxcVersionInfo(pub ::windows::core::IUnknown);
impl IDxcVersionInfo {
    pub unsafe fn GetVersion(&self, pmajor: *mut u32, pminor: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmajor), ::core::mem::transmute(pminor)).ok()
    }
    pub unsafe fn GetFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDxcVersionInfo {
    type Vtable = IDxcVersionInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb04f5b50_2059_4f12_a8ff_a1e0cde1cc7e);
}
impl ::core::convert::From<IDxcVersionInfo> for ::windows::core::IUnknown {
    fn from(value: IDxcVersionInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDxcVersionInfo> for ::windows::core::IUnknown {
    fn from(value: &IDxcVersionInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcVersionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcVersionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcVersionInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmajor: *mut u32, pminor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pflags: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDxcVersionInfo2(pub ::windows::core::IUnknown);
impl IDxcVersionInfo2 {
    pub unsafe fn GetVersion(&self, pmajor: *mut u32, pminor: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmajor), ::core::mem::transmute(pminor)).ok()
    }
    pub unsafe fn GetFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetCommitInfo(&self, pcommitcount: *mut u32, pcommithash: *mut *mut i8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcommitcount), ::core::mem::transmute(pcommithash)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDxcVersionInfo2 {
    type Vtable = IDxcVersionInfo2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb6904c4_42f0_4b62_9c46_983af7da7c83);
}
impl ::core::convert::From<IDxcVersionInfo2> for ::windows::core::IUnknown {
    fn from(value: IDxcVersionInfo2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDxcVersionInfo2> for ::windows::core::IUnknown {
    fn from(value: &IDxcVersionInfo2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcVersionInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcVersionInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDxcVersionInfo> for &IDxcVersionInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDxcVersionInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcVersionInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmajor: *mut u32, pminor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pflags: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcommitcount: *mut u32, pcommithash: *mut *mut i8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDxcVersionInfo3(pub ::windows::core::IUnknown);
impl IDxcVersionInfo3 {
    pub unsafe fn GetCustomVersionString(&self) -> ::windows::core::Result<*mut i8> {
        let mut result__: <*mut i8 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut i8>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDxcVersionInfo3 {
    type Vtable = IDxcVersionInfo3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e13e843_9d25_473c_9ad2_03b2d0b44b1e);
}
impl ::core::convert::From<IDxcVersionInfo3> for ::windows::core::IUnknown {
    fn from(value: IDxcVersionInfo3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDxcVersionInfo3> for ::windows::core::IUnknown {
    fn from(value: &IDxcVersionInfo3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDxcVersionInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDxcVersionInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDxcVersionInfo3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pversionstring: *mut *mut i8) -> ::windows::core::HRESULT,
);
