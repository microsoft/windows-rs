#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ISceSvcAttachmentData(::windows::runtime::IUnknown);
impl ISceSvcAttachmentData {
    #[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
    pub unsafe fn GetData(&self, scesvchandle: *mut ::std::ffi::c_void, scetype: SCESVC_INFO_TYPE, ppvdata: *mut *mut ::std::ffi::c_void, psceenumhandle: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(scesvchandle), ::std::mem::transmute(scetype), ::std::mem::transmute(ppvdata), ::std::mem::transmute(psceenumhandle)).ok()
    }
    #[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
    pub unsafe fn Initialize<'a, Param2: ::windows::runtime::IntoParam<'a, ISceSvcAttachmentPersistInfo>>(&self, lpservicename: *mut i8, lptemplatename: *mut i8, lpscesvcpersistinfo: Param2, pscesvchandle: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpservicename), ::std::mem::transmute(lptemplatename), lpscesvcpersistinfo.into_param().abi(), ::std::mem::transmute(pscesvchandle)).ok()
    }
    #[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
    pub unsafe fn FreeBuffer(&self, pvdata: *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pvdata)).ok()
    }
    #[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
    pub unsafe fn CloseHandle(&self, scesvchandle: *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(scesvchandle)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISceSvcAttachmentData {
    type Vtable = ISceSvcAttachmentData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(398680030, 8205, 4561, [175, 251, 0, 192, 79, 185, 132, 249]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceSvcAttachmentData_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scesvchandle: *mut ::std::ffi::c_void, scetype: SCESVC_INFO_TYPE, ppvdata: *mut *mut ::std::ffi::c_void, psceenumhandle: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpservicename: *mut i8, lptemplatename: *mut i8, lpscesvcpersistinfo: ::windows::runtime::RawPtr, pscesvchandle: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvdata: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scesvchandle: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ISceSvcAttachmentPersistInfo(::windows::runtime::IUnknown);
impl ISceSvcAttachmentPersistInfo {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Security_ConfigurationSnapin`, `Win32_Foundation`*"]
    pub unsafe fn Save(&self, lptemplatename: *mut i8, scesvchandle: *mut *mut ::std::ffi::c_void, ppvdata: *mut *mut ::std::ffi::c_void, pboverwriteall: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(lptemplatename), ::std::mem::transmute(scesvchandle), ::std::mem::transmute(ppvdata), ::std::mem::transmute(pboverwriteall)).ok()
    }
    #[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
    pub unsafe fn IsDirty(&self, lptemplatename: *mut i8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(lptemplatename)).ok()
    }
    #[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
    pub unsafe fn FreeBuffer(&self, pvdata: *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pvdata)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISceSvcAttachmentPersistInfo {
    type Vtable = ISceSvcAttachmentPersistInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1838211280, 8205, 4561, [175, 251, 0, 192, 79, 185, 132, 249]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceSvcAttachmentPersistInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lptemplatename: *mut i8, scesvchandle: *mut *mut ::std::ffi::c_void, ppvdata: *mut *mut ::std::ffi::c_void, pboverwriteall: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lptemplatename: *mut i8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvdata: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
pub type PFSCE_FREE_INFO = unsafe extern "system" fn(pvserviceinfo: *mut ::std::ffi::c_void) -> u32;
pub type PFSCE_LOG_INFO = unsafe extern "system" fn(errlevel: SCE_LOG_ERR_LEVEL, win32rc: u32, perrfmt: *mut i8) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFSCE_QUERY_INFO = unsafe extern "system" fn(scehandle: *mut ::std::ffi::c_void, scetype: SCESVC_INFO_TYPE, lpprefix: *mut i8, bexact: super::super::Foundation::BOOL, ppvinfo: *mut *mut ::std::ffi::c_void, psceenumhandle: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFSCE_SET_INFO = unsafe extern "system" fn(scehandle: *mut ::std::ffi::c_void, scetype: SCESVC_INFO_TYPE, lpprefix: *mut i8, bexact: super::super::Foundation::BOOL, pvinfo: *mut ::std::ffi::c_void) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_ConfigAnalyzeService = unsafe extern "system" fn(pscecbinfo: *mut ::std::mem::ManuallyDrop<SCESVC_CALLBACK_INFO>) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_UpdateService = unsafe extern "system" fn(pscecbinfo: *mut ::std::mem::ManuallyDrop<SCESVC_CALLBACK_INFO>, serviceinfo: *mut SCESVC_CONFIGURATION_INFO) -> u32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_ACCESS_DENIED: i32 = 9i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_ALREADY_RUNNING: i32 = 13i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_BAD_FORMAT: i32 = 7i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_BUFFER_TOO_SMALL: i32 = 5i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_CANT_DELETE: i32 = 10i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_EXCEPTION_IN_SERVER: i32 = 16i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_INVALID_DATA: i32 = 3i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_INVALID_PARAMETER: i32 = 1i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_MOD_NOT_FOUND: i32 = 15i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_NOT_ENOUGH_RESOURCE: i32 = 8i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_NO_MAPPING: i32 = 18i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_NO_TEMPLATE_GIVEN: i32 = 17i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_OBJECT_EXIST: i32 = 4i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_OTHER_ERROR: i32 = 12i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_PREFIX_OVERFLOW: i32 = 11i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_PROFILE_NOT_FOUND: i32 = 6i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_RECORD_NOT_FOUND: i32 = 2i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_SERVICE_NOT_SUPPORT: i32 = 14i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_SUCCESS: i32 = 0i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESTATUS_TRUST_FAIL: i32 = 19i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub struct SCESVC_ANALYSIS_INFO {
    pub Count: u32,
    pub Lines: *mut SCESVC_ANALYSIS_LINE,
}
impl SCESVC_ANALYSIS_INFO {}
impl ::std::default::Default for SCESVC_ANALYSIS_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SCESVC_ANALYSIS_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCESVC_ANALYSIS_INFO").field("Count", &self.Count).field("Lines", &self.Lines).finish()
    }
}
impl ::std::cmp::PartialEq for SCESVC_ANALYSIS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Lines == other.Lines
    }
}
impl ::std::cmp::Eq for SCESVC_ANALYSIS_INFO {}
unsafe impl ::windows::runtime::Abi for SCESVC_ANALYSIS_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub struct SCESVC_ANALYSIS_LINE {
    pub Key: *mut i8,
    pub Value: *mut u8,
    pub ValueLen: u32,
}
impl SCESVC_ANALYSIS_LINE {}
impl ::std::default::Default for SCESVC_ANALYSIS_LINE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SCESVC_ANALYSIS_LINE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCESVC_ANALYSIS_LINE").field("Key", &self.Key).field("Value", &self.Value).field("ValueLen", &self.ValueLen).finish()
    }
}
impl ::std::cmp::PartialEq for SCESVC_ANALYSIS_LINE {
    fn eq(&self, other: &Self) -> bool {
        self.Key == other.Key && self.Value == other.Value && self.ValueLen == other.ValueLen
    }
}
impl ::std::cmp::Eq for SCESVC_ANALYSIS_LINE {}
unsafe impl ::windows::runtime::Abi for SCESVC_ANALYSIS_LINE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`, `Win32_Foundation`*"]
pub struct SCESVC_CALLBACK_INFO {
    pub sceHandle: *mut ::std::ffi::c_void,
    pub pfQueryInfo: ::std::option::Option<PFSCE_QUERY_INFO>,
    pub pfSetInfo: ::std::option::Option<PFSCE_SET_INFO>,
    pub pfFreeInfo: ::std::option::Option<PFSCE_FREE_INFO>,
    pub pfLogInfo: ::std::option::Option<PFSCE_LOG_INFO>,
}
#[cfg(feature = "Win32_Foundation")]
impl SCESVC_CALLBACK_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SCESVC_CALLBACK_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SCESVC_CALLBACK_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCESVC_CALLBACK_INFO").field("sceHandle", &self.sceHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SCESVC_CALLBACK_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.sceHandle == other.sceHandle && self.pfQueryInfo.map(|f| f as usize) == other.pfQueryInfo.map(|f| f as usize) && self.pfSetInfo.map(|f| f as usize) == other.pfSetInfo.map(|f| f as usize) && self.pfFreeInfo.map(|f| f as usize) == other.pfFreeInfo.map(|f| f as usize) && self.pfLogInfo.map(|f| f as usize) == other.pfLogInfo.map(|f| f as usize)
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SCESVC_CALLBACK_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SCESVC_CALLBACK_INFO {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub struct SCESVC_CONFIGURATION_INFO {
    pub Count: u32,
    pub Lines: *mut SCESVC_CONFIGURATION_LINE,
}
impl SCESVC_CONFIGURATION_INFO {}
impl ::std::default::Default for SCESVC_CONFIGURATION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SCESVC_CONFIGURATION_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCESVC_CONFIGURATION_INFO").field("Count", &self.Count).field("Lines", &self.Lines).finish()
    }
}
impl ::std::cmp::PartialEq for SCESVC_CONFIGURATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Lines == other.Lines
    }
}
impl ::std::cmp::Eq for SCESVC_CONFIGURATION_INFO {}
unsafe impl ::windows::runtime::Abi for SCESVC_CONFIGURATION_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub struct SCESVC_CONFIGURATION_LINE {
    pub Key: *mut i8,
    pub Value: *mut i8,
    pub ValueLen: u32,
}
impl SCESVC_CONFIGURATION_LINE {}
impl ::std::default::Default for SCESVC_CONFIGURATION_LINE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SCESVC_CONFIGURATION_LINE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCESVC_CONFIGURATION_LINE").field("Key", &self.Key).field("Value", &self.Value).field("ValueLen", &self.ValueLen).finish()
    }
}
impl ::std::cmp::PartialEq for SCESVC_CONFIGURATION_LINE {
    fn eq(&self, other: &Self) -> bool {
        self.Key == other.Key && self.Value == other.Value && self.ValueLen == other.ValueLen
    }
}
impl ::std::cmp::Eq for SCESVC_CONFIGURATION_LINE {}
unsafe impl ::windows::runtime::Abi for SCESVC_CONFIGURATION_LINE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
pub const SCESVC_ENUMERATION_MAX: i32 = 100i32;
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SCESVC_INFO_TYPE(pub i32);
pub const SceSvcConfigurationInfo: SCESVC_INFO_TYPE = SCESVC_INFO_TYPE(0i32);
pub const SceSvcMergedPolicyInfo: SCESVC_INFO_TYPE = SCESVC_INFO_TYPE(1i32);
pub const SceSvcAnalysisInfo: SCESVC_INFO_TYPE = SCESVC_INFO_TYPE(2i32);
pub const SceSvcInternalUse: SCESVC_INFO_TYPE = SCESVC_INFO_TYPE(3i32);
impl ::std::convert::From<i32> for SCESVC_INFO_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SCESVC_INFO_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Security_ConfigurationSnapin`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SCE_LOG_ERR_LEVEL(pub u32);
pub const SCE_LOG_LEVEL_ALWAYS: SCE_LOG_ERR_LEVEL = SCE_LOG_ERR_LEVEL(0u32);
pub const SCE_LOG_LEVEL_ERROR: SCE_LOG_ERR_LEVEL = SCE_LOG_ERR_LEVEL(1u32);
pub const SCE_LOG_LEVEL_DETAIL: SCE_LOG_ERR_LEVEL = SCE_LOG_ERR_LEVEL(2u32);
pub const SCE_LOG_LEVEL_DEBUG: SCE_LOG_ERR_LEVEL = SCE_LOG_ERR_LEVEL(3u32);
impl ::std::convert::From<u32> for SCE_LOG_ERR_LEVEL {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SCE_LOG_ERR_LEVEL {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for SCE_LOG_ERR_LEVEL {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SCE_LOG_ERR_LEVEL {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SCE_LOG_ERR_LEVEL {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SCE_LOG_ERR_LEVEL {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SCE_LOG_ERR_LEVEL {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const cNodetypeSceAnalysisServices: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1736462535, 8184, 4561, [175, 251, 0, 192, 79, 185, 132, 249]);
pub const cNodetypeSceEventLog: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(752903832, 19443, 4561, [140, 48, 0, 192, 79, 185, 132, 249]);
pub const cNodetypeSceTemplateServices: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(614987543, 7948, 4561, [175, 251, 0, 192, 79, 185, 132, 249]);
