#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
#[repr(transparent)]
pub struct ISceSvcAttachmentData(::windows::core::IUnknown);
impl ISceSvcAttachmentData {
    #[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
    pub unsafe fn GetData(&self, scesvchandle: *mut ::core::ffi::c_void, scetype: SCESVC_INFO_TYPE, ppvdata: *mut *mut ::core::ffi::c_void, psceenumhandle: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(scesvchandle), ::core::mem::transmute(scetype), ::core::mem::transmute(ppvdata), ::core::mem::transmute(psceenumhandle)).ok()
    }
    #[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
    pub unsafe fn Initialize<'a, Param2: ::windows::core::IntoParam<'a, ISceSvcAttachmentPersistInfo>>(&self, lpservicename: *mut i8, lptemplatename: *mut i8, lpscesvcpersistinfo: Param2, pscesvchandle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpservicename), ::core::mem::transmute(lptemplatename), lpscesvcpersistinfo.into_param().abi(), ::core::mem::transmute(pscesvchandle)).ok()
    }
    #[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
    pub unsafe fn FreeBuffer(&self, pvdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvdata)).ok()
    }
    #[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
    pub unsafe fn CloseHandle(&self, scesvchandle: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(scesvchandle)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISceSvcAttachmentData {
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
unsafe impl ::windows::core::Interface for ISceSvcAttachmentData {
    type Vtable = ISceSvcAttachmentDataVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17c35fde_200d_11d1_affb_00c04fb984f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceSvcAttachmentDataVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scesvchandle: *mut ::core::ffi::c_void, scetype: SCESVC_INFO_TYPE, ppvdata: *mut *mut ::core::ffi::c_void, psceenumhandle: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpservicename: *mut i8, lptemplatename: *mut i8, lpscesvcpersistinfo: ::windows::core::RawPtr, pscesvchandle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scesvchandle: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
#[repr(transparent)]
pub struct ISceSvcAttachmentPersistInfo(::windows::core::IUnknown);
impl ISceSvcAttachmentPersistInfo {
    #[doc = "*Required features: 'Win32_Security_ConfigurationSnapin', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save(&self, lptemplatename: *mut i8, scesvchandle: *mut *mut ::core::ffi::c_void, ppvdata: *mut *mut ::core::ffi::c_void, pboverwriteall: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lptemplatename), ::core::mem::transmute(scesvchandle), ::core::mem::transmute(ppvdata), ::core::mem::transmute(pboverwriteall)).ok()
    }
    #[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
    pub unsafe fn IsDirty(&self, lptemplatename: *mut i8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(lptemplatename)).ok()
    }
    #[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
    pub unsafe fn FreeBuffer(&self, pvdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvdata)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISceSvcAttachmentPersistInfo {
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
unsafe impl ::windows::core::Interface for ISceSvcAttachmentPersistInfo {
    type Vtable = ISceSvcAttachmentPersistInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d90e0d0_200d_11d1_affb_00c04fb984f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceSvcAttachmentPersistInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lptemplatename: *mut i8, scesvchandle: *mut *mut ::core::ffi::c_void, ppvdata: *mut *mut ::core::ffi::c_void, pboverwriteall: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lptemplatename: *mut i8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub type PFSCE_FREE_INFO = ::core::option::Option<unsafe extern "system" fn(pvserviceinfo: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub type PFSCE_LOG_INFO = ::core::option::Option<unsafe extern "system" fn(errlevel: SCE_LOG_ERR_LEVEL, win32rc: u32, perrfmt: *mut i8) -> u32>;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFSCE_QUERY_INFO = ::core::option::Option<unsafe extern "system" fn(scehandle: *mut ::core::ffi::c_void, scetype: SCESVC_INFO_TYPE, lpprefix: *mut i8, bexact: super::super::Foundation::BOOL, ppvinfo: *mut *mut ::core::ffi::c_void, psceenumhandle: *mut u32) -> u32>;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFSCE_SET_INFO = ::core::option::Option<unsafe extern "system" fn(scehandle: *mut ::core::ffi::c_void, scetype: SCESVC_INFO_TYPE, lpprefix: *mut i8, bexact: super::super::Foundation::BOOL, pvinfo: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_ConfigAnalyzeService = ::core::option::Option<unsafe extern "system" fn(pscecbinfo: *mut SCESVC_CALLBACK_INFO) -> u32>;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_UpdateService = ::core::option::Option<unsafe extern "system" fn(pscecbinfo: *mut SCESVC_CALLBACK_INFO, serviceinfo: *mut SCESVC_CONFIGURATION_INFO) -> u32>;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub const SCESTATUS_ACCESS_DENIED: i32 = 9i32;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub const SCESTATUS_ALREADY_RUNNING: i32 = 13i32;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub const SCESTATUS_BAD_FORMAT: i32 = 7i32;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub const SCESTATUS_BUFFER_TOO_SMALL: i32 = 5i32;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub const SCESTATUS_CANT_DELETE: i32 = 10i32;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub const SCESTATUS_EXCEPTION_IN_SERVER: i32 = 16i32;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub const SCESTATUS_INVALID_DATA: i32 = 3i32;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub const SCESTATUS_INVALID_PARAMETER: i32 = 1i32;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub const SCESTATUS_MOD_NOT_FOUND: i32 = 15i32;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub const SCESTATUS_NOT_ENOUGH_RESOURCE: i32 = 8i32;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub const SCESTATUS_NO_MAPPING: i32 = 18i32;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub const SCESTATUS_NO_TEMPLATE_GIVEN: i32 = 17i32;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub const SCESTATUS_OBJECT_EXIST: i32 = 4i32;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub const SCESTATUS_OTHER_ERROR: i32 = 12i32;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub const SCESTATUS_PREFIX_OVERFLOW: i32 = 11i32;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub const SCESTATUS_PROFILE_NOT_FOUND: i32 = 6i32;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub const SCESTATUS_RECORD_NOT_FOUND: i32 = 2i32;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub const SCESTATUS_SERVICE_NOT_SUPPORT: i32 = 14i32;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub const SCESTATUS_SUCCESS: i32 = 0i32;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub const SCESTATUS_TRUST_FAIL: i32 = 19i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
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
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
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
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin', 'Win32_Foundation'*"]
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
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
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
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
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
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub const SCESVC_ENUMERATION_MAX: i32 = 100i32;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub type SCESVC_INFO_TYPE = i32;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub const SceSvcConfigurationInfo: SCESVC_INFO_TYPE = 0i32;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub const SceSvcMergedPolicyInfo: SCESVC_INFO_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub const SceSvcAnalysisInfo: SCESVC_INFO_TYPE = 2i32;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub const SceSvcInternalUse: SCESVC_INFO_TYPE = 3i32;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub type SCE_LOG_ERR_LEVEL = u32;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub const SCE_LOG_LEVEL_ALWAYS: SCE_LOG_ERR_LEVEL = 0u32;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub const SCE_LOG_LEVEL_ERROR: SCE_LOG_ERR_LEVEL = 1u32;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub const SCE_LOG_LEVEL_DETAIL: SCE_LOG_ERR_LEVEL = 2u32;
#[doc = "*Required features: 'Win32_Security_ConfigurationSnapin'*"]
pub const SCE_LOG_LEVEL_DEBUG: SCE_LOG_ERR_LEVEL = 3u32;
pub const cNodetypeSceAnalysisServices: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x678050c7_1ff8_11d1_affb_00c04fb984f9);
pub const cNodetypeSceEventLog: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ce06698_4bf3_11d1_8c30_00c04fb984f9);
pub const cNodetypeSceTemplateServices: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24a7f717_1f0c_11d1_affb_00c04fb984f9);
