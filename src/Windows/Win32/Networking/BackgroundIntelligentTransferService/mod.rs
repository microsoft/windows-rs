#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AsyncIBackgroundCopyCallback(pub ::windows::core::IUnknown);
impl AsyncIBackgroundCopyCallback {
    pub unsafe fn Begin_JobTransferred<'a, Param0: ::windows::core::IntoParam<'a, IBackgroundCopyJob>>(&self, pjob: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pjob.into_param().abi()).ok()
    }
    pub unsafe fn Finish_JobTransferred(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Begin_JobError<'a, Param0: ::windows::core::IntoParam<'a, IBackgroundCopyJob>, Param1: ::windows::core::IntoParam<'a, IBackgroundCopyError>>(&self, pjob: Param0, perror: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pjob.into_param().abi(), perror.into_param().abi()).ok()
    }
    pub unsafe fn Finish_JobError(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Begin_JobModification<'a, Param0: ::windows::core::IntoParam<'a, IBackgroundCopyJob>>(&self, pjob: Param0, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pjob.into_param().abi(), ::core::mem::transmute(dwreserved)).ok()
    }
    pub unsafe fn Finish_JobModification(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for AsyncIBackgroundCopyCallback {
    type Vtable = AsyncIBackgroundCopyCallback_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca29d251_b4bb_4679_a3d9_ae8006119d54);
}
impl ::core::convert::From<AsyncIBackgroundCopyCallback> for ::windows::core::IUnknown {
    fn from(value: AsyncIBackgroundCopyCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AsyncIBackgroundCopyCallback> for ::windows::core::IUnknown {
    fn from(value: &AsyncIBackgroundCopyCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIBackgroundCopyCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AsyncIBackgroundCopyCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIBackgroundCopyCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pjob: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pjob: ::windows::core::RawPtr, perror: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pjob: ::windows::core::RawPtr, dwreserved: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct BG_AUTH_CREDENTIALS {
    pub Target: BG_AUTH_TARGET,
    pub Scheme: BG_AUTH_SCHEME,
    pub Credentials: BG_AUTH_CREDENTIALS_UNION,
}
#[cfg(feature = "Win32_Foundation")]
impl BG_AUTH_CREDENTIALS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BG_AUTH_CREDENTIALS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for BG_AUTH_CREDENTIALS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for BG_AUTH_CREDENTIALS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for BG_AUTH_CREDENTIALS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union BG_AUTH_CREDENTIALS_UNION {
    pub Basic: BG_BASIC_CREDENTIALS,
}
#[cfg(feature = "Win32_Foundation")]
impl BG_AUTH_CREDENTIALS_UNION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BG_AUTH_CREDENTIALS_UNION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for BG_AUTH_CREDENTIALS_UNION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for BG_AUTH_CREDENTIALS_UNION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for BG_AUTH_CREDENTIALS_UNION {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BG_AUTH_SCHEME(pub i32);
pub const BG_AUTH_SCHEME_BASIC: BG_AUTH_SCHEME = BG_AUTH_SCHEME(1i32);
pub const BG_AUTH_SCHEME_DIGEST: BG_AUTH_SCHEME = BG_AUTH_SCHEME(2i32);
pub const BG_AUTH_SCHEME_NTLM: BG_AUTH_SCHEME = BG_AUTH_SCHEME(3i32);
pub const BG_AUTH_SCHEME_NEGOTIATE: BG_AUTH_SCHEME = BG_AUTH_SCHEME(4i32);
pub const BG_AUTH_SCHEME_PASSPORT: BG_AUTH_SCHEME = BG_AUTH_SCHEME(5i32);
impl ::core::convert::From<i32> for BG_AUTH_SCHEME {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BG_AUTH_SCHEME {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BG_AUTH_TARGET(pub i32);
pub const BG_AUTH_TARGET_SERVER: BG_AUTH_TARGET = BG_AUTH_TARGET(1i32);
pub const BG_AUTH_TARGET_PROXY: BG_AUTH_TARGET = BG_AUTH_TARGET(2i32);
impl ::core::convert::From<i32> for BG_AUTH_TARGET {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BG_AUTH_TARGET {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct BG_BASIC_CREDENTIALS {
    pub UserName: super::super::Foundation::PWSTR,
    pub Password: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl BG_BASIC_CREDENTIALS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BG_BASIC_CREDENTIALS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for BG_BASIC_CREDENTIALS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BG_BASIC_CREDENTIALS").field("UserName", &self.UserName).field("Password", &self.Password).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for BG_BASIC_CREDENTIALS {
    fn eq(&self, other: &Self) -> bool {
        self.UserName == other.UserName && self.Password == other.Password
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for BG_BASIC_CREDENTIALS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for BG_BASIC_CREDENTIALS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BG_CERT_STORE_LOCATION(pub i32);
pub const BG_CERT_STORE_LOCATION_CURRENT_USER: BG_CERT_STORE_LOCATION = BG_CERT_STORE_LOCATION(0i32);
pub const BG_CERT_STORE_LOCATION_LOCAL_MACHINE: BG_CERT_STORE_LOCATION = BG_CERT_STORE_LOCATION(1i32);
pub const BG_CERT_STORE_LOCATION_CURRENT_SERVICE: BG_CERT_STORE_LOCATION = BG_CERT_STORE_LOCATION(2i32);
pub const BG_CERT_STORE_LOCATION_SERVICES: BG_CERT_STORE_LOCATION = BG_CERT_STORE_LOCATION(3i32);
pub const BG_CERT_STORE_LOCATION_USERS: BG_CERT_STORE_LOCATION = BG_CERT_STORE_LOCATION(4i32);
pub const BG_CERT_STORE_LOCATION_CURRENT_USER_GROUP_POLICY: BG_CERT_STORE_LOCATION = BG_CERT_STORE_LOCATION(5i32);
pub const BG_CERT_STORE_LOCATION_LOCAL_MACHINE_GROUP_POLICY: BG_CERT_STORE_LOCATION = BG_CERT_STORE_LOCATION(6i32);
pub const BG_CERT_STORE_LOCATION_LOCAL_MACHINE_ENTERPRISE: BG_CERT_STORE_LOCATION = BG_CERT_STORE_LOCATION(7i32);
impl ::core::convert::From<i32> for BG_CERT_STORE_LOCATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BG_CERT_STORE_LOCATION {
    type Abi = Self;
}
pub const BG_COPY_FILE_ALL: u32 = 15u32;
pub const BG_COPY_FILE_DACL: u32 = 4u32;
pub const BG_COPY_FILE_GROUP: u32 = 2u32;
pub const BG_COPY_FILE_OWNER: u32 = 1u32;
pub const BG_COPY_FILE_SACL: u32 = 8u32;
pub const BG_DISABLE_BRANCH_CACHE: u32 = 4u32;
pub const BG_ENABLE_PEERCACHING_CLIENT: u32 = 1u32;
pub const BG_ENABLE_PEERCACHING_SERVER: u32 = 2u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BG_ERROR_CONTEXT(pub i32);
pub const BG_ERROR_CONTEXT_NONE: BG_ERROR_CONTEXT = BG_ERROR_CONTEXT(0i32);
pub const BG_ERROR_CONTEXT_UNKNOWN: BG_ERROR_CONTEXT = BG_ERROR_CONTEXT(1i32);
pub const BG_ERROR_CONTEXT_GENERAL_QUEUE_MANAGER: BG_ERROR_CONTEXT = BG_ERROR_CONTEXT(2i32);
pub const BG_ERROR_CONTEXT_QUEUE_MANAGER_NOTIFICATION: BG_ERROR_CONTEXT = BG_ERROR_CONTEXT(3i32);
pub const BG_ERROR_CONTEXT_LOCAL_FILE: BG_ERROR_CONTEXT = BG_ERROR_CONTEXT(4i32);
pub const BG_ERROR_CONTEXT_REMOTE_FILE: BG_ERROR_CONTEXT = BG_ERROR_CONTEXT(5i32);
pub const BG_ERROR_CONTEXT_GENERAL_TRANSPORT: BG_ERROR_CONTEXT = BG_ERROR_CONTEXT(6i32);
pub const BG_ERROR_CONTEXT_REMOTE_APPLICATION: BG_ERROR_CONTEXT = BG_ERROR_CONTEXT(7i32);
pub const BG_ERROR_CONTEXT_SERVER_CERTIFICATE_CALLBACK: BG_ERROR_CONTEXT = BG_ERROR_CONTEXT(8i32);
impl ::core::convert::From<i32> for BG_ERROR_CONTEXT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BG_ERROR_CONTEXT {
    type Abi = Self;
}
pub const BG_E_APP_PACKAGE_NOT_FOUND: i32 = -2145386390i32;
pub const BG_E_APP_PACKAGE_SCENARIO_NOT_SUPPORTED: i32 = -2145386389i32;
pub const BG_E_BLOCKED_BY_BACKGROUND_ACCESS_POLICY: i32 = -2145386386i32;
pub const BG_E_BLOCKED_BY_BATTERY_POLICY: i32 = -2145386393i32;
pub const BG_E_BLOCKED_BY_BATTERY_SAVER: i32 = -2145386392i32;
pub const BG_E_BLOCKED_BY_COST_TRANSFER_POLICY: i32 = -2145386407i32;
pub const BG_E_BLOCKED_BY_GAME_MODE: i32 = -2145386385i32;
pub const BG_E_BLOCKED_BY_POLICY: i32 = -2145386434i32;
pub const BG_E_BLOCKED_BY_SYSTEM_POLICY: i32 = -2145386384i32;
pub const BG_E_BUSYCACHERECORD: i32 = -2145386424i32;
pub const BG_E_CLIENT_SERVER_PROTOCOL_MISMATCH: i32 = -2145386462i32;
pub const BG_E_COMMIT_IN_PROGRESS: i32 = -2145386429i32;
pub const BG_E_CONNECTION_CLOSED: i32 = -2145386450i32;
pub const BG_E_CONNECT_FAILURE: i32 = -2145386451i32;
pub const BG_E_DATABASE_CORRUPT: i32 = -2145386388i32;
pub const BG_E_DESTINATION_LOCKED: i32 = -2145386483i32;
pub const BG_E_DISCOVERY_IN_PROGRESS: i32 = -2145386428i32;
pub const BG_E_EMPTY: i32 = -2145386493i32;
pub const BG_E_ERROR_CONTEXT_GENERAL_QUEUE_MANAGER: i32 = -2145386488i32;
pub const BG_E_ERROR_CONTEXT_GENERAL_TRANSPORT: i32 = -2145386485i32;
pub const BG_E_ERROR_CONTEXT_LOCAL_FILE: i32 = -2145386487i32;
pub const BG_E_ERROR_CONTEXT_QUEUE_MANAGER_NOTIFICATION: i32 = -2145386484i32;
pub const BG_E_ERROR_CONTEXT_REMOTE_APPLICATION: i32 = -2145386466i32;
pub const BG_E_ERROR_CONTEXT_REMOTE_FILE: i32 = -2145386486i32;
pub const BG_E_ERROR_CONTEXT_SERVER_CERTIFICATE_CALLBACK: i32 = -2145386378i32;
pub const BG_E_ERROR_CONTEXT_UNKNOWN: i32 = -2145386489i32;
pub const BG_E_ERROR_INFORMATION_UNAVAILABLE: i32 = -2145386481i32;
pub const BG_E_FILE_NOT_AVAILABLE: i32 = -2145386492i32;
pub const BG_E_FILE_NOT_FOUND: i32 = -2145386455i32;
pub const BG_E_HTTP_ERROR_100: i32 = -2145845148i32;
pub const BG_E_HTTP_ERROR_101: i32 = -2145845147i32;
pub const BG_E_HTTP_ERROR_200: i32 = -2145845048i32;
pub const BG_E_HTTP_ERROR_201: i32 = -2145845047i32;
pub const BG_E_HTTP_ERROR_202: i32 = -2145845046i32;
pub const BG_E_HTTP_ERROR_203: i32 = -2145845045i32;
pub const BG_E_HTTP_ERROR_204: i32 = -2145845044i32;
pub const BG_E_HTTP_ERROR_205: i32 = -2145845043i32;
pub const BG_E_HTTP_ERROR_206: i32 = -2145845042i32;
pub const BG_E_HTTP_ERROR_300: i32 = -2145844948i32;
pub const BG_E_HTTP_ERROR_301: i32 = -2145844947i32;
pub const BG_E_HTTP_ERROR_302: i32 = -2145844946i32;
pub const BG_E_HTTP_ERROR_303: i32 = -2145844945i32;
pub const BG_E_HTTP_ERROR_304: i32 = -2145844944i32;
pub const BG_E_HTTP_ERROR_305: i32 = -2145844943i32;
pub const BG_E_HTTP_ERROR_307: i32 = -2145844941i32;
pub const BG_E_HTTP_ERROR_400: i32 = -2145844848i32;
pub const BG_E_HTTP_ERROR_401: i32 = -2145844847i32;
pub const BG_E_HTTP_ERROR_402: i32 = -2145844846i32;
pub const BG_E_HTTP_ERROR_403: i32 = -2145844845i32;
pub const BG_E_HTTP_ERROR_404: i32 = -2145844844i32;
pub const BG_E_HTTP_ERROR_405: i32 = -2145844843i32;
pub const BG_E_HTTP_ERROR_406: i32 = -2145844842i32;
pub const BG_E_HTTP_ERROR_407: i32 = -2145844841i32;
pub const BG_E_HTTP_ERROR_408: i32 = -2145844840i32;
pub const BG_E_HTTP_ERROR_409: i32 = -2145844839i32;
pub const BG_E_HTTP_ERROR_410: i32 = -2145844838i32;
pub const BG_E_HTTP_ERROR_411: i32 = -2145844837i32;
pub const BG_E_HTTP_ERROR_412: i32 = -2145844836i32;
pub const BG_E_HTTP_ERROR_413: i32 = -2145844835i32;
pub const BG_E_HTTP_ERROR_414: i32 = -2145844834i32;
pub const BG_E_HTTP_ERROR_415: i32 = -2145844833i32;
pub const BG_E_HTTP_ERROR_416: i32 = -2145844832i32;
pub const BG_E_HTTP_ERROR_417: i32 = -2145844831i32;
pub const BG_E_HTTP_ERROR_449: i32 = -2145844799i32;
pub const BG_E_HTTP_ERROR_500: i32 = -2145844748i32;
pub const BG_E_HTTP_ERROR_501: i32 = -2145844747i32;
pub const BG_E_HTTP_ERROR_502: i32 = -2145844746i32;
pub const BG_E_HTTP_ERROR_503: i32 = -2145844745i32;
pub const BG_E_HTTP_ERROR_504: i32 = -2145844744i32;
pub const BG_E_HTTP_ERROR_505: i32 = -2145844743i32;
pub const BG_E_INSUFFICIENT_HTTP_SUPPORT: i32 = -2145386478i32;
pub const BG_E_INSUFFICIENT_RANGE_SUPPORT: i32 = -2145386477i32;
pub const BG_E_INVALID_AUTH_SCHEME: i32 = -2145386456i32;
pub const BG_E_INVALID_AUTH_TARGET: i32 = -2145386457i32;
pub const BG_E_INVALID_CREDENTIALS: i32 = -2145386432i32;
pub const BG_E_INVALID_HASH_ALGORITHM: i32 = -2145386431i32;
pub const BG_E_INVALID_PROXY_INFO: i32 = -2145386433i32;
pub const BG_E_INVALID_RANGE: i32 = -2145386453i32;
pub const BG_E_INVALID_SERVER_RESPONSE: i32 = -2145386469i32;
pub const BG_E_INVALID_STATE: i32 = -2145386494i32;
pub const BG_E_LOCAL_FILE_CHANGED: i32 = -2145386467i32;
pub const BG_E_MAXDOWNLOAD_TIMEOUT: i32 = -2145386412i32;
pub const BG_E_MAX_DOWNLOAD_SIZE_INVALID_VALUE: i32 = -2145386397i32;
pub const BG_E_MAX_DOWNLOAD_SIZE_LIMIT_REACHED: i32 = -2145386396i32;
pub const BG_E_MISSING_FILE_SIZE: i32 = -2145386479i32;
pub const BG_E_NETWORK_DISCONNECTED: i32 = -2145386480i32;
pub const BG_E_NEW_OWNER_DIFF_MAPPING: i32 = -2145386475i32;
pub const BG_E_NEW_OWNER_NO_FILE_ACCESS: i32 = -2145386474i32;
pub const BG_E_NOT_FOUND: i32 = -2145386495i32;
pub const BG_E_NOT_SUPPORTED_WITH_CUSTOM_HTTP_METHOD: i32 = -2145386383i32;
pub const BG_E_NO_PROGRESS: i32 = -2145386460i32;
pub const BG_E_OVERLAPPING_RANGES: i32 = -2145386452i32;
pub const BG_E_PASSWORD_TOO_LARGE: i32 = -2145386458i32;
pub const BG_E_PEERCACHING_DISABLED: i32 = -2145386425i32;
pub const BG_E_PROPERTY_SUPPORTED_FOR_DOWNLOAD_JOBS_ONLY: i32 = -2145386400i32;
pub const BG_E_PROTOCOL_NOT_AVAILABLE: i32 = -2145386491i32;
pub const BG_E_PROXY_BYPASS_LIST_TOO_LARGE: i32 = -2145386471i32;
pub const BG_E_PROXY_LIST_TOO_LARGE: i32 = -2145386472i32;
pub const BG_E_RANDOM_ACCESS_NOT_SUPPORTED: i32 = -2145386387i32;
pub const BG_E_READ_ONLY_PROPERTY: i32 = -2145386408i32;
pub const BG_E_READ_ONLY_PROPERTY_AFTER_ADDFILE: i32 = -2145386399i32;
pub const BG_E_READ_ONLY_PROPERTY_AFTER_RESUME: i32 = -2145386398i32;
pub const BG_E_READ_ONLY_WHEN_JOB_ACTIVE: i32 = -2145386379i32;
pub const BG_E_RECORD_DELETED: i32 = -2145386430i32;
pub const BG_E_REMOTE_FILE_CHANGED: i32 = -2145386381i32;
pub const BG_E_REMOTE_NOT_SUPPORTED: i32 = -2145386476i32;
pub const BG_E_SERVER_CERT_VALIDATION_INTERFACE_REQUIRED: i32 = -2145386380i32;
pub const BG_E_SERVER_EXECUTE_ENABLE: i32 = -2145386461i32;
pub const BG_E_SESSION_NOT_FOUND: i32 = -2145386465i32;
pub const BG_E_STANDBY_MODE: i32 = -2145386395i32;
pub const BG_E_STRING_TOO_LONG: i32 = -2145386463i32;
pub const BG_E_TEST_OPTION_BLOCKED_DOWNLOAD: i32 = -2145386426i32;
pub const BG_E_TOKEN_REQUIRED: i32 = -2145386410i32;
pub const BG_E_TOO_LARGE: i32 = -2145386464i32;
pub const BG_E_TOO_MANY_FILES: i32 = -2145386468i32;
pub const BG_E_TOO_MANY_FILES_IN_JOB: i32 = -2145386415i32;
pub const BG_E_TOO_MANY_JOBS_PER_MACHINE: i32 = -2145386416i32;
pub const BG_E_TOO_MANY_JOBS_PER_USER: i32 = -2145386423i32;
pub const BG_E_TOO_MANY_RANGES_IN_FILE: i32 = -2145386414i32;
pub const BG_E_UNKNOWN_PROPERTY_ID: i32 = -2145386409i32;
pub const BG_E_UNSUPPORTED_JOB_CONFIGURATION: i32 = -2145386382i32;
pub const BG_E_UPNP_ERROR: i32 = -2145386427i32;
pub const BG_E_USERNAME_TOO_LARGE: i32 = -2145386459i32;
pub const BG_E_USE_STORED_CREDENTIALS_NOT_SUPPORTED: i32 = -2145386394i32;
pub const BG_E_VALIDATION_FAILED: i32 = -2145386413i32;
pub const BG_E_VOLUME_CHANGED: i32 = -2145386482i32;
pub const BG_E_WATCHDOG_TIMEOUT: i32 = -2145386391i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct BG_FILE_INFO {
    pub RemoteName: super::super::Foundation::PWSTR,
    pub LocalName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl BG_FILE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BG_FILE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for BG_FILE_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BG_FILE_INFO").field("RemoteName", &self.RemoteName).field("LocalName", &self.LocalName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for BG_FILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.RemoteName == other.RemoteName && self.LocalName == other.LocalName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for BG_FILE_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for BG_FILE_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct BG_FILE_PROGRESS {
    pub BytesTotal: u64,
    pub BytesTransferred: u64,
    pub Completed: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl BG_FILE_PROGRESS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BG_FILE_PROGRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for BG_FILE_PROGRESS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BG_FILE_PROGRESS").field("BytesTotal", &self.BytesTotal).field("BytesTransferred", &self.BytesTransferred).field("Completed", &self.Completed).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for BG_FILE_PROGRESS {
    fn eq(&self, other: &Self) -> bool {
        self.BytesTotal == other.BytesTotal && self.BytesTransferred == other.BytesTransferred && self.Completed == other.Completed
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for BG_FILE_PROGRESS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for BG_FILE_PROGRESS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct BG_FILE_RANGE {
    pub InitialOffset: u64,
    pub Length: u64,
}
impl BG_FILE_RANGE {}
impl ::core::default::Default for BG_FILE_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for BG_FILE_RANGE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BG_FILE_RANGE").field("InitialOffset", &self.InitialOffset).field("Length", &self.Length).finish()
    }
}
impl ::core::cmp::PartialEq for BG_FILE_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.InitialOffset == other.InitialOffset && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for BG_FILE_RANGE {}
unsafe impl ::windows::core::Abi for BG_FILE_RANGE {
    type Abi = Self;
}
pub const BG_HTTP_REDIRECT_POLICY_ALLOW_HTTPS_TO_HTTP: u32 = 2048u32;
pub const BG_HTTP_REDIRECT_POLICY_ALLOW_REPORT: u32 = 256u32;
pub const BG_HTTP_REDIRECT_POLICY_ALLOW_SILENT: u32 = 0u32;
pub const BG_HTTP_REDIRECT_POLICY_DISALLOW: u32 = 512u32;
pub const BG_HTTP_REDIRECT_POLICY_MASK: u32 = 1792u32;
pub const BG_JOB_DISABLE_BRANCH_CACHE: u32 = 4u32;
pub const BG_JOB_ENABLE_PEERCACHING_CLIENT: u32 = 1u32;
pub const BG_JOB_ENABLE_PEERCACHING_SERVER: u32 = 2u32;
pub const BG_JOB_ENUM_ALL_USERS: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BG_JOB_PRIORITY(pub i32);
pub const BG_JOB_PRIORITY_FOREGROUND: BG_JOB_PRIORITY = BG_JOB_PRIORITY(0i32);
pub const BG_JOB_PRIORITY_HIGH: BG_JOB_PRIORITY = BG_JOB_PRIORITY(1i32);
pub const BG_JOB_PRIORITY_NORMAL: BG_JOB_PRIORITY = BG_JOB_PRIORITY(2i32);
pub const BG_JOB_PRIORITY_LOW: BG_JOB_PRIORITY = BG_JOB_PRIORITY(3i32);
impl ::core::convert::From<i32> for BG_JOB_PRIORITY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BG_JOB_PRIORITY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct BG_JOB_PROGRESS {
    pub BytesTotal: u64,
    pub BytesTransferred: u64,
    pub FilesTotal: u32,
    pub FilesTransferred: u32,
}
impl BG_JOB_PROGRESS {}
impl ::core::default::Default for BG_JOB_PROGRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for BG_JOB_PROGRESS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BG_JOB_PROGRESS").field("BytesTotal", &self.BytesTotal).field("BytesTransferred", &self.BytesTransferred).field("FilesTotal", &self.FilesTotal).field("FilesTransferred", &self.FilesTransferred).finish()
    }
}
impl ::core::cmp::PartialEq for BG_JOB_PROGRESS {
    fn eq(&self, other: &Self) -> bool {
        self.BytesTotal == other.BytesTotal && self.BytesTransferred == other.BytesTransferred && self.FilesTotal == other.FilesTotal && self.FilesTransferred == other.FilesTransferred
    }
}
impl ::core::cmp::Eq for BG_JOB_PROGRESS {}
unsafe impl ::windows::core::Abi for BG_JOB_PROGRESS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BG_JOB_PROXY_USAGE(pub i32);
pub const BG_JOB_PROXY_USAGE_PRECONFIG: BG_JOB_PROXY_USAGE = BG_JOB_PROXY_USAGE(0i32);
pub const BG_JOB_PROXY_USAGE_NO_PROXY: BG_JOB_PROXY_USAGE = BG_JOB_PROXY_USAGE(1i32);
pub const BG_JOB_PROXY_USAGE_OVERRIDE: BG_JOB_PROXY_USAGE = BG_JOB_PROXY_USAGE(2i32);
pub const BG_JOB_PROXY_USAGE_AUTODETECT: BG_JOB_PROXY_USAGE = BG_JOB_PROXY_USAGE(3i32);
impl ::core::convert::From<i32> for BG_JOB_PROXY_USAGE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BG_JOB_PROXY_USAGE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct BG_JOB_REPLY_PROGRESS {
    pub BytesTotal: u64,
    pub BytesTransferred: u64,
}
impl BG_JOB_REPLY_PROGRESS {}
impl ::core::default::Default for BG_JOB_REPLY_PROGRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for BG_JOB_REPLY_PROGRESS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BG_JOB_REPLY_PROGRESS").field("BytesTotal", &self.BytesTotal).field("BytesTransferred", &self.BytesTransferred).finish()
    }
}
impl ::core::cmp::PartialEq for BG_JOB_REPLY_PROGRESS {
    fn eq(&self, other: &Self) -> bool {
        self.BytesTotal == other.BytesTotal && self.BytesTransferred == other.BytesTransferred
    }
}
impl ::core::cmp::Eq for BG_JOB_REPLY_PROGRESS {}
unsafe impl ::windows::core::Abi for BG_JOB_REPLY_PROGRESS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BG_JOB_STATE(pub i32);
pub const BG_JOB_STATE_QUEUED: BG_JOB_STATE = BG_JOB_STATE(0i32);
pub const BG_JOB_STATE_CONNECTING: BG_JOB_STATE = BG_JOB_STATE(1i32);
pub const BG_JOB_STATE_TRANSFERRING: BG_JOB_STATE = BG_JOB_STATE(2i32);
pub const BG_JOB_STATE_SUSPENDED: BG_JOB_STATE = BG_JOB_STATE(3i32);
pub const BG_JOB_STATE_ERROR: BG_JOB_STATE = BG_JOB_STATE(4i32);
pub const BG_JOB_STATE_TRANSIENT_ERROR: BG_JOB_STATE = BG_JOB_STATE(5i32);
pub const BG_JOB_STATE_TRANSFERRED: BG_JOB_STATE = BG_JOB_STATE(6i32);
pub const BG_JOB_STATE_ACKNOWLEDGED: BG_JOB_STATE = BG_JOB_STATE(7i32);
pub const BG_JOB_STATE_CANCELLED: BG_JOB_STATE = BG_JOB_STATE(8i32);
impl ::core::convert::From<i32> for BG_JOB_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BG_JOB_STATE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct BG_JOB_TIMES {
    pub CreationTime: super::super::Foundation::FILETIME,
    pub ModificationTime: super::super::Foundation::FILETIME,
    pub TransferCompletionTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl BG_JOB_TIMES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BG_JOB_TIMES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for BG_JOB_TIMES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BG_JOB_TIMES").field("CreationTime", &self.CreationTime).field("ModificationTime", &self.ModificationTime).field("TransferCompletionTime", &self.TransferCompletionTime).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for BG_JOB_TIMES {
    fn eq(&self, other: &Self) -> bool {
        self.CreationTime == other.CreationTime && self.ModificationTime == other.ModificationTime && self.TransferCompletionTime == other.TransferCompletionTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for BG_JOB_TIMES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for BG_JOB_TIMES {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BG_JOB_TYPE(pub i32);
pub const BG_JOB_TYPE_DOWNLOAD: BG_JOB_TYPE = BG_JOB_TYPE(0i32);
pub const BG_JOB_TYPE_UPLOAD: BG_JOB_TYPE = BG_JOB_TYPE(1i32);
pub const BG_JOB_TYPE_UPLOAD_REPLY: BG_JOB_TYPE = BG_JOB_TYPE(2i32);
impl ::core::convert::From<i32> for BG_JOB_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BG_JOB_TYPE {
    type Abi = Self;
}
pub const BG_NOTIFY_DISABLE: u32 = 4u32;
pub const BG_NOTIFY_FILE_RANGES_TRANSFERRED: u32 = 32u32;
pub const BG_NOTIFY_FILE_TRANSFERRED: u32 = 16u32;
pub const BG_NOTIFY_JOB_ERROR: u32 = 2u32;
pub const BG_NOTIFY_JOB_MODIFICATION: u32 = 8u32;
pub const BG_NOTIFY_JOB_TRANSFERRED: u32 = 1u32;
pub const BG_SSL_ENABLE_CRL_CHECK: u32 = 1u32;
pub const BG_SSL_IGNORE_CERT_CN_INVALID: u32 = 2u32;
pub const BG_SSL_IGNORE_CERT_DATE_INVALID: u32 = 4u32;
pub const BG_SSL_IGNORE_CERT_WRONG_USAGE: u32 = 16u32;
pub const BG_SSL_IGNORE_UNKNOWN_CA: u32 = 8u32;
pub const BG_S_ERROR_CONTEXT_NONE: i32 = 2097158i32;
pub const BG_S_OVERRIDDEN_BY_POLICY: i32 = 2097237i32;
pub const BG_S_PARTIAL_COMPLETE: i32 = 2097175i32;
pub const BG_S_PROXY_CHANGED: i32 = 2097194i32;
pub const BG_S_UNABLE_TO_DELETE_FILES: i32 = 2097178i32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BG_TOKEN(pub u32);
pub const BG_TOKEN_LOCAL_FILE: BG_TOKEN = BG_TOKEN(1u32);
pub const BG_TOKEN_NETWORK: BG_TOKEN = BG_TOKEN(2u32);
impl ::core::convert::From<u32> for BG_TOKEN {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BG_TOKEN {
    type Abi = Self;
}
impl ::core::ops::BitOr for BG_TOKEN {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for BG_TOKEN {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for BG_TOKEN {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for BG_TOKEN {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for BG_TOKEN {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const BITSExtensionSetupFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefbbab68_7286_4783_94bf_9461d8b7e7e9);
pub const BITS_COST_OPTION_IGNORE_CONGESTION: u32 = 2147483648u32;
pub const BITS_COST_STATE_BELOW_CAP: u32 = 4u32;
pub const BITS_COST_STATE_CAPPED_USAGE_UNKNOWN: u32 = 2u32;
pub const BITS_COST_STATE_NEAR_CAP: u32 = 8u32;
pub const BITS_COST_STATE_OVERCAP_CHARGED: u32 = 16u32;
pub const BITS_COST_STATE_OVERCAP_THROTTLED: u32 = 32u32;
pub const BITS_COST_STATE_RESERVED: u32 = 1073741824u32;
pub const BITS_COST_STATE_ROAMING: u32 = 128u32;
pub const BITS_COST_STATE_UNRESTRICTED: u32 = 1u32;
pub const BITS_COST_STATE_USAGE_BASED: u32 = 64u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BITS_FILE_PROPERTY_ID(pub i32);
pub const BITS_FILE_PROPERTY_ID_HTTP_RESPONSE_HEADERS: BITS_FILE_PROPERTY_ID = BITS_FILE_PROPERTY_ID(1i32);
impl ::core::convert::From<i32> for BITS_FILE_PROPERTY_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BITS_FILE_PROPERTY_ID {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union BITS_FILE_PROPERTY_VALUE {
    pub String: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl BITS_FILE_PROPERTY_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BITS_FILE_PROPERTY_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for BITS_FILE_PROPERTY_VALUE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for BITS_FILE_PROPERTY_VALUE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for BITS_FILE_PROPERTY_VALUE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BITS_JOB_PROPERTY_ID(pub i32);
pub const BITS_JOB_PROPERTY_ID_COST_FLAGS: BITS_JOB_PROPERTY_ID = BITS_JOB_PROPERTY_ID(1i32);
pub const BITS_JOB_PROPERTY_NOTIFICATION_CLSID: BITS_JOB_PROPERTY_ID = BITS_JOB_PROPERTY_ID(2i32);
pub const BITS_JOB_PROPERTY_DYNAMIC_CONTENT: BITS_JOB_PROPERTY_ID = BITS_JOB_PROPERTY_ID(3i32);
pub const BITS_JOB_PROPERTY_HIGH_PERFORMANCE: BITS_JOB_PROPERTY_ID = BITS_JOB_PROPERTY_ID(4i32);
pub const BITS_JOB_PROPERTY_MAX_DOWNLOAD_SIZE: BITS_JOB_PROPERTY_ID = BITS_JOB_PROPERTY_ID(5i32);
pub const BITS_JOB_PROPERTY_USE_STORED_CREDENTIALS: BITS_JOB_PROPERTY_ID = BITS_JOB_PROPERTY_ID(7i32);
pub const BITS_JOB_PROPERTY_MINIMUM_NOTIFICATION_INTERVAL_MS: BITS_JOB_PROPERTY_ID = BITS_JOB_PROPERTY_ID(9i32);
pub const BITS_JOB_PROPERTY_ON_DEMAND_MODE: BITS_JOB_PROPERTY_ID = BITS_JOB_PROPERTY_ID(10i32);
impl ::core::convert::From<i32> for BITS_JOB_PROPERTY_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BITS_JOB_PROPERTY_ID {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union BITS_JOB_PROPERTY_VALUE {
    pub Dword: u32,
    pub ClsID: ::windows::core::GUID,
    pub Enable: super::super::Foundation::BOOL,
    pub Uint64: u64,
    pub Target: BG_AUTH_TARGET,
}
#[cfg(feature = "Win32_Foundation")]
impl BITS_JOB_PROPERTY_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BITS_JOB_PROPERTY_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for BITS_JOB_PROPERTY_VALUE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for BITS_JOB_PROPERTY_VALUE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for BITS_JOB_PROPERTY_VALUE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BITS_JOB_TRANSFER_POLICY(pub i32);
pub const BITS_JOB_TRANSFER_POLICY_ALWAYS: BITS_JOB_TRANSFER_POLICY = BITS_JOB_TRANSFER_POLICY(-2147483393i32);
pub const BITS_JOB_TRANSFER_POLICY_NOT_ROAMING: BITS_JOB_TRANSFER_POLICY = BITS_JOB_TRANSFER_POLICY(-2147483521i32);
pub const BITS_JOB_TRANSFER_POLICY_NO_SURCHARGE: BITS_JOB_TRANSFER_POLICY = BITS_JOB_TRANSFER_POLICY(-2147483537i32);
pub const BITS_JOB_TRANSFER_POLICY_STANDARD: BITS_JOB_TRANSFER_POLICY = BITS_JOB_TRANSFER_POLICY(-2147483545i32);
pub const BITS_JOB_TRANSFER_POLICY_UNRESTRICTED: BITS_JOB_TRANSFER_POLICY = BITS_JOB_TRANSFER_POLICY(-2147483615i32);
impl ::core::convert::From<i32> for BITS_JOB_TRANSFER_POLICY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BITS_JOB_TRANSFER_POLICY {
    type Abi = Self;
}
pub const BITS_MC_FAILED_TO_START: i32 = -2145828856i32;
pub const BITS_MC_FATAL_IGD_ERROR: i32 = -2145828855i32;
pub const BITS_MC_FILE_DELETION_FAILED: i32 = -2145828863i32;
pub const BITS_MC_FILE_DELETION_FAILED_MORE: i32 = -2145828862i32;
pub const BITS_MC_JOB_CANCELLED: i32 = -2145828864i32;
pub const BITS_MC_JOB_NOTIFICATION_FAILURE: i32 = -2145828858i32;
pub const BITS_MC_JOB_PROPERTY_CHANGE: i32 = -2145828861i32;
pub const BITS_MC_JOB_SCAVENGED: i32 = -2145828859i32;
pub const BITS_MC_JOB_TAKE_OWNERSHIP: i32 = -2145828860i32;
pub const BITS_MC_PEERCACHING_PORT: i32 = -2145828854i32;
pub const BITS_MC_STATE_FILE_CORRUPT: i32 = -2145828857i32;
pub const BITS_MC_WSD_PORT: i32 = -2145828853i32;
pub const BackgroundCopyManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4991d34b_80a1_4291_83b6_3328366b9097);
pub const BackgroundCopyManager10_1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bd3e4e1_7bd4_4a2b_9964_496400de5193);
pub const BackgroundCopyManager10_2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4575438f_a6c8_4976_b0fe_2f26b80d959e);
pub const BackgroundCopyManager10_3: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5fd42ad5_c04e_4d36_adc7_e08ff15737ad);
pub const BackgroundCopyManager1_5: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf087771f_d74f_4c1a_bb8a_e16aca9124ea);
pub const BackgroundCopyManager2_0: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d18ad12_bde3_4393_b311_099c346e6df9);
pub const BackgroundCopyManager2_5: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03ca98d6_ff5d_49b8_abc6_03dd84127020);
pub const BackgroundCopyManager3_0: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x659cdea7_489e_11d9_a9cd_000d56965251);
pub const BackgroundCopyManager4_0: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb6df56b_cace_11dc_9992_0019b93a3a84);
pub const BackgroundCopyManager5_0: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ecca34c_e88a_44e3_8d6a_8921bde9e452);
pub const BackgroundCopyQMgr: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69ad4aee_51be_439b_a92c_86ae490e8b30);
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FILESETINFO {
    pub bstrRemoteFile: super::super::Foundation::BSTR,
    pub bstrLocalFile: super::super::Foundation::BSTR,
    pub dwSizeHint: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl FILESETINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FILESETINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FILESETINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FILESETINFO").field("bstrRemoteFile", &self.bstrRemoteFile).field("bstrLocalFile", &self.bstrLocalFile).field("dwSizeHint", &self.dwSizeHint).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FILESETINFO {
    fn eq(&self, other: &Self) -> bool {
        self.bstrRemoteFile == other.bstrRemoteFile && self.bstrLocalFile == other.bstrLocalFile && self.dwSizeHint == other.dwSizeHint
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FILESETINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FILESETINFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GROUPPROP(pub i32);
pub const GROUPPROP_PRIORITY: GROUPPROP = GROUPPROP(0i32);
pub const GROUPPROP_REMOTEUSERID: GROUPPROP = GROUPPROP(1i32);
pub const GROUPPROP_REMOTEUSERPWD: GROUPPROP = GROUPPROP(2i32);
pub const GROUPPROP_LOCALUSERID: GROUPPROP = GROUPPROP(3i32);
pub const GROUPPROP_LOCALUSERPWD: GROUPPROP = GROUPPROP(4i32);
pub const GROUPPROP_PROTOCOLFLAGS: GROUPPROP = GROUPPROP(5i32);
pub const GROUPPROP_NOTIFYFLAGS: GROUPPROP = GROUPPROP(6i32);
pub const GROUPPROP_NOTIFYCLSID: GROUPPROP = GROUPPROP(7i32);
pub const GROUPPROP_PROGRESSSIZE: GROUPPROP = GROUPPROP(8i32);
pub const GROUPPROP_PROGRESSPERCENT: GROUPPROP = GROUPPROP(9i32);
pub const GROUPPROP_PROGRESSTIME: GROUPPROP = GROUPPROP(10i32);
pub const GROUPPROP_DISPLAYNAME: GROUPPROP = GROUPPROP(11i32);
pub const GROUPPROP_DESCRIPTION: GROUPPROP = GROUPPROP(12i32);
impl ::core::convert::From<i32> for GROUPPROP {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GROUPPROP {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBITSExtensionSetup(pub ::windows::core::IUnknown);
impl IBITSExtensionSetup {
    pub unsafe fn EnableBITSUploads(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn DisableBITSUploads(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCleanupTaskName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetCleanupTask(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
}
unsafe impl ::windows::core::Interface for IBITSExtensionSetup {
    type Vtable = IBITSExtensionSetup_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29cfbbf7_09e4_4b97_b0bc_f2287e3d8eb3);
}
impl ::core::convert::From<IBITSExtensionSetup> for ::windows::core::IUnknown {
    fn from(value: IBITSExtensionSetup) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBITSExtensionSetup> for ::windows::core::IUnknown {
    fn from(value: &IBITSExtensionSetup) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBITSExtensionSetup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBITSExtensionSetup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IBITSExtensionSetup> for super::super::System::Com::IDispatch {
    fn from(value: IBITSExtensionSetup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IBITSExtensionSetup> for super::super::System::Com::IDispatch {
    fn from(value: &IBITSExtensionSetup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IBITSExtensionSetup {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IBITSExtensionSetup {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBITSExtensionSetup_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptaskname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBITSExtensionSetupFactory(pub ::windows::core::IUnknown);
impl IBITSExtensionSetupFactory {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetObject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::core::Result<IBITSExtensionSetup> {
        let mut result__: <IBITSExtensionSetup as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), path.into_param().abi(), &mut result__).from_abi::<IBITSExtensionSetup>(result__)
    }
}
unsafe impl ::windows::core::Interface for IBITSExtensionSetupFactory {
    type Vtable = IBITSExtensionSetupFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5d2d542_5503_4e64_8b48_72ef91a32ee1);
}
impl ::core::convert::From<IBITSExtensionSetupFactory> for ::windows::core::IUnknown {
    fn from(value: IBITSExtensionSetupFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBITSExtensionSetupFactory> for ::windows::core::IUnknown {
    fn from(value: &IBITSExtensionSetupFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBITSExtensionSetupFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBITSExtensionSetupFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IBITSExtensionSetupFactory> for super::super::System::Com::IDispatch {
    fn from(value: IBITSExtensionSetupFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IBITSExtensionSetupFactory> for super::super::System::Com::IDispatch {
    fn from(value: &IBITSExtensionSetupFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IBITSExtensionSetupFactory {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IBITSExtensionSetupFactory {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBITSExtensionSetupFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppextensionsetup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBackgroundCopyCallback(pub ::windows::core::IUnknown);
impl IBackgroundCopyCallback {
    pub unsafe fn JobTransferred<'a, Param0: ::windows::core::IntoParam<'a, IBackgroundCopyJob>>(&self, pjob: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pjob.into_param().abi()).ok()
    }
    pub unsafe fn JobError<'a, Param0: ::windows::core::IntoParam<'a, IBackgroundCopyJob>, Param1: ::windows::core::IntoParam<'a, IBackgroundCopyError>>(&self, pjob: Param0, perror: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pjob.into_param().abi(), perror.into_param().abi()).ok()
    }
    pub unsafe fn JobModification<'a, Param0: ::windows::core::IntoParam<'a, IBackgroundCopyJob>>(&self, pjob: Param0, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pjob.into_param().abi(), ::core::mem::transmute(dwreserved)).ok()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyCallback {
    type Vtable = IBackgroundCopyCallback_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97ea99c7_0186_4ad4_8df9_c5b4e0ed6b22);
}
impl ::core::convert::From<IBackgroundCopyCallback> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBackgroundCopyCallback> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundCopyCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBackgroundCopyCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pjob: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pjob: ::windows::core::RawPtr, perror: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pjob: ::windows::core::RawPtr, dwreserved: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBackgroundCopyCallback1(pub ::windows::core::IUnknown);
impl IBackgroundCopyCallback1 {
    pub unsafe fn OnStatus<'a, Param0: ::windows::core::IntoParam<'a, IBackgroundCopyGroup>, Param1: ::windows::core::IntoParam<'a, IBackgroundCopyJob1>>(&self, pgroup: Param0, pjob: Param1, dwfileindex: u32, dwstatus: u32, dwnumofretries: u32, dwwin32result: u32, dwtransportresult: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pgroup.into_param().abi(), pjob.into_param().abi(), ::core::mem::transmute(dwfileindex), ::core::mem::transmute(dwstatus), ::core::mem::transmute(dwnumofretries), ::core::mem::transmute(dwwin32result), ::core::mem::transmute(dwtransportresult)).ok()
    }
    pub unsafe fn OnProgress<'a, Param1: ::windows::core::IntoParam<'a, IBackgroundCopyGroup>, Param2: ::windows::core::IntoParam<'a, IBackgroundCopyJob1>>(&self, progresstype: u32, pgroup: Param1, pjob: Param2, dwfileindex: u32, dwprogressvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(progresstype), pgroup.into_param().abi(), pjob.into_param().abi(), ::core::mem::transmute(dwfileindex), ::core::mem::transmute(dwprogressvalue)).ok()
    }
    pub unsafe fn OnProgressEx<'a, Param1: ::windows::core::IntoParam<'a, IBackgroundCopyGroup>, Param2: ::windows::core::IntoParam<'a, IBackgroundCopyJob1>>(&self, progresstype: u32, pgroup: Param1, pjob: Param2, dwfileindex: u32, dwprogressvalue: u32, dwbytearraysize: u32, pbyte: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(progresstype), pgroup.into_param().abi(), pjob.into_param().abi(), ::core::mem::transmute(dwfileindex), ::core::mem::transmute(dwprogressvalue), ::core::mem::transmute(dwbytearraysize), ::core::mem::transmute(pbyte)).ok()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyCallback1 {
    type Vtable = IBackgroundCopyCallback1_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x084f6593_3800_4e08_9b59_99fa59addf82);
}
impl ::core::convert::From<IBackgroundCopyCallback1> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyCallback1) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBackgroundCopyCallback1> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyCallback1) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundCopyCallback1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBackgroundCopyCallback1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyCallback1_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pgroup: ::windows::core::RawPtr, pjob: ::windows::core::RawPtr, dwfileindex: u32, dwstatus: u32, dwnumofretries: u32, dwwin32result: u32, dwtransportresult: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, progresstype: u32, pgroup: ::windows::core::RawPtr, pjob: ::windows::core::RawPtr, dwfileindex: u32, dwprogressvalue: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, progresstype: u32, pgroup: ::windows::core::RawPtr, pjob: ::windows::core::RawPtr, dwfileindex: u32, dwprogressvalue: u32, dwbytearraysize: u32, pbyte: *const u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBackgroundCopyCallback2(pub ::windows::core::IUnknown);
impl IBackgroundCopyCallback2 {
    pub unsafe fn JobTransferred<'a, Param0: ::windows::core::IntoParam<'a, IBackgroundCopyJob>>(&self, pjob: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pjob.into_param().abi()).ok()
    }
    pub unsafe fn JobError<'a, Param0: ::windows::core::IntoParam<'a, IBackgroundCopyJob>, Param1: ::windows::core::IntoParam<'a, IBackgroundCopyError>>(&self, pjob: Param0, perror: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pjob.into_param().abi(), perror.into_param().abi()).ok()
    }
    pub unsafe fn JobModification<'a, Param0: ::windows::core::IntoParam<'a, IBackgroundCopyJob>>(&self, pjob: Param0, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pjob.into_param().abi(), ::core::mem::transmute(dwreserved)).ok()
    }
    pub unsafe fn FileTransferred<'a, Param0: ::windows::core::IntoParam<'a, IBackgroundCopyJob>, Param1: ::windows::core::IntoParam<'a, IBackgroundCopyFile>>(&self, pjob: Param0, pfile: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pjob.into_param().abi(), pfile.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyCallback2 {
    type Vtable = IBackgroundCopyCallback2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x659cdeac_489e_11d9_a9cd_000d56965251);
}
impl ::core::convert::From<IBackgroundCopyCallback2> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyCallback2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBackgroundCopyCallback2> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyCallback2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundCopyCallback2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBackgroundCopyCallback2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IBackgroundCopyCallback2> for IBackgroundCopyCallback {
    fn from(value: IBackgroundCopyCallback2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyCallback2> for IBackgroundCopyCallback {
    fn from(value: &IBackgroundCopyCallback2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyCallback> for IBackgroundCopyCallback2 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyCallback> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyCallback> for &IBackgroundCopyCallback2 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyCallback> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyCallback2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pjob: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pjob: ::windows::core::RawPtr, perror: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pjob: ::windows::core::RawPtr, dwreserved: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pjob: ::windows::core::RawPtr, pfile: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBackgroundCopyCallback3(pub ::windows::core::IUnknown);
impl IBackgroundCopyCallback3 {
    pub unsafe fn JobTransferred<'a, Param0: ::windows::core::IntoParam<'a, IBackgroundCopyJob>>(&self, pjob: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pjob.into_param().abi()).ok()
    }
    pub unsafe fn JobError<'a, Param0: ::windows::core::IntoParam<'a, IBackgroundCopyJob>, Param1: ::windows::core::IntoParam<'a, IBackgroundCopyError>>(&self, pjob: Param0, perror: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pjob.into_param().abi(), perror.into_param().abi()).ok()
    }
    pub unsafe fn JobModification<'a, Param0: ::windows::core::IntoParam<'a, IBackgroundCopyJob>>(&self, pjob: Param0, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pjob.into_param().abi(), ::core::mem::transmute(dwreserved)).ok()
    }
    pub unsafe fn FileTransferred<'a, Param0: ::windows::core::IntoParam<'a, IBackgroundCopyJob>, Param1: ::windows::core::IntoParam<'a, IBackgroundCopyFile>>(&self, pjob: Param0, pfile: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pjob.into_param().abi(), pfile.into_param().abi()).ok()
    }
    pub unsafe fn FileRangesTransferred<'a, Param0: ::windows::core::IntoParam<'a, IBackgroundCopyJob>, Param1: ::windows::core::IntoParam<'a, IBackgroundCopyFile>>(&self, job: Param0, file: Param1, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), job.into_param().abi(), file.into_param().abi(), ::core::mem::transmute(rangecount), ::core::mem::transmute(ranges)).ok()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyCallback3 {
    type Vtable = IBackgroundCopyCallback3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98c97bd2_e32b_4ad8_a528_95fd8b16bd42);
}
impl ::core::convert::From<IBackgroundCopyCallback3> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyCallback3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBackgroundCopyCallback3> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyCallback3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundCopyCallback3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBackgroundCopyCallback3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IBackgroundCopyCallback3> for IBackgroundCopyCallback2 {
    fn from(value: IBackgroundCopyCallback3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyCallback3> for IBackgroundCopyCallback2 {
    fn from(value: &IBackgroundCopyCallback3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyCallback2> for IBackgroundCopyCallback3 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyCallback2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyCallback2> for &IBackgroundCopyCallback3 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyCallback2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundCopyCallback3> for IBackgroundCopyCallback {
    fn from(value: IBackgroundCopyCallback3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyCallback3> for IBackgroundCopyCallback {
    fn from(value: &IBackgroundCopyCallback3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyCallback> for IBackgroundCopyCallback3 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyCallback> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyCallback> for &IBackgroundCopyCallback3 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyCallback> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyCallback3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pjob: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pjob: ::windows::core::RawPtr, perror: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pjob: ::windows::core::RawPtr, dwreserved: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pjob: ::windows::core::RawPtr, pfile: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, job: ::windows::core::RawPtr, file: ::windows::core::RawPtr, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBackgroundCopyError(pub ::windows::core::IUnknown);
impl IBackgroundCopyError {
    pub unsafe fn GetError(&self, pcontext: *mut BG_ERROR_CONTEXT, pcode: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcontext), ::core::mem::transmute(pcode)).ok()
    }
    pub unsafe fn GetFile(&self) -> ::windows::core::Result<IBackgroundCopyFile> {
        let mut result__: <IBackgroundCopyFile as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IBackgroundCopyFile>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetErrorDescription(&self, languageid: u32) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(languageid), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetErrorContextDescription(&self, languageid: u32) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(languageid), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProtocol(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyError {
    type Vtable = IBackgroundCopyError_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19c613a0_fcb8_4f28_81ae_897c3d078f81);
}
impl ::core::convert::From<IBackgroundCopyError> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyError) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBackgroundCopyError> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyError) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundCopyError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBackgroundCopyError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyError_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcontext: *mut BG_ERROR_CONTEXT, pcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, languageid: u32, perrordescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, languageid: u32, pcontextdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pprotocol: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBackgroundCopyFile(pub ::windows::core::IUnknown);
impl IBackgroundCopyFile {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRemoteName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLocalName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProgress(&self) -> ::windows::core::Result<BG_FILE_PROGRESS> {
        let mut result__: <BG_FILE_PROGRESS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_FILE_PROGRESS>(result__)
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyFile {
    type Vtable = IBackgroundCopyFile_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01b7bd23_fb88_4a77_8490_5891d3e4653a);
}
impl ::core::convert::From<IBackgroundCopyFile> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyFile) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBackgroundCopyFile> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyFile) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundCopyFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBackgroundCopyFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyFile_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut BG_FILE_PROGRESS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBackgroundCopyFile2(pub ::windows::core::IUnknown);
impl IBackgroundCopyFile2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRemoteName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLocalName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProgress(&self) -> ::windows::core::Result<BG_FILE_PROGRESS> {
        let mut result__: <BG_FILE_PROGRESS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_FILE_PROGRESS>(result__)
    }
    pub unsafe fn GetFileRanges(&self, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(rangecount), ::core::mem::transmute(ranges)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRemoteName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, val: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyFile2 {
    type Vtable = IBackgroundCopyFile2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83e81b93_0873_474d_8a8c_f2018b1a939c);
}
impl ::core::convert::From<IBackgroundCopyFile2> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyFile2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBackgroundCopyFile2> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyFile2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundCopyFile2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBackgroundCopyFile2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IBackgroundCopyFile2> for IBackgroundCopyFile {
    fn from(value: IBackgroundCopyFile2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile2> for IBackgroundCopyFile {
    fn from(value: &IBackgroundCopyFile2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyFile> for IBackgroundCopyFile2 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyFile> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyFile> for &IBackgroundCopyFile2 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyFile> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyFile2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut BG_FILE_PROGRESS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, val: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBackgroundCopyFile3(pub ::windows::core::IUnknown);
impl IBackgroundCopyFile3 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRemoteName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLocalName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProgress(&self) -> ::windows::core::Result<BG_FILE_PROGRESS> {
        let mut result__: <BG_FILE_PROGRESS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_FILE_PROGRESS>(result__)
    }
    pub unsafe fn GetFileRanges(&self, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(rangecount), ::core::mem::transmute(ranges)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRemoteName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, val: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTemporaryName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValidationState<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, state: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), state.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetValidationState(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDownloadedFromPeer(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyFile3 {
    type Vtable = IBackgroundCopyFile3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x659cdeaa_489e_11d9_a9cd_000d56965251);
}
impl ::core::convert::From<IBackgroundCopyFile3> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyFile3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBackgroundCopyFile3> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyFile3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundCopyFile3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBackgroundCopyFile3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IBackgroundCopyFile3> for IBackgroundCopyFile2 {
    fn from(value: IBackgroundCopyFile3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile3> for IBackgroundCopyFile2 {
    fn from(value: &IBackgroundCopyFile3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyFile2> for IBackgroundCopyFile3 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyFile2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyFile2> for &IBackgroundCopyFile3 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyFile2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundCopyFile3> for IBackgroundCopyFile {
    fn from(value: IBackgroundCopyFile3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile3> for IBackgroundCopyFile {
    fn from(value: &IBackgroundCopyFile3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyFile> for IBackgroundCopyFile3 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyFile> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyFile> for &IBackgroundCopyFile3 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyFile> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyFile3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut BG_FILE_PROGRESS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, val: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfilename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, state: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBackgroundCopyFile4(pub ::windows::core::IUnknown);
impl IBackgroundCopyFile4 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRemoteName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLocalName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProgress(&self) -> ::windows::core::Result<BG_FILE_PROGRESS> {
        let mut result__: <BG_FILE_PROGRESS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_FILE_PROGRESS>(result__)
    }
    pub unsafe fn GetFileRanges(&self, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(rangecount), ::core::mem::transmute(ranges)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRemoteName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, val: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTemporaryName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValidationState<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, state: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), state.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetValidationState(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDownloadedFromPeer(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetPeerDownloadStats(&self, pfromorigin: *mut u64, pfrompeers: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfromorigin), ::core::mem::transmute(pfrompeers)).ok()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyFile4 {
    type Vtable = IBackgroundCopyFile4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef7e0655_7888_4960_b0e5_730846e03492);
}
impl ::core::convert::From<IBackgroundCopyFile4> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyFile4) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBackgroundCopyFile4> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyFile4) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundCopyFile4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBackgroundCopyFile4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IBackgroundCopyFile4> for IBackgroundCopyFile3 {
    fn from(value: IBackgroundCopyFile4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile4> for IBackgroundCopyFile3 {
    fn from(value: &IBackgroundCopyFile4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyFile3> for IBackgroundCopyFile4 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyFile3> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyFile3> for &IBackgroundCopyFile4 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyFile3> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundCopyFile4> for IBackgroundCopyFile2 {
    fn from(value: IBackgroundCopyFile4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile4> for IBackgroundCopyFile2 {
    fn from(value: &IBackgroundCopyFile4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyFile2> for IBackgroundCopyFile4 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyFile2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyFile2> for &IBackgroundCopyFile4 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyFile2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundCopyFile4> for IBackgroundCopyFile {
    fn from(value: IBackgroundCopyFile4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile4> for IBackgroundCopyFile {
    fn from(value: &IBackgroundCopyFile4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyFile> for IBackgroundCopyFile4 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyFile> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyFile> for &IBackgroundCopyFile4 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyFile> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyFile4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut BG_FILE_PROGRESS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, val: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfilename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, state: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfromorigin: *mut u64, pfrompeers: *mut u64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBackgroundCopyFile5(pub ::windows::core::IUnknown);
impl IBackgroundCopyFile5 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRemoteName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLocalName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProgress(&self) -> ::windows::core::Result<BG_FILE_PROGRESS> {
        let mut result__: <BG_FILE_PROGRESS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_FILE_PROGRESS>(result__)
    }
    pub unsafe fn GetFileRanges(&self, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(rangecount), ::core::mem::transmute(ranges)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRemoteName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, val: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTemporaryName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValidationState<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, state: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), state.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetValidationState(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDownloadedFromPeer(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetPeerDownloadStats(&self, pfromorigin: *mut u64, pfrompeers: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfromorigin), ::core::mem::transmute(pfrompeers)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProperty<'a, Param1: ::windows::core::IntoParam<'a, BITS_FILE_PROPERTY_VALUE>>(&self, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(propertyid), propertyvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperty(&self, propertyid: BITS_FILE_PROPERTY_ID) -> ::windows::core::Result<BITS_FILE_PROPERTY_VALUE> {
        let mut result__: <BITS_FILE_PROPERTY_VALUE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(propertyid), &mut result__).from_abi::<BITS_FILE_PROPERTY_VALUE>(result__)
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyFile5 {
    type Vtable = IBackgroundCopyFile5_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85c1657f_dafc_40e8_8834_df18ea25717e);
}
impl ::core::convert::From<IBackgroundCopyFile5> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyFile5) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBackgroundCopyFile5> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyFile5) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundCopyFile5 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBackgroundCopyFile5 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IBackgroundCopyFile5> for IBackgroundCopyFile4 {
    fn from(value: IBackgroundCopyFile5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile5> for IBackgroundCopyFile4 {
    fn from(value: &IBackgroundCopyFile5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyFile4> for IBackgroundCopyFile5 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyFile4> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyFile4> for &IBackgroundCopyFile5 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyFile4> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundCopyFile5> for IBackgroundCopyFile3 {
    fn from(value: IBackgroundCopyFile5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile5> for IBackgroundCopyFile3 {
    fn from(value: &IBackgroundCopyFile5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyFile3> for IBackgroundCopyFile5 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyFile3> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyFile3> for &IBackgroundCopyFile5 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyFile3> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundCopyFile5> for IBackgroundCopyFile2 {
    fn from(value: IBackgroundCopyFile5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile5> for IBackgroundCopyFile2 {
    fn from(value: &IBackgroundCopyFile5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyFile2> for IBackgroundCopyFile5 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyFile2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyFile2> for &IBackgroundCopyFile5 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyFile2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundCopyFile5> for IBackgroundCopyFile {
    fn from(value: IBackgroundCopyFile5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile5> for IBackgroundCopyFile {
    fn from(value: &IBackgroundCopyFile5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyFile> for IBackgroundCopyFile5 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyFile> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyFile> for &IBackgroundCopyFile5 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyFile> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyFile5_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut BG_FILE_PROGRESS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, val: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfilename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, state: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfromorigin: *mut u64, pfrompeers: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: BITS_FILE_PROPERTY_VALUE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: *mut BITS_FILE_PROPERTY_VALUE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBackgroundCopyFile6(pub ::windows::core::IUnknown);
impl IBackgroundCopyFile6 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRemoteName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLocalName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProgress(&self) -> ::windows::core::Result<BG_FILE_PROGRESS> {
        let mut result__: <BG_FILE_PROGRESS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_FILE_PROGRESS>(result__)
    }
    pub unsafe fn GetFileRanges(&self, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(rangecount), ::core::mem::transmute(ranges)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRemoteName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, val: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTemporaryName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValidationState<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, state: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), state.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetValidationState(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDownloadedFromPeer(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetPeerDownloadStats(&self, pfromorigin: *mut u64, pfrompeers: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfromorigin), ::core::mem::transmute(pfrompeers)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProperty<'a, Param1: ::windows::core::IntoParam<'a, BITS_FILE_PROPERTY_VALUE>>(&self, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(propertyid), propertyvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperty(&self, propertyid: BITS_FILE_PROPERTY_ID) -> ::windows::core::Result<BITS_FILE_PROPERTY_VALUE> {
        let mut result__: <BITS_FILE_PROPERTY_VALUE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(propertyid), &mut result__).from_abi::<BITS_FILE_PROPERTY_VALUE>(result__)
    }
    pub unsafe fn UpdateDownloadPosition(&self, offset: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(offset)).ok()
    }
    pub unsafe fn RequestFileRanges(&self, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(rangecount), ::core::mem::transmute(ranges)).ok()
    }
    pub unsafe fn GetFilledFileRanges(&self, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(rangecount), ::core::mem::transmute(ranges)).ok()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyFile6 {
    type Vtable = IBackgroundCopyFile6_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf6784f7_d677_49fd_9368_cb47aee9d1ad);
}
impl ::core::convert::From<IBackgroundCopyFile6> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyFile6) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBackgroundCopyFile6> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyFile6) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundCopyFile6 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBackgroundCopyFile6 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IBackgroundCopyFile6> for IBackgroundCopyFile5 {
    fn from(value: IBackgroundCopyFile6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile6> for IBackgroundCopyFile5 {
    fn from(value: &IBackgroundCopyFile6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyFile5> for IBackgroundCopyFile6 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyFile5> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyFile5> for &IBackgroundCopyFile6 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyFile5> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundCopyFile6> for IBackgroundCopyFile4 {
    fn from(value: IBackgroundCopyFile6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile6> for IBackgroundCopyFile4 {
    fn from(value: &IBackgroundCopyFile6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyFile4> for IBackgroundCopyFile6 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyFile4> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyFile4> for &IBackgroundCopyFile6 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyFile4> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundCopyFile6> for IBackgroundCopyFile3 {
    fn from(value: IBackgroundCopyFile6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile6> for IBackgroundCopyFile3 {
    fn from(value: &IBackgroundCopyFile6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyFile3> for IBackgroundCopyFile6 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyFile3> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyFile3> for &IBackgroundCopyFile6 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyFile3> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundCopyFile6> for IBackgroundCopyFile2 {
    fn from(value: IBackgroundCopyFile6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile6> for IBackgroundCopyFile2 {
    fn from(value: &IBackgroundCopyFile6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyFile2> for IBackgroundCopyFile6 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyFile2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyFile2> for &IBackgroundCopyFile6 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyFile2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundCopyFile6> for IBackgroundCopyFile {
    fn from(value: IBackgroundCopyFile6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile6> for IBackgroundCopyFile {
    fn from(value: &IBackgroundCopyFile6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyFile> for IBackgroundCopyFile6 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyFile> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyFile> for &IBackgroundCopyFile6 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyFile> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyFile6_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut BG_FILE_PROGRESS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, val: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfilename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, state: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfromorigin: *mut u64, pfrompeers: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: BITS_FILE_PROPERTY_VALUE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: *mut BITS_FILE_PROPERTY_VALUE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offset: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBackgroundCopyGroup(pub ::windows::core::IUnknown);
impl IBackgroundCopyGroup {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProp(&self, propid: GROUPPROP) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(propid), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProp(&self, propid: GROUPPROP, pvarval: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(propid), ::core::mem::transmute(pvarval)).ok()
    }
    pub unsafe fn GetProgress(&self, dwflags: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetStatus(&self, pdwstatus: *mut u32, pdwjobindex: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwstatus), ::core::mem::transmute(pdwjobindex)).ok()
    }
    pub unsafe fn GetJob<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, jobid: Param0) -> ::windows::core::Result<IBackgroundCopyJob1> {
        let mut result__: <IBackgroundCopyJob1 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), jobid.into_param().abi(), &mut result__).from_abi::<IBackgroundCopyJob1>(result__)
    }
    pub unsafe fn SuspendGroup(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ResumeGroup(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn CancelGroup(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Size(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GroupID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn CreateJob<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, guidjobid: Param0) -> ::windows::core::Result<IBackgroundCopyJob1> {
        let mut result__: <IBackgroundCopyJob1 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), guidjobid.into_param().abi(), &mut result__).from_abi::<IBackgroundCopyJob1>(result__)
    }
    pub unsafe fn EnumJobs(&self, dwflags: u32) -> ::windows::core::Result<IEnumBackgroundCopyJobs1> {
        let mut result__: <IEnumBackgroundCopyJobs1 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), &mut result__).from_abi::<IEnumBackgroundCopyJobs1>(result__)
    }
    pub unsafe fn SwitchToForeground(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn QueryNewJobInterface(&self, iid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(iid), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn SetNotificationPointer<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, iid: *const ::windows::core::GUID, punk: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(iid), punk.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyGroup {
    type Vtable = IBackgroundCopyGroup_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ded80a7_53ea_424f_8a04_17fea9adc4f5);
}
impl ::core::convert::From<IBackgroundCopyGroup> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyGroup) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBackgroundCopyGroup> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyGroup) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundCopyGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBackgroundCopyGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyGroup_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propid: GROUPPROP, pvarval: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propid: GROUPPROP, pvarval: *const ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwflags: u32, pdwprogress: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwstatus: *mut u32, pdwjobindex: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, jobid: ::windows::core::GUID, ppjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwsize: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pguidgroupid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, guidjobid: ::windows::core::GUID, ppjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwflags: u32, ppenumjobs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: *const ::windows::core::GUID, punk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: *const ::windows::core::GUID, punk: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBackgroundCopyJob(pub ::windows::core::IUnknown);
impl IBackgroundCopyJob {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddFileSet(&self, cfilecount: u32, pfileset: *const BG_FILE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cfilecount), ::core::mem::transmute(pfileset)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, remoteurl: Param0, localname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), remoteurl.into_param().abi(), localname.into_param().abi()).ok()
    }
    pub unsafe fn EnumFiles(&self) -> ::windows::core::Result<IEnumBackgroundCopyFiles> {
        let mut result__: <IEnumBackgroundCopyFiles as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumBackgroundCopyFiles>(result__)
    }
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Complete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<BG_JOB_TYPE> {
        let mut result__: <BG_JOB_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_JOB_TYPE>(result__)
    }
    pub unsafe fn GetProgress(&self) -> ::windows::core::Result<BG_JOB_PROGRESS> {
        let mut result__: <BG_JOB_PROGRESS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_JOB_PROGRESS>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTimes(&self) -> ::windows::core::Result<BG_JOB_TIMES> {
        let mut result__: <BG_JOB_TIMES as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_JOB_TIMES>(result__)
    }
    pub unsafe fn GetState(&self) -> ::windows::core::Result<BG_JOB_STATE> {
        let mut result__: <BG_JOB_STATE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_JOB_STATE>(result__)
    }
    pub unsafe fn GetError(&self) -> ::windows::core::Result<IBackgroundCopyError> {
        let mut result__: <IBackgroundCopyError as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IBackgroundCopyError>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, val: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, val: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn SetPriority(&self, val: BG_JOB_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(val)).ok()
    }
    pub unsafe fn GetPriority(&self) -> ::windows::core::Result<BG_JOB_PRIORITY> {
        let mut result__: <BG_JOB_PRIORITY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_JOB_PRIORITY>(result__)
    }
    pub unsafe fn SetNotifyFlags(&self, val: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(val)).ok()
    }
    pub unsafe fn GetNotifyFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetNotifyInterface<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, val: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
    pub unsafe fn GetNotifyInterface(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn SetMinimumRetryDelay(&self, seconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(seconds)).ok()
    }
    pub unsafe fn GetMinimumRetryDelay(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetNoProgressTimeout(&self, seconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(seconds)).ok()
    }
    pub unsafe fn GetNoProgressTimeout(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetErrorCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProxySettings<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, proxyusage: BG_JOB_PROXY_USAGE, proxylist: Param1, proxybypasslist: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(proxyusage), proxylist.into_param().abi(), proxybypasslist.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProxySettings(&self, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut super::super::Foundation::PWSTR, pproxybypasslist: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(pproxyusage), ::core::mem::transmute(pproxylist), ::core::mem::transmute(pproxybypasslist)).ok()
    }
    pub unsafe fn TakeOwnership(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyJob {
    type Vtable = IBackgroundCopyJob_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x37668d37_507e_4160_9316_26306d150b12);
}
impl ::core::convert::From<IBackgroundCopyJob> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyJob) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBackgroundCopyJob> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyJob) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundCopyJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBackgroundCopyJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJob_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cfilecount: u32, pfileset: *const BG_FILE_INFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, remoteurl: super::super::Foundation::PWSTR, localname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut BG_JOB_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut BG_JOB_PROGRESS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut BG_JOB_TIMES) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut BG_JOB_STATE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pperror: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, val: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, val: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, val: BG_JOB_PRIORITY) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut BG_JOB_PRIORITY) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, val: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, val: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, seconds: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, seconds: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, seconds: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, seconds: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, errors: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, proxyusage: BG_JOB_PROXY_USAGE, proxylist: super::super::Foundation::PWSTR, proxybypasslist: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut super::super::Foundation::PWSTR, pproxybypasslist: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBackgroundCopyJob1(pub ::windows::core::IUnknown);
impl IBackgroundCopyJob1 {
    pub unsafe fn CancelJob(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetProgress(&self, dwflags: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetStatus(&self, pdwstatus: *mut u32, pdwwin32result: *mut u32, pdwtransportresult: *mut u32, pdwnumofretries: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwstatus), ::core::mem::transmute(pdwwin32result), ::core::mem::transmute(pdwtransportresult), ::core::mem::transmute(pdwnumofretries)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddFiles(&self, cfilecount: u32, ppfileset: *const *const FILESETINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(cfilecount), ::core::mem::transmute(ppfileset)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFile(&self, cfileindex: u32) -> ::windows::core::Result<FILESETINFO> {
        let mut result__: <FILESETINFO as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(cfileindex), &mut result__).from_abi::<FILESETINFO>(result__)
    }
    pub unsafe fn GetFileCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SwitchToForeground(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn JobID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyJob1 {
    type Vtable = IBackgroundCopyJob1_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59f5553c_2031_4629_bb18_2645a6970947);
}
impl ::core::convert::From<IBackgroundCopyJob1> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyJob1) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBackgroundCopyJob1> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyJob1) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundCopyJob1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBackgroundCopyJob1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJob1_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwflags: u32, pdwprogress: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwstatus: *mut u32, pdwwin32result: *mut u32, pdwtransportresult: *mut u32, pdwnumofretries: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cfilecount: u32, ppfileset: *const *const FILESETINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cfileindex: u32, pfileinfo: *mut ::core::mem::ManuallyDrop<FILESETINFO>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwfilecount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pguidjobid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBackgroundCopyJob2(pub ::windows::core::IUnknown);
impl IBackgroundCopyJob2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddFileSet(&self, cfilecount: u32, pfileset: *const BG_FILE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cfilecount), ::core::mem::transmute(pfileset)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, remoteurl: Param0, localname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), remoteurl.into_param().abi(), localname.into_param().abi()).ok()
    }
    pub unsafe fn EnumFiles(&self) -> ::windows::core::Result<IEnumBackgroundCopyFiles> {
        let mut result__: <IEnumBackgroundCopyFiles as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumBackgroundCopyFiles>(result__)
    }
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Complete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<BG_JOB_TYPE> {
        let mut result__: <BG_JOB_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_JOB_TYPE>(result__)
    }
    pub unsafe fn GetProgress(&self) -> ::windows::core::Result<BG_JOB_PROGRESS> {
        let mut result__: <BG_JOB_PROGRESS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_JOB_PROGRESS>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTimes(&self) -> ::windows::core::Result<BG_JOB_TIMES> {
        let mut result__: <BG_JOB_TIMES as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_JOB_TIMES>(result__)
    }
    pub unsafe fn GetState(&self) -> ::windows::core::Result<BG_JOB_STATE> {
        let mut result__: <BG_JOB_STATE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_JOB_STATE>(result__)
    }
    pub unsafe fn GetError(&self) -> ::windows::core::Result<IBackgroundCopyError> {
        let mut result__: <IBackgroundCopyError as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IBackgroundCopyError>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, val: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, val: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn SetPriority(&self, val: BG_JOB_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(val)).ok()
    }
    pub unsafe fn GetPriority(&self) -> ::windows::core::Result<BG_JOB_PRIORITY> {
        let mut result__: <BG_JOB_PRIORITY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_JOB_PRIORITY>(result__)
    }
    pub unsafe fn SetNotifyFlags(&self, val: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(val)).ok()
    }
    pub unsafe fn GetNotifyFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetNotifyInterface<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, val: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
    pub unsafe fn GetNotifyInterface(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn SetMinimumRetryDelay(&self, seconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(seconds)).ok()
    }
    pub unsafe fn GetMinimumRetryDelay(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetNoProgressTimeout(&self, seconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(seconds)).ok()
    }
    pub unsafe fn GetNoProgressTimeout(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetErrorCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProxySettings<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, proxyusage: BG_JOB_PROXY_USAGE, proxylist: Param1, proxybypasslist: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(proxyusage), proxylist.into_param().abi(), proxybypasslist.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProxySettings(&self, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut super::super::Foundation::PWSTR, pproxybypasslist: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(pproxyusage), ::core::mem::transmute(pproxylist), ::core::mem::transmute(pproxybypasslist)).ok()
    }
    pub unsafe fn TakeOwnership(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNotifyCmdLine<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, program: Param0, parameters: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), program.into_param().abi(), parameters.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNotifyCmdLine(&self, pprogram: *mut super::super::Foundation::PWSTR, pparameters: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprogram), ::core::mem::transmute(pparameters)).ok()
    }
    pub unsafe fn GetReplyProgress(&self, pprogress: *mut BG_JOB_REPLY_PROGRESS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprogress)).ok()
    }
    pub unsafe fn GetReplyData(&self, ppbuffer: *mut *mut u8, plength: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(plength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReplyFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, replyfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), replyfilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReplyFileName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCredentials(&self, credentials: *const BG_AUTH_CREDENTIALS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(credentials)).ok()
    }
    pub unsafe fn RemoveCredentials(&self, target: BG_AUTH_TARGET, scheme: BG_AUTH_SCHEME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(target), ::core::mem::transmute(scheme)).ok()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyJob2 {
    type Vtable = IBackgroundCopyJob2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54b50739_686f_45eb_9dff_d6a9a0faa9af);
}
impl ::core::convert::From<IBackgroundCopyJob2> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyJob2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBackgroundCopyJob2> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyJob2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundCopyJob2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBackgroundCopyJob2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IBackgroundCopyJob2> for IBackgroundCopyJob {
    fn from(value: IBackgroundCopyJob2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJob2> for IBackgroundCopyJob {
    fn from(value: &IBackgroundCopyJob2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyJob> for IBackgroundCopyJob2 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyJob> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyJob> for &IBackgroundCopyJob2 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyJob> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJob2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cfilecount: u32, pfileset: *const BG_FILE_INFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, remoteurl: super::super::Foundation::PWSTR, localname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut BG_JOB_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut BG_JOB_PROGRESS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut BG_JOB_TIMES) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut BG_JOB_STATE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pperror: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, val: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, val: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, val: BG_JOB_PRIORITY) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut BG_JOB_PRIORITY) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, val: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, val: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, seconds: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, seconds: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, seconds: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, seconds: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, errors: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, proxyusage: BG_JOB_PROXY_USAGE, proxylist: super::super::Foundation::PWSTR, proxybypasslist: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut super::super::Foundation::PWSTR, pproxybypasslist: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, program: super::super::Foundation::PWSTR, parameters: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pprogram: *mut super::super::Foundation::PWSTR, pparameters: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pprogress: *mut BG_JOB_REPLY_PROGRESS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppbuffer: *mut *mut u8, plength: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, replyfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, preplyfilename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, credentials: *const BG_AUTH_CREDENTIALS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: BG_AUTH_TARGET, scheme: BG_AUTH_SCHEME) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBackgroundCopyJob3(pub ::windows::core::IUnknown);
impl IBackgroundCopyJob3 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddFileSet(&self, cfilecount: u32, pfileset: *const BG_FILE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cfilecount), ::core::mem::transmute(pfileset)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, remoteurl: Param0, localname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), remoteurl.into_param().abi(), localname.into_param().abi()).ok()
    }
    pub unsafe fn EnumFiles(&self) -> ::windows::core::Result<IEnumBackgroundCopyFiles> {
        let mut result__: <IEnumBackgroundCopyFiles as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumBackgroundCopyFiles>(result__)
    }
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Complete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<BG_JOB_TYPE> {
        let mut result__: <BG_JOB_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_JOB_TYPE>(result__)
    }
    pub unsafe fn GetProgress(&self) -> ::windows::core::Result<BG_JOB_PROGRESS> {
        let mut result__: <BG_JOB_PROGRESS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_JOB_PROGRESS>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTimes(&self) -> ::windows::core::Result<BG_JOB_TIMES> {
        let mut result__: <BG_JOB_TIMES as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_JOB_TIMES>(result__)
    }
    pub unsafe fn GetState(&self) -> ::windows::core::Result<BG_JOB_STATE> {
        let mut result__: <BG_JOB_STATE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_JOB_STATE>(result__)
    }
    pub unsafe fn GetError(&self) -> ::windows::core::Result<IBackgroundCopyError> {
        let mut result__: <IBackgroundCopyError as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IBackgroundCopyError>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, val: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, val: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn SetPriority(&self, val: BG_JOB_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(val)).ok()
    }
    pub unsafe fn GetPriority(&self) -> ::windows::core::Result<BG_JOB_PRIORITY> {
        let mut result__: <BG_JOB_PRIORITY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_JOB_PRIORITY>(result__)
    }
    pub unsafe fn SetNotifyFlags(&self, val: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(val)).ok()
    }
    pub unsafe fn GetNotifyFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetNotifyInterface<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, val: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
    pub unsafe fn GetNotifyInterface(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn SetMinimumRetryDelay(&self, seconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(seconds)).ok()
    }
    pub unsafe fn GetMinimumRetryDelay(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetNoProgressTimeout(&self, seconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(seconds)).ok()
    }
    pub unsafe fn GetNoProgressTimeout(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetErrorCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProxySettings<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, proxyusage: BG_JOB_PROXY_USAGE, proxylist: Param1, proxybypasslist: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(proxyusage), proxylist.into_param().abi(), proxybypasslist.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProxySettings(&self, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut super::super::Foundation::PWSTR, pproxybypasslist: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(pproxyusage), ::core::mem::transmute(pproxylist), ::core::mem::transmute(pproxybypasslist)).ok()
    }
    pub unsafe fn TakeOwnership(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNotifyCmdLine<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, program: Param0, parameters: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), program.into_param().abi(), parameters.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNotifyCmdLine(&self, pprogram: *mut super::super::Foundation::PWSTR, pparameters: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprogram), ::core::mem::transmute(pparameters)).ok()
    }
    pub unsafe fn GetReplyProgress(&self, pprogress: *mut BG_JOB_REPLY_PROGRESS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprogress)).ok()
    }
    pub unsafe fn GetReplyData(&self, ppbuffer: *mut *mut u8, plength: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(plength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReplyFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, replyfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), replyfilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReplyFileName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCredentials(&self, credentials: *const BG_AUTH_CREDENTIALS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(credentials)).ok()
    }
    pub unsafe fn RemoveCredentials(&self, target: BG_AUTH_TARGET, scheme: BG_AUTH_SCHEME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(target), ::core::mem::transmute(scheme)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReplaceRemotePrefix<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, oldprefix: Param0, newprefix: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), oldprefix.into_param().abi(), newprefix.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddFileWithRanges<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, remoteurl: Param0, localname: Param1, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), remoteurl.into_param().abi(), localname.into_param().abi(), ::core::mem::transmute(rangecount), ::core::mem::transmute(ranges)).ok()
    }
    pub unsafe fn SetFileACLFlags(&self, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
    pub unsafe fn GetFileACLFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyJob3 {
    type Vtable = IBackgroundCopyJob3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x443c8934_90ff_48ed_bcde_26f5c7450042);
}
impl ::core::convert::From<IBackgroundCopyJob3> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyJob3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBackgroundCopyJob3> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyJob3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundCopyJob3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBackgroundCopyJob3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IBackgroundCopyJob3> for IBackgroundCopyJob2 {
    fn from(value: IBackgroundCopyJob3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJob3> for IBackgroundCopyJob2 {
    fn from(value: &IBackgroundCopyJob3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyJob2> for IBackgroundCopyJob3 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyJob2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyJob2> for &IBackgroundCopyJob3 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyJob2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundCopyJob3> for IBackgroundCopyJob {
    fn from(value: IBackgroundCopyJob3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJob3> for IBackgroundCopyJob {
    fn from(value: &IBackgroundCopyJob3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyJob> for IBackgroundCopyJob3 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyJob> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyJob> for &IBackgroundCopyJob3 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyJob> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJob3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cfilecount: u32, pfileset: *const BG_FILE_INFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, remoteurl: super::super::Foundation::PWSTR, localname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut BG_JOB_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut BG_JOB_PROGRESS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut BG_JOB_TIMES) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut BG_JOB_STATE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pperror: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, val: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, val: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, val: BG_JOB_PRIORITY) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut BG_JOB_PRIORITY) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, val: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, val: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, seconds: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, seconds: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, seconds: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, seconds: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, errors: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, proxyusage: BG_JOB_PROXY_USAGE, proxylist: super::super::Foundation::PWSTR, proxybypasslist: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut super::super::Foundation::PWSTR, pproxybypasslist: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, program: super::super::Foundation::PWSTR, parameters: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pprogram: *mut super::super::Foundation::PWSTR, pparameters: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pprogress: *mut BG_JOB_REPLY_PROGRESS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppbuffer: *mut *mut u8, plength: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, replyfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, preplyfilename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, credentials: *const BG_AUTH_CREDENTIALS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: BG_AUTH_TARGET, scheme: BG_AUTH_SCHEME) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, oldprefix: super::super::Foundation::PWSTR, newprefix: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, remoteurl: super::super::Foundation::PWSTR, localname: super::super::Foundation::PWSTR, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flags: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBackgroundCopyJob4(pub ::windows::core::IUnknown);
impl IBackgroundCopyJob4 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddFileSet(&self, cfilecount: u32, pfileset: *const BG_FILE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cfilecount), ::core::mem::transmute(pfileset)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, remoteurl: Param0, localname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), remoteurl.into_param().abi(), localname.into_param().abi()).ok()
    }
    pub unsafe fn EnumFiles(&self) -> ::windows::core::Result<IEnumBackgroundCopyFiles> {
        let mut result__: <IEnumBackgroundCopyFiles as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumBackgroundCopyFiles>(result__)
    }
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Complete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<BG_JOB_TYPE> {
        let mut result__: <BG_JOB_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_JOB_TYPE>(result__)
    }
    pub unsafe fn GetProgress(&self) -> ::windows::core::Result<BG_JOB_PROGRESS> {
        let mut result__: <BG_JOB_PROGRESS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_JOB_PROGRESS>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTimes(&self) -> ::windows::core::Result<BG_JOB_TIMES> {
        let mut result__: <BG_JOB_TIMES as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_JOB_TIMES>(result__)
    }
    pub unsafe fn GetState(&self) -> ::windows::core::Result<BG_JOB_STATE> {
        let mut result__: <BG_JOB_STATE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_JOB_STATE>(result__)
    }
    pub unsafe fn GetError(&self) -> ::windows::core::Result<IBackgroundCopyError> {
        let mut result__: <IBackgroundCopyError as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IBackgroundCopyError>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, val: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, val: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn SetPriority(&self, val: BG_JOB_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(val)).ok()
    }
    pub unsafe fn GetPriority(&self) -> ::windows::core::Result<BG_JOB_PRIORITY> {
        let mut result__: <BG_JOB_PRIORITY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_JOB_PRIORITY>(result__)
    }
    pub unsafe fn SetNotifyFlags(&self, val: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(val)).ok()
    }
    pub unsafe fn GetNotifyFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetNotifyInterface<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, val: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
    pub unsafe fn GetNotifyInterface(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn SetMinimumRetryDelay(&self, seconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(seconds)).ok()
    }
    pub unsafe fn GetMinimumRetryDelay(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetNoProgressTimeout(&self, seconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(seconds)).ok()
    }
    pub unsafe fn GetNoProgressTimeout(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetErrorCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProxySettings<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, proxyusage: BG_JOB_PROXY_USAGE, proxylist: Param1, proxybypasslist: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(proxyusage), proxylist.into_param().abi(), proxybypasslist.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProxySettings(&self, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut super::super::Foundation::PWSTR, pproxybypasslist: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(pproxyusage), ::core::mem::transmute(pproxylist), ::core::mem::transmute(pproxybypasslist)).ok()
    }
    pub unsafe fn TakeOwnership(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNotifyCmdLine<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, program: Param0, parameters: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), program.into_param().abi(), parameters.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNotifyCmdLine(&self, pprogram: *mut super::super::Foundation::PWSTR, pparameters: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprogram), ::core::mem::transmute(pparameters)).ok()
    }
    pub unsafe fn GetReplyProgress(&self, pprogress: *mut BG_JOB_REPLY_PROGRESS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprogress)).ok()
    }
    pub unsafe fn GetReplyData(&self, ppbuffer: *mut *mut u8, plength: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(plength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReplyFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, replyfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), replyfilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReplyFileName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCredentials(&self, credentials: *const BG_AUTH_CREDENTIALS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(credentials)).ok()
    }
    pub unsafe fn RemoveCredentials(&self, target: BG_AUTH_TARGET, scheme: BG_AUTH_SCHEME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(target), ::core::mem::transmute(scheme)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReplaceRemotePrefix<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, oldprefix: Param0, newprefix: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), oldprefix.into_param().abi(), newprefix.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddFileWithRanges<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, remoteurl: Param0, localname: Param1, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), remoteurl.into_param().abi(), localname.into_param().abi(), ::core::mem::transmute(rangecount), ::core::mem::transmute(ranges)).ok()
    }
    pub unsafe fn SetFileACLFlags(&self, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
    pub unsafe fn GetFileACLFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetPeerCachingFlags(&self, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
    pub unsafe fn GetPeerCachingFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOwnerIntegrityLevel(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOwnerElevationState(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn SetMaximumDownloadTime(&self, timeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(timeout)).ok()
    }
    pub unsafe fn GetMaximumDownloadTime(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyJob4 {
    type Vtable = IBackgroundCopyJob4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x659cdeae_489e_11d9_a9cd_000d56965251);
}
impl ::core::convert::From<IBackgroundCopyJob4> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyJob4) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBackgroundCopyJob4> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyJob4) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundCopyJob4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBackgroundCopyJob4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IBackgroundCopyJob4> for IBackgroundCopyJob3 {
    fn from(value: IBackgroundCopyJob4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJob4> for IBackgroundCopyJob3 {
    fn from(value: &IBackgroundCopyJob4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyJob3> for IBackgroundCopyJob4 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyJob3> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyJob3> for &IBackgroundCopyJob4 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyJob3> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundCopyJob4> for IBackgroundCopyJob2 {
    fn from(value: IBackgroundCopyJob4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJob4> for IBackgroundCopyJob2 {
    fn from(value: &IBackgroundCopyJob4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyJob2> for IBackgroundCopyJob4 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyJob2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyJob2> for &IBackgroundCopyJob4 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyJob2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundCopyJob4> for IBackgroundCopyJob {
    fn from(value: IBackgroundCopyJob4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJob4> for IBackgroundCopyJob {
    fn from(value: &IBackgroundCopyJob4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyJob> for IBackgroundCopyJob4 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyJob> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyJob> for &IBackgroundCopyJob4 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyJob> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJob4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cfilecount: u32, pfileset: *const BG_FILE_INFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, remoteurl: super::super::Foundation::PWSTR, localname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut BG_JOB_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut BG_JOB_PROGRESS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut BG_JOB_TIMES) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut BG_JOB_STATE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pperror: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, val: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, val: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, val: BG_JOB_PRIORITY) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut BG_JOB_PRIORITY) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, val: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, val: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, seconds: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, seconds: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, seconds: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, seconds: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, errors: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, proxyusage: BG_JOB_PROXY_USAGE, proxylist: super::super::Foundation::PWSTR, proxybypasslist: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut super::super::Foundation::PWSTR, pproxybypasslist: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, program: super::super::Foundation::PWSTR, parameters: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pprogram: *mut super::super::Foundation::PWSTR, pparameters: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pprogress: *mut BG_JOB_REPLY_PROGRESS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppbuffer: *mut *mut u8, plength: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, replyfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, preplyfilename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, credentials: *const BG_AUTH_CREDENTIALS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: BG_AUTH_TARGET, scheme: BG_AUTH_SCHEME) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, oldprefix: super::super::Foundation::PWSTR, newprefix: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, remoteurl: super::super::Foundation::PWSTR, localname: super::super::Foundation::PWSTR, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flags: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pflags: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plevel: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pelevated: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timeout: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptimeout: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBackgroundCopyJob5(pub ::windows::core::IUnknown);
impl IBackgroundCopyJob5 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddFileSet(&self, cfilecount: u32, pfileset: *const BG_FILE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cfilecount), ::core::mem::transmute(pfileset)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, remoteurl: Param0, localname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), remoteurl.into_param().abi(), localname.into_param().abi()).ok()
    }
    pub unsafe fn EnumFiles(&self) -> ::windows::core::Result<IEnumBackgroundCopyFiles> {
        let mut result__: <IEnumBackgroundCopyFiles as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumBackgroundCopyFiles>(result__)
    }
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Complete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<BG_JOB_TYPE> {
        let mut result__: <BG_JOB_TYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_JOB_TYPE>(result__)
    }
    pub unsafe fn GetProgress(&self) -> ::windows::core::Result<BG_JOB_PROGRESS> {
        let mut result__: <BG_JOB_PROGRESS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_JOB_PROGRESS>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTimes(&self) -> ::windows::core::Result<BG_JOB_TIMES> {
        let mut result__: <BG_JOB_TIMES as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_JOB_TIMES>(result__)
    }
    pub unsafe fn GetState(&self) -> ::windows::core::Result<BG_JOB_STATE> {
        let mut result__: <BG_JOB_STATE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_JOB_STATE>(result__)
    }
    pub unsafe fn GetError(&self) -> ::windows::core::Result<IBackgroundCopyError> {
        let mut result__: <IBackgroundCopyError as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IBackgroundCopyError>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, val: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, val: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn SetPriority(&self, val: BG_JOB_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(val)).ok()
    }
    pub unsafe fn GetPriority(&self) -> ::windows::core::Result<BG_JOB_PRIORITY> {
        let mut result__: <BG_JOB_PRIORITY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_JOB_PRIORITY>(result__)
    }
    pub unsafe fn SetNotifyFlags(&self, val: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(val)).ok()
    }
    pub unsafe fn GetNotifyFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetNotifyInterface<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, val: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), val.into_param().abi()).ok()
    }
    pub unsafe fn GetNotifyInterface(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn SetMinimumRetryDelay(&self, seconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(seconds)).ok()
    }
    pub unsafe fn GetMinimumRetryDelay(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetNoProgressTimeout(&self, seconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(seconds)).ok()
    }
    pub unsafe fn GetNoProgressTimeout(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetErrorCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProxySettings<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, proxyusage: BG_JOB_PROXY_USAGE, proxylist: Param1, proxybypasslist: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(proxyusage), proxylist.into_param().abi(), proxybypasslist.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProxySettings(&self, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut super::super::Foundation::PWSTR, pproxybypasslist: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(pproxyusage), ::core::mem::transmute(pproxylist), ::core::mem::transmute(pproxybypasslist)).ok()
    }
    pub unsafe fn TakeOwnership(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNotifyCmdLine<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, program: Param0, parameters: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), program.into_param().abi(), parameters.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNotifyCmdLine(&self, pprogram: *mut super::super::Foundation::PWSTR, pparameters: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprogram), ::core::mem::transmute(pparameters)).ok()
    }
    pub unsafe fn GetReplyProgress(&self, pprogress: *mut BG_JOB_REPLY_PROGRESS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprogress)).ok()
    }
    pub unsafe fn GetReplyData(&self, ppbuffer: *mut *mut u8, plength: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(plength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReplyFileName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, replyfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), replyfilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReplyFileName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCredentials(&self, credentials: *const BG_AUTH_CREDENTIALS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(credentials)).ok()
    }
    pub unsafe fn RemoveCredentials(&self, target: BG_AUTH_TARGET, scheme: BG_AUTH_SCHEME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(target), ::core::mem::transmute(scheme)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReplaceRemotePrefix<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, oldprefix: Param0, newprefix: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), oldprefix.into_param().abi(), newprefix.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddFileWithRanges<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, remoteurl: Param0, localname: Param1, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), remoteurl.into_param().abi(), localname.into_param().abi(), ::core::mem::transmute(rangecount), ::core::mem::transmute(ranges)).ok()
    }
    pub unsafe fn SetFileACLFlags(&self, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
    pub unsafe fn GetFileACLFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetPeerCachingFlags(&self, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
    pub unsafe fn GetPeerCachingFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOwnerIntegrityLevel(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOwnerElevationState(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn SetMaximumDownloadTime(&self, timeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).51)(::core::mem::transmute_copy(self), ::core::mem::transmute(timeout)).ok()
    }
    pub unsafe fn GetMaximumDownloadTime(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).52)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProperty<'a, Param1: ::windows::core::IntoParam<'a, BITS_JOB_PROPERTY_VALUE>>(&self, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).53)(::core::mem::transmute_copy(self), ::core::mem::transmute(propertyid), propertyvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperty(&self, propertyid: BITS_JOB_PROPERTY_ID) -> ::windows::core::Result<BITS_JOB_PROPERTY_VALUE> {
        let mut result__: <BITS_JOB_PROPERTY_VALUE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).54)(::core::mem::transmute_copy(self), ::core::mem::transmute(propertyid), &mut result__).from_abi::<BITS_JOB_PROPERTY_VALUE>(result__)
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyJob5 {
    type Vtable = IBackgroundCopyJob5_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe847030c_bbba_4657_af6d_484aa42bf1fe);
}
impl ::core::convert::From<IBackgroundCopyJob5> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyJob5) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBackgroundCopyJob5> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyJob5) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundCopyJob5 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBackgroundCopyJob5 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IBackgroundCopyJob5> for IBackgroundCopyJob4 {
    fn from(value: IBackgroundCopyJob5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJob5> for IBackgroundCopyJob4 {
    fn from(value: &IBackgroundCopyJob5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyJob4> for IBackgroundCopyJob5 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyJob4> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyJob4> for &IBackgroundCopyJob5 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyJob4> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundCopyJob5> for IBackgroundCopyJob3 {
    fn from(value: IBackgroundCopyJob5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJob5> for IBackgroundCopyJob3 {
    fn from(value: &IBackgroundCopyJob5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyJob3> for IBackgroundCopyJob5 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyJob3> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyJob3> for &IBackgroundCopyJob5 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyJob3> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundCopyJob5> for IBackgroundCopyJob2 {
    fn from(value: IBackgroundCopyJob5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJob5> for IBackgroundCopyJob2 {
    fn from(value: &IBackgroundCopyJob5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyJob2> for IBackgroundCopyJob5 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyJob2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyJob2> for &IBackgroundCopyJob5 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyJob2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundCopyJob5> for IBackgroundCopyJob {
    fn from(value: IBackgroundCopyJob5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJob5> for IBackgroundCopyJob {
    fn from(value: &IBackgroundCopyJob5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyJob> for IBackgroundCopyJob5 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyJob> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyJob> for &IBackgroundCopyJob5 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyJob> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJob5_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cfilecount: u32, pfileset: *const BG_FILE_INFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, remoteurl: super::super::Foundation::PWSTR, localname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut BG_JOB_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut BG_JOB_PROGRESS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut BG_JOB_TIMES) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut BG_JOB_STATE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pperror: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, val: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, val: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, val: BG_JOB_PRIORITY) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut BG_JOB_PRIORITY) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, val: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, val: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, seconds: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, seconds: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, seconds: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, seconds: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, errors: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, proxyusage: BG_JOB_PROXY_USAGE, proxylist: super::super::Foundation::PWSTR, proxybypasslist: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut super::super::Foundation::PWSTR, pproxybypasslist: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, program: super::super::Foundation::PWSTR, parameters: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pprogram: *mut super::super::Foundation::PWSTR, pparameters: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pprogress: *mut BG_JOB_REPLY_PROGRESS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppbuffer: *mut *mut u8, plength: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, replyfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, preplyfilename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, credentials: *const BG_AUTH_CREDENTIALS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: BG_AUTH_TARGET, scheme: BG_AUTH_SCHEME) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, oldprefix: super::super::Foundation::PWSTR, newprefix: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, remoteurl: super::super::Foundation::PWSTR, localname: super::super::Foundation::PWSTR, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flags: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pflags: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plevel: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pelevated: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timeout: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptimeout: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: BITS_JOB_PROPERTY_VALUE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: *mut BITS_JOB_PROPERTY_VALUE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBackgroundCopyJobHttpOptions(pub ::windows::core::IUnknown);
impl IBackgroundCopyJobHttpOptions {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientCertificateByID<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, storelocation: BG_CERT_STORE_LOCATION, storename: Param1, pcerthashblob: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(storelocation), storename.into_param().abi(), ::core::mem::transmute(pcerthashblob)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientCertificateByName<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, storelocation: BG_CERT_STORE_LOCATION, storename: Param1, subjectname: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(storelocation), storename.into_param().abi(), subjectname.into_param().abi()).ok()
    }
    pub unsafe fn RemoveClientCertificate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetClientCertificate(&self, pstorelocation: *mut BG_CERT_STORE_LOCATION, pstorename: *mut super::super::Foundation::PWSTR, ppcerthashblob: *mut *mut u8, psubjectname: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstorelocation), ::core::mem::transmute(pstorename), ::core::mem::transmute(ppcerthashblob), ::core::mem::transmute(psubjectname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCustomHeaders<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, requestheaders: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), requestheaders.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCustomHeaders(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn SetSecurityFlags(&self, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
    pub unsafe fn GetSecurityFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyJobHttpOptions {
    type Vtable = IBackgroundCopyJobHttpOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1bd1079_9f01_4bdc_8036_f09b70095066);
}
impl ::core::convert::From<IBackgroundCopyJobHttpOptions> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyJobHttpOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBackgroundCopyJobHttpOptions> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyJobHttpOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundCopyJobHttpOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBackgroundCopyJobHttpOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJobHttpOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, storelocation: BG_CERT_STORE_LOCATION, storename: super::super::Foundation::PWSTR, pcerthashblob: *const u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, storelocation: BG_CERT_STORE_LOCATION, storename: super::super::Foundation::PWSTR, subjectname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstorelocation: *mut BG_CERT_STORE_LOCATION, pstorename: *mut super::super::Foundation::PWSTR, ppcerthashblob: *mut *mut u8, psubjectname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, requestheaders: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, prequestheaders: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pflags: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBackgroundCopyJobHttpOptions2(pub ::windows::core::IUnknown);
impl IBackgroundCopyJobHttpOptions2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientCertificateByID<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, storelocation: BG_CERT_STORE_LOCATION, storename: Param1, pcerthashblob: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(storelocation), storename.into_param().abi(), ::core::mem::transmute(pcerthashblob)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientCertificateByName<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, storelocation: BG_CERT_STORE_LOCATION, storename: Param1, subjectname: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(storelocation), storename.into_param().abi(), subjectname.into_param().abi()).ok()
    }
    pub unsafe fn RemoveClientCertificate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetClientCertificate(&self, pstorelocation: *mut BG_CERT_STORE_LOCATION, pstorename: *mut super::super::Foundation::PWSTR, ppcerthashblob: *mut *mut u8, psubjectname: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstorelocation), ::core::mem::transmute(pstorename), ::core::mem::transmute(ppcerthashblob), ::core::mem::transmute(psubjectname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCustomHeaders<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, requestheaders: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), requestheaders.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCustomHeaders(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn SetSecurityFlags(&self, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
    pub unsafe fn GetSecurityFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetHttpMethod<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, method: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), method.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHttpMethod(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyJobHttpOptions2 {
    type Vtable = IBackgroundCopyJobHttpOptions2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb591a192_a405_4fc3_8323_4c5c542578fc);
}
impl ::core::convert::From<IBackgroundCopyJobHttpOptions2> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyJobHttpOptions2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBackgroundCopyJobHttpOptions2> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyJobHttpOptions2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundCopyJobHttpOptions2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBackgroundCopyJobHttpOptions2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IBackgroundCopyJobHttpOptions2> for IBackgroundCopyJobHttpOptions {
    fn from(value: IBackgroundCopyJobHttpOptions2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJobHttpOptions2> for IBackgroundCopyJobHttpOptions {
    fn from(value: &IBackgroundCopyJobHttpOptions2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyJobHttpOptions> for IBackgroundCopyJobHttpOptions2 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyJobHttpOptions> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyJobHttpOptions> for &IBackgroundCopyJobHttpOptions2 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyJobHttpOptions> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJobHttpOptions2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, storelocation: BG_CERT_STORE_LOCATION, storename: super::super::Foundation::PWSTR, pcerthashblob: *const u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, storelocation: BG_CERT_STORE_LOCATION, storename: super::super::Foundation::PWSTR, subjectname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstorelocation: *mut BG_CERT_STORE_LOCATION, pstorename: *mut super::super::Foundation::PWSTR, ppcerthashblob: *mut *mut u8, psubjectname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, requestheaders: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, prequestheaders: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pflags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, method: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, method: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBackgroundCopyJobHttpOptions3(pub ::windows::core::IUnknown);
impl IBackgroundCopyJobHttpOptions3 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientCertificateByID<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, storelocation: BG_CERT_STORE_LOCATION, storename: Param1, pcerthashblob: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(storelocation), storename.into_param().abi(), ::core::mem::transmute(pcerthashblob)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientCertificateByName<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, storelocation: BG_CERT_STORE_LOCATION, storename: Param1, subjectname: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(storelocation), storename.into_param().abi(), subjectname.into_param().abi()).ok()
    }
    pub unsafe fn RemoveClientCertificate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetClientCertificate(&self, pstorelocation: *mut BG_CERT_STORE_LOCATION, pstorename: *mut super::super::Foundation::PWSTR, ppcerthashblob: *mut *mut u8, psubjectname: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstorelocation), ::core::mem::transmute(pstorename), ::core::mem::transmute(ppcerthashblob), ::core::mem::transmute(psubjectname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCustomHeaders<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, requestheaders: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), requestheaders.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCustomHeaders(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn SetSecurityFlags(&self, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
    pub unsafe fn GetSecurityFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetHttpMethod<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, method: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), method.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHttpMethod(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn SetServerCertificateValidationInterface<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, certvalidationcallback: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), certvalidationcallback.into_param().abi()).ok()
    }
    pub unsafe fn MakeCustomHeadersWriteOnly(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyJobHttpOptions3 {
    type Vtable = IBackgroundCopyJobHttpOptions3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a9263d3_fd4c_4eda_9b28_30132a4d4e3c);
}
impl ::core::convert::From<IBackgroundCopyJobHttpOptions3> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyJobHttpOptions3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBackgroundCopyJobHttpOptions3> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyJobHttpOptions3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundCopyJobHttpOptions3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBackgroundCopyJobHttpOptions3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IBackgroundCopyJobHttpOptions3> for IBackgroundCopyJobHttpOptions2 {
    fn from(value: IBackgroundCopyJobHttpOptions3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJobHttpOptions3> for IBackgroundCopyJobHttpOptions2 {
    fn from(value: &IBackgroundCopyJobHttpOptions3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyJobHttpOptions2> for IBackgroundCopyJobHttpOptions3 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyJobHttpOptions2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyJobHttpOptions2> for &IBackgroundCopyJobHttpOptions3 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyJobHttpOptions2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundCopyJobHttpOptions3> for IBackgroundCopyJobHttpOptions {
    fn from(value: IBackgroundCopyJobHttpOptions3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJobHttpOptions3> for IBackgroundCopyJobHttpOptions {
    fn from(value: &IBackgroundCopyJobHttpOptions3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyJobHttpOptions> for IBackgroundCopyJobHttpOptions3 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyJobHttpOptions> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundCopyJobHttpOptions> for &IBackgroundCopyJobHttpOptions3 {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundCopyJobHttpOptions> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJobHttpOptions3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, storelocation: BG_CERT_STORE_LOCATION, storename: super::super::Foundation::PWSTR, pcerthashblob: *const u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, storelocation: BG_CERT_STORE_LOCATION, storename: super::super::Foundation::PWSTR, subjectname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstorelocation: *mut BG_CERT_STORE_LOCATION, pstorename: *mut super::super::Foundation::PWSTR, ppcerthashblob: *mut *mut u8, psubjectname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, requestheaders: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, prequestheaders: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pflags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, method: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, method: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, certvalidationcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBackgroundCopyManager(pub ::windows::core::IUnknown);
impl IBackgroundCopyManager {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateJob<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, displayname: Param0, r#type: BG_JOB_TYPE, pjobid: *mut ::windows::core::GUID, ppjob: *mut ::core::option::Option<IBackgroundCopyJob>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), displayname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(pjobid), ::core::mem::transmute(ppjob)).ok()
    }
    pub unsafe fn GetJob(&self, jobid: *const ::windows::core::GUID) -> ::windows::core::Result<IBackgroundCopyJob> {
        let mut result__: <IBackgroundCopyJob as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(jobid), &mut result__).from_abi::<IBackgroundCopyJob>(result__)
    }
    pub unsafe fn EnumJobs(&self, dwflags: u32) -> ::windows::core::Result<IEnumBackgroundCopyJobs> {
        let mut result__: <IEnumBackgroundCopyJobs as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), &mut result__).from_abi::<IEnumBackgroundCopyJobs>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetErrorDescription(&self, hresult: ::windows::core::HRESULT, languageid: u32) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult), ::core::mem::transmute(languageid), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyManager {
    type Vtable = IBackgroundCopyManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ce34c0d_0dc9_4c1f_897c_daa1b78cee7c);
}
impl ::core::convert::From<IBackgroundCopyManager> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBackgroundCopyManager> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundCopyManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBackgroundCopyManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, displayname: super::super::Foundation::PWSTR, r#type: BG_JOB_TYPE, pjobid: *mut ::windows::core::GUID, ppjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, jobid: *const ::windows::core::GUID, ppjob: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwflags: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hresult: ::windows::core::HRESULT, languageid: u32, perrordescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBackgroundCopyQMgr(pub ::windows::core::IUnknown);
impl IBackgroundCopyQMgr {
    pub unsafe fn CreateGroup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, guidgroupid: Param0) -> ::windows::core::Result<IBackgroundCopyGroup> {
        let mut result__: <IBackgroundCopyGroup as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), guidgroupid.into_param().abi(), &mut result__).from_abi::<IBackgroundCopyGroup>(result__)
    }
    pub unsafe fn GetGroup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, groupid: Param0) -> ::windows::core::Result<IBackgroundCopyGroup> {
        let mut result__: <IBackgroundCopyGroup as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), groupid.into_param().abi(), &mut result__).from_abi::<IBackgroundCopyGroup>(result__)
    }
    pub unsafe fn EnumGroups(&self, dwflags: u32) -> ::windows::core::Result<IEnumBackgroundCopyGroups> {
        let mut result__: <IEnumBackgroundCopyGroups as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), &mut result__).from_abi::<IEnumBackgroundCopyGroups>(result__)
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyQMgr {
    type Vtable = IBackgroundCopyQMgr_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16f41c69_09f5_41d2_8cd8_3c08c47bc8a8);
}
impl ::core::convert::From<IBackgroundCopyQMgr> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyQMgr) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBackgroundCopyQMgr> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyQMgr) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundCopyQMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBackgroundCopyQMgr {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyQMgr_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, guidgroupid: ::windows::core::GUID, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, groupid: ::windows::core::GUID, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwflags: u32, ppenumgroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBackgroundCopyServerCertificateValidationCallback(pub ::windows::core::IUnknown);
impl IBackgroundCopyServerCertificateValidationCallback {
    pub unsafe fn ValidateServerCertificate<'a, Param0: ::windows::core::IntoParam<'a, IBackgroundCopyJob>, Param1: ::windows::core::IntoParam<'a, IBackgroundCopyFile>>(&self, job: Param0, file: Param1, certlength: u32, certdata: *const u8, certencodingtype: u32, certstorelength: u32, certstoredata: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), job.into_param().abi(), file.into_param().abi(), ::core::mem::transmute(certlength), ::core::mem::transmute(certdata), ::core::mem::transmute(certencodingtype), ::core::mem::transmute(certstorelength), ::core::mem::transmute(certstoredata)).ok()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyServerCertificateValidationCallback {
    type Vtable = IBackgroundCopyServerCertificateValidationCallback_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4cec0d02_def7_4158_813a_c32a46945ff7);
}
impl ::core::convert::From<IBackgroundCopyServerCertificateValidationCallback> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyServerCertificateValidationCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBackgroundCopyServerCertificateValidationCallback> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyServerCertificateValidationCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundCopyServerCertificateValidationCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBackgroundCopyServerCertificateValidationCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyServerCertificateValidationCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, job: ::windows::core::RawPtr, file: ::windows::core::RawPtr, certlength: u32, certdata: *const u8, certencodingtype: u32, certstorelength: u32, certstoredata: *const u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBitsPeer(pub ::windows::core::IUnknown);
impl IBitsPeer {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPeerName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsAuthenticated(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsAvailable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IBitsPeer {
    type Vtable = IBitsPeer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x659cdea2_489e_11d9_a9cd_000d56965251);
}
impl ::core::convert::From<IBitsPeer> for ::windows::core::IUnknown {
    fn from(value: IBitsPeer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBitsPeer> for ::windows::core::IUnknown {
    fn from(value: &IBitsPeer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBitsPeer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBitsPeer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitsPeer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pauth: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ponline: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBitsPeerCacheAdministration(pub ::windows::core::IUnknown);
impl IBitsPeerCacheAdministration {
    pub unsafe fn GetMaximumCacheSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMaximumCacheSize(&self, bytes: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(bytes)).ok()
    }
    pub unsafe fn GetMaximumContentAge(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMaximumContentAge(&self, seconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(seconds)).ok()
    }
    pub unsafe fn GetConfigurationFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetConfigurationFlags(&self, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(flags)).ok()
    }
    pub unsafe fn EnumRecords(&self) -> ::windows::core::Result<IEnumBitsPeerCacheRecords> {
        let mut result__: <IEnumBitsPeerCacheRecords as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumBitsPeerCacheRecords>(result__)
    }
    pub unsafe fn GetRecord(&self, id: *const ::windows::core::GUID) -> ::windows::core::Result<IBitsPeerCacheRecord> {
        let mut result__: <IBitsPeerCacheRecord as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(id), &mut result__).from_abi::<IBitsPeerCacheRecord>(result__)
    }
    pub unsafe fn ClearRecords(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn DeleteRecord(&self, id: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(id)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteUrl<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, url: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), url.into_param().abi()).ok()
    }
    pub unsafe fn EnumPeers(&self) -> ::windows::core::Result<IEnumBitsPeers> {
        let mut result__: <IEnumBitsPeers as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumBitsPeers>(result__)
    }
    pub unsafe fn ClearPeers(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn DiscoverPeers(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IBitsPeerCacheAdministration {
    type Vtable = IBitsPeerCacheAdministration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x659cdead_489e_11d9_a9cd_000d56965251);
}
impl ::core::convert::From<IBitsPeerCacheAdministration> for ::windows::core::IUnknown {
    fn from(value: IBitsPeerCacheAdministration) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBitsPeerCacheAdministration> for ::windows::core::IUnknown {
    fn from(value: &IBitsPeerCacheAdministration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBitsPeerCacheAdministration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBitsPeerCacheAdministration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitsPeerCacheAdministration_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbytes: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bytes: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pseconds: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, seconds: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pflags: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, id: *const ::windows::core::GUID, pprecord: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, id: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, url: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBitsPeerCacheRecord(pub ::windows::core::IUnknown);
impl IBitsPeerCacheRecord {
    pub unsafe fn GetId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOriginUrl(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetFileSize(&self) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFileModificationTime(&self) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
        let mut result__: <super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastAccessTime(&self) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
        let mut result__: <super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    pub unsafe fn IsFileValidated(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetFileRanges(&self, prangecount: *mut u32, ppranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(prangecount), ::core::mem::transmute(ppranges)).ok()
    }
}
unsafe impl ::windows::core::Interface for IBitsPeerCacheRecord {
    type Vtable = IBitsPeerCacheRecord_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x659cdeaf_489e_11d9_a9cd_000d56965251);
}
impl ::core::convert::From<IBitsPeerCacheRecord> for ::windows::core::IUnknown {
    fn from(value: IBitsPeerCacheRecord) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBitsPeerCacheRecord> for ::windows::core::IUnknown {
    fn from(value: &IBitsPeerCacheRecord) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBitsPeerCacheRecord {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBitsPeerCacheRecord {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitsPeerCacheRecord_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, prangecount: *mut u32, ppranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBitsTokenOptions(pub ::windows::core::IUnknown);
impl IBitsTokenOptions {
    pub unsafe fn SetHelperTokenFlags(&self, usageflags: BG_TOKEN) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(usageflags)).ok()
    }
    pub unsafe fn GetHelperTokenFlags(&self) -> ::windows::core::Result<BG_TOKEN> {
        let mut result__: <BG_TOKEN as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BG_TOKEN>(result__)
    }
    pub unsafe fn SetHelperToken(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ClearHelperToken(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHelperTokenSid(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IBitsTokenOptions {
    type Vtable = IBitsTokenOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a2584c3_f7d2_457a_9a5e_22b67bffc7d2);
}
impl ::core::convert::From<IBitsTokenOptions> for ::windows::core::IUnknown {
    fn from(value: IBitsTokenOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBitsTokenOptions> for ::windows::core::IUnknown {
    fn from(value: &IBitsTokenOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBitsTokenOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBitsTokenOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitsTokenOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, usageflags: BG_TOKEN) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pflags: *mut BG_TOKEN) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumBackgroundCopyFiles(pub ::windows::core::IUnknown);
impl IEnumBackgroundCopyFiles {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IBackgroundCopyFile>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumBackgroundCopyFiles> {
        let mut result__: <IEnumBackgroundCopyFiles as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumBackgroundCopyFiles>(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IEnumBackgroundCopyFiles {
    type Vtable = IEnumBackgroundCopyFiles_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca51e165_c365_424c_8d41_24aaa4ff3c40);
}
impl ::core::convert::From<IEnumBackgroundCopyFiles> for ::windows::core::IUnknown {
    fn from(value: IEnumBackgroundCopyFiles) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumBackgroundCopyFiles> for ::windows::core::IUnknown {
    fn from(value: &IEnumBackgroundCopyFiles) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumBackgroundCopyFiles {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumBackgroundCopyFiles {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumBackgroundCopyFiles_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pucount: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumBackgroundCopyGroups(pub ::windows::core::IUnknown);
impl IEnumBackgroundCopyGroups {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumBackgroundCopyGroups> {
        let mut result__: <IEnumBackgroundCopyGroups as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumBackgroundCopyGroups>(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IEnumBackgroundCopyGroups {
    type Vtable = IEnumBackgroundCopyGroups_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd993e603_4aa4_47c5_8665_c20d39c2ba4f);
}
impl ::core::convert::From<IEnumBackgroundCopyGroups> for ::windows::core::IUnknown {
    fn from(value: IEnumBackgroundCopyGroups) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumBackgroundCopyGroups> for ::windows::core::IUnknown {
    fn from(value: &IEnumBackgroundCopyGroups) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumBackgroundCopyGroups {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumBackgroundCopyGroups {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumBackgroundCopyGroups_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pucount: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumBackgroundCopyJobs(pub ::windows::core::IUnknown);
impl IEnumBackgroundCopyJobs {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IBackgroundCopyJob>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumBackgroundCopyJobs> {
        let mut result__: <IEnumBackgroundCopyJobs as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumBackgroundCopyJobs>(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IEnumBackgroundCopyJobs {
    type Vtable = IEnumBackgroundCopyJobs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1af4f612_3b71_466f_8f58_7b6f73ac57ad);
}
impl ::core::convert::From<IEnumBackgroundCopyJobs> for ::windows::core::IUnknown {
    fn from(value: IEnumBackgroundCopyJobs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumBackgroundCopyJobs> for ::windows::core::IUnknown {
    fn from(value: &IEnumBackgroundCopyJobs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumBackgroundCopyJobs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumBackgroundCopyJobs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumBackgroundCopyJobs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pucount: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumBackgroundCopyJobs1(pub ::windows::core::IUnknown);
impl IEnumBackgroundCopyJobs1 {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumBackgroundCopyJobs1> {
        let mut result__: <IEnumBackgroundCopyJobs1 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumBackgroundCopyJobs1>(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IEnumBackgroundCopyJobs1 {
    type Vtable = IEnumBackgroundCopyJobs1_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8baeba9d_8f1c_42c4_b82c_09ae79980d25);
}
impl ::core::convert::From<IEnumBackgroundCopyJobs1> for ::windows::core::IUnknown {
    fn from(value: IEnumBackgroundCopyJobs1) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumBackgroundCopyJobs1> for ::windows::core::IUnknown {
    fn from(value: &IEnumBackgroundCopyJobs1) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumBackgroundCopyJobs1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumBackgroundCopyJobs1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumBackgroundCopyJobs1_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pucount: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumBitsPeerCacheRecords(pub ::windows::core::IUnknown);
impl IEnumBitsPeerCacheRecords {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IBitsPeerCacheRecord>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumBitsPeerCacheRecords> {
        let mut result__: <IEnumBitsPeerCacheRecords as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumBitsPeerCacheRecords>(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IEnumBitsPeerCacheRecords {
    type Vtable = IEnumBitsPeerCacheRecords_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x659cdea4_489e_11d9_a9cd_000d56965251);
}
impl ::core::convert::From<IEnumBitsPeerCacheRecords> for ::windows::core::IUnknown {
    fn from(value: IEnumBitsPeerCacheRecords) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumBitsPeerCacheRecords> for ::windows::core::IUnknown {
    fn from(value: &IEnumBitsPeerCacheRecords) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumBitsPeerCacheRecords {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumBitsPeerCacheRecords {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumBitsPeerCacheRecords_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pucount: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumBitsPeers(pub ::windows::core::IUnknown);
impl IEnumBitsPeers {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IBitsPeer>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumBitsPeers> {
        let mut result__: <IEnumBitsPeers as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumBitsPeers>(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IEnumBitsPeers {
    type Vtable = IEnumBitsPeers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x659cdea5_489e_11d9_a9cd_000d56965251);
}
impl ::core::convert::From<IEnumBitsPeers> for ::windows::core::IUnknown {
    fn from(value: IEnumBitsPeers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumBitsPeers> for ::windows::core::IUnknown {
    fn from(value: &IEnumBitsPeers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumBitsPeers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumBitsPeers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumBitsPeers_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pucount: *mut u32) -> ::windows::core::HRESULT,
);
pub const QM_E_DOWNLOADER_UNAVAILABLE: u32 = 2164264963u32;
pub const QM_E_INVALID_STATE: u32 = 2164264961u32;
pub const QM_E_ITEM_NOT_FOUND: u32 = 2164264964u32;
pub const QM_E_SERVICE_UNAVAILABLE: u32 = 2164264962u32;
pub const QM_NOTIFY_DISABLE_NOTIFY: u32 = 64u32;
pub const QM_NOTIFY_FILE_DONE: u32 = 1u32;
pub const QM_NOTIFY_GROUP_DONE: u32 = 4u32;
pub const QM_NOTIFY_JOB_DONE: u32 = 2u32;
pub const QM_NOTIFY_USE_PROGRESSEX: u32 = 128u32;
pub const QM_PROGRESS_PERCENT_DONE: u32 = 1u32;
pub const QM_PROGRESS_SIZE_DONE: u32 = 3u32;
pub const QM_PROGRESS_TIME_DONE: u32 = 2u32;
pub const QM_PROTOCOL_CUSTOM: u32 = 4u32;
pub const QM_PROTOCOL_FTP: u32 = 2u32;
pub const QM_PROTOCOL_HTTP: u32 = 1u32;
pub const QM_PROTOCOL_SMB: u32 = 3u32;
pub const QM_STATUS_FILE_COMPLETE: u32 = 1u32;
pub const QM_STATUS_FILE_INCOMPLETE: u32 = 2u32;
pub const QM_STATUS_GROUP_COMPLETE: u32 = 64u32;
pub const QM_STATUS_GROUP_ERROR: u32 = 512u32;
pub const QM_STATUS_GROUP_FOREGROUND: u32 = 1024u32;
pub const QM_STATUS_GROUP_INCOMPLETE: u32 = 128u32;
pub const QM_STATUS_GROUP_SUSPENDED: u32 = 256u32;
pub const QM_STATUS_JOB_COMPLETE: u32 = 4u32;
pub const QM_STATUS_JOB_ERROR: u32 = 16u32;
pub const QM_STATUS_JOB_FOREGROUND: u32 = 32u32;
pub const QM_STATUS_JOB_INCOMPLETE: u32 = 8u32;
