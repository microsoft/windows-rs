#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub const AdSyncTask: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ae64751_b728_4d6b_97a0_b2da2e7d2a3b);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AdrClientDisplayFlags(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrClientDisplayFlags_AllowEmailRequests: AdrClientDisplayFlags = AdrClientDisplayFlags(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrClientDisplayFlags_ShowDeviceTroubleshooting: AdrClientDisplayFlags = AdrClientDisplayFlags(2i32);
impl ::core::marker::Copy for AdrClientDisplayFlags {}
impl ::core::clone::Clone for AdrClientDisplayFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AdrClientDisplayFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AdrClientDisplayFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for AdrClientDisplayFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdrClientDisplayFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AdrClientErrorType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrClientErrorType_Unknown: AdrClientErrorType = AdrClientErrorType(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrClientErrorType_AccessDenied: AdrClientErrorType = AdrClientErrorType(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrClientErrorType_FileNotFound: AdrClientErrorType = AdrClientErrorType(2i32);
impl ::core::marker::Copy for AdrClientErrorType {}
impl ::core::clone::Clone for AdrClientErrorType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AdrClientErrorType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AdrClientErrorType {
    type Abi = Self;
}
impl ::core::fmt::Debug for AdrClientErrorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdrClientErrorType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AdrClientFlags(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrClientFlags_None: AdrClientFlags = AdrClientFlags(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrClientFlags_FailForLocalPaths: AdrClientFlags = AdrClientFlags(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrClientFlags_FailIfNotSupportedByServer: AdrClientFlags = AdrClientFlags(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrClientFlags_FailIfNotDomainJoined: AdrClientFlags = AdrClientFlags(4i32);
impl ::core::marker::Copy for AdrClientFlags {}
impl ::core::clone::Clone for AdrClientFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AdrClientFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AdrClientFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for AdrClientFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdrClientFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AdrEmailFlags(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrEmailFlags_PutDataOwnerOnToLine: AdrEmailFlags = AdrEmailFlags(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrEmailFlags_PutAdminOnToLine: AdrEmailFlags = AdrEmailFlags(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrEmailFlags_IncludeDeviceClaims: AdrEmailFlags = AdrEmailFlags(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrEmailFlags_IncludeUserInfo: AdrEmailFlags = AdrEmailFlags(8i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrEmailFlags_GenerateEventLog: AdrEmailFlags = AdrEmailFlags(16i32);
impl ::core::marker::Copy for AdrEmailFlags {}
impl ::core::clone::Clone for AdrEmailFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AdrEmailFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AdrEmailFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for AdrEmailFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdrEmailFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct DIFsrmClassificationEvents(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl DIFsrmClassificationEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DIFsrmClassificationEvents> for ::windows::core::IUnknown {
    fn from(value: DIFsrmClassificationEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DIFsrmClassificationEvents> for ::windows::core::IUnknown {
    fn from(value: &DIFsrmClassificationEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DIFsrmClassificationEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DIFsrmClassificationEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a DIFsrmClassificationEvents {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for DIFsrmClassificationEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for DIFsrmClassificationEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for DIFsrmClassificationEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for DIFsrmClassificationEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIFsrmClassificationEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for DIFsrmClassificationEvents {
    type Vtable = DIFsrmClassificationEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26942db0_dabf_41d8_bbdd_b129a9f70424);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DIFsrmClassificationEvents_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_FEATURE_CLASSIFICATION: u32 = 83886080u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_FEATURE_FILESCREEN: u32 = 50331648u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_FEATURE_GENERAL: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_FEATURE_MASK: u32 = 251658240u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_FEATURE_PIPELINE: u32 = 100663296u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_FEATURE_QUOTA: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_FEATURE_REPORTS: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_INTERFACE_A_MASK: u32 = 15728640u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_INTERFACE_B_MASK: u32 = 983040u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_INTERFACE_C_MASK: u32 = 61440u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_INTERFACE_D_MASK: u32 = 3840u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_IS_PROPERTY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_METHOD_NUM_MASK: u32 = 127u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_ADR_MAX_EMAILS_SENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200130i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_ADR_NOT_DOMAIN_JOINED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200110i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_ADR_PATH_IS_LOCAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200111i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_ADR_SRV_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200112i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200253i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_AUTO_QUOTA: ::windows::core::HRESULT = ::windows::core::HRESULT(283419i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CACHE_INVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200187i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CACHE_MODULE_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200186i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CANNOT_AGGREGATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200201i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CANNOT_ALLOW_REPARSE_POINT_TAG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200170i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CANNOT_CHANGE_PROPERTY_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200197i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CANNOT_CREATE_TEMP_COPY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200132i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CANNOT_DELETE_SYSTEM_PROPERTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200135i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CANNOT_REMOVE_READONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200109i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CANNOT_RENAME_PROPERTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200198i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CANNOT_STORE_PROPERTIES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200171i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CANNOT_USE_DELETED_PROPERTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200143i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CANNOT_USE_DEPRECATED_PROPERTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200145i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CLASSIFICATION_ALREADY_RUNNING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200195i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CLASSIFICATION_CANCELED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200141i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CLASSIFICATION_NOT_RUNNING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200194i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CLASSIFICATION_PARTIAL_BATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200136i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CLASSIFICATION_SCAN_FAIL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200148i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CLASSIFICATION_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200137i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CLUSTER_NOT_RUNNING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200210i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CSC_PATH_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200106i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_DIFFERENT_CLUSTER_GROUP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200207i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_DRIVER_NOT_READY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200237i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_DUPLICATE_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200240i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_EMAIL_NOT_SENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200228i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_ENUM_PROPERTIES_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200173i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_ERROR_NOT_ENABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200133i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_EXPIRATION_PATH_NOT_WRITEABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200105i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_EXPIRATION_PATH_TOO_LONG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200104i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_EXPIRATION_VOLUME_NOT_NTFS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200103i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FAIL_BATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200247i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_ENCRYPTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200156i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200134i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_ACTION_GET_EXITCODE_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200152i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_ACTION_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200153i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_EXPIRATION_DIR_IN_SCOPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200185i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200184i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_ALREADY_RUNNING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200193i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_CUSTOM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200191i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_DEPRECATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200102i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_EXPIRATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200192i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_INVALID_CONTINUOUS_CONFIG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200108i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_MAX_FILE_CONDITIONS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200146i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_NOTIFICATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200190i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_NOT_LEGACY_ACCESSIBLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200147i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_RMS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200120i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_OPEN_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200189i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_SYSTEM_CORRUPT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200225i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INCOMPATIBLE_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200157i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INPROC_MODULE_BLOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200174i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INSECURE_PATH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200233i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INSUFFICIENT_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200236i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_AD_CLAIM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200142i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_COMBINATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200241i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_DATASCREEN_DEFINITION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200220i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_EMAIL_ADDRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200226i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_FILEGROUP_DEFINITION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200223i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_FILENAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200214i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_FOLDER_PROPERTY_STORE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200140i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_IMPORT_VERSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200245i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_LIMIT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200249i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200248i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_PATH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200250i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_REPORT_DESC: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200215i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_REPORT_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200216i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_SCHEDULER_ARGUMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200254i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_SMTP_SERVER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200232i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_TEXT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200246i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_USER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200251i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_LAST_ACCESS_UPDATE_DISABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200176i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_LEGACY_SCHEDULE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200107i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_LOADING_DISABLED_MODULE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200202i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_LONG_CMDLINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200224i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_MAX_PROPERTY_DEFINITIONS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200196i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_MESSAGE_LIMIT_EXCEEDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200200i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_MODULE_INITIALIZATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200150i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_MODULE_INVALID_PARAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200151i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_MODULE_SESSION_INITIALIZATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200149i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_MODULE_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200101i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_NOT_CLUSTER_VOLUME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200208i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200255i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200239i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_NO_EMAIL_ADDRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200131i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_NO_PROPERTY_VALUE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200175i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_OBJECT_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200199i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_OUT_OF_RANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200243i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_PARTIAL_CLASSIFICATION_PROPERTY_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200169i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_PATH_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200252i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_PATH_NOT_IN_NAMESPACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200129i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_PERSIST_PROPERTIES_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200155i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_PERSIST_PROPERTIES_FAILED_ENCRYPTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200166i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_PROPERTY_DELETED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200183i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_PROPERTY_MUST_APPLY_TO_FILES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200138i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_PROPERTY_MUST_APPLY_TO_FOLDERS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200124i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_PROPERTY_MUST_BE_GLOBAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200122i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_PROPERTY_MUST_BE_SECURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200123i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_REBUILDING_FODLER_TYPE_INDEX: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200139i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_REPORT_GENERATION_ERR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200204i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_REPORT_JOB_ALREADY_RUNNING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200205i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_REPORT_TASK_TRIGGER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200203i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_REPORT_TYPE_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200206i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_REQD_PARAM_MISSING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200242i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_RMS_NO_PROTECTORS_INSTALLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200126i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_RMS_NO_PROTECTOR_INSTALLED_FOR_FILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200125i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_RMS_TEMPLATE_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200128i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_SECURE_PROPERTIES_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200127i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_SET_PROPERTY_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200172i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_SHADOW_COPY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200212i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_STORE_NOT_INSTALLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200209i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_SYNC_TASK_HAD_ERRORS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200119i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_SYNC_TASK_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200144i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_TEXTREADER_FILENAME_TOO_LONG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200158i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_TEXTREADER_IFILTER_CLSID_MALFORMED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200160i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_TEXTREADER_IFILTER_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200167i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_TEXTREADER_NOT_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200168i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_TEXTREADER_STREAM_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200159i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_UNEXPECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200234i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_UNSECURE_LINK_TO_HOSTED_MODULE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200188i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_VOLUME_OFFLINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200154i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_VOLUME_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200235i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_WMI_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200121i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_XML_CORRUPTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200211i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_S_CLASSIFICATION_SCAN_FAILURES: ::windows::core::HRESULT = ::windows::core::HRESULT(283398i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_S_PARTIAL_BATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(283396i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_S_PARTIAL_CLASSIFICATION: ::windows::core::HRESULT = ::windows::core::HRESULT(283397i32);
pub const FsrmAccessDeniedRemediationClient: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x100b4fc8_74c1_470f_b1b7_dd7b6bae79bd);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmAccountType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmAccountType_Unknown: FsrmAccountType = FsrmAccountType(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmAccountType_NetworkService: FsrmAccountType = FsrmAccountType(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmAccountType_LocalService: FsrmAccountType = FsrmAccountType(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmAccountType_LocalSystem: FsrmAccountType = FsrmAccountType(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmAccountType_InProc: FsrmAccountType = FsrmAccountType(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmAccountType_External: FsrmAccountType = FsrmAccountType(5i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmAccountType_Automatic: FsrmAccountType = FsrmAccountType(500i32);
impl ::core::marker::Copy for FsrmAccountType {}
impl ::core::clone::Clone for FsrmAccountType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmAccountType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmAccountType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmAccountType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmAccountType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmActionType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmActionType_Unknown: FsrmActionType = FsrmActionType(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmActionType_EventLog: FsrmActionType = FsrmActionType(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmActionType_Email: FsrmActionType = FsrmActionType(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmActionType_Command: FsrmActionType = FsrmActionType(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmActionType_Report: FsrmActionType = FsrmActionType(4i32);
impl ::core::marker::Copy for FsrmActionType {}
impl ::core::clone::Clone for FsrmActionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmActionType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmActionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmActionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmActionType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmClassificationLoggingFlags(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmClassificationLoggingFlags_None: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmClassificationLoggingFlags_ClassificationsInLogFile: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmClassificationLoggingFlags_ErrorsInLogFile: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmClassificationLoggingFlags_ClassificationsInSystemLog: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmClassificationLoggingFlags_ErrorsInSystemLog: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(8i32);
impl ::core::marker::Copy for FsrmClassificationLoggingFlags {}
impl ::core::clone::Clone for FsrmClassificationLoggingFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmClassificationLoggingFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmClassificationLoggingFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmClassificationLoggingFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmClassificationLoggingFlags").field(&self.0).finish()
    }
}
pub const FsrmClassificationManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb15c0e47_c391_45b9_95c8_eb596c853f3a);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmCollectionState(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmCollectionState_Fetching: FsrmCollectionState = FsrmCollectionState(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmCollectionState_Committing: FsrmCollectionState = FsrmCollectionState(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmCollectionState_Complete: FsrmCollectionState = FsrmCollectionState(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmCollectionState_Cancelled: FsrmCollectionState = FsrmCollectionState(4i32);
impl ::core::marker::Copy for FsrmCollectionState {}
impl ::core::clone::Clone for FsrmCollectionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmCollectionState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmCollectionState {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmCollectionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmCollectionState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmCommitOptions(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmCommitOptions_None: FsrmCommitOptions = FsrmCommitOptions(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmCommitOptions_Asynchronous: FsrmCommitOptions = FsrmCommitOptions(1i32);
impl ::core::marker::Copy for FsrmCommitOptions {}
impl ::core::clone::Clone for FsrmCommitOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmCommitOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmCommitOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmCommitOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmCommitOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmDaysNotSpecified: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmEnumOptions(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmEnumOptions_None: FsrmEnumOptions = FsrmEnumOptions(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmEnumOptions_Asynchronous: FsrmEnumOptions = FsrmEnumOptions(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmEnumOptions_CheckRecycleBin: FsrmEnumOptions = FsrmEnumOptions(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmEnumOptions_IncludeClusterNodes: FsrmEnumOptions = FsrmEnumOptions(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmEnumOptions_IncludeDeprecatedObjects: FsrmEnumOptions = FsrmEnumOptions(8i32);
impl ::core::marker::Copy for FsrmEnumOptions {}
impl ::core::clone::Clone for FsrmEnumOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmEnumOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmEnumOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmEnumOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmEnumOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmEventType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmEventType_Unknown: FsrmEventType = FsrmEventType(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmEventType_Information: FsrmEventType = FsrmEventType(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmEventType_Warning: FsrmEventType = FsrmEventType(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmEventType_Error: FsrmEventType = FsrmEventType(3i32);
impl ::core::marker::Copy for FsrmEventType {}
impl ::core::clone::Clone for FsrmEventType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmEventType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmEventType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmEventType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmEventType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmExecutionOption(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmExecutionOption_Unknown: FsrmExecutionOption = FsrmExecutionOption(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmExecutionOption_EvaluateUnset: FsrmExecutionOption = FsrmExecutionOption(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmExecutionOption_ReEvaluate_ConsiderExistingValue: FsrmExecutionOption = FsrmExecutionOption(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmExecutionOption_ReEvaluate_IgnoreExistingValue: FsrmExecutionOption = FsrmExecutionOption(3i32);
impl ::core::marker::Copy for FsrmExecutionOption {}
impl ::core::clone::Clone for FsrmExecutionOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmExecutionOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmExecutionOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmExecutionOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmExecutionOption").field(&self.0).finish()
    }
}
pub const FsrmExportImport: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1482dc37_fae9_4787_9025_8ce4e024ab56);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmFileConditionType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileConditionType_Unknown: FsrmFileConditionType = FsrmFileConditionType(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileConditionType_Property: FsrmFileConditionType = FsrmFileConditionType(1i32);
impl ::core::marker::Copy for FsrmFileConditionType {}
impl ::core::clone::Clone for FsrmFileConditionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmFileConditionType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmFileConditionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmFileConditionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmFileConditionType").field(&self.0).finish()
    }
}
pub const FsrmFileGroupManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f1363f6_656f_4496_9226_13aecbd7718f);
pub const FsrmFileManagementJobManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb18f9b2_4c3a_4321_b203_205120cff614);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmFileManagementLoggingFlags(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileManagementLoggingFlags_None: FsrmFileManagementLoggingFlags = FsrmFileManagementLoggingFlags(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileManagementLoggingFlags_Error: FsrmFileManagementLoggingFlags = FsrmFileManagementLoggingFlags(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileManagementLoggingFlags_Information: FsrmFileManagementLoggingFlags = FsrmFileManagementLoggingFlags(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileManagementLoggingFlags_Audit: FsrmFileManagementLoggingFlags = FsrmFileManagementLoggingFlags(4i32);
impl ::core::marker::Copy for FsrmFileManagementLoggingFlags {}
impl ::core::clone::Clone for FsrmFileManagementLoggingFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmFileManagementLoggingFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmFileManagementLoggingFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmFileManagementLoggingFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmFileManagementLoggingFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmFileManagementType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileManagementType_Unknown: FsrmFileManagementType = FsrmFileManagementType(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileManagementType_Expiration: FsrmFileManagementType = FsrmFileManagementType(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileManagementType_Custom: FsrmFileManagementType = FsrmFileManagementType(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileManagementType_Rms: FsrmFileManagementType = FsrmFileManagementType(3i32);
impl ::core::marker::Copy for FsrmFileManagementType {}
impl ::core::clone::Clone for FsrmFileManagementType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmFileManagementType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmFileManagementType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmFileManagementType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmFileManagementType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmFileScreenFlags(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileScreenFlags_Enforce: FsrmFileScreenFlags = FsrmFileScreenFlags(1i32);
impl ::core::marker::Copy for FsrmFileScreenFlags {}
impl ::core::clone::Clone for FsrmFileScreenFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmFileScreenFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmFileScreenFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmFileScreenFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmFileScreenFlags").field(&self.0).finish()
    }
}
pub const FsrmFileScreenManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95941183_db53_4c5f_b37b_7d0921cf9dc7);
pub const FsrmFileScreenTemplateManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x243111df_e474_46aa_a054_eaa33edc292a);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmFileStreamingInterfaceType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileStreamingInterfaceType_Unknown: FsrmFileStreamingInterfaceType = FsrmFileStreamingInterfaceType(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileStreamingInterfaceType_ILockBytes: FsrmFileStreamingInterfaceType = FsrmFileStreamingInterfaceType(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileStreamingInterfaceType_IStream: FsrmFileStreamingInterfaceType = FsrmFileStreamingInterfaceType(2i32);
impl ::core::marker::Copy for FsrmFileStreamingInterfaceType {}
impl ::core::clone::Clone for FsrmFileStreamingInterfaceType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmFileStreamingInterfaceType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmFileStreamingInterfaceType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmFileStreamingInterfaceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmFileStreamingInterfaceType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmFileStreamingMode(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileStreamingMode_Unknown: FsrmFileStreamingMode = FsrmFileStreamingMode(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileStreamingMode_Read: FsrmFileStreamingMode = FsrmFileStreamingMode(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileStreamingMode_Write: FsrmFileStreamingMode = FsrmFileStreamingMode(2i32);
impl ::core::marker::Copy for FsrmFileStreamingMode {}
impl ::core::clone::Clone for FsrmFileStreamingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmFileStreamingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmFileStreamingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmFileStreamingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmFileStreamingMode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmFileSystemPropertyId(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileSystemPropertyId_Undefined: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileSystemPropertyId_FileName: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileSystemPropertyId_DateCreated: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileSystemPropertyId_DateLastAccessed: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileSystemPropertyId_DateLastModified: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileSystemPropertyId_DateNow: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(5i32);
impl ::core::marker::Copy for FsrmFileSystemPropertyId {}
impl ::core::clone::Clone for FsrmFileSystemPropertyId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmFileSystemPropertyId {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmFileSystemPropertyId {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmFileSystemPropertyId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmFileSystemPropertyId").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmGetFilePropertyOptions(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmGetFilePropertyOptions_None: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmGetFilePropertyOptions_NoRuleEvaluation: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmGetFilePropertyOptions_Persistent: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmGetFilePropertyOptions_FailOnPersistErrors: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmGetFilePropertyOptions_SkipOrphaned: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(8i32);
impl ::core::marker::Copy for FsrmGetFilePropertyOptions {}
impl ::core::clone::Clone for FsrmGetFilePropertyOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmGetFilePropertyOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmGetFilePropertyOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmGetFilePropertyOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmGetFilePropertyOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmMaxExcludeFolders: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmMaxNumberPropertyDefinitions: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmMaxNumberThresholds: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmMaxThresholdValue: u32 = 250u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmMinQuotaLimit: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmMinThresholdValue: u32 = 1u32;
pub const FsrmPathMapper: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3be42bd_8ac2_409e_bbd8_faf9b6b41feb);
pub const FsrmPipelineModuleConnector: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7643375_1eb5_44de_a062_623547d933bc);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmPipelineModuleType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPipelineModuleType_Unknown: FsrmPipelineModuleType = FsrmPipelineModuleType(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPipelineModuleType_Storage: FsrmPipelineModuleType = FsrmPipelineModuleType(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPipelineModuleType_Classifier: FsrmPipelineModuleType = FsrmPipelineModuleType(2i32);
impl ::core::marker::Copy for FsrmPipelineModuleType {}
impl ::core::clone::Clone for FsrmPipelineModuleType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmPipelineModuleType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmPipelineModuleType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmPipelineModuleType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmPipelineModuleType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmPropertyBagField(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyBagField_AccessVolume: FsrmPropertyBagField = FsrmPropertyBagField(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyBagField_VolumeGuidName: FsrmPropertyBagField = FsrmPropertyBagField(1i32);
impl ::core::marker::Copy for FsrmPropertyBagField {}
impl ::core::clone::Clone for FsrmPropertyBagField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmPropertyBagField {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmPropertyBagField {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmPropertyBagField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmPropertyBagField").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmPropertyBagFlags(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyBagFlags_UpdatedByClassifier: FsrmPropertyBagFlags = FsrmPropertyBagFlags(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyBagFlags_FailedLoadingProperties: FsrmPropertyBagFlags = FsrmPropertyBagFlags(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyBagFlags_FailedSavingProperties: FsrmPropertyBagFlags = FsrmPropertyBagFlags(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyBagFlags_FailedClassifyingProperties: FsrmPropertyBagFlags = FsrmPropertyBagFlags(8i32);
impl ::core::marker::Copy for FsrmPropertyBagFlags {}
impl ::core::clone::Clone for FsrmPropertyBagFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmPropertyBagFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmPropertyBagFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmPropertyBagFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmPropertyBagFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmPropertyConditionType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_Unknown: FsrmPropertyConditionType = FsrmPropertyConditionType(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_Equal: FsrmPropertyConditionType = FsrmPropertyConditionType(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_NotEqual: FsrmPropertyConditionType = FsrmPropertyConditionType(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_GreaterThan: FsrmPropertyConditionType = FsrmPropertyConditionType(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_LessThan: FsrmPropertyConditionType = FsrmPropertyConditionType(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_Contain: FsrmPropertyConditionType = FsrmPropertyConditionType(5i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_Exist: FsrmPropertyConditionType = FsrmPropertyConditionType(6i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_NotExist: FsrmPropertyConditionType = FsrmPropertyConditionType(7i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_StartWith: FsrmPropertyConditionType = FsrmPropertyConditionType(8i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_EndWith: FsrmPropertyConditionType = FsrmPropertyConditionType(9i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_ContainedIn: FsrmPropertyConditionType = FsrmPropertyConditionType(10i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_PrefixOf: FsrmPropertyConditionType = FsrmPropertyConditionType(11i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_SuffixOf: FsrmPropertyConditionType = FsrmPropertyConditionType(12i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_MatchesPattern: FsrmPropertyConditionType = FsrmPropertyConditionType(13i32);
impl ::core::marker::Copy for FsrmPropertyConditionType {}
impl ::core::clone::Clone for FsrmPropertyConditionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmPropertyConditionType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmPropertyConditionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmPropertyConditionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmPropertyConditionType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmPropertyDefinitionAppliesTo(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionAppliesTo_Files: FsrmPropertyDefinitionAppliesTo = FsrmPropertyDefinitionAppliesTo(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionAppliesTo_Folders: FsrmPropertyDefinitionAppliesTo = FsrmPropertyDefinitionAppliesTo(2i32);
impl ::core::marker::Copy for FsrmPropertyDefinitionAppliesTo {}
impl ::core::clone::Clone for FsrmPropertyDefinitionAppliesTo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmPropertyDefinitionAppliesTo {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmPropertyDefinitionAppliesTo {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmPropertyDefinitionAppliesTo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmPropertyDefinitionAppliesTo").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmPropertyDefinitionFlags(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionFlags_Global: FsrmPropertyDefinitionFlags = FsrmPropertyDefinitionFlags(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionFlags_Deprecated: FsrmPropertyDefinitionFlags = FsrmPropertyDefinitionFlags(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionFlags_Secure: FsrmPropertyDefinitionFlags = FsrmPropertyDefinitionFlags(4i32);
impl ::core::marker::Copy for FsrmPropertyDefinitionFlags {}
impl ::core::clone::Clone for FsrmPropertyDefinitionFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmPropertyDefinitionFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmPropertyDefinitionFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmPropertyDefinitionFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmPropertyDefinitionFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmPropertyDefinitionType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionType_Unknown: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionType_OrderedList: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionType_MultiChoiceList: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionType_SingleChoiceList: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionType_String: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionType_MultiString: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(5i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionType_Int: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(6i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionType_Bool: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(7i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionType_Date: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(8i32);
impl ::core::marker::Copy for FsrmPropertyDefinitionType {}
impl ::core::clone::Clone for FsrmPropertyDefinitionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmPropertyDefinitionType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmPropertyDefinitionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmPropertyDefinitionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmPropertyDefinitionType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmPropertyFlags(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_None: FsrmPropertyFlags = FsrmPropertyFlags(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_Orphaned: FsrmPropertyFlags = FsrmPropertyFlags(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_RetrievedFromCache: FsrmPropertyFlags = FsrmPropertyFlags(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_RetrievedFromStorage: FsrmPropertyFlags = FsrmPropertyFlags(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_SetByClassifier: FsrmPropertyFlags = FsrmPropertyFlags(8i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_Deleted: FsrmPropertyFlags = FsrmPropertyFlags(16i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_Reclassified: FsrmPropertyFlags = FsrmPropertyFlags(32i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_AggregationFailed: FsrmPropertyFlags = FsrmPropertyFlags(64i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_Existing: FsrmPropertyFlags = FsrmPropertyFlags(128i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_FailedLoadingProperties: FsrmPropertyFlags = FsrmPropertyFlags(256i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_FailedClassifyingProperties: FsrmPropertyFlags = FsrmPropertyFlags(512i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_FailedSavingProperties: FsrmPropertyFlags = FsrmPropertyFlags(1024i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_Secure: FsrmPropertyFlags = FsrmPropertyFlags(2048i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_PolicyDerived: FsrmPropertyFlags = FsrmPropertyFlags(4096i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_Inherited: FsrmPropertyFlags = FsrmPropertyFlags(8192i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_Manual: FsrmPropertyFlags = FsrmPropertyFlags(16384i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_ExplicitValueDeleted: FsrmPropertyFlags = FsrmPropertyFlags(32768i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_PropertyDeletedFromClear: FsrmPropertyFlags = FsrmPropertyFlags(65536i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_PropertySourceMask: FsrmPropertyFlags = FsrmPropertyFlags(14i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_PersistentMask: FsrmPropertyFlags = FsrmPropertyFlags(20480i32);
impl ::core::marker::Copy for FsrmPropertyFlags {}
impl ::core::clone::Clone for FsrmPropertyFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmPropertyFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmPropertyFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmPropertyFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmPropertyFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmPropertyValueType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyValueType_Undefined: FsrmPropertyValueType = FsrmPropertyValueType(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyValueType_Literal: FsrmPropertyValueType = FsrmPropertyValueType(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyValueType_DateOffset: FsrmPropertyValueType = FsrmPropertyValueType(2i32);
impl ::core::marker::Copy for FsrmPropertyValueType {}
impl ::core::clone::Clone for FsrmPropertyValueType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmPropertyValueType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmPropertyValueType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmPropertyValueType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmPropertyValueType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmQuotaFlags(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmQuotaFlags_Enforce: FsrmQuotaFlags = FsrmQuotaFlags(256i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmQuotaFlags_Disable: FsrmQuotaFlags = FsrmQuotaFlags(512i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmQuotaFlags_StatusIncomplete: FsrmQuotaFlags = FsrmQuotaFlags(65536i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmQuotaFlags_StatusRebuilding: FsrmQuotaFlags = FsrmQuotaFlags(131072i32);
impl ::core::marker::Copy for FsrmQuotaFlags {}
impl ::core::clone::Clone for FsrmQuotaFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmQuotaFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmQuotaFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmQuotaFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmQuotaFlags").field(&self.0).finish()
    }
}
pub const FsrmQuotaManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90dcab7f_347c_4bfc_b543_540326305fbe);
pub const FsrmQuotaTemplateManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97d3d443_251c_4337_81e7_b32e8f4ee65e);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmReportFilter(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFilter_MinSize: FsrmReportFilter = FsrmReportFilter(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFilter_MinAgeDays: FsrmReportFilter = FsrmReportFilter(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFilter_MaxAgeDays: FsrmReportFilter = FsrmReportFilter(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFilter_MinQuotaUsage: FsrmReportFilter = FsrmReportFilter(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFilter_FileGroups: FsrmReportFilter = FsrmReportFilter(5i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFilter_Owners: FsrmReportFilter = FsrmReportFilter(6i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFilter_NamePattern: FsrmReportFilter = FsrmReportFilter(7i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFilter_Property: FsrmReportFilter = FsrmReportFilter(8i32);
impl ::core::marker::Copy for FsrmReportFilter {}
impl ::core::clone::Clone for FsrmReportFilter {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmReportFilter {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmReportFilter {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmReportFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmReportFilter").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmReportFormat(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFormat_Unknown: FsrmReportFormat = FsrmReportFormat(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFormat_DHtml: FsrmReportFormat = FsrmReportFormat(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFormat_Html: FsrmReportFormat = FsrmReportFormat(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFormat_Txt: FsrmReportFormat = FsrmReportFormat(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFormat_Csv: FsrmReportFormat = FsrmReportFormat(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFormat_Xml: FsrmReportFormat = FsrmReportFormat(5i32);
impl ::core::marker::Copy for FsrmReportFormat {}
impl ::core::clone::Clone for FsrmReportFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmReportFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmReportFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmReportFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmReportFormat").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmReportGenerationContext(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportGenerationContext_Undefined: FsrmReportGenerationContext = FsrmReportGenerationContext(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportGenerationContext_ScheduledReport: FsrmReportGenerationContext = FsrmReportGenerationContext(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportGenerationContext_InteractiveReport: FsrmReportGenerationContext = FsrmReportGenerationContext(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportGenerationContext_IncidentReport: FsrmReportGenerationContext = FsrmReportGenerationContext(4i32);
impl ::core::marker::Copy for FsrmReportGenerationContext {}
impl ::core::clone::Clone for FsrmReportGenerationContext {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmReportGenerationContext {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmReportGenerationContext {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmReportGenerationContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmReportGenerationContext").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmReportLimit(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxFiles: FsrmReportLimit = FsrmReportLimit(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxFileGroups: FsrmReportLimit = FsrmReportLimit(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxOwners: FsrmReportLimit = FsrmReportLimit(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxFilesPerFileGroup: FsrmReportLimit = FsrmReportLimit(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxFilesPerOwner: FsrmReportLimit = FsrmReportLimit(5i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxFilesPerDuplGroup: FsrmReportLimit = FsrmReportLimit(6i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxDuplicateGroups: FsrmReportLimit = FsrmReportLimit(7i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxQuotas: FsrmReportLimit = FsrmReportLimit(8i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxFileScreenEvents: FsrmReportLimit = FsrmReportLimit(9i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxPropertyValues: FsrmReportLimit = FsrmReportLimit(10i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxFilesPerPropertyValue: FsrmReportLimit = FsrmReportLimit(11i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxFolders: FsrmReportLimit = FsrmReportLimit(12i32);
impl ::core::marker::Copy for FsrmReportLimit {}
impl ::core::clone::Clone for FsrmReportLimit {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmReportLimit {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmReportLimit {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmReportLimit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmReportLimit").field(&self.0).finish()
    }
}
pub const FsrmReportManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0058ef37_aa66_4c48_bd5b_2fce432ab0c8);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmReportRunningStatus(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportRunningStatus_Unknown: FsrmReportRunningStatus = FsrmReportRunningStatus(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportRunningStatus_NotRunning: FsrmReportRunningStatus = FsrmReportRunningStatus(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportRunningStatus_Queued: FsrmReportRunningStatus = FsrmReportRunningStatus(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportRunningStatus_Running: FsrmReportRunningStatus = FsrmReportRunningStatus(3i32);
impl ::core::marker::Copy for FsrmReportRunningStatus {}
impl ::core::clone::Clone for FsrmReportRunningStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmReportRunningStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmReportRunningStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmReportRunningStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmReportRunningStatus").field(&self.0).finish()
    }
}
pub const FsrmReportScheduler: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea25f1b8_1b8d_4290_8ee8_e17c12c2fe20);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmReportType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_Unknown: FsrmReportType = FsrmReportType(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_LargeFiles: FsrmReportType = FsrmReportType(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_FilesByType: FsrmReportType = FsrmReportType(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_LeastRecentlyAccessed: FsrmReportType = FsrmReportType(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_MostRecentlyAccessed: FsrmReportType = FsrmReportType(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_QuotaUsage: FsrmReportType = FsrmReportType(5i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_FilesByOwner: FsrmReportType = FsrmReportType(6i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_ExportReport: FsrmReportType = FsrmReportType(7i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_DuplicateFiles: FsrmReportType = FsrmReportType(8i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_FileScreenAudit: FsrmReportType = FsrmReportType(9i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_FilesByProperty: FsrmReportType = FsrmReportType(10i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_AutomaticClassification: FsrmReportType = FsrmReportType(11i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_Expiration: FsrmReportType = FsrmReportType(12i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_FoldersByProperty: FsrmReportType = FsrmReportType(13i32);
impl ::core::marker::Copy for FsrmReportType {}
impl ::core::clone::Clone for FsrmReportType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmReportType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmReportType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmReportType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmReportType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmRuleFlags(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmRuleFlags_Disabled: FsrmRuleFlags = FsrmRuleFlags(256i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmRuleFlags_ClearAutomaticallyClassifiedProperty: FsrmRuleFlags = FsrmRuleFlags(1024i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmRuleFlags_ClearManuallyClassifiedProperty: FsrmRuleFlags = FsrmRuleFlags(2048i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmRuleFlags_Invalid: FsrmRuleFlags = FsrmRuleFlags(4096i32);
impl ::core::marker::Copy for FsrmRuleFlags {}
impl ::core::clone::Clone for FsrmRuleFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmRuleFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmRuleFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmRuleFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmRuleFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmRuleType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmRuleType_Unknown: FsrmRuleType = FsrmRuleType(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmRuleType_Classification: FsrmRuleType = FsrmRuleType(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmRuleType_Generic: FsrmRuleType = FsrmRuleType(2i32);
impl ::core::marker::Copy for FsrmRuleType {}
impl ::core::clone::Clone for FsrmRuleType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmRuleType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmRuleType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmRuleType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmRuleType").field(&self.0).finish()
    }
}
pub const FsrmSetting: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf556d708_6d4d_4594_9c61_7dbb0dae2a46);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmStorageModuleCaps(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmStorageModuleCaps_Unknown: FsrmStorageModuleCaps = FsrmStorageModuleCaps(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmStorageModuleCaps_CanGet: FsrmStorageModuleCaps = FsrmStorageModuleCaps(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmStorageModuleCaps_CanSet: FsrmStorageModuleCaps = FsrmStorageModuleCaps(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmStorageModuleCaps_CanHandleDirectories: FsrmStorageModuleCaps = FsrmStorageModuleCaps(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmStorageModuleCaps_CanHandleFiles: FsrmStorageModuleCaps = FsrmStorageModuleCaps(8i32);
impl ::core::marker::Copy for FsrmStorageModuleCaps {}
impl ::core::clone::Clone for FsrmStorageModuleCaps {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmStorageModuleCaps {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmStorageModuleCaps {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmStorageModuleCaps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmStorageModuleCaps").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmStorageModuleType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmStorageModuleType_Unknown: FsrmStorageModuleType = FsrmStorageModuleType(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmStorageModuleType_Cache: FsrmStorageModuleType = FsrmStorageModuleType(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmStorageModuleType_InFile: FsrmStorageModuleType = FsrmStorageModuleType(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmStorageModuleType_Database: FsrmStorageModuleType = FsrmStorageModuleType(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmStorageModuleType_System: FsrmStorageModuleType = FsrmStorageModuleType(100i32);
impl ::core::marker::Copy for FsrmStorageModuleType {}
impl ::core::clone::Clone for FsrmStorageModuleType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmStorageModuleType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmStorageModuleType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmStorageModuleType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmStorageModuleType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FsrmTemplateApplyOptions(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmTemplateApplyOptions_ApplyToDerivedMatching: FsrmTemplateApplyOptions = FsrmTemplateApplyOptions(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmTemplateApplyOptions_ApplyToDerivedAll: FsrmTemplateApplyOptions = FsrmTemplateApplyOptions(2i32);
impl ::core::marker::Copy for FsrmTemplateApplyOptions {}
impl ::core::clone::Clone for FsrmTemplateApplyOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmTemplateApplyOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmTemplateApplyOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmTemplateApplyOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmTemplateApplyOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmAccessDeniedRemediationClient(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmAccessDeniedRemediationClient {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, parentwnd: usize, accesspath: Param1, errortype: AdrClientErrorType, flags: i32, windowtitle: Param4, windowmessage: Param5) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Show)(::core::mem::transmute_copy(self), ::core::mem::transmute(parentwnd), accesspath.into_param().abi(), ::core::mem::transmute(errortype), ::core::mem::transmute(flags), windowtitle.into_param().abi(), windowmessage.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmAccessDeniedRemediationClient> for ::windows::core::IUnknown {
    fn from(value: IFsrmAccessDeniedRemediationClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmAccessDeniedRemediationClient> for ::windows::core::IUnknown {
    fn from(value: &IFsrmAccessDeniedRemediationClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmAccessDeniedRemediationClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmAccessDeniedRemediationClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmAccessDeniedRemediationClient {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmAccessDeniedRemediationClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmAccessDeniedRemediationClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmAccessDeniedRemediationClient {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmAccessDeniedRemediationClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmAccessDeniedRemediationClient").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmAccessDeniedRemediationClient {
    type Vtable = IFsrmAccessDeniedRemediationClient_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40002314_590b_45a5_8e1b_8c05da527e52);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmAccessDeniedRemediationClient_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parentwnd: usize, accesspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, errortype: AdrClientErrorType, flags: i32, windowtitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, windowmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, result: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Show: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmAction(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmAction {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn ActionType(&self) -> ::windows::core::Result<FsrmActionType> {
        let mut result__: FsrmActionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ActionType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmActionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn RunLimitInterval(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RunLimitInterval)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRunLimitInterval)(::core::mem::transmute_copy(self), ::core::mem::transmute(minutes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmAction> for ::windows::core::IUnknown {
    fn from(value: IFsrmAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmAction> for ::windows::core::IUnknown {
    fn from(value: &IFsrmAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmAction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmAction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmAction {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmAction {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmAction").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmAction {
    type Vtable = IFsrmAction_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6cd6408a_ae60_463b_9ef1_e117534d69dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmAction_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub ActionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actiontype: *mut FsrmActionType) -> ::windows::core::HRESULT,
    pub RunLimitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows::core::HRESULT,
    pub SetRunLimitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmActionCommand(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmActionCommand {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn ActionType(&self) -> ::windows::core::Result<FsrmActionType> {
        let mut result__: FsrmActionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ActionType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmActionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn RunLimitInterval(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.RunLimitInterval)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetRunLimitInterval)(::core::mem::transmute_copy(self), ::core::mem::transmute(minutes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExecutablePath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ExecutablePath)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExecutablePath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, executablepath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetExecutablePath)(::core::mem::transmute_copy(self), executablepath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Arguments(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Arguments)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetArguments<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, arguments: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetArguments)(::core::mem::transmute_copy(self), arguments.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Account(&self) -> ::windows::core::Result<FsrmAccountType> {
        let mut result__: FsrmAccountType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Account)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmAccountType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetAccount(&self, account: FsrmAccountType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAccount)(::core::mem::transmute_copy(self), ::core::mem::transmute(account)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WorkingDirectory(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).WorkingDirectory)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWorkingDirectory<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, workingdirectory: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWorkingDirectory)(::core::mem::transmute_copy(self), workingdirectory.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn MonitorCommand(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MonitorCommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetMonitorCommand(&self, monitorcommand: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMonitorCommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(monitorcommand)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn KillTimeOut(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).KillTimeOut)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetKillTimeOut(&self, minutes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetKillTimeOut)(::core::mem::transmute_copy(self), ::core::mem::transmute(minutes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn LogResult(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LogResult)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetLogResult(&self, logresults: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLogResult)(::core::mem::transmute_copy(self), ::core::mem::transmute(logresults)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmActionCommand> for ::windows::core::IUnknown {
    fn from(value: IFsrmActionCommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmActionCommand> for ::windows::core::IUnknown {
    fn from(value: &IFsrmActionCommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmActionCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmActionCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmActionCommand {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmActionCommand> for IFsrmAction {
    fn from(value: IFsrmActionCommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmActionCommand> for IFsrmAction {
    fn from(value: &IFsrmActionCommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmAction> for IFsrmActionCommand {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmAction> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmAction> for &'a IFsrmActionCommand {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmAction> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmActionCommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmActionCommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmActionCommand {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmActionCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmActionCommand").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmActionCommand {
    type Vtable = IFsrmActionCommand_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12937789_e247_4917_9c20_f3ee9c7ee783);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionCommand_Vtbl {
    pub base: IFsrmAction_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ExecutablePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, executablepath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExecutablePath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetExecutablePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, executablepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetExecutablePath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Arguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, arguments: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Arguments: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, arguments: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetArguments: usize,
    pub Account: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, account: *mut FsrmAccountType) -> ::windows::core::HRESULT,
    pub SetAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, account: FsrmAccountType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WorkingDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, workingdirectory: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WorkingDirectory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetWorkingDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, workingdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetWorkingDirectory: usize,
    pub MonitorCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, monitorcommand: *mut i16) -> ::windows::core::HRESULT,
    pub SetMonitorCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, monitorcommand: i16) -> ::windows::core::HRESULT,
    pub KillTimeOut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows::core::HRESULT,
    pub SetKillTimeOut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows::core::HRESULT,
    pub LogResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logresults: *mut i16) -> ::windows::core::HRESULT,
    pub SetLogResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logresults: i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmActionEmail(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmActionEmail {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn ActionType(&self) -> ::windows::core::Result<FsrmActionType> {
        let mut result__: FsrmActionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ActionType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmActionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn RunLimitInterval(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.RunLimitInterval)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetRunLimitInterval)(::core::mem::transmute_copy(self), ::core::mem::transmute(minutes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailFrom(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MailFrom)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailFrom<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailfrom: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMailFrom)(::core::mem::transmute_copy(self), mailfrom.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailReplyTo(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MailReplyTo)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailReplyTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailreplyto: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMailReplyTo)(::core::mem::transmute_copy(self), mailreplyto.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailTo(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MailTo)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailto: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMailTo)(::core::mem::transmute_copy(self), mailto.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailCc(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MailCc)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailCc<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailcc: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMailCc)(::core::mem::transmute_copy(self), mailcc.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailBcc(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MailBcc)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailBcc<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailbcc: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMailBcc)(::core::mem::transmute_copy(self), mailbcc.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailSubject(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MailSubject)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailSubject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailsubject: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMailSubject)(::core::mem::transmute_copy(self), mailsubject.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MessageText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MessageText)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMessageText<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, messagetext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMessageText)(::core::mem::transmute_copy(self), messagetext.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmActionEmail> for ::windows::core::IUnknown {
    fn from(value: IFsrmActionEmail) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmActionEmail> for ::windows::core::IUnknown {
    fn from(value: &IFsrmActionEmail) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmActionEmail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmActionEmail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmActionEmail {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmActionEmail> for IFsrmAction {
    fn from(value: IFsrmActionEmail) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmActionEmail> for IFsrmAction {
    fn from(value: &IFsrmActionEmail) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmAction> for IFsrmActionEmail {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmAction> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmAction> for &'a IFsrmActionEmail {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmAction> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmActionEmail {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmActionEmail {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmActionEmail {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmActionEmail {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmActionEmail").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmActionEmail {
    type Vtable = IFsrmActionEmail_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd646567d_26ae_4caa_9f84_4e0aad207fca);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionEmail_Vtbl {
    pub base: IFsrmAction_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub MailFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailfrom: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MailFrom: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMailFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailfrom: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMailFrom: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MailReplyTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailreplyto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MailReplyTo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMailReplyTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailreplyto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMailReplyTo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MailTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MailTo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMailTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMailTo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MailCc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailcc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MailCc: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMailCc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailcc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMailCc: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MailBcc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailbcc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MailBcc: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMailBcc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailbcc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMailBcc: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MailSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailsubject: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MailSubject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMailSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailsubject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMailSubject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MessageText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MessageText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMessageText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMessageText: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmActionEmail2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmActionEmail2 {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn ActionType(&self) -> ::windows::core::Result<FsrmActionType> {
        let mut result__: FsrmActionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.ActionType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmActionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn RunLimitInterval(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.RunLimitInterval)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetRunLimitInterval)(::core::mem::transmute_copy(self), ::core::mem::transmute(minutes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailFrom(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.MailFrom)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailFrom<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailfrom: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetMailFrom)(::core::mem::transmute_copy(self), mailfrom.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailReplyTo(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.MailReplyTo)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailReplyTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailreplyto: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetMailReplyTo)(::core::mem::transmute_copy(self), mailreplyto.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailTo(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.MailTo)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailto: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetMailTo)(::core::mem::transmute_copy(self), mailto.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailCc(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.MailCc)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailCc<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailcc: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetMailCc)(::core::mem::transmute_copy(self), mailcc.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailBcc(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.MailBcc)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailBcc<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailbcc: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetMailBcc)(::core::mem::transmute_copy(self), mailbcc.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailSubject(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.MailSubject)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailSubject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailsubject: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetMailSubject)(::core::mem::transmute_copy(self), mailsubject.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MessageText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.MessageText)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMessageText<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, messagetext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetMessageText)(::core::mem::transmute_copy(self), messagetext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn AttachmentFileListSize(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AttachmentFileListSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetAttachmentFileListSize(&self, attachmentfilelistsize: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAttachmentFileListSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(attachmentfilelistsize)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmActionEmail2> for ::windows::core::IUnknown {
    fn from(value: IFsrmActionEmail2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmActionEmail2> for ::windows::core::IUnknown {
    fn from(value: &IFsrmActionEmail2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmActionEmail2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmActionEmail2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmActionEmail2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmActionEmail2> for IFsrmAction {
    fn from(value: IFsrmActionEmail2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmActionEmail2> for IFsrmAction {
    fn from(value: &IFsrmActionEmail2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmAction> for IFsrmActionEmail2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmAction> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmAction> for &'a IFsrmActionEmail2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmAction> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmActionEmail2> for IFsrmActionEmail {
    fn from(value: IFsrmActionEmail2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmActionEmail2> for IFsrmActionEmail {
    fn from(value: &IFsrmActionEmail2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmActionEmail> for IFsrmActionEmail2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmActionEmail> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmActionEmail> for &'a IFsrmActionEmail2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmActionEmail> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmActionEmail2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmActionEmail2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmActionEmail2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmActionEmail2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmActionEmail2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmActionEmail2 {
    type Vtable = IFsrmActionEmail2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8276702f_2532_4839_89bf_4872609a2ea4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionEmail2_Vtbl {
    pub base: IFsrmActionEmail_Vtbl,
    pub AttachmentFileListSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attachmentfilelistsize: *mut i32) -> ::windows::core::HRESULT,
    pub SetAttachmentFileListSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attachmentfilelistsize: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmActionEventLog(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmActionEventLog {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn ActionType(&self) -> ::windows::core::Result<FsrmActionType> {
        let mut result__: FsrmActionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ActionType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmActionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn RunLimitInterval(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.RunLimitInterval)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetRunLimitInterval)(::core::mem::transmute_copy(self), ::core::mem::transmute(minutes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn EventType(&self) -> ::windows::core::Result<FsrmEventType> {
        let mut result__: FsrmEventType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EventType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmEventType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetEventType(&self, eventtype: FsrmEventType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEventType)(::core::mem::transmute_copy(self), ::core::mem::transmute(eventtype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MessageText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MessageText)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMessageText<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, messagetext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMessageText)(::core::mem::transmute_copy(self), messagetext.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmActionEventLog> for ::windows::core::IUnknown {
    fn from(value: IFsrmActionEventLog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmActionEventLog> for ::windows::core::IUnknown {
    fn from(value: &IFsrmActionEventLog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmActionEventLog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmActionEventLog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmActionEventLog {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmActionEventLog> for IFsrmAction {
    fn from(value: IFsrmActionEventLog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmActionEventLog> for IFsrmAction {
    fn from(value: &IFsrmActionEventLog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmAction> for IFsrmActionEventLog {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmAction> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmAction> for &'a IFsrmActionEventLog {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmAction> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmActionEventLog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmActionEventLog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmActionEventLog {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmActionEventLog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmActionEventLog").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmActionEventLog {
    type Vtable = IFsrmActionEventLog_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c8f96c3_5d94_4f37_a4f4_f56ab463546f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionEventLog_Vtbl {
    pub base: IFsrmAction_Vtbl,
    pub EventType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventtype: *mut FsrmEventType) -> ::windows::core::HRESULT,
    pub SetEventType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventtype: FsrmEventType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub MessageText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MessageText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMessageText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMessageText: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmActionReport(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmActionReport {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn ActionType(&self) -> ::windows::core::Result<FsrmActionType> {
        let mut result__: FsrmActionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ActionType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmActionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn RunLimitInterval(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.RunLimitInterval)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetRunLimitInterval)(::core::mem::transmute_copy(self), ::core::mem::transmute(minutes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReportTypes(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ReportTypes)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetReportTypes(&self, reporttypes: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetReportTypes)(::core::mem::transmute_copy(self), ::core::mem::transmute(reporttypes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailTo(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MailTo)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailto: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMailTo)(::core::mem::transmute_copy(self), mailto.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmActionReport> for ::windows::core::IUnknown {
    fn from(value: IFsrmActionReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmActionReport> for ::windows::core::IUnknown {
    fn from(value: &IFsrmActionReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmActionReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmActionReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmActionReport {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmActionReport> for IFsrmAction {
    fn from(value: IFsrmActionReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmActionReport> for IFsrmAction {
    fn from(value: &IFsrmActionReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmAction> for IFsrmActionReport {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmAction> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmAction> for &'a IFsrmActionReport {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmAction> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmActionReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmActionReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmActionReport {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmActionReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmActionReport").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmActionReport {
    type Vtable = IFsrmActionReport_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2dbe63c4_b340_48a0_a5b0_158e07fc567e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionReport_Vtbl {
    pub base: IFsrmAction_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ReportTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttypes: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReportTypes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetReportTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttypes: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetReportTypes: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MailTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MailTo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMailTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMailTo: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmAutoApplyQuota(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmAutoApplyQuota {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetDescription)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.Commit)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn QuotaLimit(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.QuotaLimit)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetQuotaLimit<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, quotalimit: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetQuotaLimit)(::core::mem::transmute_copy(self), quotalimit.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn QuotaFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.QuotaFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetQuotaFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(quotaflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Thresholds(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.Thresholds)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn AddThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.AddThreshold)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.DeleteThreshold)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.ModifyThreshold)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(newthreshold)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.CreateThresholdAction)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(actiontype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmAction>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.EnumThresholdActions)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Path)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserSid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.UserSid)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserAccount(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.UserAccount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SourceTemplateName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SourceTemplateName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn MatchesSourceTemplate(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.MatchesSourceTemplate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplyTemplate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, quotatemplatename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.ApplyTemplate)(::core::mem::transmute_copy(self), quotatemplatename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExcludeFolders(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ExcludeFolders)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetExcludeFolders(&self, folders: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetExcludeFolders)(::core::mem::transmute_copy(self), ::core::mem::transmute(folders)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows::core::Result<IFsrmDerivedObjectsResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CommitAndUpdateDerived)(::core::mem::transmute_copy(self), ::core::mem::transmute(commitoptions), ::core::mem::transmute(applyoptions), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmDerivedObjectsResult>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmAutoApplyQuota> for ::windows::core::IUnknown {
    fn from(value: IFsrmAutoApplyQuota) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmAutoApplyQuota> for ::windows::core::IUnknown {
    fn from(value: &IFsrmAutoApplyQuota) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmAutoApplyQuota> for IFsrmObject {
    fn from(value: IFsrmAutoApplyQuota) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmAutoApplyQuota> for IFsrmObject {
    fn from(value: &IFsrmAutoApplyQuota) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &'a IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmAutoApplyQuota> for IFsrmQuotaBase {
    fn from(value: IFsrmAutoApplyQuota) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmAutoApplyQuota> for IFsrmQuotaBase {
    fn from(value: &IFsrmAutoApplyQuota) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaBase> for IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaBase> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaBase> for &'a IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmAutoApplyQuota> for IFsrmQuotaObject {
    fn from(value: IFsrmAutoApplyQuota) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmAutoApplyQuota> for IFsrmQuotaObject {
    fn from(value: &IFsrmAutoApplyQuota) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaObject> for IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaObject> for &'a IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmAutoApplyQuota {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmAutoApplyQuota {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmAutoApplyQuota {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmAutoApplyQuota {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmAutoApplyQuota").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmAutoApplyQuota {
    type Vtable = IFsrmAutoApplyQuota_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf82e5729_6aba_4740_bfc7_c7f58f75fb7b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmAutoApplyQuota_Vtbl {
    pub base: IFsrmQuotaObject_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ExcludeFolders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExcludeFolders: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetExcludeFolders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folders: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetExcludeFolders: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CommitAndUpdateDerived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommitAndUpdateDerived: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmClassificationManager(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmClassificationManager {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ClassificationReportFormats(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ClassificationReportFormats)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetClassificationReportFormats(&self, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetClassificationReportFormats)(::core::mem::transmute_copy(self), ::core::mem::transmute(formats)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Logging(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Logging)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetLogging(&self, logging: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLogging)(::core::mem::transmute_copy(self), ::core::mem::transmute(logging)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClassificationReportMailTo(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ClassificationReportMailTo)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClassificationReportMailTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailto: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetClassificationReportMailTo)(::core::mem::transmute_copy(self), mailto.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn ClassificationReportEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ClassificationReportEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetClassificationReportEnabled(&self, reportenabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetClassificationReportEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(reportenabled)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClassificationLastReportPathWithoutExtension(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ClassificationLastReportPathWithoutExtension)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClassificationLastError(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ClassificationLastError)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn ClassificationRunningStatus(&self) -> ::windows::core::Result<FsrmReportRunningStatus> {
        let mut result__: FsrmReportRunningStatus = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ClassificationRunningStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmReportRunningStatus>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumPropertyDefinitions(&self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumPropertyDefinitions)(::core::mem::transmute_copy(self), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePropertyDefinition(&self) -> ::windows::core::Result<IFsrmPropertyDefinition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreatePropertyDefinition)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmPropertyDefinition>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetPropertyDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, propertyname: Param0) -> ::windows::core::Result<IFsrmPropertyDefinition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPropertyDefinition)(::core::mem::transmute_copy(self), propertyname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmPropertyDefinition>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumRules(&self, ruletype: FsrmRuleType, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumRules)(::core::mem::transmute_copy(self), ::core::mem::transmute(ruletype), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateRule(&self, ruletype: FsrmRuleType) -> ::windows::core::Result<IFsrmRule> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateRule)(::core::mem::transmute_copy(self), ::core::mem::transmute(ruletype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmRule>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetRule<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, rulename: Param0, ruletype: FsrmRuleType) -> ::windows::core::Result<IFsrmRule> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRule)(::core::mem::transmute_copy(self), rulename.into_param().abi(), ::core::mem::transmute(ruletype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmRule>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumModuleDefinitions(&self, moduletype: FsrmPipelineModuleType, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumModuleDefinitions)(::core::mem::transmute_copy(self), ::core::mem::transmute(moduletype), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateModuleDefinition(&self, moduletype: FsrmPipelineModuleType) -> ::windows::core::Result<IFsrmPipelineModuleDefinition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateModuleDefinition)(::core::mem::transmute_copy(self), ::core::mem::transmute(moduletype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmPipelineModuleDefinition>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetModuleDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, modulename: Param0, moduletype: FsrmPipelineModuleType) -> ::windows::core::Result<IFsrmPipelineModuleDefinition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetModuleDefinition)(::core::mem::transmute_copy(self), modulename.into_param().abi(), ::core::mem::transmute(moduletype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmPipelineModuleDefinition>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunClassification<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, context: FsrmReportGenerationContext, reserved: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RunClassification)(::core::mem::transmute_copy(self), ::core::mem::transmute(context), reserved.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn WaitForClassificationCompletion(&self, waitseconds: i32) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).WaitForClassificationCompletion)(::core::mem::transmute_copy(self), ::core::mem::transmute(waitseconds), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn CancelClassification(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CancelClassification)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn EnumFileProperties<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, options: FsrmGetFilePropertyOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumFileProperties)(::core::mem::transmute_copy(self), filepath.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetFileProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, propertyname: Param1, options: FsrmGetFilePropertyOptions) -> ::windows::core::Result<IFsrmProperty> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFileProperty)(::core::mem::transmute_copy(self), filepath.into_param().abi(), propertyname.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, propertyname: Param1, propertyvalue: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFileProperty)(::core::mem::transmute_copy(self), filepath.into_param().abi(), propertyname.into_param().abi(), propertyvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearFileProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, property: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClearFileProperty)(::core::mem::transmute_copy(self), filepath.into_param().abi(), property.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmClassificationManager> for ::windows::core::IUnknown {
    fn from(value: IFsrmClassificationManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmClassificationManager> for ::windows::core::IUnknown {
    fn from(value: &IFsrmClassificationManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmClassificationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmClassificationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmClassificationManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmClassificationManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmClassificationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmClassificationManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmClassificationManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmClassificationManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmClassificationManager {
    type Vtable = IFsrmClassificationManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2dc89da_ee91_48a0_85d8_cc72a56f7d04);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassificationManager_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ClassificationReportFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ClassificationReportFormats: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetClassificationReportFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetClassificationReportFormats: usize,
    pub Logging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logging: *mut i32) -> ::windows::core::HRESULT,
    pub SetLogging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logging: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ClassificationReportMailTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClassificationReportMailTo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClassificationReportMailTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClassificationReportMailTo: usize,
    pub ClassificationReportEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportenabled: *mut i16) -> ::windows::core::HRESULT,
    pub SetClassificationReportEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportenabled: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ClassificationLastReportPathWithoutExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastreportpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClassificationLastReportPathWithoutExtension: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ClassificationLastError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lasterror: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClassificationLastError: usize,
    pub ClassificationRunningStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runningstatus: *mut FsrmReportRunningStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumPropertyDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, propertydefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumPropertyDefinitions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePropertyDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertydefinition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePropertyDefinition: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetPropertyDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertydefinition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetPropertyDefinition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumRules: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ruletype: FsrmRuleType, options: FsrmEnumOptions, rules: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumRules: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ruletype: FsrmRuleType, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateRule: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rulename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ruletype: FsrmRuleType, rule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetRule: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumModuleDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduletype: FsrmPipelineModuleType, options: FsrmEnumOptions, moduledefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumModuleDefinitions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateModuleDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduletype: FsrmPipelineModuleType, moduledefinition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateModuleDefinition: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetModuleDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modulename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, moduletype: FsrmPipelineModuleType, moduledefinition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetModuleDefinition: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RunClassification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext, reserved: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RunClassification: usize,
    pub WaitForClassificationCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut i16) -> ::windows::core::HRESULT,
    pub CancelClassification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub EnumFileProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmGetFilePropertyOptions, fileproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    EnumFileProperties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetFileProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmGetFilePropertyOptions, property: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetFileProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFileProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFileProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ClearFileProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, property: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClearFileProperty: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmClassificationManager2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmClassificationManager2 {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ClassificationReportFormats(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ClassificationReportFormats)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetClassificationReportFormats(&self, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetClassificationReportFormats)(::core::mem::transmute_copy(self), ::core::mem::transmute(formats)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Logging(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Logging)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetLogging(&self, logging: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLogging)(::core::mem::transmute_copy(self), ::core::mem::transmute(logging)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClassificationReportMailTo(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ClassificationReportMailTo)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClassificationReportMailTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailto: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetClassificationReportMailTo)(::core::mem::transmute_copy(self), mailto.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn ClassificationReportEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ClassificationReportEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetClassificationReportEnabled(&self, reportenabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetClassificationReportEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(reportenabled)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClassificationLastReportPathWithoutExtension(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ClassificationLastReportPathWithoutExtension)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClassificationLastError(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ClassificationLastError)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn ClassificationRunningStatus(&self) -> ::windows::core::Result<FsrmReportRunningStatus> {
        let mut result__: FsrmReportRunningStatus = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ClassificationRunningStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmReportRunningStatus>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumPropertyDefinitions(&self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.EnumPropertyDefinitions)(::core::mem::transmute_copy(self), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePropertyDefinition(&self) -> ::windows::core::Result<IFsrmPropertyDefinition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreatePropertyDefinition)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmPropertyDefinition>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetPropertyDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, propertyname: Param0) -> ::windows::core::Result<IFsrmPropertyDefinition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetPropertyDefinition)(::core::mem::transmute_copy(self), propertyname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmPropertyDefinition>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumRules(&self, ruletype: FsrmRuleType, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.EnumRules)(::core::mem::transmute_copy(self), ::core::mem::transmute(ruletype), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateRule(&self, ruletype: FsrmRuleType) -> ::windows::core::Result<IFsrmRule> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateRule)(::core::mem::transmute_copy(self), ::core::mem::transmute(ruletype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmRule>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetRule<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, rulename: Param0, ruletype: FsrmRuleType) -> ::windows::core::Result<IFsrmRule> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetRule)(::core::mem::transmute_copy(self), rulename.into_param().abi(), ::core::mem::transmute(ruletype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmRule>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumModuleDefinitions(&self, moduletype: FsrmPipelineModuleType, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.EnumModuleDefinitions)(::core::mem::transmute_copy(self), ::core::mem::transmute(moduletype), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateModuleDefinition(&self, moduletype: FsrmPipelineModuleType) -> ::windows::core::Result<IFsrmPipelineModuleDefinition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateModuleDefinition)(::core::mem::transmute_copy(self), ::core::mem::transmute(moduletype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmPipelineModuleDefinition>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetModuleDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, modulename: Param0, moduletype: FsrmPipelineModuleType) -> ::windows::core::Result<IFsrmPipelineModuleDefinition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetModuleDefinition)(::core::mem::transmute_copy(self), modulename.into_param().abi(), ::core::mem::transmute(moduletype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmPipelineModuleDefinition>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunClassification<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, context: FsrmReportGenerationContext, reserved: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.RunClassification)(::core::mem::transmute_copy(self), ::core::mem::transmute(context), reserved.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn WaitForClassificationCompletion(&self, waitseconds: i32) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.WaitForClassificationCompletion)(::core::mem::transmute_copy(self), ::core::mem::transmute(waitseconds), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn CancelClassification(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CancelClassification)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn EnumFileProperties<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, options: FsrmGetFilePropertyOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.EnumFileProperties)(::core::mem::transmute_copy(self), filepath.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetFileProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, propertyname: Param1, options: FsrmGetFilePropertyOptions) -> ::windows::core::Result<IFsrmProperty> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetFileProperty)(::core::mem::transmute_copy(self), filepath.into_param().abi(), propertyname.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, propertyname: Param1, propertyvalue: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetFileProperty)(::core::mem::transmute_copy(self), filepath.into_param().abi(), propertyname.into_param().abi(), propertyvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearFileProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, property: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.ClearFileProperty)(::core::mem::transmute_copy(self), filepath.into_param().abi(), property.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ClassifyFiles(&self, filepaths: *const super::super::System::Com::SAFEARRAY, propertynames: *const super::super::System::Com::SAFEARRAY, propertyvalues: *const super::super::System::Com::SAFEARRAY, options: FsrmGetFilePropertyOptions) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClassifyFiles)(::core::mem::transmute_copy(self), ::core::mem::transmute(filepaths), ::core::mem::transmute(propertynames), ::core::mem::transmute(propertyvalues), ::core::mem::transmute(options)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmClassificationManager2> for ::windows::core::IUnknown {
    fn from(value: IFsrmClassificationManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmClassificationManager2> for ::windows::core::IUnknown {
    fn from(value: &IFsrmClassificationManager2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmClassificationManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmClassificationManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmClassificationManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmClassificationManager2> for IFsrmClassificationManager {
    fn from(value: IFsrmClassificationManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmClassificationManager2> for IFsrmClassificationManager {
    fn from(value: &IFsrmClassificationManager2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmClassificationManager> for IFsrmClassificationManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmClassificationManager> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmClassificationManager> for &'a IFsrmClassificationManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmClassificationManager> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmClassificationManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmClassificationManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmClassificationManager2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmClassificationManager2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmClassificationManager2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmClassificationManager2 {
    type Vtable = IFsrmClassificationManager2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0004c1c9_127e_4765_ba07_6a3147bca112);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassificationManager2_Vtbl {
    pub base: IFsrmClassificationManager_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ClassifyFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepaths: *const super::super::System::Com::SAFEARRAY, propertynames: *const super::super::System::Com::SAFEARRAY, propertyvalues: *const super::super::System::Com::SAFEARRAY, options: FsrmGetFilePropertyOptions) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ClassifyFiles: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmClassificationRule(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmClassificationRule {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetDescription)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Commit)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn RuleType(&self) -> ::windows::core::Result<FsrmRuleType> {
        let mut result__: FsrmRuleType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.RuleType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmRuleType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModuleDefinitionName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ModuleDefinitionName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetModuleDefinitionName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, moduledefinitionname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetModuleDefinitionName)(::core::mem::transmute_copy(self), moduledefinitionname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NamespaceRoots(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.NamespaceRoots)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetNamespaceRoots(&self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetNamespaceRoots)(::core::mem::transmute_copy(self), ::core::mem::transmute(namespaceroots)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn RuleFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.RuleFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetRuleFlags(&self, ruleflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetRuleFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(ruleflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Parameters)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetParameters)(::core::mem::transmute_copy(self), ::core::mem::transmute(parameters)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LastModified(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LastModified)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn ExecutionOption(&self) -> ::windows::core::Result<FsrmExecutionOption> {
        let mut result__: FsrmExecutionOption = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ExecutionOption)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmExecutionOption>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetExecutionOption(&self, executionoption: FsrmExecutionOption) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetExecutionOption)(::core::mem::transmute_copy(self), ::core::mem::transmute(executionoption)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PropertyAffected(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PropertyAffected)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPropertyAffected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, property: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPropertyAffected)(::core::mem::transmute_copy(self), property.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Value)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, value: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetValue)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmClassificationRule> for ::windows::core::IUnknown {
    fn from(value: IFsrmClassificationRule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmClassificationRule> for ::windows::core::IUnknown {
    fn from(value: &IFsrmClassificationRule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmClassificationRule {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmClassificationRule {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmClassificationRule {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmClassificationRule> for IFsrmObject {
    fn from(value: IFsrmClassificationRule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmClassificationRule> for IFsrmObject {
    fn from(value: &IFsrmClassificationRule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmClassificationRule {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &'a IFsrmClassificationRule {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmClassificationRule> for IFsrmRule {
    fn from(value: IFsrmClassificationRule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmClassificationRule> for IFsrmRule {
    fn from(value: &IFsrmClassificationRule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmRule> for IFsrmClassificationRule {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmRule> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmRule> for &'a IFsrmClassificationRule {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmRule> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmClassificationRule {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmClassificationRule {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmClassificationRule {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmClassificationRule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmClassificationRule").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmClassificationRule {
    type Vtable = IFsrmClassificationRule_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xafc052c2_5315_45ab_841b_c6db0e120148);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassificationRule_Vtbl {
    pub base: IFsrmRule_Vtbl,
    pub ExecutionOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, executionoption: *mut FsrmExecutionOption) -> ::windows::core::HRESULT,
    pub SetExecutionOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, executionoption: FsrmExecutionOption) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PropertyAffected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PropertyAffected: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPropertyAffected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPropertyAffected: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Value: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetValue: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmClassifierModuleDefinition(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmClassifierModuleDefinition {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetDescription)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Commit)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModuleClsid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ModuleClsid)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetModuleClsid<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, moduleclsid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetModuleClsid)(::core::mem::transmute_copy(self), moduleclsid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Company(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Company)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCompany<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, company: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetCompany)(::core::mem::transmute_copy(self), company.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Version(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Version)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVersion<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, version: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetVersion)(::core::mem::transmute_copy(self), version.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn ModuleType(&self) -> ::windows::core::Result<FsrmPipelineModuleType> {
        let mut result__: FsrmPipelineModuleType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ModuleType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmPipelineModuleType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Enabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn NeedsFileContent(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.NeedsFileContent)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetNeedsFileContent(&self, needsfilecontent: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetNeedsFileContent)(::core::mem::transmute_copy(self), ::core::mem::transmute(needsfilecontent)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Account(&self) -> ::windows::core::Result<FsrmAccountType> {
        let mut result__: FsrmAccountType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Account)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmAccountType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetAccount(&self, retrievalaccount: FsrmAccountType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetAccount)(::core::mem::transmute_copy(self), ::core::mem::transmute(retrievalaccount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedExtensions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SupportedExtensions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSupportedExtensions(&self, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetSupportedExtensions)(::core::mem::transmute_copy(self), ::core::mem::transmute(supportedextensions)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Parameters)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetParameters)(::core::mem::transmute_copy(self), ::core::mem::transmute(parameters)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PropertiesAffected(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PropertiesAffected)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPropertiesAffected(&self, propertiesaffected: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPropertiesAffected)(::core::mem::transmute_copy(self), ::core::mem::transmute(propertiesaffected)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PropertiesUsed(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PropertiesUsed)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPropertiesUsed(&self, propertiesused: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPropertiesUsed)(::core::mem::transmute_copy(self), ::core::mem::transmute(propertiesused)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn NeedsExplicitValue(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).NeedsExplicitValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetNeedsExplicitValue(&self, needsexplicitvalue: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNeedsExplicitValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(needsexplicitvalue)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmClassifierModuleDefinition> for ::windows::core::IUnknown {
    fn from(value: IFsrmClassifierModuleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmClassifierModuleDefinition> for ::windows::core::IUnknown {
    fn from(value: &IFsrmClassifierModuleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmClassifierModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmClassifierModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmClassifierModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmClassifierModuleDefinition> for IFsrmObject {
    fn from(value: IFsrmClassifierModuleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmClassifierModuleDefinition> for IFsrmObject {
    fn from(value: &IFsrmClassifierModuleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmClassifierModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &'a IFsrmClassifierModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmClassifierModuleDefinition> for IFsrmPipelineModuleDefinition {
    fn from(value: IFsrmClassifierModuleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmClassifierModuleDefinition> for IFsrmPipelineModuleDefinition {
    fn from(value: &IFsrmClassifierModuleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmPipelineModuleDefinition> for IFsrmClassifierModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmPipelineModuleDefinition> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmPipelineModuleDefinition> for &'a IFsrmClassifierModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmPipelineModuleDefinition> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmClassifierModuleDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmClassifierModuleDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmClassifierModuleDefinition {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmClassifierModuleDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmClassifierModuleDefinition").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmClassifierModuleDefinition {
    type Vtable = IFsrmClassifierModuleDefinition_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb36ea26_6318_4b8c_8592_f72dd602e7a5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassifierModuleDefinition_Vtbl {
    pub base: IFsrmPipelineModuleDefinition_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub PropertiesAffected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertiesaffected: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PropertiesAffected: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetPropertiesAffected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertiesaffected: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetPropertiesAffected: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PropertiesUsed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertiesused: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PropertiesUsed: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetPropertiesUsed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertiesused: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetPropertiesUsed: usize,
    pub NeedsExplicitValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, needsexplicitvalue: *mut i16) -> ::windows::core::HRESULT,
    pub SetNeedsExplicitValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, needsexplicitvalue: i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmClassifierModuleImplementation(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmClassifierModuleImplementation {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnLoad<'a, Param0: ::windows::core::IntoParam<'a, IFsrmPipelineModuleDefinition>>(&self, moduledefinition: Param0) -> ::windows::core::Result<IFsrmPipelineModuleConnector> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.OnLoad)(::core::mem::transmute_copy(self), moduledefinition.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmPipelineModuleConnector>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn OnUnload(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OnUnload)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LastModified(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LastModified)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UseRulesAndDefinitions<'a, Param0: ::windows::core::IntoParam<'a, IFsrmCollection>, Param1: ::windows::core::IntoParam<'a, IFsrmCollection>>(&self, rules: Param0, propertydefinitions: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UseRulesAndDefinitions)(::core::mem::transmute_copy(self), rules.into_param().abi(), propertydefinitions.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnBeginFile<'a, Param0: ::windows::core::IntoParam<'a, IFsrmPropertyBag>>(&self, propertybag: Param0, arrayruleids: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnBeginFile)(::core::mem::transmute_copy(self), propertybag.into_param().abi(), ::core::mem::transmute(arrayruleids)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesPropertyValueApply<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param4: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, property: Param0, value: Param1, applyvalue: *mut i16, idrule: Param3, idpropdef: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DoesPropertyValueApply)(::core::mem::transmute_copy(self), property.into_param().abi(), value.into_param().abi(), ::core::mem::transmute(applyvalue), idrule.into_param().abi(), idpropdef.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyValueToApply<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param3: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, property: Param0, value: *mut super::super::Foundation::BSTR, idrule: Param2, idpropdef: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPropertyValueToApply)(::core::mem::transmute_copy(self), property.into_param().abi(), ::core::mem::transmute(value), idrule.into_param().abi(), idpropdef.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn OnEndFile(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnEndFile)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmClassifierModuleImplementation> for ::windows::core::IUnknown {
    fn from(value: IFsrmClassifierModuleImplementation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmClassifierModuleImplementation> for ::windows::core::IUnknown {
    fn from(value: &IFsrmClassifierModuleImplementation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmClassifierModuleImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmClassifierModuleImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmClassifierModuleImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmClassifierModuleImplementation> for IFsrmPipelineModuleImplementation {
    fn from(value: IFsrmClassifierModuleImplementation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmClassifierModuleImplementation> for IFsrmPipelineModuleImplementation {
    fn from(value: &IFsrmClassifierModuleImplementation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmPipelineModuleImplementation> for IFsrmClassifierModuleImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmPipelineModuleImplementation> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmPipelineModuleImplementation> for &'a IFsrmClassifierModuleImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmPipelineModuleImplementation> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmClassifierModuleImplementation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmClassifierModuleImplementation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmClassifierModuleImplementation {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmClassifierModuleImplementation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmClassifierModuleImplementation").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmClassifierModuleImplementation {
    type Vtable = IFsrmClassifierModuleImplementation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c968fc6_6edb_4051_9c18_73b7291ae106);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassifierModuleImplementation_Vtbl {
    pub base: IFsrmPipelineModuleImplementation_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LastModified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastmodified: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LastModified: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub UseRulesAndDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rules: ::windows::core::RawPtr, propertydefinitions: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UseRulesAndDefinitions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnBeginFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertybag: ::windows::core::RawPtr, arrayruleids: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnBeginFile: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DoesPropertyValueApply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, applyvalue: *mut i16, idrule: ::windows::core::GUID, idpropdef: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DoesPropertyValueApply: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPropertyValueToApply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut super::super::Foundation::BSTR, idrule: ::windows::core::GUID, idpropdef: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPropertyValueToApply: usize,
    pub OnEndFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmCollection {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn State(&self) -> ::windows::core::Result<FsrmCollectionState> {
        let mut result__: FsrmCollectionState = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).State)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmCollectionState>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).WaitForCompletion)(::core::mem::transmute_copy(self), ::core::mem::transmute(waitseconds), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetById<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, id: Param0) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetById)(::core::mem::transmute_copy(self), id.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmCollection> for ::windows::core::IUnknown {
    fn from(value: IFsrmCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmCollection> for ::windows::core::IUnknown {
    fn from(value: &IFsrmCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmCollection {
    type Vtable = IFsrmCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf76fbf3b_8ddd_4b42_b05a_cb1c3ff1fee8);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmCollection_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, item: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: *mut FsrmCollectionState) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WaitForCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::windows::core::GUID, entry: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetById: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmCommittableCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmCommittableCollection {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.Item)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn State(&self) -> ::windows::core::Result<FsrmCollectionState> {
        let mut result__: FsrmCollectionState = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.State)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmCollectionState>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Cancel)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.WaitForCompletion)(::core::mem::transmute_copy(self), ::core::mem::transmute(waitseconds), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetById<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, id: Param0) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetById)(::core::mem::transmute_copy(self), id.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, item: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Add)(::core::mem::transmute_copy(self), item.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Remove(&self, index: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Remove)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn RemoveById<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, id: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.RemoveById)(::core::mem::transmute_copy(self), id.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmMutableCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Commit(&self, options: FsrmCommitOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Commit)(::core::mem::transmute_copy(self), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmCommittableCollection> for ::windows::core::IUnknown {
    fn from(value: IFsrmCommittableCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmCommittableCollection> for ::windows::core::IUnknown {
    fn from(value: &IFsrmCommittableCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmCommittableCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmCommittableCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmCommittableCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmCommittableCollection> for IFsrmCollection {
    fn from(value: IFsrmCommittableCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmCommittableCollection> for IFsrmCollection {
    fn from(value: &IFsrmCommittableCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmCollection> for IFsrmCommittableCollection {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmCollection> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmCollection> for &'a IFsrmCommittableCollection {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmCollection> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmCommittableCollection> for IFsrmMutableCollection {
    fn from(value: IFsrmCommittableCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmCommittableCollection> for IFsrmMutableCollection {
    fn from(value: &IFsrmCommittableCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmMutableCollection> for IFsrmCommittableCollection {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmMutableCollection> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmMutableCollection> for &'a IFsrmCommittableCollection {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmMutableCollection> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmCommittableCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmCommittableCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmCommittableCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmCommittableCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmCommittableCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmCommittableCollection {
    type Vtable = IFsrmCommittableCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96deb3b5_8b91_4a2a_9d93_80a35d8aa847);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmCommittableCollection_Vtbl {
    pub base: IFsrmMutableCollection_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FsrmCommitOptions, results: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Commit: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmDerivedObjectsResult(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmDerivedObjectsResult {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DerivedObjects(&self) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DerivedObjects)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Results(&self) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Results)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmDerivedObjectsResult> for ::windows::core::IUnknown {
    fn from(value: IFsrmDerivedObjectsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmDerivedObjectsResult> for ::windows::core::IUnknown {
    fn from(value: &IFsrmDerivedObjectsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmDerivedObjectsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmDerivedObjectsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmDerivedObjectsResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmDerivedObjectsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmDerivedObjectsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmDerivedObjectsResult {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmDerivedObjectsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmDerivedObjectsResult").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmDerivedObjectsResult {
    type Vtable = IFsrmDerivedObjectsResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39322a2d_38ee_4d0d_8095_421a80849a82);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmDerivedObjectsResult_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub DerivedObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, derivedobjects: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DerivedObjects: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Results: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, results: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Results: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmExportImport(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmExportImport {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ExportFileGroups<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, filegroupnamessafearray: *const super::super::System::Com::VARIANT, remotehost: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExportFileGroups)(::core::mem::transmute_copy(self), filepath.into_param().abi(), ::core::mem::transmute(filegroupnamessafearray), remotehost.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ImportFileGroups<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, filegroupnamessafearray: *const super::super::System::Com::VARIANT, remotehost: Param2) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ImportFileGroups)(::core::mem::transmute_copy(self), filepath.into_param().abi(), ::core::mem::transmute(filegroupnamessafearray), remotehost.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ExportFileScreenTemplates<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExportFileScreenTemplates)(::core::mem::transmute_copy(self), filepath.into_param().abi(), ::core::mem::transmute(templatenamessafearray), remotehost.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ImportFileScreenTemplates<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: Param2) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ImportFileScreenTemplates)(::core::mem::transmute_copy(self), filepath.into_param().abi(), ::core::mem::transmute(templatenamessafearray), remotehost.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ExportQuotaTemplates<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExportQuotaTemplates)(::core::mem::transmute_copy(self), filepath.into_param().abi(), ::core::mem::transmute(templatenamessafearray), remotehost.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ImportQuotaTemplates<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: Param2) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ImportQuotaTemplates)(::core::mem::transmute_copy(self), filepath.into_param().abi(), ::core::mem::transmute(templatenamessafearray), remotehost.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmExportImport> for ::windows::core::IUnknown {
    fn from(value: IFsrmExportImport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmExportImport> for ::windows::core::IUnknown {
    fn from(value: &IFsrmExportImport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmExportImport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmExportImport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmExportImport {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmExportImport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmExportImport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmExportImport {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmExportImport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmExportImport").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmExportImport {
    type Vtable = IFsrmExportImport_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefcb0ab1_16c4_4a79_812c_725614c3306b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmExportImport_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ExportFileGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filegroupnamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ExportFileGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ImportFileGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filegroupnamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filegroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ImportFileGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ExportFileScreenTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ExportFileScreenTemplates: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ImportFileScreenTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, templates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ImportFileScreenTemplates: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ExportQuotaTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ExportQuotaTemplates: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ImportQuotaTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, templates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ImportQuotaTemplates: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmFileCondition(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileCondition {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Type(&self) -> ::windows::core::Result<FsrmFileConditionType> {
        let mut result__: FsrmFileConditionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Type)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmFileConditionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileCondition> for ::windows::core::IUnknown {
    fn from(value: IFsrmFileCondition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileCondition> for ::windows::core::IUnknown {
    fn from(value: &IFsrmFileCondition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmFileCondition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmFileCondition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmFileCondition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmFileCondition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmFileCondition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmFileCondition {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmFileCondition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileCondition").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmFileCondition {
    type Vtable = IFsrmFileCondition_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70684ffc_691a_4a1a_b922_97752e138cc1);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileCondition_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut FsrmFileConditionType) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmFileConditionProperty(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileConditionProperty {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Type(&self) -> ::windows::core::Result<FsrmFileConditionType> {
        let mut result__: FsrmFileConditionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Type)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmFileConditionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PropertyName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PropertyName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPropertyName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPropertyName)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn PropertyId(&self) -> ::windows::core::Result<FsrmFileSystemPropertyId> {
        let mut result__: FsrmFileSystemPropertyId = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PropertyId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmFileSystemPropertyId>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetPropertyId(&self, newval: FsrmFileSystemPropertyId) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPropertyId)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Operator(&self) -> ::windows::core::Result<FsrmPropertyConditionType> {
        let mut result__: FsrmPropertyConditionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Operator)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmPropertyConditionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetOperator(&self, newval: FsrmPropertyConditionType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOperator)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn ValueType(&self) -> ::windows::core::Result<FsrmPropertyValueType> {
        let mut result__: FsrmPropertyValueType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ValueType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmPropertyValueType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetValueType(&self, newval: FsrmPropertyValueType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetValueType)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Value)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, newval: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetValue)(::core::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileConditionProperty> for ::windows::core::IUnknown {
    fn from(value: IFsrmFileConditionProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileConditionProperty> for ::windows::core::IUnknown {
    fn from(value: &IFsrmFileConditionProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmFileConditionProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmFileConditionProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmFileConditionProperty {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileConditionProperty> for IFsrmFileCondition {
    fn from(value: IFsrmFileConditionProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileConditionProperty> for IFsrmFileCondition {
    fn from(value: &IFsrmFileConditionProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmFileCondition> for IFsrmFileConditionProperty {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmFileCondition> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmFileCondition> for &'a IFsrmFileConditionProperty {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmFileCondition> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmFileConditionProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmFileConditionProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmFileConditionProperty {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmFileConditionProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileConditionProperty").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmFileConditionProperty {
    type Vtable = IFsrmFileConditionProperty_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81926775_b981_4479_988f_da171d627360);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileConditionProperty_Vtbl {
    pub base: IFsrmFileCondition_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub PropertyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PropertyName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPropertyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPropertyName: usize,
    pub PropertyId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut FsrmFileSystemPropertyId) -> ::windows::core::HRESULT,
    pub SetPropertyId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: FsrmFileSystemPropertyId) -> ::windows::core::HRESULT,
    pub Operator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut FsrmPropertyConditionType) -> ::windows::core::HRESULT,
    pub SetOperator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: FsrmPropertyConditionType) -> ::windows::core::HRESULT,
    pub ValueType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut FsrmPropertyValueType) -> ::windows::core::HRESULT,
    pub SetValueType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: FsrmPropertyValueType) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Value: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmFileGroup(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileGroup {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDescription)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Commit)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Members(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Members)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmMutableCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetMembers<'a, Param0: ::windows::core::IntoParam<'a, IFsrmMutableCollection>>(&self, members: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMembers)(::core::mem::transmute_copy(self), members.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NonMembers(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).NonMembers)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmMutableCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetNonMembers<'a, Param0: ::windows::core::IntoParam<'a, IFsrmMutableCollection>>(&self, nonmembers: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNonMembers)(::core::mem::transmute_copy(self), nonmembers.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileGroup> for ::windows::core::IUnknown {
    fn from(value: IFsrmFileGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileGroup> for ::windows::core::IUnknown {
    fn from(value: &IFsrmFileGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmFileGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmFileGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmFileGroup {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileGroup> for IFsrmObject {
    fn from(value: IFsrmFileGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileGroup> for IFsrmObject {
    fn from(value: &IFsrmFileGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmFileGroup {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &'a IFsrmFileGroup {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmFileGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmFileGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmFileGroup {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmFileGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileGroup").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmFileGroup {
    type Vtable = IFsrmFileGroup_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8dd04909_0e34_4d55_afaa_89e1f1a1bbb9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileGroup_Vtbl {
    pub base: IFsrmObject_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Members: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, members: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Members: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetMembers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, members: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetMembers: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NonMembers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nonmembers: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NonMembers: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetNonMembers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nonmembers: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetNonMembers: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmFileGroupImported(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileGroupImported {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetDescription)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Commit)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Members(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Members)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmMutableCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetMembers<'a, Param0: ::windows::core::IntoParam<'a, IFsrmMutableCollection>>(&self, members: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetMembers)(::core::mem::transmute_copy(self), members.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NonMembers(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.NonMembers)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmMutableCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetNonMembers<'a, Param0: ::windows::core::IntoParam<'a, IFsrmMutableCollection>>(&self, nonmembers: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetNonMembers)(::core::mem::transmute_copy(self), nonmembers.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn OverwriteOnCommit(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OverwriteOnCommit)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetOverwriteOnCommit(&self, overwrite: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOverwriteOnCommit)(::core::mem::transmute_copy(self), ::core::mem::transmute(overwrite)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileGroupImported> for ::windows::core::IUnknown {
    fn from(value: IFsrmFileGroupImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileGroupImported> for ::windows::core::IUnknown {
    fn from(value: &IFsrmFileGroupImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmFileGroupImported {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmFileGroupImported {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmFileGroupImported {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileGroupImported> for IFsrmObject {
    fn from(value: IFsrmFileGroupImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileGroupImported> for IFsrmObject {
    fn from(value: &IFsrmFileGroupImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmFileGroupImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &'a IFsrmFileGroupImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileGroupImported> for IFsrmFileGroup {
    fn from(value: IFsrmFileGroupImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileGroupImported> for IFsrmFileGroup {
    fn from(value: &IFsrmFileGroupImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmFileGroup> for IFsrmFileGroupImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmFileGroup> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmFileGroup> for &'a IFsrmFileGroupImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmFileGroup> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmFileGroupImported {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmFileGroupImported {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmFileGroupImported {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmFileGroupImported {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileGroupImported").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmFileGroupImported {
    type Vtable = IFsrmFileGroupImported_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad55f10b_5f11_4be7_94ef_d9ee2e470ded);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileGroupImported_Vtbl {
    pub base: IFsrmFileGroup_Vtbl,
    pub OverwriteOnCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: *mut i16) -> ::windows::core::HRESULT,
    pub SetOverwriteOnCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmFileGroupManager(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileGroupManager {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateFileGroup(&self) -> ::windows::core::Result<IFsrmFileGroup> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateFileGroup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmFileGroup>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetFileGroup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<IFsrmFileGroup> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFileGroup)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmFileGroup>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumFileGroups(&self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumFileGroups)(::core::mem::transmute_copy(self), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ExportFileGroups(&self, filegroupnamesarray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ExportFileGroups)(::core::mem::transmute_copy(self), ::core::mem::transmute(filegroupnamesarray), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ImportFileGroups<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, serializedfilegroups: Param0, filegroupnamesarray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ImportFileGroups)(::core::mem::transmute_copy(self), serializedfilegroups.into_param().abi(), ::core::mem::transmute(filegroupnamesarray), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileGroupManager> for ::windows::core::IUnknown {
    fn from(value: IFsrmFileGroupManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileGroupManager> for ::windows::core::IUnknown {
    fn from(value: &IFsrmFileGroupManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmFileGroupManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmFileGroupManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmFileGroupManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmFileGroupManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmFileGroupManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmFileGroupManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmFileGroupManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileGroupManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmFileGroupManager {
    type Vtable = IFsrmFileGroupManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x426677d5_018c_485c_8a51_20b86d00bdc4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileGroupManager_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateFileGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filegroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateFileGroup: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetFileGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filegroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetFileGroup: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumFileGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, filegroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumFileGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ExportFileGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filegroupnamesarray: *const super::super::System::Com::VARIANT, serializedfilegroups: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ExportFileGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ImportFileGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serializedfilegroups: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filegroupnamesarray: *const super::super::System::Com::VARIANT, filegroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ImportFileGroups: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmFileManagementJob(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileManagementJob {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDescription)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Commit)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NamespaceRoots(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).NamespaceRoots)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetNamespaceRoots(&self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNamespaceRoots)(::core::mem::transmute_copy(self), ::core::mem::transmute(namespaceroots)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Enabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn OperationType(&self) -> ::windows::core::Result<FsrmFileManagementType> {
        let mut result__: FsrmFileManagementType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OperationType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmFileManagementType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetOperationType(&self, operationtype: FsrmFileManagementType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOperationType)(::core::mem::transmute_copy(self), ::core::mem::transmute(operationtype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExpirationDirectory(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ExpirationDirectory)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExpirationDirectory<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, expirationdirectory: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetExpirationDirectory)(::core::mem::transmute_copy(self), expirationdirectory.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CustomAction(&self) -> ::windows::core::Result<IFsrmActionCommand> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CustomAction)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmActionCommand>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Notifications(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Notifications)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Logging(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Logging)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetLogging(&self, loggingflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLogging)(::core::mem::transmute_copy(self), ::core::mem::transmute(loggingflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn ReportEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ReportEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetReportEnabled(&self, reportenabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetReportEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(reportenabled)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Formats(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Formats)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFormats(&self, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFormats)(::core::mem::transmute_copy(self), ::core::mem::transmute(formats)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailTo(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MailTo)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailto: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMailTo)(::core::mem::transmute_copy(self), mailto.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn DaysSinceFileCreated(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DaysSinceFileCreated)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetDaysSinceFileCreated(&self, dayssincecreation: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDaysSinceFileCreated)(::core::mem::transmute_copy(self), ::core::mem::transmute(dayssincecreation)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn DaysSinceFileLastAccessed(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DaysSinceFileLastAccessed)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetDaysSinceFileLastAccessed(&self, dayssinceaccess: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDaysSinceFileLastAccessed)(::core::mem::transmute_copy(self), ::core::mem::transmute(dayssinceaccess)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn DaysSinceFileLastModified(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DaysSinceFileLastModified)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetDaysSinceFileLastModified(&self, dayssincemodify: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDaysSinceFileLastModified)(::core::mem::transmute_copy(self), ::core::mem::transmute(dayssincemodify)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PropertyConditions(&self) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PropertyConditions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn FromDate(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FromDate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetFromDate(&self, fromdate: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFromDate)(::core::mem::transmute_copy(self), ::core::mem::transmute(fromdate)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Task(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Task)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTask<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, taskname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTask)(::core::mem::transmute_copy(self), taskname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Parameters)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetParameters)(::core::mem::transmute_copy(self), ::core::mem::transmute(parameters)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn RunningStatus(&self) -> ::windows::core::Result<FsrmReportRunningStatus> {
        let mut result__: FsrmReportRunningStatus = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RunningStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmReportRunningStatus>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastError(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LastError)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastReportPathWithoutExtension(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LastReportPathWithoutExtension)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn LastRun(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LastRun)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileNamePattern(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FileNamePattern)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileNamePattern<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filenamepattern: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFileNamePattern)(::core::mem::transmute_copy(self), filenamepattern.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Run(&self, context: FsrmReportGenerationContext) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Run)(::core::mem::transmute_copy(self), ::core::mem::transmute(context)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).WaitForCompletion)(::core::mem::transmute_copy(self), ::core::mem::transmute(waitseconds), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn AddNotification(&self, days: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddNotification)(::core::mem::transmute_copy(self), ::core::mem::transmute(days)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn DeleteNotification(&self, days: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteNotification)(::core::mem::transmute_copy(self), ::core::mem::transmute(days)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn ModifyNotification(&self, days: i32, newdays: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ModifyNotification)(::core::mem::transmute_copy(self), ::core::mem::transmute(days), ::core::mem::transmute(newdays)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateNotificationAction(&self, days: i32, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateNotificationAction)(::core::mem::transmute_copy(self), ::core::mem::transmute(days), ::core::mem::transmute(actiontype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmAction>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumNotificationActions(&self, days: i32) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumNotificationActions)(::core::mem::transmute_copy(self), ::core::mem::transmute(days), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePropertyCondition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<IFsrmPropertyCondition> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreatePropertyCondition)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmPropertyCondition>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateCustomAction(&self) -> ::windows::core::Result<IFsrmActionCommand> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateCustomAction)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmActionCommand>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileManagementJob> for ::windows::core::IUnknown {
    fn from(value: IFsrmFileManagementJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileManagementJob> for ::windows::core::IUnknown {
    fn from(value: &IFsrmFileManagementJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmFileManagementJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmFileManagementJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmFileManagementJob {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileManagementJob> for IFsrmObject {
    fn from(value: IFsrmFileManagementJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileManagementJob> for IFsrmObject {
    fn from(value: &IFsrmFileManagementJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmFileManagementJob {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &'a IFsrmFileManagementJob {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmFileManagementJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmFileManagementJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmFileManagementJob {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmFileManagementJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileManagementJob").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmFileManagementJob {
    type Vtable = IFsrmFileManagementJob_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0770687e_9f36_4d6f_8778_599d188461c9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileManagementJob_Vtbl {
    pub base: IFsrmObject_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NamespaceRoots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NamespaceRoots: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetNamespaceRoots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetNamespaceRoots: usize,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT,
    pub OperationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operationtype: *mut FsrmFileManagementType) -> ::windows::core::HRESULT,
    pub SetOperationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operationtype: FsrmFileManagementType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ExpirationDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, expirationdirectory: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExpirationDirectory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetExpirationDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, expirationdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetExpirationDirectory: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CustomAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CustomAction: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Notifications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notifications: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Notifications: usize,
    pub Logging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loggingflags: *mut i32) -> ::windows::core::HRESULT,
    pub SetLogging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loggingflags: i32) -> ::windows::core::HRESULT,
    pub ReportEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportenabled: *mut i16) -> ::windows::core::HRESULT,
    pub SetReportEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportenabled: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Formats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Formats: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetFormats: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MailTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MailTo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMailTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMailTo: usize,
    pub DaysSinceFileCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dayssincecreation: *mut i32) -> ::windows::core::HRESULT,
    pub SetDaysSinceFileCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dayssincecreation: i32) -> ::windows::core::HRESULT,
    pub DaysSinceFileLastAccessed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dayssinceaccess: *mut i32) -> ::windows::core::HRESULT,
    pub SetDaysSinceFileLastAccessed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dayssinceaccess: i32) -> ::windows::core::HRESULT,
    pub DaysSinceFileLastModified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dayssincemodify: *mut i32) -> ::windows::core::HRESULT,
    pub SetDaysSinceFileLastModified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dayssincemodify: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PropertyConditions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyconditions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PropertyConditions: usize,
    pub FromDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fromdate: *mut f64) -> ::windows::core::HRESULT,
    pub SetFromDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fromdate: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Task: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Task: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTask: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Parameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parameters: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetParameters: usize,
    pub RunningStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runningstatus: *mut FsrmReportRunningStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LastError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lasterror: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LastError: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LastReportPathWithoutExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LastReportPathWithoutExtension: usize,
    pub LastRun: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastrun: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FileNamePattern: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filenamepattern: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FileNamePattern: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFileNamePattern: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filenamepattern: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFileNamePattern: usize,
    pub Run: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext) -> ::windows::core::HRESULT,
    pub WaitForCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut i16) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: i32) -> ::windows::core::HRESULT,
    pub DeleteNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: i32) -> ::windows::core::HRESULT,
    pub ModifyNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: i32, newdays: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateNotificationAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: i32, actiontype: FsrmActionType, action: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateNotificationAction: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumNotificationActions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: i32, actions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumNotificationActions: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreatePropertyCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertycondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreatePropertyCondition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateCustomAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateCustomAction: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmFileManagementJobManager(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileManagementJobManager {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariables(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ActionVariables)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariableDescriptions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ActionVariableDescriptions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumFileManagementJobs(&self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumFileManagementJobs)(::core::mem::transmute_copy(self), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateFileManagementJob(&self) -> ::windows::core::Result<IFsrmFileManagementJob> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateFileManagementJob)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmFileManagementJob>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetFileManagementJob<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<IFsrmFileManagementJob> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFileManagementJob)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmFileManagementJob>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileManagementJobManager> for ::windows::core::IUnknown {
    fn from(value: IFsrmFileManagementJobManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileManagementJobManager> for ::windows::core::IUnknown {
    fn from(value: &IFsrmFileManagementJobManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmFileManagementJobManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmFileManagementJobManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmFileManagementJobManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmFileManagementJobManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmFileManagementJobManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmFileManagementJobManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmFileManagementJobManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileManagementJobManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmFileManagementJobManager {
    type Vtable = IFsrmFileManagementJobManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee321ecb_d95e_48e9_907c_c7685a013235);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileManagementJobManager_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ActionVariables: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActionVariables: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ActionVariableDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActionVariableDescriptions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumFileManagementJobs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, filemanagementjobs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumFileManagementJobs: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateFileManagementJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filemanagementjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateFileManagementJob: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetFileManagementJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filemanagementjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetFileManagementJob: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmFileScreen(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileScreen {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetDescription)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Commit)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BlockedFileGroups(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.BlockedFileGroups)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmMutableCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetBlockedFileGroups<'a, Param0: ::windows::core::IntoParam<'a, IFsrmMutableCollection>>(&self, blocklist: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetBlockedFileGroups)(::core::mem::transmute_copy(self), blocklist.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn FileScreenFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.FileScreenFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetFileScreenFlags(&self, filescreenflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetFileScreenFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(filescreenflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateAction(&self, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateAction)(::core::mem::transmute_copy(self), ::core::mem::transmute(actiontype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmAction>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumActions(&self) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.EnumActions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Path)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SourceTemplateName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SourceTemplateName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn MatchesSourceTemplate(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MatchesSourceTemplate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserSid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).UserSid)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserAccount(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).UserAccount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplyTemplate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filescreentemplatename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ApplyTemplate)(::core::mem::transmute_copy(self), filescreentemplatename.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileScreen> for ::windows::core::IUnknown {
    fn from(value: IFsrmFileScreen) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileScreen> for ::windows::core::IUnknown {
    fn from(value: &IFsrmFileScreen) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmFileScreen {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmFileScreen {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmFileScreen {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileScreen> for IFsrmObject {
    fn from(value: IFsrmFileScreen) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileScreen> for IFsrmObject {
    fn from(value: &IFsrmFileScreen) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmFileScreen {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &'a IFsrmFileScreen {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileScreen> for IFsrmFileScreenBase {
    fn from(value: IFsrmFileScreen) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileScreen> for IFsrmFileScreenBase {
    fn from(value: &IFsrmFileScreen) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmFileScreenBase> for IFsrmFileScreen {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmFileScreenBase> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmFileScreenBase> for &'a IFsrmFileScreen {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmFileScreenBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmFileScreen {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmFileScreen {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmFileScreen {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmFileScreen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileScreen").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmFileScreen {
    type Vtable = IFsrmFileScreen_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f6325d3_ce88_4733_84c1_2d6aefc5ea07);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreen_Vtbl {
    pub base: IFsrmFileScreenBase_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Path: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SourceTemplateName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreentemplatename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SourceTemplateName: usize,
    pub MatchesSourceTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matches: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UserSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserSid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UserAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccount: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserAccount: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ApplyTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreentemplatename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ApplyTemplate: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmFileScreenBase(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileScreenBase {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDescription)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Commit)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BlockedFileGroups(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).BlockedFileGroups)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmMutableCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetBlockedFileGroups<'a, Param0: ::windows::core::IntoParam<'a, IFsrmMutableCollection>>(&self, blocklist: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBlockedFileGroups)(::core::mem::transmute_copy(self), blocklist.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn FileScreenFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FileScreenFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetFileScreenFlags(&self, filescreenflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFileScreenFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(filescreenflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateAction(&self, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateAction)(::core::mem::transmute_copy(self), ::core::mem::transmute(actiontype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmAction>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumActions(&self) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumActions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileScreenBase> for ::windows::core::IUnknown {
    fn from(value: IFsrmFileScreenBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileScreenBase> for ::windows::core::IUnknown {
    fn from(value: &IFsrmFileScreenBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmFileScreenBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmFileScreenBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmFileScreenBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileScreenBase> for IFsrmObject {
    fn from(value: IFsrmFileScreenBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileScreenBase> for IFsrmObject {
    fn from(value: &IFsrmFileScreenBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmFileScreenBase {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &'a IFsrmFileScreenBase {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmFileScreenBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmFileScreenBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmFileScreenBase {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmFileScreenBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileScreenBase").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmFileScreenBase {
    type Vtable = IFsrmFileScreenBase_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3637e80_5b22_4a2b_a637_bbb642b41cfc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenBase_Vtbl {
    pub base: IFsrmObject_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub BlockedFileGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blocklist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BlockedFileGroups: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetBlockedFileGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blocklist: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetBlockedFileGroups: usize,
    pub FileScreenFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreenflags: *mut i32) -> ::windows::core::HRESULT,
    pub SetFileScreenFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreenflags: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actiontype: FsrmActionType, action: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateAction: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumActions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumActions: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmFileScreenException(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileScreenException {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDescription)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Commit)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Path)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AllowedFileGroups(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AllowedFileGroups)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmMutableCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetAllowedFileGroups<'a, Param0: ::windows::core::IntoParam<'a, IFsrmMutableCollection>>(&self, allowlist: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllowedFileGroups)(::core::mem::transmute_copy(self), allowlist.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileScreenException> for ::windows::core::IUnknown {
    fn from(value: IFsrmFileScreenException) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileScreenException> for ::windows::core::IUnknown {
    fn from(value: &IFsrmFileScreenException) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmFileScreenException {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmFileScreenException {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmFileScreenException {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileScreenException> for IFsrmObject {
    fn from(value: IFsrmFileScreenException) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileScreenException> for IFsrmObject {
    fn from(value: &IFsrmFileScreenException) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmFileScreenException {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &'a IFsrmFileScreenException {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmFileScreenException {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmFileScreenException {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmFileScreenException {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmFileScreenException {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileScreenException").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmFileScreenException {
    type Vtable = IFsrmFileScreenException_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbee7ce02_df77_4515_9389_78f01c5afc1a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenException_Vtbl {
    pub base: IFsrmObject_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Path: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AllowedFileGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allowlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AllowedFileGroups: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetAllowedFileGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allowlist: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetAllowedFileGroups: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmFileScreenManager(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileScreenManager {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariables(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ActionVariables)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariableDescriptions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ActionVariableDescriptions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateFileScreen<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<IFsrmFileScreen> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateFileScreen)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmFileScreen>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetFileScreen<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<IFsrmFileScreen> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFileScreen)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmFileScreen>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn EnumFileScreens<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumFileScreens)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateFileScreenException<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<IFsrmFileScreenException> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateFileScreenException)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmFileScreenException>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetFileScreenException<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<IFsrmFileScreenException> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFileScreenException)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmFileScreenException>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn EnumFileScreenExceptions<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumFileScreenExceptions)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateFileScreenCollection(&self) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateFileScreenCollection)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileScreenManager> for ::windows::core::IUnknown {
    fn from(value: IFsrmFileScreenManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileScreenManager> for ::windows::core::IUnknown {
    fn from(value: &IFsrmFileScreenManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmFileScreenManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmFileScreenManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmFileScreenManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmFileScreenManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmFileScreenManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmFileScreenManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmFileScreenManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileScreenManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmFileScreenManager {
    type Vtable = IFsrmFileScreenManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff4fa04e_5a94_4bda_a3a0_d5b4d3c52eba);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenManager_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ActionVariables: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActionVariables: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ActionVariableDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActionVariableDescriptions: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateFileScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreen: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateFileScreen: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetFileScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreen: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetFileScreen: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub EnumFileScreens: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, filescreens: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    EnumFileScreens: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateFileScreenException: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreenexception: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateFileScreenException: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetFileScreenException: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreenexception: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetFileScreenException: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub EnumFileScreenExceptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, filescreenexceptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    EnumFileScreenExceptions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateFileScreenCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateFileScreenCollection: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmFileScreenTemplate(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileScreenTemplate {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetDescription)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Commit)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BlockedFileGroups(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.BlockedFileGroups)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmMutableCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetBlockedFileGroups<'a, Param0: ::windows::core::IntoParam<'a, IFsrmMutableCollection>>(&self, blocklist: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetBlockedFileGroups)(::core::mem::transmute_copy(self), blocklist.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn FileScreenFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.FileScreenFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetFileScreenFlags(&self, filescreenflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetFileScreenFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(filescreenflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateAction(&self, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateAction)(::core::mem::transmute_copy(self), ::core::mem::transmute(actiontype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmAction>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumActions(&self) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.EnumActions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTemplate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filescreentemplatename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CopyTemplate)(::core::mem::transmute_copy(self), filescreentemplatename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows::core::Result<IFsrmDerivedObjectsResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CommitAndUpdateDerived)(::core::mem::transmute_copy(self), ::core::mem::transmute(commitoptions), ::core::mem::transmute(applyoptions), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmDerivedObjectsResult>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileScreenTemplate> for ::windows::core::IUnknown {
    fn from(value: IFsrmFileScreenTemplate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileScreenTemplate> for ::windows::core::IUnknown {
    fn from(value: &IFsrmFileScreenTemplate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmFileScreenTemplate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmFileScreenTemplate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmFileScreenTemplate {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileScreenTemplate> for IFsrmObject {
    fn from(value: IFsrmFileScreenTemplate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileScreenTemplate> for IFsrmObject {
    fn from(value: &IFsrmFileScreenTemplate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmFileScreenTemplate {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &'a IFsrmFileScreenTemplate {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileScreenTemplate> for IFsrmFileScreenBase {
    fn from(value: IFsrmFileScreenTemplate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileScreenTemplate> for IFsrmFileScreenBase {
    fn from(value: &IFsrmFileScreenTemplate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmFileScreenBase> for IFsrmFileScreenTemplate {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmFileScreenBase> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmFileScreenBase> for &'a IFsrmFileScreenTemplate {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmFileScreenBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmFileScreenTemplate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmFileScreenTemplate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmFileScreenTemplate {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmFileScreenTemplate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileScreenTemplate").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmFileScreenTemplate {
    type Vtable = IFsrmFileScreenTemplate_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x205bebf8_dd93_452a_95a6_32b566b35828);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenTemplate_Vtbl {
    pub base: IFsrmFileScreenBase_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CopyTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreentemplatename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CopyTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CommitAndUpdateDerived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommitAndUpdateDerived: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmFileScreenTemplateImported(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileScreenTemplateImported {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetDescription)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.Commit)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BlockedFileGroups(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.BlockedFileGroups)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmMutableCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetBlockedFileGroups<'a, Param0: ::windows::core::IntoParam<'a, IFsrmMutableCollection>>(&self, blocklist: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetBlockedFileGroups)(::core::mem::transmute_copy(self), blocklist.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn FileScreenFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.FileScreenFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetFileScreenFlags(&self, filescreenflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetFileScreenFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(filescreenflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateAction(&self, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.CreateAction)(::core::mem::transmute_copy(self), ::core::mem::transmute(actiontype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmAction>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumActions(&self) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.EnumActions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTemplate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filescreentemplatename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CopyTemplate)(::core::mem::transmute_copy(self), filescreentemplatename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows::core::Result<IFsrmDerivedObjectsResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CommitAndUpdateDerived)(::core::mem::transmute_copy(self), ::core::mem::transmute(commitoptions), ::core::mem::transmute(applyoptions), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmDerivedObjectsResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn OverwriteOnCommit(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OverwriteOnCommit)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetOverwriteOnCommit(&self, overwrite: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOverwriteOnCommit)(::core::mem::transmute_copy(self), ::core::mem::transmute(overwrite)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileScreenTemplateImported> for ::windows::core::IUnknown {
    fn from(value: IFsrmFileScreenTemplateImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileScreenTemplateImported> for ::windows::core::IUnknown {
    fn from(value: &IFsrmFileScreenTemplateImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileScreenTemplateImported> for IFsrmObject {
    fn from(value: IFsrmFileScreenTemplateImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileScreenTemplateImported> for IFsrmObject {
    fn from(value: &IFsrmFileScreenTemplateImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &'a IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileScreenTemplateImported> for IFsrmFileScreenBase {
    fn from(value: IFsrmFileScreenTemplateImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileScreenTemplateImported> for IFsrmFileScreenBase {
    fn from(value: &IFsrmFileScreenTemplateImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmFileScreenBase> for IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmFileScreenBase> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmFileScreenBase> for &'a IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmFileScreenBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileScreenTemplateImported> for IFsrmFileScreenTemplate {
    fn from(value: IFsrmFileScreenTemplateImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileScreenTemplateImported> for IFsrmFileScreenTemplate {
    fn from(value: &IFsrmFileScreenTemplateImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmFileScreenTemplate> for IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmFileScreenTemplate> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmFileScreenTemplate> for &'a IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmFileScreenTemplate> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmFileScreenTemplateImported {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmFileScreenTemplateImported {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmFileScreenTemplateImported {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmFileScreenTemplateImported {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileScreenTemplateImported").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmFileScreenTemplateImported {
    type Vtable = IFsrmFileScreenTemplateImported_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1010359_3e5d_4ecd_9fe4_ef48622fdf30);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenTemplateImported_Vtbl {
    pub base: IFsrmFileScreenTemplate_Vtbl,
    pub OverwriteOnCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: *mut i16) -> ::windows::core::HRESULT,
    pub SetOverwriteOnCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmFileScreenTemplateManager(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileScreenTemplateManager {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateTemplate(&self) -> ::windows::core::Result<IFsrmFileScreenTemplate> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateTemplate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmFileScreenTemplate>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetTemplate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<IFsrmFileScreenTemplate> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTemplate)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmFileScreenTemplate>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumTemplates(&self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumTemplates)(::core::mem::transmute_copy(self), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ExportTemplates(&self, filescreentemplatenamesarray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ExportTemplates)(::core::mem::transmute_copy(self), ::core::mem::transmute(filescreentemplatenamesarray), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ImportTemplates<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, serializedfilescreentemplates: Param0, filescreentemplatenamesarray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ImportTemplates)(::core::mem::transmute_copy(self), serializedfilescreentemplates.into_param().abi(), ::core::mem::transmute(filescreentemplatenamesarray), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmFileScreenTemplateManager> for ::windows::core::IUnknown {
    fn from(value: IFsrmFileScreenTemplateManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmFileScreenTemplateManager> for ::windows::core::IUnknown {
    fn from(value: &IFsrmFileScreenTemplateManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmFileScreenTemplateManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmFileScreenTemplateManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmFileScreenTemplateManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmFileScreenTemplateManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmFileScreenTemplateManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmFileScreenTemplateManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmFileScreenTemplateManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileScreenTemplateManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmFileScreenTemplateManager {
    type Vtable = IFsrmFileScreenTemplateManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfe36cba_1949_4e74_a14f_f1d580ceaf13);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenTemplateManager_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreentemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateTemplate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreentemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, filescreentemplates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumTemplates: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ExportTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreentemplatenamesarray: *const super::super::System::Com::VARIANT, serializedfilescreentemplates: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ExportTemplates: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ImportTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serializedfilescreentemplates: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreentemplatenamesarray: *const super::super::System::Com::VARIANT, filescreentemplates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ImportTemplates: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmMutableCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmMutableCollection {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Item)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn State(&self) -> ::windows::core::Result<FsrmCollectionState> {
        let mut result__: FsrmCollectionState = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.State)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmCollectionState>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Cancel)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.WaitForCompletion)(::core::mem::transmute_copy(self), ::core::mem::transmute(waitseconds), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetById<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, id: Param0) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetById)(::core::mem::transmute_copy(self), id.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, item: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Add)(::core::mem::transmute_copy(self), item.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Remove(&self, index: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Remove)(::core::mem::transmute_copy(self), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn RemoveById<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, id: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveById)(::core::mem::transmute_copy(self), id.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmMutableCollection>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmMutableCollection> for ::windows::core::IUnknown {
    fn from(value: IFsrmMutableCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmMutableCollection> for ::windows::core::IUnknown {
    fn from(value: &IFsrmMutableCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmMutableCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmMutableCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmMutableCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmMutableCollection> for IFsrmCollection {
    fn from(value: IFsrmMutableCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmMutableCollection> for IFsrmCollection {
    fn from(value: &IFsrmMutableCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmCollection> for IFsrmMutableCollection {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmCollection> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmCollection> for &'a IFsrmMutableCollection {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmCollection> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmMutableCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmMutableCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmMutableCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmMutableCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmMutableCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmMutableCollection {
    type Vtable = IFsrmMutableCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bb617b8_3886_49dc_af82_a6c90fa35dda);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmMutableCollection_Vtbl {
    pub base: IFsrmCollection_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT,
    pub RemoveById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmObject(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmObject {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDescription)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Commit)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmObject> for ::windows::core::IUnknown {
    fn from(value: IFsrmObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmObject> for ::windows::core::IUnknown {
    fn from(value: &IFsrmObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmObject {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmObject {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmObject").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmObject {
    type Vtable = IFsrmObject_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22bcef93_4a3f_4183_89f9_2f8b8a628aee);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmObject_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmPathMapper(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPathMapper {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetSharePathsForLocalPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, localpath: Param0) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSharePathsForLocalPath)(::core::mem::transmute_copy(self), localpath.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmPathMapper> for ::windows::core::IUnknown {
    fn from(value: IFsrmPathMapper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmPathMapper> for ::windows::core::IUnknown {
    fn from(value: &IFsrmPathMapper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmPathMapper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmPathMapper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmPathMapper {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmPathMapper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmPathMapper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmPathMapper {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmPathMapper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmPathMapper").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmPathMapper {
    type Vtable = IFsrmPathMapper_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f4dbfff_6920_4821_a6c3_b7e94c1fd60c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPathMapper_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetSharePathsForLocalPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sharepaths: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetSharePathsForLocalPath: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmPipelineModuleConnector(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPipelineModuleConnector {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ModuleImplementation(&self) -> ::windows::core::Result<IFsrmPipelineModuleImplementation> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ModuleImplementation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmPipelineModuleImplementation>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModuleName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ModuleName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HostingUserAccount(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).HostingUserAccount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn HostingProcessPid(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).HostingProcessPid)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Bind<'a, Param0: ::windows::core::IntoParam<'a, IFsrmPipelineModuleDefinition>, Param1: ::windows::core::IntoParam<'a, IFsrmPipelineModuleImplementation>>(&self, moduledefinition: Param0, moduleimplementation: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Bind)(::core::mem::transmute_copy(self), moduledefinition.into_param().abi(), moduleimplementation.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmPipelineModuleConnector> for ::windows::core::IUnknown {
    fn from(value: IFsrmPipelineModuleConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmPipelineModuleConnector> for ::windows::core::IUnknown {
    fn from(value: &IFsrmPipelineModuleConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmPipelineModuleConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmPipelineModuleConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmPipelineModuleConnector {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmPipelineModuleConnector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmPipelineModuleConnector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmPipelineModuleConnector {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmPipelineModuleConnector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmPipelineModuleConnector").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmPipelineModuleConnector {
    type Vtable = IFsrmPipelineModuleConnector_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc16014f3_9aa1_46b3_b0a7_ab146eb205f2);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPipelineModuleConnector_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ModuleImplementation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipelinemoduleimplementation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ModuleImplementation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ModuleName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ModuleName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub HostingUserAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccount: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HostingUserAccount: usize,
    pub HostingProcessPid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Bind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduledefinition: ::windows::core::RawPtr, moduleimplementation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Bind: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmPipelineModuleDefinition(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPipelineModuleDefinition {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDescription)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Commit)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModuleClsid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ModuleClsid)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetModuleClsid<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, moduleclsid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetModuleClsid)(::core::mem::transmute_copy(self), moduleclsid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Company(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Company)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCompany<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, company: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCompany)(::core::mem::transmute_copy(self), company.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Version(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Version)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVersion<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, version: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVersion)(::core::mem::transmute_copy(self), version.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn ModuleType(&self) -> ::windows::core::Result<FsrmPipelineModuleType> {
        let mut result__: FsrmPipelineModuleType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ModuleType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmPipelineModuleType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Enabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn NeedsFileContent(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).NeedsFileContent)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetNeedsFileContent(&self, needsfilecontent: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNeedsFileContent)(::core::mem::transmute_copy(self), ::core::mem::transmute(needsfilecontent)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Account(&self) -> ::windows::core::Result<FsrmAccountType> {
        let mut result__: FsrmAccountType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Account)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmAccountType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetAccount(&self, retrievalaccount: FsrmAccountType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAccount)(::core::mem::transmute_copy(self), ::core::mem::transmute(retrievalaccount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedExtensions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SupportedExtensions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSupportedExtensions(&self, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSupportedExtensions)(::core::mem::transmute_copy(self), ::core::mem::transmute(supportedextensions)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Parameters)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetParameters)(::core::mem::transmute_copy(self), ::core::mem::transmute(parameters)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmPipelineModuleDefinition> for ::windows::core::IUnknown {
    fn from(value: IFsrmPipelineModuleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmPipelineModuleDefinition> for ::windows::core::IUnknown {
    fn from(value: &IFsrmPipelineModuleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmPipelineModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmPipelineModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmPipelineModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmPipelineModuleDefinition> for IFsrmObject {
    fn from(value: IFsrmPipelineModuleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmPipelineModuleDefinition> for IFsrmObject {
    fn from(value: &IFsrmPipelineModuleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmPipelineModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &'a IFsrmPipelineModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmPipelineModuleDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmPipelineModuleDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmPipelineModuleDefinition {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmPipelineModuleDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmPipelineModuleDefinition").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmPipelineModuleDefinition {
    type Vtable = IFsrmPipelineModuleDefinition_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x515c1277_2c81_440e_8fcf_367921ed4f59);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPipelineModuleDefinition_Vtbl {
    pub base: IFsrmObject_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ModuleClsid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleclsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ModuleClsid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetModuleClsid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetModuleClsid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Company: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, company: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Company: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCompany: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, company: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCompany: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, version: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Version: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, version: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetVersion: usize,
    pub ModuleType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduletype: *mut FsrmPipelineModuleType) -> ::windows::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT,
    pub NeedsFileContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, needsfilecontent: *mut i16) -> ::windows::core::HRESULT,
    pub SetNeedsFileContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, needsfilecontent: i16) -> ::windows::core::HRESULT,
    pub Account: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retrievalaccount: *mut FsrmAccountType) -> ::windows::core::HRESULT,
    pub SetAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retrievalaccount: FsrmAccountType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedextensions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedExtensions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSupportedExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSupportedExtensions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Parameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parameters: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetParameters: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmPipelineModuleImplementation(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPipelineModuleImplementation {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnLoad<'a, Param0: ::windows::core::IntoParam<'a, IFsrmPipelineModuleDefinition>>(&self, moduledefinition: Param0) -> ::windows::core::Result<IFsrmPipelineModuleConnector> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OnLoad)(::core::mem::transmute_copy(self), moduledefinition.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmPipelineModuleConnector>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn OnUnload(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnUnload)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmPipelineModuleImplementation> for ::windows::core::IUnknown {
    fn from(value: IFsrmPipelineModuleImplementation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmPipelineModuleImplementation> for ::windows::core::IUnknown {
    fn from(value: &IFsrmPipelineModuleImplementation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmPipelineModuleImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmPipelineModuleImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmPipelineModuleImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmPipelineModuleImplementation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmPipelineModuleImplementation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmPipelineModuleImplementation {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmPipelineModuleImplementation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmPipelineModuleImplementation").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmPipelineModuleImplementation {
    type Vtable = IFsrmPipelineModuleImplementation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7907906_2b02_4cb5_84a9_fdf54613d6cd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPipelineModuleImplementation_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnLoad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduledefinition: ::windows::core::RawPtr, moduleconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnLoad: usize,
    pub OnUnload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmProperty(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmProperty {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Value)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Sources(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Sources)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn PropertyFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PropertyFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmProperty> for ::windows::core::IUnknown {
    fn from(value: IFsrmProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmProperty> for ::windows::core::IUnknown {
    fn from(value: &IFsrmProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmProperty {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmProperty {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmProperty").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmProperty {
    type Vtable = IFsrmProperty_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a73fee4_4102_4fcc_9ffb_38614f9ee768);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmProperty_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Value: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Sources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sources: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Sources: usize,
    pub PropertyFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmPropertyBag(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPropertyBag {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RelativePath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RelativePath)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VolumeName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).VolumeName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RelativeNamespaceRoot(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RelativeNamespaceRoot)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn VolumeIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).VolumeIndex)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn FileId(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FileId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ParentDirectoryId(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ParentDirectoryId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Size(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Size)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SizeAllocated(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SizeAllocated)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreationTime(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreationTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LastAccessTime(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LastAccessTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LastModificationTime(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LastModificationTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Attributes(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Attributes)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OwnerSid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OwnerSid)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FilePropertyNames(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FilePropertyNames)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Messages(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Messages)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn PropertyBagFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PropertyBagFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetFileProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<IFsrmProperty> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFileProperty)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0, value: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFileProperty)(::core::mem::transmute_copy(self), name.into_param().abi(), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddMessage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, message: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddMessage)(::core::mem::transmute_copy(self), message.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetFileStreamInterface(&self, accessmode: FsrmFileStreamingMode, interfacetype: FsrmFileStreamingInterfaceType) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFileStreamInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(accessmode), ::core::mem::transmute(interfacetype), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmPropertyBag> for ::windows::core::IUnknown {
    fn from(value: IFsrmPropertyBag) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmPropertyBag> for ::windows::core::IUnknown {
    fn from(value: &IFsrmPropertyBag) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmPropertyBag {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmPropertyBag {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmPropertyBag {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmPropertyBag {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmPropertyBag {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmPropertyBag {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmPropertyBag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmPropertyBag").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmPropertyBag {
    type Vtable = IFsrmPropertyBag_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x774589d1_d300_4f7a_9a24_f7b766800250);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyBag_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RelativePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RelativePath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub VolumeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, volumename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    VolumeName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RelativeNamespaceRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativenamespaceroot: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RelativeNamespaceRoot: usize,
    pub VolumeIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, volumeid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub FileId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fileid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    FileId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ParentDirectoryId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parentdirectoryid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ParentDirectoryId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Size: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SizeAllocated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sizeallocated: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SizeAllocated: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, creationtime: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreationTime: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LastAccessTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastaccesstime: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LastAccessTime: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LastModificationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastmodificationtime: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LastModificationTime: usize,
    pub Attributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributes: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OwnerSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ownersid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OwnerSid: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub FilePropertyNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepropertynames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FilePropertyNames: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Messages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messages: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Messages: usize,
    pub PropertyBagFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetFileProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fileproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetFileProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFileProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFileProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AddMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddMessage: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetFileStreamInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accessmode: FsrmFileStreamingMode, interfacetype: FsrmFileStreamingInterfaceType, pstreaminterface: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetFileStreamInterface: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmPropertyBag2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPropertyBag2 {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RelativePath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.RelativePath)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VolumeName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.VolumeName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RelativeNamespaceRoot(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.RelativeNamespaceRoot)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn VolumeIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.VolumeIndex)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn FileId(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.FileId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ParentDirectoryId(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ParentDirectoryId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Size(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Size)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SizeAllocated(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SizeAllocated)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreationTime(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreationTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LastAccessTime(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LastAccessTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LastModificationTime(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LastModificationTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Attributes(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Attributes)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OwnerSid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.OwnerSid)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FilePropertyNames(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.FilePropertyNames)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Messages(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Messages)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn PropertyBagFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.PropertyBagFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetFileProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<IFsrmProperty> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetFileProperty)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0, value: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetFileProperty)(::core::mem::transmute_copy(self), name.into_param().abi(), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddMessage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, message: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.AddMessage)(::core::mem::transmute_copy(self), message.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetFileStreamInterface(&self, accessmode: FsrmFileStreamingMode, interfacetype: FsrmFileStreamingInterfaceType) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetFileStreamInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(accessmode), ::core::mem::transmute(interfacetype), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetFieldValue(&self, field: FsrmPropertyBagField) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFieldValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(field), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetUntrustedInFileProperties(&self) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetUntrustedInFileProperties)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmPropertyBag2> for ::windows::core::IUnknown {
    fn from(value: IFsrmPropertyBag2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmPropertyBag2> for ::windows::core::IUnknown {
    fn from(value: &IFsrmPropertyBag2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmPropertyBag2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmPropertyBag2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmPropertyBag2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmPropertyBag2> for IFsrmPropertyBag {
    fn from(value: IFsrmPropertyBag2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmPropertyBag2> for IFsrmPropertyBag {
    fn from(value: &IFsrmPropertyBag2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmPropertyBag> for IFsrmPropertyBag2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmPropertyBag> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmPropertyBag> for &'a IFsrmPropertyBag2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmPropertyBag> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmPropertyBag2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmPropertyBag2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmPropertyBag2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmPropertyBag2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmPropertyBag2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmPropertyBag2 {
    type Vtable = IFsrmPropertyBag2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e46bdbd_2402_4fed_9c30_9266e6eb2cc9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyBag2_Vtbl {
    pub base: IFsrmPropertyBag_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetFieldValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, field: FsrmPropertyBagField, value: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetFieldValue: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetUntrustedInFileProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, props: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetUntrustedInFileProperties: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmPropertyCondition(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPropertyCondition {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Type(&self) -> ::windows::core::Result<FsrmPropertyConditionType> {
        let mut result__: FsrmPropertyConditionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Type)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmPropertyConditionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetType(&self, r#type: FsrmPropertyConditionType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Value)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, value: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetValue)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmPropertyCondition> for ::windows::core::IUnknown {
    fn from(value: IFsrmPropertyCondition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmPropertyCondition> for ::windows::core::IUnknown {
    fn from(value: &IFsrmPropertyCondition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmPropertyCondition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmPropertyCondition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmPropertyCondition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmPropertyCondition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmPropertyCondition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmPropertyCondition {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmPropertyCondition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmPropertyCondition").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmPropertyCondition {
    type Vtable = IFsrmPropertyCondition_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x326af66f_2ac0_4f68_bf8c_4759f054fa29);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyCondition_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut FsrmPropertyConditionType) -> ::windows::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: FsrmPropertyConditionType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Value: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetValue: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmPropertyDefinition(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPropertyDefinition {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDescription)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Commit)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Type(&self) -> ::windows::core::Result<FsrmPropertyDefinitionType> {
        let mut result__: FsrmPropertyDefinitionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Type)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmPropertyDefinitionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetType(&self, r#type: FsrmPropertyDefinitionType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PossibleValues(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PossibleValues)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPossibleValues(&self, possiblevalues: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPossibleValues)(::core::mem::transmute_copy(self), ::core::mem::transmute(possiblevalues)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ValueDescriptions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ValueDescriptions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetValueDescriptions(&self, valuedescriptions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetValueDescriptions)(::core::mem::transmute_copy(self), ::core::mem::transmute(valuedescriptions)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Parameters)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetParameters)(::core::mem::transmute_copy(self), ::core::mem::transmute(parameters)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmPropertyDefinition> for ::windows::core::IUnknown {
    fn from(value: IFsrmPropertyDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmPropertyDefinition> for ::windows::core::IUnknown {
    fn from(value: &IFsrmPropertyDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmPropertyDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmPropertyDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmPropertyDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmPropertyDefinition> for IFsrmObject {
    fn from(value: IFsrmPropertyDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmPropertyDefinition> for IFsrmObject {
    fn from(value: &IFsrmPropertyDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmPropertyDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &'a IFsrmPropertyDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmPropertyDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmPropertyDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmPropertyDefinition {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmPropertyDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmPropertyDefinition").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmPropertyDefinition {
    type Vtable = IFsrmPropertyDefinition_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xede0150f_e9a3_419c_877c_01fe5d24c5d3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyDefinition_Vtbl {
    pub base: IFsrmObject_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut FsrmPropertyDefinitionType) -> ::windows::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: FsrmPropertyDefinitionType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PossibleValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, possiblevalues: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PossibleValues: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetPossibleValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, possiblevalues: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetPossibleValues: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ValueDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, valuedescriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ValueDescriptions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetValueDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, valuedescriptions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetValueDescriptions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Parameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parameters: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetParameters: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmPropertyDefinition2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPropertyDefinition2 {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetDescription)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Commit)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Type(&self) -> ::windows::core::Result<FsrmPropertyDefinitionType> {
        let mut result__: FsrmPropertyDefinitionType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Type)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmPropertyDefinitionType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetType(&self, r#type: FsrmPropertyDefinitionType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PossibleValues(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.PossibleValues)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPossibleValues(&self, possiblevalues: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetPossibleValues)(::core::mem::transmute_copy(self), ::core::mem::transmute(possiblevalues)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ValueDescriptions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ValueDescriptions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetValueDescriptions(&self, valuedescriptions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetValueDescriptions)(::core::mem::transmute_copy(self), ::core::mem::transmute(valuedescriptions)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Parameters)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetParameters)(::core::mem::transmute_copy(self), ::core::mem::transmute(parameters)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn PropertyDefinitionFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PropertyDefinitionFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DisplayName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDisplayName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn AppliesTo(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AppliesTo)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ValueDefinitions(&self) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ValueDefinitions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmPropertyDefinition2> for ::windows::core::IUnknown {
    fn from(value: IFsrmPropertyDefinition2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmPropertyDefinition2> for ::windows::core::IUnknown {
    fn from(value: &IFsrmPropertyDefinition2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmPropertyDefinition2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmPropertyDefinition2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmPropertyDefinition2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmPropertyDefinition2> for IFsrmObject {
    fn from(value: IFsrmPropertyDefinition2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmPropertyDefinition2> for IFsrmObject {
    fn from(value: &IFsrmPropertyDefinition2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmPropertyDefinition2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &'a IFsrmPropertyDefinition2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmPropertyDefinition2> for IFsrmPropertyDefinition {
    fn from(value: IFsrmPropertyDefinition2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmPropertyDefinition2> for IFsrmPropertyDefinition {
    fn from(value: &IFsrmPropertyDefinition2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmPropertyDefinition> for IFsrmPropertyDefinition2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmPropertyDefinition> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmPropertyDefinition> for &'a IFsrmPropertyDefinition2 {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmPropertyDefinition> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmPropertyDefinition2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmPropertyDefinition2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmPropertyDefinition2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmPropertyDefinition2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmPropertyDefinition2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmPropertyDefinition2 {
    type Vtable = IFsrmPropertyDefinition2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47782152_d16c_4229_b4e1_0ddfe308b9f6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyDefinition2_Vtbl {
    pub base: IFsrmPropertyDefinition_Vtbl,
    pub PropertyDefinitionFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertydefinitionflags: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDisplayName: usize,
    pub AppliesTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appliesto: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ValueDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, valuedefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ValueDefinitions: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmPropertyDefinitionValue(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPropertyDefinitionValue {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DisplayName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UniqueID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).UniqueID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmPropertyDefinitionValue> for ::windows::core::IUnknown {
    fn from(value: IFsrmPropertyDefinitionValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmPropertyDefinitionValue> for ::windows::core::IUnknown {
    fn from(value: &IFsrmPropertyDefinitionValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmPropertyDefinitionValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmPropertyDefinitionValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmPropertyDefinitionValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmPropertyDefinitionValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmPropertyDefinitionValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmPropertyDefinitionValue {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmPropertyDefinitionValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmPropertyDefinitionValue").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmPropertyDefinitionValue {
    type Vtable = IFsrmPropertyDefinitionValue_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe946d148_bd67_4178_8e22_1c44925ed710);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyDefinitionValue_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UniqueID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uniqueid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UniqueID: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmQuota(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuota {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetDescription)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.Commit)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn QuotaLimit(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.QuotaLimit)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetQuotaLimit<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, quotalimit: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetQuotaLimit)(::core::mem::transmute_copy(self), quotalimit.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn QuotaFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.QuotaFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetQuotaFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(quotaflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Thresholds(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.Thresholds)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn AddThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.AddThreshold)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.DeleteThreshold)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.ModifyThreshold)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(newthreshold)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.CreateThresholdAction)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(actiontype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmAction>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.EnumThresholdActions)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Path)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserSid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.UserSid)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserAccount(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.UserAccount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SourceTemplateName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SourceTemplateName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn MatchesSourceTemplate(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.MatchesSourceTemplate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplyTemplate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, quotatemplatename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.ApplyTemplate)(::core::mem::transmute_copy(self), quotatemplatename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn QuotaUsed(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).QuotaUsed)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn QuotaPeakUsage(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).QuotaPeakUsage)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn QuotaPeakUsageTime(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).QuotaPeakUsageTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn ResetPeakUsage(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResetPeakUsage)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn RefreshUsageProperties(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RefreshUsageProperties)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmQuota> for ::windows::core::IUnknown {
    fn from(value: IFsrmQuota) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmQuota> for ::windows::core::IUnknown {
    fn from(value: &IFsrmQuota) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmQuota {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmQuota {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmQuota {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmQuota> for IFsrmObject {
    fn from(value: IFsrmQuota) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmQuota> for IFsrmObject {
    fn from(value: &IFsrmQuota) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmQuota {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &'a IFsrmQuota {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmQuota> for IFsrmQuotaBase {
    fn from(value: IFsrmQuota) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmQuota> for IFsrmQuotaBase {
    fn from(value: &IFsrmQuota) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaBase> for IFsrmQuota {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaBase> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaBase> for &'a IFsrmQuota {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmQuota> for IFsrmQuotaObject {
    fn from(value: IFsrmQuota) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmQuota> for IFsrmQuotaObject {
    fn from(value: &IFsrmQuota) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaObject> for IFsrmQuota {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaObject> for &'a IFsrmQuota {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmQuota {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmQuota {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmQuota {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmQuota {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmQuota").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmQuota {
    type Vtable = IFsrmQuota_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x377f739d_9647_4b8e_97d2_5ffce6d759cd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuota_Vtbl {
    pub base: IFsrmQuotaObject_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub QuotaUsed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, used: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    QuotaUsed: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub QuotaPeakUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peakusage: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    QuotaPeakUsage: usize,
    pub QuotaPeakUsageTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peakusagedatetime: *mut f64) -> ::windows::core::HRESULT,
    pub ResetPeakUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RefreshUsageProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmQuotaBase(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuotaBase {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDescription)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Commit)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn QuotaLimit(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).QuotaLimit)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetQuotaLimit<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, quotalimit: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetQuotaLimit)(::core::mem::transmute_copy(self), quotalimit.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn QuotaFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).QuotaFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetQuotaFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(quotaflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Thresholds(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Thresholds)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn AddThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddThreshold)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteThreshold)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ModifyThreshold)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(newthreshold)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateThresholdAction)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(actiontype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmAction>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumThresholdActions)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmQuotaBase> for ::windows::core::IUnknown {
    fn from(value: IFsrmQuotaBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmQuotaBase> for ::windows::core::IUnknown {
    fn from(value: &IFsrmQuotaBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmQuotaBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmQuotaBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmQuotaBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmQuotaBase> for IFsrmObject {
    fn from(value: IFsrmQuotaBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmQuotaBase> for IFsrmObject {
    fn from(value: &IFsrmQuotaBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmQuotaBase {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &'a IFsrmQuotaBase {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmQuotaBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmQuotaBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmQuotaBase {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmQuotaBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmQuotaBase").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmQuotaBase {
    type Vtable = IFsrmQuotaBase_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1568a795_3924_4118_b74b_68d8f0fa5daf);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaBase_Vtbl {
    pub base: IFsrmObject_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub QuotaLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotalimit: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    QuotaLimit: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetQuotaLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotalimit: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetQuotaLimit: usize,
    pub QuotaFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotaflags: *mut i32) -> ::windows::core::HRESULT,
    pub SetQuotaFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotaflags: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Thresholds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, thresholds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Thresholds: usize,
    pub AddThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32) -> ::windows::core::HRESULT,
    pub DeleteThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32) -> ::windows::core::HRESULT,
    pub ModifyThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32, newthreshold: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateThresholdAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32, actiontype: FsrmActionType, action: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateThresholdAction: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumThresholdActions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32, actions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumThresholdActions: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmQuotaManager(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuotaManager {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariables(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ActionVariables)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariableDescriptions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ActionVariableDescriptions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateQuota<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<IFsrmQuota> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateQuota)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmQuota>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateAutoApplyQuota<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, quotatemplatename: Param0, path: Param1) -> ::windows::core::Result<IFsrmAutoApplyQuota> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateAutoApplyQuota)(::core::mem::transmute_copy(self), quotatemplatename.into_param().abi(), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmAutoApplyQuota>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetQuota<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<IFsrmQuota> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetQuota)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmQuota>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetAutoApplyQuota<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<IFsrmAutoApplyQuota> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAutoApplyQuota)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmAutoApplyQuota>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetRestrictiveQuota<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<IFsrmQuota> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRestrictiveQuota)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmQuota>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn EnumQuotas<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumQuotas)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn EnumAutoApplyQuotas<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumAutoApplyQuotas)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn EnumEffectiveQuotas<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumEffectiveQuotas)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Scan<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strpath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Scan)(::core::mem::transmute_copy(self), strpath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateQuotaCollection(&self) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateQuotaCollection)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmQuotaManager> for ::windows::core::IUnknown {
    fn from(value: IFsrmQuotaManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmQuotaManager> for ::windows::core::IUnknown {
    fn from(value: &IFsrmQuotaManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmQuotaManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmQuotaManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmQuotaManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmQuotaManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmQuotaManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmQuotaManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmQuotaManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmQuotaManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmQuotaManager {
    type Vtable = IFsrmQuotaManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bb68c7d_19d8_4ffb_809e_be4fc1734014);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaManager_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ActionVariables: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActionVariables: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ActionVariableDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActionVariableDescriptions: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateQuota: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateAutoApplyQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotatemplatename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateAutoApplyQuota: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetQuota: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetAutoApplyQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetAutoApplyQuota: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetRestrictiveQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetRestrictiveQuota: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub EnumQuotas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, quotas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    EnumQuotas: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub EnumAutoApplyQuotas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, quotas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    EnumAutoApplyQuotas: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub EnumEffectiveQuotas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, quotas: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    EnumEffectiveQuotas: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Scan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Scan: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateQuotaCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateQuotaCollection: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmQuotaManagerEx(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuotaManagerEx {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariables(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ActionVariables)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariableDescriptions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ActionVariableDescriptions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateQuota<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<IFsrmQuota> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateQuota)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmQuota>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateAutoApplyQuota<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, quotatemplatename: Param0, path: Param1) -> ::windows::core::Result<IFsrmAutoApplyQuota> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateAutoApplyQuota)(::core::mem::transmute_copy(self), quotatemplatename.into_param().abi(), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmAutoApplyQuota>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetQuota<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<IFsrmQuota> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetQuota)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmQuota>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetAutoApplyQuota<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<IFsrmAutoApplyQuota> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetAutoApplyQuota)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmAutoApplyQuota>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetRestrictiveQuota<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<IFsrmQuota> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetRestrictiveQuota)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmQuota>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn EnumQuotas<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.EnumQuotas)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn EnumAutoApplyQuotas<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.EnumAutoApplyQuotas)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn EnumEffectiveQuotas<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.EnumEffectiveQuotas)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Scan<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strpath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Scan)(::core::mem::transmute_copy(self), strpath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateQuotaCollection(&self) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateQuotaCollection)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsAffectedByQuota<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsAffectedByQuota)(::core::mem::transmute_copy(self), path.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmQuotaManagerEx> for ::windows::core::IUnknown {
    fn from(value: IFsrmQuotaManagerEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmQuotaManagerEx> for ::windows::core::IUnknown {
    fn from(value: &IFsrmQuotaManagerEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmQuotaManagerEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmQuotaManagerEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmQuotaManagerEx {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmQuotaManagerEx> for IFsrmQuotaManager {
    fn from(value: IFsrmQuotaManagerEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmQuotaManagerEx> for IFsrmQuotaManager {
    fn from(value: &IFsrmQuotaManagerEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaManager> for IFsrmQuotaManagerEx {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaManager> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaManager> for &'a IFsrmQuotaManagerEx {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaManager> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmQuotaManagerEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmQuotaManagerEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmQuotaManagerEx {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmQuotaManagerEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmQuotaManagerEx").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmQuotaManagerEx {
    type Vtable = IFsrmQuotaManagerEx_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4846cb01_d430_494f_abb4_b1054999fb09);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaManagerEx_Vtbl {
    pub base: IFsrmQuotaManager_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsAffectedByQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, affected: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsAffectedByQuota: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmQuotaObject(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuotaObject {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetDescription)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Commit)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn QuotaLimit(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.QuotaLimit)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetQuotaLimit<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, quotalimit: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetQuotaLimit)(::core::mem::transmute_copy(self), quotalimit.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn QuotaFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.QuotaFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetQuotaFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(quotaflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Thresholds(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Thresholds)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn AddThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.AddThreshold)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.DeleteThreshold)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.ModifyThreshold)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(newthreshold)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateThresholdAction)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(actiontype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmAction>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.EnumThresholdActions)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Path)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserSid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).UserSid)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserAccount(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).UserAccount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SourceTemplateName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SourceTemplateName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn MatchesSourceTemplate(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MatchesSourceTemplate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplyTemplate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, quotatemplatename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ApplyTemplate)(::core::mem::transmute_copy(self), quotatemplatename.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmQuotaObject> for ::windows::core::IUnknown {
    fn from(value: IFsrmQuotaObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmQuotaObject> for ::windows::core::IUnknown {
    fn from(value: &IFsrmQuotaObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmQuotaObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmQuotaObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmQuotaObject {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmQuotaObject> for IFsrmObject {
    fn from(value: IFsrmQuotaObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmQuotaObject> for IFsrmObject {
    fn from(value: &IFsrmQuotaObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmQuotaObject {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &'a IFsrmQuotaObject {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmQuotaObject> for IFsrmQuotaBase {
    fn from(value: IFsrmQuotaObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmQuotaObject> for IFsrmQuotaBase {
    fn from(value: &IFsrmQuotaObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaBase> for IFsrmQuotaObject {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaBase> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaBase> for &'a IFsrmQuotaObject {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmQuotaObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmQuotaObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmQuotaObject {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmQuotaObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmQuotaObject").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmQuotaObject {
    type Vtable = IFsrmQuotaObject_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42dc3511_61d5_48ae_b6dc_59fc00c0a8d6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaObject_Vtbl {
    pub base: IFsrmQuotaBase_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Path: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UserSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserSid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UserAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccount: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserAccount: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SourceTemplateName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotatemplatename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SourceTemplateName: usize,
    pub MatchesSourceTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matches: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ApplyTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotatemplatename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ApplyTemplate: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmQuotaTemplate(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuotaTemplate {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetDescription)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Commit)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn QuotaLimit(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.QuotaLimit)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetQuotaLimit<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, quotalimit: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetQuotaLimit)(::core::mem::transmute_copy(self), quotalimit.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn QuotaFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.QuotaFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetQuotaFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(quotaflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Thresholds(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Thresholds)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn AddThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.AddThreshold)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.DeleteThreshold)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.ModifyThreshold)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(newthreshold)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateThresholdAction)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(actiontype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmAction>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.EnumThresholdActions)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTemplate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, quotatemplatename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CopyTemplate)(::core::mem::transmute_copy(self), quotatemplatename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows::core::Result<IFsrmDerivedObjectsResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CommitAndUpdateDerived)(::core::mem::transmute_copy(self), ::core::mem::transmute(commitoptions), ::core::mem::transmute(applyoptions), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmDerivedObjectsResult>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmQuotaTemplate> for ::windows::core::IUnknown {
    fn from(value: IFsrmQuotaTemplate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmQuotaTemplate> for ::windows::core::IUnknown {
    fn from(value: &IFsrmQuotaTemplate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmQuotaTemplate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmQuotaTemplate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmQuotaTemplate {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmQuotaTemplate> for IFsrmObject {
    fn from(value: IFsrmQuotaTemplate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmQuotaTemplate> for IFsrmObject {
    fn from(value: &IFsrmQuotaTemplate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmQuotaTemplate {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &'a IFsrmQuotaTemplate {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmQuotaTemplate> for IFsrmQuotaBase {
    fn from(value: IFsrmQuotaTemplate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmQuotaTemplate> for IFsrmQuotaBase {
    fn from(value: &IFsrmQuotaTemplate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaBase> for IFsrmQuotaTemplate {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaBase> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaBase> for &'a IFsrmQuotaTemplate {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmQuotaTemplate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmQuotaTemplate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmQuotaTemplate {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmQuotaTemplate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmQuotaTemplate").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmQuotaTemplate {
    type Vtable = IFsrmQuotaTemplate_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2efab31_295e_46bb_b976_e86d58b52e8b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaTemplate_Vtbl {
    pub base: IFsrmQuotaBase_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CopyTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotatemplatename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CopyTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CommitAndUpdateDerived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommitAndUpdateDerived: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmQuotaTemplateImported(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuotaTemplateImported {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetDescription)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.Commit)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn QuotaLimit(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.QuotaLimit)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetQuotaLimit<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, quotalimit: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetQuotaLimit)(::core::mem::transmute_copy(self), quotalimit.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn QuotaFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.QuotaFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetQuotaFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(quotaflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Thresholds(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.Thresholds)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn AddThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.AddThreshold)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.DeleteThreshold)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.ModifyThreshold)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(newthreshold)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.CreateThresholdAction)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(actiontype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmAction>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.EnumThresholdActions)(::core::mem::transmute_copy(self), ::core::mem::transmute(threshold), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTemplate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, quotatemplatename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CopyTemplate)(::core::mem::transmute_copy(self), quotatemplatename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows::core::Result<IFsrmDerivedObjectsResult> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CommitAndUpdateDerived)(::core::mem::transmute_copy(self), ::core::mem::transmute(commitoptions), ::core::mem::transmute(applyoptions), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmDerivedObjectsResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn OverwriteOnCommit(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OverwriteOnCommit)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetOverwriteOnCommit(&self, overwrite: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOverwriteOnCommit)(::core::mem::transmute_copy(self), ::core::mem::transmute(overwrite)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmQuotaTemplateImported> for ::windows::core::IUnknown {
    fn from(value: IFsrmQuotaTemplateImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmQuotaTemplateImported> for ::windows::core::IUnknown {
    fn from(value: &IFsrmQuotaTemplateImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmQuotaTemplateImported> for IFsrmObject {
    fn from(value: IFsrmQuotaTemplateImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmQuotaTemplateImported> for IFsrmObject {
    fn from(value: &IFsrmQuotaTemplateImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &'a IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmQuotaTemplateImported> for IFsrmQuotaBase {
    fn from(value: IFsrmQuotaTemplateImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmQuotaTemplateImported> for IFsrmQuotaBase {
    fn from(value: &IFsrmQuotaTemplateImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaBase> for IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaBase> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaBase> for &'a IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmQuotaTemplateImported> for IFsrmQuotaTemplate {
    fn from(value: IFsrmQuotaTemplateImported) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmQuotaTemplateImported> for IFsrmQuotaTemplate {
    fn from(value: &IFsrmQuotaTemplateImported) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaTemplate> for IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaTemplate> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmQuotaTemplate> for &'a IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmQuotaTemplate> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmQuotaTemplateImported {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmQuotaTemplateImported {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmQuotaTemplateImported {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmQuotaTemplateImported {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmQuotaTemplateImported").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmQuotaTemplateImported {
    type Vtable = IFsrmQuotaTemplateImported_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a2bf113_a329_44cc_809a_5c00fce8da40);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaTemplateImported_Vtbl {
    pub base: IFsrmQuotaTemplate_Vtbl,
    pub OverwriteOnCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: *mut i16) -> ::windows::core::HRESULT,
    pub SetOverwriteOnCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmQuotaTemplateManager(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuotaTemplateManager {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateTemplate(&self) -> ::windows::core::Result<IFsrmQuotaTemplate> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateTemplate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmQuotaTemplate>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetTemplate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<IFsrmQuotaTemplate> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTemplate)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmQuotaTemplate>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumTemplates(&self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumTemplates)(::core::mem::transmute_copy(self), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ExportTemplates(&self, quotatemplatenamesarray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ExportTemplates)(::core::mem::transmute_copy(self), ::core::mem::transmute(quotatemplatenamesarray), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ImportTemplates<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, serializedquotatemplates: Param0, quotatemplatenamesarray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ImportTemplates)(::core::mem::transmute_copy(self), serializedquotatemplates.into_param().abi(), ::core::mem::transmute(quotatemplatenamesarray), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCommittableCollection>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmQuotaTemplateManager> for ::windows::core::IUnknown {
    fn from(value: IFsrmQuotaTemplateManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmQuotaTemplateManager> for ::windows::core::IUnknown {
    fn from(value: &IFsrmQuotaTemplateManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmQuotaTemplateManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmQuotaTemplateManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmQuotaTemplateManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmQuotaTemplateManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmQuotaTemplateManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmQuotaTemplateManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmQuotaTemplateManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmQuotaTemplateManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmQuotaTemplateManager {
    type Vtable = IFsrmQuotaTemplateManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4173ac41_172d_4d52_963c_fdc7e415f717);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaTemplateManager_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotatemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateTemplate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quotatemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, quotatemplates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumTemplates: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ExportTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotatemplatenamesarray: *const super::super::System::Com::VARIANT, serializedquotatemplates: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ExportTemplates: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ImportTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serializedquotatemplates: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, quotatemplatenamesarray: *const super::super::System::Com::VARIANT, quotatemplates: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ImportTemplates: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmReport(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmReport {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Type(&self) -> ::windows::core::Result<FsrmReportType> {
        let mut result__: FsrmReportType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Type)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmReportType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDescription)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastGeneratedFileNamePrefix(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LastGeneratedFileNamePrefix)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetFilter(&self, filter: FsrmReportFilter) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFilter)(::core::mem::transmute_copy(self), ::core::mem::transmute(filter), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetFilter<'a, Param1: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, filter: FsrmReportFilter, filtervalue: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFilter)(::core::mem::transmute_copy(self), ::core::mem::transmute(filter), filtervalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmReport> for ::windows::core::IUnknown {
    fn from(value: IFsrmReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmReport> for ::windows::core::IUnknown {
    fn from(value: &IFsrmReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmReport {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmReport {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmReport").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmReport {
    type Vtable = IFsrmReport_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8cc81d9_46b8_4fa4_bfa5_4aa9dec9b638);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmReport_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: *mut FsrmReportType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LastGeneratedFileNamePrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prefix: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LastGeneratedFileNamePrefix: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filter: FsrmReportFilter, filtervalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetFilter: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filter: FsrmReportFilter, filtervalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetFilter: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmReportJob(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmReportJob {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDescription)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Commit)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Task(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Task)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTask<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, taskname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTask)(::core::mem::transmute_copy(self), taskname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NamespaceRoots(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).NamespaceRoots)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetNamespaceRoots(&self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNamespaceRoots)(::core::mem::transmute_copy(self), ::core::mem::transmute(namespaceroots)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Formats(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Formats)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFormats(&self, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFormats)(::core::mem::transmute_copy(self), ::core::mem::transmute(formats)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailTo(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MailTo)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailto: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMailTo)(::core::mem::transmute_copy(self), mailto.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn RunningStatus(&self) -> ::windows::core::Result<FsrmReportRunningStatus> {
        let mut result__: FsrmReportRunningStatus = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RunningStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmReportRunningStatus>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn LastRun(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LastRun)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastError(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LastError)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastGeneratedInDirectory(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LastGeneratedInDirectory)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumReports(&self) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumReports)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateReport(&self, reporttype: FsrmReportType) -> ::windows::core::Result<IFsrmReport> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateReport)(::core::mem::transmute_copy(self), ::core::mem::transmute(reporttype), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmReport>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Run(&self, context: FsrmReportGenerationContext) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Run)(::core::mem::transmute_copy(self), ::core::mem::transmute(context)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).WaitForCompletion)(::core::mem::transmute_copy(self), ::core::mem::transmute(waitseconds), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmReportJob> for ::windows::core::IUnknown {
    fn from(value: IFsrmReportJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmReportJob> for ::windows::core::IUnknown {
    fn from(value: &IFsrmReportJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmReportJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmReportJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmReportJob {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmReportJob> for IFsrmObject {
    fn from(value: IFsrmReportJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmReportJob> for IFsrmObject {
    fn from(value: &IFsrmReportJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmReportJob {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &'a IFsrmReportJob {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmReportJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmReportJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmReportJob {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmReportJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmReportJob").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmReportJob {
    type Vtable = IFsrmReportJob_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38e87280_715c_4c7d_a280_ea1651a19fef);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmReportJob_Vtbl {
    pub base: IFsrmObject_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Task: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Task: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTask: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NamespaceRoots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NamespaceRoots: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetNamespaceRoots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetNamespaceRoots: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Formats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Formats: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetFormats: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MailTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MailTo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMailTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMailTo: usize,
    pub RunningStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runningstatus: *mut FsrmReportRunningStatus) -> ::windows::core::HRESULT,
    pub LastRun: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastrun: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LastError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lasterror: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LastError: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LastGeneratedInDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LastGeneratedInDirectory: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumReports: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reports: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumReports: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, report: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateReport: usize,
    pub Run: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext) -> ::windows::core::HRESULT,
    pub WaitForCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut i16) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmReportManager(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmReportManager {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumReportJobs(&self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumReportJobs)(::core::mem::transmute_copy(self), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateReportJob(&self) -> ::windows::core::Result<IFsrmReportJob> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateReportJob)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmReportJob>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetReportJob<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, taskname: Param0) -> ::windows::core::Result<IFsrmReportJob> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetReportJob)(::core::mem::transmute_copy(self), taskname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmReportJob>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOutputDirectory(&self, context: FsrmReportGenerationContext) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOutputDirectory)(::core::mem::transmute_copy(self), ::core::mem::transmute(context), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOutputDirectory<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, context: FsrmReportGenerationContext, path: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOutputDirectory)(::core::mem::transmute_copy(self), ::core::mem::transmute(context), path.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn IsFilterValidForReportType(&self, reporttype: FsrmReportType, filter: FsrmReportFilter) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsFilterValidForReportType)(::core::mem::transmute_copy(self), ::core::mem::transmute(reporttype), ::core::mem::transmute(filter), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetDefaultFilter(&self, reporttype: FsrmReportType, filter: FsrmReportFilter) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDefaultFilter)(::core::mem::transmute_copy(self), ::core::mem::transmute(reporttype), ::core::mem::transmute(filter), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetDefaultFilter<'a, Param2: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDefaultFilter)(::core::mem::transmute_copy(self), ::core::mem::transmute(reporttype), ::core::mem::transmute(filter), filtervalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetReportSizeLimit(&self, limit: FsrmReportLimit) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetReportSizeLimit)(::core::mem::transmute_copy(self), ::core::mem::transmute(limit), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetReportSizeLimit<'a, Param1: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, limit: FsrmReportLimit, limitvalue: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetReportSizeLimit)(::core::mem::transmute_copy(self), ::core::mem::transmute(limit), limitvalue.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmReportManager> for ::windows::core::IUnknown {
    fn from(value: IFsrmReportManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmReportManager> for ::windows::core::IUnknown {
    fn from(value: &IFsrmReportManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmReportManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmReportManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmReportManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmReportManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmReportManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmReportManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmReportManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmReportManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmReportManager {
    type Vtable = IFsrmReportManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27b899fe_6ffa_4481_a184_d3daade8a02b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmReportManager_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumReportJobs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, reportjobs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumReportJobs: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateReportJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateReportJob: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetReportJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, reportjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetReportJob: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetOutputDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetOutputDirectory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOutputDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOutputDirectory: usize,
    pub IsFilterValidForReportType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, filter: FsrmReportFilter, valid: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetDefaultFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetDefaultFilter: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetDefaultFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetDefaultFilter: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetReportSizeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, limit: FsrmReportLimit, limitvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetReportSizeLimit: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetReportSizeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, limit: FsrmReportLimit, limitvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetReportSizeLimit: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmReportScheduler(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmReportScheduler {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn VerifyNamespaces(&self, namespacessafearray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).VerifyNamespaces)(::core::mem::transmute_copy(self), ::core::mem::transmute(namespacessafearray)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateScheduleTask<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, taskname: Param0, namespacessafearray: *const super::super::System::Com::VARIANT, serializedtask: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateScheduleTask)(::core::mem::transmute_copy(self), taskname.into_param().abi(), ::core::mem::transmute(namespacessafearray), serializedtask.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ModifyScheduleTask<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, taskname: Param0, namespacessafearray: *const super::super::System::Com::VARIANT, serializedtask: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ModifyScheduleTask)(::core::mem::transmute_copy(self), taskname.into_param().abi(), ::core::mem::transmute(namespacessafearray), serializedtask.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteScheduleTask<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, taskname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteScheduleTask)(::core::mem::transmute_copy(self), taskname.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmReportScheduler> for ::windows::core::IUnknown {
    fn from(value: IFsrmReportScheduler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmReportScheduler> for ::windows::core::IUnknown {
    fn from(value: &IFsrmReportScheduler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmReportScheduler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmReportScheduler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmReportScheduler {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmReportScheduler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmReportScheduler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmReportScheduler {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmReportScheduler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmReportScheduler").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmReportScheduler {
    type Vtable = IFsrmReportScheduler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6879caf9_6617_4484_8719_71c3d8645f94);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmReportScheduler_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub VerifyNamespaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespacessafearray: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    VerifyNamespaces: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateScheduleTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, namespacessafearray: *const super::super::System::Com::VARIANT, serializedtask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateScheduleTask: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ModifyScheduleTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, namespacessafearray: *const super::super::System::Com::VARIANT, serializedtask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ModifyScheduleTask: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteScheduleTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteScheduleTask: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmRule(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmRule {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDescription)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Commit)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn RuleType(&self) -> ::windows::core::Result<FsrmRuleType> {
        let mut result__: FsrmRuleType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RuleType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmRuleType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModuleDefinitionName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ModuleDefinitionName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetModuleDefinitionName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, moduledefinitionname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetModuleDefinitionName)(::core::mem::transmute_copy(self), moduledefinitionname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NamespaceRoots(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).NamespaceRoots)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetNamespaceRoots(&self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNamespaceRoots)(::core::mem::transmute_copy(self), ::core::mem::transmute(namespaceroots)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn RuleFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RuleFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetRuleFlags(&self, ruleflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRuleFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(ruleflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Parameters)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetParameters)(::core::mem::transmute_copy(self), ::core::mem::transmute(parameters)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LastModified(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LastModified)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmRule> for ::windows::core::IUnknown {
    fn from(value: IFsrmRule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmRule> for ::windows::core::IUnknown {
    fn from(value: &IFsrmRule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmRule {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmRule {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmRule {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmRule> for IFsrmObject {
    fn from(value: IFsrmRule) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmRule> for IFsrmObject {
    fn from(value: &IFsrmRule) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmRule {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &'a IFsrmRule {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmRule {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmRule {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmRule {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmRule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmRule").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmRule {
    type Vtable = IFsrmRule_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb0df960_16f5_4495_9079_3f9360d831df);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmRule_Vtbl {
    pub base: IFsrmObject_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    pub RuleType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ruletype: *mut FsrmRuleType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ModuleDefinitionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduledefinitionname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ModuleDefinitionName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetModuleDefinitionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetModuleDefinitionName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NamespaceRoots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NamespaceRoots: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetNamespaceRoots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetNamespaceRoots: usize,
    pub RuleFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ruleflags: *mut i32) -> ::windows::core::HRESULT,
    pub SetRuleFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ruleflags: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Parameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parameters: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetParameters: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LastModified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastmodified: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LastModified: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmSetting(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmSetting {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SmtpServer(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SmtpServer)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSmtpServer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, smtpserver: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSmtpServer)(::core::mem::transmute_copy(self), smtpserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailFrom(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MailFrom)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailFrom<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailfrom: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMailFrom)(::core::mem::transmute_copy(self), mailfrom.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AdminEmail(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AdminEmail)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAdminEmail<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, adminemail: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAdminEmail)(::core::mem::transmute_copy(self), adminemail.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn DisableCommandLine(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DisableCommandLine)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetDisableCommandLine(&self, disablecommandline: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDisableCommandLine)(::core::mem::transmute_copy(self), ::core::mem::transmute(disablecommandline)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn EnableScreeningAudit(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnableScreeningAudit)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetEnableScreeningAudit(&self, enablescreeningaudit: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnableScreeningAudit)(::core::mem::transmute_copy(self), ::core::mem::transmute(enablescreeningaudit)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EmailTest<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailto: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EmailTest)(::core::mem::transmute_copy(self), mailto.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetActionRunLimitInterval(&self, actiontype: FsrmActionType, delaytimeminutes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetActionRunLimitInterval)(::core::mem::transmute_copy(self), ::core::mem::transmute(actiontype), ::core::mem::transmute(delaytimeminutes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn GetActionRunLimitInterval(&self, actiontype: FsrmActionType) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetActionRunLimitInterval)(::core::mem::transmute_copy(self), ::core::mem::transmute(actiontype), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmSetting> for ::windows::core::IUnknown {
    fn from(value: IFsrmSetting) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmSetting> for ::windows::core::IUnknown {
    fn from(value: &IFsrmSetting) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmSetting {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmSetting {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmSetting {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmSetting {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmSetting {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmSetting {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmSetting {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmSetting").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmSetting {
    type Vtable = IFsrmSetting_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf411d4fd_14be_4260_8c40_03b7c95e608a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmSetting_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SmtpServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smtpserver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SmtpServer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSmtpServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smtpserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSmtpServer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MailFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailfrom: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MailFrom: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMailFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailfrom: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMailFrom: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AdminEmail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adminemail: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AdminEmail: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAdminEmail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adminemail: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAdminEmail: usize,
    pub DisableCommandLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disablecommandline: *mut i16) -> ::windows::core::HRESULT,
    pub SetDisableCommandLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disablecommandline: i16) -> ::windows::core::HRESULT,
    pub EnableScreeningAudit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enablescreeningaudit: *mut i16) -> ::windows::core::HRESULT,
    pub SetEnableScreeningAudit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enablescreeningaudit: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EmailTest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EmailTest: usize,
    pub SetActionRunLimitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actiontype: FsrmActionType, delaytimeminutes: i32) -> ::windows::core::HRESULT,
    pub GetActionRunLimitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actiontype: FsrmActionType, delaytimeminutes: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmStorageModuleDefinition(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmStorageModuleDefinition {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.Id)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetDescription)(::core::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Delete)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Commit)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModuleClsid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ModuleClsid)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetModuleClsid<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, moduleclsid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetModuleClsid)(::core::mem::transmute_copy(self), moduleclsid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetName)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Company(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Company)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCompany<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, company: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetCompany)(::core::mem::transmute_copy(self), company.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Version(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Version)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVersion<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, version: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetVersion)(::core::mem::transmute_copy(self), version.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn ModuleType(&self) -> ::windows::core::Result<FsrmPipelineModuleType> {
        let mut result__: FsrmPipelineModuleType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.ModuleType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmPipelineModuleType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Enabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(enabled)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn NeedsFileContent(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.NeedsFileContent)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetNeedsFileContent(&self, needsfilecontent: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetNeedsFileContent)(::core::mem::transmute_copy(self), ::core::mem::transmute(needsfilecontent)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Account(&self) -> ::windows::core::Result<FsrmAccountType> {
        let mut result__: FsrmAccountType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Account)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmAccountType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetAccount(&self, retrievalaccount: FsrmAccountType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetAccount)(::core::mem::transmute_copy(self), ::core::mem::transmute(retrievalaccount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedExtensions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.SupportedExtensions)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSupportedExtensions(&self, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetSupportedExtensions)(::core::mem::transmute_copy(self), ::core::mem::transmute(supportedextensions)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: *mut super::super::System::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Parameters)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetParameters)(::core::mem::transmute_copy(self), ::core::mem::transmute(parameters)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn Capabilities(&self) -> ::windows::core::Result<FsrmStorageModuleCaps> {
        let mut result__: FsrmStorageModuleCaps = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Capabilities)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmStorageModuleCaps>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetCapabilities(&self, capabilities: FsrmStorageModuleCaps) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCapabilities)(::core::mem::transmute_copy(self), ::core::mem::transmute(capabilities)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn StorageType(&self) -> ::windows::core::Result<FsrmStorageModuleType> {
        let mut result__: FsrmStorageModuleType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).StorageType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<FsrmStorageModuleType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetStorageType(&self, storagetype: FsrmStorageModuleType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStorageType)(::core::mem::transmute_copy(self), ::core::mem::transmute(storagetype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn UpdatesFileContent(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).UpdatesFileContent)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn SetUpdatesFileContent(&self, updatesfilecontent: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUpdatesFileContent)(::core::mem::transmute_copy(self), ::core::mem::transmute(updatesfilecontent)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmStorageModuleDefinition> for ::windows::core::IUnknown {
    fn from(value: IFsrmStorageModuleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmStorageModuleDefinition> for ::windows::core::IUnknown {
    fn from(value: &IFsrmStorageModuleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmStorageModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmStorageModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmStorageModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmStorageModuleDefinition> for IFsrmObject {
    fn from(value: IFsrmStorageModuleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmStorageModuleDefinition> for IFsrmObject {
    fn from(value: &IFsrmStorageModuleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for IFsrmStorageModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmObject> for &'a IFsrmStorageModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmStorageModuleDefinition> for IFsrmPipelineModuleDefinition {
    fn from(value: IFsrmStorageModuleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmStorageModuleDefinition> for IFsrmPipelineModuleDefinition {
    fn from(value: &IFsrmStorageModuleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmPipelineModuleDefinition> for IFsrmStorageModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmPipelineModuleDefinition> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmPipelineModuleDefinition> for &'a IFsrmStorageModuleDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmPipelineModuleDefinition> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmStorageModuleDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmStorageModuleDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmStorageModuleDefinition {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmStorageModuleDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmStorageModuleDefinition").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmStorageModuleDefinition {
    type Vtable = IFsrmStorageModuleDefinition_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15a81350_497d_4aba_80e9_d4dbcc5521fe);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmStorageModuleDefinition_Vtbl {
    pub base: IFsrmPipelineModuleDefinition_Vtbl,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capabilities: *mut FsrmStorageModuleCaps) -> ::windows::core::HRESULT,
    pub SetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capabilities: FsrmStorageModuleCaps) -> ::windows::core::HRESULT,
    pub StorageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storagetype: *mut FsrmStorageModuleType) -> ::windows::core::HRESULT,
    pub SetStorageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storagetype: FsrmStorageModuleType) -> ::windows::core::HRESULT,
    pub UpdatesFileContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updatesfilecontent: *mut i16) -> ::windows::core::HRESULT,
    pub SetUpdatesFileContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updatesfilecontent: i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmStorageModuleImplementation(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmStorageModuleImplementation {
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnLoad<'a, Param0: ::windows::core::IntoParam<'a, IFsrmPipelineModuleDefinition>>(&self, moduledefinition: Param0) -> ::windows::core::Result<IFsrmPipelineModuleConnector> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.OnLoad)(::core::mem::transmute_copy(self), moduledefinition.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IFsrmPipelineModuleConnector>(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
    pub unsafe fn OnUnload(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OnUnload)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UseDefinitions<'a, Param0: ::windows::core::IntoParam<'a, IFsrmCollection>>(&self, propertydefinitions: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UseDefinitions)(::core::mem::transmute_copy(self), propertydefinitions.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoadProperties<'a, Param0: ::windows::core::IntoParam<'a, IFsrmPropertyBag>>(&self, propertybag: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LoadProperties)(::core::mem::transmute_copy(self), propertybag.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SaveProperties<'a, Param0: ::windows::core::IntoParam<'a, IFsrmPropertyBag>>(&self, propertybag: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SaveProperties)(::core::mem::transmute_copy(self), propertybag.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmStorageModuleImplementation> for ::windows::core::IUnknown {
    fn from(value: IFsrmStorageModuleImplementation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmStorageModuleImplementation> for ::windows::core::IUnknown {
    fn from(value: &IFsrmStorageModuleImplementation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFsrmStorageModuleImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFsrmStorageModuleImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IFsrmStorageModuleImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsrmStorageModuleImplementation> for IFsrmPipelineModuleImplementation {
    fn from(value: IFsrmStorageModuleImplementation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsrmStorageModuleImplementation> for IFsrmPipelineModuleImplementation {
    fn from(value: &IFsrmStorageModuleImplementation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmPipelineModuleImplementation> for IFsrmStorageModuleImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmPipelineModuleImplementation> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, IFsrmPipelineModuleImplementation> for &'a IFsrmStorageModuleImplementation {
    fn into_param(self) -> ::windows::core::Param<'a, IFsrmPipelineModuleImplementation> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmStorageModuleImplementation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmStorageModuleImplementation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmStorageModuleImplementation {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmStorageModuleImplementation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmStorageModuleImplementation").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmStorageModuleImplementation {
    type Vtable = IFsrmStorageModuleImplementation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0af4a0da_895a_4e50_8712_a96724bcec64);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmStorageModuleImplementation_Vtbl {
    pub base: IFsrmPipelineModuleImplementation_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub UseDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertydefinitions: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UseDefinitions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub LoadProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertybag: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LoadProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SaveProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertybag: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SaveProperties: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const MessageSizeLimit: u32 = 4096u32;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
