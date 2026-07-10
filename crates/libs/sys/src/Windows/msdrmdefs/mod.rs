pub type DRMATTESTTYPE = i32;
pub const DRMATTESTTYPE_FULLENVIRONMENT: DRMATTESTTYPE = 0;
pub const DRMATTESTTYPE_HASHONLY: DRMATTESTTYPE = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DRMBOUNDLICENSEPARAMS {
    pub uVersion: u32,
    pub hEnablingPrincipal: DRMHANDLE,
    pub hSecureStore: DRMHANDLE,
    pub wszRightsRequested: windows_sys::core::PWSTR,
    pub wszRightsGroup: windows_sys::core::PWSTR,
    pub idResource: DRMID,
    pub cAuthenticatorCount: u32,
    pub rghAuthenticators: *mut DRMHANDLE,
    pub wszDefaultEnablingPrincipalCredentials: windows_sys::core::PWSTR,
    pub dwFlags: u32,
}
impl Default for DRMBOUNDLICENSEPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DRMCALLBACK = Option<unsafe extern "system" fn(param0: DRM_STATUS_MSG, param1: windows_sys::core::HRESULT, param2: *mut core::ffi::c_void, param3: *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
pub type DRMENCODINGTYPE = i32;
pub const DRMENCODINGTYPE_BASE64: DRMENCODINGTYPE = 0;
pub const DRMENCODINGTYPE_LONG: DRMENCODINGTYPE = 2;
pub const DRMENCODINGTYPE_RAW: DRMENCODINGTYPE = 5;
pub const DRMENCODINGTYPE_STRING: DRMENCODINGTYPE = 1;
pub const DRMENCODINGTYPE_TIME: DRMENCODINGTYPE = 3;
pub const DRMENCODINGTYPE_UINT: DRMENCODINGTYPE = 4;
pub type DRMENVHANDLE = u32;
pub const DRMENVHANDLE_INVALID: u32 = 0;
pub type DRMGLOBALOPTIONS = i32;
pub const DRMGLOBALOPTIONS_USE_SERVERSECURITYPROCESSOR: DRMGLOBALOPTIONS = 1;
pub const DRMGLOBALOPTIONS_USE_WINHTTP: DRMGLOBALOPTIONS = 0;
pub type DRMHANDLE = u32;
pub const DRMHANDLE_INVALID: u32 = 0;
pub type DRMHSESSION = u32;
pub const DRMHSESSION_INVALID: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DRMID {
    pub uVersion: u32,
    pub wszIDType: windows_sys::core::PWSTR,
    pub wszID: windows_sys::core::PWSTR,
}
impl Default for DRMID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DRMPUBHANDLE = u32;
pub const DRMPUBHANDLE_INVALID: u32 = 0;
pub type DRMQUERYHANDLE = u32;
pub const DRMQUERYHANDLE_INVALID: u32 = 0;
pub type DRMSECURITYPROVIDERTYPE = i32;
pub const DRMSECURITYPROVIDERTYPE_SOFTWARESECREP: DRMSECURITYPROVIDERTYPE = 0;
pub type DRMSPECTYPE = i32;
pub const DRMSPECTYPE_FILENAME: DRMSPECTYPE = 1;
pub const DRMSPECTYPE_UNKNOWN: DRMSPECTYPE = 0;
pub type DRMTIMETYPE = i32;
pub const DRMTIMETYPE_SYSTEMLOCAL: DRMTIMETYPE = 1;
pub const DRMTIMETYPE_SYSTEMUTC: DRMTIMETYPE = 0;
pub const DRM_ACTIVATE_CANCEL: u32 = 8;
pub const DRM_ACTIVATE_DELAYED: u32 = 64;
pub const DRM_ACTIVATE_GROUPIDENTITY: u32 = 2;
pub const DRM_ACTIVATE_MACHINE: u32 = 1;
pub const DRM_ACTIVATE_SHARED_GROUPIDENTITY: u32 = 32;
pub const DRM_ACTIVATE_SILENT: u32 = 16;
pub const DRM_ACTIVATE_TEMPORARY: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DRM_ACTSERV_INFO {
    pub uVersion: u32,
    pub wszPubKey: windows_sys::core::PWSTR,
    pub wszURL: windows_sys::core::PWSTR,
}
impl Default for DRM_ACTSERV_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DRM_ADD_LICENSE_NOPERSIST: u32 = 0;
pub const DRM_ADD_LICENSE_PERSIST: u32 = 1;
pub const DRM_AILT_CANCEL: u32 = 4;
pub const DRM_AILT_NONSILENT: u32 = 1;
pub const DRM_AILT_OBTAIN_ALL: u32 = 2;
pub const DRM_AL_CANCEL: u32 = 4;
pub const DRM_AL_FETCHNOADVISORY: u32 = 8;
pub const DRM_AL_NONSILENT: u32 = 1;
pub const DRM_AL_NOPERSIST: u32 = 2;
pub const DRM_AL_NOUI: u32 = 16;
pub const DRM_AUTO_GENERATE_KEY: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DRM_CLIENT_VERSION_INFO {
    pub uStructVersion: u32,
    pub dwVersion: [u32; 4],
    pub wszHierarchy: [u16; 256],
    pub wszProductId: [u16; 256],
    pub wszProductDescription: [u16; 256],
}
impl Default for DRM_CLIENT_VERSION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DRM_DEFAULTGROUPIDTYPE_PASSPORT: windows_sys::core::PCWSTR = windows_sys::core::w!("PassportAuthProvider");
pub const DRM_DEFAULTGROUPIDTYPE_WINDOWSAUTH: windows_sys::core::PCWSTR = windows_sys::core::w!("WindowsAuthProvider");
pub type DRM_DISTRIBUTION_POINT_INFO = i32;
pub const DRM_DISTRIBUTION_POINT_LICENSE_ACQUISITION: DRM_DISTRIBUTION_POINT_INFO = 0;
pub const DRM_DISTRIBUTION_POINT_PUBLISHING: DRM_DISTRIBUTION_POINT_INFO = 1;
pub const DRM_DISTRIBUTION_POINT_REFERRAL_INFO: DRM_DISTRIBUTION_POINT_INFO = 2;
pub const DRM_EL_CLIENTLICENSOR: u32 = 128;
pub const DRM_EL_CLIENTLICENSOR_LID: u32 = 256;
pub const DRM_EL_EUL: u32 = 32;
pub const DRM_EL_EUL_LID: u32 = 64;
pub const DRM_EL_EXPIRED: u32 = 4096;
pub const DRM_EL_GROUPIDENTITY: u32 = 2;
pub const DRM_EL_GROUPIDENTITY_LID: u32 = 8;
pub const DRM_EL_GROUPIDENTITY_NAME: u32 = 4;
pub const DRM_EL_ISSUANCELICENSE_TEMPLATE: u32 = 16384;
pub const DRM_EL_ISSUANCELICENSE_TEMPLATE_LID: u32 = 32768;
pub const DRM_EL_ISSUERNAME: u32 = 8192;
pub const DRM_EL_MACHINE: u32 = 1;
pub const DRM_EL_REVOCATIONLIST: u32 = 1024;
pub const DRM_EL_REVOCATIONLIST_LID: u32 = 2048;
pub const DRM_EL_SPECIFIED_CLIENTLICENSOR: u32 = 512;
pub const DRM_EL_SPECIFIED_GROUPIDENTITY: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DRM_LICENSE_ACQ_DATA {
    pub uVersion: u32,
    pub wszURL: windows_sys::core::PWSTR,
    pub wszLocalFilename: windows_sys::core::PWSTR,
    pub pbPostData: *mut u8,
    pub dwPostDataSize: u32,
    pub wszFriendlyName: windows_sys::core::PWSTR,
}
impl Default for DRM_LICENSE_ACQ_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DRM_LOCKBOXTYPE_BLACKBOX: u32 = 2;
pub const DRM_LOCKBOXTYPE_DEFAULT: u32 = 2;
pub const DRM_LOCKBOXTYPE_NONE: u32 = 0;
pub const DRM_LOCKBOXTYPE_WHITEBOX: u32 = 1;
pub const DRM_MSG_ACQUIRE_ADVISORY: DRM_STATUS_MSG = 3;
pub const DRM_MSG_ACQUIRE_CLIENTLICENSOR: DRM_STATUS_MSG = 5;
pub const DRM_MSG_ACQUIRE_ISSUANCE_LICENSE_TEMPLATE: DRM_STATUS_MSG = 6;
pub const DRM_MSG_ACQUIRE_LICENSE: DRM_STATUS_MSG = 2;
pub const DRM_MSG_ACTIVATE_GROUPIDENTITY: DRM_STATUS_MSG = 1;
pub const DRM_MSG_ACTIVATE_MACHINE: DRM_STATUS_MSG = 0;
pub const DRM_MSG_SIGN_ISSUANCE_LICENSE: DRM_STATUS_MSG = 4;
pub const DRM_OWNER_LICENSE_NOPERSIST: u32 = 32;
pub const DRM_REUSE_KEY: u32 = 64;
pub const DRM_SERVER_ISSUANCELICENSE: u32 = 8;
pub const DRM_SERVICE_LOCATION_ENTERPRISE: u32 = 2;
pub const DRM_SERVICE_LOCATION_INTERNET: u32 = 1;
pub const DRM_SERVICE_TYPE_ACTIVATION: u32 = 1;
pub const DRM_SERVICE_TYPE_CERTIFICATION: u32 = 2;
pub const DRM_SERVICE_TYPE_CLIENTLICENSOR: u32 = 8;
pub const DRM_SERVICE_TYPE_PUBLISHING: u32 = 4;
pub const DRM_SERVICE_TYPE_SILENT: u32 = 16;
pub const DRM_SIGN_CANCEL: u32 = 4;
pub const DRM_SIGN_OFFLINE: u32 = 2;
pub const DRM_SIGN_ONLINE: u32 = 1;
pub type DRM_STATUS_MSG = i32;
pub type DRM_USAGEPOLICY_TYPE = i32;
pub const DRM_USAGEPOLICY_TYPE_BYDIGEST: DRM_USAGEPOLICY_TYPE = 2;
pub const DRM_USAGEPOLICY_TYPE_BYNAME: DRM_USAGEPOLICY_TYPE = 0;
pub const DRM_USAGEPOLICY_TYPE_BYPUBLICKEY: DRM_USAGEPOLICY_TYPE = 1;
pub const DRM_USAGEPOLICY_TYPE_OSEXCLUSION: DRM_USAGEPOLICY_TYPE = 3;
