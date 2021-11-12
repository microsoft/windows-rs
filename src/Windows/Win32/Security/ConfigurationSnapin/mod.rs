#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISceSvcAttachmentData(pub ::windows::core::IUnknown);
impl ISceSvcAttachmentData {
    pub unsafe fn GetData(&self, scesvchandle: *mut ::core::ffi::c_void, scetype: SCESVC_INFO_TYPE, ppvdata: *mut *mut ::core::ffi::c_void, psceenumhandle: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(scesvchandle), ::core::mem::transmute(scetype), ::core::mem::transmute(ppvdata), ::core::mem::transmute(psceenumhandle)).ok()
    }
    pub unsafe fn Initialize<'a, Param2: ::windows::core::IntoParam<'a, ISceSvcAttachmentPersistInfo>>(&self, lpservicename: *mut i8, lptemplatename: *mut i8, lpscesvcpersistinfo: Param2, pscesvchandle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpservicename), ::core::mem::transmute(lptemplatename), lpscesvcpersistinfo.into_param().abi(), ::core::mem::transmute(pscesvchandle)).ok()
    }
    pub unsafe fn FreeBuffer(&self, pvdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvdata)).ok()
    }
    pub unsafe fn CloseHandle(&self, scesvchandle: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(scesvchandle)).ok()
    }
}
unsafe impl ::windows::core::Interface for ISceSvcAttachmentData {
    type Vtable = ISceSvcAttachmentData_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17c35fde_200d_11d1_affb_00c04fb984f9);
}
impl ::core::convert::From<ISceSvcAttachmentData> for ::windows::core::IUnknown {
    fn from(value: ISceSvcAttachmentData) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISceSvcAttachmentData> for ::windows::core::IUnknown {
    fn from(value: &ISceSvcAttachmentData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISceSvcAttachmentData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISceSvcAttachmentData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceSvcAttachmentData_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scesvchandle: *mut ::core::ffi::c_void, scetype: SCESVC_INFO_TYPE, ppvdata: *mut *mut ::core::ffi::c_void, psceenumhandle: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpservicename: *mut i8, lptemplatename: *mut i8, lpscesvcpersistinfo: ::windows::core::RawPtr, pscesvchandle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scesvchandle: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISceSvcAttachmentPersistInfo(pub ::windows::core::IUnknown);
impl ISceSvcAttachmentPersistInfo {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save(&self, lptemplatename: *mut i8, scesvchandle: *mut *mut ::core::ffi::c_void, ppvdata: *mut *mut ::core::ffi::c_void, pboverwriteall: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lptemplatename), ::core::mem::transmute(scesvchandle), ::core::mem::transmute(ppvdata), ::core::mem::transmute(pboverwriteall)).ok()
    }
    pub unsafe fn IsDirty(&self, lptemplatename: *mut i8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(lptemplatename)).ok()
    }
    pub unsafe fn FreeBuffer(&self, pvdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvdata)).ok()
    }
}
unsafe impl ::windows::core::Interface for ISceSvcAttachmentPersistInfo {
    type Vtable = ISceSvcAttachmentPersistInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d90e0d0_200d_11d1_affb_00c04fb984f9);
}
impl ::core::convert::From<ISceSvcAttachmentPersistInfo> for ::windows::core::IUnknown {
    fn from(value: ISceSvcAttachmentPersistInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISceSvcAttachmentPersistInfo> for ::windows::core::IUnknown {
    fn from(value: &ISceSvcAttachmentPersistInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISceSvcAttachmentPersistInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISceSvcAttachmentPersistInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceSvcAttachmentPersistInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lptemplatename: *mut i8, scesvchandle: *mut *mut ::core::ffi::c_void, ppvdata: *mut *mut ::core::ffi::c_void, pboverwriteall: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lptemplatename: *mut i8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
pub type PFSCE_FREE_INFO = unsafe extern "system" fn(pvserviceinfo: *mut ::core::ffi::c_void) -> u32;
pub type PFSCE_LOG_INFO = unsafe extern "system" fn(errlevel: SCE_LOG_ERR_LEVEL, win32rc: u32, perrfmt: *mut i8) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFSCE_QUERY_INFO = unsafe extern "system" fn(scehandle: *mut ::core::ffi::c_void, scetype: SCESVC_INFO_TYPE, lpprefix: *mut i8, bexact: super::super::Foundation::BOOL, ppvinfo: *mut *mut ::core::ffi::c_void, psceenumhandle: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFSCE_SET_INFO = unsafe extern "system" fn(scehandle: *mut ::core::ffi::c_void, scetype: SCESVC_INFO_TYPE, lpprefix: *mut i8, bexact: super::super::Foundation::BOOL, pvinfo: *mut ::core::ffi::c_void) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_ConfigAnalyzeService = unsafe extern "system" fn(pscecbinfo: *mut ::core::mem::ManuallyDrop<SCESVC_CALLBACK_INFO>) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_UpdateService = unsafe extern "system" fn(pscecbinfo: *mut ::core::mem::ManuallyDrop<SCESVC_CALLBACK_INFO>, serviceinfo: *mut SCESVC_CONFIGURATION_INFO) -> u32;
pub const SCESTATUS_ACCESS_DENIED: i32 = 9i32;
pub const SCESTATUS_ALREADY_RUNNING: i32 = 13i32;
pub const SCESTATUS_BAD_FORMAT: i32 = 7i32;
pub const SCESTATUS_BUFFER_TOO_SMALL: i32 = 5i32;
pub const SCESTATUS_CANT_DELETE: i32 = 10i32;
pub const SCESTATUS_EXCEPTION_IN_SERVER: i32 = 16i32;
pub const SCESTATUS_INVALID_DATA: i32 = 3i32;
pub const SCESTATUS_INVALID_PARAMETER: i32 = 1i32;
pub const SCESTATUS_MOD_NOT_FOUND: i32 = 15i32;
pub const SCESTATUS_NOT_ENOUGH_RESOURCE: i32 = 8i32;
pub const SCESTATUS_NO_MAPPING: i32 = 18i32;
pub const SCESTATUS_NO_TEMPLATE_GIVEN: i32 = 17i32;
pub const SCESTATUS_OBJECT_EXIST: i32 = 4i32;
pub const SCESTATUS_OTHER_ERROR: i32 = 12i32;
pub const SCESTATUS_PREFIX_OVERFLOW: i32 = 11i32;
pub const SCESTATUS_PROFILE_NOT_FOUND: i32 = 6i32;
pub const SCESTATUS_RECORD_NOT_FOUND: i32 = 2i32;
pub const SCESTATUS_SERVICE_NOT_SUPPORT: i32 = 14i32;
pub const SCESTATUS_SUCCESS: i32 = 0i32;
pub const SCESTATUS_TRUST_FAIL: i32 = 19i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SCESVC_ANALYSIS_INFO {
    pub Count: u32,
    pub Lines: *mut SCESVC_ANALYSIS_LINE,
}
impl SCESVC_ANALYSIS_INFO {}
impl ::core::default::Default for SCESVC_ANALYSIS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SCESVC_ANALYSIS_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SCESVC_ANALYSIS_INFO").field("Count", &self.Count).field("Lines", &self.Lines).finish()
    }
}
impl ::core::cmp::PartialEq for SCESVC_ANALYSIS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Lines == other.Lines
    }
}
impl ::core::cmp::Eq for SCESVC_ANALYSIS_INFO {}
unsafe impl ::windows::core::Abi for SCESVC_ANALYSIS_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SCESVC_ANALYSIS_LINE {
    pub Key: *mut i8,
    pub Value: *mut u8,
    pub ValueLen: u32,
}
impl SCESVC_ANALYSIS_LINE {}
impl ::core::default::Default for SCESVC_ANALYSIS_LINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SCESVC_ANALYSIS_LINE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SCESVC_ANALYSIS_LINE").field("Key", &self.Key).field("Value", &self.Value).field("ValueLen", &self.ValueLen).finish()
    }
}
impl ::core::cmp::PartialEq for SCESVC_ANALYSIS_LINE {
    fn eq(&self, other: &Self) -> bool {
        self.Key == other.Key && self.Value == other.Value && self.ValueLen == other.ValueLen
    }
}
impl ::core::cmp::Eq for SCESVC_ANALYSIS_LINE {}
unsafe impl ::windows::core::Abi for SCESVC_ANALYSIS_LINE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SCESVC_CALLBACK_INFO {
    pub sceHandle: *mut ::core::ffi::c_void,
    pub pfQueryInfo: ::core::option::Option<PFSCE_QUERY_INFO>,
    pub pfSetInfo: ::core::option::Option<PFSCE_SET_INFO>,
    pub pfFreeInfo: ::core::option::Option<PFSCE_FREE_INFO>,
    pub pfLogInfo: ::core::option::Option<PFSCE_LOG_INFO>,
}
#[cfg(feature = "Win32_Foundation")]
impl SCESVC_CALLBACK_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SCESVC_CALLBACK_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SCESVC_CALLBACK_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SCESVC_CALLBACK_INFO").field("sceHandle", &self.sceHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SCESVC_CALLBACK_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.sceHandle == other.sceHandle && self.pfQueryInfo.map(|f| f as usize) == other.pfQueryInfo.map(|f| f as usize) && self.pfSetInfo.map(|f| f as usize) == other.pfSetInfo.map(|f| f as usize) && self.pfFreeInfo.map(|f| f as usize) == other.pfFreeInfo.map(|f| f as usize) && self.pfLogInfo.map(|f| f as usize) == other.pfLogInfo.map(|f| f as usize)
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SCESVC_CALLBACK_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SCESVC_CALLBACK_INFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SCESVC_CONFIGURATION_INFO {
    pub Count: u32,
    pub Lines: *mut SCESVC_CONFIGURATION_LINE,
}
impl SCESVC_CONFIGURATION_INFO {}
impl ::core::default::Default for SCESVC_CONFIGURATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SCESVC_CONFIGURATION_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SCESVC_CONFIGURATION_INFO").field("Count", &self.Count).field("Lines", &self.Lines).finish()
    }
}
impl ::core::cmp::PartialEq for SCESVC_CONFIGURATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Lines == other.Lines
    }
}
impl ::core::cmp::Eq for SCESVC_CONFIGURATION_INFO {}
unsafe impl ::windows::core::Abi for SCESVC_CONFIGURATION_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SCESVC_CONFIGURATION_LINE {
    pub Key: *mut i8,
    pub Value: *mut i8,
    pub ValueLen: u32,
}
impl SCESVC_CONFIGURATION_LINE {}
impl ::core::default::Default for SCESVC_CONFIGURATION_LINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SCESVC_CONFIGURATION_LINE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SCESVC_CONFIGURATION_LINE").field("Key", &self.Key).field("Value", &self.Value).field("ValueLen", &self.ValueLen).finish()
    }
}
impl ::core::cmp::PartialEq for SCESVC_CONFIGURATION_LINE {
    fn eq(&self, other: &Self) -> bool {
        self.Key == other.Key && self.Value == other.Value && self.ValueLen == other.ValueLen
    }
}
impl ::core::cmp::Eq for SCESVC_CONFIGURATION_LINE {}
unsafe impl ::windows::core::Abi for SCESVC_CONFIGURATION_LINE {
    type Abi = Self;
}
pub const SCESVC_ENUMERATION_MAX: i32 = 100i32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SCESVC_INFO_TYPE(pub i32);
pub const SceSvcConfigurationInfo: SCESVC_INFO_TYPE = SCESVC_INFO_TYPE(0i32);
pub const SceSvcMergedPolicyInfo: SCESVC_INFO_TYPE = SCESVC_INFO_TYPE(1i32);
pub const SceSvcAnalysisInfo: SCESVC_INFO_TYPE = SCESVC_INFO_TYPE(2i32);
pub const SceSvcInternalUse: SCESVC_INFO_TYPE = SCESVC_INFO_TYPE(3i32);
impl ::core::convert::From<i32> for SCESVC_INFO_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SCESVC_INFO_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SCE_LOG_ERR_LEVEL(pub u32);
pub const SCE_LOG_LEVEL_ALWAYS: SCE_LOG_ERR_LEVEL = SCE_LOG_ERR_LEVEL(0u32);
pub const SCE_LOG_LEVEL_ERROR: SCE_LOG_ERR_LEVEL = SCE_LOG_ERR_LEVEL(1u32);
pub const SCE_LOG_LEVEL_DETAIL: SCE_LOG_ERR_LEVEL = SCE_LOG_ERR_LEVEL(2u32);
pub const SCE_LOG_LEVEL_DEBUG: SCE_LOG_ERR_LEVEL = SCE_LOG_ERR_LEVEL(3u32);
impl ::core::convert::From<u32> for SCE_LOG_ERR_LEVEL {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SCE_LOG_ERR_LEVEL {
    type Abi = Self;
}
impl ::core::ops::BitOr for SCE_LOG_ERR_LEVEL {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for SCE_LOG_ERR_LEVEL {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for SCE_LOG_ERR_LEVEL {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for SCE_LOG_ERR_LEVEL {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for SCE_LOG_ERR_LEVEL {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const cNodetypeSceAnalysisServices: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x678050c7_1ff8_11d1_affb_00c04fb984f9);
pub const cNodetypeSceEventLog: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ce06698_4bf3_11d1_8c30_00c04fb984f9);
pub const cNodetypeSceTemplateServices: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24a7f717_1f0c_11d1_affb_00c04fb984f9);
