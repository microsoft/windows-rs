#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct AsyncIBackgroundCopyCallback(::windows::core::IUnknown);
impl AsyncIBackgroundCopyCallback {
    pub unsafe fn Begin_JobTransferred<'a, P0>(&self, pjob: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IBackgroundCopyJob>>,
    {
        (::windows::core::Interface::vtable(self).Begin_JobTransferred)(::windows::core::Interface::as_raw(self), pjob.into().abi()).ok()
    }
    pub unsafe fn Finish_JobTransferred(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_JobTransferred)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Begin_JobError<'a, P0, P1>(&self, pjob: P0, perror: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IBackgroundCopyJob>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IBackgroundCopyError>>,
    {
        (::windows::core::Interface::vtable(self).Begin_JobError)(::windows::core::Interface::as_raw(self), pjob.into().abi(), perror.into().abi()).ok()
    }
    pub unsafe fn Finish_JobError(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_JobError)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Begin_JobModification<'a, P0>(&self, pjob: P0, dwreserved: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IBackgroundCopyJob>>,
    {
        (::windows::core::Interface::vtable(self).Begin_JobModification)(::windows::core::Interface::as_raw(self), pjob.into().abi(), dwreserved).ok()
    }
    pub unsafe fn Finish_JobModification(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_JobModification)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<AsyncIBackgroundCopyCallback> for ::windows::core::IUnknown {
    fn from(value: AsyncIBackgroundCopyCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a AsyncIBackgroundCopyCallback> for &'a ::windows::core::IUnknown {
    fn from(value: &'a AsyncIBackgroundCopyCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIBackgroundCopyCallback> for ::windows::core::IUnknown {
    fn from(value: &AsyncIBackgroundCopyCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for AsyncIBackgroundCopyCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIBackgroundCopyCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIBackgroundCopyCallback {}
impl ::core::fmt::Debug for AsyncIBackgroundCopyCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIBackgroundCopyCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for AsyncIBackgroundCopyCallback {
    type Vtable = AsyncIBackgroundCopyCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca29d251_b4bb_4679_a3d9_ae8006119d54);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIBackgroundCopyCallback_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Begin_JobTransferred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pjob: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Finish_JobTransferred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Begin_JobError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pjob: *mut ::core::ffi::c_void, perror: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Finish_JobError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Begin_JobModification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pjob: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT,
    pub Finish_JobModification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub struct BG_AUTH_CREDENTIALS {
    pub Target: BG_AUTH_TARGET,
    pub Scheme: BG_AUTH_SCHEME,
    pub Credentials: BG_AUTH_CREDENTIALS_UNION,
}
impl ::core::marker::Copy for BG_AUTH_CREDENTIALS {}
impl ::core::clone::Clone for BG_AUTH_CREDENTIALS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BG_AUTH_CREDENTIALS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BG_AUTH_CREDENTIALS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BG_AUTH_CREDENTIALS>()) == 0 }
    }
}
impl ::core::cmp::Eq for BG_AUTH_CREDENTIALS {}
impl ::core::default::Default for BG_AUTH_CREDENTIALS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub union BG_AUTH_CREDENTIALS_UNION {
    pub Basic: BG_BASIC_CREDENTIALS,
}
impl ::core::marker::Copy for BG_AUTH_CREDENTIALS_UNION {}
impl ::core::clone::Clone for BG_AUTH_CREDENTIALS_UNION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BG_AUTH_CREDENTIALS_UNION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BG_AUTH_CREDENTIALS_UNION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BG_AUTH_CREDENTIALS_UNION>()) == 0 }
    }
}
impl ::core::cmp::Eq for BG_AUTH_CREDENTIALS_UNION {}
impl ::core::default::Default for BG_AUTH_CREDENTIALS_UNION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BG_AUTH_SCHEME(pub i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_AUTH_SCHEME_BASIC: BG_AUTH_SCHEME = BG_AUTH_SCHEME(1i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_AUTH_SCHEME_DIGEST: BG_AUTH_SCHEME = BG_AUTH_SCHEME(2i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_AUTH_SCHEME_NTLM: BG_AUTH_SCHEME = BG_AUTH_SCHEME(3i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_AUTH_SCHEME_NEGOTIATE: BG_AUTH_SCHEME = BG_AUTH_SCHEME(4i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_AUTH_SCHEME_PASSPORT: BG_AUTH_SCHEME = BG_AUTH_SCHEME(5i32);
impl ::core::marker::Copy for BG_AUTH_SCHEME {}
impl ::core::clone::Clone for BG_AUTH_SCHEME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BG_AUTH_SCHEME {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BG_AUTH_SCHEME {
    type Abi = Self;
}
impl ::core::fmt::Debug for BG_AUTH_SCHEME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BG_AUTH_SCHEME").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BG_AUTH_TARGET(pub i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_AUTH_TARGET_SERVER: BG_AUTH_TARGET = BG_AUTH_TARGET(1i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_AUTH_TARGET_PROXY: BG_AUTH_TARGET = BG_AUTH_TARGET(2i32);
impl ::core::marker::Copy for BG_AUTH_TARGET {}
impl ::core::clone::Clone for BG_AUTH_TARGET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BG_AUTH_TARGET {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BG_AUTH_TARGET {
    type Abi = Self;
}
impl ::core::fmt::Debug for BG_AUTH_TARGET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BG_AUTH_TARGET").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub struct BG_BASIC_CREDENTIALS {
    pub UserName: ::windows::core::PWSTR,
    pub Password: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for BG_BASIC_CREDENTIALS {}
impl ::core::clone::Clone for BG_BASIC_CREDENTIALS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BG_BASIC_CREDENTIALS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BG_BASIC_CREDENTIALS").field("UserName", &self.UserName).field("Password", &self.Password).finish()
    }
}
unsafe impl ::windows::core::Abi for BG_BASIC_CREDENTIALS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BG_BASIC_CREDENTIALS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BG_BASIC_CREDENTIALS>()) == 0 }
    }
}
impl ::core::cmp::Eq for BG_BASIC_CREDENTIALS {}
impl ::core::default::Default for BG_BASIC_CREDENTIALS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BG_CERT_STORE_LOCATION(pub i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_CERT_STORE_LOCATION_CURRENT_USER: BG_CERT_STORE_LOCATION = BG_CERT_STORE_LOCATION(0i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_CERT_STORE_LOCATION_LOCAL_MACHINE: BG_CERT_STORE_LOCATION = BG_CERT_STORE_LOCATION(1i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_CERT_STORE_LOCATION_CURRENT_SERVICE: BG_CERT_STORE_LOCATION = BG_CERT_STORE_LOCATION(2i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_CERT_STORE_LOCATION_SERVICES: BG_CERT_STORE_LOCATION = BG_CERT_STORE_LOCATION(3i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_CERT_STORE_LOCATION_USERS: BG_CERT_STORE_LOCATION = BG_CERT_STORE_LOCATION(4i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_CERT_STORE_LOCATION_CURRENT_USER_GROUP_POLICY: BG_CERT_STORE_LOCATION = BG_CERT_STORE_LOCATION(5i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_CERT_STORE_LOCATION_LOCAL_MACHINE_GROUP_POLICY: BG_CERT_STORE_LOCATION = BG_CERT_STORE_LOCATION(6i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_CERT_STORE_LOCATION_LOCAL_MACHINE_ENTERPRISE: BG_CERT_STORE_LOCATION = BG_CERT_STORE_LOCATION(7i32);
impl ::core::marker::Copy for BG_CERT_STORE_LOCATION {}
impl ::core::clone::Clone for BG_CERT_STORE_LOCATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BG_CERT_STORE_LOCATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BG_CERT_STORE_LOCATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for BG_CERT_STORE_LOCATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BG_CERT_STORE_LOCATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_COPY_FILE_ALL: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_COPY_FILE_DACL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_COPY_FILE_GROUP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_COPY_FILE_OWNER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_COPY_FILE_SACL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_DISABLE_BRANCH_CACHE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_ENABLE_PEERCACHING_CLIENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_ENABLE_PEERCACHING_SERVER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BG_ERROR_CONTEXT(pub i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_ERROR_CONTEXT_NONE: BG_ERROR_CONTEXT = BG_ERROR_CONTEXT(0i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_ERROR_CONTEXT_UNKNOWN: BG_ERROR_CONTEXT = BG_ERROR_CONTEXT(1i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_ERROR_CONTEXT_GENERAL_QUEUE_MANAGER: BG_ERROR_CONTEXT = BG_ERROR_CONTEXT(2i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_ERROR_CONTEXT_QUEUE_MANAGER_NOTIFICATION: BG_ERROR_CONTEXT = BG_ERROR_CONTEXT(3i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_ERROR_CONTEXT_LOCAL_FILE: BG_ERROR_CONTEXT = BG_ERROR_CONTEXT(4i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_ERROR_CONTEXT_REMOTE_FILE: BG_ERROR_CONTEXT = BG_ERROR_CONTEXT(5i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_ERROR_CONTEXT_GENERAL_TRANSPORT: BG_ERROR_CONTEXT = BG_ERROR_CONTEXT(6i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_ERROR_CONTEXT_REMOTE_APPLICATION: BG_ERROR_CONTEXT = BG_ERROR_CONTEXT(7i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_ERROR_CONTEXT_SERVER_CERTIFICATE_CALLBACK: BG_ERROR_CONTEXT = BG_ERROR_CONTEXT(8i32);
impl ::core::marker::Copy for BG_ERROR_CONTEXT {}
impl ::core::clone::Clone for BG_ERROR_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BG_ERROR_CONTEXT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BG_ERROR_CONTEXT {
    type Abi = Self;
}
impl ::core::fmt::Debug for BG_ERROR_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BG_ERROR_CONTEXT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_APP_PACKAGE_NOT_FOUND: i32 = -2145386390i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_APP_PACKAGE_SCENARIO_NOT_SUPPORTED: i32 = -2145386389i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_BLOCKED_BY_BACKGROUND_ACCESS_POLICY: i32 = -2145386386i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_BLOCKED_BY_BATTERY_POLICY: i32 = -2145386393i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_BLOCKED_BY_BATTERY_SAVER: i32 = -2145386392i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_BLOCKED_BY_COST_TRANSFER_POLICY: i32 = -2145386407i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_BLOCKED_BY_GAME_MODE: i32 = -2145386385i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_BLOCKED_BY_POLICY: i32 = -2145386434i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_BLOCKED_BY_SYSTEM_POLICY: i32 = -2145386384i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_BUSYCACHERECORD: i32 = -2145386424i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_CLIENT_SERVER_PROTOCOL_MISMATCH: i32 = -2145386462i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_COMMIT_IN_PROGRESS: i32 = -2145386429i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_CONNECTION_CLOSED: i32 = -2145386450i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_CONNECT_FAILURE: i32 = -2145386451i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_DATABASE_CORRUPT: i32 = -2145386388i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_DESTINATION_LOCKED: i32 = -2145386483i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_DISCOVERY_IN_PROGRESS: i32 = -2145386428i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_EMPTY: i32 = -2145386493i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_ERROR_CONTEXT_GENERAL_QUEUE_MANAGER: i32 = -2145386488i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_ERROR_CONTEXT_GENERAL_TRANSPORT: i32 = -2145386485i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_ERROR_CONTEXT_LOCAL_FILE: i32 = -2145386487i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_ERROR_CONTEXT_QUEUE_MANAGER_NOTIFICATION: i32 = -2145386484i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_ERROR_CONTEXT_REMOTE_APPLICATION: i32 = -2145386466i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_ERROR_CONTEXT_REMOTE_FILE: i32 = -2145386486i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_ERROR_CONTEXT_SERVER_CERTIFICATE_CALLBACK: i32 = -2145386378i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_ERROR_CONTEXT_UNKNOWN: i32 = -2145386489i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_ERROR_INFORMATION_UNAVAILABLE: i32 = -2145386481i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_FILE_NOT_AVAILABLE: i32 = -2145386492i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_FILE_NOT_FOUND: i32 = -2145386455i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_100: i32 = -2145845148i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_101: i32 = -2145845147i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_200: i32 = -2145845048i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_201: i32 = -2145845047i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_202: i32 = -2145845046i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_203: i32 = -2145845045i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_204: i32 = -2145845044i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_205: i32 = -2145845043i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_206: i32 = -2145845042i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_300: i32 = -2145844948i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_301: i32 = -2145844947i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_302: i32 = -2145844946i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_303: i32 = -2145844945i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_304: i32 = -2145844944i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_305: i32 = -2145844943i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_307: i32 = -2145844941i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_400: i32 = -2145844848i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_401: i32 = -2145844847i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_402: i32 = -2145844846i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_403: i32 = -2145844845i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_404: i32 = -2145844844i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_405: i32 = -2145844843i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_406: i32 = -2145844842i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_407: i32 = -2145844841i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_408: i32 = -2145844840i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_409: i32 = -2145844839i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_410: i32 = -2145844838i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_411: i32 = -2145844837i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_412: i32 = -2145844836i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_413: i32 = -2145844835i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_414: i32 = -2145844834i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_415: i32 = -2145844833i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_416: i32 = -2145844832i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_417: i32 = -2145844831i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_449: i32 = -2145844799i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_500: i32 = -2145844748i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_501: i32 = -2145844747i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_502: i32 = -2145844746i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_503: i32 = -2145844745i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_504: i32 = -2145844744i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_HTTP_ERROR_505: i32 = -2145844743i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_INSUFFICIENT_HTTP_SUPPORT: i32 = -2145386478i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_INSUFFICIENT_RANGE_SUPPORT: i32 = -2145386477i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_INVALID_AUTH_SCHEME: i32 = -2145386456i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_INVALID_AUTH_TARGET: i32 = -2145386457i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_INVALID_CREDENTIALS: i32 = -2145386432i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_INVALID_HASH_ALGORITHM: i32 = -2145386431i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_INVALID_PROXY_INFO: i32 = -2145386433i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_INVALID_RANGE: i32 = -2145386453i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_INVALID_SERVER_RESPONSE: i32 = -2145386469i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_INVALID_STATE: i32 = -2145386494i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_LOCAL_FILE_CHANGED: i32 = -2145386467i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_MAXDOWNLOAD_TIMEOUT: i32 = -2145386412i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_MAX_DOWNLOAD_SIZE_INVALID_VALUE: i32 = -2145386397i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_MAX_DOWNLOAD_SIZE_LIMIT_REACHED: i32 = -2145386396i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_MISSING_FILE_SIZE: i32 = -2145386479i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_NETWORK_DISCONNECTED: i32 = -2145386480i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_NEW_OWNER_DIFF_MAPPING: i32 = -2145386475i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_NEW_OWNER_NO_FILE_ACCESS: i32 = -2145386474i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_NOT_FOUND: i32 = -2145386495i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_NOT_SUPPORTED_WITH_CUSTOM_HTTP_METHOD: i32 = -2145386383i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_NO_PROGRESS: i32 = -2145386460i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_OVERLAPPING_RANGES: i32 = -2145386452i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_PASSWORD_TOO_LARGE: i32 = -2145386458i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_PEERCACHING_DISABLED: i32 = -2145386425i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_PROPERTY_SUPPORTED_FOR_DOWNLOAD_JOBS_ONLY: i32 = -2145386400i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_PROTOCOL_NOT_AVAILABLE: i32 = -2145386491i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_PROXY_BYPASS_LIST_TOO_LARGE: i32 = -2145386471i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_PROXY_LIST_TOO_LARGE: i32 = -2145386472i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_RANDOM_ACCESS_NOT_SUPPORTED: i32 = -2145386387i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_READ_ONLY_PROPERTY: i32 = -2145386408i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_READ_ONLY_PROPERTY_AFTER_ADDFILE: i32 = -2145386399i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_READ_ONLY_PROPERTY_AFTER_RESUME: i32 = -2145386398i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_READ_ONLY_WHEN_JOB_ACTIVE: i32 = -2145386379i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_RECORD_DELETED: i32 = -2145386430i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_REMOTE_FILE_CHANGED: i32 = -2145386381i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_REMOTE_NOT_SUPPORTED: i32 = -2145386476i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_SERVER_CERT_VALIDATION_INTERFACE_REQUIRED: i32 = -2145386380i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_SERVER_EXECUTE_ENABLE: i32 = -2145386461i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_SESSION_NOT_FOUND: i32 = -2145386465i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_STANDBY_MODE: i32 = -2145386395i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_STRING_TOO_LONG: i32 = -2145386463i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_TEST_OPTION_BLOCKED_DOWNLOAD: i32 = -2145386426i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_TOKEN_REQUIRED: i32 = -2145386410i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_TOO_LARGE: i32 = -2145386464i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_TOO_MANY_FILES: i32 = -2145386468i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_TOO_MANY_FILES_IN_JOB: i32 = -2145386415i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_TOO_MANY_JOBS_PER_MACHINE: i32 = -2145386416i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_TOO_MANY_JOBS_PER_USER: i32 = -2145386423i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_TOO_MANY_RANGES_IN_FILE: i32 = -2145386414i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_UNKNOWN_PROPERTY_ID: i32 = -2145386409i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_UNSUPPORTED_JOB_CONFIGURATION: i32 = -2145386382i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_UPNP_ERROR: i32 = -2145386427i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_USERNAME_TOO_LARGE: i32 = -2145386459i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_USE_STORED_CREDENTIALS_NOT_SUPPORTED: i32 = -2145386394i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_VALIDATION_FAILED: i32 = -2145386413i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_VOLUME_CHANGED: i32 = -2145386482i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_E_WATCHDOG_TIMEOUT: i32 = -2145386391i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub struct BG_FILE_INFO {
    pub RemoteName: ::windows::core::PWSTR,
    pub LocalName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for BG_FILE_INFO {}
impl ::core::clone::Clone for BG_FILE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BG_FILE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BG_FILE_INFO").field("RemoteName", &self.RemoteName).field("LocalName", &self.LocalName).finish()
    }
}
unsafe impl ::windows::core::Abi for BG_FILE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BG_FILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BG_FILE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for BG_FILE_INFO {}
impl ::core::default::Default for BG_FILE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct BG_FILE_PROGRESS {
    pub BytesTotal: u64,
    pub BytesTransferred: u64,
    pub Completed: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for BG_FILE_PROGRESS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for BG_FILE_PROGRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for BG_FILE_PROGRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BG_FILE_PROGRESS").field("BytesTotal", &self.BytesTotal).field("BytesTransferred", &self.BytesTransferred).field("Completed", &self.Completed).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for BG_FILE_PROGRESS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for BG_FILE_PROGRESS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BG_FILE_PROGRESS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for BG_FILE_PROGRESS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BG_FILE_PROGRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub struct BG_FILE_RANGE {
    pub InitialOffset: u64,
    pub Length: u64,
}
impl ::core::marker::Copy for BG_FILE_RANGE {}
impl ::core::clone::Clone for BG_FILE_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BG_FILE_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BG_FILE_RANGE").field("InitialOffset", &self.InitialOffset).field("Length", &self.Length).finish()
    }
}
unsafe impl ::windows::core::Abi for BG_FILE_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BG_FILE_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BG_FILE_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for BG_FILE_RANGE {}
impl ::core::default::Default for BG_FILE_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_HTTP_REDIRECT_POLICY_ALLOW_HTTPS_TO_HTTP: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_HTTP_REDIRECT_POLICY_ALLOW_REPORT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_HTTP_REDIRECT_POLICY_ALLOW_SILENT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_HTTP_REDIRECT_POLICY_DISALLOW: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_HTTP_REDIRECT_POLICY_MASK: u32 = 1792u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_DISABLE_BRANCH_CACHE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_ENABLE_PEERCACHING_CLIENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_ENABLE_PEERCACHING_SERVER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_ENUM_ALL_USERS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BG_JOB_PRIORITY(pub i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_PRIORITY_FOREGROUND: BG_JOB_PRIORITY = BG_JOB_PRIORITY(0i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_PRIORITY_HIGH: BG_JOB_PRIORITY = BG_JOB_PRIORITY(1i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_PRIORITY_NORMAL: BG_JOB_PRIORITY = BG_JOB_PRIORITY(2i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_PRIORITY_LOW: BG_JOB_PRIORITY = BG_JOB_PRIORITY(3i32);
impl ::core::marker::Copy for BG_JOB_PRIORITY {}
impl ::core::clone::Clone for BG_JOB_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BG_JOB_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BG_JOB_PRIORITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for BG_JOB_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BG_JOB_PRIORITY").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub struct BG_JOB_PROGRESS {
    pub BytesTotal: u64,
    pub BytesTransferred: u64,
    pub FilesTotal: u32,
    pub FilesTransferred: u32,
}
impl ::core::marker::Copy for BG_JOB_PROGRESS {}
impl ::core::clone::Clone for BG_JOB_PROGRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BG_JOB_PROGRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BG_JOB_PROGRESS").field("BytesTotal", &self.BytesTotal).field("BytesTransferred", &self.BytesTransferred).field("FilesTotal", &self.FilesTotal).field("FilesTransferred", &self.FilesTransferred).finish()
    }
}
unsafe impl ::windows::core::Abi for BG_JOB_PROGRESS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BG_JOB_PROGRESS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BG_JOB_PROGRESS>()) == 0 }
    }
}
impl ::core::cmp::Eq for BG_JOB_PROGRESS {}
impl ::core::default::Default for BG_JOB_PROGRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BG_JOB_PROXY_USAGE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_PROXY_USAGE_PRECONFIG: BG_JOB_PROXY_USAGE = BG_JOB_PROXY_USAGE(0i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_PROXY_USAGE_NO_PROXY: BG_JOB_PROXY_USAGE = BG_JOB_PROXY_USAGE(1i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_PROXY_USAGE_OVERRIDE: BG_JOB_PROXY_USAGE = BG_JOB_PROXY_USAGE(2i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_PROXY_USAGE_AUTODETECT: BG_JOB_PROXY_USAGE = BG_JOB_PROXY_USAGE(3i32);
impl ::core::marker::Copy for BG_JOB_PROXY_USAGE {}
impl ::core::clone::Clone for BG_JOB_PROXY_USAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BG_JOB_PROXY_USAGE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BG_JOB_PROXY_USAGE {
    type Abi = Self;
}
impl ::core::fmt::Debug for BG_JOB_PROXY_USAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BG_JOB_PROXY_USAGE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub struct BG_JOB_REPLY_PROGRESS {
    pub BytesTotal: u64,
    pub BytesTransferred: u64,
}
impl ::core::marker::Copy for BG_JOB_REPLY_PROGRESS {}
impl ::core::clone::Clone for BG_JOB_REPLY_PROGRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BG_JOB_REPLY_PROGRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BG_JOB_REPLY_PROGRESS").field("BytesTotal", &self.BytesTotal).field("BytesTransferred", &self.BytesTransferred).finish()
    }
}
unsafe impl ::windows::core::Abi for BG_JOB_REPLY_PROGRESS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BG_JOB_REPLY_PROGRESS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BG_JOB_REPLY_PROGRESS>()) == 0 }
    }
}
impl ::core::cmp::Eq for BG_JOB_REPLY_PROGRESS {}
impl ::core::default::Default for BG_JOB_REPLY_PROGRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BG_JOB_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_STATE_QUEUED: BG_JOB_STATE = BG_JOB_STATE(0i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_STATE_CONNECTING: BG_JOB_STATE = BG_JOB_STATE(1i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_STATE_TRANSFERRING: BG_JOB_STATE = BG_JOB_STATE(2i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_STATE_SUSPENDED: BG_JOB_STATE = BG_JOB_STATE(3i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_STATE_ERROR: BG_JOB_STATE = BG_JOB_STATE(4i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_STATE_TRANSIENT_ERROR: BG_JOB_STATE = BG_JOB_STATE(5i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_STATE_TRANSFERRED: BG_JOB_STATE = BG_JOB_STATE(6i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_STATE_ACKNOWLEDGED: BG_JOB_STATE = BG_JOB_STATE(7i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_STATE_CANCELLED: BG_JOB_STATE = BG_JOB_STATE(8i32);
impl ::core::marker::Copy for BG_JOB_STATE {}
impl ::core::clone::Clone for BG_JOB_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BG_JOB_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BG_JOB_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for BG_JOB_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BG_JOB_STATE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct BG_JOB_TIMES {
    pub CreationTime: super::super::Foundation::FILETIME,
    pub ModificationTime: super::super::Foundation::FILETIME,
    pub TransferCompletionTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for BG_JOB_TIMES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for BG_JOB_TIMES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for BG_JOB_TIMES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BG_JOB_TIMES").field("CreationTime", &self.CreationTime).field("ModificationTime", &self.ModificationTime).field("TransferCompletionTime", &self.TransferCompletionTime).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for BG_JOB_TIMES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for BG_JOB_TIMES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BG_JOB_TIMES>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for BG_JOB_TIMES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BG_JOB_TIMES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BG_JOB_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_TYPE_DOWNLOAD: BG_JOB_TYPE = BG_JOB_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_TYPE_UPLOAD: BG_JOB_TYPE = BG_JOB_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_JOB_TYPE_UPLOAD_REPLY: BG_JOB_TYPE = BG_JOB_TYPE(2i32);
impl ::core::marker::Copy for BG_JOB_TYPE {}
impl ::core::clone::Clone for BG_JOB_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BG_JOB_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BG_JOB_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for BG_JOB_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BG_JOB_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_NOTIFY_DISABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_NOTIFY_FILE_RANGES_TRANSFERRED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_NOTIFY_FILE_TRANSFERRED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_NOTIFY_JOB_ERROR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_NOTIFY_JOB_MODIFICATION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_NOTIFY_JOB_TRANSFERRED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_SSL_ENABLE_CRL_CHECK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_SSL_IGNORE_CERT_CN_INVALID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_SSL_IGNORE_CERT_DATE_INVALID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_SSL_IGNORE_CERT_WRONG_USAGE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_SSL_IGNORE_UNKNOWN_CA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_S_ERROR_CONTEXT_NONE: i32 = 2097158i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_S_OVERRIDDEN_BY_POLICY: i32 = 2097237i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_S_PARTIAL_COMPLETE: i32 = 2097175i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_S_PROXY_CHANGED: i32 = 2097194i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_S_UNABLE_TO_DELETE_FILES: i32 = 2097178i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BG_TOKEN(pub u32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_TOKEN_LOCAL_FILE: BG_TOKEN = BG_TOKEN(1u32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BG_TOKEN_NETWORK: BG_TOKEN = BG_TOKEN(2u32);
impl ::core::marker::Copy for BG_TOKEN {}
impl ::core::clone::Clone for BG_TOKEN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BG_TOKEN {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BG_TOKEN {
    type Abi = Self;
}
impl ::core::fmt::Debug for BG_TOKEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BG_TOKEN").field(&self.0).finish()
    }
}
pub const BITSExtensionSetupFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefbbab68_7286_4783_94bf_9461d8b7e7e9);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_COST_OPTION_IGNORE_CONGESTION: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_COST_STATE_BELOW_CAP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_COST_STATE_CAPPED_USAGE_UNKNOWN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_COST_STATE_NEAR_CAP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_COST_STATE_OVERCAP_CHARGED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_COST_STATE_OVERCAP_THROTTLED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_COST_STATE_RESERVED: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_COST_STATE_ROAMING: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_COST_STATE_UNRESTRICTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_COST_STATE_USAGE_BASED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BITS_FILE_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_FILE_PROPERTY_ID_HTTP_RESPONSE_HEADERS: BITS_FILE_PROPERTY_ID = BITS_FILE_PROPERTY_ID(1i32);
impl ::core::marker::Copy for BITS_FILE_PROPERTY_ID {}
impl ::core::clone::Clone for BITS_FILE_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BITS_FILE_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BITS_FILE_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for BITS_FILE_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BITS_FILE_PROPERTY_ID").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub union BITS_FILE_PROPERTY_VALUE {
    pub String: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for BITS_FILE_PROPERTY_VALUE {}
impl ::core::clone::Clone for BITS_FILE_PROPERTY_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BITS_FILE_PROPERTY_VALUE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BITS_FILE_PROPERTY_VALUE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BITS_FILE_PROPERTY_VALUE>()) == 0 }
    }
}
impl ::core::cmp::Eq for BITS_FILE_PROPERTY_VALUE {}
impl ::core::default::Default for BITS_FILE_PROPERTY_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BITS_JOB_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_JOB_PROPERTY_ID_COST_FLAGS: BITS_JOB_PROPERTY_ID = BITS_JOB_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_JOB_PROPERTY_NOTIFICATION_CLSID: BITS_JOB_PROPERTY_ID = BITS_JOB_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_JOB_PROPERTY_DYNAMIC_CONTENT: BITS_JOB_PROPERTY_ID = BITS_JOB_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_JOB_PROPERTY_HIGH_PERFORMANCE: BITS_JOB_PROPERTY_ID = BITS_JOB_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_JOB_PROPERTY_MAX_DOWNLOAD_SIZE: BITS_JOB_PROPERTY_ID = BITS_JOB_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_JOB_PROPERTY_USE_STORED_CREDENTIALS: BITS_JOB_PROPERTY_ID = BITS_JOB_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_JOB_PROPERTY_MINIMUM_NOTIFICATION_INTERVAL_MS: BITS_JOB_PROPERTY_ID = BITS_JOB_PROPERTY_ID(9i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_JOB_PROPERTY_ON_DEMAND_MODE: BITS_JOB_PROPERTY_ID = BITS_JOB_PROPERTY_ID(10i32);
impl ::core::marker::Copy for BITS_JOB_PROPERTY_ID {}
impl ::core::clone::Clone for BITS_JOB_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BITS_JOB_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BITS_JOB_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for BITS_JOB_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BITS_JOB_PROPERTY_ID").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union BITS_JOB_PROPERTY_VALUE {
    pub Dword: u32,
    pub ClsID: ::windows::core::GUID,
    pub Enable: super::super::Foundation::BOOL,
    pub Uint64: u64,
    pub Target: BG_AUTH_TARGET,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for BITS_JOB_PROPERTY_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for BITS_JOB_PROPERTY_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for BITS_JOB_PROPERTY_VALUE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for BITS_JOB_PROPERTY_VALUE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BITS_JOB_PROPERTY_VALUE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for BITS_JOB_PROPERTY_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BITS_JOB_PROPERTY_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BITS_JOB_TRANSFER_POLICY(pub i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_JOB_TRANSFER_POLICY_ALWAYS: BITS_JOB_TRANSFER_POLICY = BITS_JOB_TRANSFER_POLICY(-2147483393i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_JOB_TRANSFER_POLICY_NOT_ROAMING: BITS_JOB_TRANSFER_POLICY = BITS_JOB_TRANSFER_POLICY(-2147483521i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_JOB_TRANSFER_POLICY_NO_SURCHARGE: BITS_JOB_TRANSFER_POLICY = BITS_JOB_TRANSFER_POLICY(-2147483537i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_JOB_TRANSFER_POLICY_STANDARD: BITS_JOB_TRANSFER_POLICY = BITS_JOB_TRANSFER_POLICY(-2147483545i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_JOB_TRANSFER_POLICY_UNRESTRICTED: BITS_JOB_TRANSFER_POLICY = BITS_JOB_TRANSFER_POLICY(-2147483615i32);
impl ::core::marker::Copy for BITS_JOB_TRANSFER_POLICY {}
impl ::core::clone::Clone for BITS_JOB_TRANSFER_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BITS_JOB_TRANSFER_POLICY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BITS_JOB_TRANSFER_POLICY {
    type Abi = Self;
}
impl ::core::fmt::Debug for BITS_JOB_TRANSFER_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BITS_JOB_TRANSFER_POLICY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_MC_FAILED_TO_START: i32 = -2145828856i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_MC_FATAL_IGD_ERROR: i32 = -2145828855i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_MC_FILE_DELETION_FAILED: i32 = -2145828863i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_MC_FILE_DELETION_FAILED_MORE: i32 = -2145828862i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_MC_JOB_CANCELLED: i32 = -2145828864i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_MC_JOB_NOTIFICATION_FAILURE: i32 = -2145828858i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_MC_JOB_PROPERTY_CHANGE: i32 = -2145828861i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_MC_JOB_SCAVENGED: i32 = -2145828859i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_MC_JOB_TAKE_OWNERSHIP: i32 = -2145828860i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_MC_PEERCACHING_PORT: i32 = -2145828854i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const BITS_MC_STATE_FILE_CORRUPT: i32 = -2145828857i32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
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
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FILESETINFO {
    pub bstrRemoteFile: super::super::Foundation::BSTR,
    pub bstrLocalFile: super::super::Foundation::BSTR,
    pub dwSizeHint: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILESETINFO {
    fn clone(&self) -> Self {
        Self { bstrRemoteFile: self.bstrRemoteFile.clone(), bstrLocalFile: self.bstrLocalFile.clone(), dwSizeHint: self.dwSizeHint }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FILESETINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILESETINFO").field("bstrRemoteFile", &self.bstrRemoteFile).field("bstrLocalFile", &self.bstrLocalFile).field("dwSizeHint", &self.dwSizeHint).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FILESETINFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
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
impl ::core::default::Default for FILESETINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GROUPPROP(pub i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const GROUPPROP_PRIORITY: GROUPPROP = GROUPPROP(0i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const GROUPPROP_REMOTEUSERID: GROUPPROP = GROUPPROP(1i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const GROUPPROP_REMOTEUSERPWD: GROUPPROP = GROUPPROP(2i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const GROUPPROP_LOCALUSERID: GROUPPROP = GROUPPROP(3i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const GROUPPROP_LOCALUSERPWD: GROUPPROP = GROUPPROP(4i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const GROUPPROP_PROTOCOLFLAGS: GROUPPROP = GROUPPROP(5i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const GROUPPROP_NOTIFYFLAGS: GROUPPROP = GROUPPROP(6i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const GROUPPROP_NOTIFYCLSID: GROUPPROP = GROUPPROP(7i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const GROUPPROP_PROGRESSSIZE: GROUPPROP = GROUPPROP(8i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const GROUPPROP_PROGRESSPERCENT: GROUPPROP = GROUPPROP(9i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const GROUPPROP_PROGRESSTIME: GROUPPROP = GROUPPROP(10i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const GROUPPROP_DISPLAYNAME: GROUPPROP = GROUPPROP(11i32);
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const GROUPPROP_DESCRIPTION: GROUPPROP = GROUPPROP(12i32);
impl ::core::marker::Copy for GROUPPROP {}
impl ::core::clone::Clone for GROUPPROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GROUPPROP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GROUPPROP {
    type Abi = Self;
}
impl ::core::fmt::Debug for GROUPPROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GROUPPROP").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IBITSExtensionSetup(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IBITSExtensionSetup {
    pub unsafe fn EnableBITSUploads(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnableBITSUploads)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DisableBITSUploads(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisableBITSUploads)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCleanupTaskName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetCleanupTaskName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetCleanupTask(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetCleanupTask)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IBITSExtensionSetup> for ::windows::core::IUnknown {
    fn from(value: IBITSExtensionSetup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IBITSExtensionSetup> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBITSExtensionSetup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IBITSExtensionSetup> for ::windows::core::IUnknown {
    fn from(value: &IBITSExtensionSetup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IBITSExtensionSetup> for super::super::System::Com::IDispatch {
    fn from(value: IBITSExtensionSetup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IBITSExtensionSetup> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IBITSExtensionSetup) -> Self {
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
impl ::core::clone::Clone for IBITSExtensionSetup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IBITSExtensionSetup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IBITSExtensionSetup {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IBITSExtensionSetup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBITSExtensionSetup").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IBITSExtensionSetup {
    type Vtable = IBITSExtensionSetup_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29cfbbf7_09e4_4b97_b0bc_f2287e3d8eb3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IBITSExtensionSetup_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub EnableBITSUploads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisableBITSUploads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCleanupTaskName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptaskname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCleanupTaskName: usize,
    pub GetCleanupTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IBITSExtensionSetupFactory(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IBITSExtensionSetupFactory {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetObject<'a, P0>(&self, path: P0) -> ::windows::core::Result<IBITSExtensionSetup>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetObject)(::windows::core::Interface::as_raw(self), path.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IBITSExtensionSetup>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IBITSExtensionSetupFactory> for ::windows::core::IUnknown {
    fn from(value: IBITSExtensionSetupFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IBITSExtensionSetupFactory> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBITSExtensionSetupFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IBITSExtensionSetupFactory> for ::windows::core::IUnknown {
    fn from(value: &IBITSExtensionSetupFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IBITSExtensionSetupFactory> for super::super::System::Com::IDispatch {
    fn from(value: IBITSExtensionSetupFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IBITSExtensionSetupFactory> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IBITSExtensionSetupFactory) -> Self {
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
impl ::core::clone::Clone for IBITSExtensionSetupFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IBITSExtensionSetupFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IBITSExtensionSetupFactory {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IBITSExtensionSetupFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBITSExtensionSetupFactory").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IBITSExtensionSetupFactory {
    type Vtable = IBITSExtensionSetupFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5d2d542_5503_4e64_8b48_72ef91a32ee1);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IBITSExtensionSetupFactory_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppextensionsetup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetObject: usize,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IBackgroundCopyCallback(::windows::core::IUnknown);
impl IBackgroundCopyCallback {
    pub unsafe fn JobTransferred<'a, P0>(&self, pjob: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IBackgroundCopyJob>>,
    {
        (::windows::core::Interface::vtable(self).JobTransferred)(::windows::core::Interface::as_raw(self), pjob.into().abi()).ok()
    }
    pub unsafe fn JobError<'a, P0, P1>(&self, pjob: P0, perror: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IBackgroundCopyJob>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IBackgroundCopyError>>,
    {
        (::windows::core::Interface::vtable(self).JobError)(::windows::core::Interface::as_raw(self), pjob.into().abi(), perror.into().abi()).ok()
    }
    pub unsafe fn JobModification<'a, P0>(&self, pjob: P0, dwreserved: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IBackgroundCopyJob>>,
    {
        (::windows::core::Interface::vtable(self).JobModification)(::windows::core::Interface::as_raw(self), pjob.into().abi(), dwreserved).ok()
    }
}
impl ::core::convert::From<IBackgroundCopyCallback> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyCallback> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBackgroundCopyCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyCallback> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IBackgroundCopyCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundCopyCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundCopyCallback {}
impl ::core::fmt::Debug for IBackgroundCopyCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundCopyCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyCallback {
    type Vtable = IBackgroundCopyCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97ea99c7_0186_4ad4_8df9_c5b4e0ed6b22);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyCallback_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub JobTransferred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pjob: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub JobError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pjob: *mut ::core::ffi::c_void, perror: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub JobModification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pjob: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IBackgroundCopyCallback1(::windows::core::IUnknown);
impl IBackgroundCopyCallback1 {
    pub unsafe fn OnStatus<'a, P0, P1>(&self, pgroup: P0, pjob: P1, dwfileindex: u32, dwstatus: u32, dwnumofretries: u32, dwwin32result: u32, dwtransportresult: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IBackgroundCopyGroup>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IBackgroundCopyJob1>>,
    {
        (::windows::core::Interface::vtable(self).OnStatus)(::windows::core::Interface::as_raw(self), pgroup.into().abi(), pjob.into().abi(), dwfileindex, dwstatus, dwnumofretries, dwwin32result, dwtransportresult).ok()
    }
    pub unsafe fn OnProgress<'a, P0, P1>(&self, progresstype: u32, pgroup: P0, pjob: P1, dwfileindex: u32, dwprogressvalue: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IBackgroundCopyGroup>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IBackgroundCopyJob1>>,
    {
        (::windows::core::Interface::vtable(self).OnProgress)(::windows::core::Interface::as_raw(self), progresstype, pgroup.into().abi(), pjob.into().abi(), dwfileindex, dwprogressvalue).ok()
    }
    pub unsafe fn OnProgressEx<'a, P0, P1>(&self, progresstype: u32, pgroup: P0, pjob: P1, dwfileindex: u32, dwprogressvalue: u32, pbyte: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IBackgroundCopyGroup>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IBackgroundCopyJob1>>,
    {
        (::windows::core::Interface::vtable(self).OnProgressEx)(::windows::core::Interface::as_raw(self), progresstype, pgroup.into().abi(), pjob.into().abi(), dwfileindex, dwprogressvalue, pbyte.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pbyte))).ok()
    }
}
impl ::core::convert::From<IBackgroundCopyCallback1> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyCallback1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyCallback1> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBackgroundCopyCallback1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyCallback1> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyCallback1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IBackgroundCopyCallback1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundCopyCallback1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundCopyCallback1 {}
impl ::core::fmt::Debug for IBackgroundCopyCallback1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundCopyCallback1").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyCallback1 {
    type Vtable = IBackgroundCopyCallback1_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x084f6593_3800_4e08_9b59_99fa59addf82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyCallback1_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OnStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgroup: *mut ::core::ffi::c_void, pjob: *mut ::core::ffi::c_void, dwfileindex: u32, dwstatus: u32, dwnumofretries: u32, dwwin32result: u32, dwtransportresult: u32) -> ::windows::core::HRESULT,
    pub OnProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, progresstype: u32, pgroup: *mut ::core::ffi::c_void, pjob: *mut ::core::ffi::c_void, dwfileindex: u32, dwprogressvalue: u32) -> ::windows::core::HRESULT,
    pub OnProgressEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, progresstype: u32, pgroup: *mut ::core::ffi::c_void, pjob: *mut ::core::ffi::c_void, dwfileindex: u32, dwprogressvalue: u32, dwbytearraysize: u32, pbyte: *const u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IBackgroundCopyCallback2(::windows::core::IUnknown);
impl IBackgroundCopyCallback2 {
    pub unsafe fn JobTransferred<'a, P0>(&self, pjob: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IBackgroundCopyJob>>,
    {
        (::windows::core::Interface::vtable(self).base__.JobTransferred)(::windows::core::Interface::as_raw(self), pjob.into().abi()).ok()
    }
    pub unsafe fn JobError<'a, P0, P1>(&self, pjob: P0, perror: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IBackgroundCopyJob>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IBackgroundCopyError>>,
    {
        (::windows::core::Interface::vtable(self).base__.JobError)(::windows::core::Interface::as_raw(self), pjob.into().abi(), perror.into().abi()).ok()
    }
    pub unsafe fn JobModification<'a, P0>(&self, pjob: P0, dwreserved: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IBackgroundCopyJob>>,
    {
        (::windows::core::Interface::vtable(self).base__.JobModification)(::windows::core::Interface::as_raw(self), pjob.into().abi(), dwreserved).ok()
    }
    pub unsafe fn FileTransferred<'a, P0, P1>(&self, pjob: P0, pfile: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IBackgroundCopyJob>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IBackgroundCopyFile>>,
    {
        (::windows::core::Interface::vtable(self).FileTransferred)(::windows::core::Interface::as_raw(self), pjob.into().abi(), pfile.into().abi()).ok()
    }
}
impl ::core::convert::From<IBackgroundCopyCallback2> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyCallback2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyCallback2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBackgroundCopyCallback2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyCallback2> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyCallback2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundCopyCallback2> for IBackgroundCopyCallback {
    fn from(value: IBackgroundCopyCallback2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyCallback2> for &'a IBackgroundCopyCallback {
    fn from(value: &'a IBackgroundCopyCallback2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyCallback2> for IBackgroundCopyCallback {
    fn from(value: &IBackgroundCopyCallback2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IBackgroundCopyCallback2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundCopyCallback2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundCopyCallback2 {}
impl ::core::fmt::Debug for IBackgroundCopyCallback2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundCopyCallback2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyCallback2 {
    type Vtable = IBackgroundCopyCallback2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x659cdeac_489e_11d9_a9cd_000d56965251);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyCallback2_Vtbl {
    pub base__: IBackgroundCopyCallback_Vtbl,
    pub FileTransferred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pjob: *mut ::core::ffi::c_void, pfile: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IBackgroundCopyCallback3(::windows::core::IUnknown);
impl IBackgroundCopyCallback3 {
    pub unsafe fn JobTransferred<'a, P0>(&self, pjob: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IBackgroundCopyJob>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.JobTransferred)(::windows::core::Interface::as_raw(self), pjob.into().abi()).ok()
    }
    pub unsafe fn JobError<'a, P0, P1>(&self, pjob: P0, perror: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IBackgroundCopyJob>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IBackgroundCopyError>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.JobError)(::windows::core::Interface::as_raw(self), pjob.into().abi(), perror.into().abi()).ok()
    }
    pub unsafe fn JobModification<'a, P0>(&self, pjob: P0, dwreserved: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IBackgroundCopyJob>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.JobModification)(::windows::core::Interface::as_raw(self), pjob.into().abi(), dwreserved).ok()
    }
    pub unsafe fn FileTransferred<'a, P0, P1>(&self, pjob: P0, pfile: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IBackgroundCopyJob>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IBackgroundCopyFile>>,
    {
        (::windows::core::Interface::vtable(self).base__.FileTransferred)(::windows::core::Interface::as_raw(self), pjob.into().abi(), pfile.into().abi()).ok()
    }
    pub unsafe fn FileRangesTransferred<'a, P0, P1>(&self, job: P0, file: P1, ranges: &[BG_FILE_RANGE]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IBackgroundCopyJob>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IBackgroundCopyFile>>,
    {
        (::windows::core::Interface::vtable(self).FileRangesTransferred)(::windows::core::Interface::as_raw(self), job.into().abi(), file.into().abi(), ranges.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ranges))).ok()
    }
}
impl ::core::convert::From<IBackgroundCopyCallback3> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyCallback3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyCallback3> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBackgroundCopyCallback3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyCallback3> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyCallback3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundCopyCallback3> for IBackgroundCopyCallback {
    fn from(value: IBackgroundCopyCallback3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyCallback3> for &'a IBackgroundCopyCallback {
    fn from(value: &'a IBackgroundCopyCallback3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyCallback3> for IBackgroundCopyCallback {
    fn from(value: &IBackgroundCopyCallback3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundCopyCallback3> for IBackgroundCopyCallback2 {
    fn from(value: IBackgroundCopyCallback3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyCallback3> for &'a IBackgroundCopyCallback2 {
    fn from(value: &'a IBackgroundCopyCallback3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyCallback3> for IBackgroundCopyCallback2 {
    fn from(value: &IBackgroundCopyCallback3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IBackgroundCopyCallback3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundCopyCallback3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundCopyCallback3 {}
impl ::core::fmt::Debug for IBackgroundCopyCallback3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundCopyCallback3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyCallback3 {
    type Vtable = IBackgroundCopyCallback3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98c97bd2_e32b_4ad8_a528_95fd8b16bd42);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyCallback3_Vtbl {
    pub base__: IBackgroundCopyCallback2_Vtbl,
    pub FileRangesTransferred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, job: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IBackgroundCopyError(::windows::core::IUnknown);
impl IBackgroundCopyError {
    pub unsafe fn GetError(&self, pcontext: *mut BG_ERROR_CONTEXT, pcode: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetError)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcontext), ::core::mem::transmute(pcode)).ok()
    }
    pub unsafe fn GetFile(&self) -> ::windows::core::Result<IBackgroundCopyFile> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetFile)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IBackgroundCopyFile>(result__)
    }
    pub unsafe fn GetErrorDescription(&self, languageid: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetErrorDescription)(::windows::core::Interface::as_raw(self), languageid, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetErrorContextDescription(&self, languageid: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetErrorContextDescription)(::windows::core::Interface::as_raw(self), languageid, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetProtocol(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetProtocol)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IBackgroundCopyError> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyError> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBackgroundCopyError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyError> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IBackgroundCopyError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundCopyError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundCopyError {}
impl ::core::fmt::Debug for IBackgroundCopyError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundCopyError").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyError {
    type Vtable = IBackgroundCopyError_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19c613a0_fcb8_4f28_81ae_897c3d078f81);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyError_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *mut BG_ERROR_CONTEXT, pcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub GetFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetErrorDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languageid: u32, perrordescription: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetErrorContextDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languageid: u32, pcontextdescription: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetProtocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprotocol: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IBackgroundCopyFile(::windows::core::IUnknown);
impl IBackgroundCopyFile {
    pub unsafe fn GetRemoteName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetRemoteName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetLocalName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetLocalName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProgress(&self) -> ::windows::core::Result<BG_FILE_PROGRESS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetProgress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_FILE_PROGRESS>(result__)
    }
}
impl ::core::convert::From<IBackgroundCopyFile> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyFile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyFile> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBackgroundCopyFile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyFile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IBackgroundCopyFile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundCopyFile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundCopyFile {}
impl ::core::fmt::Debug for IBackgroundCopyFile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundCopyFile").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyFile {
    type Vtable = IBackgroundCopyFile_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01b7bd23_fb88_4a77_8490_5891d3e4653a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyFile_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetRemoteName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetLocalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut BG_FILE_PROGRESS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetProgress: usize,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IBackgroundCopyFile2(::windows::core::IUnknown);
impl IBackgroundCopyFile2 {
    pub unsafe fn GetRemoteName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetRemoteName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetLocalName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetLocalName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProgress(&self) -> ::windows::core::Result<BG_FILE_PROGRESS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetProgress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_FILE_PROGRESS>(result__)
    }
    pub unsafe fn GetFileRanges(&self, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFileRanges)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(rangecount), ::core::mem::transmute(ranges)).ok()
    }
    pub unsafe fn SetRemoteName<'a, P0>(&self, val: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetRemoteName)(::windows::core::Interface::as_raw(self), val.into()).ok()
    }
}
impl ::core::convert::From<IBackgroundCopyFile2> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyFile2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyFile2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBackgroundCopyFile2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile2> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyFile2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundCopyFile2> for IBackgroundCopyFile {
    fn from(value: IBackgroundCopyFile2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyFile2> for &'a IBackgroundCopyFile {
    fn from(value: &'a IBackgroundCopyFile2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile2> for IBackgroundCopyFile {
    fn from(value: &IBackgroundCopyFile2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IBackgroundCopyFile2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundCopyFile2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundCopyFile2 {}
impl ::core::fmt::Debug for IBackgroundCopyFile2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundCopyFile2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyFile2 {
    type Vtable = IBackgroundCopyFile2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83e81b93_0873_474d_8a8c_f2018b1a939c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyFile2_Vtbl {
    pub base__: IBackgroundCopyFile_Vtbl,
    pub GetFileRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::HRESULT,
    pub SetRemoteName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, val: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IBackgroundCopyFile3(::windows::core::IUnknown);
impl IBackgroundCopyFile3 {
    pub unsafe fn GetRemoteName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetRemoteName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetLocalName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetLocalName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProgress(&self) -> ::windows::core::Result<BG_FILE_PROGRESS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetProgress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_FILE_PROGRESS>(result__)
    }
    pub unsafe fn GetFileRanges(&self, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFileRanges)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(rangecount), ::core::mem::transmute(ranges)).ok()
    }
    pub unsafe fn SetRemoteName<'a, P0>(&self, val: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetRemoteName)(::windows::core::Interface::as_raw(self), val.into()).ok()
    }
    pub unsafe fn GetTemporaryName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetTemporaryName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValidationState<'a, P0>(&self, state: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetValidationState)(::windows::core::Interface::as_raw(self), state.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetValidationState(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetValidationState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDownloadedFromPeer(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsDownloadedFromPeer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IBackgroundCopyFile3> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyFile3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyFile3> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBackgroundCopyFile3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile3> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyFile3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundCopyFile3> for IBackgroundCopyFile {
    fn from(value: IBackgroundCopyFile3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyFile3> for &'a IBackgroundCopyFile {
    fn from(value: &'a IBackgroundCopyFile3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile3> for IBackgroundCopyFile {
    fn from(value: &IBackgroundCopyFile3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundCopyFile3> for IBackgroundCopyFile2 {
    fn from(value: IBackgroundCopyFile3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyFile3> for &'a IBackgroundCopyFile2 {
    fn from(value: &'a IBackgroundCopyFile3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile3> for IBackgroundCopyFile2 {
    fn from(value: &IBackgroundCopyFile3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IBackgroundCopyFile3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundCopyFile3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundCopyFile3 {}
impl ::core::fmt::Debug for IBackgroundCopyFile3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundCopyFile3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyFile3 {
    type Vtable = IBackgroundCopyFile3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x659cdeaa_489e_11d9_a9cd_000d56965251);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyFile3_Vtbl {
    pub base__: IBackgroundCopyFile2_Vtbl,
    pub GetTemporaryName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilename: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetValidationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetValidationState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetValidationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetValidationState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsDownloadedFromPeer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsDownloadedFromPeer: usize,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IBackgroundCopyFile4(::windows::core::IUnknown);
impl IBackgroundCopyFile4 {
    pub unsafe fn GetRemoteName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetRemoteName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetLocalName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetLocalName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProgress(&self) -> ::windows::core::Result<BG_FILE_PROGRESS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetProgress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_FILE_PROGRESS>(result__)
    }
    pub unsafe fn GetFileRanges(&self, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetFileRanges)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(rangecount), ::core::mem::transmute(ranges)).ok()
    }
    pub unsafe fn SetRemoteName<'a, P0>(&self, val: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetRemoteName)(::windows::core::Interface::as_raw(self), val.into()).ok()
    }
    pub unsafe fn GetTemporaryName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetTemporaryName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValidationState<'a, P0>(&self, state: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetValidationState)(::windows::core::Interface::as_raw(self), state.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetValidationState(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetValidationState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDownloadedFromPeer(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.IsDownloadedFromPeer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetPeerDownloadStats(&self, pfromorigin: *mut u64, pfrompeers: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPeerDownloadStats)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pfromorigin), ::core::mem::transmute(pfrompeers)).ok()
    }
}
impl ::core::convert::From<IBackgroundCopyFile4> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyFile4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyFile4> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBackgroundCopyFile4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile4> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyFile4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundCopyFile4> for IBackgroundCopyFile {
    fn from(value: IBackgroundCopyFile4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyFile4> for &'a IBackgroundCopyFile {
    fn from(value: &'a IBackgroundCopyFile4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile4> for IBackgroundCopyFile {
    fn from(value: &IBackgroundCopyFile4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundCopyFile4> for IBackgroundCopyFile2 {
    fn from(value: IBackgroundCopyFile4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyFile4> for &'a IBackgroundCopyFile2 {
    fn from(value: &'a IBackgroundCopyFile4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile4> for IBackgroundCopyFile2 {
    fn from(value: &IBackgroundCopyFile4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundCopyFile4> for IBackgroundCopyFile3 {
    fn from(value: IBackgroundCopyFile4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyFile4> for &'a IBackgroundCopyFile3 {
    fn from(value: &'a IBackgroundCopyFile4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile4> for IBackgroundCopyFile3 {
    fn from(value: &IBackgroundCopyFile4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IBackgroundCopyFile4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundCopyFile4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundCopyFile4 {}
impl ::core::fmt::Debug for IBackgroundCopyFile4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundCopyFile4").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyFile4 {
    type Vtable = IBackgroundCopyFile4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef7e0655_7888_4960_b0e5_730846e03492);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyFile4_Vtbl {
    pub base__: IBackgroundCopyFile3_Vtbl,
    pub GetPeerDownloadStats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfromorigin: *mut u64, pfrompeers: *mut u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IBackgroundCopyFile5(::windows::core::IUnknown);
impl IBackgroundCopyFile5 {
    pub unsafe fn GetRemoteName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetRemoteName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetLocalName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetLocalName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProgress(&self) -> ::windows::core::Result<BG_FILE_PROGRESS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetProgress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_FILE_PROGRESS>(result__)
    }
    pub unsafe fn GetFileRanges(&self, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetFileRanges)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(rangecount), ::core::mem::transmute(ranges)).ok()
    }
    pub unsafe fn SetRemoteName<'a, P0>(&self, val: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetRemoteName)(::windows::core::Interface::as_raw(self), val.into()).ok()
    }
    pub unsafe fn GetTemporaryName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetTemporaryName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValidationState<'a, P0>(&self, state: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetValidationState)(::windows::core::Interface::as_raw(self), state.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetValidationState(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetValidationState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDownloadedFromPeer(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.IsDownloadedFromPeer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetPeerDownloadStats(&self, pfromorigin: *mut u64, pfrompeers: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPeerDownloadStats)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pfromorigin), ::core::mem::transmute(pfrompeers)).ok()
    }
    pub unsafe fn SetProperty(&self, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: BITS_FILE_PROPERTY_VALUE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProperty)(::windows::core::Interface::as_raw(self), propertyid, ::core::mem::transmute(propertyvalue)).ok()
    }
    pub unsafe fn GetProperty(&self, propertyid: BITS_FILE_PROPERTY_ID) -> ::windows::core::Result<BITS_FILE_PROPERTY_VALUE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetProperty)(::windows::core::Interface::as_raw(self), propertyid, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BITS_FILE_PROPERTY_VALUE>(result__)
    }
}
impl ::core::convert::From<IBackgroundCopyFile5> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyFile5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyFile5> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBackgroundCopyFile5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile5> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyFile5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundCopyFile5> for IBackgroundCopyFile {
    fn from(value: IBackgroundCopyFile5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyFile5> for &'a IBackgroundCopyFile {
    fn from(value: &'a IBackgroundCopyFile5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile5> for IBackgroundCopyFile {
    fn from(value: &IBackgroundCopyFile5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundCopyFile5> for IBackgroundCopyFile2 {
    fn from(value: IBackgroundCopyFile5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyFile5> for &'a IBackgroundCopyFile2 {
    fn from(value: &'a IBackgroundCopyFile5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile5> for IBackgroundCopyFile2 {
    fn from(value: &IBackgroundCopyFile5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundCopyFile5> for IBackgroundCopyFile3 {
    fn from(value: IBackgroundCopyFile5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyFile5> for &'a IBackgroundCopyFile3 {
    fn from(value: &'a IBackgroundCopyFile5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile5> for IBackgroundCopyFile3 {
    fn from(value: &IBackgroundCopyFile5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundCopyFile5> for IBackgroundCopyFile4 {
    fn from(value: IBackgroundCopyFile5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyFile5> for &'a IBackgroundCopyFile4 {
    fn from(value: &'a IBackgroundCopyFile5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile5> for IBackgroundCopyFile4 {
    fn from(value: &IBackgroundCopyFile5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IBackgroundCopyFile5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundCopyFile5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundCopyFile5 {}
impl ::core::fmt::Debug for IBackgroundCopyFile5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundCopyFile5").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyFile5 {
    type Vtable = IBackgroundCopyFile5_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85c1657f_dafc_40e8_8834_df18ea25717e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyFile5_Vtbl {
    pub base__: IBackgroundCopyFile4_Vtbl,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: BITS_FILE_PROPERTY_VALUE) -> ::windows::core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: *mut BITS_FILE_PROPERTY_VALUE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IBackgroundCopyFile6(::windows::core::IUnknown);
impl IBackgroundCopyFile6 {
    pub unsafe fn GetRemoteName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetRemoteName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetLocalName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetLocalName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProgress(&self) -> ::windows::core::Result<BG_FILE_PROGRESS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.base__.GetProgress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_FILE_PROGRESS>(result__)
    }
    pub unsafe fn GetFileRanges(&self, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetFileRanges)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(rangecount), ::core::mem::transmute(ranges)).ok()
    }
    pub unsafe fn SetRemoteName<'a, P0>(&self, val: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetRemoteName)(::windows::core::Interface::as_raw(self), val.into()).ok()
    }
    pub unsafe fn GetTemporaryName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetTemporaryName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValidationState<'a, P0>(&self, state: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetValidationState)(::windows::core::Interface::as_raw(self), state.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetValidationState(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetValidationState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDownloadedFromPeer(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.IsDownloadedFromPeer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetPeerDownloadStats(&self, pfromorigin: *mut u64, pfrompeers: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPeerDownloadStats)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pfromorigin), ::core::mem::transmute(pfrompeers)).ok()
    }
    pub unsafe fn SetProperty(&self, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: BITS_FILE_PROPERTY_VALUE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetProperty)(::windows::core::Interface::as_raw(self), propertyid, ::core::mem::transmute(propertyvalue)).ok()
    }
    pub unsafe fn GetProperty(&self, propertyid: BITS_FILE_PROPERTY_ID) -> ::windows::core::Result<BITS_FILE_PROPERTY_VALUE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetProperty)(::windows::core::Interface::as_raw(self), propertyid, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BITS_FILE_PROPERTY_VALUE>(result__)
    }
    pub unsafe fn UpdateDownloadPosition(&self, offset: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateDownloadPosition)(::windows::core::Interface::as_raw(self), offset).ok()
    }
    pub unsafe fn RequestFileRanges(&self, ranges: &[BG_FILE_RANGE]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RequestFileRanges)(::windows::core::Interface::as_raw(self), ranges.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ranges))).ok()
    }
    pub unsafe fn GetFilledFileRanges(&self, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFilledFileRanges)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(rangecount), ::core::mem::transmute(ranges)).ok()
    }
}
impl ::core::convert::From<IBackgroundCopyFile6> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyFile6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyFile6> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBackgroundCopyFile6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile6> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyFile6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundCopyFile6> for IBackgroundCopyFile {
    fn from(value: IBackgroundCopyFile6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyFile6> for &'a IBackgroundCopyFile {
    fn from(value: &'a IBackgroundCopyFile6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile6> for IBackgroundCopyFile {
    fn from(value: &IBackgroundCopyFile6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundCopyFile6> for IBackgroundCopyFile2 {
    fn from(value: IBackgroundCopyFile6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyFile6> for &'a IBackgroundCopyFile2 {
    fn from(value: &'a IBackgroundCopyFile6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile6> for IBackgroundCopyFile2 {
    fn from(value: &IBackgroundCopyFile6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundCopyFile6> for IBackgroundCopyFile3 {
    fn from(value: IBackgroundCopyFile6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyFile6> for &'a IBackgroundCopyFile3 {
    fn from(value: &'a IBackgroundCopyFile6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile6> for IBackgroundCopyFile3 {
    fn from(value: &IBackgroundCopyFile6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundCopyFile6> for IBackgroundCopyFile4 {
    fn from(value: IBackgroundCopyFile6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyFile6> for &'a IBackgroundCopyFile4 {
    fn from(value: &'a IBackgroundCopyFile6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile6> for IBackgroundCopyFile4 {
    fn from(value: &IBackgroundCopyFile6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundCopyFile6> for IBackgroundCopyFile5 {
    fn from(value: IBackgroundCopyFile6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyFile6> for &'a IBackgroundCopyFile5 {
    fn from(value: &'a IBackgroundCopyFile6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyFile6> for IBackgroundCopyFile5 {
    fn from(value: &IBackgroundCopyFile6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IBackgroundCopyFile6 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundCopyFile6 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundCopyFile6 {}
impl ::core::fmt::Debug for IBackgroundCopyFile6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundCopyFile6").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyFile6 {
    type Vtable = IBackgroundCopyFile6_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf6784f7_d677_49fd_9368_cb47aee9d1ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyFile6_Vtbl {
    pub base__: IBackgroundCopyFile5_Vtbl,
    pub UpdateDownloadPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: u64) -> ::windows::core::HRESULT,
    pub RequestFileRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows::core::HRESULT,
    pub GetFilledFileRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IBackgroundCopyGroup(::windows::core::IUnknown);
impl IBackgroundCopyGroup {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProp(&self, propid: GROUPPROP) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetProp)(::windows::core::Interface::as_raw(self), propid, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProp(&self, propid: GROUPPROP, pvarval: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProp)(::windows::core::Interface::as_raw(self), propid, ::core::mem::transmute(pvarval)).ok()
    }
    pub unsafe fn GetProgress(&self, dwflags: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetProgress)(::windows::core::Interface::as_raw(self), dwflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetStatus(&self, pdwstatus: *mut u32, pdwjobindex: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwstatus), ::core::mem::transmute(pdwjobindex)).ok()
    }
    pub unsafe fn GetJob(&self, jobid: ::windows::core::GUID) -> ::windows::core::Result<IBackgroundCopyJob1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetJob)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(jobid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IBackgroundCopyJob1>(result__)
    }
    pub unsafe fn SuspendGroup(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SuspendGroup)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ResumeGroup(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResumeGroup)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CancelGroup(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CancelGroup)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Size(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Size)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GroupID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GroupID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn CreateJob(&self, guidjobid: ::windows::core::GUID) -> ::windows::core::Result<IBackgroundCopyJob1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateJob)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guidjobid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IBackgroundCopyJob1>(result__)
    }
    pub unsafe fn EnumJobs(&self, dwflags: u32) -> ::windows::core::Result<IEnumBackgroundCopyJobs1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EnumJobs)(::windows::core::Interface::as_raw(self), dwflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumBackgroundCopyJobs1>(result__)
    }
    pub unsafe fn SwitchToForeground(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SwitchToForeground)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn QueryNewJobInterface(&self, iid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).QueryNewJobInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(iid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn SetNotificationPointer<'a, P0>(&self, iid: *const ::windows::core::GUID, punk: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).SetNotificationPointer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(iid), punk.into().abi()).ok()
    }
}
impl ::core::convert::From<IBackgroundCopyGroup> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyGroup> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBackgroundCopyGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyGroup> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IBackgroundCopyGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundCopyGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundCopyGroup {}
impl ::core::fmt::Debug for IBackgroundCopyGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundCopyGroup").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyGroup {
    type Vtable = IBackgroundCopyGroup_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ded80a7_53ea_424f_8a04_17fea9adc4f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyGroup_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propid: GROUPPROP, pvarval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProp: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propid: GROUPPROP, pvarval: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProp: usize,
    pub GetProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pdwprogress: *mut u32) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32, pdwjobindex: *mut u32) -> ::windows::core::HRESULT,
    pub GetJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobid: ::windows::core::GUID, ppjob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SuspendGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ResumeGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CancelGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwsize: *mut u32) -> ::windows::core::HRESULT,
    pub GroupID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidgroupid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub CreateJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidjobid: ::windows::core::GUID, ppjob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumJobs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, ppenumjobs: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SwitchToForeground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryNewJobInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetNotificationPointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IBackgroundCopyJob(::windows::core::IUnknown);
impl IBackgroundCopyJob {
    pub unsafe fn AddFileSet(&self, pfileset: &[BG_FILE_INFO]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddFileSet)(::windows::core::Interface::as_raw(self), pfileset.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pfileset))).ok()
    }
    pub unsafe fn AddFile<'a, P0, P1>(&self, remoteurl: P0, localname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddFile)(::windows::core::Interface::as_raw(self), remoteurl.into(), localname.into()).ok()
    }
    pub unsafe fn EnumFiles(&self) -> ::windows::core::Result<IEnumBackgroundCopyFiles> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EnumFiles)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumBackgroundCopyFiles>(result__)
    }
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Suspend)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Resume)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Complete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Complete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<BG_JOB_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_JOB_TYPE>(result__)
    }
    pub unsafe fn GetProgress(&self) -> ::windows::core::Result<BG_JOB_PROGRESS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetProgress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_JOB_PROGRESS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTimes(&self) -> ::windows::core::Result<BG_JOB_TIMES> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetTimes)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_JOB_TIMES>(result__)
    }
    pub unsafe fn GetState(&self) -> ::windows::core::Result<BG_JOB_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_JOB_STATE>(result__)
    }
    pub unsafe fn GetError(&self) -> ::windows::core::Result<IBackgroundCopyError> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetError)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IBackgroundCopyError>(result__)
    }
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetOwner)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetDisplayName<'a, P0>(&self, val: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetDisplayName)(::windows::core::Interface::as_raw(self), val.into()).ok()
    }
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetDisplayName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, P0>(&self, val: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetDescription)(::windows::core::Interface::as_raw(self), val.into()).ok()
    }
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetDescription)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetPriority(&self, val: BG_JOB_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPriority)(::windows::core::Interface::as_raw(self), val).ok()
    }
    pub unsafe fn GetPriority(&self) -> ::windows::core::Result<BG_JOB_PRIORITY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetPriority)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_JOB_PRIORITY>(result__)
    }
    pub unsafe fn SetNotifyFlags(&self, val: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNotifyFlags)(::windows::core::Interface::as_raw(self), val).ok()
    }
    pub unsafe fn GetNotifyFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetNotifyFlags)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetNotifyInterface<'a, P0>(&self, val: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).SetNotifyInterface)(::windows::core::Interface::as_raw(self), val.into().abi()).ok()
    }
    pub unsafe fn GetNotifyInterface(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetNotifyInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn SetMinimumRetryDelay(&self, seconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMinimumRetryDelay)(::windows::core::Interface::as_raw(self), seconds).ok()
    }
    pub unsafe fn GetMinimumRetryDelay(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMinimumRetryDelay)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetNoProgressTimeout(&self, seconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNoProgressTimeout)(::windows::core::Interface::as_raw(self), seconds).ok()
    }
    pub unsafe fn GetNoProgressTimeout(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetNoProgressTimeout)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetErrorCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetErrorCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetProxySettings<'a, P0, P1>(&self, proxyusage: BG_JOB_PROXY_USAGE, proxylist: P0, proxybypasslist: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetProxySettings)(::windows::core::Interface::as_raw(self), proxyusage, proxylist.into(), proxybypasslist.into()).ok()
    }
    pub unsafe fn GetProxySettings(&self, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut ::windows::core::PWSTR, pproxybypasslist: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProxySettings)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pproxyusage), ::core::mem::transmute(pproxylist), ::core::mem::transmute(pproxybypasslist)).ok()
    }
    pub unsafe fn TakeOwnership(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TakeOwnership)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IBackgroundCopyJob> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyJob> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBackgroundCopyJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJob> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IBackgroundCopyJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundCopyJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundCopyJob {}
impl ::core::fmt::Debug for IBackgroundCopyJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundCopyJob").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyJob {
    type Vtable = IBackgroundCopyJob_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x37668d37_507e_4160_9316_26306d150b12);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJob_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub AddFileSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cfilecount: u32, pfileset: *const BG_FILE_INFO) -> ::windows::core::HRESULT,
    pub AddFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteurl: ::windows::core::PCWSTR, localname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub EnumFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Suspend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut BG_JOB_TYPE) -> ::windows::core::HRESULT,
    pub GetProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut BG_JOB_PROGRESS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTimes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut BG_JOB_TIMES) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTimes: usize,
    pub GetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut BG_JOB_STATE) -> ::windows::core::HRESULT,
    pub GetError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pperror: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, val: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, val: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, val: BG_JOB_PRIORITY) -> ::windows::core::HRESULT,
    pub GetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut BG_JOB_PRIORITY) -> ::windows::core::HRESULT,
    pub SetNotifyFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, val: u32) -> ::windows::core::HRESULT,
    pub GetNotifyFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT,
    pub SetNotifyInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, val: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNotifyInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMinimumRetryDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, seconds: u32) -> ::windows::core::HRESULT,
    pub GetMinimumRetryDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, seconds: *mut u32) -> ::windows::core::HRESULT,
    pub SetNoProgressTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, seconds: u32) -> ::windows::core::HRESULT,
    pub GetNoProgressTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, seconds: *mut u32) -> ::windows::core::HRESULT,
    pub GetErrorCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errors: *mut u32) -> ::windows::core::HRESULT,
    pub SetProxySettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, proxyusage: BG_JOB_PROXY_USAGE, proxylist: ::windows::core::PCWSTR, proxybypasslist: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetProxySettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut ::windows::core::PWSTR, pproxybypasslist: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub TakeOwnership: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IBackgroundCopyJob1(::windows::core::IUnknown);
impl IBackgroundCopyJob1 {
    pub unsafe fn CancelJob(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CancelJob)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetProgress(&self, dwflags: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetProgress)(::windows::core::Interface::as_raw(self), dwflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetStatus(&self, pdwstatus: *mut u32, pdwwin32result: *mut u32, pdwtransportresult: *mut u32, pdwnumofretries: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwstatus), ::core::mem::transmute(pdwwin32result), ::core::mem::transmute(pdwtransportresult), ::core::mem::transmute(pdwnumofretries)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddFiles(&self, ppfileset: &[*const FILESETINFO]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddFiles)(::windows::core::Interface::as_raw(self), ppfileset.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppfileset))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFile(&self, cfileindex: u32) -> ::windows::core::Result<FILESETINFO> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetFile)(::windows::core::Interface::as_raw(self), cfileindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FILESETINFO>(result__)
    }
    pub unsafe fn GetFileCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetFileCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SwitchToForeground(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SwitchToForeground)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn JobID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).JobID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
}
impl ::core::convert::From<IBackgroundCopyJob1> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyJob1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyJob1> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBackgroundCopyJob1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJob1> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyJob1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IBackgroundCopyJob1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundCopyJob1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundCopyJob1 {}
impl ::core::fmt::Debug for IBackgroundCopyJob1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundCopyJob1").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyJob1 {
    type Vtable = IBackgroundCopyJob1_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59f5553c_2031_4629_bb18_2645a6970947);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJob1_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub CancelJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pdwprogress: *mut u32) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32, pdwwin32result: *mut u32, pdwtransportresult: *mut u32, pdwnumofretries: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cfilecount: u32, ppfileset: *const *const FILESETINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddFiles: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cfileindex: u32, pfileinfo: *mut FILESETINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFile: usize,
    pub GetFileCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwfilecount: *mut u32) -> ::windows::core::HRESULT,
    pub SwitchToForeground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub JobID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidjobid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IBackgroundCopyJob2(::windows::core::IUnknown);
impl IBackgroundCopyJob2 {
    pub unsafe fn AddFileSet(&self, pfileset: &[BG_FILE_INFO]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AddFileSet)(::windows::core::Interface::as_raw(self), pfileset.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pfileset))).ok()
    }
    pub unsafe fn AddFile<'a, P0, P1>(&self, remoteurl: P0, localname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddFile)(::windows::core::Interface::as_raw(self), remoteurl.into(), localname.into()).ok()
    }
    pub unsafe fn EnumFiles(&self) -> ::windows::core::Result<IEnumBackgroundCopyFiles> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.EnumFiles)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumBackgroundCopyFiles>(result__)
    }
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Suspend)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Resume)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Cancel)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Complete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Complete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<BG_JOB_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_JOB_TYPE>(result__)
    }
    pub unsafe fn GetProgress(&self) -> ::windows::core::Result<BG_JOB_PROGRESS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetProgress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_JOB_PROGRESS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTimes(&self) -> ::windows::core::Result<BG_JOB_TIMES> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetTimes)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_JOB_TIMES>(result__)
    }
    pub unsafe fn GetState(&self) -> ::windows::core::Result<BG_JOB_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_JOB_STATE>(result__)
    }
    pub unsafe fn GetError(&self) -> ::windows::core::Result<IBackgroundCopyError> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetError)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IBackgroundCopyError>(result__)
    }
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetOwner)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetDisplayName<'a, P0>(&self, val: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetDisplayName)(::windows::core::Interface::as_raw(self), val.into()).ok()
    }
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetDisplayName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, P0>(&self, val: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetDescription)(::windows::core::Interface::as_raw(self), val.into()).ok()
    }
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetDescription)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetPriority(&self, val: BG_JOB_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPriority)(::windows::core::Interface::as_raw(self), val).ok()
    }
    pub unsafe fn GetPriority(&self) -> ::windows::core::Result<BG_JOB_PRIORITY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetPriority)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_JOB_PRIORITY>(result__)
    }
    pub unsafe fn SetNotifyFlags(&self, val: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetNotifyFlags)(::windows::core::Interface::as_raw(self), val).ok()
    }
    pub unsafe fn GetNotifyFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetNotifyFlags)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetNotifyInterface<'a, P0>(&self, val: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetNotifyInterface)(::windows::core::Interface::as_raw(self), val.into().abi()).ok()
    }
    pub unsafe fn GetNotifyInterface(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetNotifyInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn SetMinimumRetryDelay(&self, seconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetMinimumRetryDelay)(::windows::core::Interface::as_raw(self), seconds).ok()
    }
    pub unsafe fn GetMinimumRetryDelay(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetMinimumRetryDelay)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetNoProgressTimeout(&self, seconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetNoProgressTimeout)(::windows::core::Interface::as_raw(self), seconds).ok()
    }
    pub unsafe fn GetNoProgressTimeout(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetNoProgressTimeout)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetErrorCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetErrorCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetProxySettings<'a, P0, P1>(&self, proxyusage: BG_JOB_PROXY_USAGE, proxylist: P0, proxybypasslist: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetProxySettings)(::windows::core::Interface::as_raw(self), proxyusage, proxylist.into(), proxybypasslist.into()).ok()
    }
    pub unsafe fn GetProxySettings(&self, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut ::windows::core::PWSTR, pproxybypasslist: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetProxySettings)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pproxyusage), ::core::mem::transmute(pproxylist), ::core::mem::transmute(pproxybypasslist)).ok()
    }
    pub unsafe fn TakeOwnership(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.TakeOwnership)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetNotifyCmdLine<'a, P0, P1>(&self, program: P0, parameters: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetNotifyCmdLine)(::windows::core::Interface::as_raw(self), program.into(), parameters.into()).ok()
    }
    pub unsafe fn GetNotifyCmdLine(&self, pprogram: *mut ::windows::core::PWSTR, pparameters: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNotifyCmdLine)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pprogram), ::core::mem::transmute(pparameters)).ok()
    }
    pub unsafe fn GetReplyProgress(&self, pprogress: *mut BG_JOB_REPLY_PROGRESS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetReplyProgress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pprogress)).ok()
    }
    pub unsafe fn GetReplyData(&self, ppbuffer: *mut *mut u8, plength: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetReplyData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(plength)).ok()
    }
    pub unsafe fn SetReplyFileName<'a, P0>(&self, replyfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetReplyFileName)(::windows::core::Interface::as_raw(self), replyfilename.into()).ok()
    }
    pub unsafe fn GetReplyFileName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetReplyFileName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetCredentials(&self, credentials: *const BG_AUTH_CREDENTIALS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCredentials)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(credentials)).ok()
    }
    pub unsafe fn RemoveCredentials(&self, target: BG_AUTH_TARGET, scheme: BG_AUTH_SCHEME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveCredentials)(::windows::core::Interface::as_raw(self), target, scheme).ok()
    }
}
impl ::core::convert::From<IBackgroundCopyJob2> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyJob2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyJob2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBackgroundCopyJob2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJob2> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyJob2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundCopyJob2> for IBackgroundCopyJob {
    fn from(value: IBackgroundCopyJob2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyJob2> for &'a IBackgroundCopyJob {
    fn from(value: &'a IBackgroundCopyJob2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJob2> for IBackgroundCopyJob {
    fn from(value: &IBackgroundCopyJob2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IBackgroundCopyJob2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundCopyJob2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundCopyJob2 {}
impl ::core::fmt::Debug for IBackgroundCopyJob2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundCopyJob2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyJob2 {
    type Vtable = IBackgroundCopyJob2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54b50739_686f_45eb_9dff_d6a9a0faa9af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJob2_Vtbl {
    pub base__: IBackgroundCopyJob_Vtbl,
    pub SetNotifyCmdLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, program: ::windows::core::PCWSTR, parameters: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetNotifyCmdLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprogram: *mut ::windows::core::PWSTR, pparameters: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetReplyProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprogress: *mut BG_JOB_REPLY_PROGRESS) -> ::windows::core::HRESULT,
    pub GetReplyData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbuffer: *mut *mut u8, plength: *mut u64) -> ::windows::core::HRESULT,
    pub SetReplyFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, replyfilename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetReplyFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preplyfilename: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credentials: *const BG_AUTH_CREDENTIALS) -> ::windows::core::HRESULT,
    pub RemoveCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: BG_AUTH_TARGET, scheme: BG_AUTH_SCHEME) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IBackgroundCopyJob3(::windows::core::IUnknown);
impl IBackgroundCopyJob3 {
    pub unsafe fn AddFileSet(&self, pfileset: &[BG_FILE_INFO]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.AddFileSet)(::windows::core::Interface::as_raw(self), pfileset.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pfileset))).ok()
    }
    pub unsafe fn AddFile<'a, P0, P1>(&self, remoteurl: P0, localname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.AddFile)(::windows::core::Interface::as_raw(self), remoteurl.into(), localname.into()).ok()
    }
    pub unsafe fn EnumFiles(&self) -> ::windows::core::Result<IEnumBackgroundCopyFiles> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.EnumFiles)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumBackgroundCopyFiles>(result__)
    }
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Suspend)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Resume)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Cancel)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Complete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Complete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<BG_JOB_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_JOB_TYPE>(result__)
    }
    pub unsafe fn GetProgress(&self) -> ::windows::core::Result<BG_JOB_PROGRESS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetProgress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_JOB_PROGRESS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTimes(&self) -> ::windows::core::Result<BG_JOB_TIMES> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetTimes)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_JOB_TIMES>(result__)
    }
    pub unsafe fn GetState(&self) -> ::windows::core::Result<BG_JOB_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_JOB_STATE>(result__)
    }
    pub unsafe fn GetError(&self) -> ::windows::core::Result<IBackgroundCopyError> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetError)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IBackgroundCopyError>(result__)
    }
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetOwner)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetDisplayName<'a, P0>(&self, val: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetDisplayName)(::windows::core::Interface::as_raw(self), val.into()).ok()
    }
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetDisplayName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, P0>(&self, val: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetDescription)(::windows::core::Interface::as_raw(self), val.into()).ok()
    }
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetDescription)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetPriority(&self, val: BG_JOB_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetPriority)(::windows::core::Interface::as_raw(self), val).ok()
    }
    pub unsafe fn GetPriority(&self) -> ::windows::core::Result<BG_JOB_PRIORITY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetPriority)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_JOB_PRIORITY>(result__)
    }
    pub unsafe fn SetNotifyFlags(&self, val: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetNotifyFlags)(::windows::core::Interface::as_raw(self), val).ok()
    }
    pub unsafe fn GetNotifyFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetNotifyFlags)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetNotifyInterface<'a, P0>(&self, val: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetNotifyInterface)(::windows::core::Interface::as_raw(self), val.into().abi()).ok()
    }
    pub unsafe fn GetNotifyInterface(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetNotifyInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn SetMinimumRetryDelay(&self, seconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetMinimumRetryDelay)(::windows::core::Interface::as_raw(self), seconds).ok()
    }
    pub unsafe fn GetMinimumRetryDelay(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetMinimumRetryDelay)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetNoProgressTimeout(&self, seconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetNoProgressTimeout)(::windows::core::Interface::as_raw(self), seconds).ok()
    }
    pub unsafe fn GetNoProgressTimeout(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetNoProgressTimeout)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetErrorCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetErrorCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetProxySettings<'a, P0, P1>(&self, proxyusage: BG_JOB_PROXY_USAGE, proxylist: P0, proxybypasslist: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetProxySettings)(::windows::core::Interface::as_raw(self), proxyusage, proxylist.into(), proxybypasslist.into()).ok()
    }
    pub unsafe fn GetProxySettings(&self, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut ::windows::core::PWSTR, pproxybypasslist: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetProxySettings)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pproxyusage), ::core::mem::transmute(pproxylist), ::core::mem::transmute(pproxybypasslist)).ok()
    }
    pub unsafe fn TakeOwnership(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.TakeOwnership)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetNotifyCmdLine<'a, P0, P1>(&self, program: P0, parameters: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetNotifyCmdLine)(::windows::core::Interface::as_raw(self), program.into(), parameters.into()).ok()
    }
    pub unsafe fn GetNotifyCmdLine(&self, pprogram: *mut ::windows::core::PWSTR, pparameters: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetNotifyCmdLine)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pprogram), ::core::mem::transmute(pparameters)).ok()
    }
    pub unsafe fn GetReplyProgress(&self, pprogress: *mut BG_JOB_REPLY_PROGRESS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetReplyProgress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pprogress)).ok()
    }
    pub unsafe fn GetReplyData(&self, ppbuffer: *mut *mut u8, plength: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetReplyData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(plength)).ok()
    }
    pub unsafe fn SetReplyFileName<'a, P0>(&self, replyfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetReplyFileName)(::windows::core::Interface::as_raw(self), replyfilename.into()).ok()
    }
    pub unsafe fn GetReplyFileName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetReplyFileName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetCredentials(&self, credentials: *const BG_AUTH_CREDENTIALS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetCredentials)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(credentials)).ok()
    }
    pub unsafe fn RemoveCredentials(&self, target: BG_AUTH_TARGET, scheme: BG_AUTH_SCHEME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RemoveCredentials)(::windows::core::Interface::as_raw(self), target, scheme).ok()
    }
    pub unsafe fn ReplaceRemotePrefix<'a, P0, P1>(&self, oldprefix: P0, newprefix: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).ReplaceRemotePrefix)(::windows::core::Interface::as_raw(self), oldprefix.into(), newprefix.into()).ok()
    }
    pub unsafe fn AddFileWithRanges<'a, P0, P1>(&self, remoteurl: P0, localname: P1, ranges: &[BG_FILE_RANGE]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddFileWithRanges)(::windows::core::Interface::as_raw(self), remoteurl.into(), localname.into(), ranges.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ranges))).ok()
    }
    pub unsafe fn SetFileACLFlags(&self, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFileACLFlags)(::windows::core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn GetFileACLFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetFileACLFlags)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IBackgroundCopyJob3> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyJob3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyJob3> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBackgroundCopyJob3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJob3> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyJob3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundCopyJob3> for IBackgroundCopyJob {
    fn from(value: IBackgroundCopyJob3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyJob3> for &'a IBackgroundCopyJob {
    fn from(value: &'a IBackgroundCopyJob3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJob3> for IBackgroundCopyJob {
    fn from(value: &IBackgroundCopyJob3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundCopyJob3> for IBackgroundCopyJob2 {
    fn from(value: IBackgroundCopyJob3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyJob3> for &'a IBackgroundCopyJob2 {
    fn from(value: &'a IBackgroundCopyJob3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJob3> for IBackgroundCopyJob2 {
    fn from(value: &IBackgroundCopyJob3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IBackgroundCopyJob3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundCopyJob3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundCopyJob3 {}
impl ::core::fmt::Debug for IBackgroundCopyJob3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundCopyJob3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyJob3 {
    type Vtable = IBackgroundCopyJob3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x443c8934_90ff_48ed_bcde_26f5c7450042);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJob3_Vtbl {
    pub base__: IBackgroundCopyJob2_Vtbl,
    pub ReplaceRemotePrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oldprefix: ::windows::core::PCWSTR, newprefix: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub AddFileWithRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remoteurl: ::windows::core::PCWSTR, localname: ::windows::core::PCWSTR, rangecount: u32, ranges: *const BG_FILE_RANGE) -> ::windows::core::HRESULT,
    pub SetFileACLFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT,
    pub GetFileACLFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IBackgroundCopyJob4(::windows::core::IUnknown);
impl IBackgroundCopyJob4 {
    pub unsafe fn AddFileSet(&self, pfileset: &[BG_FILE_INFO]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.AddFileSet)(::windows::core::Interface::as_raw(self), pfileset.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pfileset))).ok()
    }
    pub unsafe fn AddFile<'a, P0, P1>(&self, remoteurl: P0, localname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.AddFile)(::windows::core::Interface::as_raw(self), remoteurl.into(), localname.into()).ok()
    }
    pub unsafe fn EnumFiles(&self) -> ::windows::core::Result<IEnumBackgroundCopyFiles> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.EnumFiles)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumBackgroundCopyFiles>(result__)
    }
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.Suspend)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.Resume)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.Cancel)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Complete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.Complete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<BG_JOB_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_JOB_TYPE>(result__)
    }
    pub unsafe fn GetProgress(&self) -> ::windows::core::Result<BG_JOB_PROGRESS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetProgress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_JOB_PROGRESS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTimes(&self) -> ::windows::core::Result<BG_JOB_TIMES> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetTimes)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_JOB_TIMES>(result__)
    }
    pub unsafe fn GetState(&self) -> ::windows::core::Result<BG_JOB_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_JOB_STATE>(result__)
    }
    pub unsafe fn GetError(&self) -> ::windows::core::Result<IBackgroundCopyError> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetError)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IBackgroundCopyError>(result__)
    }
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetOwner)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetDisplayName<'a, P0>(&self, val: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetDisplayName)(::windows::core::Interface::as_raw(self), val.into()).ok()
    }
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetDisplayName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, P0>(&self, val: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetDescription)(::windows::core::Interface::as_raw(self), val.into()).ok()
    }
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetDescription)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetPriority(&self, val: BG_JOB_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetPriority)(::windows::core::Interface::as_raw(self), val).ok()
    }
    pub unsafe fn GetPriority(&self) -> ::windows::core::Result<BG_JOB_PRIORITY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetPriority)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_JOB_PRIORITY>(result__)
    }
    pub unsafe fn SetNotifyFlags(&self, val: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetNotifyFlags)(::windows::core::Interface::as_raw(self), val).ok()
    }
    pub unsafe fn GetNotifyFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetNotifyFlags)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetNotifyInterface<'a, P0>(&self, val: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetNotifyInterface)(::windows::core::Interface::as_raw(self), val.into().abi()).ok()
    }
    pub unsafe fn GetNotifyInterface(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetNotifyInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn SetMinimumRetryDelay(&self, seconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetMinimumRetryDelay)(::windows::core::Interface::as_raw(self), seconds).ok()
    }
    pub unsafe fn GetMinimumRetryDelay(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetMinimumRetryDelay)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetNoProgressTimeout(&self, seconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetNoProgressTimeout)(::windows::core::Interface::as_raw(self), seconds).ok()
    }
    pub unsafe fn GetNoProgressTimeout(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetNoProgressTimeout)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetErrorCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetErrorCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetProxySettings<'a, P0, P1>(&self, proxyusage: BG_JOB_PROXY_USAGE, proxylist: P0, proxybypasslist: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetProxySettings)(::windows::core::Interface::as_raw(self), proxyusage, proxylist.into(), proxybypasslist.into()).ok()
    }
    pub unsafe fn GetProxySettings(&self, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut ::windows::core::PWSTR, pproxybypasslist: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetProxySettings)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pproxyusage), ::core::mem::transmute(pproxylist), ::core::mem::transmute(pproxybypasslist)).ok()
    }
    pub unsafe fn TakeOwnership(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.TakeOwnership)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetNotifyCmdLine<'a, P0, P1>(&self, program: P0, parameters: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetNotifyCmdLine)(::windows::core::Interface::as_raw(self), program.into(), parameters.into()).ok()
    }
    pub unsafe fn GetNotifyCmdLine(&self, pprogram: *mut ::windows::core::PWSTR, pparameters: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetNotifyCmdLine)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pprogram), ::core::mem::transmute(pparameters)).ok()
    }
    pub unsafe fn GetReplyProgress(&self, pprogress: *mut BG_JOB_REPLY_PROGRESS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetReplyProgress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pprogress)).ok()
    }
    pub unsafe fn GetReplyData(&self, ppbuffer: *mut *mut u8, plength: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetReplyData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(plength)).ok()
    }
    pub unsafe fn SetReplyFileName<'a, P0>(&self, replyfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetReplyFileName)(::windows::core::Interface::as_raw(self), replyfilename.into()).ok()
    }
    pub unsafe fn GetReplyFileName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetReplyFileName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetCredentials(&self, credentials: *const BG_AUTH_CREDENTIALS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetCredentials)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(credentials)).ok()
    }
    pub unsafe fn RemoveCredentials(&self, target: BG_AUTH_TARGET, scheme: BG_AUTH_SCHEME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.RemoveCredentials)(::windows::core::Interface::as_raw(self), target, scheme).ok()
    }
    pub unsafe fn ReplaceRemotePrefix<'a, P0, P1>(&self, oldprefix: P0, newprefix: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.ReplaceRemotePrefix)(::windows::core::Interface::as_raw(self), oldprefix.into(), newprefix.into()).ok()
    }
    pub unsafe fn AddFileWithRanges<'a, P0, P1>(&self, remoteurl: P0, localname: P1, ranges: &[BG_FILE_RANGE]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddFileWithRanges)(::windows::core::Interface::as_raw(self), remoteurl.into(), localname.into(), ranges.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ranges))).ok()
    }
    pub unsafe fn SetFileACLFlags(&self, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetFileACLFlags)(::windows::core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn GetFileACLFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetFileACLFlags)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetPeerCachingFlags(&self, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPeerCachingFlags)(::windows::core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn GetPeerCachingFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetPeerCachingFlags)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOwnerIntegrityLevel(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetOwnerIntegrityLevel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOwnerElevationState(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetOwnerElevationState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn SetMaximumDownloadTime(&self, timeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaximumDownloadTime)(::windows::core::Interface::as_raw(self), timeout).ok()
    }
    pub unsafe fn GetMaximumDownloadTime(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMaximumDownloadTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IBackgroundCopyJob4> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyJob4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyJob4> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBackgroundCopyJob4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJob4> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyJob4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundCopyJob4> for IBackgroundCopyJob {
    fn from(value: IBackgroundCopyJob4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyJob4> for &'a IBackgroundCopyJob {
    fn from(value: &'a IBackgroundCopyJob4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJob4> for IBackgroundCopyJob {
    fn from(value: &IBackgroundCopyJob4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundCopyJob4> for IBackgroundCopyJob2 {
    fn from(value: IBackgroundCopyJob4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyJob4> for &'a IBackgroundCopyJob2 {
    fn from(value: &'a IBackgroundCopyJob4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJob4> for IBackgroundCopyJob2 {
    fn from(value: &IBackgroundCopyJob4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundCopyJob4> for IBackgroundCopyJob3 {
    fn from(value: IBackgroundCopyJob4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyJob4> for &'a IBackgroundCopyJob3 {
    fn from(value: &'a IBackgroundCopyJob4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJob4> for IBackgroundCopyJob3 {
    fn from(value: &IBackgroundCopyJob4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IBackgroundCopyJob4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundCopyJob4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundCopyJob4 {}
impl ::core::fmt::Debug for IBackgroundCopyJob4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundCopyJob4").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyJob4 {
    type Vtable = IBackgroundCopyJob4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x659cdeae_489e_11d9_a9cd_000d56965251);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJob4_Vtbl {
    pub base__: IBackgroundCopyJob3_Vtbl,
    pub SetPeerCachingFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT,
    pub GetPeerCachingFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT,
    pub GetOwnerIntegrityLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plevel: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetOwnerElevationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pelevated: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetOwnerElevationState: usize,
    pub SetMaximumDownloadTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeout: u32) -> ::windows::core::HRESULT,
    pub GetMaximumDownloadTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptimeout: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IBackgroundCopyJob5(::windows::core::IUnknown);
impl IBackgroundCopyJob5 {
    pub unsafe fn AddFileSet(&self, pfileset: &[BG_FILE_INFO]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.AddFileSet)(::windows::core::Interface::as_raw(self), pfileset.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pfileset))).ok()
    }
    pub unsafe fn AddFile<'a, P0, P1>(&self, remoteurl: P0, localname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.AddFile)(::windows::core::Interface::as_raw(self), remoteurl.into(), localname.into()).ok()
    }
    pub unsafe fn EnumFiles(&self) -> ::windows::core::Result<IEnumBackgroundCopyFiles> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.EnumFiles)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumBackgroundCopyFiles>(result__)
    }
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.Suspend)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.Resume)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.Cancel)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Complete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.Complete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<BG_JOB_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_JOB_TYPE>(result__)
    }
    pub unsafe fn GetProgress(&self) -> ::windows::core::Result<BG_JOB_PROGRESS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetProgress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_JOB_PROGRESS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTimes(&self) -> ::windows::core::Result<BG_JOB_TIMES> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetTimes)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_JOB_TIMES>(result__)
    }
    pub unsafe fn GetState(&self) -> ::windows::core::Result<BG_JOB_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_JOB_STATE>(result__)
    }
    pub unsafe fn GetError(&self) -> ::windows::core::Result<IBackgroundCopyError> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetError)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IBackgroundCopyError>(result__)
    }
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetOwner)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetDisplayName<'a, P0>(&self, val: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetDisplayName)(::windows::core::Interface::as_raw(self), val.into()).ok()
    }
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetDisplayName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, P0>(&self, val: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetDescription)(::windows::core::Interface::as_raw(self), val.into()).ok()
    }
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetDescription)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetPriority(&self, val: BG_JOB_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetPriority)(::windows::core::Interface::as_raw(self), val).ok()
    }
    pub unsafe fn GetPriority(&self) -> ::windows::core::Result<BG_JOB_PRIORITY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetPriority)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_JOB_PRIORITY>(result__)
    }
    pub unsafe fn SetNotifyFlags(&self, val: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetNotifyFlags)(::windows::core::Interface::as_raw(self), val).ok()
    }
    pub unsafe fn GetNotifyFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetNotifyFlags)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetNotifyInterface<'a, P0>(&self, val: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetNotifyInterface)(::windows::core::Interface::as_raw(self), val.into().abi()).ok()
    }
    pub unsafe fn GetNotifyInterface(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetNotifyInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn SetMinimumRetryDelay(&self, seconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetMinimumRetryDelay)(::windows::core::Interface::as_raw(self), seconds).ok()
    }
    pub unsafe fn GetMinimumRetryDelay(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetMinimumRetryDelay)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetNoProgressTimeout(&self, seconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetNoProgressTimeout)(::windows::core::Interface::as_raw(self), seconds).ok()
    }
    pub unsafe fn GetNoProgressTimeout(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetNoProgressTimeout)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetErrorCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetErrorCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetProxySettings<'a, P0, P1>(&self, proxyusage: BG_JOB_PROXY_USAGE, proxylist: P0, proxybypasslist: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetProxySettings)(::windows::core::Interface::as_raw(self), proxyusage, proxylist.into(), proxybypasslist.into()).ok()
    }
    pub unsafe fn GetProxySettings(&self, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut ::windows::core::PWSTR, pproxybypasslist: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetProxySettings)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pproxyusage), ::core::mem::transmute(pproxylist), ::core::mem::transmute(pproxybypasslist)).ok()
    }
    pub unsafe fn TakeOwnership(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.TakeOwnership)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetNotifyCmdLine<'a, P0, P1>(&self, program: P0, parameters: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetNotifyCmdLine)(::windows::core::Interface::as_raw(self), program.into(), parameters.into()).ok()
    }
    pub unsafe fn GetNotifyCmdLine(&self, pprogram: *mut ::windows::core::PWSTR, pparameters: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetNotifyCmdLine)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pprogram), ::core::mem::transmute(pparameters)).ok()
    }
    pub unsafe fn GetReplyProgress(&self, pprogress: *mut BG_JOB_REPLY_PROGRESS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetReplyProgress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pprogress)).ok()
    }
    pub unsafe fn GetReplyData(&self, ppbuffer: *mut *mut u8, plength: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetReplyData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(plength)).ok()
    }
    pub unsafe fn SetReplyFileName<'a, P0>(&self, replyfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetReplyFileName)(::windows::core::Interface::as_raw(self), replyfilename.into()).ok()
    }
    pub unsafe fn GetReplyFileName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetReplyFileName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetCredentials(&self, credentials: *const BG_AUTH_CREDENTIALS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetCredentials)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(credentials)).ok()
    }
    pub unsafe fn RemoveCredentials(&self, target: BG_AUTH_TARGET, scheme: BG_AUTH_SCHEME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.RemoveCredentials)(::windows::core::Interface::as_raw(self), target, scheme).ok()
    }
    pub unsafe fn ReplaceRemotePrefix<'a, P0, P1>(&self, oldprefix: P0, newprefix: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.ReplaceRemotePrefix)(::windows::core::Interface::as_raw(self), oldprefix.into(), newprefix.into()).ok()
    }
    pub unsafe fn AddFileWithRanges<'a, P0, P1>(&self, remoteurl: P0, localname: P1, ranges: &[BG_FILE_RANGE]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.AddFileWithRanges)(::windows::core::Interface::as_raw(self), remoteurl.into(), localname.into(), ranges.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ranges))).ok()
    }
    pub unsafe fn SetFileACLFlags(&self, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetFileACLFlags)(::windows::core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn GetFileACLFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetFileACLFlags)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetPeerCachingFlags(&self, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPeerCachingFlags)(::windows::core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn GetPeerCachingFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetPeerCachingFlags)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOwnerIntegrityLevel(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetOwnerIntegrityLevel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOwnerElevationState(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetOwnerElevationState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn SetMaximumDownloadTime(&self, timeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetMaximumDownloadTime)(::windows::core::Interface::as_raw(self), timeout).ok()
    }
    pub unsafe fn GetMaximumDownloadTime(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetMaximumDownloadTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProperty(&self, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: BITS_JOB_PROPERTY_VALUE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProperty)(::windows::core::Interface::as_raw(self), propertyid, ::core::mem::transmute(propertyvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperty(&self, propertyid: BITS_JOB_PROPERTY_ID) -> ::windows::core::Result<BITS_JOB_PROPERTY_VALUE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetProperty)(::windows::core::Interface::as_raw(self), propertyid, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BITS_JOB_PROPERTY_VALUE>(result__)
    }
}
impl ::core::convert::From<IBackgroundCopyJob5> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyJob5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyJob5> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBackgroundCopyJob5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJob5> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyJob5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundCopyJob5> for IBackgroundCopyJob {
    fn from(value: IBackgroundCopyJob5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyJob5> for &'a IBackgroundCopyJob {
    fn from(value: &'a IBackgroundCopyJob5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJob5> for IBackgroundCopyJob {
    fn from(value: &IBackgroundCopyJob5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundCopyJob5> for IBackgroundCopyJob2 {
    fn from(value: IBackgroundCopyJob5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyJob5> for &'a IBackgroundCopyJob2 {
    fn from(value: &'a IBackgroundCopyJob5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJob5> for IBackgroundCopyJob2 {
    fn from(value: &IBackgroundCopyJob5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundCopyJob5> for IBackgroundCopyJob3 {
    fn from(value: IBackgroundCopyJob5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyJob5> for &'a IBackgroundCopyJob3 {
    fn from(value: &'a IBackgroundCopyJob5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJob5> for IBackgroundCopyJob3 {
    fn from(value: &IBackgroundCopyJob5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundCopyJob5> for IBackgroundCopyJob4 {
    fn from(value: IBackgroundCopyJob5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyJob5> for &'a IBackgroundCopyJob4 {
    fn from(value: &'a IBackgroundCopyJob5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJob5> for IBackgroundCopyJob4 {
    fn from(value: &IBackgroundCopyJob5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IBackgroundCopyJob5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundCopyJob5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundCopyJob5 {}
impl ::core::fmt::Debug for IBackgroundCopyJob5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundCopyJob5").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyJob5 {
    type Vtable = IBackgroundCopyJob5_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe847030c_bbba_4657_af6d_484aa42bf1fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJob5_Vtbl {
    pub base__: IBackgroundCopyJob4_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: BITS_JOB_PROPERTY_VALUE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: *mut BITS_JOB_PROPERTY_VALUE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetProperty: usize,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IBackgroundCopyJobHttpOptions(::windows::core::IUnknown);
impl IBackgroundCopyJobHttpOptions {
    pub unsafe fn SetClientCertificateByID<'a, P0>(&self, storelocation: BG_CERT_STORE_LOCATION, storename: P0, pcerthashblob: &[u8; 20]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetClientCertificateByID)(::windows::core::Interface::as_raw(self), storelocation, storename.into(), ::core::mem::transmute(::windows::core::as_ptr_or_null(pcerthashblob))).ok()
    }
    pub unsafe fn SetClientCertificateByName<'a, P0, P1>(&self, storelocation: BG_CERT_STORE_LOCATION, storename: P0, subjectname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetClientCertificateByName)(::windows::core::Interface::as_raw(self), storelocation, storename.into(), subjectname.into()).ok()
    }
    pub unsafe fn RemoveClientCertificate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveClientCertificate)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetClientCertificate(&self, pstorelocation: *mut BG_CERT_STORE_LOCATION, pstorename: *mut ::windows::core::PWSTR, ppcerthashblob: *mut *mut u8, psubjectname: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetClientCertificate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstorelocation), ::core::mem::transmute(pstorename), ::core::mem::transmute(ppcerthashblob), ::core::mem::transmute(psubjectname)).ok()
    }
    pub unsafe fn SetCustomHeaders<'a, P0>(&self, requestheaders: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetCustomHeaders)(::windows::core::Interface::as_raw(self), requestheaders.into()).ok()
    }
    pub unsafe fn GetCustomHeaders(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetCustomHeaders)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetSecurityFlags(&self, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSecurityFlags)(::windows::core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn GetSecurityFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSecurityFlags)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IBackgroundCopyJobHttpOptions> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyJobHttpOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyJobHttpOptions> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBackgroundCopyJobHttpOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJobHttpOptions> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyJobHttpOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IBackgroundCopyJobHttpOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundCopyJobHttpOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundCopyJobHttpOptions {}
impl ::core::fmt::Debug for IBackgroundCopyJobHttpOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundCopyJobHttpOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyJobHttpOptions {
    type Vtable = IBackgroundCopyJobHttpOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1bd1079_9f01_4bdc_8036_f09b70095066);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJobHttpOptions_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetClientCertificateByID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storelocation: BG_CERT_STORE_LOCATION, storename: ::windows::core::PCWSTR, pcerthashblob: *const u8) -> ::windows::core::HRESULT,
    pub SetClientCertificateByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storelocation: BG_CERT_STORE_LOCATION, storename: ::windows::core::PCWSTR, subjectname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub RemoveClientCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetClientCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstorelocation: *mut BG_CERT_STORE_LOCATION, pstorename: *mut ::windows::core::PWSTR, ppcerthashblob: *mut *mut u8, psubjectname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetCustomHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestheaders: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetCustomHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prequestheaders: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetSecurityFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT,
    pub GetSecurityFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IBackgroundCopyJobHttpOptions2(::windows::core::IUnknown);
impl IBackgroundCopyJobHttpOptions2 {
    pub unsafe fn SetClientCertificateByID<'a, P0>(&self, storelocation: BG_CERT_STORE_LOCATION, storename: P0, pcerthashblob: &[u8; 20]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetClientCertificateByID)(::windows::core::Interface::as_raw(self), storelocation, storename.into(), ::core::mem::transmute(::windows::core::as_ptr_or_null(pcerthashblob))).ok()
    }
    pub unsafe fn SetClientCertificateByName<'a, P0, P1>(&self, storelocation: BG_CERT_STORE_LOCATION, storename: P0, subjectname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetClientCertificateByName)(::windows::core::Interface::as_raw(self), storelocation, storename.into(), subjectname.into()).ok()
    }
    pub unsafe fn RemoveClientCertificate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RemoveClientCertificate)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetClientCertificate(&self, pstorelocation: *mut BG_CERT_STORE_LOCATION, pstorename: *mut ::windows::core::PWSTR, ppcerthashblob: *mut *mut u8, psubjectname: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetClientCertificate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstorelocation), ::core::mem::transmute(pstorename), ::core::mem::transmute(ppcerthashblob), ::core::mem::transmute(psubjectname)).ok()
    }
    pub unsafe fn SetCustomHeaders<'a, P0>(&self, requestheaders: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetCustomHeaders)(::windows::core::Interface::as_raw(self), requestheaders.into()).ok()
    }
    pub unsafe fn GetCustomHeaders(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetCustomHeaders)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetSecurityFlags(&self, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetSecurityFlags)(::windows::core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn GetSecurityFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetSecurityFlags)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetHttpMethod<'a, P0>(&self, method: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetHttpMethod)(::windows::core::Interface::as_raw(self), method.into()).ok()
    }
    pub unsafe fn GetHttpMethod(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetHttpMethod)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IBackgroundCopyJobHttpOptions2> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyJobHttpOptions2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyJobHttpOptions2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBackgroundCopyJobHttpOptions2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJobHttpOptions2> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyJobHttpOptions2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundCopyJobHttpOptions2> for IBackgroundCopyJobHttpOptions {
    fn from(value: IBackgroundCopyJobHttpOptions2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyJobHttpOptions2> for &'a IBackgroundCopyJobHttpOptions {
    fn from(value: &'a IBackgroundCopyJobHttpOptions2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJobHttpOptions2> for IBackgroundCopyJobHttpOptions {
    fn from(value: &IBackgroundCopyJobHttpOptions2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IBackgroundCopyJobHttpOptions2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundCopyJobHttpOptions2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundCopyJobHttpOptions2 {}
impl ::core::fmt::Debug for IBackgroundCopyJobHttpOptions2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundCopyJobHttpOptions2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyJobHttpOptions2 {
    type Vtable = IBackgroundCopyJobHttpOptions2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb591a192_a405_4fc3_8323_4c5c542578fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJobHttpOptions2_Vtbl {
    pub base__: IBackgroundCopyJobHttpOptions_Vtbl,
    pub SetHttpMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, method: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetHttpMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, method: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IBackgroundCopyJobHttpOptions3(::windows::core::IUnknown);
impl IBackgroundCopyJobHttpOptions3 {
    pub unsafe fn SetClientCertificateByID<'a, P0>(&self, storelocation: BG_CERT_STORE_LOCATION, storename: P0, pcerthashblob: &[u8; 20]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetClientCertificateByID)(::windows::core::Interface::as_raw(self), storelocation, storename.into(), ::core::mem::transmute(::windows::core::as_ptr_or_null(pcerthashblob))).ok()
    }
    pub unsafe fn SetClientCertificateByName<'a, P0, P1>(&self, storelocation: BG_CERT_STORE_LOCATION, storename: P0, subjectname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetClientCertificateByName)(::windows::core::Interface::as_raw(self), storelocation, storename.into(), subjectname.into()).ok()
    }
    pub unsafe fn RemoveClientCertificate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.RemoveClientCertificate)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetClientCertificate(&self, pstorelocation: *mut BG_CERT_STORE_LOCATION, pstorename: *mut ::windows::core::PWSTR, ppcerthashblob: *mut *mut u8, psubjectname: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetClientCertificate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstorelocation), ::core::mem::transmute(pstorename), ::core::mem::transmute(ppcerthashblob), ::core::mem::transmute(psubjectname)).ok()
    }
    pub unsafe fn SetCustomHeaders<'a, P0>(&self, requestheaders: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetCustomHeaders)(::windows::core::Interface::as_raw(self), requestheaders.into()).ok()
    }
    pub unsafe fn GetCustomHeaders(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetCustomHeaders)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetSecurityFlags(&self, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetSecurityFlags)(::windows::core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn GetSecurityFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetSecurityFlags)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetHttpMethod<'a, P0>(&self, method: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetHttpMethod)(::windows::core::Interface::as_raw(self), method.into()).ok()
    }
    pub unsafe fn GetHttpMethod(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetHttpMethod)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetServerCertificateValidationInterface<'a, P0>(&self, certvalidationcallback: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).SetServerCertificateValidationInterface)(::windows::core::Interface::as_raw(self), certvalidationcallback.into().abi()).ok()
    }
    pub unsafe fn MakeCustomHeadersWriteOnly(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MakeCustomHeadersWriteOnly)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IBackgroundCopyJobHttpOptions3> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyJobHttpOptions3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyJobHttpOptions3> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBackgroundCopyJobHttpOptions3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJobHttpOptions3> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyJobHttpOptions3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundCopyJobHttpOptions3> for IBackgroundCopyJobHttpOptions {
    fn from(value: IBackgroundCopyJobHttpOptions3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyJobHttpOptions3> for &'a IBackgroundCopyJobHttpOptions {
    fn from(value: &'a IBackgroundCopyJobHttpOptions3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJobHttpOptions3> for IBackgroundCopyJobHttpOptions {
    fn from(value: &IBackgroundCopyJobHttpOptions3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IBackgroundCopyJobHttpOptions3> for IBackgroundCopyJobHttpOptions2 {
    fn from(value: IBackgroundCopyJobHttpOptions3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyJobHttpOptions3> for &'a IBackgroundCopyJobHttpOptions2 {
    fn from(value: &'a IBackgroundCopyJobHttpOptions3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyJobHttpOptions3> for IBackgroundCopyJobHttpOptions2 {
    fn from(value: &IBackgroundCopyJobHttpOptions3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IBackgroundCopyJobHttpOptions3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundCopyJobHttpOptions3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundCopyJobHttpOptions3 {}
impl ::core::fmt::Debug for IBackgroundCopyJobHttpOptions3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundCopyJobHttpOptions3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyJobHttpOptions3 {
    type Vtable = IBackgroundCopyJobHttpOptions3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a9263d3_fd4c_4eda_9b28_30132a4d4e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJobHttpOptions3_Vtbl {
    pub base__: IBackgroundCopyJobHttpOptions2_Vtbl,
    pub SetServerCertificateValidationInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certvalidationcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MakeCustomHeadersWriteOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IBackgroundCopyManager(::windows::core::IUnknown);
impl IBackgroundCopyManager {
    pub unsafe fn CreateJob<'a, P0>(&self, displayname: P0, r#type: BG_JOB_TYPE, pjobid: *mut ::windows::core::GUID, ppjob: *mut ::core::option::Option<IBackgroundCopyJob>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).CreateJob)(::windows::core::Interface::as_raw(self), displayname.into(), r#type, ::core::mem::transmute(pjobid), ::core::mem::transmute(ppjob)).ok()
    }
    pub unsafe fn GetJob(&self, jobid: *const ::windows::core::GUID) -> ::windows::core::Result<IBackgroundCopyJob> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetJob)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(jobid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IBackgroundCopyJob>(result__)
    }
    pub unsafe fn EnumJobs(&self, dwflags: u32) -> ::windows::core::Result<IEnumBackgroundCopyJobs> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EnumJobs)(::windows::core::Interface::as_raw(self), dwflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumBackgroundCopyJobs>(result__)
    }
    pub unsafe fn GetErrorDescription(&self, hresult: ::windows::core::HRESULT, languageid: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetErrorDescription)(::windows::core::Interface::as_raw(self), hresult, languageid, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IBackgroundCopyManager> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyManager> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBackgroundCopyManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyManager> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IBackgroundCopyManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundCopyManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundCopyManager {}
impl ::core::fmt::Debug for IBackgroundCopyManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundCopyManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyManager {
    type Vtable = IBackgroundCopyManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ce34c0d_0dc9_4c1f_897c_daa1b78cee7c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyManager_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub CreateJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayname: ::windows::core::PCWSTR, r#type: BG_JOB_TYPE, pjobid: *mut ::windows::core::GUID, ppjob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobid: *const ::windows::core::GUID, ppjob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumJobs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetErrorDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, languageid: u32, perrordescription: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IBackgroundCopyQMgr(::windows::core::IUnknown);
impl IBackgroundCopyQMgr {
    pub unsafe fn CreateGroup(&self, guidgroupid: ::windows::core::GUID) -> ::windows::core::Result<IBackgroundCopyGroup> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guidgroupid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IBackgroundCopyGroup>(result__)
    }
    pub unsafe fn GetGroup(&self, groupid: ::windows::core::GUID) -> ::windows::core::Result<IBackgroundCopyGroup> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(groupid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IBackgroundCopyGroup>(result__)
    }
    pub unsafe fn EnumGroups(&self, dwflags: u32) -> ::windows::core::Result<IEnumBackgroundCopyGroups> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EnumGroups)(::windows::core::Interface::as_raw(self), dwflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumBackgroundCopyGroups>(result__)
    }
}
impl ::core::convert::From<IBackgroundCopyQMgr> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyQMgr) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyQMgr> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBackgroundCopyQMgr) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyQMgr> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyQMgr) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IBackgroundCopyQMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundCopyQMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundCopyQMgr {}
impl ::core::fmt::Debug for IBackgroundCopyQMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundCopyQMgr").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyQMgr {
    type Vtable = IBackgroundCopyQMgr_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16f41c69_09f5_41d2_8cd8_3c08c47bc8a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyQMgr_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub CreateGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidgroupid: ::windows::core::GUID, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, groupid: ::windows::core::GUID, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, ppenumgroups: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IBackgroundCopyServerCertificateValidationCallback(::windows::core::IUnknown);
impl IBackgroundCopyServerCertificateValidationCallback {
    pub unsafe fn ValidateServerCertificate<'a, P0, P1>(&self, job: P0, file: P1, certdata: &[u8], certencodingtype: u32, certstoredata: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IBackgroundCopyJob>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IBackgroundCopyFile>>,
    {
        (::windows::core::Interface::vtable(self).ValidateServerCertificate)(::windows::core::Interface::as_raw(self), job.into().abi(), file.into().abi(), certdata.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(certdata)), certencodingtype, certstoredata.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(certstoredata))).ok()
    }
}
impl ::core::convert::From<IBackgroundCopyServerCertificateValidationCallback> for ::windows::core::IUnknown {
    fn from(value: IBackgroundCopyServerCertificateValidationCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBackgroundCopyServerCertificateValidationCallback> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBackgroundCopyServerCertificateValidationCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundCopyServerCertificateValidationCallback> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundCopyServerCertificateValidationCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IBackgroundCopyServerCertificateValidationCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundCopyServerCertificateValidationCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundCopyServerCertificateValidationCallback {}
impl ::core::fmt::Debug for IBackgroundCopyServerCertificateValidationCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundCopyServerCertificateValidationCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBackgroundCopyServerCertificateValidationCallback {
    type Vtable = IBackgroundCopyServerCertificateValidationCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4cec0d02_def7_4158_813a_c32a46945ff7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyServerCertificateValidationCallback_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub ValidateServerCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, job: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, certlength: u32, certdata: *const u8, certencodingtype: u32, certstorelength: u32, certstoredata: *const u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IBitsPeer(::windows::core::IUnknown);
impl IBitsPeer {
    pub unsafe fn GetPeerName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetPeerName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsAuthenticated(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsAuthenticated)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsAvailable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsAvailable)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IBitsPeer> for ::windows::core::IUnknown {
    fn from(value: IBitsPeer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBitsPeer> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBitsPeer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBitsPeer> for ::windows::core::IUnknown {
    fn from(value: &IBitsPeer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IBitsPeer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBitsPeer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBitsPeer {}
impl ::core::fmt::Debug for IBitsPeer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBitsPeer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBitsPeer {
    type Vtable = IBitsPeer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x659cdea2_489e_11d9_a9cd_000d56965251);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitsPeer_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetPeerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsAuthenticated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pauth: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsAuthenticated: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ponline: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsAvailable: usize,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IBitsPeerCacheAdministration(::windows::core::IUnknown);
impl IBitsPeerCacheAdministration {
    pub unsafe fn GetMaximumCacheSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMaximumCacheSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMaximumCacheSize(&self, bytes: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaximumCacheSize)(::windows::core::Interface::as_raw(self), bytes).ok()
    }
    pub unsafe fn GetMaximumContentAge(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMaximumContentAge)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMaximumContentAge(&self, seconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaximumContentAge)(::windows::core::Interface::as_raw(self), seconds).ok()
    }
    pub unsafe fn GetConfigurationFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetConfigurationFlags)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetConfigurationFlags(&self, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetConfigurationFlags)(::windows::core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn EnumRecords(&self) -> ::windows::core::Result<IEnumBitsPeerCacheRecords> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EnumRecords)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumBitsPeerCacheRecords>(result__)
    }
    pub unsafe fn GetRecord(&self, id: *const ::windows::core::GUID) -> ::windows::core::Result<IBitsPeerCacheRecord> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetRecord)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(id), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IBitsPeerCacheRecord>(result__)
    }
    pub unsafe fn ClearRecords(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClearRecords)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DeleteRecord(&self, id: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteRecord)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(id)).ok()
    }
    pub unsafe fn DeleteUrl<'a, P0>(&self, url: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteUrl)(::windows::core::Interface::as_raw(self), url.into()).ok()
    }
    pub unsafe fn EnumPeers(&self) -> ::windows::core::Result<IEnumBitsPeers> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EnumPeers)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumBitsPeers>(result__)
    }
    pub unsafe fn ClearPeers(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClearPeers)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DiscoverPeers(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DiscoverPeers)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IBitsPeerCacheAdministration> for ::windows::core::IUnknown {
    fn from(value: IBitsPeerCacheAdministration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBitsPeerCacheAdministration> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBitsPeerCacheAdministration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBitsPeerCacheAdministration> for ::windows::core::IUnknown {
    fn from(value: &IBitsPeerCacheAdministration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IBitsPeerCacheAdministration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBitsPeerCacheAdministration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBitsPeerCacheAdministration {}
impl ::core::fmt::Debug for IBitsPeerCacheAdministration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBitsPeerCacheAdministration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBitsPeerCacheAdministration {
    type Vtable = IBitsPeerCacheAdministration_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x659cdead_489e_11d9_a9cd_000d56965251);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitsPeerCacheAdministration_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetMaximumCacheSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbytes: *mut u32) -> ::windows::core::HRESULT,
    pub SetMaximumCacheSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bytes: u32) -> ::windows::core::HRESULT,
    pub GetMaximumContentAge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pseconds: *mut u32) -> ::windows::core::HRESULT,
    pub SetMaximumContentAge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, seconds: u32) -> ::windows::core::HRESULT,
    pub GetConfigurationFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT,
    pub SetConfigurationFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT,
    pub EnumRecords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *const ::windows::core::GUID, pprecord: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ClearRecords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeleteRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub DeleteUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, url: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub EnumPeers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ClearPeers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DiscoverPeers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IBitsPeerCacheRecord(::windows::core::IUnknown);
impl IBitsPeerCacheRecord {
    pub unsafe fn GetId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetOriginUrl(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetOriginUrl)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetFileSize(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetFileSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFileModificationTime(&self) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetFileModificationTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastAccessTime(&self) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetLastAccessTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    pub unsafe fn IsFileValidated(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsFileValidated)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetFileRanges(&self, prangecount: *mut u32, ppranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFileRanges)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prangecount), ::core::mem::transmute(ppranges)).ok()
    }
}
impl ::core::convert::From<IBitsPeerCacheRecord> for ::windows::core::IUnknown {
    fn from(value: IBitsPeerCacheRecord) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBitsPeerCacheRecord> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBitsPeerCacheRecord) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBitsPeerCacheRecord> for ::windows::core::IUnknown {
    fn from(value: &IBitsPeerCacheRecord) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IBitsPeerCacheRecord {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBitsPeerCacheRecord {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBitsPeerCacheRecord {}
impl ::core::fmt::Debug for IBitsPeerCacheRecord {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBitsPeerCacheRecord").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBitsPeerCacheRecord {
    type Vtable = IBitsPeerCacheRecord_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x659cdeaf_489e_11d9_a9cd_000d56965251);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitsPeerCacheRecord_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetOriginUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetFileSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFileModificationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFileModificationTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLastAccessTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLastAccessTime: usize,
    pub IsFileValidated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFileRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prangecount: *mut u32, ppranges: *mut *mut BG_FILE_RANGE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IBitsTokenOptions(::windows::core::IUnknown);
impl IBitsTokenOptions {
    pub unsafe fn SetHelperTokenFlags(&self, usageflags: BG_TOKEN) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetHelperTokenFlags)(::windows::core::Interface::as_raw(self), usageflags).ok()
    }
    pub unsafe fn GetHelperTokenFlags(&self) -> ::windows::core::Result<BG_TOKEN> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetHelperTokenFlags)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BG_TOKEN>(result__)
    }
    pub unsafe fn SetHelperToken(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetHelperToken)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ClearHelperToken(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClearHelperToken)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetHelperTokenSid(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetHelperTokenSid)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IBitsTokenOptions> for ::windows::core::IUnknown {
    fn from(value: IBitsTokenOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBitsTokenOptions> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBitsTokenOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBitsTokenOptions> for ::windows::core::IUnknown {
    fn from(value: &IBitsTokenOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IBitsTokenOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBitsTokenOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBitsTokenOptions {}
impl ::core::fmt::Debug for IBitsTokenOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBitsTokenOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBitsTokenOptions {
    type Vtable = IBitsTokenOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a2584c3_f7d2_457a_9a5e_22b67bffc7d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitsTokenOptions_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetHelperTokenFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usageflags: BG_TOKEN) -> ::windows::core::HRESULT,
    pub GetHelperTokenFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflags: *mut BG_TOKEN) -> ::windows::core::HRESULT,
    pub SetHelperToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ClearHelperToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetHelperTokenSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IEnumBackgroundCopyFiles(::windows::core::IUnknown);
impl IEnumBackgroundCopyFiles {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IBackgroundCopyFile>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), celt, ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumBackgroundCopyFiles> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumBackgroundCopyFiles>(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IEnumBackgroundCopyFiles> for ::windows::core::IUnknown {
    fn from(value: IEnumBackgroundCopyFiles) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEnumBackgroundCopyFiles> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEnumBackgroundCopyFiles) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumBackgroundCopyFiles> for ::windows::core::IUnknown {
    fn from(value: &IEnumBackgroundCopyFiles) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEnumBackgroundCopyFiles {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumBackgroundCopyFiles {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumBackgroundCopyFiles {}
impl ::core::fmt::Debug for IEnumBackgroundCopyFiles {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumBackgroundCopyFiles").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumBackgroundCopyFiles {
    type Vtable = IEnumBackgroundCopyFiles_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca51e165_c365_424c_8d41_24aaa4ff3c40);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumBackgroundCopyFiles_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IEnumBackgroundCopyGroups(::windows::core::IUnknown);
impl IEnumBackgroundCopyGroups {
    pub unsafe fn Next(&self, rgelt: &mut [::windows::core::GUID], pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(rgelt)), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumBackgroundCopyGroups> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumBackgroundCopyGroups>(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IEnumBackgroundCopyGroups> for ::windows::core::IUnknown {
    fn from(value: IEnumBackgroundCopyGroups) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEnumBackgroundCopyGroups> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEnumBackgroundCopyGroups) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumBackgroundCopyGroups> for ::windows::core::IUnknown {
    fn from(value: &IEnumBackgroundCopyGroups) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEnumBackgroundCopyGroups {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumBackgroundCopyGroups {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumBackgroundCopyGroups {}
impl ::core::fmt::Debug for IEnumBackgroundCopyGroups {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumBackgroundCopyGroups").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumBackgroundCopyGroups {
    type Vtable = IEnumBackgroundCopyGroups_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd993e603_4aa4_47c5_8665_c20d39c2ba4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumBackgroundCopyGroups_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IEnumBackgroundCopyJobs(::windows::core::IUnknown);
impl IEnumBackgroundCopyJobs {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IBackgroundCopyJob>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), celt, ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumBackgroundCopyJobs> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumBackgroundCopyJobs>(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IEnumBackgroundCopyJobs> for ::windows::core::IUnknown {
    fn from(value: IEnumBackgroundCopyJobs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEnumBackgroundCopyJobs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEnumBackgroundCopyJobs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumBackgroundCopyJobs> for ::windows::core::IUnknown {
    fn from(value: &IEnumBackgroundCopyJobs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEnumBackgroundCopyJobs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumBackgroundCopyJobs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumBackgroundCopyJobs {}
impl ::core::fmt::Debug for IEnumBackgroundCopyJobs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumBackgroundCopyJobs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumBackgroundCopyJobs {
    type Vtable = IEnumBackgroundCopyJobs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1af4f612_3b71_466f_8f58_7b6f73ac57ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumBackgroundCopyJobs_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IEnumBackgroundCopyJobs1(::windows::core::IUnknown);
impl IEnumBackgroundCopyJobs1 {
    pub unsafe fn Next(&self, rgelt: &mut [::windows::core::GUID], pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(rgelt)), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumBackgroundCopyJobs1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumBackgroundCopyJobs1>(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IEnumBackgroundCopyJobs1> for ::windows::core::IUnknown {
    fn from(value: IEnumBackgroundCopyJobs1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEnumBackgroundCopyJobs1> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEnumBackgroundCopyJobs1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumBackgroundCopyJobs1> for ::windows::core::IUnknown {
    fn from(value: &IEnumBackgroundCopyJobs1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEnumBackgroundCopyJobs1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumBackgroundCopyJobs1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumBackgroundCopyJobs1 {}
impl ::core::fmt::Debug for IEnumBackgroundCopyJobs1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumBackgroundCopyJobs1").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumBackgroundCopyJobs1 {
    type Vtable = IEnumBackgroundCopyJobs1_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8baeba9d_8f1c_42c4_b82c_09ae79980d25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumBackgroundCopyJobs1_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IEnumBitsPeerCacheRecords(::windows::core::IUnknown);
impl IEnumBitsPeerCacheRecords {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IBitsPeerCacheRecord>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), celt, ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumBitsPeerCacheRecords> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumBitsPeerCacheRecords>(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IEnumBitsPeerCacheRecords> for ::windows::core::IUnknown {
    fn from(value: IEnumBitsPeerCacheRecords) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEnumBitsPeerCacheRecords> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEnumBitsPeerCacheRecords) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumBitsPeerCacheRecords> for ::windows::core::IUnknown {
    fn from(value: &IEnumBitsPeerCacheRecords) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEnumBitsPeerCacheRecords {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumBitsPeerCacheRecords {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumBitsPeerCacheRecords {}
impl ::core::fmt::Debug for IEnumBitsPeerCacheRecords {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumBitsPeerCacheRecords").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumBitsPeerCacheRecords {
    type Vtable = IEnumBitsPeerCacheRecords_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x659cdea4_489e_11d9_a9cd_000d56965251);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumBitsPeerCacheRecords_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
#[repr(transparent)]
pub struct IEnumBitsPeers(::windows::core::IUnknown);
impl IEnumBitsPeers {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IBitsPeer>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), celt, ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumBitsPeers> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumBitsPeers>(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IEnumBitsPeers> for ::windows::core::IUnknown {
    fn from(value: IEnumBitsPeers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEnumBitsPeers> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEnumBitsPeers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumBitsPeers> for ::windows::core::IUnknown {
    fn from(value: &IEnumBitsPeers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEnumBitsPeers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumBitsPeers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumBitsPeers {}
impl ::core::fmt::Debug for IEnumBitsPeers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumBitsPeers").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumBitsPeers {
    type Vtable = IEnumBitsPeers_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x659cdea5_489e_11d9_a9cd_000d56965251);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumBitsPeers_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_E_DOWNLOADER_UNAVAILABLE: u32 = 2164264963u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_E_INVALID_STATE: u32 = 2164264961u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_E_ITEM_NOT_FOUND: u32 = 2164264964u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_E_SERVICE_UNAVAILABLE: u32 = 2164264962u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_NOTIFY_DISABLE_NOTIFY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_NOTIFY_FILE_DONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_NOTIFY_GROUP_DONE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_NOTIFY_JOB_DONE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_NOTIFY_USE_PROGRESSEX: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_PROGRESS_PERCENT_DONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_PROGRESS_SIZE_DONE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_PROGRESS_TIME_DONE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_PROTOCOL_CUSTOM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_PROTOCOL_FTP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_PROTOCOL_HTTP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_PROTOCOL_SMB: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_STATUS_FILE_COMPLETE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_STATUS_FILE_INCOMPLETE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_STATUS_GROUP_COMPLETE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_STATUS_GROUP_ERROR: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_STATUS_GROUP_FOREGROUND: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_STATUS_GROUP_INCOMPLETE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_STATUS_GROUP_SUSPENDED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_STATUS_JOB_COMPLETE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_STATUS_JOB_ERROR: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_STATUS_JOB_FOREGROUND: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_BackgroundIntelligentTransferService\"`*"]
pub const QM_STATUS_JOB_INCOMPLETE: u32 = 8u32;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
