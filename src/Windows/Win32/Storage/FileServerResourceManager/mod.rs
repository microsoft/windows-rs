#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub const AdSyncTask: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ae64751_b728_4d6b_97a0_b2da2e7d2a3b);
pub type AdrClientDisplayFlags = i32;
pub const AdrClientDisplayFlags_AllowEmailRequests: AdrClientDisplayFlags = 1i32;
pub const AdrClientDisplayFlags_ShowDeviceTroubleshooting: AdrClientDisplayFlags = 2i32;
pub type AdrClientErrorType = i32;
pub const AdrClientErrorType_Unknown: AdrClientErrorType = 0i32;
pub const AdrClientErrorType_AccessDenied: AdrClientErrorType = 1i32;
pub const AdrClientErrorType_FileNotFound: AdrClientErrorType = 2i32;
pub type AdrClientFlags = i32;
pub const AdrClientFlags_None: AdrClientFlags = 0i32;
pub const AdrClientFlags_FailForLocalPaths: AdrClientFlags = 1i32;
pub const AdrClientFlags_FailIfNotSupportedByServer: AdrClientFlags = 2i32;
pub const AdrClientFlags_FailIfNotDomainJoined: AdrClientFlags = 4i32;
pub type AdrEmailFlags = i32;
pub const AdrEmailFlags_PutDataOwnerOnToLine: AdrEmailFlags = 1i32;
pub const AdrEmailFlags_PutAdminOnToLine: AdrEmailFlags = 2i32;
pub const AdrEmailFlags_IncludeDeviceClaims: AdrEmailFlags = 4i32;
pub const AdrEmailFlags_IncludeUserInfo: AdrEmailFlags = 8i32;
pub const AdrEmailFlags_GenerateEventLog: AdrEmailFlags = 16i32;
#[repr(transparent)]
pub struct DIFsrmClassificationEvents(::windows::core::IUnknown);
impl DIFsrmClassificationEvents {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DIFsrmClassificationEvents> for super::super::System::Com::IDispatch {
    fn from(value: DIFsrmClassificationEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DIFsrmClassificationEvents> for super::super::System::Com::IDispatch {
    fn from(value: &DIFsrmClassificationEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for DIFsrmClassificationEvents {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &DIFsrmClassificationEvents {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DIFsrmClassificationEvents> for ::windows::core::IUnknown {
    fn from(value: DIFsrmClassificationEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DIFsrmClassificationEvents> for ::windows::core::IUnknown {
    fn from(value: &DIFsrmClassificationEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DIFsrmClassificationEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DIFsrmClassificationEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for DIFsrmClassificationEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DIFsrmClassificationEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DIFsrmClassificationEvents {}
unsafe impl ::windows::core::Interface for DIFsrmClassificationEvents {
    type Vtable = DIFsrmClassificationEventsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26942db0_dabf_41d8_bbdd_b129a9f70424);
}
#[repr(C)]
#[doc(hidden)]
pub struct DIFsrmClassificationEventsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
pub const FSRM_DISPID_FEATURE_CLASSIFICATION: u32 = 83886080u32;
pub const FSRM_DISPID_FEATURE_FILESCREEN: u32 = 50331648u32;
pub const FSRM_DISPID_FEATURE_GENERAL: u32 = 16777216u32;
pub const FSRM_DISPID_FEATURE_MASK: u32 = 251658240u32;
pub const FSRM_DISPID_FEATURE_PIPELINE: u32 = 100663296u32;
pub const FSRM_DISPID_FEATURE_QUOTA: u32 = 33554432u32;
pub const FSRM_DISPID_FEATURE_REPORTS: u32 = 67108864u32;
pub const FSRM_DISPID_INTERFACE_A_MASK: u32 = 15728640u32;
pub const FSRM_DISPID_INTERFACE_B_MASK: u32 = 983040u32;
pub const FSRM_DISPID_INTERFACE_C_MASK: u32 = 61440u32;
pub const FSRM_DISPID_INTERFACE_D_MASK: u32 = 3840u32;
pub const FSRM_DISPID_IS_PROPERTY: u32 = 128u32;
pub const FSRM_DISPID_METHOD_NUM_MASK: u32 = 127u32;
pub const FSRM_E_ADR_MAX_EMAILS_SENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200130i32);
pub const FSRM_E_ADR_NOT_DOMAIN_JOINED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200110i32);
pub const FSRM_E_ADR_PATH_IS_LOCAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200111i32);
pub const FSRM_E_ADR_SRV_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200112i32);
pub const FSRM_E_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200253i32);
pub const FSRM_E_AUTO_QUOTA: ::windows::core::HRESULT = ::windows::core::HRESULT(283419i32);
pub const FSRM_E_CACHE_INVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200187i32);
pub const FSRM_E_CACHE_MODULE_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200186i32);
pub const FSRM_E_CANNOT_AGGREGATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200201i32);
pub const FSRM_E_CANNOT_ALLOW_REPARSE_POINT_TAG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200170i32);
pub const FSRM_E_CANNOT_CHANGE_PROPERTY_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200197i32);
pub const FSRM_E_CANNOT_CREATE_TEMP_COPY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200132i32);
pub const FSRM_E_CANNOT_DELETE_SYSTEM_PROPERTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200135i32);
pub const FSRM_E_CANNOT_REMOVE_READONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200109i32);
pub const FSRM_E_CANNOT_RENAME_PROPERTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200198i32);
pub const FSRM_E_CANNOT_STORE_PROPERTIES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200171i32);
pub const FSRM_E_CANNOT_USE_DELETED_PROPERTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200143i32);
pub const FSRM_E_CANNOT_USE_DEPRECATED_PROPERTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200145i32);
pub const FSRM_E_CLASSIFICATION_ALREADY_RUNNING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200195i32);
pub const FSRM_E_CLASSIFICATION_CANCELED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200141i32);
pub const FSRM_E_CLASSIFICATION_NOT_RUNNING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200194i32);
pub const FSRM_E_CLASSIFICATION_PARTIAL_BATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200136i32);
pub const FSRM_E_CLASSIFICATION_SCAN_FAIL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200148i32);
pub const FSRM_E_CLASSIFICATION_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200137i32);
pub const FSRM_E_CLUSTER_NOT_RUNNING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200210i32);
pub const FSRM_E_CSC_PATH_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200106i32);
pub const FSRM_E_DIFFERENT_CLUSTER_GROUP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200207i32);
pub const FSRM_E_DRIVER_NOT_READY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200237i32);
pub const FSRM_E_DUPLICATE_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200240i32);
pub const FSRM_E_EMAIL_NOT_SENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200228i32);
pub const FSRM_E_ENUM_PROPERTIES_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200173i32);
pub const FSRM_E_ERROR_NOT_ENABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200133i32);
pub const FSRM_E_EXPIRATION_PATH_NOT_WRITEABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200105i32);
pub const FSRM_E_EXPIRATION_PATH_TOO_LONG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200104i32);
pub const FSRM_E_EXPIRATION_VOLUME_NOT_NTFS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200103i32);
pub const FSRM_E_FAIL_BATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200247i32);
pub const FSRM_E_FILE_ENCRYPTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200156i32);
pub const FSRM_E_FILE_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200134i32);
pub const FSRM_E_FILE_MANAGEMENT_ACTION_GET_EXITCODE_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200152i32);
pub const FSRM_E_FILE_MANAGEMENT_ACTION_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200153i32);
pub const FSRM_E_FILE_MANAGEMENT_EXPIRATION_DIR_IN_SCOPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200185i32);
pub const FSRM_E_FILE_MANAGEMENT_JOB_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200184i32);
pub const FSRM_E_FILE_MANAGEMENT_JOB_ALREADY_RUNNING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200193i32);
pub const FSRM_E_FILE_MANAGEMENT_JOB_CUSTOM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200191i32);
pub const FSRM_E_FILE_MANAGEMENT_JOB_DEPRECATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200102i32);
pub const FSRM_E_FILE_MANAGEMENT_JOB_EXPIRATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200192i32);
pub const FSRM_E_FILE_MANAGEMENT_JOB_INVALID_CONTINUOUS_CONFIG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200108i32);
pub const FSRM_E_FILE_MANAGEMENT_JOB_MAX_FILE_CONDITIONS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200146i32);
pub const FSRM_E_FILE_MANAGEMENT_JOB_NOTIFICATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200190i32);
pub const FSRM_E_FILE_MANAGEMENT_JOB_NOT_LEGACY_ACCESSIBLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200147i32);
pub const FSRM_E_FILE_MANAGEMENT_JOB_RMS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200120i32);
pub const FSRM_E_FILE_OPEN_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200189i32);
pub const FSRM_E_FILE_SYSTEM_CORRUPT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200225i32);
pub const FSRM_E_INCOMPATIBLE_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200157i32);
pub const FSRM_E_INPROC_MODULE_BLOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200174i32);
pub const FSRM_E_INSECURE_PATH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200233i32);
pub const FSRM_E_INSUFFICIENT_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200236i32);
pub const FSRM_E_INVALID_AD_CLAIM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200142i32);
pub const FSRM_E_INVALID_COMBINATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200241i32);
pub const FSRM_E_INVALID_DATASCREEN_DEFINITION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200220i32);
pub const FSRM_E_INVALID_EMAIL_ADDRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200226i32);
pub const FSRM_E_INVALID_FILEGROUP_DEFINITION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200223i32);
pub const FSRM_E_INVALID_FILENAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200214i32);
pub const FSRM_E_INVALID_FOLDER_PROPERTY_STORE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200140i32);
pub const FSRM_E_INVALID_IMPORT_VERSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200245i32);
pub const FSRM_E_INVALID_LIMIT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200249i32);
pub const FSRM_E_INVALID_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200248i32);
pub const FSRM_E_INVALID_PATH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200250i32);
pub const FSRM_E_INVALID_REPORT_DESC: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200215i32);
pub const FSRM_E_INVALID_REPORT_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200216i32);
pub const FSRM_E_INVALID_SCHEDULER_ARGUMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200254i32);
pub const FSRM_E_INVALID_SMTP_SERVER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200232i32);
pub const FSRM_E_INVALID_TEXT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200246i32);
pub const FSRM_E_INVALID_USER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200251i32);
pub const FSRM_E_LAST_ACCESS_UPDATE_DISABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200176i32);
pub const FSRM_E_LEGACY_SCHEDULE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200107i32);
pub const FSRM_E_LOADING_DISABLED_MODULE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200202i32);
pub const FSRM_E_LONG_CMDLINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200224i32);
pub const FSRM_E_MAX_PROPERTY_DEFINITIONS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200196i32);
pub const FSRM_E_MESSAGE_LIMIT_EXCEEDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200200i32);
pub const FSRM_E_MODULE_INITIALIZATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200150i32);
pub const FSRM_E_MODULE_INVALID_PARAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200151i32);
pub const FSRM_E_MODULE_SESSION_INITIALIZATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200149i32);
pub const FSRM_E_MODULE_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200101i32);
pub const FSRM_E_NOT_CLUSTER_VOLUME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200208i32);
pub const FSRM_E_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200255i32);
pub const FSRM_E_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200239i32);
pub const FSRM_E_NO_EMAIL_ADDRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200131i32);
pub const FSRM_E_NO_PROPERTY_VALUE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200175i32);
pub const FSRM_E_OBJECT_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200199i32);
pub const FSRM_E_OUT_OF_RANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200243i32);
pub const FSRM_E_PARTIAL_CLASSIFICATION_PROPERTY_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200169i32);
pub const FSRM_E_PATH_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200252i32);
pub const FSRM_E_PATH_NOT_IN_NAMESPACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200129i32);
pub const FSRM_E_PERSIST_PROPERTIES_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200155i32);
pub const FSRM_E_PERSIST_PROPERTIES_FAILED_ENCRYPTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200166i32);
pub const FSRM_E_PROPERTY_DELETED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200183i32);
pub const FSRM_E_PROPERTY_MUST_APPLY_TO_FILES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200138i32);
pub const FSRM_E_PROPERTY_MUST_APPLY_TO_FOLDERS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200124i32);
pub const FSRM_E_PROPERTY_MUST_BE_GLOBAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200122i32);
pub const FSRM_E_PROPERTY_MUST_BE_SECURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200123i32);
pub const FSRM_E_REBUILDING_FODLER_TYPE_INDEX: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200139i32);
pub const FSRM_E_REPORT_GENERATION_ERR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200204i32);
pub const FSRM_E_REPORT_JOB_ALREADY_RUNNING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200205i32);
pub const FSRM_E_REPORT_TASK_TRIGGER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200203i32);
pub const FSRM_E_REPORT_TYPE_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200206i32);
pub const FSRM_E_REQD_PARAM_MISSING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200242i32);
pub const FSRM_E_RMS_NO_PROTECTORS_INSTALLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200126i32);
pub const FSRM_E_RMS_NO_PROTECTOR_INSTALLED_FOR_FILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200125i32);
pub const FSRM_E_RMS_TEMPLATE_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200128i32);
pub const FSRM_E_SECURE_PROPERTIES_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200127i32);
pub const FSRM_E_SET_PROPERTY_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200172i32);
pub const FSRM_E_SHADOW_COPY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200212i32);
pub const FSRM_E_STORE_NOT_INSTALLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200209i32);
pub const FSRM_E_SYNC_TASK_HAD_ERRORS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200119i32);
pub const FSRM_E_SYNC_TASK_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200144i32);
pub const FSRM_E_TEXTREADER_FILENAME_TOO_LONG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200158i32);
pub const FSRM_E_TEXTREADER_IFILTER_CLSID_MALFORMED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200160i32);
pub const FSRM_E_TEXTREADER_IFILTER_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200167i32);
pub const FSRM_E_TEXTREADER_NOT_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200168i32);
pub const FSRM_E_TEXTREADER_STREAM_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200159i32);
pub const FSRM_E_UNEXPECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200234i32);
pub const FSRM_E_UNSECURE_LINK_TO_HOSTED_MODULE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200188i32);
pub const FSRM_E_VOLUME_OFFLINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200154i32);
pub const FSRM_E_VOLUME_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200235i32);
pub const FSRM_E_WMI_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200121i32);
pub const FSRM_E_XML_CORRUPTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200211i32);
pub const FSRM_S_CLASSIFICATION_SCAN_FAILURES: ::windows::core::HRESULT = ::windows::core::HRESULT(283398i32);
pub const FSRM_S_PARTIAL_BATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(283396i32);
pub const FSRM_S_PARTIAL_CLASSIFICATION: ::windows::core::HRESULT = ::windows::core::HRESULT(283397i32);
pub const FsrmAccessDeniedRemediationClient: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x100b4fc8_74c1_470f_b1b7_dd7b6bae79bd);
pub type FsrmAccountType = i32;
pub const FsrmAccountType_Unknown: FsrmAccountType = 0i32;
pub const FsrmAccountType_NetworkService: FsrmAccountType = 1i32;
pub const FsrmAccountType_LocalService: FsrmAccountType = 2i32;
pub const FsrmAccountType_LocalSystem: FsrmAccountType = 3i32;
pub const FsrmAccountType_InProc: FsrmAccountType = 4i32;
pub const FsrmAccountType_External: FsrmAccountType = 5i32;
pub const FsrmAccountType_Automatic: FsrmAccountType = 500i32;
pub type FsrmActionType = i32;
pub const FsrmActionType_Unknown: FsrmActionType = 0i32;
pub const FsrmActionType_EventLog: FsrmActionType = 1i32;
pub const FsrmActionType_Email: FsrmActionType = 2i32;
pub const FsrmActionType_Command: FsrmActionType = 3i32;
pub const FsrmActionType_Report: FsrmActionType = 4i32;
pub type FsrmClassificationLoggingFlags = i32;
pub const FsrmClassificationLoggingFlags_None: FsrmClassificationLoggingFlags = 0i32;
pub const FsrmClassificationLoggingFlags_ClassificationsInLogFile: FsrmClassificationLoggingFlags = 1i32;
pub const FsrmClassificationLoggingFlags_ErrorsInLogFile: FsrmClassificationLoggingFlags = 2i32;
pub const FsrmClassificationLoggingFlags_ClassificationsInSystemLog: FsrmClassificationLoggingFlags = 4i32;
pub const FsrmClassificationLoggingFlags_ErrorsInSystemLog: FsrmClassificationLoggingFlags = 8i32;
pub const FsrmClassificationManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb15c0e47_c391_45b9_95c8_eb596c853f3a);
pub type FsrmCollectionState = i32;
pub const FsrmCollectionState_Fetching: FsrmCollectionState = 1i32;
pub const FsrmCollectionState_Committing: FsrmCollectionState = 2i32;
pub const FsrmCollectionState_Complete: FsrmCollectionState = 3i32;
pub const FsrmCollectionState_Cancelled: FsrmCollectionState = 4i32;
pub type FsrmCommitOptions = i32;
pub const FsrmCommitOptions_None: FsrmCommitOptions = 0i32;
pub const FsrmCommitOptions_Asynchronous: FsrmCommitOptions = 1i32;
pub const FsrmDaysNotSpecified: i32 = -1i32;
pub type FsrmEnumOptions = i32;
pub const FsrmEnumOptions_None: FsrmEnumOptions = 0i32;
pub const FsrmEnumOptions_Asynchronous: FsrmEnumOptions = 1i32;
pub const FsrmEnumOptions_CheckRecycleBin: FsrmEnumOptions = 2i32;
pub const FsrmEnumOptions_IncludeClusterNodes: FsrmEnumOptions = 4i32;
pub const FsrmEnumOptions_IncludeDeprecatedObjects: FsrmEnumOptions = 8i32;
pub type FsrmEventType = i32;
pub const FsrmEventType_Unknown: FsrmEventType = 0i32;
pub const FsrmEventType_Information: FsrmEventType = 1i32;
pub const FsrmEventType_Warning: FsrmEventType = 2i32;
pub const FsrmEventType_Error: FsrmEventType = 3i32;
pub type FsrmExecutionOption = i32;
pub const FsrmExecutionOption_Unknown: FsrmExecutionOption = 0i32;
pub const FsrmExecutionOption_EvaluateUnset: FsrmExecutionOption = 1i32;
pub const FsrmExecutionOption_ReEvaluate_ConsiderExistingValue: FsrmExecutionOption = 2i32;
pub const FsrmExecutionOption_ReEvaluate_IgnoreExistingValue: FsrmExecutionOption = 3i32;
pub const FsrmExportImport: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1482dc37_fae9_4787_9025_8ce4e024ab56);
pub type FsrmFileConditionType = i32;
pub const FsrmFileConditionType_Unknown: FsrmFileConditionType = 0i32;
pub const FsrmFileConditionType_Property: FsrmFileConditionType = 1i32;
pub const FsrmFileGroupManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f1363f6_656f_4496_9226_13aecbd7718f);
pub const FsrmFileManagementJobManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb18f9b2_4c3a_4321_b203_205120cff614);
pub type FsrmFileManagementLoggingFlags = i32;
pub const FsrmFileManagementLoggingFlags_None: FsrmFileManagementLoggingFlags = 0i32;
pub const FsrmFileManagementLoggingFlags_Error: FsrmFileManagementLoggingFlags = 1i32;
pub const FsrmFileManagementLoggingFlags_Information: FsrmFileManagementLoggingFlags = 2i32;
pub const FsrmFileManagementLoggingFlags_Audit: FsrmFileManagementLoggingFlags = 4i32;
pub type FsrmFileManagementType = i32;
pub const FsrmFileManagementType_Unknown: FsrmFileManagementType = 0i32;
pub const FsrmFileManagementType_Expiration: FsrmFileManagementType = 1i32;
pub const FsrmFileManagementType_Custom: FsrmFileManagementType = 2i32;
pub const FsrmFileManagementType_Rms: FsrmFileManagementType = 3i32;
pub type FsrmFileScreenFlags = i32;
pub const FsrmFileScreenFlags_Enforce: FsrmFileScreenFlags = 1i32;
pub const FsrmFileScreenManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95941183_db53_4c5f_b37b_7d0921cf9dc7);
pub const FsrmFileScreenTemplateManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x243111df_e474_46aa_a054_eaa33edc292a);
pub type FsrmFileStreamingInterfaceType = i32;
pub const FsrmFileStreamingInterfaceType_Unknown: FsrmFileStreamingInterfaceType = 0i32;
pub const FsrmFileStreamingInterfaceType_ILockBytes: FsrmFileStreamingInterfaceType = 1i32;
pub const FsrmFileStreamingInterfaceType_IStream: FsrmFileStreamingInterfaceType = 2i32;
pub type FsrmFileStreamingMode = i32;
pub const FsrmFileStreamingMode_Unknown: FsrmFileStreamingMode = 0i32;
pub const FsrmFileStreamingMode_Read: FsrmFileStreamingMode = 1i32;
pub const FsrmFileStreamingMode_Write: FsrmFileStreamingMode = 2i32;
pub type FsrmFileSystemPropertyId = i32;
pub const FsrmFileSystemPropertyId_Undefined: FsrmFileSystemPropertyId = 0i32;
pub const FsrmFileSystemPropertyId_FileName: FsrmFileSystemPropertyId = 1i32;
pub const FsrmFileSystemPropertyId_DateCreated: FsrmFileSystemPropertyId = 2i32;
pub const FsrmFileSystemPropertyId_DateLastAccessed: FsrmFileSystemPropertyId = 3i32;
pub const FsrmFileSystemPropertyId_DateLastModified: FsrmFileSystemPropertyId = 4i32;
pub const FsrmFileSystemPropertyId_DateNow: FsrmFileSystemPropertyId = 5i32;
pub type FsrmGetFilePropertyOptions = i32;
pub const FsrmGetFilePropertyOptions_None: FsrmGetFilePropertyOptions = 0i32;
pub const FsrmGetFilePropertyOptions_NoRuleEvaluation: FsrmGetFilePropertyOptions = 1i32;
pub const FsrmGetFilePropertyOptions_Persistent: FsrmGetFilePropertyOptions = 2i32;
pub const FsrmGetFilePropertyOptions_FailOnPersistErrors: FsrmGetFilePropertyOptions = 4i32;
pub const FsrmGetFilePropertyOptions_SkipOrphaned: FsrmGetFilePropertyOptions = 8i32;
pub const FsrmMaxExcludeFolders: u32 = 32u32;
pub const FsrmMaxNumberPropertyDefinitions: u32 = 100u32;
pub const FsrmMaxNumberThresholds: u32 = 16u32;
pub const FsrmMaxThresholdValue: u32 = 250u32;
pub const FsrmMinQuotaLimit: u32 = 1024u32;
pub const FsrmMinThresholdValue: u32 = 1u32;
pub const FsrmPathMapper: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3be42bd_8ac2_409e_bbd8_faf9b6b41feb);
pub const FsrmPipelineModuleConnector: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7643375_1eb5_44de_a062_623547d933bc);
pub type FsrmPipelineModuleType = i32;
pub const FsrmPipelineModuleType_Unknown: FsrmPipelineModuleType = 0i32;
pub const FsrmPipelineModuleType_Storage: FsrmPipelineModuleType = 1i32;
pub const FsrmPipelineModuleType_Classifier: FsrmPipelineModuleType = 2i32;
pub type FsrmPropertyBagField = i32;
pub const FsrmPropertyBagField_AccessVolume: FsrmPropertyBagField = 0i32;
pub const FsrmPropertyBagField_VolumeGuidName: FsrmPropertyBagField = 1i32;
pub type FsrmPropertyBagFlags = i32;
pub const FsrmPropertyBagFlags_UpdatedByClassifier: FsrmPropertyBagFlags = 1i32;
pub const FsrmPropertyBagFlags_FailedLoadingProperties: FsrmPropertyBagFlags = 2i32;
pub const FsrmPropertyBagFlags_FailedSavingProperties: FsrmPropertyBagFlags = 4i32;
pub const FsrmPropertyBagFlags_FailedClassifyingProperties: FsrmPropertyBagFlags = 8i32;
pub type FsrmPropertyConditionType = i32;
pub const FsrmPropertyConditionType_Unknown: FsrmPropertyConditionType = 0i32;
pub const FsrmPropertyConditionType_Equal: FsrmPropertyConditionType = 1i32;
pub const FsrmPropertyConditionType_NotEqual: FsrmPropertyConditionType = 2i32;
pub const FsrmPropertyConditionType_GreaterThan: FsrmPropertyConditionType = 3i32;
pub const FsrmPropertyConditionType_LessThan: FsrmPropertyConditionType = 4i32;
pub const FsrmPropertyConditionType_Contain: FsrmPropertyConditionType = 5i32;
pub const FsrmPropertyConditionType_Exist: FsrmPropertyConditionType = 6i32;
pub const FsrmPropertyConditionType_NotExist: FsrmPropertyConditionType = 7i32;
pub const FsrmPropertyConditionType_StartWith: FsrmPropertyConditionType = 8i32;
pub const FsrmPropertyConditionType_EndWith: FsrmPropertyConditionType = 9i32;
pub const FsrmPropertyConditionType_ContainedIn: FsrmPropertyConditionType = 10i32;
pub const FsrmPropertyConditionType_PrefixOf: FsrmPropertyConditionType = 11i32;
pub const FsrmPropertyConditionType_SuffixOf: FsrmPropertyConditionType = 12i32;
pub const FsrmPropertyConditionType_MatchesPattern: FsrmPropertyConditionType = 13i32;
pub type FsrmPropertyDefinitionAppliesTo = i32;
pub const FsrmPropertyDefinitionAppliesTo_Files: FsrmPropertyDefinitionAppliesTo = 1i32;
pub const FsrmPropertyDefinitionAppliesTo_Folders: FsrmPropertyDefinitionAppliesTo = 2i32;
pub type FsrmPropertyDefinitionFlags = i32;
pub const FsrmPropertyDefinitionFlags_Global: FsrmPropertyDefinitionFlags = 1i32;
pub const FsrmPropertyDefinitionFlags_Deprecated: FsrmPropertyDefinitionFlags = 2i32;
pub const FsrmPropertyDefinitionFlags_Secure: FsrmPropertyDefinitionFlags = 4i32;
pub type FsrmPropertyDefinitionType = i32;
pub const FsrmPropertyDefinitionType_Unknown: FsrmPropertyDefinitionType = 0i32;
pub const FsrmPropertyDefinitionType_OrderedList: FsrmPropertyDefinitionType = 1i32;
pub const FsrmPropertyDefinitionType_MultiChoiceList: FsrmPropertyDefinitionType = 2i32;
pub const FsrmPropertyDefinitionType_SingleChoiceList: FsrmPropertyDefinitionType = 3i32;
pub const FsrmPropertyDefinitionType_String: FsrmPropertyDefinitionType = 4i32;
pub const FsrmPropertyDefinitionType_MultiString: FsrmPropertyDefinitionType = 5i32;
pub const FsrmPropertyDefinitionType_Int: FsrmPropertyDefinitionType = 6i32;
pub const FsrmPropertyDefinitionType_Bool: FsrmPropertyDefinitionType = 7i32;
pub const FsrmPropertyDefinitionType_Date: FsrmPropertyDefinitionType = 8i32;
pub type FsrmPropertyFlags = i32;
pub const FsrmPropertyFlags_None: FsrmPropertyFlags = 0i32;
pub const FsrmPropertyFlags_Orphaned: FsrmPropertyFlags = 1i32;
pub const FsrmPropertyFlags_RetrievedFromCache: FsrmPropertyFlags = 2i32;
pub const FsrmPropertyFlags_RetrievedFromStorage: FsrmPropertyFlags = 4i32;
pub const FsrmPropertyFlags_SetByClassifier: FsrmPropertyFlags = 8i32;
pub const FsrmPropertyFlags_Deleted: FsrmPropertyFlags = 16i32;
pub const FsrmPropertyFlags_Reclassified: FsrmPropertyFlags = 32i32;
pub const FsrmPropertyFlags_AggregationFailed: FsrmPropertyFlags = 64i32;
pub const FsrmPropertyFlags_Existing: FsrmPropertyFlags = 128i32;
pub const FsrmPropertyFlags_FailedLoadingProperties: FsrmPropertyFlags = 256i32;
pub const FsrmPropertyFlags_FailedClassifyingProperties: FsrmPropertyFlags = 512i32;
pub const FsrmPropertyFlags_FailedSavingProperties: FsrmPropertyFlags = 1024i32;
pub const FsrmPropertyFlags_Secure: FsrmPropertyFlags = 2048i32;
pub const FsrmPropertyFlags_PolicyDerived: FsrmPropertyFlags = 4096i32;
pub const FsrmPropertyFlags_Inherited: FsrmPropertyFlags = 8192i32;
pub const FsrmPropertyFlags_Manual: FsrmPropertyFlags = 16384i32;
pub const FsrmPropertyFlags_ExplicitValueDeleted: FsrmPropertyFlags = 32768i32;
pub const FsrmPropertyFlags_PropertyDeletedFromClear: FsrmPropertyFlags = 65536i32;
pub const FsrmPropertyFlags_PropertySourceMask: FsrmPropertyFlags = 14i32;
pub const FsrmPropertyFlags_PersistentMask: FsrmPropertyFlags = 20480i32;
pub type FsrmPropertyValueType = i32;
pub const FsrmPropertyValueType_Undefined: FsrmPropertyValueType = 0i32;
pub const FsrmPropertyValueType_Literal: FsrmPropertyValueType = 1i32;
pub const FsrmPropertyValueType_DateOffset: FsrmPropertyValueType = 2i32;
pub type FsrmQuotaFlags = i32;
pub const FsrmQuotaFlags_Enforce: FsrmQuotaFlags = 256i32;
pub const FsrmQuotaFlags_Disable: FsrmQuotaFlags = 512i32;
pub const FsrmQuotaFlags_StatusIncomplete: FsrmQuotaFlags = 65536i32;
pub const FsrmQuotaFlags_StatusRebuilding: FsrmQuotaFlags = 131072i32;
pub const FsrmQuotaManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90dcab7f_347c_4bfc_b543_540326305fbe);
pub const FsrmQuotaTemplateManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97d3d443_251c_4337_81e7_b32e8f4ee65e);
pub type FsrmReportFilter = i32;
pub const FsrmReportFilter_MinSize: FsrmReportFilter = 1i32;
pub const FsrmReportFilter_MinAgeDays: FsrmReportFilter = 2i32;
pub const FsrmReportFilter_MaxAgeDays: FsrmReportFilter = 3i32;
pub const FsrmReportFilter_MinQuotaUsage: FsrmReportFilter = 4i32;
pub const FsrmReportFilter_FileGroups: FsrmReportFilter = 5i32;
pub const FsrmReportFilter_Owners: FsrmReportFilter = 6i32;
pub const FsrmReportFilter_NamePattern: FsrmReportFilter = 7i32;
pub const FsrmReportFilter_Property: FsrmReportFilter = 8i32;
pub type FsrmReportFormat = i32;
pub const FsrmReportFormat_Unknown: FsrmReportFormat = 0i32;
pub const FsrmReportFormat_DHtml: FsrmReportFormat = 1i32;
pub const FsrmReportFormat_Html: FsrmReportFormat = 2i32;
pub const FsrmReportFormat_Txt: FsrmReportFormat = 3i32;
pub const FsrmReportFormat_Csv: FsrmReportFormat = 4i32;
pub const FsrmReportFormat_Xml: FsrmReportFormat = 5i32;
pub type FsrmReportGenerationContext = i32;
pub const FsrmReportGenerationContext_Undefined: FsrmReportGenerationContext = 1i32;
pub const FsrmReportGenerationContext_ScheduledReport: FsrmReportGenerationContext = 2i32;
pub const FsrmReportGenerationContext_InteractiveReport: FsrmReportGenerationContext = 3i32;
pub const FsrmReportGenerationContext_IncidentReport: FsrmReportGenerationContext = 4i32;
pub type FsrmReportLimit = i32;
pub const FsrmReportLimit_MaxFiles: FsrmReportLimit = 1i32;
pub const FsrmReportLimit_MaxFileGroups: FsrmReportLimit = 2i32;
pub const FsrmReportLimit_MaxOwners: FsrmReportLimit = 3i32;
pub const FsrmReportLimit_MaxFilesPerFileGroup: FsrmReportLimit = 4i32;
pub const FsrmReportLimit_MaxFilesPerOwner: FsrmReportLimit = 5i32;
pub const FsrmReportLimit_MaxFilesPerDuplGroup: FsrmReportLimit = 6i32;
pub const FsrmReportLimit_MaxDuplicateGroups: FsrmReportLimit = 7i32;
pub const FsrmReportLimit_MaxQuotas: FsrmReportLimit = 8i32;
pub const FsrmReportLimit_MaxFileScreenEvents: FsrmReportLimit = 9i32;
pub const FsrmReportLimit_MaxPropertyValues: FsrmReportLimit = 10i32;
pub const FsrmReportLimit_MaxFilesPerPropertyValue: FsrmReportLimit = 11i32;
pub const FsrmReportLimit_MaxFolders: FsrmReportLimit = 12i32;
pub const FsrmReportManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0058ef37_aa66_4c48_bd5b_2fce432ab0c8);
pub type FsrmReportRunningStatus = i32;
pub const FsrmReportRunningStatus_Unknown: FsrmReportRunningStatus = 0i32;
pub const FsrmReportRunningStatus_NotRunning: FsrmReportRunningStatus = 1i32;
pub const FsrmReportRunningStatus_Queued: FsrmReportRunningStatus = 2i32;
pub const FsrmReportRunningStatus_Running: FsrmReportRunningStatus = 3i32;
pub const FsrmReportScheduler: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea25f1b8_1b8d_4290_8ee8_e17c12c2fe20);
pub type FsrmReportType = i32;
pub const FsrmReportType_Unknown: FsrmReportType = 0i32;
pub const FsrmReportType_LargeFiles: FsrmReportType = 1i32;
pub const FsrmReportType_FilesByType: FsrmReportType = 2i32;
pub const FsrmReportType_LeastRecentlyAccessed: FsrmReportType = 3i32;
pub const FsrmReportType_MostRecentlyAccessed: FsrmReportType = 4i32;
pub const FsrmReportType_QuotaUsage: FsrmReportType = 5i32;
pub const FsrmReportType_FilesByOwner: FsrmReportType = 6i32;
pub const FsrmReportType_ExportReport: FsrmReportType = 7i32;
pub const FsrmReportType_DuplicateFiles: FsrmReportType = 8i32;
pub const FsrmReportType_FileScreenAudit: FsrmReportType = 9i32;
pub const FsrmReportType_FilesByProperty: FsrmReportType = 10i32;
pub const FsrmReportType_AutomaticClassification: FsrmReportType = 11i32;
pub const FsrmReportType_Expiration: FsrmReportType = 12i32;
pub const FsrmReportType_FoldersByProperty: FsrmReportType = 13i32;
pub type FsrmRuleFlags = i32;
pub const FsrmRuleFlags_Disabled: FsrmRuleFlags = 256i32;
pub const FsrmRuleFlags_ClearAutomaticallyClassifiedProperty: FsrmRuleFlags = 1024i32;
pub const FsrmRuleFlags_ClearManuallyClassifiedProperty: FsrmRuleFlags = 2048i32;
pub const FsrmRuleFlags_Invalid: FsrmRuleFlags = 4096i32;
pub type FsrmRuleType = i32;
pub const FsrmRuleType_Unknown: FsrmRuleType = 0i32;
pub const FsrmRuleType_Classification: FsrmRuleType = 1i32;
pub const FsrmRuleType_Generic: FsrmRuleType = 2i32;
pub const FsrmSetting: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf556d708_6d4d_4594_9c61_7dbb0dae2a46);
pub type FsrmStorageModuleCaps = i32;
pub const FsrmStorageModuleCaps_Unknown: FsrmStorageModuleCaps = 0i32;
pub const FsrmStorageModuleCaps_CanGet: FsrmStorageModuleCaps = 1i32;
pub const FsrmStorageModuleCaps_CanSet: FsrmStorageModuleCaps = 2i32;
pub const FsrmStorageModuleCaps_CanHandleDirectories: FsrmStorageModuleCaps = 4i32;
pub const FsrmStorageModuleCaps_CanHandleFiles: FsrmStorageModuleCaps = 8i32;
pub type FsrmStorageModuleType = i32;
pub const FsrmStorageModuleType_Unknown: FsrmStorageModuleType = 0i32;
pub const FsrmStorageModuleType_Cache: FsrmStorageModuleType = 1i32;
pub const FsrmStorageModuleType_InFile: FsrmStorageModuleType = 2i32;
pub const FsrmStorageModuleType_Database: FsrmStorageModuleType = 3i32;
pub const FsrmStorageModuleType_System: FsrmStorageModuleType = 100i32;
pub type FsrmTemplateApplyOptions = i32;
pub const FsrmTemplateApplyOptions_ApplyToDerivedMatching: FsrmTemplateApplyOptions = 1i32;
pub const FsrmTemplateApplyOptions_ApplyToDerivedAll: FsrmTemplateApplyOptions = 2i32;
#[repr(transparent)]
pub struct IFsrmAccessDeniedRemediationClient(::windows::core::IUnknown);
impl IFsrmAccessDeniedRemediationClient {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, parentwnd: usize, accesspath: Param1, errortype: AdrClientErrorType, flags: i32, windowtitle: Param4, windowmessage: Param5) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(parentwnd), accesspath.into_param().abi(), ::core::mem::transmute(errortype), ::core::mem::transmute(flags), windowtitle.into_param().abi(), windowmessage.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmAccessDeniedRemediationClient> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmAccessDeniedRemediationClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmAccessDeniedRemediationClient> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmAccessDeniedRemediationClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmAccessDeniedRemediationClient {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmAccessDeniedRemediationClient {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmAccessDeniedRemediationClient> for ::windows::core::IUnknown {
    fn from(value: IFsrmAccessDeniedRemediationClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmAccessDeniedRemediationClient> for ::windows::core::IUnknown {
    fn from(value: &IFsrmAccessDeniedRemediationClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmAccessDeniedRemediationClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmAccessDeniedRemediationClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmAccessDeniedRemediationClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmAccessDeniedRemediationClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmAccessDeniedRemediationClient {}
unsafe impl ::windows::core::Interface for IFsrmAccessDeniedRemediationClient {
    type Vtable = IFsrmAccessDeniedRemediationClientVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40002314_590b_45a5_8e1b_8c05da527e52);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmAccessDeniedRemediationClientVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parentwnd: usize, accesspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, errortype: AdrClientErrorType, flags: i32, windowtitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, windowmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, result: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IFsrmAction(::windows::core::IUnknown);
impl IFsrmAction {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn ActionType(&self) -> ::windows::core::Result<FsrmActionType> {
        let mut result__: FsrmActionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmActionType>(result__)
    }
    pub unsafe fn RunLimitInterval(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(minutes)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmAction> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmAction> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmAction {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmAction {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmAction> for ::windows::core::IUnknown {
    fn from(value: IFsrmAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmAction> for ::windows::core::IUnknown {
    fn from(value: &IFsrmAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmAction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmAction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmAction {}
unsafe impl ::windows::core::Interface for IFsrmAction {
    type Vtable = IFsrmActionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6cd6408a_ae60_463b_9ef1_e117534d69dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actiontype: *mut FsrmActionType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmActionCommand(::windows::core::IUnknown);
impl IFsrmActionCommand {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn ActionType(&self) -> ::windows::core::Result<FsrmActionType> {
        let mut result__: FsrmActionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmActionType>(result__)
    }
    pub unsafe fn RunLimitInterval(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(minutes)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExecutablePath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExecutablePath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, executablepath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), executablepath.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Arguments(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetArguments<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, arguments: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), arguments.into_param().abi()).ok()
    }
    pub unsafe fn Account(&self) -> ::windows::core::Result<FsrmAccountType> {
        let mut result__: FsrmAccountType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmAccountType>(result__)
    }
    pub unsafe fn SetAccount(&self, account: FsrmAccountType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(account)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WorkingDirectory(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWorkingDirectory<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, workingdirectory: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), workingdirectory.into_param().abi()).ok()
    }
    pub unsafe fn MonitorCommand(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    pub unsafe fn SetMonitorCommand(&self, monitorcommand: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(monitorcommand)).ok()
    }
    pub unsafe fn KillTimeOut(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn SetKillTimeOut(&self, minutes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(minutes)).ok()
    }
    pub unsafe fn LogResult(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    pub unsafe fn SetLogResult(&self, logresults: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(logresults)).ok()
    }
}
impl ::core::convert::From<IFsrmActionCommand> for IFsrmAction {
    fn from(value: IFsrmActionCommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmActionCommand> for IFsrmAction {
    fn from(value: &IFsrmActionCommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmAction> for IFsrmActionCommand {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmAction> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmAction> for &IFsrmActionCommand {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmAction> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmActionCommand> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmActionCommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmActionCommand> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmActionCommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmActionCommand {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmActionCommand {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmActionCommand> for ::windows::core::IUnknown {
    fn from(value: IFsrmActionCommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmActionCommand> for ::windows::core::IUnknown {
    fn from(value: &IFsrmActionCommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmActionCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmActionCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmActionCommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmActionCommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmActionCommand {}
unsafe impl ::windows::core::Interface for IFsrmActionCommand {
    type Vtable = IFsrmActionCommandVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12937789_e247_4917_9c20_f3ee9c7ee783);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionCommandVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actiontype: *mut FsrmActionType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, executablepath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, executablepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, arguments: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, arguments: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, account: *mut FsrmAccountType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, account: FsrmAccountType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, workingdirectory: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, workingdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, monitorcommand: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, monitorcommand: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logresults: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logresults: i16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmActionEmail(::windows::core::IUnknown);
impl IFsrmActionEmail {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn ActionType(&self) -> ::windows::core::Result<FsrmActionType> {
        let mut result__: FsrmActionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmActionType>(result__)
    }
    pub unsafe fn RunLimitInterval(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(minutes)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailFrom(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailFrom<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailfrom: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), mailfrom.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailReplyTo(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailReplyTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailreplyto: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), mailreplyto.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailTo(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailto: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), mailto.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailCc(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailCc<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailcc: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), mailcc.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailBcc(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailBcc<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailbcc: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), mailbcc.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailSubject(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailSubject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailsubject: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), mailsubject.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MessageText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMessageText<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, messagetext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), messagetext.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IFsrmActionEmail> for IFsrmAction {
    fn from(value: IFsrmActionEmail) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmActionEmail> for IFsrmAction {
    fn from(value: &IFsrmActionEmail) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmAction> for IFsrmActionEmail {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmAction> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmAction> for &IFsrmActionEmail {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmAction> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmActionEmail> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmActionEmail) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmActionEmail> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmActionEmail) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmActionEmail {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmActionEmail {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmActionEmail> for ::windows::core::IUnknown {
    fn from(value: IFsrmActionEmail) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmActionEmail> for ::windows::core::IUnknown {
    fn from(value: &IFsrmActionEmail) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmActionEmail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmActionEmail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmActionEmail {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmActionEmail {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmActionEmail {}
unsafe impl ::windows::core::Interface for IFsrmActionEmail {
    type Vtable = IFsrmActionEmailVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd646567d_26ae_4caa_9f84_4e0aad207fca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionEmailVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actiontype: *mut FsrmActionType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailfrom: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailfrom: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailreplyto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailreplyto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailcc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailcc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailbcc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailbcc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailsubject: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailsubject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IFsrmActionEmail2(::windows::core::IUnknown);
impl IFsrmActionEmail2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn ActionType(&self) -> ::windows::core::Result<FsrmActionType> {
        let mut result__: FsrmActionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmActionType>(result__)
    }
    pub unsafe fn RunLimitInterval(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(minutes)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailFrom(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailFrom<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailfrom: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), mailfrom.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailReplyTo(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailReplyTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailreplyto: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), mailreplyto.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailTo(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailto: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), mailto.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailCc(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailCc<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailcc: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), mailcc.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailBcc(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailBcc<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailbcc: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), mailbcc.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailSubject(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailSubject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailsubject: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), mailsubject.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MessageText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMessageText<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, messagetext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), messagetext.into_param().abi()).ok()
    }
    pub unsafe fn AttachmentFileListSize(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAttachmentFileListSize(&self, attachmentfilelistsize: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(attachmentfilelistsize)).ok()
    }
}
impl ::core::convert::From<IFsrmActionEmail2> for IFsrmActionEmail {
    fn from(value: IFsrmActionEmail2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmActionEmail2> for IFsrmActionEmail {
    fn from(value: &IFsrmActionEmail2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmActionEmail> for IFsrmActionEmail2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmActionEmail> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmActionEmail> for &IFsrmActionEmail2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmActionEmail> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmActionEmail2> for IFsrmAction {
    fn from(value: IFsrmActionEmail2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmActionEmail2> for IFsrmAction {
    fn from(value: &IFsrmActionEmail2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmAction> for IFsrmActionEmail2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmAction> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmAction> for &IFsrmActionEmail2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmAction> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmActionEmail2> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmActionEmail2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmActionEmail2> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmActionEmail2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmActionEmail2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmActionEmail2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmActionEmail2> for ::windows::core::IUnknown {
    fn from(value: IFsrmActionEmail2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmActionEmail2> for ::windows::core::IUnknown {
    fn from(value: &IFsrmActionEmail2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmActionEmail2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmActionEmail2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmActionEmail2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmActionEmail2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmActionEmail2 {}
unsafe impl ::windows::core::Interface for IFsrmActionEmail2 {
    type Vtable = IFsrmActionEmail2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8276702f_2532_4839_89bf_4872609a2ea4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionEmail2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actiontype: *mut FsrmActionType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailfrom: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailfrom: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailreplyto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailreplyto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailcc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailcc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailbcc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailbcc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailsubject: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailsubject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attachmentfilelistsize: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attachmentfilelistsize: i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmActionEventLog(::windows::core::IUnknown);
impl IFsrmActionEventLog {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn ActionType(&self) -> ::windows::core::Result<FsrmActionType> {
        let mut result__: FsrmActionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmActionType>(result__)
    }
    pub unsafe fn RunLimitInterval(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(minutes)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn EventType(&self) -> ::windows::core::Result<FsrmEventType> {
        let mut result__: FsrmEventType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmEventType>(result__)
    }
    pub unsafe fn SetEventType(&self, eventtype: FsrmEventType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(eventtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MessageText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMessageText<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, messagetext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), messagetext.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IFsrmActionEventLog> for IFsrmAction {
    fn from(value: IFsrmActionEventLog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmActionEventLog> for IFsrmAction {
    fn from(value: &IFsrmActionEventLog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmAction> for IFsrmActionEventLog {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmAction> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmAction> for &IFsrmActionEventLog {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmAction> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmActionEventLog> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmActionEventLog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmActionEventLog> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmActionEventLog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmActionEventLog {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmActionEventLog {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmActionEventLog> for ::windows::core::IUnknown {
    fn from(value: IFsrmActionEventLog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmActionEventLog> for ::windows::core::IUnknown {
    fn from(value: &IFsrmActionEventLog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmActionEventLog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmActionEventLog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmActionEventLog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmActionEventLog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmActionEventLog {}
unsafe impl ::windows::core::Interface for IFsrmActionEventLog {
    type Vtable = IFsrmActionEventLogVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c8f96c3_5d94_4f37_a4f4_f56ab463546f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionEventLogVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actiontype: *mut FsrmActionType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventtype: *mut FsrmEventType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventtype: FsrmEventType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IFsrmActionReport(::windows::core::IUnknown);
impl IFsrmActionReport {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn ActionType(&self) -> ::windows::core::Result<FsrmActionType> {
        let mut result__: FsrmActionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmActionType>(result__)
    }
    pub unsafe fn RunLimitInterval(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(minutes)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReportTypes(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetReportTypes(&self, reporttypes: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(reporttypes)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailTo(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailto: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), mailto.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IFsrmActionReport> for IFsrmAction {
    fn from(value: IFsrmActionReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmActionReport> for IFsrmAction {
    fn from(value: &IFsrmActionReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmAction> for IFsrmActionReport {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmAction> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmAction> for &IFsrmActionReport {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmAction> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmActionReport> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmActionReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmActionReport> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmActionReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmActionReport {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmActionReport {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmActionReport> for ::windows::core::IUnknown {
    fn from(value: IFsrmActionReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmActionReport> for ::windows::core::IUnknown {
    fn from(value: &IFsrmActionReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmActionReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmActionReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmActionReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmActionReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmActionReport {}
unsafe impl ::windows::core::Interface for IFsrmActionReport {
    type Vtable = IFsrmActionReportVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2dbe63c4_b340_48a0_a5b0_158e07fc567e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionReportVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actiontype: *mut FsrmActionType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttypes: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttypes: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IFsrmAutoApplyQuota(::windows::core::IUnknown);
impl IFsrmAutoApplyQuota {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn QuotaLimit(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetQuotaLimit<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, quotalimit: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), quotalimit.into_param().abi()).ok()
    }
    pub unsafe fn QuotaFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(quotaflags)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Thresholds(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn AddThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold)).ok()
    }
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold)).ok()
    }
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(newthreshold)).ok()
    }
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(actiontype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmAction>(result__)
    }
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserSid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserAccount(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SourceTemplateName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn MatchesSourceTemplate(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplyTemplate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, quotatemplatename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), quotatemplatename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExcludeFolders(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetExcludeFolders(&self, folders: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(folders)).ok()
    }
    pub unsafe fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows::core::Result<IFsrmDerivedObjectsResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(commitoptions), ::core::mem::transmute(applyoptions), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmDerivedObjectsResult>(result__)
    }
}
impl ::core::convert::From<IFsrmAutoApplyQuota> for IFsrmQuotaObject {
    fn from(value: IFsrmAutoApplyQuota) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmAutoApplyQuota> for IFsrmQuotaObject {
    fn from(value: &IFsrmAutoApplyQuota) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaObject> for IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaObject> for &IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmAutoApplyQuota> for IFsrmQuotaBase {
    fn from(value: IFsrmAutoApplyQuota) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmAutoApplyQuota> for IFsrmQuotaBase {
    fn from(value: &IFsrmAutoApplyQuota) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaBase> for IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaBase> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaBase> for &IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmAutoApplyQuota> for IFsrmObject {
    fn from(value: IFsrmAutoApplyQuota) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmAutoApplyQuota> for IFsrmObject {
    fn from(value: &IFsrmAutoApplyQuota) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmAutoApplyQuota> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmAutoApplyQuota) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmAutoApplyQuota> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmAutoApplyQuota) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmAutoApplyQuota> for ::windows::core::IUnknown {
    fn from(value: IFsrmAutoApplyQuota) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmAutoApplyQuota> for ::windows::core::IUnknown {
    fn from(value: &IFsrmAutoApplyQuota) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmAutoApplyQuota {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmAutoApplyQuota {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmAutoApplyQuota {}
unsafe impl ::windows::core::Interface for IFsrmAutoApplyQuota {
    type Vtable = IFsrmAutoApplyQuotaVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf82e5729_6aba_4740_bfc7_c7f58f75fb7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmAutoApplyQuotaVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotalimit: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotalimit: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotaflags: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotaflags: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, thresholds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32, newthreshold: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32, actiontype: FsrmActionType, action: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32, actions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccount: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotatemplatename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matches: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotatemplatename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folders: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmClassificationManager(::windows::core::IUnknown);
impl IFsrmClassificationManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ClassificationReportFormats(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetClassificationReportFormats(&self, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(formats)).ok()
    }
    pub unsafe fn Logging(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn SetLogging(&self, logging: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(logging)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClassificationReportMailTo(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClassificationReportMailTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailto: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), mailto.into_param().abi()).ok()
    }
    pub unsafe fn ClassificationReportEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    pub unsafe fn SetClassificationReportEnabled(&self, reportenabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(reportenabled)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClassificationLastReportPathWithoutExtension(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClassificationLastError(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn ClassificationRunningStatus(&self) -> ::windows::core::Result<FsrmReportRunningStatus> {
        let mut result__: FsrmReportRunningStatus = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmReportRunningStatus>(result__)
    }
    pub unsafe fn EnumPropertyDefinitions(&self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    pub unsafe fn CreatePropertyDefinition(&self) -> ::windows::core::Result<IFsrmPropertyDefinition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmPropertyDefinition>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, propertyname: Param0) -> ::windows::core::Result<IFsrmPropertyDefinition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), propertyname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmPropertyDefinition>(result__)
    }
    pub unsafe fn EnumRules(&self, ruletype: FsrmRuleType, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(ruletype), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    pub unsafe fn CreateRule(&self, ruletype: FsrmRuleType) -> ::windows::core::Result<IFsrmRule> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(ruletype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmRule>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRule<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, rulename: Param0, ruletype: FsrmRuleType) -> ::windows::core::Result<IFsrmRule> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), rulename.into_param().abi(), ::core::mem::transmute(ruletype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmRule>(result__)
    }
    pub unsafe fn EnumModuleDefinitions(&self, moduletype: FsrmPipelineModuleType, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(moduletype), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    pub unsafe fn CreateModuleDefinition(&self, moduletype: FsrmPipelineModuleType) -> ::windows::core::Result<IFsrmPipelineModuleDefinition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(moduletype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmPipelineModuleDefinition>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetModuleDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, modulename: Param0, moduletype: FsrmPipelineModuleType) -> ::windows::core::Result<IFsrmPipelineModuleDefinition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), modulename.into_param().abi(), ::core::mem::transmute(moduletype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmPipelineModuleDefinition>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunClassification<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, context: FsrmReportGenerationContext, reserved: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(context), reserved.into_param().abi()).ok()
    }
    pub unsafe fn WaitForClassificationCompletion(&self, waitseconds: i32) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(waitseconds), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    pub unsafe fn CancelClassification(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumFileProperties<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, options: FsrmGetFilePropertyOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), filepath.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFileProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, propertyname: Param1, options: FsrmGetFilePropertyOptions) -> ::windows::core::Result<IFsrmProperty> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), filepath.into_param().abi(), propertyname.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmProperty>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, propertyname: Param1, propertyvalue: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), filepath.into_param().abi(), propertyname.into_param().abi(), propertyvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearFileProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, property: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), filepath.into_param().abi(), property.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmClassificationManager> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmClassificationManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmClassificationManager> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmClassificationManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmClassificationManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmClassificationManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmClassificationManager> for ::windows::core::IUnknown {
    fn from(value: IFsrmClassificationManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmClassificationManager> for ::windows::core::IUnknown {
    fn from(value: &IFsrmClassificationManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmClassificationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmClassificationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmClassificationManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmClassificationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmClassificationManager {}
unsafe impl ::windows::core::Interface for IFsrmClassificationManager {
    type Vtable = IFsrmClassificationManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2dc89da_ee91_48a0_85d8_cc72a56f7d04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassificationManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logging: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logging: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportenabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportenabled: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastreportpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lasterror: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runningstatus: *mut FsrmReportRunningStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, propertydefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertydefinition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertydefinition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ruletype: FsrmRuleType, options: FsrmEnumOptions, rules: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ruletype: FsrmRuleType, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rulename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ruletype: FsrmRuleType, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduletype: FsrmPipelineModuleType, options: FsrmEnumOptions, moduledefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduletype: FsrmPipelineModuleType, moduledefinition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modulename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, moduletype: FsrmPipelineModuleType, moduledefinition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext, reserved: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmGetFilePropertyOptions, fileproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmGetFilePropertyOptions, property: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, property: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IFsrmClassificationManager2(::windows::core::IUnknown);
impl IFsrmClassificationManager2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ClassificationReportFormats(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetClassificationReportFormats(&self, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(formats)).ok()
    }
    pub unsafe fn Logging(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn SetLogging(&self, logging: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(logging)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClassificationReportMailTo(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClassificationReportMailTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailto: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), mailto.into_param().abi()).ok()
    }
    pub unsafe fn ClassificationReportEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    pub unsafe fn SetClassificationReportEnabled(&self, reportenabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(reportenabled)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClassificationLastReportPathWithoutExtension(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClassificationLastError(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn ClassificationRunningStatus(&self) -> ::windows::core::Result<FsrmReportRunningStatus> {
        let mut result__: FsrmReportRunningStatus = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmReportRunningStatus>(result__)
    }
    pub unsafe fn EnumPropertyDefinitions(&self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    pub unsafe fn CreatePropertyDefinition(&self) -> ::windows::core::Result<IFsrmPropertyDefinition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmPropertyDefinition>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, propertyname: Param0) -> ::windows::core::Result<IFsrmPropertyDefinition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), propertyname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmPropertyDefinition>(result__)
    }
    pub unsafe fn EnumRules(&self, ruletype: FsrmRuleType, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(ruletype), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    pub unsafe fn CreateRule(&self, ruletype: FsrmRuleType) -> ::windows::core::Result<IFsrmRule> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(ruletype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmRule>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRule<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, rulename: Param0, ruletype: FsrmRuleType) -> ::windows::core::Result<IFsrmRule> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), rulename.into_param().abi(), ::core::mem::transmute(ruletype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmRule>(result__)
    }
    pub unsafe fn EnumModuleDefinitions(&self, moduletype: FsrmPipelineModuleType, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(moduletype), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    pub unsafe fn CreateModuleDefinition(&self, moduletype: FsrmPipelineModuleType) -> ::windows::core::Result<IFsrmPipelineModuleDefinition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(moduletype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmPipelineModuleDefinition>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetModuleDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, modulename: Param0, moduletype: FsrmPipelineModuleType) -> ::windows::core::Result<IFsrmPipelineModuleDefinition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), modulename.into_param().abi(), ::core::mem::transmute(moduletype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmPipelineModuleDefinition>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunClassification<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, context: FsrmReportGenerationContext, reserved: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(context), reserved.into_param().abi()).ok()
    }
    pub unsafe fn WaitForClassificationCompletion(&self, waitseconds: i32) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(waitseconds), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    pub unsafe fn CancelClassification(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumFileProperties<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, options: FsrmGetFilePropertyOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), filepath.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFileProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, propertyname: Param1, options: FsrmGetFilePropertyOptions) -> ::windows::core::Result<IFsrmProperty> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), filepath.into_param().abi(), propertyname.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmProperty>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, propertyname: Param1, propertyvalue: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), filepath.into_param().abi(), propertyname.into_param().abi(), propertyvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearFileProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, property: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), filepath.into_param().abi(), property.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ClassifyFiles(&self, filepaths: *const super::super::System::Com::SAFEARRAY, propertynames: *const super::super::System::Com::SAFEARRAY, propertyvalues: *const super::super::System::Com::SAFEARRAY, options: FsrmGetFilePropertyOptions) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(filepaths), ::core::mem::transmute(propertynames), ::core::mem::transmute(propertyvalues), ::core::mem::transmute(options)).ok()
    }
}
impl ::core::convert::From<IFsrmClassificationManager2> for IFsrmClassificationManager {
    fn from(value: IFsrmClassificationManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmClassificationManager2> for IFsrmClassificationManager {
    fn from(value: &IFsrmClassificationManager2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmClassificationManager> for IFsrmClassificationManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmClassificationManager> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmClassificationManager> for &IFsrmClassificationManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmClassificationManager> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmClassificationManager2> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmClassificationManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmClassificationManager2> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmClassificationManager2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmClassificationManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmClassificationManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmClassificationManager2> for ::windows::core::IUnknown {
    fn from(value: IFsrmClassificationManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmClassificationManager2> for ::windows::core::IUnknown {
    fn from(value: &IFsrmClassificationManager2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmClassificationManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmClassificationManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmClassificationManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmClassificationManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmClassificationManager2 {}
unsafe impl ::windows::core::Interface for IFsrmClassificationManager2 {
    type Vtable = IFsrmClassificationManager2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0004c1c9_127e_4765_ba07_6a3147bca112);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassificationManager2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logging: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logging: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportenabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportenabled: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastreportpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lasterror: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runningstatus: *mut FsrmReportRunningStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, propertydefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertydefinition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertydefinition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ruletype: FsrmRuleType, options: FsrmEnumOptions, rules: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ruletype: FsrmRuleType, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rulename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ruletype: FsrmRuleType, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduletype: FsrmPipelineModuleType, options: FsrmEnumOptions, moduledefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduletype: FsrmPipelineModuleType, moduledefinition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modulename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, moduletype: FsrmPipelineModuleType, moduledefinition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext, reserved: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmGetFilePropertyOptions, fileproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmGetFilePropertyOptions, property: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, property: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepaths: *const super::super::System::Com::SAFEARRAY, propertynames: *const super::super::System::Com::SAFEARRAY, propertyvalues: *const super::super::System::Com::SAFEARRAY, options: FsrmGetFilePropertyOptions) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
pub struct IFsrmClassificationRule(::windows::core::IUnknown);
impl IFsrmClassificationRule {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn RuleType(&self) -> ::windows::core::Result<FsrmRuleType> {
        let mut result__: FsrmRuleType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmRuleType>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModuleDefinitionName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetModuleDefinitionName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, moduledefinitionname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), moduledefinitionname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NamespaceRoots(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetNamespaceRoots(&self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(namespaceroots)).ok()
    }
    pub unsafe fn RuleFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRuleFlags(&self, ruleflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(ruleflags)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(parameters)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LastModified(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    pub unsafe fn ExecutionOption(&self) -> ::windows::core::Result<FsrmExecutionOption> {
        let mut result__: FsrmExecutionOption = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmExecutionOption>(result__)
    }
    pub unsafe fn SetExecutionOption(&self, executionoption: FsrmExecutionOption) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(executionoption)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PropertyAffected(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPropertyAffected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, property: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), property.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, value: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IFsrmClassificationRule> for IFsrmRule {
    fn from(value: IFsrmClassificationRule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmClassificationRule> for IFsrmRule {
    fn from(value: &IFsrmClassificationRule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmRule> for IFsrmClassificationRule {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmRule> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmRule> for &IFsrmClassificationRule {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmRule> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmClassificationRule> for IFsrmObject {
    fn from(value: IFsrmClassificationRule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmClassificationRule> for IFsrmObject {
    fn from(value: &IFsrmClassificationRule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmClassificationRule {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &IFsrmClassificationRule {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmClassificationRule> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmClassificationRule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmClassificationRule> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmClassificationRule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmClassificationRule {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmClassificationRule {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmClassificationRule> for ::windows::core::IUnknown {
    fn from(value: IFsrmClassificationRule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmClassificationRule> for ::windows::core::IUnknown {
    fn from(value: &IFsrmClassificationRule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmClassificationRule {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmClassificationRule {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmClassificationRule {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmClassificationRule {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmClassificationRule {}
unsafe impl ::windows::core::Interface for IFsrmClassificationRule {
    type Vtable = IFsrmClassificationRuleVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xafc052c2_5315_45ab_841b_c6db0e120148);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassificationRuleVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ruletype: *mut FsrmRuleType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduledefinitionname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ruleflags: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ruleflags: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastmodified: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, executionoption: *mut FsrmExecutionOption) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, executionoption: FsrmExecutionOption) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IFsrmClassifierModuleDefinition(::windows::core::IUnknown);
impl IFsrmClassifierModuleDefinition {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModuleClsid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetModuleClsid<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, moduleclsid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), moduleclsid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Company(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCompany<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, company: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), company.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Version(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVersion<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, version: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), version.into_param().abi()).ok()
    }
    pub unsafe fn ModuleType(&self) -> ::windows::core::Result<FsrmPipelineModuleType> {
        let mut result__: FsrmPipelineModuleType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmPipelineModuleType>(result__)
    }
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn NeedsFileContent(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    pub unsafe fn SetNeedsFileContent(&self, needsfilecontent: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(needsfilecontent)).ok()
    }
    pub unsafe fn Account(&self) -> ::windows::core::Result<FsrmAccountType> {
        let mut result__: FsrmAccountType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmAccountType>(result__)
    }
    pub unsafe fn SetAccount(&self, retrievalaccount: FsrmAccountType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(retrievalaccount)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedExtensions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSupportedExtensions(&self, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(supportedextensions)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(parameters)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PropertiesAffected(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPropertiesAffected(&self, propertiesaffected: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(propertiesaffected)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PropertiesUsed(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPropertiesUsed(&self, propertiesused: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(propertiesused)).ok()
    }
    pub unsafe fn NeedsExplicitValue(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    pub unsafe fn SetNeedsExplicitValue(&self, needsexplicitvalue: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(needsexplicitvalue)).ok()
    }
}
impl ::core::convert::From<IFsrmClassifierModuleDefinition> for IFsrmPipelineModuleDefinition {
    fn from(value: IFsrmClassifierModuleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmClassifierModuleDefinition> for IFsrmPipelineModuleDefinition {
    fn from(value: &IFsrmClassifierModuleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmPipelineModuleDefinition> for IFsrmClassifierModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmPipelineModuleDefinition> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmPipelineModuleDefinition> for &IFsrmClassifierModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmPipelineModuleDefinition> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmClassifierModuleDefinition> for IFsrmObject {
    fn from(value: IFsrmClassifierModuleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmClassifierModuleDefinition> for IFsrmObject {
    fn from(value: &IFsrmClassifierModuleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmClassifierModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &IFsrmClassifierModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmClassifierModuleDefinition> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmClassifierModuleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmClassifierModuleDefinition> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmClassifierModuleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmClassifierModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmClassifierModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmClassifierModuleDefinition> for ::windows::core::IUnknown {
    fn from(value: IFsrmClassifierModuleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmClassifierModuleDefinition> for ::windows::core::IUnknown {
    fn from(value: &IFsrmClassifierModuleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmClassifierModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmClassifierModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmClassifierModuleDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmClassifierModuleDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmClassifierModuleDefinition {}
unsafe impl ::windows::core::Interface for IFsrmClassifierModuleDefinition {
    type Vtable = IFsrmClassifierModuleDefinitionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb36ea26_6318_4b8c_8592_f72dd602e7a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassifierModuleDefinitionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleclsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, company: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, company: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, version: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, version: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduletype: *mut FsrmPipelineModuleType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, needsfilecontent: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, needsfilecontent: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retrievalaccount: *mut FsrmAccountType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retrievalaccount: FsrmAccountType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedextensions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertiesaffected: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertiesaffected: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertiesused: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertiesused: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, needsexplicitvalue: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, needsexplicitvalue: i16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmClassifierModuleImplementation(::windows::core::IUnknown);
impl IFsrmClassifierModuleImplementation {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn OnLoad<'a, Param0: ::windows::core::IntoParam<'a, IFsrmPipelineModuleDefinition>>(&self, moduledefinition: Param0) -> ::windows::core::Result<IFsrmPipelineModuleConnector> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), moduledefinition.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmPipelineModuleConnector>(result__)
    }
    pub unsafe fn OnUnload(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LastModified(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    pub unsafe fn UseRulesAndDefinitions<'a, Param0: ::windows::core::IntoParam<'a, IFsrmCollection>, Param1: ::windows::core::IntoParam<'a, IFsrmCollection>>(&self, rules: Param0, propertydefinitions: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), rules.into_param().abi(), propertydefinitions.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnBeginFile<'a, Param0: ::windows::core::IntoParam<'a, IFsrmPropertyBag>>(&self, propertybag: Param0, arrayruleids: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), propertybag.into_param().abi(), ::core::mem::transmute(arrayruleids)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesPropertyValueApply<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param4: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, property: Param0, value: Param1, applyvalue: *mut i16, idrule: Param3, idpropdef: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), property.into_param().abi(), value.into_param().abi(), ::core::mem::transmute(applyvalue), idrule.into_param().abi(), idpropdef.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyValueToApply<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param3: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, property: Param0, value: *mut super::super::Foundation::BSTR, idrule: Param2, idpropdef: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), property.into_param().abi(), ::core::mem::transmute(value), idrule.into_param().abi(), idpropdef.into_param().abi()).ok()
    }
    pub unsafe fn OnEndFile(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IFsrmClassifierModuleImplementation> for IFsrmPipelineModuleImplementation {
    fn from(value: IFsrmClassifierModuleImplementation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmClassifierModuleImplementation> for IFsrmPipelineModuleImplementation {
    fn from(value: &IFsrmClassifierModuleImplementation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmPipelineModuleImplementation> for IFsrmClassifierModuleImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmPipelineModuleImplementation> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmPipelineModuleImplementation> for &IFsrmClassifierModuleImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmPipelineModuleImplementation> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmClassifierModuleImplementation> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmClassifierModuleImplementation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmClassifierModuleImplementation> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmClassifierModuleImplementation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmClassifierModuleImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmClassifierModuleImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmClassifierModuleImplementation> for ::windows::core::IUnknown {
    fn from(value: IFsrmClassifierModuleImplementation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmClassifierModuleImplementation> for ::windows::core::IUnknown {
    fn from(value: &IFsrmClassifierModuleImplementation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmClassifierModuleImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmClassifierModuleImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmClassifierModuleImplementation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmClassifierModuleImplementation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmClassifierModuleImplementation {}
unsafe impl ::windows::core::Interface for IFsrmClassifierModuleImplementation {
    type Vtable = IFsrmClassifierModuleImplementationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c968fc6_6edb_4051_9c18_73b7291ae106);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassifierModuleImplementationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduledefinition: ::windows::core::RawPtr, moduleconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastmodified: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rules: ::windows::core::RawPtr, propertydefinitions: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertybag: ::windows::core::RawPtr, arrayruleids: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, applyvalue: *mut i16, idrule: ::windows::core::GUID, idpropdef: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut super::super::Foundation::BSTR, idrule: ::windows::core::GUID, idpropdef: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmCollection(::windows::core::IUnknown);
impl IFsrmCollection {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<FsrmCollectionState> {
        let mut result__: FsrmCollectionState = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmCollectionState>(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(waitseconds), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetById<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, id: Param0) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), id.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmCollection> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmCollection> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmCollection> for ::windows::core::IUnknown {
    fn from(value: IFsrmCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmCollection> for ::windows::core::IUnknown {
    fn from(value: &IFsrmCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmCollection {}
unsafe impl ::windows::core::Interface for IFsrmCollection {
    type Vtable = IFsrmCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf76fbf3b_8ddd_4b42_b05a_cb1c3ff1fee8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, item: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: *mut FsrmCollectionState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::windows::core::GUID, entry: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
pub struct IFsrmCommittableCollection(::windows::core::IUnknown);
impl IFsrmCommittableCollection {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<FsrmCollectionState> {
        let mut result__: FsrmCollectionState = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmCollectionState>(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(waitseconds), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetById<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, id: Param0) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), id.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, item: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), item.into_param().abi()).ok()
    }
    pub unsafe fn Remove(&self, index: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    pub unsafe fn RemoveById<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, id: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), id.into_param().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmMutableCollection>(result__)
    }
    pub unsafe fn Commit(&self, options: FsrmCommitOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
}
impl ::core::convert::From<IFsrmCommittableCollection> for IFsrmMutableCollection {
    fn from(value: IFsrmCommittableCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmCommittableCollection> for IFsrmMutableCollection {
    fn from(value: &IFsrmCommittableCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmMutableCollection> for IFsrmCommittableCollection {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmMutableCollection> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmMutableCollection> for &IFsrmCommittableCollection {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmMutableCollection> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmCommittableCollection> for IFsrmCollection {
    fn from(value: IFsrmCommittableCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmCommittableCollection> for IFsrmCollection {
    fn from(value: &IFsrmCommittableCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmCollection> for IFsrmCommittableCollection {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmCollection> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmCollection> for &IFsrmCommittableCollection {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmCollection> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmCommittableCollection> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmCommittableCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmCommittableCollection> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmCommittableCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmCommittableCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmCommittableCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmCommittableCollection> for ::windows::core::IUnknown {
    fn from(value: IFsrmCommittableCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmCommittableCollection> for ::windows::core::IUnknown {
    fn from(value: &IFsrmCommittableCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmCommittableCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmCommittableCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmCommittableCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmCommittableCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmCommittableCollection {}
unsafe impl ::windows::core::Interface for IFsrmCommittableCollection {
    type Vtable = IFsrmCommittableCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96deb3b5_8b91_4a2a_9d93_80a35d8aa847);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmCommittableCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, item: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: *mut FsrmCollectionState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::windows::core::GUID, entry: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FsrmCommitOptions, results: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmDerivedObjectsResult(::windows::core::IUnknown);
impl IFsrmDerivedObjectsResult {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn DerivedObjects(&self) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    pub unsafe fn Results(&self) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmDerivedObjectsResult> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmDerivedObjectsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmDerivedObjectsResult> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmDerivedObjectsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmDerivedObjectsResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmDerivedObjectsResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmDerivedObjectsResult> for ::windows::core::IUnknown {
    fn from(value: IFsrmDerivedObjectsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmDerivedObjectsResult> for ::windows::core::IUnknown {
    fn from(value: &IFsrmDerivedObjectsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmDerivedObjectsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmDerivedObjectsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmDerivedObjectsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmDerivedObjectsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmDerivedObjectsResult {}
unsafe impl ::windows::core::Interface for IFsrmDerivedObjectsResult {
    type Vtable = IFsrmDerivedObjectsResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39322a2d_38ee_4d0d_8095_421a80849a82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmDerivedObjectsResultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, derivedobjects: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, results: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmExportImport(::windows::core::IUnknown);
impl IFsrmExportImport {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ExportFileGroups<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, filegroupnamessafearray: *const super::super::System::Com::VARIANT, remotehost: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), filepath.into_param().abi(), ::core::mem::transmute(filegroupnamessafearray), remotehost.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ImportFileGroups<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, filegroupnamessafearray: *const super::super::System::Com::VARIANT, remotehost: Param2) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), filepath.into_param().abi(), ::core::mem::transmute(filegroupnamessafearray), remotehost.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ExportFileScreenTemplates<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), filepath.into_param().abi(), ::core::mem::transmute(templatenamessafearray), remotehost.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ImportFileScreenTemplates<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: Param2) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), filepath.into_param().abi(), ::core::mem::transmute(templatenamessafearray), remotehost.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ExportQuotaTemplates<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), filepath.into_param().abi(), ::core::mem::transmute(templatenamessafearray), remotehost.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ImportQuotaTemplates<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: Param2) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), filepath.into_param().abi(), ::core::mem::transmute(templatenamessafearray), remotehost.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmExportImport> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmExportImport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmExportImport> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmExportImport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmExportImport {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmExportImport {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmExportImport> for ::windows::core::IUnknown {
    fn from(value: IFsrmExportImport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmExportImport> for ::windows::core::IUnknown {
    fn from(value: &IFsrmExportImport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmExportImport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmExportImport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmExportImport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmExportImport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmExportImport {}
unsafe impl ::windows::core::Interface for IFsrmExportImport {
    type Vtable = IFsrmExportImportVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefcb0ab1_16c4_4a79_812c_725614c3306b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmExportImportVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filegroupnamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filegroupnamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filegroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, templates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, templates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
pub struct IFsrmFileCondition(::windows::core::IUnknown);
impl IFsrmFileCondition {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<FsrmFileConditionType> {
        let mut result__: FsrmFileConditionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmFileConditionType>(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileCondition> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmFileCondition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileCondition> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmFileCondition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmFileCondition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmFileCondition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmFileCondition> for ::windows::core::IUnknown {
    fn from(value: IFsrmFileCondition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmFileCondition> for ::windows::core::IUnknown {
    fn from(value: &IFsrmFileCondition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmFileCondition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmFileCondition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmFileCondition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmFileCondition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmFileCondition {}
unsafe impl ::windows::core::Interface for IFsrmFileCondition {
    type Vtable = IFsrmFileConditionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70684ffc_691a_4a1a_b922_97752e138cc1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileConditionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut FsrmFileConditionType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmFileConditionProperty(::windows::core::IUnknown);
impl IFsrmFileConditionProperty {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<FsrmFileConditionType> {
        let mut result__: FsrmFileConditionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmFileConditionType>(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PropertyName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPropertyName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn PropertyId(&self) -> ::windows::core::Result<FsrmFileSystemPropertyId> {
        let mut result__: FsrmFileSystemPropertyId = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmFileSystemPropertyId>(result__)
    }
    pub unsafe fn SetPropertyId(&self, newval: FsrmFileSystemPropertyId) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    pub unsafe fn Operator(&self) -> ::windows::core::Result<FsrmPropertyConditionType> {
        let mut result__: FsrmPropertyConditionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmPropertyConditionType>(result__)
    }
    pub unsafe fn SetOperator(&self, newval: FsrmPropertyConditionType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    pub unsafe fn ValueType(&self) -> ::windows::core::Result<FsrmPropertyValueType> {
        let mut result__: FsrmPropertyValueType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmPropertyValueType>(result__)
    }
    pub unsafe fn SetValueType(&self, newval: FsrmPropertyValueType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, newval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IFsrmFileConditionProperty> for IFsrmFileCondition {
    fn from(value: IFsrmFileConditionProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmFileConditionProperty> for IFsrmFileCondition {
    fn from(value: &IFsrmFileConditionProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmFileCondition> for IFsrmFileConditionProperty {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmFileCondition> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmFileCondition> for &IFsrmFileConditionProperty {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmFileCondition> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileConditionProperty> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmFileConditionProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileConditionProperty> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmFileConditionProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmFileConditionProperty {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmFileConditionProperty {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmFileConditionProperty> for ::windows::core::IUnknown {
    fn from(value: IFsrmFileConditionProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmFileConditionProperty> for ::windows::core::IUnknown {
    fn from(value: &IFsrmFileConditionProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmFileConditionProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmFileConditionProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmFileConditionProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmFileConditionProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmFileConditionProperty {}
unsafe impl ::windows::core::Interface for IFsrmFileConditionProperty {
    type Vtable = IFsrmFileConditionPropertyVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81926775_b981_4479_988f_da171d627360);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileConditionPropertyVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut FsrmFileConditionType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut FsrmFileSystemPropertyId) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: FsrmFileSystemPropertyId) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut FsrmPropertyConditionType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: FsrmPropertyConditionType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut FsrmPropertyValueType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: FsrmPropertyValueType) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
pub struct IFsrmFileGroup(::windows::core::IUnknown);
impl IFsrmFileGroup {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn Members(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmMutableCollection>(result__)
    }
    pub unsafe fn SetMembers<'a, Param0: ::windows::core::IntoParam<'a, IFsrmMutableCollection>>(&self, members: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), members.into_param().abi()).ok()
    }
    pub unsafe fn NonMembers(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmMutableCollection>(result__)
    }
    pub unsafe fn SetNonMembers<'a, Param0: ::windows::core::IntoParam<'a, IFsrmMutableCollection>>(&self, nonmembers: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), nonmembers.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IFsrmFileGroup> for IFsrmObject {
    fn from(value: IFsrmFileGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmFileGroup> for IFsrmObject {
    fn from(value: &IFsrmFileGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmFileGroup {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &IFsrmFileGroup {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileGroup> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmFileGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileGroup> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmFileGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmFileGroup {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmFileGroup {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmFileGroup> for ::windows::core::IUnknown {
    fn from(value: IFsrmFileGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmFileGroup> for ::windows::core::IUnknown {
    fn from(value: &IFsrmFileGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmFileGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmFileGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmFileGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmFileGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmFileGroup {}
unsafe impl ::windows::core::Interface for IFsrmFileGroup {
    type Vtable = IFsrmFileGroupVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8dd04909_0e34_4d55_afaa_89e1f1a1bbb9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileGroupVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, members: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, members: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nonmembers: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nonmembers: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmFileGroupImported(::windows::core::IUnknown);
impl IFsrmFileGroupImported {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn Members(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmMutableCollection>(result__)
    }
    pub unsafe fn SetMembers<'a, Param0: ::windows::core::IntoParam<'a, IFsrmMutableCollection>>(&self, members: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), members.into_param().abi()).ok()
    }
    pub unsafe fn NonMembers(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmMutableCollection>(result__)
    }
    pub unsafe fn SetNonMembers<'a, Param0: ::windows::core::IntoParam<'a, IFsrmMutableCollection>>(&self, nonmembers: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), nonmembers.into_param().abi()).ok()
    }
    pub unsafe fn OverwriteOnCommit(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    pub unsafe fn SetOverwriteOnCommit(&self, overwrite: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(overwrite)).ok()
    }
}
impl ::core::convert::From<IFsrmFileGroupImported> for IFsrmFileGroup {
    fn from(value: IFsrmFileGroupImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmFileGroupImported> for IFsrmFileGroup {
    fn from(value: &IFsrmFileGroupImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmFileGroup> for IFsrmFileGroupImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmFileGroup> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmFileGroup> for &IFsrmFileGroupImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmFileGroup> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmFileGroupImported> for IFsrmObject {
    fn from(value: IFsrmFileGroupImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmFileGroupImported> for IFsrmObject {
    fn from(value: &IFsrmFileGroupImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmFileGroupImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &IFsrmFileGroupImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileGroupImported> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmFileGroupImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileGroupImported> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmFileGroupImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmFileGroupImported {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmFileGroupImported {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmFileGroupImported> for ::windows::core::IUnknown {
    fn from(value: IFsrmFileGroupImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmFileGroupImported> for ::windows::core::IUnknown {
    fn from(value: &IFsrmFileGroupImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmFileGroupImported {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmFileGroupImported {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmFileGroupImported {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmFileGroupImported {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmFileGroupImported {}
unsafe impl ::windows::core::Interface for IFsrmFileGroupImported {
    type Vtable = IFsrmFileGroupImportedVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad55f10b_5f11_4be7_94ef_d9ee2e470ded);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileGroupImportedVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, members: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, members: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nonmembers: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nonmembers: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: i16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmFileGroupManager(::windows::core::IUnknown);
impl IFsrmFileGroupManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn CreateFileGroup(&self) -> ::windows::core::Result<IFsrmFileGroup> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmFileGroup>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFileGroup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<IFsrmFileGroup> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmFileGroup>(result__)
    }
    pub unsafe fn EnumFileGroups(&self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ExportFileGroups(&self, filegroupnamesarray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(filegroupnamesarray), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ImportFileGroups<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, serializedfilegroups: Param0, filegroupnamesarray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), serializedfilegroups.into_param().abi(), ::core::mem::transmute(filegroupnamesarray), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileGroupManager> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmFileGroupManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileGroupManager> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmFileGroupManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmFileGroupManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmFileGroupManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmFileGroupManager> for ::windows::core::IUnknown {
    fn from(value: IFsrmFileGroupManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmFileGroupManager> for ::windows::core::IUnknown {
    fn from(value: &IFsrmFileGroupManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmFileGroupManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmFileGroupManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmFileGroupManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmFileGroupManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmFileGroupManager {}
unsafe impl ::windows::core::Interface for IFsrmFileGroupManager {
    type Vtable = IFsrmFileGroupManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x426677d5_018c_485c_8a51_20b86d00bdc4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileGroupManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filegroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filegroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, filegroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filegroupnamesarray: *const super::super::System::Com::VARIANT, serializedfilegroups: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serializedfilegroups: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filegroupnamesarray: *const super::super::System::Com::VARIANT, filegroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
pub struct IFsrmFileManagementJob(::windows::core::IUnknown);
impl IFsrmFileManagementJob {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NamespaceRoots(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetNamespaceRoots(&self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(namespaceroots)).ok()
    }
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn OperationType(&self) -> ::windows::core::Result<FsrmFileManagementType> {
        let mut result__: FsrmFileManagementType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmFileManagementType>(result__)
    }
    pub unsafe fn SetOperationType(&self, operationtype: FsrmFileManagementType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(operationtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExpirationDirectory(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExpirationDirectory<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, expirationdirectory: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), expirationdirectory.into_param().abi()).ok()
    }
    pub unsafe fn CustomAction(&self) -> ::windows::core::Result<IFsrmActionCommand> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmActionCommand>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Notifications(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn Logging(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn SetLogging(&self, loggingflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(loggingflags)).ok()
    }
    pub unsafe fn ReportEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    pub unsafe fn SetReportEnabled(&self, reportenabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(reportenabled)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Formats(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFormats(&self, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(formats)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailTo(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailto: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), mailto.into_param().abi()).ok()
    }
    pub unsafe fn DaysSinceFileCreated(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn SetDaysSinceFileCreated(&self, dayssincecreation: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(dayssincecreation)).ok()
    }
    pub unsafe fn DaysSinceFileLastAccessed(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn SetDaysSinceFileLastAccessed(&self, dayssinceaccess: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(dayssinceaccess)).ok()
    }
    pub unsafe fn DaysSinceFileLastModified(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn SetDaysSinceFileLastModified(&self, dayssincemodify: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(dayssincemodify)).ok()
    }
    pub unsafe fn PropertyConditions(&self) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    pub unsafe fn FromDate(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    pub unsafe fn SetFromDate(&self, fromdate: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(fromdate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Task(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTask<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, taskname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), taskname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), ::core::mem::transmute(parameters)).ok()
    }
    pub unsafe fn RunningStatus(&self) -> ::windows::core::Result<FsrmReportRunningStatus> {
        let mut result__: FsrmReportRunningStatus = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmReportRunningStatus>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastError(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastReportPathWithoutExtension(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn LastRun(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileNamePattern(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileNamePattern<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filenamepattern: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), filenamepattern.into_param().abi()).ok()
    }
    pub unsafe fn Run(&self, context: FsrmReportGenerationContext) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(context)).ok()
    }
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), ::core::mem::transmute(waitseconds), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn AddNotification(&self, days: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(days)).ok()
    }
    pub unsafe fn DeleteNotification(&self, days: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).55)(::core::mem::transmute_copy(self), ::core::mem::transmute(days)).ok()
    }
    pub unsafe fn ModifyNotification(&self, days: i32, newdays: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).56)(::core::mem::transmute_copy(self), ::core::mem::transmute(days), ::core::mem::transmute(newdays)).ok()
    }
    pub unsafe fn CreateNotificationAction(&self, days: i32, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).57)(::core::mem::transmute_copy(self), ::core::mem::transmute(days), ::core::mem::transmute(actiontype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmAction>(result__)
    }
    pub unsafe fn EnumNotificationActions(&self, days: i32) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).58)(::core::mem::transmute_copy(self), ::core::mem::transmute(days), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreatePropertyCondition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<IFsrmPropertyCondition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).59)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmPropertyCondition>(result__)
    }
    pub unsafe fn CreateCustomAction(&self) -> ::windows::core::Result<IFsrmActionCommand> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).60)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmActionCommand>(result__)
    }
}
impl ::core::convert::From<IFsrmFileManagementJob> for IFsrmObject {
    fn from(value: IFsrmFileManagementJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmFileManagementJob> for IFsrmObject {
    fn from(value: &IFsrmFileManagementJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmFileManagementJob {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &IFsrmFileManagementJob {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileManagementJob> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmFileManagementJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileManagementJob> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmFileManagementJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmFileManagementJob {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmFileManagementJob {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmFileManagementJob> for ::windows::core::IUnknown {
    fn from(value: IFsrmFileManagementJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmFileManagementJob> for ::windows::core::IUnknown {
    fn from(value: &IFsrmFileManagementJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmFileManagementJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmFileManagementJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmFileManagementJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmFileManagementJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmFileManagementJob {}
unsafe impl ::windows::core::Interface for IFsrmFileManagementJob {
    type Vtable = IFsrmFileManagementJobVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0770687e_9f36_4d6f_8778_599d188461c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileManagementJobVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operationtype: *mut FsrmFileManagementType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operationtype: FsrmFileManagementType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, expirationdirectory: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, expirationdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notifications: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loggingflags: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loggingflags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportenabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportenabled: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dayssincecreation: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dayssincecreation: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dayssinceaccess: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dayssinceaccess: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dayssincemodify: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dayssincemodify: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyconditions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fromdate: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fromdate: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runningstatus: *mut FsrmReportRunningStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lasterror: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastrun: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filenamepattern: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filenamepattern: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: i32, newdays: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: i32, actiontype: FsrmActionType, action: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: i32, actions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertycondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmFileManagementJobManager(::windows::core::IUnknown);
impl IFsrmFileManagementJobManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariables(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariableDescriptions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn EnumFileManagementJobs(&self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    pub unsafe fn CreateFileManagementJob(&self) -> ::windows::core::Result<IFsrmFileManagementJob> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmFileManagementJob>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFileManagementJob<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<IFsrmFileManagementJob> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmFileManagementJob>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileManagementJobManager> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmFileManagementJobManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileManagementJobManager> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmFileManagementJobManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmFileManagementJobManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmFileManagementJobManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmFileManagementJobManager> for ::windows::core::IUnknown {
    fn from(value: IFsrmFileManagementJobManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmFileManagementJobManager> for ::windows::core::IUnknown {
    fn from(value: &IFsrmFileManagementJobManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmFileManagementJobManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmFileManagementJobManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmFileManagementJobManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmFileManagementJobManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmFileManagementJobManager {}
unsafe impl ::windows::core::Interface for IFsrmFileManagementJobManager {
    type Vtable = IFsrmFileManagementJobManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee321ecb_d95e_48e9_907c_c7685a013235);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileManagementJobManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, filemanagementjobs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filemanagementjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filemanagementjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IFsrmFileScreen(::windows::core::IUnknown);
impl IFsrmFileScreen {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn BlockedFileGroups(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmMutableCollection>(result__)
    }
    pub unsafe fn SetBlockedFileGroups<'a, Param0: ::windows::core::IntoParam<'a, IFsrmMutableCollection>>(&self, blocklist: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), blocklist.into_param().abi()).ok()
    }
    pub unsafe fn FileScreenFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn SetFileScreenFlags(&self, filescreenflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(filescreenflags)).ok()
    }
    pub unsafe fn CreateAction(&self, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(actiontype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmAction>(result__)
    }
    pub unsafe fn EnumActions(&self) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SourceTemplateName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn MatchesSourceTemplate(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserSid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserAccount(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplyTemplate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filescreentemplatename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), filescreentemplatename.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IFsrmFileScreen> for IFsrmFileScreenBase {
    fn from(value: IFsrmFileScreen) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmFileScreen> for IFsrmFileScreenBase {
    fn from(value: &IFsrmFileScreen) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmFileScreenBase> for IFsrmFileScreen {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmFileScreenBase> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmFileScreenBase> for &IFsrmFileScreen {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmFileScreenBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmFileScreen> for IFsrmObject {
    fn from(value: IFsrmFileScreen) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmFileScreen> for IFsrmObject {
    fn from(value: &IFsrmFileScreen) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmFileScreen {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &IFsrmFileScreen {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileScreen> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmFileScreen) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileScreen> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmFileScreen) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmFileScreen {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmFileScreen {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmFileScreen> for ::windows::core::IUnknown {
    fn from(value: IFsrmFileScreen) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmFileScreen> for ::windows::core::IUnknown {
    fn from(value: &IFsrmFileScreen) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmFileScreen {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmFileScreen {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmFileScreen {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmFileScreen {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmFileScreen {}
unsafe impl ::windows::core::Interface for IFsrmFileScreen {
    type Vtable = IFsrmFileScreenVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f6325d3_ce88_4733_84c1_2d6aefc5ea07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blocklist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blocklist: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreenflags: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreenflags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actiontype: FsrmActionType, action: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreentemplatename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matches: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccount: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreentemplatename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IFsrmFileScreenBase(::windows::core::IUnknown);
impl IFsrmFileScreenBase {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn BlockedFileGroups(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmMutableCollection>(result__)
    }
    pub unsafe fn SetBlockedFileGroups<'a, Param0: ::windows::core::IntoParam<'a, IFsrmMutableCollection>>(&self, blocklist: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), blocklist.into_param().abi()).ok()
    }
    pub unsafe fn FileScreenFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn SetFileScreenFlags(&self, filescreenflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(filescreenflags)).ok()
    }
    pub unsafe fn CreateAction(&self, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(actiontype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmAction>(result__)
    }
    pub unsafe fn EnumActions(&self) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
}
impl ::core::convert::From<IFsrmFileScreenBase> for IFsrmObject {
    fn from(value: IFsrmFileScreenBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmFileScreenBase> for IFsrmObject {
    fn from(value: &IFsrmFileScreenBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmFileScreenBase {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &IFsrmFileScreenBase {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileScreenBase> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmFileScreenBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileScreenBase> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmFileScreenBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmFileScreenBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmFileScreenBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmFileScreenBase> for ::windows::core::IUnknown {
    fn from(value: IFsrmFileScreenBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmFileScreenBase> for ::windows::core::IUnknown {
    fn from(value: &IFsrmFileScreenBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmFileScreenBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmFileScreenBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmFileScreenBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmFileScreenBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmFileScreenBase {}
unsafe impl ::windows::core::Interface for IFsrmFileScreenBase {
    type Vtable = IFsrmFileScreenBaseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3637e80_5b22_4a2b_a637_bbb642b41cfc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenBaseVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blocklist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blocklist: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreenflags: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreenflags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actiontype: FsrmActionType, action: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmFileScreenException(::windows::core::IUnknown);
impl IFsrmFileScreenException {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn AllowedFileGroups(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmMutableCollection>(result__)
    }
    pub unsafe fn SetAllowedFileGroups<'a, Param0: ::windows::core::IntoParam<'a, IFsrmMutableCollection>>(&self, allowlist: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), allowlist.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IFsrmFileScreenException> for IFsrmObject {
    fn from(value: IFsrmFileScreenException) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmFileScreenException> for IFsrmObject {
    fn from(value: &IFsrmFileScreenException) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmFileScreenException {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &IFsrmFileScreenException {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileScreenException> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmFileScreenException) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileScreenException> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmFileScreenException) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmFileScreenException {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmFileScreenException {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmFileScreenException> for ::windows::core::IUnknown {
    fn from(value: IFsrmFileScreenException) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmFileScreenException> for ::windows::core::IUnknown {
    fn from(value: &IFsrmFileScreenException) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmFileScreenException {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmFileScreenException {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmFileScreenException {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmFileScreenException {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmFileScreenException {}
unsafe impl ::windows::core::Interface for IFsrmFileScreenException {
    type Vtable = IFsrmFileScreenExceptionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbee7ce02_df77_4515_9389_78f01c5afc1a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenExceptionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allowlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allowlist: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmFileScreenManager(::windows::core::IUnknown);
impl IFsrmFileScreenManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariables(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariableDescriptions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateFileScreen<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<IFsrmFileScreen> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmFileScreen>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFileScreen<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<IFsrmFileScreen> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmFileScreen>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumFileScreens<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateFileScreenException<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<IFsrmFileScreenException> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmFileScreenException>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFileScreenException<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<IFsrmFileScreenException> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmFileScreenException>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumFileScreenExceptions<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
    pub unsafe fn CreateFileScreenCollection(&self) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileScreenManager> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmFileScreenManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileScreenManager> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmFileScreenManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmFileScreenManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmFileScreenManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmFileScreenManager> for ::windows::core::IUnknown {
    fn from(value: IFsrmFileScreenManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmFileScreenManager> for ::windows::core::IUnknown {
    fn from(value: &IFsrmFileScreenManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmFileScreenManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmFileScreenManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmFileScreenManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmFileScreenManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmFileScreenManager {}
unsafe impl ::windows::core::Interface for IFsrmFileScreenManager {
    type Vtable = IFsrmFileScreenManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff4fa04e_5a94_4bda_a3a0_d5b4d3c52eba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreen: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreen: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, filescreens: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreenexception: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreenexception: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, filescreenexceptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmFileScreenTemplate(::windows::core::IUnknown);
impl IFsrmFileScreenTemplate {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn BlockedFileGroups(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmMutableCollection>(result__)
    }
    pub unsafe fn SetBlockedFileGroups<'a, Param0: ::windows::core::IntoParam<'a, IFsrmMutableCollection>>(&self, blocklist: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), blocklist.into_param().abi()).ok()
    }
    pub unsafe fn FileScreenFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn SetFileScreenFlags(&self, filescreenflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(filescreenflags)).ok()
    }
    pub unsafe fn CreateAction(&self, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(actiontype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmAction>(result__)
    }
    pub unsafe fn EnumActions(&self) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTemplate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filescreentemplatename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), filescreentemplatename.into_param().abi()).ok()
    }
    pub unsafe fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows::core::Result<IFsrmDerivedObjectsResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(commitoptions), ::core::mem::transmute(applyoptions), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmDerivedObjectsResult>(result__)
    }
}
impl ::core::convert::From<IFsrmFileScreenTemplate> for IFsrmFileScreenBase {
    fn from(value: IFsrmFileScreenTemplate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmFileScreenTemplate> for IFsrmFileScreenBase {
    fn from(value: &IFsrmFileScreenTemplate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmFileScreenBase> for IFsrmFileScreenTemplate {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmFileScreenBase> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmFileScreenBase> for &IFsrmFileScreenTemplate {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmFileScreenBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmFileScreenTemplate> for IFsrmObject {
    fn from(value: IFsrmFileScreenTemplate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmFileScreenTemplate> for IFsrmObject {
    fn from(value: &IFsrmFileScreenTemplate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmFileScreenTemplate {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &IFsrmFileScreenTemplate {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileScreenTemplate> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmFileScreenTemplate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileScreenTemplate> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmFileScreenTemplate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmFileScreenTemplate {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmFileScreenTemplate {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmFileScreenTemplate> for ::windows::core::IUnknown {
    fn from(value: IFsrmFileScreenTemplate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmFileScreenTemplate> for ::windows::core::IUnknown {
    fn from(value: &IFsrmFileScreenTemplate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmFileScreenTemplate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmFileScreenTemplate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmFileScreenTemplate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmFileScreenTemplate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmFileScreenTemplate {}
unsafe impl ::windows::core::Interface for IFsrmFileScreenTemplate {
    type Vtable = IFsrmFileScreenTemplateVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x205bebf8_dd93_452a_95a6_32b566b35828);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenTemplateVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blocklist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blocklist: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreenflags: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreenflags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actiontype: FsrmActionType, action: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreentemplatename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmFileScreenTemplateImported(::windows::core::IUnknown);
impl IFsrmFileScreenTemplateImported {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn BlockedFileGroups(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmMutableCollection>(result__)
    }
    pub unsafe fn SetBlockedFileGroups<'a, Param0: ::windows::core::IntoParam<'a, IFsrmMutableCollection>>(&self, blocklist: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), blocklist.into_param().abi()).ok()
    }
    pub unsafe fn FileScreenFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn SetFileScreenFlags(&self, filescreenflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(filescreenflags)).ok()
    }
    pub unsafe fn CreateAction(&self, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(actiontype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmAction>(result__)
    }
    pub unsafe fn EnumActions(&self) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTemplate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filescreentemplatename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), filescreentemplatename.into_param().abi()).ok()
    }
    pub unsafe fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows::core::Result<IFsrmDerivedObjectsResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(commitoptions), ::core::mem::transmute(applyoptions), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmDerivedObjectsResult>(result__)
    }
    pub unsafe fn OverwriteOnCommit(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    pub unsafe fn SetOverwriteOnCommit(&self, overwrite: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(overwrite)).ok()
    }
}
impl ::core::convert::From<IFsrmFileScreenTemplateImported> for IFsrmFileScreenTemplate {
    fn from(value: IFsrmFileScreenTemplateImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmFileScreenTemplateImported> for IFsrmFileScreenTemplate {
    fn from(value: &IFsrmFileScreenTemplateImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmFileScreenTemplate> for IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmFileScreenTemplate> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmFileScreenTemplate> for &IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmFileScreenTemplate> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmFileScreenTemplateImported> for IFsrmFileScreenBase {
    fn from(value: IFsrmFileScreenTemplateImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmFileScreenTemplateImported> for IFsrmFileScreenBase {
    fn from(value: &IFsrmFileScreenTemplateImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmFileScreenBase> for IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmFileScreenBase> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmFileScreenBase> for &IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmFileScreenBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmFileScreenTemplateImported> for IFsrmObject {
    fn from(value: IFsrmFileScreenTemplateImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmFileScreenTemplateImported> for IFsrmObject {
    fn from(value: &IFsrmFileScreenTemplateImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileScreenTemplateImported> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmFileScreenTemplateImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileScreenTemplateImported> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmFileScreenTemplateImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmFileScreenTemplateImported> for ::windows::core::IUnknown {
    fn from(value: IFsrmFileScreenTemplateImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmFileScreenTemplateImported> for ::windows::core::IUnknown {
    fn from(value: &IFsrmFileScreenTemplateImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmFileScreenTemplateImported {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmFileScreenTemplateImported {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmFileScreenTemplateImported {}
unsafe impl ::windows::core::Interface for IFsrmFileScreenTemplateImported {
    type Vtable = IFsrmFileScreenTemplateImportedVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1010359_3e5d_4ecd_9fe4_ef48622fdf30);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenTemplateImportedVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blocklist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blocklist: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreenflags: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreenflags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actiontype: FsrmActionType, action: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreentemplatename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: i16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmFileScreenTemplateManager(::windows::core::IUnknown);
impl IFsrmFileScreenTemplateManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn CreateTemplate(&self) -> ::windows::core::Result<IFsrmFileScreenTemplate> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmFileScreenTemplate>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTemplate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<IFsrmFileScreenTemplate> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmFileScreenTemplate>(result__)
    }
    pub unsafe fn EnumTemplates(&self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ExportTemplates(&self, filescreentemplatenamesarray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(filescreentemplatenamesarray), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ImportTemplates<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, serializedfilescreentemplates: Param0, filescreentemplatenamesarray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), serializedfilescreentemplates.into_param().abi(), ::core::mem::transmute(filescreentemplatenamesarray), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileScreenTemplateManager> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmFileScreenTemplateManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileScreenTemplateManager> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmFileScreenTemplateManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmFileScreenTemplateManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmFileScreenTemplateManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmFileScreenTemplateManager> for ::windows::core::IUnknown {
    fn from(value: IFsrmFileScreenTemplateManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmFileScreenTemplateManager> for ::windows::core::IUnknown {
    fn from(value: &IFsrmFileScreenTemplateManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmFileScreenTemplateManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmFileScreenTemplateManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmFileScreenTemplateManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmFileScreenTemplateManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmFileScreenTemplateManager {}
unsafe impl ::windows::core::Interface for IFsrmFileScreenTemplateManager {
    type Vtable = IFsrmFileScreenTemplateManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfe36cba_1949_4e74_a14f_f1d580ceaf13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenTemplateManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreentemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreentemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, filescreentemplates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreentemplatenamesarray: *const super::super::System::Com::VARIANT, serializedfilescreentemplates: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serializedfilescreentemplates: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreentemplatenamesarray: *const super::super::System::Com::VARIANT, filescreentemplates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
pub struct IFsrmMutableCollection(::windows::core::IUnknown);
impl IFsrmMutableCollection {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<FsrmCollectionState> {
        let mut result__: FsrmCollectionState = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmCollectionState>(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(waitseconds), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetById<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, id: Param0) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), id.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, item: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), item.into_param().abi()).ok()
    }
    pub unsafe fn Remove(&self, index: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    pub unsafe fn RemoveById<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, id: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), id.into_param().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmMutableCollection>(result__)
    }
}
impl ::core::convert::From<IFsrmMutableCollection> for IFsrmCollection {
    fn from(value: IFsrmMutableCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmMutableCollection> for IFsrmCollection {
    fn from(value: &IFsrmMutableCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmCollection> for IFsrmMutableCollection {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmCollection> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmCollection> for &IFsrmMutableCollection {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmCollection> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmMutableCollection> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmMutableCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmMutableCollection> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmMutableCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmMutableCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmMutableCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmMutableCollection> for ::windows::core::IUnknown {
    fn from(value: IFsrmMutableCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmMutableCollection> for ::windows::core::IUnknown {
    fn from(value: &IFsrmMutableCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmMutableCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmMutableCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmMutableCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmMutableCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmMutableCollection {}
unsafe impl ::windows::core::Interface for IFsrmMutableCollection {
    type Vtable = IFsrmMutableCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bb617b8_3886_49dc_af82_a6c90fa35dda);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmMutableCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, item: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: *mut FsrmCollectionState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::windows::core::GUID, entry: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmObject(::windows::core::IUnknown);
impl IFsrmObject {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmObject> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmObject> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmObject {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmObject {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmObject> for ::windows::core::IUnknown {
    fn from(value: IFsrmObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmObject> for ::windows::core::IUnknown {
    fn from(value: &IFsrmObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmObject {}
unsafe impl ::windows::core::Interface for IFsrmObject {
    type Vtable = IFsrmObjectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22bcef93_4a3f_4183_89f9_2f8b8a628aee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmObjectVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmPathMapper(::windows::core::IUnknown);
impl IFsrmPathMapper {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetSharePathsForLocalPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, localpath: Param0) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), localpath.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmPathMapper> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmPathMapper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmPathMapper> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmPathMapper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmPathMapper {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmPathMapper {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmPathMapper> for ::windows::core::IUnknown {
    fn from(value: IFsrmPathMapper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmPathMapper> for ::windows::core::IUnknown {
    fn from(value: &IFsrmPathMapper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmPathMapper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmPathMapper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmPathMapper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmPathMapper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmPathMapper {}
unsafe impl ::windows::core::Interface for IFsrmPathMapper {
    type Vtable = IFsrmPathMapperVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f4dbfff_6920_4821_a6c3_b7e94c1fd60c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPathMapperVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sharepaths: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
pub struct IFsrmPipelineModuleConnector(::windows::core::IUnknown);
impl IFsrmPipelineModuleConnector {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn ModuleImplementation(&self) -> ::windows::core::Result<IFsrmPipelineModuleImplementation> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmPipelineModuleImplementation>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModuleName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HostingUserAccount(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn HostingProcessPid(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn Bind<'a, Param0: ::windows::core::IntoParam<'a, IFsrmPipelineModuleDefinition>, Param1: ::windows::core::IntoParam<'a, IFsrmPipelineModuleImplementation>>(&self, moduledefinition: Param0, moduleimplementation: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), moduledefinition.into_param().abi(), moduleimplementation.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmPipelineModuleConnector> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmPipelineModuleConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmPipelineModuleConnector> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmPipelineModuleConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmPipelineModuleConnector {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmPipelineModuleConnector {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmPipelineModuleConnector> for ::windows::core::IUnknown {
    fn from(value: IFsrmPipelineModuleConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmPipelineModuleConnector> for ::windows::core::IUnknown {
    fn from(value: &IFsrmPipelineModuleConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmPipelineModuleConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmPipelineModuleConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmPipelineModuleConnector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmPipelineModuleConnector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmPipelineModuleConnector {}
unsafe impl ::windows::core::Interface for IFsrmPipelineModuleConnector {
    type Vtable = IFsrmPipelineModuleConnectorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc16014f3_9aa1_46b3_b0a7_ab146eb205f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPipelineModuleConnectorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipelinemoduleimplementation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccount: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduledefinition: ::windows::core::RawPtr, moduleimplementation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmPipelineModuleDefinition(::windows::core::IUnknown);
impl IFsrmPipelineModuleDefinition {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModuleClsid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetModuleClsid<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, moduleclsid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), moduleclsid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Company(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCompany<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, company: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), company.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Version(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVersion<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, version: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), version.into_param().abi()).ok()
    }
    pub unsafe fn ModuleType(&self) -> ::windows::core::Result<FsrmPipelineModuleType> {
        let mut result__: FsrmPipelineModuleType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmPipelineModuleType>(result__)
    }
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn NeedsFileContent(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    pub unsafe fn SetNeedsFileContent(&self, needsfilecontent: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(needsfilecontent)).ok()
    }
    pub unsafe fn Account(&self) -> ::windows::core::Result<FsrmAccountType> {
        let mut result__: FsrmAccountType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmAccountType>(result__)
    }
    pub unsafe fn SetAccount(&self, retrievalaccount: FsrmAccountType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(retrievalaccount)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedExtensions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSupportedExtensions(&self, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(supportedextensions)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(parameters)).ok()
    }
}
impl ::core::convert::From<IFsrmPipelineModuleDefinition> for IFsrmObject {
    fn from(value: IFsrmPipelineModuleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmPipelineModuleDefinition> for IFsrmObject {
    fn from(value: &IFsrmPipelineModuleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmPipelineModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &IFsrmPipelineModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmPipelineModuleDefinition> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmPipelineModuleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmPipelineModuleDefinition> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmPipelineModuleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmPipelineModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmPipelineModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmPipelineModuleDefinition> for ::windows::core::IUnknown {
    fn from(value: IFsrmPipelineModuleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmPipelineModuleDefinition> for ::windows::core::IUnknown {
    fn from(value: &IFsrmPipelineModuleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmPipelineModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmPipelineModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmPipelineModuleDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmPipelineModuleDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmPipelineModuleDefinition {}
unsafe impl ::windows::core::Interface for IFsrmPipelineModuleDefinition {
    type Vtable = IFsrmPipelineModuleDefinitionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x515c1277_2c81_440e_8fcf_367921ed4f59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPipelineModuleDefinitionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleclsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, company: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, company: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, version: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, version: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduletype: *mut FsrmPipelineModuleType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, needsfilecontent: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, needsfilecontent: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retrievalaccount: *mut FsrmAccountType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retrievalaccount: FsrmAccountType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedextensions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
pub struct IFsrmPipelineModuleImplementation(::windows::core::IUnknown);
impl IFsrmPipelineModuleImplementation {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn OnLoad<'a, Param0: ::windows::core::IntoParam<'a, IFsrmPipelineModuleDefinition>>(&self, moduledefinition: Param0) -> ::windows::core::Result<IFsrmPipelineModuleConnector> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), moduledefinition.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmPipelineModuleConnector>(result__)
    }
    pub unsafe fn OnUnload(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmPipelineModuleImplementation> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmPipelineModuleImplementation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmPipelineModuleImplementation> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmPipelineModuleImplementation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmPipelineModuleImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmPipelineModuleImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmPipelineModuleImplementation> for ::windows::core::IUnknown {
    fn from(value: IFsrmPipelineModuleImplementation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmPipelineModuleImplementation> for ::windows::core::IUnknown {
    fn from(value: &IFsrmPipelineModuleImplementation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmPipelineModuleImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmPipelineModuleImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmPipelineModuleImplementation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmPipelineModuleImplementation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmPipelineModuleImplementation {}
unsafe impl ::windows::core::Interface for IFsrmPipelineModuleImplementation {
    type Vtable = IFsrmPipelineModuleImplementationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7907906_2b02_4cb5_84a9_fdf54613d6cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPipelineModuleImplementationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduledefinition: ::windows::core::RawPtr, moduleconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmProperty(::windows::core::IUnknown);
impl IFsrmProperty {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Sources(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn PropertyFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmProperty> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmProperty> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmProperty {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmProperty {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmProperty> for ::windows::core::IUnknown {
    fn from(value: IFsrmProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmProperty> for ::windows::core::IUnknown {
    fn from(value: &IFsrmProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmProperty {}
unsafe impl ::windows::core::Interface for IFsrmProperty {
    type Vtable = IFsrmPropertyVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a73fee4_4102_4fcc_9ffb_38614f9ee768);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sources: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmPropertyBag(::windows::core::IUnknown);
impl IFsrmPropertyBag {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RelativePath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VolumeName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RelativeNamespaceRoot(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn VolumeIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn FileId(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ParentDirectoryId(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Size(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SizeAllocated(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreationTime(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LastAccessTime(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LastModificationTime(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    pub unsafe fn Attributes(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OwnerSid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FilePropertyNames(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Messages(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn PropertyBagFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFileProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<IFsrmProperty> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmProperty>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0, value: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), name.into_param().abi(), value.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddMessage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, message: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), message.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetFileStreamInterface(&self, accessmode: FsrmFileStreamingMode, interfacetype: FsrmFileStreamingInterfaceType) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(accessmode), ::core::mem::transmute(interfacetype), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmPropertyBag> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmPropertyBag) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmPropertyBag> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmPropertyBag) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmPropertyBag {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmPropertyBag {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmPropertyBag> for ::windows::core::IUnknown {
    fn from(value: IFsrmPropertyBag) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmPropertyBag> for ::windows::core::IUnknown {
    fn from(value: &IFsrmPropertyBag) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmPropertyBag {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmPropertyBag {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmPropertyBag {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmPropertyBag {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmPropertyBag {}
unsafe impl ::windows::core::Interface for IFsrmPropertyBag {
    type Vtable = IFsrmPropertyBagVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x774589d1_d300_4f7a_9a24_f7b766800250);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyBagVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, volumename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativenamespaceroot: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, volumeid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fileid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parentdirectoryid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sizeallocated: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, creationtime: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastaccesstime: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastmodificationtime: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributes: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ownersid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepropertynames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messages: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fileproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accessmode: FsrmFileStreamingMode, interfacetype: FsrmFileStreamingInterfaceType, pstreaminterface: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
pub struct IFsrmPropertyBag2(::windows::core::IUnknown);
impl IFsrmPropertyBag2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RelativePath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VolumeName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RelativeNamespaceRoot(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn VolumeIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn FileId(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ParentDirectoryId(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Size(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SizeAllocated(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreationTime(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LastAccessTime(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LastModificationTime(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    pub unsafe fn Attributes(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OwnerSid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FilePropertyNames(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Messages(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn PropertyBagFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFileProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<IFsrmProperty> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmProperty>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0, value: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), name.into_param().abi(), value.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddMessage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, message: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), message.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetFileStreamInterface(&self, accessmode: FsrmFileStreamingMode, interfacetype: FsrmFileStreamingInterfaceType) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(accessmode), ::core::mem::transmute(interfacetype), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetFieldValue(&self, field: FsrmPropertyBagField) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(field), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    pub unsafe fn GetUntrustedInFileProperties(&self) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
}
impl ::core::convert::From<IFsrmPropertyBag2> for IFsrmPropertyBag {
    fn from(value: IFsrmPropertyBag2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmPropertyBag2> for IFsrmPropertyBag {
    fn from(value: &IFsrmPropertyBag2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmPropertyBag> for IFsrmPropertyBag2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmPropertyBag> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmPropertyBag> for &IFsrmPropertyBag2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmPropertyBag> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmPropertyBag2> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmPropertyBag2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmPropertyBag2> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmPropertyBag2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmPropertyBag2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmPropertyBag2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmPropertyBag2> for ::windows::core::IUnknown {
    fn from(value: IFsrmPropertyBag2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmPropertyBag2> for ::windows::core::IUnknown {
    fn from(value: &IFsrmPropertyBag2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmPropertyBag2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmPropertyBag2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmPropertyBag2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmPropertyBag2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmPropertyBag2 {}
unsafe impl ::windows::core::Interface for IFsrmPropertyBag2 {
    type Vtable = IFsrmPropertyBag2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e46bdbd_2402_4fed_9c30_9266e6eb2cc9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyBag2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, volumename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativenamespaceroot: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, volumeid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fileid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parentdirectoryid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sizeallocated: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, creationtime: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastaccesstime: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastmodificationtime: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributes: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ownersid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepropertynames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messages: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fileproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accessmode: FsrmFileStreamingMode, interfacetype: FsrmFileStreamingInterfaceType, pstreaminterface: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, field: FsrmPropertyBagField, value: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, props: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmPropertyCondition(::windows::core::IUnknown);
impl IFsrmPropertyCondition {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<FsrmPropertyConditionType> {
        let mut result__: FsrmPropertyConditionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmPropertyConditionType>(result__)
    }
    pub unsafe fn SetType(&self, r#type: FsrmPropertyConditionType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, value: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmPropertyCondition> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmPropertyCondition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmPropertyCondition> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmPropertyCondition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmPropertyCondition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmPropertyCondition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmPropertyCondition> for ::windows::core::IUnknown {
    fn from(value: IFsrmPropertyCondition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmPropertyCondition> for ::windows::core::IUnknown {
    fn from(value: &IFsrmPropertyCondition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmPropertyCondition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmPropertyCondition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmPropertyCondition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmPropertyCondition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmPropertyCondition {}
unsafe impl ::windows::core::Interface for IFsrmPropertyCondition {
    type Vtable = IFsrmPropertyConditionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x326af66f_2ac0_4f68_bf8c_4759f054fa29);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyConditionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut FsrmPropertyConditionType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: FsrmPropertyConditionType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmPropertyDefinition(::windows::core::IUnknown);
impl IFsrmPropertyDefinition {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<FsrmPropertyDefinitionType> {
        let mut result__: FsrmPropertyDefinitionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmPropertyDefinitionType>(result__)
    }
    pub unsafe fn SetType(&self, r#type: FsrmPropertyDefinitionType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PossibleValues(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPossibleValues(&self, possiblevalues: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(possiblevalues)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ValueDescriptions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetValueDescriptions(&self, valuedescriptions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(valuedescriptions)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(parameters)).ok()
    }
}
impl ::core::convert::From<IFsrmPropertyDefinition> for IFsrmObject {
    fn from(value: IFsrmPropertyDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmPropertyDefinition> for IFsrmObject {
    fn from(value: &IFsrmPropertyDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmPropertyDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &IFsrmPropertyDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmPropertyDefinition> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmPropertyDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmPropertyDefinition> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmPropertyDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmPropertyDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmPropertyDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmPropertyDefinition> for ::windows::core::IUnknown {
    fn from(value: IFsrmPropertyDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmPropertyDefinition> for ::windows::core::IUnknown {
    fn from(value: &IFsrmPropertyDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmPropertyDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmPropertyDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmPropertyDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmPropertyDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmPropertyDefinition {}
unsafe impl ::windows::core::Interface for IFsrmPropertyDefinition {
    type Vtable = IFsrmPropertyDefinitionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xede0150f_e9a3_419c_877c_01fe5d24c5d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyDefinitionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut FsrmPropertyDefinitionType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: FsrmPropertyDefinitionType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, possiblevalues: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, possiblevalues: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, valuedescriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, valuedescriptions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
pub struct IFsrmPropertyDefinition2(::windows::core::IUnknown);
impl IFsrmPropertyDefinition2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<FsrmPropertyDefinitionType> {
        let mut result__: FsrmPropertyDefinitionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmPropertyDefinitionType>(result__)
    }
    pub unsafe fn SetType(&self, r#type: FsrmPropertyDefinitionType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PossibleValues(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPossibleValues(&self, possiblevalues: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(possiblevalues)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ValueDescriptions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetValueDescriptions(&self, valuedescriptions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(valuedescriptions)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(parameters)).ok()
    }
    pub unsafe fn PropertyDefinitionFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn AppliesTo(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn ValueDefinitions(&self) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
}
impl ::core::convert::From<IFsrmPropertyDefinition2> for IFsrmPropertyDefinition {
    fn from(value: IFsrmPropertyDefinition2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmPropertyDefinition2> for IFsrmPropertyDefinition {
    fn from(value: &IFsrmPropertyDefinition2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmPropertyDefinition> for IFsrmPropertyDefinition2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmPropertyDefinition> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmPropertyDefinition> for &IFsrmPropertyDefinition2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmPropertyDefinition> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmPropertyDefinition2> for IFsrmObject {
    fn from(value: IFsrmPropertyDefinition2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmPropertyDefinition2> for IFsrmObject {
    fn from(value: &IFsrmPropertyDefinition2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmPropertyDefinition2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &IFsrmPropertyDefinition2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmPropertyDefinition2> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmPropertyDefinition2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmPropertyDefinition2> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmPropertyDefinition2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmPropertyDefinition2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmPropertyDefinition2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmPropertyDefinition2> for ::windows::core::IUnknown {
    fn from(value: IFsrmPropertyDefinition2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmPropertyDefinition2> for ::windows::core::IUnknown {
    fn from(value: &IFsrmPropertyDefinition2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmPropertyDefinition2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmPropertyDefinition2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmPropertyDefinition2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmPropertyDefinition2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmPropertyDefinition2 {}
unsafe impl ::windows::core::Interface for IFsrmPropertyDefinition2 {
    type Vtable = IFsrmPropertyDefinition2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47782152_d16c_4229_b4e1_0ddfe308b9f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyDefinition2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut FsrmPropertyDefinitionType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: FsrmPropertyDefinitionType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, possiblevalues: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, possiblevalues: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, valuedescriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, valuedescriptions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertydefinitionflags: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appliesto: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, valuedefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmPropertyDefinitionValue(::windows::core::IUnknown);
impl IFsrmPropertyDefinitionValue {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UniqueID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmPropertyDefinitionValue> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmPropertyDefinitionValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmPropertyDefinitionValue> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmPropertyDefinitionValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmPropertyDefinitionValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmPropertyDefinitionValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmPropertyDefinitionValue> for ::windows::core::IUnknown {
    fn from(value: IFsrmPropertyDefinitionValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmPropertyDefinitionValue> for ::windows::core::IUnknown {
    fn from(value: &IFsrmPropertyDefinitionValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmPropertyDefinitionValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmPropertyDefinitionValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmPropertyDefinitionValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmPropertyDefinitionValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmPropertyDefinitionValue {}
unsafe impl ::windows::core::Interface for IFsrmPropertyDefinitionValue {
    type Vtable = IFsrmPropertyDefinitionValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe946d148_bd67_4178_8e22_1c44925ed710);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyDefinitionValueVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uniqueid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IFsrmQuota(::windows::core::IUnknown);
impl IFsrmQuota {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn QuotaLimit(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetQuotaLimit<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, quotalimit: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), quotalimit.into_param().abi()).ok()
    }
    pub unsafe fn QuotaFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(quotaflags)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Thresholds(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn AddThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold)).ok()
    }
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold)).ok()
    }
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(newthreshold)).ok()
    }
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(actiontype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmAction>(result__)
    }
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserSid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserAccount(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SourceTemplateName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn MatchesSourceTemplate(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplyTemplate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, quotatemplatename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), quotatemplatename.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn QuotaUsed(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn QuotaPeakUsage(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    pub unsafe fn QuotaPeakUsageTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    pub unsafe fn ResetPeakUsage(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn RefreshUsageProperties(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IFsrmQuota> for IFsrmQuotaObject {
    fn from(value: IFsrmQuota) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmQuota> for IFsrmQuotaObject {
    fn from(value: &IFsrmQuota) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaObject> for IFsrmQuota {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaObject> for &IFsrmQuota {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmQuota> for IFsrmQuotaBase {
    fn from(value: IFsrmQuota) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmQuota> for IFsrmQuotaBase {
    fn from(value: &IFsrmQuota) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaBase> for IFsrmQuota {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaBase> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaBase> for &IFsrmQuota {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmQuota> for IFsrmObject {
    fn from(value: IFsrmQuota) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmQuota> for IFsrmObject {
    fn from(value: &IFsrmQuota) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmQuota {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &IFsrmQuota {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmQuota> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmQuota) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmQuota> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmQuota) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmQuota {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmQuota {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmQuota> for ::windows::core::IUnknown {
    fn from(value: IFsrmQuota) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmQuota> for ::windows::core::IUnknown {
    fn from(value: &IFsrmQuota) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmQuota {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmQuota {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmQuota {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmQuota {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmQuota {}
unsafe impl ::windows::core::Interface for IFsrmQuota {
    type Vtable = IFsrmQuotaVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x377f739d_9647_4b8e_97d2_5ffce6d759cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotalimit: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotalimit: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotaflags: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotaflags: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, thresholds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32, newthreshold: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32, actiontype: FsrmActionType, action: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32, actions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccount: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotatemplatename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matches: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotatemplatename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, used: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peakusage: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peakusagedatetime: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmQuotaBase(::windows::core::IUnknown);
impl IFsrmQuotaBase {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn QuotaLimit(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetQuotaLimit<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, quotalimit: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), quotalimit.into_param().abi()).ok()
    }
    pub unsafe fn QuotaFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(quotaflags)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Thresholds(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn AddThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold)).ok()
    }
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold)).ok()
    }
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(newthreshold)).ok()
    }
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(actiontype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmAction>(result__)
    }
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
}
impl ::core::convert::From<IFsrmQuotaBase> for IFsrmObject {
    fn from(value: IFsrmQuotaBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmQuotaBase> for IFsrmObject {
    fn from(value: &IFsrmQuotaBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmQuotaBase {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &IFsrmQuotaBase {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmQuotaBase> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmQuotaBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmQuotaBase> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmQuotaBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmQuotaBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmQuotaBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmQuotaBase> for ::windows::core::IUnknown {
    fn from(value: IFsrmQuotaBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmQuotaBase> for ::windows::core::IUnknown {
    fn from(value: &IFsrmQuotaBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmQuotaBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmQuotaBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmQuotaBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmQuotaBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmQuotaBase {}
unsafe impl ::windows::core::Interface for IFsrmQuotaBase {
    type Vtable = IFsrmQuotaBaseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1568a795_3924_4118_b74b_68d8f0fa5daf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaBaseVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotalimit: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotalimit: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotaflags: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotaflags: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, thresholds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32, newthreshold: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32, actiontype: FsrmActionType, action: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32, actions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmQuotaManager(::windows::core::IUnknown);
impl IFsrmQuotaManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariables(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariableDescriptions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateQuota<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<IFsrmQuota> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmQuota>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateAutoApplyQuota<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, quotatemplatename: Param0, path: Param1) -> ::windows::core::Result<IFsrmAutoApplyQuota> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), quotatemplatename.into_param().abi(), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmAutoApplyQuota>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetQuota<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<IFsrmQuota> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmQuota>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAutoApplyQuota<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<IFsrmAutoApplyQuota> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmAutoApplyQuota>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestrictiveQuota<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<IFsrmQuota> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmQuota>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumQuotas<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumAutoApplyQuotas<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumEffectiveQuotas<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Scan<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strpath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), strpath.into_param().abi()).ok()
    }
    pub unsafe fn CreateQuotaCollection(&self) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmQuotaManager> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmQuotaManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmQuotaManager> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmQuotaManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmQuotaManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmQuotaManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmQuotaManager> for ::windows::core::IUnknown {
    fn from(value: IFsrmQuotaManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmQuotaManager> for ::windows::core::IUnknown {
    fn from(value: &IFsrmQuotaManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmQuotaManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmQuotaManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmQuotaManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmQuotaManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmQuotaManager {}
unsafe impl ::windows::core::Interface for IFsrmQuotaManager {
    type Vtable = IFsrmQuotaManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bb68c7d_19d8_4ffb_809e_be4fc1734014);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotatemplatename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, quotas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, quotas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, quotas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmQuotaManagerEx(::windows::core::IUnknown);
impl IFsrmQuotaManagerEx {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariables(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariableDescriptions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateQuota<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<IFsrmQuota> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmQuota>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateAutoApplyQuota<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, quotatemplatename: Param0, path: Param1) -> ::windows::core::Result<IFsrmAutoApplyQuota> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), quotatemplatename.into_param().abi(), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmAutoApplyQuota>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetQuota<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<IFsrmQuota> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmQuota>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAutoApplyQuota<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<IFsrmAutoApplyQuota> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmAutoApplyQuota>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestrictiveQuota<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<IFsrmQuota> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmQuota>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumQuotas<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumAutoApplyQuotas<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumEffectiveQuotas<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Scan<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strpath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), strpath.into_param().abi()).ok()
    }
    pub unsafe fn CreateQuotaCollection(&self) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsAffectedByQuota<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
}
impl ::core::convert::From<IFsrmQuotaManagerEx> for IFsrmQuotaManager {
    fn from(value: IFsrmQuotaManagerEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmQuotaManagerEx> for IFsrmQuotaManager {
    fn from(value: &IFsrmQuotaManagerEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaManager> for IFsrmQuotaManagerEx {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaManager> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaManager> for &IFsrmQuotaManagerEx {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaManager> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmQuotaManagerEx> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmQuotaManagerEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmQuotaManagerEx> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmQuotaManagerEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmQuotaManagerEx {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmQuotaManagerEx {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmQuotaManagerEx> for ::windows::core::IUnknown {
    fn from(value: IFsrmQuotaManagerEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmQuotaManagerEx> for ::windows::core::IUnknown {
    fn from(value: &IFsrmQuotaManagerEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmQuotaManagerEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmQuotaManagerEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmQuotaManagerEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmQuotaManagerEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmQuotaManagerEx {}
unsafe impl ::windows::core::Interface for IFsrmQuotaManagerEx {
    type Vtable = IFsrmQuotaManagerExVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4846cb01_d430_494f_abb4_b1054999fb09);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaManagerExVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotatemplatename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, quotas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, quotas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, quotas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, affected: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IFsrmQuotaObject(::windows::core::IUnknown);
impl IFsrmQuotaObject {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn QuotaLimit(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetQuotaLimit<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, quotalimit: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), quotalimit.into_param().abi()).ok()
    }
    pub unsafe fn QuotaFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(quotaflags)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Thresholds(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn AddThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold)).ok()
    }
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold)).ok()
    }
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(newthreshold)).ok()
    }
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(actiontype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmAction>(result__)
    }
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserSid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserAccount(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SourceTemplateName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn MatchesSourceTemplate(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplyTemplate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, quotatemplatename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), quotatemplatename.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IFsrmQuotaObject> for IFsrmQuotaBase {
    fn from(value: IFsrmQuotaObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmQuotaObject> for IFsrmQuotaBase {
    fn from(value: &IFsrmQuotaObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaBase> for IFsrmQuotaObject {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaBase> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaBase> for &IFsrmQuotaObject {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmQuotaObject> for IFsrmObject {
    fn from(value: IFsrmQuotaObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmQuotaObject> for IFsrmObject {
    fn from(value: &IFsrmQuotaObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmQuotaObject {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &IFsrmQuotaObject {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmQuotaObject> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmQuotaObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmQuotaObject> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmQuotaObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmQuotaObject {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmQuotaObject {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmQuotaObject> for ::windows::core::IUnknown {
    fn from(value: IFsrmQuotaObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmQuotaObject> for ::windows::core::IUnknown {
    fn from(value: &IFsrmQuotaObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmQuotaObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmQuotaObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmQuotaObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmQuotaObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmQuotaObject {}
unsafe impl ::windows::core::Interface for IFsrmQuotaObject {
    type Vtable = IFsrmQuotaObjectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42dc3511_61d5_48ae_b6dc_59fc00c0a8d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaObjectVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotalimit: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotalimit: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotaflags: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotaflags: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, thresholds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32, newthreshold: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32, actiontype: FsrmActionType, action: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32, actions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccount: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotatemplatename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matches: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotatemplatename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IFsrmQuotaTemplate(::windows::core::IUnknown);
impl IFsrmQuotaTemplate {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn QuotaLimit(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetQuotaLimit<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, quotalimit: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), quotalimit.into_param().abi()).ok()
    }
    pub unsafe fn QuotaFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(quotaflags)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Thresholds(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn AddThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold)).ok()
    }
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold)).ok()
    }
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(newthreshold)).ok()
    }
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(actiontype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmAction>(result__)
    }
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTemplate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, quotatemplatename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), quotatemplatename.into_param().abi()).ok()
    }
    pub unsafe fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows::core::Result<IFsrmDerivedObjectsResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(commitoptions), ::core::mem::transmute(applyoptions), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmDerivedObjectsResult>(result__)
    }
}
impl ::core::convert::From<IFsrmQuotaTemplate> for IFsrmQuotaBase {
    fn from(value: IFsrmQuotaTemplate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmQuotaTemplate> for IFsrmQuotaBase {
    fn from(value: &IFsrmQuotaTemplate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaBase> for IFsrmQuotaTemplate {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaBase> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaBase> for &IFsrmQuotaTemplate {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmQuotaTemplate> for IFsrmObject {
    fn from(value: IFsrmQuotaTemplate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmQuotaTemplate> for IFsrmObject {
    fn from(value: &IFsrmQuotaTemplate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmQuotaTemplate {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &IFsrmQuotaTemplate {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmQuotaTemplate> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmQuotaTemplate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmQuotaTemplate> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmQuotaTemplate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmQuotaTemplate {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmQuotaTemplate {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmQuotaTemplate> for ::windows::core::IUnknown {
    fn from(value: IFsrmQuotaTemplate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmQuotaTemplate> for ::windows::core::IUnknown {
    fn from(value: &IFsrmQuotaTemplate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmQuotaTemplate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmQuotaTemplate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmQuotaTemplate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmQuotaTemplate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmQuotaTemplate {}
unsafe impl ::windows::core::Interface for IFsrmQuotaTemplate {
    type Vtable = IFsrmQuotaTemplateVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2efab31_295e_46bb_b976_e86d58b52e8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaTemplateVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotalimit: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotalimit: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotaflags: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotaflags: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, thresholds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32, newthreshold: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32, actiontype: FsrmActionType, action: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32, actions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotatemplatename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmQuotaTemplateImported(::windows::core::IUnknown);
impl IFsrmQuotaTemplateImported {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn QuotaLimit(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetQuotaLimit<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, quotalimit: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), quotalimit.into_param().abi()).ok()
    }
    pub unsafe fn QuotaFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(quotaflags)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Thresholds(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn AddThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold)).ok()
    }
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold)).ok()
    }
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(newthreshold)).ok()
    }
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(actiontype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmAction>(result__)
    }
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTemplate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, quotatemplatename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), quotatemplatename.into_param().abi()).ok()
    }
    pub unsafe fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows::core::Result<IFsrmDerivedObjectsResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(commitoptions), ::core::mem::transmute(applyoptions), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmDerivedObjectsResult>(result__)
    }
    pub unsafe fn OverwriteOnCommit(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    pub unsafe fn SetOverwriteOnCommit(&self, overwrite: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(overwrite)).ok()
    }
}
impl ::core::convert::From<IFsrmQuotaTemplateImported> for IFsrmQuotaTemplate {
    fn from(value: IFsrmQuotaTemplateImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmQuotaTemplateImported> for IFsrmQuotaTemplate {
    fn from(value: &IFsrmQuotaTemplateImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaTemplate> for IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaTemplate> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaTemplate> for &IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaTemplate> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmQuotaTemplateImported> for IFsrmQuotaBase {
    fn from(value: IFsrmQuotaTemplateImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmQuotaTemplateImported> for IFsrmQuotaBase {
    fn from(value: &IFsrmQuotaTemplateImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaBase> for IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaBase> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaBase> for &IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmQuotaTemplateImported> for IFsrmObject {
    fn from(value: IFsrmQuotaTemplateImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmQuotaTemplateImported> for IFsrmObject {
    fn from(value: &IFsrmQuotaTemplateImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmQuotaTemplateImported> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmQuotaTemplateImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmQuotaTemplateImported> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmQuotaTemplateImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmQuotaTemplateImported> for ::windows::core::IUnknown {
    fn from(value: IFsrmQuotaTemplateImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmQuotaTemplateImported> for ::windows::core::IUnknown {
    fn from(value: &IFsrmQuotaTemplateImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmQuotaTemplateImported {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmQuotaTemplateImported {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmQuotaTemplateImported {}
unsafe impl ::windows::core::Interface for IFsrmQuotaTemplateImported {
    type Vtable = IFsrmQuotaTemplateImportedVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a2bf113_a329_44cc_809a_5c00fce8da40);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaTemplateImportedVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotalimit: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotalimit: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotaflags: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotaflags: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, thresholds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32, newthreshold: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32, actiontype: FsrmActionType, action: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32, actions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotatemplatename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: i16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmQuotaTemplateManager(::windows::core::IUnknown);
impl IFsrmQuotaTemplateManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn CreateTemplate(&self) -> ::windows::core::Result<IFsrmQuotaTemplate> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmQuotaTemplate>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTemplate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<IFsrmQuotaTemplate> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmQuotaTemplate>(result__)
    }
    pub unsafe fn EnumTemplates(&self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ExportTemplates(&self, quotatemplatenamesarray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(quotatemplatenamesarray), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ImportTemplates<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, serializedquotatemplates: Param0, quotatemplatenamesarray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), serializedquotatemplates.into_param().abi(), ::core::mem::transmute(quotatemplatenamesarray), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmQuotaTemplateManager> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmQuotaTemplateManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmQuotaTemplateManager> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmQuotaTemplateManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmQuotaTemplateManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmQuotaTemplateManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmQuotaTemplateManager> for ::windows::core::IUnknown {
    fn from(value: IFsrmQuotaTemplateManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmQuotaTemplateManager> for ::windows::core::IUnknown {
    fn from(value: &IFsrmQuotaTemplateManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmQuotaTemplateManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmQuotaTemplateManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmQuotaTemplateManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmQuotaTemplateManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmQuotaTemplateManager {}
unsafe impl ::windows::core::Interface for IFsrmQuotaTemplateManager {
    type Vtable = IFsrmQuotaTemplateManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4173ac41_172d_4d52_963c_fdc7e415f717);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaTemplateManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotatemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quotatemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, quotatemplates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotatemplatenamesarray: *const super::super::System::Com::VARIANT, serializedquotatemplates: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serializedquotatemplates: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quotatemplatenamesarray: *const super::super::System::Com::VARIANT, quotatemplates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
pub struct IFsrmReport(::windows::core::IUnknown);
impl IFsrmReport {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<FsrmReportType> {
        let mut result__: FsrmReportType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmReportType>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastGeneratedFileNamePrefix(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetFilter(&self, filter: FsrmReportFilter) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(filter), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetFilter<'a, Param1: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, filter: FsrmReportFilter, filtervalue: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(filter), filtervalue.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmReport> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmReport> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmReport {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmReport {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmReport> for ::windows::core::IUnknown {
    fn from(value: IFsrmReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmReport> for ::windows::core::IUnknown {
    fn from(value: &IFsrmReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmReport {}
unsafe impl ::windows::core::Interface for IFsrmReport {
    type Vtable = IFsrmReportVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8cc81d9_46b8_4fa4_bfa5_4aa9dec9b638);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmReportVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: *mut FsrmReportType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prefix: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filter: FsrmReportFilter, filtervalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filter: FsrmReportFilter, filtervalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmReportJob(::windows::core::IUnknown);
impl IFsrmReportJob {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Task(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTask<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, taskname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), taskname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NamespaceRoots(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetNamespaceRoots(&self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(namespaceroots)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Formats(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFormats(&self, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(formats)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailTo(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailto: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), mailto.into_param().abi()).ok()
    }
    pub unsafe fn RunningStatus(&self) -> ::windows::core::Result<FsrmReportRunningStatus> {
        let mut result__: FsrmReportRunningStatus = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmReportRunningStatus>(result__)
    }
    pub unsafe fn LastRun(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastError(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastGeneratedInDirectory(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn EnumReports(&self) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    pub unsafe fn CreateReport(&self, reporttype: FsrmReportType) -> ::windows::core::Result<IFsrmReport> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(reporttype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmReport>(result__)
    }
    pub unsafe fn Run(&self, context: FsrmReportGenerationContext) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(context)).ok()
    }
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(waitseconds), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IFsrmReportJob> for IFsrmObject {
    fn from(value: IFsrmReportJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmReportJob> for IFsrmObject {
    fn from(value: &IFsrmReportJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmReportJob {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &IFsrmReportJob {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmReportJob> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmReportJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmReportJob> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmReportJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmReportJob {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmReportJob {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmReportJob> for ::windows::core::IUnknown {
    fn from(value: IFsrmReportJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmReportJob> for ::windows::core::IUnknown {
    fn from(value: &IFsrmReportJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmReportJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmReportJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmReportJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmReportJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmReportJob {}
unsafe impl ::windows::core::Interface for IFsrmReportJob {
    type Vtable = IFsrmReportJobVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38e87280_715c_4c7d_a280_ea1651a19fef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmReportJobVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runningstatus: *mut FsrmReportRunningStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastrun: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lasterror: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reports: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, report: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmReportManager(::windows::core::IUnknown);
impl IFsrmReportManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn EnumReportJobs(&self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    pub unsafe fn CreateReportJob(&self) -> ::windows::core::Result<IFsrmReportJob> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmReportJob>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReportJob<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, taskname: Param0) -> ::windows::core::Result<IFsrmReportJob> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), taskname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmReportJob>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOutputDirectory(&self, context: FsrmReportGenerationContext) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(context), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOutputDirectory<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, context: FsrmReportGenerationContext, path: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(context), path.into_param().abi()).ok()
    }
    pub unsafe fn IsFilterValidForReportType(&self, reporttype: FsrmReportType, filter: FsrmReportFilter) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(reporttype), ::core::mem::transmute(filter), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetDefaultFilter(&self, reporttype: FsrmReportType, filter: FsrmReportFilter) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(reporttype), ::core::mem::transmute(filter), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetDefaultFilter<'a, Param2: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(reporttype), ::core::mem::transmute(filter), filtervalue.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetReportSizeLimit(&self, limit: FsrmReportLimit) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(limit), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetReportSizeLimit<'a, Param1: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, limit: FsrmReportLimit, limitvalue: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(limit), limitvalue.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmReportManager> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmReportManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmReportManager> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmReportManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmReportManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmReportManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmReportManager> for ::windows::core::IUnknown {
    fn from(value: IFsrmReportManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmReportManager> for ::windows::core::IUnknown {
    fn from(value: &IFsrmReportManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmReportManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmReportManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmReportManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmReportManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmReportManager {}
unsafe impl ::windows::core::Interface for IFsrmReportManager {
    type Vtable = IFsrmReportManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27b899fe_6ffa_4481_a184_d3daade8a02b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmReportManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, reportjobs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, reportjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, filter: FsrmReportFilter, valid: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, limit: FsrmReportLimit, limitvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, limit: FsrmReportLimit, limitvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
pub struct IFsrmReportScheduler(::windows::core::IUnknown);
impl IFsrmReportScheduler {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn VerifyNamespaces(&self, namespacessafearray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(namespacessafearray)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateScheduleTask<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, taskname: Param0, namespacessafearray: *const super::super::System::Com::VARIANT, serializedtask: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), taskname.into_param().abi(), ::core::mem::transmute(namespacessafearray), serializedtask.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ModifyScheduleTask<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, taskname: Param0, namespacessafearray: *const super::super::System::Com::VARIANT, serializedtask: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), taskname.into_param().abi(), ::core::mem::transmute(namespacessafearray), serializedtask.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteScheduleTask<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, taskname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), taskname.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmReportScheduler> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmReportScheduler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmReportScheduler> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmReportScheduler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmReportScheduler {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmReportScheduler {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmReportScheduler> for ::windows::core::IUnknown {
    fn from(value: IFsrmReportScheduler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmReportScheduler> for ::windows::core::IUnknown {
    fn from(value: &IFsrmReportScheduler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmReportScheduler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmReportScheduler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmReportScheduler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmReportScheduler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmReportScheduler {}
unsafe impl ::windows::core::Interface for IFsrmReportScheduler {
    type Vtable = IFsrmReportSchedulerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6879caf9_6617_4484_8719_71c3d8645f94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmReportSchedulerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespacessafearray: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, namespacessafearray: *const super::super::System::Com::VARIANT, serializedtask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, namespacessafearray: *const super::super::System::Com::VARIANT, serializedtask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IFsrmRule(::windows::core::IUnknown);
impl IFsrmRule {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn RuleType(&self) -> ::windows::core::Result<FsrmRuleType> {
        let mut result__: FsrmRuleType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmRuleType>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModuleDefinitionName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetModuleDefinitionName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, moduledefinitionname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), moduledefinitionname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NamespaceRoots(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetNamespaceRoots(&self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(namespaceroots)).ok()
    }
    pub unsafe fn RuleFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRuleFlags(&self, ruleflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(ruleflags)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(parameters)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LastModified(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
}
impl ::core::convert::From<IFsrmRule> for IFsrmObject {
    fn from(value: IFsrmRule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmRule> for IFsrmObject {
    fn from(value: &IFsrmRule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmRule {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &IFsrmRule {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmRule> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmRule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmRule> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmRule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmRule {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmRule {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmRule> for ::windows::core::IUnknown {
    fn from(value: IFsrmRule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmRule> for ::windows::core::IUnknown {
    fn from(value: &IFsrmRule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmRule {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmRule {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmRule {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmRule {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmRule {}
unsafe impl ::windows::core::Interface for IFsrmRule {
    type Vtable = IFsrmRuleVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb0df960_16f5_4495_9079_3f9360d831df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmRuleVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ruletype: *mut FsrmRuleType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduledefinitionname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ruleflags: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ruleflags: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastmodified: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
pub struct IFsrmSetting(::windows::core::IUnknown);
impl IFsrmSetting {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SmtpServer(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSmtpServer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, smtpserver: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), smtpserver.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailFrom(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailFrom<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailfrom: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), mailfrom.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AdminEmail(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAdminEmail<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, adminemail: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), adminemail.into_param().abi()).ok()
    }
    pub unsafe fn DisableCommandLine(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    pub unsafe fn SetDisableCommandLine(&self, disablecommandline: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(disablecommandline)).ok()
    }
    pub unsafe fn EnableScreeningAudit(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnableScreeningAudit(&self, enablescreeningaudit: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(enablescreeningaudit)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EmailTest<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailto: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), mailto.into_param().abi()).ok()
    }
    pub unsafe fn SetActionRunLimitInterval(&self, actiontype: FsrmActionType, delaytimeminutes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(actiontype), ::core::mem::transmute(delaytimeminutes)).ok()
    }
    pub unsafe fn GetActionRunLimitInterval(&self, actiontype: FsrmActionType) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(actiontype), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmSetting> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmSetting) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmSetting> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmSetting) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmSetting {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmSetting {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmSetting> for ::windows::core::IUnknown {
    fn from(value: IFsrmSetting) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmSetting> for ::windows::core::IUnknown {
    fn from(value: &IFsrmSetting) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmSetting {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmSetting {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmSetting {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmSetting {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmSetting {}
unsafe impl ::windows::core::Interface for IFsrmSetting {
    type Vtable = IFsrmSettingVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf411d4fd_14be_4260_8c40_03b7c95e608a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmSettingVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smtpserver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smtpserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailfrom: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailfrom: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adminemail: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adminemail: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disablecommandline: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disablecommandline: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enablescreeningaudit: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enablescreeningaudit: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actiontype: FsrmActionType, delaytimeminutes: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actiontype: FsrmActionType, delaytimeminutes: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmStorageModuleDefinition(::windows::core::IUnknown);
impl IFsrmStorageModuleDefinition {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModuleClsid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetModuleClsid<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, moduleclsid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), moduleclsid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Company(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCompany<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, company: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), company.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Version(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVersion<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, version: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), version.into_param().abi()).ok()
    }
    pub unsafe fn ModuleType(&self) -> ::windows::core::Result<FsrmPipelineModuleType> {
        let mut result__: FsrmPipelineModuleType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmPipelineModuleType>(result__)
    }
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn NeedsFileContent(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    pub unsafe fn SetNeedsFileContent(&self, needsfilecontent: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(needsfilecontent)).ok()
    }
    pub unsafe fn Account(&self) -> ::windows::core::Result<FsrmAccountType> {
        let mut result__: FsrmAccountType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmAccountType>(result__)
    }
    pub unsafe fn SetAccount(&self, retrievalaccount: FsrmAccountType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(retrievalaccount)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedExtensions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSupportedExtensions(&self, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(supportedextensions)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(parameters)).ok()
    }
    pub unsafe fn Capabilities(&self) -> ::windows::core::Result<FsrmStorageModuleCaps> {
        let mut result__: FsrmStorageModuleCaps = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmStorageModuleCaps>(result__)
    }
    pub unsafe fn SetCapabilities(&self, capabilities: FsrmStorageModuleCaps) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(capabilities)).ok()
    }
    pub unsafe fn StorageType(&self) -> ::windows::core::Result<FsrmStorageModuleType> {
        let mut result__: FsrmStorageModuleType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmStorageModuleType>(result__)
    }
    pub unsafe fn SetStorageType(&self, storagetype: FsrmStorageModuleType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(storagetype)).ok()
    }
    pub unsafe fn UpdatesFileContent(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    pub unsafe fn SetUpdatesFileContent(&self, updatesfilecontent: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(updatesfilecontent)).ok()
    }
}
impl ::core::convert::From<IFsrmStorageModuleDefinition> for IFsrmPipelineModuleDefinition {
    fn from(value: IFsrmStorageModuleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmStorageModuleDefinition> for IFsrmPipelineModuleDefinition {
    fn from(value: &IFsrmStorageModuleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmPipelineModuleDefinition> for IFsrmStorageModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmPipelineModuleDefinition> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmPipelineModuleDefinition> for &IFsrmStorageModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmPipelineModuleDefinition> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmStorageModuleDefinition> for IFsrmObject {
    fn from(value: IFsrmStorageModuleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmStorageModuleDefinition> for IFsrmObject {
    fn from(value: &IFsrmStorageModuleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmStorageModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &IFsrmStorageModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmStorageModuleDefinition> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmStorageModuleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmStorageModuleDefinition> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmStorageModuleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmStorageModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmStorageModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmStorageModuleDefinition> for ::windows::core::IUnknown {
    fn from(value: IFsrmStorageModuleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmStorageModuleDefinition> for ::windows::core::IUnknown {
    fn from(value: &IFsrmStorageModuleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmStorageModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmStorageModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmStorageModuleDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmStorageModuleDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmStorageModuleDefinition {}
unsafe impl ::windows::core::Interface for IFsrmStorageModuleDefinition {
    type Vtable = IFsrmStorageModuleDefinitionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15a81350_497d_4aba_80e9_d4dbcc5521fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmStorageModuleDefinitionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleclsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, company: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, company: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, version: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, version: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduletype: *mut FsrmPipelineModuleType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, needsfilecontent: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, needsfilecontent: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retrievalaccount: *mut FsrmAccountType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retrievalaccount: FsrmAccountType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedextensions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capabilities: *mut FsrmStorageModuleCaps) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capabilities: FsrmStorageModuleCaps) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storagetype: *mut FsrmStorageModuleType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storagetype: FsrmStorageModuleType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updatesfilecontent: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updatesfilecontent: i16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFsrmStorageModuleImplementation(::windows::core::IUnknown);
impl IFsrmStorageModuleImplementation {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::ITypeInfo>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn OnLoad<'a, Param0: ::windows::core::IntoParam<'a, IFsrmPipelineModuleDefinition>>(&self, moduledefinition: Param0) -> ::windows::core::Result<IFsrmPipelineModuleConnector> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), moduledefinition.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmPipelineModuleConnector>(result__)
    }
    pub unsafe fn OnUnload(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn UseDefinitions<'a, Param0: ::windows::core::IntoParam<'a, IFsrmCollection>>(&self, propertydefinitions: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), propertydefinitions.into_param().abi()).ok()
    }
    pub unsafe fn LoadProperties<'a, Param0: ::windows::core::IntoParam<'a, IFsrmPropertyBag>>(&self, propertybag: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), propertybag.into_param().abi()).ok()
    }
    pub unsafe fn SaveProperties<'a, Param0: ::windows::core::IntoParam<'a, IFsrmPropertyBag>>(&self, propertybag: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), propertybag.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IFsrmStorageModuleImplementation> for IFsrmPipelineModuleImplementation {
    fn from(value: IFsrmStorageModuleImplementation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmStorageModuleImplementation> for IFsrmPipelineModuleImplementation {
    fn from(value: &IFsrmStorageModuleImplementation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmPipelineModuleImplementation> for IFsrmStorageModuleImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmPipelineModuleImplementation> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFsrmPipelineModuleImplementation> for &IFsrmStorageModuleImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmPipelineModuleImplementation> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmStorageModuleImplementation> for super::super::System::Com::IDispatch {
    fn from(value: IFsrmStorageModuleImplementation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmStorageModuleImplementation> for super::super::System::Com::IDispatch {
    fn from(value: &IFsrmStorageModuleImplementation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IFsrmStorageModuleImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IFsrmStorageModuleImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFsrmStorageModuleImplementation> for ::windows::core::IUnknown {
    fn from(value: IFsrmStorageModuleImplementation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFsrmStorageModuleImplementation> for ::windows::core::IUnknown {
    fn from(value: &IFsrmStorageModuleImplementation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmStorageModuleImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFsrmStorageModuleImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFsrmStorageModuleImplementation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFsrmStorageModuleImplementation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFsrmStorageModuleImplementation {}
unsafe impl ::windows::core::Interface for IFsrmStorageModuleImplementation {
    type Vtable = IFsrmStorageModuleImplementationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0af4a0da_895a_4e50_8712_a96724bcec64);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmStorageModuleImplementationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduledefinition: ::windows::core::RawPtr, moduleconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertydefinitions: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertybag: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertybag: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
pub const MessageSizeLimit: u32 = 4096u32;
