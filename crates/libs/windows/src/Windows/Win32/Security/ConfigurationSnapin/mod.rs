#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const CCF_SCESVC_ATTACHMENT: &'static str = "CCF_SCESVC_ATTACHMENT";
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const CCF_SCESVC_ATTACHMENT_DATA: &'static str = "CCF_SCESVC_ATTACHMENT_DATA";
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
#[repr(transparent)]
pub struct ISceSvcAttachmentData(::windows::core::IUnknown);
impl ISceSvcAttachmentData {
    #[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
    pub unsafe fn GetData(&self, scesvchandle: *mut ::core::ffi::c_void, scetype: SCESVC_INFO_TYPE, ppvdata: *mut *mut ::core::ffi::c_void, psceenumhandle: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetData)(::core::mem::transmute_copy(self), ::core::mem::transmute(scesvchandle), ::core::mem::transmute(scetype), ::core::mem::transmute(ppvdata), ::core::mem::transmute(psceenumhandle)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
    pub unsafe fn Initialize<'a, Param2: ::windows::core::IntoParam<'a, ISceSvcAttachmentPersistInfo>>(&self, lpservicename: *mut i8, lptemplatename: *mut i8, lpscesvcpersistinfo: Param2, pscesvchandle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpservicename), ::core::mem::transmute(lptemplatename), lpscesvcpersistinfo.into_param().abi(), ::core::mem::transmute(pscesvchandle)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
    pub unsafe fn FreeBuffer(&self, pvdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FreeBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
    pub unsafe fn CloseHandle(&self, scesvchandle: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CloseHandle)(::core::mem::transmute_copy(self), ::core::mem::transmute(scesvchandle)).ok()
    }
}
impl ::core::convert::From<ISceSvcAttachmentData> for ::windows::core::IUnknown {
    fn from(value: ISceSvcAttachmentData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISceSvcAttachmentData> for ::windows::core::IUnknown {
    fn from(value: &ISceSvcAttachmentData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISceSvcAttachmentData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISceSvcAttachmentData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISceSvcAttachmentData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISceSvcAttachmentData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISceSvcAttachmentData {}
impl ::core::fmt::Debug for ISceSvcAttachmentData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISceSvcAttachmentData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISceSvcAttachmentData {
    type Vtable = ISceSvcAttachmentData_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17c35fde_200d_11d1_affb_00c04fb984f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceSvcAttachmentData_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scesvchandle: *mut ::core::ffi::c_void, scetype: SCESVC_INFO_TYPE, ppvdata: *mut *mut ::core::ffi::c_void, psceenumhandle: *mut u32) -> ::windows::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpservicename: *mut i8, lptemplatename: *mut i8, lpscesvcpersistinfo: ::windows::core::RawPtr, pscesvchandle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FreeBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CloseHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scesvchandle: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
#[repr(transparent)]
pub struct ISceSvcAttachmentPersistInfo(::windows::core::IUnknown);
impl ISceSvcAttachmentPersistInfo {
    #[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save(&self, lptemplatename: *mut i8, scesvchandle: *mut *mut ::core::ffi::c_void, ppvdata: *mut *mut ::core::ffi::c_void, pboverwriteall: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Save)(::core::mem::transmute_copy(self), ::core::mem::transmute(lptemplatename), ::core::mem::transmute(scesvchandle), ::core::mem::transmute(ppvdata), ::core::mem::transmute(pboverwriteall)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
    pub unsafe fn IsDirty(&self, lptemplatename: *mut i8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsDirty)(::core::mem::transmute_copy(self), ::core::mem::transmute(lptemplatename)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
    pub unsafe fn FreeBuffer(&self, pvdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FreeBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvdata)).ok()
    }
}
impl ::core::convert::From<ISceSvcAttachmentPersistInfo> for ::windows::core::IUnknown {
    fn from(value: ISceSvcAttachmentPersistInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISceSvcAttachmentPersistInfo> for ::windows::core::IUnknown {
    fn from(value: &ISceSvcAttachmentPersistInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISceSvcAttachmentPersistInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISceSvcAttachmentPersistInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISceSvcAttachmentPersistInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISceSvcAttachmentPersistInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISceSvcAttachmentPersistInfo {}
impl ::core::fmt::Debug for ISceSvcAttachmentPersistInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISceSvcAttachmentPersistInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISceSvcAttachmentPersistInfo {
    type Vtable = ISceSvcAttachmentPersistInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d90e0d0_200d_11d1_affb_00c04fb984f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceSvcAttachmentPersistInfo_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lptemplatename: *mut i8, scesvchandle: *mut *mut ::core::ffi::c_void, ppvdata: *mut *mut ::core::ffi::c_void, pboverwriteall: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Save: usize,
    pub IsDirty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lptemplatename: *mut i8) -> ::windows::core::HRESULT,
    pub FreeBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub type PFSCE_FREE_INFO = ::core::option::Option<unsafe extern "system" fn(pvserviceinfo: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub type PFSCE_LOG_INFO = ::core::option::Option<unsafe extern "system" fn(errlevel: SCE_LOG_ERR_LEVEL, win32rc: u32, perrfmt: *mut i8) -> u32>;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFSCE_QUERY_INFO = ::core::option::Option<unsafe extern "system" fn(scehandle: *mut ::core::ffi::c_void, scetype: SCESVC_INFO_TYPE, lpprefix: *mut i8, bexact: super::super::Foundation::BOOL, ppvinfo: *mut *mut ::core::ffi::c_void, psceenumhandle: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFSCE_SET_INFO = ::core::option::Option<unsafe extern "system" fn(scehandle: *mut ::core::ffi::c_void, scetype: SCESVC_INFO_TYPE, lpprefix: *mut i8, bexact: super::super::Foundation::BOOL, pvinfo: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_ConfigAnalyzeService = ::core::option::Option<unsafe extern "system" fn(pscecbinfo: *mut SCESVC_CALLBACK_INFO) -> u32>;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_UpdateService = ::core::option::Option<unsafe extern "system" fn(pscecbinfo: *mut SCESVC_CALLBACK_INFO, serviceinfo: *mut SCESVC_CONFIGURATION_INFO) -> u32>;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_ACCESS_DENIED: i32 = 9i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_ALREADY_RUNNING: i32 = 13i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_BAD_FORMAT: i32 = 7i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_BUFFER_TOO_SMALL: i32 = 5i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_CANT_DELETE: i32 = 10i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_EXCEPTION_IN_SERVER: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_INVALID_DATA: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_INVALID_PARAMETER: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_MOD_NOT_FOUND: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_NOT_ENOUGH_RESOURCE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_NO_MAPPING: i32 = 18i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_NO_TEMPLATE_GIVEN: i32 = 17i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_OBJECT_EXIST: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_OTHER_ERROR: i32 = 12i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_PREFIX_OVERFLOW: i32 = 11i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_PROFILE_NOT_FOUND: i32 = 6i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_RECORD_NOT_FOUND: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_SERVICE_NOT_SUPPORT: i32 = 14i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_SUCCESS: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESTATUS_TRUST_FAIL: i32 = 19i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub struct SCESVC_ANALYSIS_INFO {
    pub Count: u32,
    pub Lines: *mut SCESVC_ANALYSIS_LINE,
}
impl ::core::marker::Copy for SCESVC_ANALYSIS_INFO {}
impl ::core::clone::Clone for SCESVC_ANALYSIS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCESVC_ANALYSIS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCESVC_ANALYSIS_INFO").field("Count", &self.Count).field("Lines", &self.Lines).finish()
    }
}
unsafe impl ::windows::core::Abi for SCESVC_ANALYSIS_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCESVC_ANALYSIS_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCESVC_ANALYSIS_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCESVC_ANALYSIS_INFO {}
impl ::core::default::Default for SCESVC_ANALYSIS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub struct SCESVC_ANALYSIS_LINE {
    pub Key: *mut i8,
    pub Value: *mut u8,
    pub ValueLen: u32,
}
impl ::core::marker::Copy for SCESVC_ANALYSIS_LINE {}
impl ::core::clone::Clone for SCESVC_ANALYSIS_LINE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCESVC_ANALYSIS_LINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCESVC_ANALYSIS_LINE").field("Key", &self.Key).field("Value", &self.Value).field("ValueLen", &self.ValueLen).finish()
    }
}
unsafe impl ::windows::core::Abi for SCESVC_ANALYSIS_LINE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCESVC_ANALYSIS_LINE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCESVC_ANALYSIS_LINE>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCESVC_ANALYSIS_LINE {}
impl ::core::default::Default for SCESVC_ANALYSIS_LINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SCESVC_CALLBACK_INFO {
    pub sceHandle: *mut ::core::ffi::c_void,
    pub pfQueryInfo: PFSCE_QUERY_INFO,
    pub pfSetInfo: PFSCE_SET_INFO,
    pub pfFreeInfo: PFSCE_FREE_INFO,
    pub pfLogInfo: PFSCE_LOG_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SCESVC_CALLBACK_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SCESVC_CALLBACK_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SCESVC_CALLBACK_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCESVC_CALLBACK_INFO").field("sceHandle", &self.sceHandle).field("pfQueryInfo", &self.pfQueryInfo.map(|f| f as usize)).field("pfSetInfo", &self.pfSetInfo.map(|f| f as usize)).field("pfFreeInfo", &self.pfFreeInfo.map(|f| f as usize)).field("pfLogInfo", &self.pfLogInfo.map(|f| f as usize)).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SCESVC_CALLBACK_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SCESVC_CALLBACK_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCESVC_CALLBACK_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SCESVC_CALLBACK_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SCESVC_CALLBACK_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub struct SCESVC_CONFIGURATION_INFO {
    pub Count: u32,
    pub Lines: *mut SCESVC_CONFIGURATION_LINE,
}
impl ::core::marker::Copy for SCESVC_CONFIGURATION_INFO {}
impl ::core::clone::Clone for SCESVC_CONFIGURATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCESVC_CONFIGURATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCESVC_CONFIGURATION_INFO").field("Count", &self.Count).field("Lines", &self.Lines).finish()
    }
}
unsafe impl ::windows::core::Abi for SCESVC_CONFIGURATION_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCESVC_CONFIGURATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCESVC_CONFIGURATION_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCESVC_CONFIGURATION_INFO {}
impl ::core::default::Default for SCESVC_CONFIGURATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub struct SCESVC_CONFIGURATION_LINE {
    pub Key: *mut i8,
    pub Value: *mut i8,
    pub ValueLen: u32,
}
impl ::core::marker::Copy for SCESVC_CONFIGURATION_LINE {}
impl ::core::clone::Clone for SCESVC_CONFIGURATION_LINE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCESVC_CONFIGURATION_LINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCESVC_CONFIGURATION_LINE").field("Key", &self.Key).field("Value", &self.Value).field("ValueLen", &self.ValueLen).finish()
    }
}
unsafe impl ::windows::core::Abi for SCESVC_CONFIGURATION_LINE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCESVC_CONFIGURATION_LINE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCESVC_CONFIGURATION_LINE>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCESVC_CONFIGURATION_LINE {}
impl ::core::default::Default for SCESVC_CONFIGURATION_LINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCESVC_ENUMERATION_MAX: i32 = 100i32;
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SCESVC_INFO_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SceSvcConfigurationInfo: SCESVC_INFO_TYPE = SCESVC_INFO_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SceSvcMergedPolicyInfo: SCESVC_INFO_TYPE = SCESVC_INFO_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SceSvcAnalysisInfo: SCESVC_INFO_TYPE = SCESVC_INFO_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SceSvcInternalUse: SCESVC_INFO_TYPE = SCESVC_INFO_TYPE(3i32);
impl ::core::marker::Copy for SCESVC_INFO_TYPE {}
impl ::core::clone::Clone for SCESVC_INFO_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCESVC_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SCESVC_INFO_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SCESVC_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCESVC_INFO_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SCE_LOG_ERR_LEVEL(pub u32);
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCE_LOG_LEVEL_ALWAYS: SCE_LOG_ERR_LEVEL = SCE_LOG_ERR_LEVEL(0u32);
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCE_LOG_LEVEL_ERROR: SCE_LOG_ERR_LEVEL = SCE_LOG_ERR_LEVEL(1u32);
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCE_LOG_LEVEL_DETAIL: SCE_LOG_ERR_LEVEL = SCE_LOG_ERR_LEVEL(2u32);
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCE_LOG_LEVEL_DEBUG: SCE_LOG_ERR_LEVEL = SCE_LOG_ERR_LEVEL(3u32);
impl ::core::marker::Copy for SCE_LOG_ERR_LEVEL {}
impl ::core::clone::Clone for SCE_LOG_ERR_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCE_LOG_ERR_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SCE_LOG_ERR_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for SCE_LOG_ERR_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCE_LOG_ERR_LEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const SCE_ROOT_PATH: &'static str = "Software\\Microsoft\\Windows NT\\CurrentVersion\\SeCEdit";
pub const cNodetypeSceAnalysisServices: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x678050c7_1ff8_11d1_affb_00c04fb984f9);
pub const cNodetypeSceEventLog: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ce06698_4bf3_11d1_8c30_00c04fb984f9);
pub const cNodetypeSceTemplateServices: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24a7f717_1f0c_11d1_affb_00c04fb984f9);
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const lstruuidNodetypeSceAnalysisServices: &'static str = "{678050c7-1ff8-11d1-affb-00c04fb984f9}";
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const lstruuidNodetypeSceEventLog: &'static str = "{2ce06698-4bf3-11d1-8c30-00c04fb984f9}";
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const lstruuidNodetypeSceTemplateServices: &'static str = "{24a7f717-1f0c-11d1-affb-00c04fb984f9}";
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const struuidNodetypeSceAnalysisServices: &'static str = "{678050c7-1ff8-11d1-affb-00c04fb984f9}";
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const struuidNodetypeSceEventLog: &'static str = "{2ce06698-4bf3-11d1-8c30-00c04fb984f9}";
#[doc = "*Required features: `\"Win32_Security_ConfigurationSnapin\"`*"]
pub const struuidNodetypeSceTemplateServices: &'static str = "{24a7f717-1f0c-11d1-affb-00c04fb984f9}";
#[cfg(feature = "implement")]
::core::include!("impl.rs");
