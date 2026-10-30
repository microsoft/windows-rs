windows_core::imp::define_interface!(AsyncIBackgroundCopyCallback, AsyncIBackgroundCopyCallback_Vtbl, 0xca29d251_b4bb_4679_a3d9_ae8006119d54);
windows_core::imp::interface_hierarchy!(AsyncIBackgroundCopyCallback, windows_core::IUnknown);
impl AsyncIBackgroundCopyCallback {
    pub unsafe fn Begin_JobTransferred<P0>(&self, pjob: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IBackgroundCopyJob>,
    {
        unsafe { (windows_core::Interface::vtable(self).Begin_JobTransferred)(windows_core::Interface::as_raw(self), pjob.param().abi()) }
    }
    pub unsafe fn Finish_JobTransferred(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Finish_JobTransferred)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Begin_JobError<P0, P1>(&self, pjob: P0, perror: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IBackgroundCopyJob>,
        P1: windows_core::Param<IBackgroundCopyError>,
    {
        unsafe { (windows_core::Interface::vtable(self).Begin_JobError)(windows_core::Interface::as_raw(self), pjob.param().abi(), perror.param().abi()) }
    }
    pub unsafe fn Finish_JobError(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Finish_JobError)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Begin_JobModification<P0>(&self, pjob: P0, dwreserved: super::DWORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IBackgroundCopyJob>,
    {
        unsafe { (windows_core::Interface::vtable(self).Begin_JobModification)(windows_core::Interface::as_raw(self), pjob.param().abi(), dwreserved) }
    }
    pub unsafe fn Finish_JobModification(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Finish_JobModification)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIBackgroundCopyCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_JobTransferred: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Finish_JobTransferred: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Begin_JobError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Finish_JobError: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub Begin_JobModification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Begin_JobModification: usize,
    pub Finish_JobModification: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait AsyncIBackgroundCopyCallback_Impl: windows_core::IUnknownImpl {
    fn Begin_JobTransferred(&self, pjob: windows_core::Ref<IBackgroundCopyJob>) -> windows_core::Result<()>;
    fn Finish_JobTransferred(&self) -> windows_core::Result<()>;
    fn Begin_JobError(&self, pjob: windows_core::Ref<IBackgroundCopyJob>, perror: windows_core::Ref<IBackgroundCopyError>) -> windows_core::Result<()>;
    fn Finish_JobError(&self) -> windows_core::Result<()>;
    fn Begin_JobModification(&self, pjob: windows_core::Ref<IBackgroundCopyJob>, dwreserved: super::DWORD) -> windows_core::Result<()>;
    fn Finish_JobModification(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl AsyncIBackgroundCopyCallback_Vtbl {
    pub const fn new<Identity: AsyncIBackgroundCopyCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin_JobTransferred<Identity: AsyncIBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIBackgroundCopyCallback_Impl::Begin_JobTransferred(this, core::mem::transmute_copy(&pjob)).into()
            }
        }
        unsafe extern "system" fn Finish_JobTransferred<Identity: AsyncIBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIBackgroundCopyCallback_Impl::Finish_JobTransferred(this).into()
            }
        }
        unsafe extern "system" fn Begin_JobError<Identity: AsyncIBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void, perror: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIBackgroundCopyCallback_Impl::Begin_JobError(this, core::mem::transmute_copy(&pjob), core::mem::transmute_copy(&perror)).into()
            }
        }
        unsafe extern "system" fn Finish_JobError<Identity: AsyncIBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIBackgroundCopyCallback_Impl::Finish_JobError(this).into()
            }
        }
        unsafe extern "system" fn Begin_JobModification<Identity: AsyncIBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void, dwreserved: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIBackgroundCopyCallback_Impl::Begin_JobModification(this, core::mem::transmute_copy(&pjob), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn Finish_JobModification<Identity: AsyncIBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIBackgroundCopyCallback_Impl::Finish_JobModification(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_JobTransferred: Begin_JobTransferred::<Identity, OFFSET>,
            Finish_JobTransferred: Finish_JobTransferred::<Identity, OFFSET>,
            Begin_JobError: Begin_JobError::<Identity, OFFSET>,
            Finish_JobError: Finish_JobError::<Identity, OFFSET>,
            Begin_JobModification: Begin_JobModification::<Identity, OFFSET>,
            Finish_JobModification: Finish_JobModification::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIBackgroundCopyCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for AsyncIBackgroundCopyCallback {}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BG_AUTH_CREDENTIALS {
    pub Target: BG_AUTH_TARGET,
    pub Scheme: BG_AUTH_SCHEME,
    pub Credentials: BG_AUTH_CREDENTIALS_UNION,
}
impl Default for BG_AUTH_CREDENTIALS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union BG_AUTH_CREDENTIALS_UNION {
    pub Basic: BG_BASIC_CREDENTIALS,
}
impl Default for BG_AUTH_CREDENTIALS_UNION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type BG_AUTH_SCHEME = i32;
pub const BG_AUTH_SCHEME_BASIC: BG_AUTH_SCHEME = 1;
pub const BG_AUTH_SCHEME_DIGEST: BG_AUTH_SCHEME = 2;
pub const BG_AUTH_SCHEME_NEGOTIATE: BG_AUTH_SCHEME = 4;
pub const BG_AUTH_SCHEME_NTLM: BG_AUTH_SCHEME = 3;
pub const BG_AUTH_SCHEME_PASSPORT: BG_AUTH_SCHEME = 5;
pub type BG_AUTH_TARGET = i32;
pub const BG_AUTH_TARGET_PROXY: BG_AUTH_TARGET = 2;
pub const BG_AUTH_TARGET_SERVER: BG_AUTH_TARGET = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BG_BASIC_CREDENTIALS {
    pub UserName: windows_core::PWSTR,
    pub Password: windows_core::PWSTR,
}
pub type BG_CERT_STORE_LOCATION = i32;
pub const BG_CERT_STORE_LOCATION_CURRENT_SERVICE: BG_CERT_STORE_LOCATION = 2;
pub const BG_CERT_STORE_LOCATION_CURRENT_USER: BG_CERT_STORE_LOCATION = 0;
pub const BG_CERT_STORE_LOCATION_CURRENT_USER_GROUP_POLICY: BG_CERT_STORE_LOCATION = 5;
pub const BG_CERT_STORE_LOCATION_LOCAL_MACHINE: BG_CERT_STORE_LOCATION = 1;
pub const BG_CERT_STORE_LOCATION_LOCAL_MACHINE_ENTERPRISE: BG_CERT_STORE_LOCATION = 7;
pub const BG_CERT_STORE_LOCATION_LOCAL_MACHINE_GROUP_POLICY: BG_CERT_STORE_LOCATION = 6;
pub const BG_CERT_STORE_LOCATION_SERVICES: BG_CERT_STORE_LOCATION = 3;
pub const BG_CERT_STORE_LOCATION_USERS: BG_CERT_STORE_LOCATION = 4;
pub const BG_COPY_FILE_ALL: i32 = 15;
pub const BG_COPY_FILE_DACL: i32 = 4;
pub const BG_COPY_FILE_GROUP: i32 = 2;
pub const BG_COPY_FILE_OWNER: i32 = 1;
pub const BG_COPY_FILE_SACL: i32 = 8;
pub const BG_DISABLE_BRANCH_CACHE: i32 = 4;
pub const BG_ENABLE_PEERCACHING_CLIENT: i32 = 1;
pub const BG_ENABLE_PEERCACHING_SERVER: i32 = 2;
pub type BG_ERROR_CONTEXT = i32;
pub const BG_ERROR_CONTEXT_GENERAL_QUEUE_MANAGER: BG_ERROR_CONTEXT = 2;
pub const BG_ERROR_CONTEXT_GENERAL_TRANSPORT: BG_ERROR_CONTEXT = 6;
pub const BG_ERROR_CONTEXT_LOCAL_FILE: BG_ERROR_CONTEXT = 4;
pub const BG_ERROR_CONTEXT_NONE: BG_ERROR_CONTEXT = 0;
pub const BG_ERROR_CONTEXT_QUEUE_MANAGER_NOTIFICATION: BG_ERROR_CONTEXT = 3;
pub const BG_ERROR_CONTEXT_REMOTE_APPLICATION: BG_ERROR_CONTEXT = 7;
pub const BG_ERROR_CONTEXT_REMOTE_FILE: BG_ERROR_CONTEXT = 5;
pub const BG_ERROR_CONTEXT_SERVER_CERTIFICATE_CALLBACK: BG_ERROR_CONTEXT = 8;
pub const BG_ERROR_CONTEXT_UNKNOWN: BG_ERROR_CONTEXT = 1;
pub const BG_E_APP_PACKAGE_NOT_FOUND: u32 = 2149580906;
pub const BG_E_APP_PACKAGE_SCENARIO_NOT_SUPPORTED: u32 = 2149580907;
pub const BG_E_BLOCKED_BY_BACKGROUND_ACCESS_POLICY: u32 = 2149580910;
pub const BG_E_BLOCKED_BY_BATTERY_POLICY: u32 = 2149580903;
pub const BG_E_BLOCKED_BY_BATTERY_SAVER: u32 = 2149580904;
pub const BG_E_BLOCKED_BY_COST_TRANSFER_POLICY: u32 = 2149580889;
pub const BG_E_BLOCKED_BY_GAME_MODE: u32 = 2149580911;
pub const BG_E_BLOCKED_BY_POLICY: u32 = 2149580862;
pub const BG_E_BLOCKED_BY_SYSTEM_POLICY: u32 = 2149580912;
pub const BG_E_BUSYCACHERECORD: u32 = 2149580872;
pub const BG_E_CLIENT_SERVER_PROTOCOL_MISMATCH: u32 = 2149580834;
pub const BG_E_COMMIT_IN_PROGRESS: u32 = 2149580867;
pub const BG_E_CONNECTION_CLOSED: u32 = 2149580846;
pub const BG_E_CONNECT_FAILURE: u32 = 2149580845;
pub const BG_E_DATABASE_CORRUPT: u32 = 2149580908;
pub const BG_E_DESTINATION_LOCKED: u32 = 2149580813;
pub const BG_E_DISCOVERY_IN_PROGRESS: u32 = 2149580868;
pub const BG_E_EMPTY: u32 = 2149580803;
pub const BG_E_ERROR_CONTEXT_GENERAL_QUEUE_MANAGER: u32 = 2149580808;
pub const BG_E_ERROR_CONTEXT_GENERAL_TRANSPORT: u32 = 2149580811;
pub const BG_E_ERROR_CONTEXT_LOCAL_FILE: u32 = 2149580809;
pub const BG_E_ERROR_CONTEXT_QUEUE_MANAGER_NOTIFICATION: u32 = 2149580812;
pub const BG_E_ERROR_CONTEXT_REMOTE_APPLICATION: u32 = 2149580830;
pub const BG_E_ERROR_CONTEXT_REMOTE_FILE: u32 = 2149580810;
pub const BG_E_ERROR_CONTEXT_SERVER_CERTIFICATE_CALLBACK: u32 = 2149580918;
pub const BG_E_ERROR_CONTEXT_UNKNOWN: u32 = 2149580807;
pub const BG_E_ERROR_INFORMATION_UNAVAILABLE: u32 = 2149580815;
pub const BG_E_FILE_NOT_AVAILABLE: u32 = 2149580804;
pub const BG_E_FILE_NOT_FOUND: u32 = 2149580841;
pub const BG_E_HTTP_ERROR_100: u32 = 2149122148;
pub const BG_E_HTTP_ERROR_101: u32 = 2149122149;
pub const BG_E_HTTP_ERROR_200: u32 = 2149122248;
pub const BG_E_HTTP_ERROR_201: u32 = 2149122249;
pub const BG_E_HTTP_ERROR_202: u32 = 2149122250;
pub const BG_E_HTTP_ERROR_203: u32 = 2149122251;
pub const BG_E_HTTP_ERROR_204: u32 = 2149122252;
pub const BG_E_HTTP_ERROR_205: u32 = 2149122253;
pub const BG_E_HTTP_ERROR_206: u32 = 2149122254;
pub const BG_E_HTTP_ERROR_300: u32 = 2149122348;
pub const BG_E_HTTP_ERROR_301: u32 = 2149122349;
pub const BG_E_HTTP_ERROR_302: u32 = 2149122350;
pub const BG_E_HTTP_ERROR_303: u32 = 2149122351;
pub const BG_E_HTTP_ERROR_304: u32 = 2149122352;
pub const BG_E_HTTP_ERROR_305: u32 = 2149122353;
pub const BG_E_HTTP_ERROR_307: u32 = 2149122355;
pub const BG_E_HTTP_ERROR_400: u32 = 2149122448;
pub const BG_E_HTTP_ERROR_401: u32 = 2149122449;
pub const BG_E_HTTP_ERROR_402: u32 = 2149122450;
pub const BG_E_HTTP_ERROR_403: u32 = 2149122451;
pub const BG_E_HTTP_ERROR_404: u32 = 2149122452;
pub const BG_E_HTTP_ERROR_405: u32 = 2149122453;
pub const BG_E_HTTP_ERROR_406: u32 = 2149122454;
pub const BG_E_HTTP_ERROR_407: u32 = 2149122455;
pub const BG_E_HTTP_ERROR_408: u32 = 2149122456;
pub const BG_E_HTTP_ERROR_409: u32 = 2149122457;
pub const BG_E_HTTP_ERROR_410: u32 = 2149122458;
pub const BG_E_HTTP_ERROR_411: u32 = 2149122459;
pub const BG_E_HTTP_ERROR_412: u32 = 2149122460;
pub const BG_E_HTTP_ERROR_413: u32 = 2149122461;
pub const BG_E_HTTP_ERROR_414: u32 = 2149122462;
pub const BG_E_HTTP_ERROR_415: u32 = 2149122463;
pub const BG_E_HTTP_ERROR_416: u32 = 2149122464;
pub const BG_E_HTTP_ERROR_417: u32 = 2149122465;
pub const BG_E_HTTP_ERROR_449: u32 = 2149122497;
pub const BG_E_HTTP_ERROR_500: u32 = 2149122548;
pub const BG_E_HTTP_ERROR_501: u32 = 2149122549;
pub const BG_E_HTTP_ERROR_502: u32 = 2149122550;
pub const BG_E_HTTP_ERROR_503: u32 = 2149122551;
pub const BG_E_HTTP_ERROR_504: u32 = 2149122552;
pub const BG_E_HTTP_ERROR_505: u32 = 2149122553;
pub const BG_E_INSUFFICIENT_HTTP_SUPPORT: u32 = 2149580818;
pub const BG_E_INSUFFICIENT_RANGE_SUPPORT: u32 = 2149580819;
pub const BG_E_INVALID_AUTH_SCHEME: u32 = 2149580840;
pub const BG_E_INVALID_AUTH_TARGET: u32 = 2149580839;
pub const BG_E_INVALID_CREDENTIALS: u32 = 2149580864;
pub const BG_E_INVALID_HASH_ALGORITHM: u32 = 2149580865;
pub const BG_E_INVALID_PROXY_INFO: u32 = 2149580863;
pub const BG_E_INVALID_RANGE: u32 = 2149580843;
pub const BG_E_INVALID_SERVER_RESPONSE: u32 = 2149580827;
pub const BG_E_INVALID_STATE: u32 = 2149580802;
pub const BG_E_LOCAL_FILE_CHANGED: u32 = 2149580829;
pub const BG_E_MAXDOWNLOAD_TIMEOUT: u32 = 2149580884;
pub const BG_E_MAX_DOWNLOAD_SIZE_INVALID_VALUE: u32 = 2149580899;
pub const BG_E_MAX_DOWNLOAD_SIZE_LIMIT_REACHED: u32 = 2149580900;
pub const BG_E_MISSING_FILE_SIZE: u32 = 2149580817;
pub const BG_E_NETWORK_DISCONNECTED: u32 = 2149580816;
pub const BG_E_NEW_OWNER_DIFF_MAPPING: u32 = 2149580821;
pub const BG_E_NEW_OWNER_NO_FILE_ACCESS: u32 = 2149580822;
pub const BG_E_NOT_FOUND: u32 = 2149580801;
pub const BG_E_NOT_SUPPORTED_WITH_CUSTOM_HTTP_METHOD: u32 = 2149580913;
pub const BG_E_NO_PROGRESS: u32 = 2149580836;
pub const BG_E_OVERLAPPING_RANGES: u32 = 2149580844;
pub const BG_E_PASSWORD_TOO_LARGE: u32 = 2149580838;
pub const BG_E_PEERCACHING_DISABLED: u32 = 2149580871;
pub const BG_E_PROPERTY_SUPPORTED_FOR_DOWNLOAD_JOBS_ONLY: u32 = 2149580896;
pub const BG_E_PROTOCOL_NOT_AVAILABLE: u32 = 2149580805;
pub const BG_E_PROXY_BYPASS_LIST_TOO_LARGE: u32 = 2149580825;
pub const BG_E_PROXY_LIST_TOO_LARGE: u32 = 2149580824;
pub const BG_E_RANDOM_ACCESS_NOT_SUPPORTED: u32 = 2149580909;
pub const BG_E_READ_ONLY_PROPERTY: u32 = 2149580888;
pub const BG_E_READ_ONLY_PROPERTY_AFTER_ADDFILE: u32 = 2149580897;
pub const BG_E_READ_ONLY_PROPERTY_AFTER_RESUME: u32 = 2149580898;
pub const BG_E_READ_ONLY_WHEN_JOB_ACTIVE: u32 = 2149580917;
pub const BG_E_RECORD_DELETED: u32 = 2149580866;
pub const BG_E_REMOTE_FILE_CHANGED: u32 = 2149580915;
pub const BG_E_REMOTE_NOT_SUPPORTED: u32 = 2149580820;
pub const BG_E_SERVER_CERT_VALIDATION_INTERFACE_REQUIRED: u32 = 2149580916;
pub const BG_E_SERVER_EXECUTE_ENABLE: u32 = 2149580835;
pub const BG_E_SESSION_NOT_FOUND: u32 = 2149580831;
pub const BG_E_STANDBY_MODE: u32 = 2149580901;
pub const BG_E_STRING_TOO_LONG: u32 = 2149580833;
pub const BG_E_TEST_OPTION_BLOCKED_DOWNLOAD: u32 = 2149580870;
pub const BG_E_TOKEN_REQUIRED: u32 = 2149580886;
pub const BG_E_TOO_LARGE: u32 = 2149580832;
pub const BG_E_TOO_MANY_FILES: u32 = 2149580828;
pub const BG_E_TOO_MANY_FILES_IN_JOB: u32 = 2149580881;
pub const BG_E_TOO_MANY_JOBS_PER_MACHINE: u32 = 2149580880;
pub const BG_E_TOO_MANY_JOBS_PER_USER: u32 = 2149580873;
pub const BG_E_TOO_MANY_RANGES_IN_FILE: u32 = 2149580882;
pub const BG_E_UNKNOWN_PROPERTY_ID: u32 = 2149580887;
pub const BG_E_UNSUPPORTED_JOB_CONFIGURATION: u32 = 2149580914;
pub const BG_E_UPNP_ERROR: u32 = 2149580869;
pub const BG_E_USERNAME_TOO_LARGE: u32 = 2149580837;
pub const BG_E_USE_STORED_CREDENTIALS_NOT_SUPPORTED: u32 = 2149580902;
pub const BG_E_VALIDATION_FAILED: u32 = 2149580883;
pub const BG_E_VOLUME_CHANGED: u32 = 2149580814;
pub const BG_E_WATCHDOG_TIMEOUT: u32 = 2149580905;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BG_FILE_INFO {
    pub RemoteName: windows_core::PWSTR,
    pub LocalName: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BG_FILE_PROGRESS {
    pub BytesTotal: super::UINT64,
    pub BytesTransferred: super::UINT64,
    pub Completed: windows_core::BOOL,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BG_FILE_RANGE {
    pub InitialOffset: super::UINT64,
    pub Length: super::UINT64,
}
pub const BG_HTTP_REDIRECT_POLICY_ALLOW_HTTPS_TO_HTTP: i32 = 2048;
pub const BG_HTTP_REDIRECT_POLICY_ALLOW_REPORT: i32 = 256;
pub const BG_HTTP_REDIRECT_POLICY_ALLOW_SILENT: i32 = 0;
pub const BG_HTTP_REDIRECT_POLICY_DISALLOW: i32 = 512;
pub const BG_HTTP_REDIRECT_POLICY_MASK: i32 = 1792;
pub const BG_JOB_DISABLE_BRANCH_CACHE: i32 = 4;
pub const BG_JOB_ENABLE_PEERCACHING_CLIENT: i32 = 1;
pub const BG_JOB_ENABLE_PEERCACHING_SERVER: i32 = 2;
pub const BG_JOB_ENUM_ALL_USERS: i32 = 1;
pub type BG_JOB_PRIORITY = i32;
pub const BG_JOB_PRIORITY_FOREGROUND: BG_JOB_PRIORITY = 0;
pub const BG_JOB_PRIORITY_HIGH: BG_JOB_PRIORITY = 1;
pub const BG_JOB_PRIORITY_LOW: BG_JOB_PRIORITY = 3;
pub const BG_JOB_PRIORITY_NORMAL: BG_JOB_PRIORITY = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BG_JOB_PROGRESS {
    pub BytesTotal: super::UINT64,
    pub BytesTransferred: super::UINT64,
    pub FilesTotal: super::ULONG,
    pub FilesTransferred: super::ULONG,
}
pub type BG_JOB_PROXY_USAGE = i32;
pub const BG_JOB_PROXY_USAGE_AUTODETECT: BG_JOB_PROXY_USAGE = 3;
pub const BG_JOB_PROXY_USAGE_NO_PROXY: BG_JOB_PROXY_USAGE = 1;
pub const BG_JOB_PROXY_USAGE_OVERRIDE: BG_JOB_PROXY_USAGE = 2;
pub const BG_JOB_PROXY_USAGE_PRECONFIG: BG_JOB_PROXY_USAGE = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BG_JOB_REPLY_PROGRESS {
    pub BytesTotal: super::UINT64,
    pub BytesTransferred: super::UINT64,
}
pub type BG_JOB_STATE = i32;
pub const BG_JOB_STATE_ACKNOWLEDGED: BG_JOB_STATE = 7;
pub const BG_JOB_STATE_CANCELLED: BG_JOB_STATE = 8;
pub const BG_JOB_STATE_CONNECTING: BG_JOB_STATE = 1;
pub const BG_JOB_STATE_ERROR: BG_JOB_STATE = 4;
pub const BG_JOB_STATE_QUEUED: BG_JOB_STATE = 0;
pub const BG_JOB_STATE_SUSPENDED: BG_JOB_STATE = 3;
pub const BG_JOB_STATE_TRANSFERRED: BG_JOB_STATE = 6;
pub const BG_JOB_STATE_TRANSFERRING: BG_JOB_STATE = 2;
pub const BG_JOB_STATE_TRANSIENT_ERROR: BG_JOB_STATE = 5;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BG_JOB_TIMES {
    pub CreationTime: super::FILETIME,
    pub ModificationTime: super::FILETIME,
    pub TransferCompletionTime: super::FILETIME,
}
pub type BG_JOB_TYPE = i32;
pub const BG_JOB_TYPE_DOWNLOAD: BG_JOB_TYPE = 0;
pub const BG_JOB_TYPE_UPLOAD: BG_JOB_TYPE = 1;
pub const BG_JOB_TYPE_UPLOAD_REPLY: BG_JOB_TYPE = 2;
#[cfg(feature = "shared")]
pub const BG_LENGTH_TO_EOF: super::UINT64 = 18446744073709551615;
pub const BG_NOTIFY_DISABLE: i32 = 4;
pub const BG_NOTIFY_FILE_RANGES_TRANSFERRED: i32 = 32;
pub const BG_NOTIFY_FILE_TRANSFERRED: i32 = 16;
pub const BG_NOTIFY_JOB_ERROR: i32 = 2;
pub const BG_NOTIFY_JOB_MODIFICATION: i32 = 8;
pub const BG_NOTIFY_JOB_TRANSFERRED: i32 = 1;
#[cfg(feature = "shared")]
pub const BG_SIZE_UNKNOWN: super::UINT64 = 18446744073709551615;
pub const BG_SSL_ENABLE_CRL_CHECK: i32 = 1;
pub const BG_SSL_IGNORE_CERT_CN_INVALID: i32 = 2;
pub const BG_SSL_IGNORE_CERT_DATE_INVALID: i32 = 4;
pub const BG_SSL_IGNORE_CERT_WRONG_USAGE: i32 = 16;
pub const BG_SSL_IGNORE_UNKNOWN_CA: i32 = 8;
pub const BG_S_ERROR_CONTEXT_NONE: i32 = 2097158;
pub const BG_S_OVERRIDDEN_BY_POLICY: i32 = 2097237;
pub const BG_S_PARTIAL_COMPLETE: i32 = 2097175;
pub const BG_S_PROXY_CHANGED: i32 = 2097194;
pub const BG_S_UNABLE_TO_DELETE_FILES: i32 = 2097178;
pub const BG_TOKEN_LOCAL_FILE: i32 = 1;
pub const BG_TOKEN_NETWORK: i32 = 2;
pub const BITS_COST_OPTION_IGNORE_CONGESTION: u32 = 2147483648;
pub const BITS_COST_STATE_BELOW_CAP: i32 = 4;
pub const BITS_COST_STATE_CAPPED_USAGE_UNKNOWN: i32 = 2;
pub const BITS_COST_STATE_NEAR_CAP: i32 = 8;
pub const BITS_COST_STATE_OVERCAP_CHARGED: i32 = 16;
pub const BITS_COST_STATE_OVERCAP_THROTTLED: i32 = 32;
pub const BITS_COST_STATE_RESERVED: i32 = 1073741824;
pub const BITS_COST_STATE_ROAMING: i32 = 128;
pub const BITS_COST_STATE_TRANSFER_ALWAYS: u32 = 2147483903;
pub const BITS_COST_STATE_TRANSFER_NOT_ROAMING: u32 = 2147483775;
pub const BITS_COST_STATE_TRANSFER_NO_SURCHARGE: u32 = 2147483759;
pub const BITS_COST_STATE_TRANSFER_STANDARD: u32 = 2147483751;
pub const BITS_COST_STATE_TRANSFER_UNRESTRICTED: u32 = 2147483681;
pub const BITS_COST_STATE_UNRESTRICTED: i32 = 1;
pub const BITS_COST_STATE_USAGE_BASED: i32 = 64;
pub type BITS_FILE_PROPERTY_ID = i32;
pub const BITS_FILE_PROPERTY_ID_HTTP_RESPONSE_HEADERS: BITS_FILE_PROPERTY_ID = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub union BITS_FILE_PROPERTY_VALUE {
    pub String: windows_core::PWSTR,
}
impl Default for BITS_FILE_PROPERTY_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BITS_JOB_PROPERTY_DYNAMIC_CONTENT: BITS_JOB_PROPERTY_ID = 3;
pub const BITS_JOB_PROPERTY_HIGH_PERFORMANCE: BITS_JOB_PROPERTY_ID = 4;
pub type BITS_JOB_PROPERTY_ID = i32;
pub const BITS_JOB_PROPERTY_ID_COST_FLAGS: BITS_JOB_PROPERTY_ID = 1;
pub const BITS_JOB_PROPERTY_MAX_DOWNLOAD_SIZE: BITS_JOB_PROPERTY_ID = 5;
pub const BITS_JOB_PROPERTY_MINIMUM_NOTIFICATION_INTERVAL_MS: BITS_JOB_PROPERTY_ID = 9;
pub const BITS_JOB_PROPERTY_NOTIFICATION_CLSID: BITS_JOB_PROPERTY_ID = 2;
pub const BITS_JOB_PROPERTY_ON_DEMAND_MODE: BITS_JOB_PROPERTY_ID = 10;
pub const BITS_JOB_PROPERTY_USE_STORED_CREDENTIALS: BITS_JOB_PROPERTY_ID = 7;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union BITS_JOB_PROPERTY_VALUE {
    pub Dword: super::DWORD,
    pub ClsID: windows_core::GUID,
    pub Enable: windows_core::BOOL,
    pub Uint64: super::UINT64,
    pub Target: BG_AUTH_TARGET,
}
#[cfg(feature = "shared")]
impl Default for BITS_JOB_PROPERTY_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type BITS_JOB_TRANSFER_POLICY = i32;
pub const BITS_JOB_TRANSFER_POLICY_ALWAYS: BITS_JOB_TRANSFER_POLICY = -2147483393;
pub const BITS_JOB_TRANSFER_POLICY_NOT_ROAMING: BITS_JOB_TRANSFER_POLICY = -2147483521;
pub const BITS_JOB_TRANSFER_POLICY_NO_SURCHARGE: BITS_JOB_TRANSFER_POLICY = -2147483537;
pub const BITS_JOB_TRANSFER_POLICY_STANDARD: BITS_JOB_TRANSFER_POLICY = -2147483545;
pub const BITS_JOB_TRANSFER_POLICY_UNRESTRICTED: BITS_JOB_TRANSFER_POLICY = -2147483615;
pub const BITS_MC_FAILED_TO_START: u32 = 2149138440;
pub const BITS_MC_FATAL_IGD_ERROR: u32 = 2149138441;
pub const BITS_MC_FILE_DELETION_FAILED: u32 = 2149138433;
pub const BITS_MC_FILE_DELETION_FAILED_MORE: u32 = 2149138434;
pub const BITS_MC_JOB_CANCELLED: u32 = 2149138432;
pub const BITS_MC_JOB_NOTIFICATION_FAILURE: u32 = 2149138438;
pub const BITS_MC_JOB_PROPERTY_CHANGE: u32 = 2149138435;
pub const BITS_MC_JOB_SCAVENGED: u32 = 2149138437;
pub const BITS_MC_JOB_TAKE_OWNERSHIP: u32 = 2149138436;
pub const BITS_MC_PEERCACHING_PORT: u32 = 2149138442;
pub const BITS_MC_STATE_FILE_CORRUPT: u32 = 2149138439;
pub const BITS_MC_WSD_PORT: u32 = 2149138443;
pub const BackgroundCopyManager: windows_core::GUID = windows_core::GUID::from_u128(0x4991d34b_80a1_4291_83b6_3328366b9097);
pub const BackgroundCopyManager10_1: windows_core::GUID = windows_core::GUID::from_u128(0x4bd3e4e1_7bd4_4a2b_9964_496400de5193);
pub const BackgroundCopyManager10_2: windows_core::GUID = windows_core::GUID::from_u128(0x4575438f_a6c8_4976_b0fe_2f26b80d959e);
pub const BackgroundCopyManager10_3: windows_core::GUID = windows_core::GUID::from_u128(0x5fd42ad5_c04e_4d36_adc7_e08ff15737ad);
pub const BackgroundCopyManager1_5: windows_core::GUID = windows_core::GUID::from_u128(0xf087771f_d74f_4c1a_bb8a_e16aca9124ea);
pub const BackgroundCopyManager2_0: windows_core::GUID = windows_core::GUID::from_u128(0x6d18ad12_bde3_4393_b311_099c346e6df9);
pub const BackgroundCopyManager2_5: windows_core::GUID = windows_core::GUID::from_u128(0x03ca98d6_ff5d_49b8_abc6_03dd84127020);
pub const BackgroundCopyManager3_0: windows_core::GUID = windows_core::GUID::from_u128(0x659cdea7_489e_11d9_a9cd_000d56965251);
pub const BackgroundCopyManager4_0: windows_core::GUID = windows_core::GUID::from_u128(0xbb6df56b_cace_11dc_9992_0019b93a3a84);
pub const BackgroundCopyManager5_0: windows_core::GUID = windows_core::GUID::from_u128(0x1ecca34c_e88a_44e3_8d6a_8921bde9e452);
windows_core::imp::define_interface!(IBackgroundCopyCallback, IBackgroundCopyCallback_Vtbl, 0x97ea99c7_0186_4ad4_8df9_c5b4e0ed6b22);
windows_core::imp::interface_hierarchy!(IBackgroundCopyCallback, windows_core::IUnknown);
impl IBackgroundCopyCallback {
    pub unsafe fn JobTransferred<P0>(&self, pjob: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IBackgroundCopyJob>,
    {
        unsafe { (windows_core::Interface::vtable(self).JobTransferred)(windows_core::Interface::as_raw(self), pjob.param().abi()) }
    }
    pub unsafe fn JobError<P0, P1>(&self, pjob: P0, perror: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IBackgroundCopyJob>,
        P1: windows_core::Param<IBackgroundCopyError>,
    {
        unsafe { (windows_core::Interface::vtable(self).JobError)(windows_core::Interface::as_raw(self), pjob.param().abi(), perror.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn JobModification<P0>(&self, pjob: P0, dwreserved: super::DWORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IBackgroundCopyJob>,
    {
        unsafe { (windows_core::Interface::vtable(self).JobModification)(windows_core::Interface::as_raw(self), pjob.param().abi(), dwreserved) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub JobTransferred: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub JobError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub JobModification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    JobModification: usize,
}
#[cfg(feature = "shared")]
pub trait IBackgroundCopyCallback_Impl: windows_core::IUnknownImpl {
    fn JobTransferred(&self, pjob: windows_core::Ref<IBackgroundCopyJob>) -> windows_core::Result<()>;
    fn JobError(&self, pjob: windows_core::Ref<IBackgroundCopyJob>, perror: windows_core::Ref<IBackgroundCopyError>) -> windows_core::Result<()>;
    fn JobModification(&self, pjob: windows_core::Ref<IBackgroundCopyJob>, dwreserved: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IBackgroundCopyCallback_Vtbl {
    pub const fn new<Identity: IBackgroundCopyCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn JobTransferred<Identity: IBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyCallback_Impl::JobTransferred(this, core::mem::transmute_copy(&pjob)).into()
            }
        }
        unsafe extern "system" fn JobError<Identity: IBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void, perror: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyCallback_Impl::JobError(this, core::mem::transmute_copy(&pjob), core::mem::transmute_copy(&perror)).into()
            }
        }
        unsafe extern "system" fn JobModification<Identity: IBackgroundCopyCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void, dwreserved: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyCallback_Impl::JobModification(this, core::mem::transmute_copy(&pjob), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            JobTransferred: JobTransferred::<Identity, OFFSET>,
            JobError: JobError::<Identity, OFFSET>,
            JobModification: JobModification::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IBackgroundCopyCallback {}
windows_core::imp::define_interface!(IBackgroundCopyCallback2, IBackgroundCopyCallback2_Vtbl, 0x659cdeac_489e_11d9_a9cd_000d56965251);
impl core::ops::Deref for IBackgroundCopyCallback2 {
    type Target = IBackgroundCopyCallback;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyCallback2, windows_core::IUnknown, IBackgroundCopyCallback);
impl IBackgroundCopyCallback2 {
    pub unsafe fn FileTransferred<P0, P1>(&self, pjob: P0, pfile: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IBackgroundCopyJob>,
        P1: windows_core::Param<IBackgroundCopyFile>,
    {
        unsafe { (windows_core::Interface::vtable(self).FileTransferred)(windows_core::Interface::as_raw(self), pjob.param().abi(), pfile.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyCallback2_Vtbl {
    pub base__: IBackgroundCopyCallback_Vtbl,
    pub FileTransferred: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IBackgroundCopyCallback2_Impl: IBackgroundCopyCallback_Impl {
    fn FileTransferred(&self, pjob: windows_core::Ref<IBackgroundCopyJob>, pfile: windows_core::Ref<IBackgroundCopyFile>) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IBackgroundCopyCallback2_Vtbl {
    pub const fn new<Identity: IBackgroundCopyCallback2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FileTransferred<Identity: IBackgroundCopyCallback2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void, pfile: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyCallback2_Impl::FileTransferred(this, core::mem::transmute_copy(&pjob), core::mem::transmute_copy(&pfile)).into()
            }
        }
        Self { base__: IBackgroundCopyCallback_Vtbl::new::<Identity, OFFSET>(), FileTransferred: FileTransferred::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyCallback2 as windows_core::Interface>::IID || iid == &<IBackgroundCopyCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IBackgroundCopyCallback2 {}
windows_core::imp::define_interface!(IBackgroundCopyCallback3, IBackgroundCopyCallback3_Vtbl, 0x98c97bd2_e32b_4ad8_a528_95fd8b16bd42);
impl core::ops::Deref for IBackgroundCopyCallback3 {
    type Target = IBackgroundCopyCallback2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyCallback3, windows_core::IUnknown, IBackgroundCopyCallback, IBackgroundCopyCallback2);
impl IBackgroundCopyCallback3 {
    #[cfg(feature = "shared")]
    pub unsafe fn FileRangesTransferred<P0, P1>(&self, job: P0, file: P1, ranges: &[BG_FILE_RANGE]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IBackgroundCopyJob>,
        P1: windows_core::Param<IBackgroundCopyFile>,
    {
        unsafe { (windows_core::Interface::vtable(self).FileRangesTransferred)(windows_core::Interface::as_raw(self), job.param().abi(), file.param().abi(), ranges.len().try_into().unwrap(), ranges.as_ptr()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyCallback3_Vtbl {
    pub base__: IBackgroundCopyCallback2_Vtbl,
    #[cfg(feature = "shared")]
    pub FileRangesTransferred: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::DWORD, *const BG_FILE_RANGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    FileRangesTransferred: usize,
}
#[cfg(feature = "shared")]
pub trait IBackgroundCopyCallback3_Impl: IBackgroundCopyCallback2_Impl {
    fn FileRangesTransferred(&self, job: windows_core::Ref<IBackgroundCopyJob>, file: windows_core::Ref<IBackgroundCopyFile>, rangecount: super::DWORD, ranges: *const BG_FILE_RANGE) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IBackgroundCopyCallback3_Vtbl {
    pub const fn new<Identity: IBackgroundCopyCallback3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FileRangesTransferred<Identity: IBackgroundCopyCallback3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, job: *mut core::ffi::c_void, file: *mut core::ffi::c_void, rangecount: super::DWORD, ranges: *const BG_FILE_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyCallback3_Impl::FileRangesTransferred(this, core::mem::transmute_copy(&job), core::mem::transmute_copy(&file), core::mem::transmute_copy(&rangecount), core::mem::transmute_copy(&ranges)).into()
            }
        }
        Self { base__: IBackgroundCopyCallback2_Vtbl::new::<Identity, OFFSET>(), FileRangesTransferred: FileRangesTransferred::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyCallback3 as windows_core::Interface>::IID || iid == &<IBackgroundCopyCallback as windows_core::Interface>::IID || iid == &<IBackgroundCopyCallback2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IBackgroundCopyCallback3 {}
windows_core::imp::define_interface!(IBackgroundCopyError, IBackgroundCopyError_Vtbl, 0x19c613a0_fcb8_4f28_81ae_897c3d078f81);
windows_core::imp::interface_hierarchy!(IBackgroundCopyError, windows_core::IUnknown);
impl IBackgroundCopyError {
    pub unsafe fn GetError(&self, pcontext: *mut BG_ERROR_CONTEXT, pcode: *mut windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetError)(windows_core::Interface::as_raw(self), pcontext as _, pcode as _) }
    }
    pub unsafe fn GetFile(&self, pval: *mut Option<IBackgroundCopyFile>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFile)(windows_core::Interface::as_raw(self), core::mem::transmute(pval)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetErrorDescription(&self, languageid: super::DWORD, perrordescription: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetErrorDescription)(windows_core::Interface::as_raw(self), languageid, perrordescription as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetErrorContextDescription(&self, languageid: super::DWORD, pcontextdescription: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetErrorContextDescription)(windows_core::Interface::as_raw(self), languageid, pcontextdescription as _) }
    }
    pub unsafe fn GetProtocol(&self, pprotocol: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetProtocol)(windows_core::Interface::as_raw(self), pprotocol as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyError_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_ERROR_CONTEXT, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetErrorDescription: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetErrorDescription: usize,
    #[cfg(feature = "shared")]
    pub GetErrorContextDescription: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetErrorContextDescription: usize,
    pub GetProtocol: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IBackgroundCopyError_Impl: windows_core::IUnknownImpl {
    fn GetError(&self, pcontext: *mut BG_ERROR_CONTEXT, pcode: *mut windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetFile(&self, pval: windows_core::OutRef<IBackgroundCopyFile>) -> windows_core::Result<()>;
    fn GetErrorDescription(&self, languageid: super::DWORD, perrordescription: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetErrorContextDescription(&self, languageid: super::DWORD, pcontextdescription: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetProtocol(&self, pprotocol: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IBackgroundCopyError_Vtbl {
    pub const fn new<Identity: IBackgroundCopyError_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetError<Identity: IBackgroundCopyError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *mut BG_ERROR_CONTEXT, pcode: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyError_Impl::GetError(this, core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&pcode)).into()
            }
        }
        unsafe extern "system" fn GetFile<Identity: IBackgroundCopyError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyError_Impl::GetFile(this, core::mem::transmute_copy(&pval)).into()
            }
        }
        unsafe extern "system" fn GetErrorDescription<Identity: IBackgroundCopyError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, languageid: super::DWORD, perrordescription: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyError_Impl::GetErrorDescription(this, core::mem::transmute_copy(&languageid), core::mem::transmute_copy(&perrordescription)).into()
            }
        }
        unsafe extern "system" fn GetErrorContextDescription<Identity: IBackgroundCopyError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, languageid: super::DWORD, pcontextdescription: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyError_Impl::GetErrorContextDescription(this, core::mem::transmute_copy(&languageid), core::mem::transmute_copy(&pcontextdescription)).into()
            }
        }
        unsafe extern "system" fn GetProtocol<Identity: IBackgroundCopyError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprotocol: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyError_Impl::GetProtocol(this, core::mem::transmute_copy(&pprotocol)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetError: GetError::<Identity, OFFSET>,
            GetFile: GetFile::<Identity, OFFSET>,
            GetErrorDescription: GetErrorDescription::<Identity, OFFSET>,
            GetErrorContextDescription: GetErrorContextDescription::<Identity, OFFSET>,
            GetProtocol: GetProtocol::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyError as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IBackgroundCopyError {}
windows_core::imp::define_interface!(IBackgroundCopyFile, IBackgroundCopyFile_Vtbl, 0x01b7bd23_fb88_4a77_8490_5891d3e4653a);
windows_core::imp::interface_hierarchy!(IBackgroundCopyFile, windows_core::IUnknown);
impl IBackgroundCopyFile {
    pub unsafe fn GetRemoteName(&self, pval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRemoteName)(windows_core::Interface::as_raw(self), pval as _) }
    }
    pub unsafe fn GetLocalName(&self, pval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLocalName)(windows_core::Interface::as_raw(self), pval as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetProgress(&self, pval: *mut BG_FILE_PROGRESS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetProgress)(windows_core::Interface::as_raw(self), pval as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyFile_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRemoteName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetLocalName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_FILE_PROGRESS) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetProgress: usize,
}
#[cfg(feature = "shared")]
pub trait IBackgroundCopyFile_Impl: windows_core::IUnknownImpl {
    fn GetRemoteName(&self, pval: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetLocalName(&self, pval: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetProgress(&self, pval: *mut BG_FILE_PROGRESS) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IBackgroundCopyFile_Vtbl {
    pub const fn new<Identity: IBackgroundCopyFile_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRemoteName<Identity: IBackgroundCopyFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyFile_Impl::GetRemoteName(this, core::mem::transmute_copy(&pval)).into()
            }
        }
        unsafe extern "system" fn GetLocalName<Identity: IBackgroundCopyFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyFile_Impl::GetLocalName(this, core::mem::transmute_copy(&pval)).into()
            }
        }
        unsafe extern "system" fn GetProgress<Identity: IBackgroundCopyFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut BG_FILE_PROGRESS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyFile_Impl::GetProgress(this, core::mem::transmute_copy(&pval)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRemoteName: GetRemoteName::<Identity, OFFSET>,
            GetLocalName: GetLocalName::<Identity, OFFSET>,
            GetProgress: GetProgress::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyFile as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IBackgroundCopyFile {}
windows_core::imp::define_interface!(IBackgroundCopyFile2, IBackgroundCopyFile2_Vtbl, 0x83e81b93_0873_474d_8a8c_f2018b1a939c);
impl core::ops::Deref for IBackgroundCopyFile2 {
    type Target = IBackgroundCopyFile;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyFile2, windows_core::IUnknown, IBackgroundCopyFile);
impl IBackgroundCopyFile2 {
    #[cfg(feature = "shared")]
    pub unsafe fn GetFileRanges(&self, rangecount: Option<*mut super::DWORD>, ranges: *mut *mut BG_FILE_RANGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFileRanges)(windows_core::Interface::as_raw(self), rangecount.unwrap_or(core::mem::zeroed()) as _, ranges as _) }
    }
    pub unsafe fn SetRemoteName<P0>(&self, val: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRemoteName)(windows_core::Interface::as_raw(self), val.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyFile2_Vtbl {
    pub base__: IBackgroundCopyFile_Vtbl,
    #[cfg(feature = "shared")]
    pub GetFileRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD, *mut *mut BG_FILE_RANGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetFileRanges: usize,
    pub SetRemoteName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IBackgroundCopyFile2_Impl: IBackgroundCopyFile_Impl {
    fn GetFileRanges(&self, rangecount: *mut super::DWORD, ranges: *mut *mut BG_FILE_RANGE) -> windows_core::Result<()>;
    fn SetRemoteName(&self, val: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IBackgroundCopyFile2_Vtbl {
    pub const fn new<Identity: IBackgroundCopyFile2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFileRanges<Identity: IBackgroundCopyFile2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rangecount: *mut super::DWORD, ranges: *mut *mut BG_FILE_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyFile2_Impl::GetFileRanges(this, core::mem::transmute_copy(&rangecount), core::mem::transmute_copy(&ranges)).into()
            }
        }
        unsafe extern "system" fn SetRemoteName<Identity: IBackgroundCopyFile2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyFile2_Impl::SetRemoteName(this, core::mem::transmute(&val)).into()
            }
        }
        Self {
            base__: IBackgroundCopyFile_Vtbl::new::<Identity, OFFSET>(),
            GetFileRanges: GetFileRanges::<Identity, OFFSET>,
            SetRemoteName: SetRemoteName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyFile2 as windows_core::Interface>::IID || iid == &<IBackgroundCopyFile as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IBackgroundCopyFile2 {}
windows_core::imp::define_interface!(IBackgroundCopyFile3, IBackgroundCopyFile3_Vtbl, 0x659cdeaa_489e_11d9_a9cd_000d56965251);
impl core::ops::Deref for IBackgroundCopyFile3 {
    type Target = IBackgroundCopyFile2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyFile3, windows_core::IUnknown, IBackgroundCopyFile, IBackgroundCopyFile2);
impl IBackgroundCopyFile3 {
    pub unsafe fn GetTemporaryName(&self, pfilename: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTemporaryName)(windows_core::Interface::as_raw(self), pfilename as _) }
    }
    pub unsafe fn SetValidationState(&self, state: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetValidationState)(windows_core::Interface::as_raw(self), state.into()) }
    }
    pub unsafe fn GetValidationState(&self, pstate: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetValidationState)(windows_core::Interface::as_raw(self), pstate as _) }
    }
    pub unsafe fn IsDownloadedFromPeer(&self, pval: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsDownloadedFromPeer)(windows_core::Interface::as_raw(self), pval as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyFile3_Vtbl {
    pub base__: IBackgroundCopyFile2_Vtbl,
    pub GetTemporaryName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetValidationState: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetValidationState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsDownloadedFromPeer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IBackgroundCopyFile3_Impl: IBackgroundCopyFile2_Impl {
    fn GetTemporaryName(&self, pfilename: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetValidationState(&self, state: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetValidationState(&self, pstate: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn IsDownloadedFromPeer(&self, pval: *mut windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IBackgroundCopyFile3_Vtbl {
    pub const fn new<Identity: IBackgroundCopyFile3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTemporaryName<Identity: IBackgroundCopyFile3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilename: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyFile3_Impl::GetTemporaryName(this, core::mem::transmute_copy(&pfilename)).into()
            }
        }
        unsafe extern "system" fn SetValidationState<Identity: IBackgroundCopyFile3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyFile3_Impl::SetValidationState(this, core::mem::transmute_copy(&state)).into()
            }
        }
        unsafe extern "system" fn GetValidationState<Identity: IBackgroundCopyFile3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyFile3_Impl::GetValidationState(this, core::mem::transmute_copy(&pstate)).into()
            }
        }
        unsafe extern "system" fn IsDownloadedFromPeer<Identity: IBackgroundCopyFile3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyFile3_Impl::IsDownloadedFromPeer(this, core::mem::transmute_copy(&pval)).into()
            }
        }
        Self {
            base__: IBackgroundCopyFile2_Vtbl::new::<Identity, OFFSET>(),
            GetTemporaryName: GetTemporaryName::<Identity, OFFSET>,
            SetValidationState: SetValidationState::<Identity, OFFSET>,
            GetValidationState: GetValidationState::<Identity, OFFSET>,
            IsDownloadedFromPeer: IsDownloadedFromPeer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyFile3 as windows_core::Interface>::IID || iid == &<IBackgroundCopyFile as windows_core::Interface>::IID || iid == &<IBackgroundCopyFile2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IBackgroundCopyFile3 {}
windows_core::imp::define_interface!(IBackgroundCopyFile4, IBackgroundCopyFile4_Vtbl, 0xef7e0655_7888_4960_b0e5_730846e03492);
impl core::ops::Deref for IBackgroundCopyFile4 {
    type Target = IBackgroundCopyFile3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyFile4, windows_core::IUnknown, IBackgroundCopyFile, IBackgroundCopyFile2, IBackgroundCopyFile3);
impl IBackgroundCopyFile4 {
    #[cfg(feature = "basetsd")]
    pub unsafe fn GetPeerDownloadStats(&self, pfromorigin: super::PUINT64, pfrompeers: super::PUINT64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPeerDownloadStats)(windows_core::Interface::as_raw(self), pfromorigin as _, pfrompeers as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyFile4_Vtbl {
    pub base__: IBackgroundCopyFile3_Vtbl,
    #[cfg(feature = "basetsd")]
    pub GetPeerDownloadStats: unsafe extern "system" fn(*mut core::ffi::c_void, super::PUINT64, super::PUINT64) -> windows_core::HRESULT,
    #[cfg(not(feature = "basetsd"))]
    GetPeerDownloadStats: usize,
}
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub trait IBackgroundCopyFile4_Impl: IBackgroundCopyFile3_Impl {
    fn GetPeerDownloadStats(&self, pfromorigin: super::PUINT64, pfrompeers: super::PUINT64) -> windows_core::Result<()>;
}
#[cfg(all(feature = "basetsd", feature = "shared"))]
impl IBackgroundCopyFile4_Vtbl {
    pub const fn new<Identity: IBackgroundCopyFile4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPeerDownloadStats<Identity: IBackgroundCopyFile4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfromorigin: super::PUINT64, pfrompeers: super::PUINT64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyFile4_Impl::GetPeerDownloadStats(this, core::mem::transmute_copy(&pfromorigin), core::mem::transmute_copy(&pfrompeers)).into()
            }
        }
        Self { base__: IBackgroundCopyFile3_Vtbl::new::<Identity, OFFSET>(), GetPeerDownloadStats: GetPeerDownloadStats::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyFile4 as windows_core::Interface>::IID || iid == &<IBackgroundCopyFile as windows_core::Interface>::IID || iid == &<IBackgroundCopyFile2 as windows_core::Interface>::IID || iid == &<IBackgroundCopyFile3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "basetsd", feature = "shared"))]
impl windows_core::RuntimeName for IBackgroundCopyFile4 {}
windows_core::imp::define_interface!(IBackgroundCopyFile5, IBackgroundCopyFile5_Vtbl, 0x85c1657f_dafc_40e8_8834_df18ea25717e);
impl core::ops::Deref for IBackgroundCopyFile5 {
    type Target = IBackgroundCopyFile4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyFile5, windows_core::IUnknown, IBackgroundCopyFile, IBackgroundCopyFile2, IBackgroundCopyFile3, IBackgroundCopyFile4);
impl IBackgroundCopyFile5 {
    pub unsafe fn SetProperty(&self, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: BITS_FILE_PROPERTY_VALUE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), propertyid, propertyvalue) }
    }
    pub unsafe fn GetProperty(&self, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: *mut BITS_FILE_PROPERTY_VALUE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), propertyid, propertyvalue as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyFile5_Vtbl {
    pub base__: IBackgroundCopyFile4_Vtbl,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, BITS_FILE_PROPERTY_ID, BITS_FILE_PROPERTY_VALUE) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, BITS_FILE_PROPERTY_ID, *mut BITS_FILE_PROPERTY_VALUE) -> windows_core::HRESULT,
}
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub trait IBackgroundCopyFile5_Impl: IBackgroundCopyFile4_Impl {
    fn SetProperty(&self, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: &BITS_FILE_PROPERTY_VALUE) -> windows_core::Result<()>;
    fn GetProperty(&self, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: *mut BITS_FILE_PROPERTY_VALUE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "basetsd", feature = "shared"))]
impl IBackgroundCopyFile5_Vtbl {
    pub const fn new<Identity: IBackgroundCopyFile5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetProperty<Identity: IBackgroundCopyFile5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: BITS_FILE_PROPERTY_VALUE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyFile5_Impl::SetProperty(this, core::mem::transmute_copy(&propertyid), core::mem::transmute(&propertyvalue)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IBackgroundCopyFile5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: *mut BITS_FILE_PROPERTY_VALUE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyFile5_Impl::GetProperty(this, core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&propertyvalue)).into()
            }
        }
        Self {
            base__: IBackgroundCopyFile4_Vtbl::new::<Identity, OFFSET>(),
            SetProperty: SetProperty::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyFile5 as windows_core::Interface>::IID || iid == &<IBackgroundCopyFile as windows_core::Interface>::IID || iid == &<IBackgroundCopyFile2 as windows_core::Interface>::IID || iid == &<IBackgroundCopyFile3 as windows_core::Interface>::IID || iid == &<IBackgroundCopyFile4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "basetsd", feature = "shared"))]
impl windows_core::RuntimeName for IBackgroundCopyFile5 {}
windows_core::imp::define_interface!(IBackgroundCopyFile6, IBackgroundCopyFile6_Vtbl, 0xcf6784f7_d677_49fd_9368_cb47aee9d1ad);
impl core::ops::Deref for IBackgroundCopyFile6 {
    type Target = IBackgroundCopyFile5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyFile6, windows_core::IUnknown, IBackgroundCopyFile, IBackgroundCopyFile2, IBackgroundCopyFile3, IBackgroundCopyFile4, IBackgroundCopyFile5);
impl IBackgroundCopyFile6 {
    #[cfg(feature = "shared")]
    pub unsafe fn UpdateDownloadPosition(&self, offset: super::UINT64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateDownloadPosition)(windows_core::Interface::as_raw(self), offset) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RequestFileRanges(&self, ranges: &[BG_FILE_RANGE]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestFileRanges)(windows_core::Interface::as_raw(self), ranges.len().try_into().unwrap(), ranges.as_ptr()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetFilledFileRanges(&self, rangecount: *mut super::DWORD, ranges: *mut *mut BG_FILE_RANGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFilledFileRanges)(windows_core::Interface::as_raw(self), rangecount as _, ranges as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyFile6_Vtbl {
    pub base__: IBackgroundCopyFile5_Vtbl,
    #[cfg(feature = "shared")]
    pub UpdateDownloadPosition: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT64) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    UpdateDownloadPosition: usize,
    #[cfg(feature = "shared")]
    pub RequestFileRanges: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const BG_FILE_RANGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RequestFileRanges: usize,
    #[cfg(feature = "shared")]
    pub GetFilledFileRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD, *mut *mut BG_FILE_RANGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetFilledFileRanges: usize,
}
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub trait IBackgroundCopyFile6_Impl: IBackgroundCopyFile5_Impl {
    fn UpdateDownloadPosition(&self, offset: super::UINT64) -> windows_core::Result<()>;
    fn RequestFileRanges(&self, rangecount: super::DWORD, ranges: *const BG_FILE_RANGE) -> windows_core::Result<()>;
    fn GetFilledFileRanges(&self, rangecount: *mut super::DWORD, ranges: *mut *mut BG_FILE_RANGE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "basetsd", feature = "shared"))]
impl IBackgroundCopyFile6_Vtbl {
    pub const fn new<Identity: IBackgroundCopyFile6_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UpdateDownloadPosition<Identity: IBackgroundCopyFile6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: super::UINT64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyFile6_Impl::UpdateDownloadPosition(this, core::mem::transmute_copy(&offset)).into()
            }
        }
        unsafe extern "system" fn RequestFileRanges<Identity: IBackgroundCopyFile6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rangecount: super::DWORD, ranges: *const BG_FILE_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyFile6_Impl::RequestFileRanges(this, core::mem::transmute_copy(&rangecount), core::mem::transmute_copy(&ranges)).into()
            }
        }
        unsafe extern "system" fn GetFilledFileRanges<Identity: IBackgroundCopyFile6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rangecount: *mut super::DWORD, ranges: *mut *mut BG_FILE_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyFile6_Impl::GetFilledFileRanges(this, core::mem::transmute_copy(&rangecount), core::mem::transmute_copy(&ranges)).into()
            }
        }
        Self {
            base__: IBackgroundCopyFile5_Vtbl::new::<Identity, OFFSET>(),
            UpdateDownloadPosition: UpdateDownloadPosition::<Identity, OFFSET>,
            RequestFileRanges: RequestFileRanges::<Identity, OFFSET>,
            GetFilledFileRanges: GetFilledFileRanges::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyFile6 as windows_core::Interface>::IID || iid == &<IBackgroundCopyFile as windows_core::Interface>::IID || iid == &<IBackgroundCopyFile2 as windows_core::Interface>::IID || iid == &<IBackgroundCopyFile3 as windows_core::Interface>::IID || iid == &<IBackgroundCopyFile4 as windows_core::Interface>::IID || iid == &<IBackgroundCopyFile5 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "basetsd", feature = "shared"))]
impl windows_core::RuntimeName for IBackgroundCopyFile6 {}
windows_core::imp::define_interface!(IBackgroundCopyJob, IBackgroundCopyJob_Vtbl, 0x37668d37_507e_4160_9316_26306d150b12);
windows_core::imp::interface_hierarchy!(IBackgroundCopyJob, windows_core::IUnknown);
impl IBackgroundCopyJob {
    #[cfg(feature = "shared")]
    pub unsafe fn AddFileSet(&self, pfileset: &[BG_FILE_INFO]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddFileSet)(windows_core::Interface::as_raw(self), pfileset.len().try_into().unwrap(), pfileset.as_ptr()) }
    }
    pub unsafe fn AddFile<P0, P1>(&self, remoteurl: P0, localname: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddFile)(windows_core::Interface::as_raw(self), remoteurl.param().abi(), localname.param().abi()) }
    }
    pub unsafe fn EnumFiles(&self, penum: *mut Option<IEnumBackgroundCopyFiles>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumFiles)(windows_core::Interface::as_raw(self), core::mem::transmute(penum)) }
    }
    pub unsafe fn Suspend(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Suspend)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Resume(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Cancel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Complete(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Complete)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetId(&self, pval: *mut windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetId)(windows_core::Interface::as_raw(self), pval as _) }
    }
    pub unsafe fn GetType(&self, pval: *mut BG_JOB_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), pval as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetProgress(&self, pval: *mut BG_JOB_PROGRESS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetProgress)(windows_core::Interface::as_raw(self), pval as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn GetTimes(&self, pval: *mut BG_JOB_TIMES) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTimes)(windows_core::Interface::as_raw(self), pval as _) }
    }
    pub unsafe fn GetState(&self, pval: *mut BG_JOB_STATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), pval as _) }
    }
    pub unsafe fn GetError(&self, pperror: *mut Option<IBackgroundCopyError>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetError)(windows_core::Interface::as_raw(self), core::mem::transmute(pperror)) }
    }
    pub unsafe fn GetOwner(&self, pval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOwner)(windows_core::Interface::as_raw(self), pval as _) }
    }
    pub unsafe fn SetDisplayName<P0>(&self, val: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayName)(windows_core::Interface::as_raw(self), val.param().abi()) }
    }
    pub unsafe fn GetDisplayName(&self, pval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayName)(windows_core::Interface::as_raw(self), pval as _) }
    }
    pub unsafe fn SetDescription<P0>(&self, val: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), val.param().abi()) }
    }
    pub unsafe fn GetDescription(&self, pval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), pval as _) }
    }
    pub unsafe fn SetPriority(&self, val: BG_JOB_PRIORITY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), val) }
    }
    pub unsafe fn GetPriority(&self, pval: *mut BG_JOB_PRIORITY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPriority)(windows_core::Interface::as_raw(self), pval as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetNotifyFlags(&self, val: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNotifyFlags)(windows_core::Interface::as_raw(self), val) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetNotifyFlags(&self, pval: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNotifyFlags)(windows_core::Interface::as_raw(self), pval as _) }
    }
    pub unsafe fn SetNotifyInterface<P0>(&self, val: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetNotifyInterface)(windows_core::Interface::as_raw(self), val.param().abi()) }
    }
    pub unsafe fn GetNotifyInterface(&self, pval: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNotifyInterface)(windows_core::Interface::as_raw(self), core::mem::transmute(pval)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetMinimumRetryDelay(&self, seconds: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMinimumRetryDelay)(windows_core::Interface::as_raw(self), seconds) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetMinimumRetryDelay(&self, seconds: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMinimumRetryDelay)(windows_core::Interface::as_raw(self), seconds as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetNoProgressTimeout(&self, seconds: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNoProgressTimeout)(windows_core::Interface::as_raw(self), seconds) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetNoProgressTimeout(&self, seconds: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNoProgressTimeout)(windows_core::Interface::as_raw(self), seconds as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetErrorCount(&self, errors: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetErrorCount)(windows_core::Interface::as_raw(self), errors as _) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn SetProxySettings(&self, proxyusage: BG_JOB_PROXY_USAGE, proxylist: Option<*const super::WCHAR>, proxybypasslist: Option<*const super::WCHAR>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProxySettings)(windows_core::Interface::as_raw(self), proxyusage, proxylist.unwrap_or(core::mem::zeroed()) as _, proxybypasslist.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetProxySettings(&self, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut windows_core::PWSTR, pproxybypasslist: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetProxySettings)(windows_core::Interface::as_raw(self), pproxyusage as _, pproxylist as _, pproxybypasslist as _) }
    }
    pub unsafe fn TakeOwnership(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TakeOwnership)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJob_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub AddFileSet: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const BG_FILE_INFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddFileSet: usize,
    pub AddFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub EnumFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Suspend: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_JOB_TYPE) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_JOB_PROGRESS) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetProgress: usize,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub GetTimes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_JOB_TIMES) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    GetTimes: usize,
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_JOB_STATE) -> windows_core::HRESULT,
    pub GetError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, BG_JOB_PRIORITY) -> windows_core::HRESULT,
    pub GetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_JOB_PRIORITY) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetNotifyFlags: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetNotifyFlags: usize,
    #[cfg(feature = "shared")]
    pub GetNotifyFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetNotifyFlags: usize,
    pub SetNotifyInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNotifyInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetMinimumRetryDelay: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetMinimumRetryDelay: usize,
    #[cfg(feature = "shared")]
    pub GetMinimumRetryDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMinimumRetryDelay: usize,
    #[cfg(feature = "shared")]
    pub SetNoProgressTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetNoProgressTimeout: usize,
    #[cfg(feature = "shared")]
    pub GetNoProgressTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetNoProgressTimeout: usize,
    #[cfg(feature = "shared")]
    pub GetErrorCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetErrorCount: usize,
    #[cfg(feature = "winnt")]
    pub SetProxySettings: unsafe extern "system" fn(*mut core::ffi::c_void, BG_JOB_PROXY_USAGE, *const super::WCHAR, *const super::WCHAR) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    SetProxySettings: usize,
    pub GetProxySettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_JOB_PROXY_USAGE, *mut windows_core::PWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub TakeOwnership: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub trait IBackgroundCopyJob_Impl: windows_core::IUnknownImpl {
    fn AddFileSet(&self, cfilecount: super::ULONG, pfileset: *const BG_FILE_INFO) -> windows_core::Result<()>;
    fn AddFile(&self, remoteurl: &windows_core::PCWSTR, localname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn EnumFiles(&self, penum: windows_core::OutRef<IEnumBackgroundCopyFiles>) -> windows_core::Result<()>;
    fn Suspend(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Complete(&self) -> windows_core::Result<()>;
    fn GetId(&self, pval: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetType(&self, pval: *mut BG_JOB_TYPE) -> windows_core::Result<()>;
    fn GetProgress(&self, pval: *mut BG_JOB_PROGRESS) -> windows_core::Result<()>;
    fn GetTimes(&self, pval: *mut BG_JOB_TIMES) -> windows_core::Result<()>;
    fn GetState(&self, pval: *mut BG_JOB_STATE) -> windows_core::Result<()>;
    fn GetError(&self, pperror: windows_core::OutRef<IBackgroundCopyError>) -> windows_core::Result<()>;
    fn GetOwner(&self, pval: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetDisplayName(&self, val: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetDisplayName(&self, pval: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetDescription(&self, val: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetDescription(&self, pval: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetPriority(&self, val: BG_JOB_PRIORITY) -> windows_core::Result<()>;
    fn GetPriority(&self, pval: *mut BG_JOB_PRIORITY) -> windows_core::Result<()>;
    fn SetNotifyFlags(&self, val: super::ULONG) -> windows_core::Result<()>;
    fn GetNotifyFlags(&self, pval: *mut super::ULONG) -> windows_core::Result<()>;
    fn SetNotifyInterface(&self, val: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetNotifyInterface(&self, pval: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetMinimumRetryDelay(&self, seconds: super::ULONG) -> windows_core::Result<()>;
    fn GetMinimumRetryDelay(&self, seconds: *mut super::ULONG) -> windows_core::Result<()>;
    fn SetNoProgressTimeout(&self, seconds: super::ULONG) -> windows_core::Result<()>;
    fn GetNoProgressTimeout(&self, seconds: *mut super::ULONG) -> windows_core::Result<()>;
    fn GetErrorCount(&self, errors: *mut super::ULONG) -> windows_core::Result<()>;
    fn SetProxySettings(&self, proxyusage: BG_JOB_PROXY_USAGE, proxylist: *const super::WCHAR, proxybypasslist: *const super::WCHAR) -> windows_core::Result<()>;
    fn GetProxySettings(&self, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut windows_core::PWSTR, pproxybypasslist: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn TakeOwnership(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl IBackgroundCopyJob_Vtbl {
    pub const fn new<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddFileSet<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cfilecount: super::ULONG, pfileset: *const BG_FILE_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::AddFileSet(this, core::mem::transmute_copy(&cfilecount), core::mem::transmute_copy(&pfileset)).into()
            }
        }
        unsafe extern "system" fn AddFile<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteurl: windows_core::PCWSTR, localname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::AddFile(this, core::mem::transmute(&remoteurl), core::mem::transmute(&localname)).into()
            }
        }
        unsafe extern "system" fn EnumFiles<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::EnumFiles(this, core::mem::transmute_copy(&penum)).into()
            }
        }
        unsafe extern "system" fn Suspend<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::Suspend(this).into()
            }
        }
        unsafe extern "system" fn Resume<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::Resume(this).into()
            }
        }
        unsafe extern "system" fn Cancel<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn Complete<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::Complete(this).into()
            }
        }
        unsafe extern "system" fn GetId<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::GetId(this, core::mem::transmute_copy(&pval)).into()
            }
        }
        unsafe extern "system" fn GetType<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut BG_JOB_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::GetType(this, core::mem::transmute_copy(&pval)).into()
            }
        }
        unsafe extern "system" fn GetProgress<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut BG_JOB_PROGRESS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::GetProgress(this, core::mem::transmute_copy(&pval)).into()
            }
        }
        unsafe extern "system" fn GetTimes<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut BG_JOB_TIMES) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::GetTimes(this, core::mem::transmute_copy(&pval)).into()
            }
        }
        unsafe extern "system" fn GetState<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut BG_JOB_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::GetState(this, core::mem::transmute_copy(&pval)).into()
            }
        }
        unsafe extern "system" fn GetError<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pperror: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::GetError(this, core::mem::transmute_copy(&pperror)).into()
            }
        }
        unsafe extern "system" fn GetOwner<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::GetOwner(this, core::mem::transmute_copy(&pval)).into()
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::SetDisplayName(this, core::mem::transmute(&val)).into()
            }
        }
        unsafe extern "system" fn GetDisplayName<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::GetDisplayName(this, core::mem::transmute_copy(&pval)).into()
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::SetDescription(this, core::mem::transmute(&val)).into()
            }
        }
        unsafe extern "system" fn GetDescription<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::GetDescription(this, core::mem::transmute_copy(&pval)).into()
            }
        }
        unsafe extern "system" fn SetPriority<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: BG_JOB_PRIORITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::SetPriority(this, core::mem::transmute_copy(&val)).into()
            }
        }
        unsafe extern "system" fn GetPriority<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut BG_JOB_PRIORITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::GetPriority(this, core::mem::transmute_copy(&pval)).into()
            }
        }
        unsafe extern "system" fn SetNotifyFlags<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::SetNotifyFlags(this, core::mem::transmute_copy(&val)).into()
            }
        }
        unsafe extern "system" fn GetNotifyFlags<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::GetNotifyFlags(this, core::mem::transmute_copy(&pval)).into()
            }
        }
        unsafe extern "system" fn SetNotifyInterface<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::SetNotifyInterface(this, core::mem::transmute_copy(&val)).into()
            }
        }
        unsafe extern "system" fn GetNotifyInterface<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::GetNotifyInterface(this, core::mem::transmute_copy(&pval)).into()
            }
        }
        unsafe extern "system" fn SetMinimumRetryDelay<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::SetMinimumRetryDelay(this, core::mem::transmute_copy(&seconds)).into()
            }
        }
        unsafe extern "system" fn GetMinimumRetryDelay<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::GetMinimumRetryDelay(this, core::mem::transmute_copy(&seconds)).into()
            }
        }
        unsafe extern "system" fn SetNoProgressTimeout<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::SetNoProgressTimeout(this, core::mem::transmute_copy(&seconds)).into()
            }
        }
        unsafe extern "system" fn GetNoProgressTimeout<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::GetNoProgressTimeout(this, core::mem::transmute_copy(&seconds)).into()
            }
        }
        unsafe extern "system" fn GetErrorCount<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, errors: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::GetErrorCount(this, core::mem::transmute_copy(&errors)).into()
            }
        }
        unsafe extern "system" fn SetProxySettings<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, proxyusage: BG_JOB_PROXY_USAGE, proxylist: *const super::WCHAR, proxybypasslist: *const super::WCHAR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::SetProxySettings(this, core::mem::transmute_copy(&proxyusage), core::mem::transmute_copy(&proxylist), core::mem::transmute_copy(&proxybypasslist)).into()
            }
        }
        unsafe extern "system" fn GetProxySettings<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproxyusage: *mut BG_JOB_PROXY_USAGE, pproxylist: *mut windows_core::PWSTR, pproxybypasslist: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::GetProxySettings(this, core::mem::transmute_copy(&pproxyusage), core::mem::transmute_copy(&pproxylist), core::mem::transmute_copy(&pproxybypasslist)).into()
            }
        }
        unsafe extern "system" fn TakeOwnership<Identity: IBackgroundCopyJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob_Impl::TakeOwnership(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddFileSet: AddFileSet::<Identity, OFFSET>,
            AddFile: AddFile::<Identity, OFFSET>,
            EnumFiles: EnumFiles::<Identity, OFFSET>,
            Suspend: Suspend::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            Complete: Complete::<Identity, OFFSET>,
            GetId: GetId::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            GetProgress: GetProgress::<Identity, OFFSET>,
            GetTimes: GetTimes::<Identity, OFFSET>,
            GetState: GetState::<Identity, OFFSET>,
            GetError: GetError::<Identity, OFFSET>,
            GetOwner: GetOwner::<Identity, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            GetPriority: GetPriority::<Identity, OFFSET>,
            SetNotifyFlags: SetNotifyFlags::<Identity, OFFSET>,
            GetNotifyFlags: GetNotifyFlags::<Identity, OFFSET>,
            SetNotifyInterface: SetNotifyInterface::<Identity, OFFSET>,
            GetNotifyInterface: GetNotifyInterface::<Identity, OFFSET>,
            SetMinimumRetryDelay: SetMinimumRetryDelay::<Identity, OFFSET>,
            GetMinimumRetryDelay: GetMinimumRetryDelay::<Identity, OFFSET>,
            SetNoProgressTimeout: SetNoProgressTimeout::<Identity, OFFSET>,
            GetNoProgressTimeout: GetNoProgressTimeout::<Identity, OFFSET>,
            GetErrorCount: GetErrorCount::<Identity, OFFSET>,
            SetProxySettings: SetProxySettings::<Identity, OFFSET>,
            GetProxySettings: GetProxySettings::<Identity, OFFSET>,
            TakeOwnership: TakeOwnership::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyJob as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IBackgroundCopyJob {}
windows_core::imp::define_interface!(IBackgroundCopyJob2, IBackgroundCopyJob2_Vtbl, 0x54b50739_686f_45eb_9dff_d6a9a0faa9af);
impl core::ops::Deref for IBackgroundCopyJob2 {
    type Target = IBackgroundCopyJob;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyJob2, windows_core::IUnknown, IBackgroundCopyJob);
impl IBackgroundCopyJob2 {
    pub unsafe fn SetNotifyCmdLine<P0, P1>(&self, program: P0, parameters: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetNotifyCmdLine)(windows_core::Interface::as_raw(self), program.param().abi(), parameters.param().abi()) }
    }
    pub unsafe fn GetNotifyCmdLine(&self, pprogram: *mut windows_core::PWSTR, pparameters: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNotifyCmdLine)(windows_core::Interface::as_raw(self), pprogram as _, pparameters as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetReplyProgress(&self, pprogress: *mut BG_JOB_REPLY_PROGRESS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetReplyProgress)(windows_core::Interface::as_raw(self), pprogress as _) }
    }
    #[cfg(all(feature = "rpc", feature = "shared"))]
    pub unsafe fn GetReplyData(&self, ppbuffer: *mut *mut super::byte, plength: Option<*mut super::UINT64>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetReplyData)(windows_core::Interface::as_raw(self), ppbuffer as _, plength.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SetReplyFileName<P0>(&self, replyfilename: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetReplyFileName)(windows_core::Interface::as_raw(self), replyfilename.param().abi()) }
    }
    pub unsafe fn GetReplyFileName(&self, preplyfilename: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetReplyFileName)(windows_core::Interface::as_raw(self), preplyfilename as _) }
    }
    pub unsafe fn SetCredentials(&self, credentials: *const BG_AUTH_CREDENTIALS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCredentials)(windows_core::Interface::as_raw(self), credentials) }
    }
    pub unsafe fn RemoveCredentials(&self, target: BG_AUTH_TARGET, scheme: BG_AUTH_SCHEME) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveCredentials)(windows_core::Interface::as_raw(self), target, scheme) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJob2_Vtbl {
    pub base__: IBackgroundCopyJob_Vtbl,
    pub SetNotifyCmdLine: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetNotifyCmdLine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetReplyProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_JOB_REPLY_PROGRESS) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetReplyProgress: usize,
    #[cfg(all(feature = "rpc", feature = "shared"))]
    pub GetReplyData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::byte, *mut super::UINT64) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "rpc", feature = "shared")))]
    GetReplyData: usize,
    pub SetReplyFileName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetReplyFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, *const BG_AUTH_CREDENTIALS) -> windows_core::HRESULT,
    pub RemoveCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, BG_AUTH_TARGET, BG_AUTH_SCHEME) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "rpc", feature = "shared", feature = "winnt"))]
pub trait IBackgroundCopyJob2_Impl: IBackgroundCopyJob_Impl {
    fn SetNotifyCmdLine(&self, program: &windows_core::PCWSTR, parameters: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetNotifyCmdLine(&self, pprogram: *mut windows_core::PWSTR, pparameters: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetReplyProgress(&self, pprogress: *mut BG_JOB_REPLY_PROGRESS) -> windows_core::Result<()>;
    fn GetReplyData(&self, ppbuffer: *mut *mut super::byte, plength: *mut super::UINT64) -> windows_core::Result<()>;
    fn SetReplyFileName(&self, replyfilename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetReplyFileName(&self, preplyfilename: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetCredentials(&self, credentials: *const BG_AUTH_CREDENTIALS) -> windows_core::Result<()>;
    fn RemoveCredentials(&self, target: BG_AUTH_TARGET, scheme: BG_AUTH_SCHEME) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "rpc", feature = "shared", feature = "winnt"))]
impl IBackgroundCopyJob2_Vtbl {
    pub const fn new<Identity: IBackgroundCopyJob2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetNotifyCmdLine<Identity: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, program: windows_core::PCWSTR, parameters: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob2_Impl::SetNotifyCmdLine(this, core::mem::transmute(&program), core::mem::transmute(&parameters)).into()
            }
        }
        unsafe extern "system" fn GetNotifyCmdLine<Identity: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprogram: *mut windows_core::PWSTR, pparameters: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob2_Impl::GetNotifyCmdLine(this, core::mem::transmute_copy(&pprogram), core::mem::transmute_copy(&pparameters)).into()
            }
        }
        unsafe extern "system" fn GetReplyProgress<Identity: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprogress: *mut BG_JOB_REPLY_PROGRESS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob2_Impl::GetReplyProgress(this, core::mem::transmute_copy(&pprogress)).into()
            }
        }
        unsafe extern "system" fn GetReplyData<Identity: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbuffer: *mut *mut super::byte, plength: *mut super::UINT64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob2_Impl::GetReplyData(this, core::mem::transmute_copy(&ppbuffer), core::mem::transmute_copy(&plength)).into()
            }
        }
        unsafe extern "system" fn SetReplyFileName<Identity: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, replyfilename: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob2_Impl::SetReplyFileName(this, core::mem::transmute(&replyfilename)).into()
            }
        }
        unsafe extern "system" fn GetReplyFileName<Identity: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preplyfilename: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob2_Impl::GetReplyFileName(this, core::mem::transmute_copy(&preplyfilename)).into()
            }
        }
        unsafe extern "system" fn SetCredentials<Identity: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, credentials: *const BG_AUTH_CREDENTIALS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob2_Impl::SetCredentials(this, core::mem::transmute_copy(&credentials)).into()
            }
        }
        unsafe extern "system" fn RemoveCredentials<Identity: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: BG_AUTH_TARGET, scheme: BG_AUTH_SCHEME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob2_Impl::RemoveCredentials(this, core::mem::transmute_copy(&target), core::mem::transmute_copy(&scheme)).into()
            }
        }
        Self {
            base__: IBackgroundCopyJob_Vtbl::new::<Identity, OFFSET>(),
            SetNotifyCmdLine: SetNotifyCmdLine::<Identity, OFFSET>,
            GetNotifyCmdLine: GetNotifyCmdLine::<Identity, OFFSET>,
            GetReplyProgress: GetReplyProgress::<Identity, OFFSET>,
            GetReplyData: GetReplyData::<Identity, OFFSET>,
            SetReplyFileName: SetReplyFileName::<Identity, OFFSET>,
            GetReplyFileName: GetReplyFileName::<Identity, OFFSET>,
            SetCredentials: SetCredentials::<Identity, OFFSET>,
            RemoveCredentials: RemoveCredentials::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyJob2 as windows_core::Interface>::IID || iid == &<IBackgroundCopyJob as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "rpc", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IBackgroundCopyJob2 {}
windows_core::imp::define_interface!(IBackgroundCopyJob3, IBackgroundCopyJob3_Vtbl, 0x443c8934_90ff_48ed_bcde_26f5c7450042);
impl core::ops::Deref for IBackgroundCopyJob3 {
    type Target = IBackgroundCopyJob2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyJob3, windows_core::IUnknown, IBackgroundCopyJob, IBackgroundCopyJob2);
impl IBackgroundCopyJob3 {
    pub unsafe fn ReplaceRemotePrefix<P0, P1>(&self, oldprefix: P0, newprefix: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReplaceRemotePrefix)(windows_core::Interface::as_raw(self), oldprefix.param().abi(), newprefix.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddFileWithRanges<P0, P1>(&self, remoteurl: P0, localname: P1, ranges: &[BG_FILE_RANGE]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddFileWithRanges)(windows_core::Interface::as_raw(self), remoteurl.param().abi(), localname.param().abi(), ranges.len().try_into().unwrap(), ranges.as_ptr()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetFileACLFlags(&self, flags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFileACLFlags)(windows_core::Interface::as_raw(self), flags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetFileACLFlags(&self, flags: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFileACLFlags)(windows_core::Interface::as_raw(self), flags as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJob3_Vtbl {
    pub base__: IBackgroundCopyJob2_Vtbl,
    pub ReplaceRemotePrefix: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub AddFileWithRanges: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, super::DWORD, *const BG_FILE_RANGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddFileWithRanges: usize,
    #[cfg(feature = "shared")]
    pub SetFileACLFlags: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetFileACLFlags: usize,
    #[cfg(feature = "shared")]
    pub GetFileACLFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetFileACLFlags: usize,
}
#[cfg(all(feature = "minwindef", feature = "rpc", feature = "shared", feature = "winnt"))]
pub trait IBackgroundCopyJob3_Impl: IBackgroundCopyJob2_Impl {
    fn ReplaceRemotePrefix(&self, oldprefix: &windows_core::PCWSTR, newprefix: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddFileWithRanges(&self, remoteurl: &windows_core::PCWSTR, localname: &windows_core::PCWSTR, rangecount: super::DWORD, ranges: *const BG_FILE_RANGE) -> windows_core::Result<()>;
    fn SetFileACLFlags(&self, flags: super::DWORD) -> windows_core::Result<()>;
    fn GetFileACLFlags(&self, flags: *mut super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "rpc", feature = "shared", feature = "winnt"))]
impl IBackgroundCopyJob3_Vtbl {
    pub const fn new<Identity: IBackgroundCopyJob3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReplaceRemotePrefix<Identity: IBackgroundCopyJob3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldprefix: windows_core::PCWSTR, newprefix: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob3_Impl::ReplaceRemotePrefix(this, core::mem::transmute(&oldprefix), core::mem::transmute(&newprefix)).into()
            }
        }
        unsafe extern "system" fn AddFileWithRanges<Identity: IBackgroundCopyJob3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteurl: windows_core::PCWSTR, localname: windows_core::PCWSTR, rangecount: super::DWORD, ranges: *const BG_FILE_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob3_Impl::AddFileWithRanges(this, core::mem::transmute(&remoteurl), core::mem::transmute(&localname), core::mem::transmute_copy(&rangecount), core::mem::transmute_copy(&ranges)).into()
            }
        }
        unsafe extern "system" fn SetFileACLFlags<Identity: IBackgroundCopyJob3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob3_Impl::SetFileACLFlags(this, core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn GetFileACLFlags<Identity: IBackgroundCopyJob3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob3_Impl::GetFileACLFlags(this, core::mem::transmute_copy(&flags)).into()
            }
        }
        Self {
            base__: IBackgroundCopyJob2_Vtbl::new::<Identity, OFFSET>(),
            ReplaceRemotePrefix: ReplaceRemotePrefix::<Identity, OFFSET>,
            AddFileWithRanges: AddFileWithRanges::<Identity, OFFSET>,
            SetFileACLFlags: SetFileACLFlags::<Identity, OFFSET>,
            GetFileACLFlags: GetFileACLFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyJob3 as windows_core::Interface>::IID || iid == &<IBackgroundCopyJob as windows_core::Interface>::IID || iid == &<IBackgroundCopyJob2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "rpc", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IBackgroundCopyJob3 {}
windows_core::imp::define_interface!(IBackgroundCopyJob4, IBackgroundCopyJob4_Vtbl, 0x659cdeae_489e_11d9_a9cd_000d56965251);
impl core::ops::Deref for IBackgroundCopyJob4 {
    type Target = IBackgroundCopyJob3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyJob4, windows_core::IUnknown, IBackgroundCopyJob, IBackgroundCopyJob2, IBackgroundCopyJob3);
impl IBackgroundCopyJob4 {
    #[cfg(feature = "shared")]
    pub unsafe fn SetPeerCachingFlags(&self, flags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPeerCachingFlags)(windows_core::Interface::as_raw(self), flags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetPeerCachingFlags(&self, pflags: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPeerCachingFlags)(windows_core::Interface::as_raw(self), pflags as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetOwnerIntegrityLevel(&self, plevel: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOwnerIntegrityLevel)(windows_core::Interface::as_raw(self), plevel as _) }
    }
    pub unsafe fn GetOwnerElevationState(&self, pelevated: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOwnerElevationState)(windows_core::Interface::as_raw(self), pelevated as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetMaximumDownloadTime(&self, timeout: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaximumDownloadTime)(windows_core::Interface::as_raw(self), timeout) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetMaximumDownloadTime(&self, ptimeout: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMaximumDownloadTime)(windows_core::Interface::as_raw(self), ptimeout as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJob4_Vtbl {
    pub base__: IBackgroundCopyJob3_Vtbl,
    #[cfg(feature = "shared")]
    pub SetPeerCachingFlags: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetPeerCachingFlags: usize,
    #[cfg(feature = "shared")]
    pub GetPeerCachingFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPeerCachingFlags: usize,
    #[cfg(feature = "shared")]
    pub GetOwnerIntegrityLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetOwnerIntegrityLevel: usize,
    pub GetOwnerElevationState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetMaximumDownloadTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetMaximumDownloadTime: usize,
    #[cfg(feature = "shared")]
    pub GetMaximumDownloadTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMaximumDownloadTime: usize,
}
#[cfg(all(feature = "minwindef", feature = "rpc", feature = "shared", feature = "winnt"))]
pub trait IBackgroundCopyJob4_Impl: IBackgroundCopyJob3_Impl {
    fn SetPeerCachingFlags(&self, flags: super::DWORD) -> windows_core::Result<()>;
    fn GetPeerCachingFlags(&self, pflags: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetOwnerIntegrityLevel(&self, plevel: *mut super::ULONG) -> windows_core::Result<()>;
    fn GetOwnerElevationState(&self, pelevated: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn SetMaximumDownloadTime(&self, timeout: super::ULONG) -> windows_core::Result<()>;
    fn GetMaximumDownloadTime(&self, ptimeout: *mut super::ULONG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "rpc", feature = "shared", feature = "winnt"))]
impl IBackgroundCopyJob4_Vtbl {
    pub const fn new<Identity: IBackgroundCopyJob4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPeerCachingFlags<Identity: IBackgroundCopyJob4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob4_Impl::SetPeerCachingFlags(this, core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn GetPeerCachingFlags<Identity: IBackgroundCopyJob4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob4_Impl::GetPeerCachingFlags(this, core::mem::transmute_copy(&pflags)).into()
            }
        }
        unsafe extern "system" fn GetOwnerIntegrityLevel<Identity: IBackgroundCopyJob4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plevel: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob4_Impl::GetOwnerIntegrityLevel(this, core::mem::transmute_copy(&plevel)).into()
            }
        }
        unsafe extern "system" fn GetOwnerElevationState<Identity: IBackgroundCopyJob4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pelevated: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob4_Impl::GetOwnerElevationState(this, core::mem::transmute_copy(&pelevated)).into()
            }
        }
        unsafe extern "system" fn SetMaximumDownloadTime<Identity: IBackgroundCopyJob4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeout: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob4_Impl::SetMaximumDownloadTime(this, core::mem::transmute_copy(&timeout)).into()
            }
        }
        unsafe extern "system" fn GetMaximumDownloadTime<Identity: IBackgroundCopyJob4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptimeout: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob4_Impl::GetMaximumDownloadTime(this, core::mem::transmute_copy(&ptimeout)).into()
            }
        }
        Self {
            base__: IBackgroundCopyJob3_Vtbl::new::<Identity, OFFSET>(),
            SetPeerCachingFlags: SetPeerCachingFlags::<Identity, OFFSET>,
            GetPeerCachingFlags: GetPeerCachingFlags::<Identity, OFFSET>,
            GetOwnerIntegrityLevel: GetOwnerIntegrityLevel::<Identity, OFFSET>,
            GetOwnerElevationState: GetOwnerElevationState::<Identity, OFFSET>,
            SetMaximumDownloadTime: SetMaximumDownloadTime::<Identity, OFFSET>,
            GetMaximumDownloadTime: GetMaximumDownloadTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyJob4 as windows_core::Interface>::IID || iid == &<IBackgroundCopyJob as windows_core::Interface>::IID || iid == &<IBackgroundCopyJob2 as windows_core::Interface>::IID || iid == &<IBackgroundCopyJob3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "rpc", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IBackgroundCopyJob4 {}
windows_core::imp::define_interface!(IBackgroundCopyJob5, IBackgroundCopyJob5_Vtbl, 0xe847030c_bbba_4657_af6d_484aa42bf1fe);
impl core::ops::Deref for IBackgroundCopyJob5 {
    type Target = IBackgroundCopyJob4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyJob5, windows_core::IUnknown, IBackgroundCopyJob, IBackgroundCopyJob2, IBackgroundCopyJob3, IBackgroundCopyJob4);
impl IBackgroundCopyJob5 {
    #[cfg(feature = "shared")]
    pub unsafe fn SetProperty(&self, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: BITS_JOB_PROPERTY_VALUE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), propertyid, propertyvalue) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetProperty(&self, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: *mut BITS_JOB_PROPERTY_VALUE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), propertyid, propertyvalue as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJob5_Vtbl {
    pub base__: IBackgroundCopyJob4_Vtbl,
    #[cfg(feature = "shared")]
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, BITS_JOB_PROPERTY_ID, BITS_JOB_PROPERTY_VALUE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetProperty: usize,
    #[cfg(feature = "shared")]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, BITS_JOB_PROPERTY_ID, *mut BITS_JOB_PROPERTY_VALUE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetProperty: usize,
}
#[cfg(all(feature = "minwindef", feature = "rpc", feature = "shared", feature = "winnt"))]
pub trait IBackgroundCopyJob5_Impl: IBackgroundCopyJob4_Impl {
    fn SetProperty(&self, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: &BITS_JOB_PROPERTY_VALUE) -> windows_core::Result<()>;
    fn GetProperty(&self, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: *mut BITS_JOB_PROPERTY_VALUE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "rpc", feature = "shared", feature = "winnt"))]
impl IBackgroundCopyJob5_Vtbl {
    pub const fn new<Identity: IBackgroundCopyJob5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetProperty<Identity: IBackgroundCopyJob5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: BITS_JOB_PROPERTY_VALUE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob5_Impl::SetProperty(this, core::mem::transmute_copy(&propertyid), core::mem::transmute(&propertyvalue)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IBackgroundCopyJob5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: *mut BITS_JOB_PROPERTY_VALUE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob5_Impl::GetProperty(this, core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&propertyvalue)).into()
            }
        }
        Self {
            base__: IBackgroundCopyJob4_Vtbl::new::<Identity, OFFSET>(),
            SetProperty: SetProperty::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyJob5 as windows_core::Interface>::IID || iid == &<IBackgroundCopyJob as windows_core::Interface>::IID || iid == &<IBackgroundCopyJob2 as windows_core::Interface>::IID || iid == &<IBackgroundCopyJob3 as windows_core::Interface>::IID || iid == &<IBackgroundCopyJob4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "rpc", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IBackgroundCopyJob5 {}
windows_core::imp::define_interface!(IBackgroundCopyJobHttpOptions, IBackgroundCopyJobHttpOptions_Vtbl, 0xf1bd1079_9f01_4bdc_8036_f09b70095066);
windows_core::imp::interface_hierarchy!(IBackgroundCopyJobHttpOptions, windows_core::IUnknown);
impl IBackgroundCopyJobHttpOptions {
    #[cfg(feature = "rpc")]
    pub unsafe fn SetClientCertificateByID<P1>(&self, storelocation: BG_CERT_STORE_LOCATION, storename: P1, pcerthashblob: &[super::byte; 20]) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetClientCertificateByID)(windows_core::Interface::as_raw(self), storelocation, storename.param().abi(), pcerthashblob.as_ptr()) }
    }
    pub unsafe fn SetClientCertificateByName<P1, P2>(&self, storelocation: BG_CERT_STORE_LOCATION, storename: P1, subjectname: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetClientCertificateByName)(windows_core::Interface::as_raw(self), storelocation, storename.param().abi(), subjectname.param().abi()) }
    }
    pub unsafe fn RemoveClientCertificate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveClientCertificate)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "rpc")]
    pub unsafe fn GetClientCertificate(&self, pstorelocation: *mut BG_CERT_STORE_LOCATION, pstorename: *mut windows_core::PWSTR, ppcerthashblob: *mut *mut super::byte, psubjectname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetClientCertificate)(windows_core::Interface::as_raw(self), pstorelocation as _, pstorename as _, ppcerthashblob as _, psubjectname as _) }
    }
    pub unsafe fn SetCustomHeaders<P0>(&self, requestheaders: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCustomHeaders)(windows_core::Interface::as_raw(self), requestheaders.param().abi()) }
    }
    pub unsafe fn GetCustomHeaders(&self, prequestheaders: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCustomHeaders)(windows_core::Interface::as_raw(self), prequestheaders as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetSecurityFlags(&self, flags: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSecurityFlags)(windows_core::Interface::as_raw(self), flags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSecurityFlags(&self, pflags: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSecurityFlags)(windows_core::Interface::as_raw(self), pflags as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJobHttpOptions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "rpc")]
    pub SetClientCertificateByID: unsafe extern "system" fn(*mut core::ffi::c_void, BG_CERT_STORE_LOCATION, windows_core::PCWSTR, *const super::byte) -> windows_core::HRESULT,
    #[cfg(not(feature = "rpc"))]
    SetClientCertificateByID: usize,
    pub SetClientCertificateByName: unsafe extern "system" fn(*mut core::ffi::c_void, BG_CERT_STORE_LOCATION, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub RemoveClientCertificate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "rpc")]
    pub GetClientCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_CERT_STORE_LOCATION, *mut windows_core::PWSTR, *mut *mut super::byte, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "rpc"))]
    GetClientCertificate: usize,
    pub SetCustomHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetCustomHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetSecurityFlags: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetSecurityFlags: usize,
    #[cfg(feature = "shared")]
    pub GetSecurityFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSecurityFlags: usize,
}
#[cfg(all(feature = "rpc", feature = "shared"))]
pub trait IBackgroundCopyJobHttpOptions_Impl: windows_core::IUnknownImpl {
    fn SetClientCertificateByID(&self, storelocation: BG_CERT_STORE_LOCATION, storename: &windows_core::PCWSTR, pcerthashblob: *const super::byte) -> windows_core::Result<()>;
    fn SetClientCertificateByName(&self, storelocation: BG_CERT_STORE_LOCATION, storename: &windows_core::PCWSTR, subjectname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RemoveClientCertificate(&self) -> windows_core::Result<()>;
    fn GetClientCertificate(&self, pstorelocation: *mut BG_CERT_STORE_LOCATION, pstorename: *mut windows_core::PWSTR, ppcerthashblob: *mut *mut super::byte, psubjectname: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetCustomHeaders(&self, requestheaders: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetCustomHeaders(&self, prequestheaders: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetSecurityFlags(&self, flags: super::ULONG) -> windows_core::Result<()>;
    fn GetSecurityFlags(&self, pflags: *mut super::ULONG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl IBackgroundCopyJobHttpOptions_Vtbl {
    pub const fn new<Identity: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetClientCertificateByID<Identity: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storelocation: BG_CERT_STORE_LOCATION, storename: windows_core::PCWSTR, pcerthashblob: *const super::byte) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJobHttpOptions_Impl::SetClientCertificateByID(this, core::mem::transmute_copy(&storelocation), core::mem::transmute(&storename), core::mem::transmute_copy(&pcerthashblob)).into()
            }
        }
        unsafe extern "system" fn SetClientCertificateByName<Identity: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storelocation: BG_CERT_STORE_LOCATION, storename: windows_core::PCWSTR, subjectname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJobHttpOptions_Impl::SetClientCertificateByName(this, core::mem::transmute_copy(&storelocation), core::mem::transmute(&storename), core::mem::transmute(&subjectname)).into()
            }
        }
        unsafe extern "system" fn RemoveClientCertificate<Identity: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJobHttpOptions_Impl::RemoveClientCertificate(this).into()
            }
        }
        unsafe extern "system" fn GetClientCertificate<Identity: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstorelocation: *mut BG_CERT_STORE_LOCATION, pstorename: *mut windows_core::PWSTR, ppcerthashblob: *mut *mut super::byte, psubjectname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJobHttpOptions_Impl::GetClientCertificate(this, core::mem::transmute_copy(&pstorelocation), core::mem::transmute_copy(&pstorename), core::mem::transmute_copy(&ppcerthashblob), core::mem::transmute_copy(&psubjectname)).into()
            }
        }
        unsafe extern "system" fn SetCustomHeaders<Identity: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestheaders: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJobHttpOptions_Impl::SetCustomHeaders(this, core::mem::transmute(&requestheaders)).into()
            }
        }
        unsafe extern "system" fn GetCustomHeaders<Identity: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prequestheaders: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJobHttpOptions_Impl::GetCustomHeaders(this, core::mem::transmute_copy(&prequestheaders)).into()
            }
        }
        unsafe extern "system" fn SetSecurityFlags<Identity: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJobHttpOptions_Impl::SetSecurityFlags(this, core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn GetSecurityFlags<Identity: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJobHttpOptions_Impl::GetSecurityFlags(this, core::mem::transmute_copy(&pflags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetClientCertificateByID: SetClientCertificateByID::<Identity, OFFSET>,
            SetClientCertificateByName: SetClientCertificateByName::<Identity, OFFSET>,
            RemoveClientCertificate: RemoveClientCertificate::<Identity, OFFSET>,
            GetClientCertificate: GetClientCertificate::<Identity, OFFSET>,
            SetCustomHeaders: SetCustomHeaders::<Identity, OFFSET>,
            GetCustomHeaders: GetCustomHeaders::<Identity, OFFSET>,
            SetSecurityFlags: SetSecurityFlags::<Identity, OFFSET>,
            GetSecurityFlags: GetSecurityFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyJobHttpOptions as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl windows_core::RuntimeName for IBackgroundCopyJobHttpOptions {}
windows_core::imp::define_interface!(IBackgroundCopyJobHttpOptions2, IBackgroundCopyJobHttpOptions2_Vtbl, 0xb591a192_a405_4fc3_8323_4c5c542578fc);
impl core::ops::Deref for IBackgroundCopyJobHttpOptions2 {
    type Target = IBackgroundCopyJobHttpOptions;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyJobHttpOptions2, windows_core::IUnknown, IBackgroundCopyJobHttpOptions);
impl IBackgroundCopyJobHttpOptions2 {
    pub unsafe fn SetHttpMethod<P0>(&self, method: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetHttpMethod)(windows_core::Interface::as_raw(self), method.param().abi()) }
    }
    pub unsafe fn GetHttpMethod(&self, method: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetHttpMethod)(windows_core::Interface::as_raw(self), method as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJobHttpOptions2_Vtbl {
    pub base__: IBackgroundCopyJobHttpOptions_Vtbl,
    pub SetHttpMethod: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetHttpMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
#[cfg(all(feature = "rpc", feature = "shared"))]
pub trait IBackgroundCopyJobHttpOptions2_Impl: IBackgroundCopyJobHttpOptions_Impl {
    fn SetHttpMethod(&self, method: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetHttpMethod(&self, method: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl IBackgroundCopyJobHttpOptions2_Vtbl {
    pub const fn new<Identity: IBackgroundCopyJobHttpOptions2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetHttpMethod<Identity: IBackgroundCopyJobHttpOptions2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, method: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJobHttpOptions2_Impl::SetHttpMethod(this, core::mem::transmute(&method)).into()
            }
        }
        unsafe extern "system" fn GetHttpMethod<Identity: IBackgroundCopyJobHttpOptions2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, method: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJobHttpOptions2_Impl::GetHttpMethod(this, core::mem::transmute_copy(&method)).into()
            }
        }
        Self {
            base__: IBackgroundCopyJobHttpOptions_Vtbl::new::<Identity, OFFSET>(),
            SetHttpMethod: SetHttpMethod::<Identity, OFFSET>,
            GetHttpMethod: GetHttpMethod::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyJobHttpOptions2 as windows_core::Interface>::IID || iid == &<IBackgroundCopyJobHttpOptions as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl windows_core::RuntimeName for IBackgroundCopyJobHttpOptions2 {}
windows_core::imp::define_interface!(IBackgroundCopyJobHttpOptions3, IBackgroundCopyJobHttpOptions3_Vtbl, 0x8a9263d3_fd4c_4eda_9b28_30132a4d4e3c);
impl core::ops::Deref for IBackgroundCopyJobHttpOptions3 {
    type Target = IBackgroundCopyJobHttpOptions2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCopyJobHttpOptions3, windows_core::IUnknown, IBackgroundCopyJobHttpOptions, IBackgroundCopyJobHttpOptions2);
impl IBackgroundCopyJobHttpOptions3 {
    pub unsafe fn SetServerCertificateValidationInterface<P0>(&self, certvalidationcallback: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetServerCertificateValidationInterface)(windows_core::Interface::as_raw(self), certvalidationcallback.param().abi()) }
    }
    pub unsafe fn MakeCustomHeadersWriteOnly(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MakeCustomHeadersWriteOnly)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJobHttpOptions3_Vtbl {
    pub base__: IBackgroundCopyJobHttpOptions2_Vtbl,
    pub SetServerCertificateValidationInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MakeCustomHeadersWriteOnly: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "rpc", feature = "shared"))]
pub trait IBackgroundCopyJobHttpOptions3_Impl: IBackgroundCopyJobHttpOptions2_Impl {
    fn SetServerCertificateValidationInterface(&self, certvalidationcallback: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn MakeCustomHeadersWriteOnly(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl IBackgroundCopyJobHttpOptions3_Vtbl {
    pub const fn new<Identity: IBackgroundCopyJobHttpOptions3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetServerCertificateValidationInterface<Identity: IBackgroundCopyJobHttpOptions3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, certvalidationcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJobHttpOptions3_Impl::SetServerCertificateValidationInterface(this, core::mem::transmute_copy(&certvalidationcallback)).into()
            }
        }
        unsafe extern "system" fn MakeCustomHeadersWriteOnly<Identity: IBackgroundCopyJobHttpOptions3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJobHttpOptions3_Impl::MakeCustomHeadersWriteOnly(this).into()
            }
        }
        Self {
            base__: IBackgroundCopyJobHttpOptions2_Vtbl::new::<Identity, OFFSET>(),
            SetServerCertificateValidationInterface: SetServerCertificateValidationInterface::<Identity, OFFSET>,
            MakeCustomHeadersWriteOnly: MakeCustomHeadersWriteOnly::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyJobHttpOptions3 as windows_core::Interface>::IID || iid == &<IBackgroundCopyJobHttpOptions as windows_core::Interface>::IID || iid == &<IBackgroundCopyJobHttpOptions2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "rpc", feature = "shared"))]
impl windows_core::RuntimeName for IBackgroundCopyJobHttpOptions3 {}
windows_core::imp::define_interface!(IBackgroundCopyManager, IBackgroundCopyManager_Vtbl, 0x5ce34c0d_0dc9_4c1f_897c_daa1b78cee7c);
windows_core::imp::interface_hierarchy!(IBackgroundCopyManager, windows_core::IUnknown);
impl IBackgroundCopyManager {
    pub unsafe fn CreateJob<P0>(&self, displayname: P0, r#type: BG_JOB_TYPE, pjobid: *mut windows_core::GUID, ppjob: *mut Option<IBackgroundCopyJob>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateJob)(windows_core::Interface::as_raw(self), displayname.param().abi(), r#type, pjobid as _, core::mem::transmute(ppjob)) }
    }
    pub unsafe fn GetJob(&self, jobid: *const windows_core::GUID, ppjob: *mut Option<IBackgroundCopyJob>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetJob)(windows_core::Interface::as_raw(self), jobid, core::mem::transmute(ppjob)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn EnumJobs(&self, dwflags: super::DWORD, ppenum: *mut Option<IEnumBackgroundCopyJobs>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumJobs)(windows_core::Interface::as_raw(self), dwflags, core::mem::transmute(ppenum)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetErrorDescription(&self, hresult: windows_core::HRESULT, languageid: super::DWORD, perrordescription: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetErrorDescription)(windows_core::Interface::as_raw(self), hresult, languageid, perrordescription as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateJob: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, BG_JOB_TYPE, *mut windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetJob: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub EnumJobs: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    EnumJobs: usize,
    #[cfg(feature = "shared")]
    pub GetErrorDescription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, super::DWORD, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetErrorDescription: usize,
}
#[cfg(feature = "shared")]
pub trait IBackgroundCopyManager_Impl: windows_core::IUnknownImpl {
    fn CreateJob(&self, displayname: &windows_core::PCWSTR, r#type: BG_JOB_TYPE, pjobid: *mut windows_core::GUID, ppjob: windows_core::OutRef<IBackgroundCopyJob>) -> windows_core::Result<()>;
    fn GetJob(&self, jobid: *const windows_core::GUID, ppjob: windows_core::OutRef<IBackgroundCopyJob>) -> windows_core::Result<()>;
    fn EnumJobs(&self, dwflags: super::DWORD, ppenum: windows_core::OutRef<IEnumBackgroundCopyJobs>) -> windows_core::Result<()>;
    fn GetErrorDescription(&self, hresult: windows_core::HRESULT, languageid: super::DWORD, perrordescription: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IBackgroundCopyManager_Vtbl {
    pub const fn new<Identity: IBackgroundCopyManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateJob<Identity: IBackgroundCopyManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayname: windows_core::PCWSTR, r#type: BG_JOB_TYPE, pjobid: *mut windows_core::GUID, ppjob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyManager_Impl::CreateJob(this, core::mem::transmute(&displayname), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pjobid), core::mem::transmute_copy(&ppjob)).into()
            }
        }
        unsafe extern "system" fn GetJob<Identity: IBackgroundCopyManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, jobid: *const windows_core::GUID, ppjob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyManager_Impl::GetJob(this, core::mem::transmute_copy(&jobid), core::mem::transmute_copy(&ppjob)).into()
            }
        }
        unsafe extern "system" fn EnumJobs<Identity: IBackgroundCopyManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyManager_Impl::EnumJobs(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&ppenum)).into()
            }
        }
        unsafe extern "system" fn GetErrorDescription<Identity: IBackgroundCopyManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresult: windows_core::HRESULT, languageid: super::DWORD, perrordescription: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyManager_Impl::GetErrorDescription(this, core::mem::transmute_copy(&hresult), core::mem::transmute_copy(&languageid), core::mem::transmute_copy(&perrordescription)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateJob: CreateJob::<Identity, OFFSET>,
            GetJob: GetJob::<Identity, OFFSET>,
            EnumJobs: EnumJobs::<Identity, OFFSET>,
            GetErrorDescription: GetErrorDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IBackgroundCopyManager {}
windows_core::imp::define_interface!(IBackgroundCopyServerCertificateValidationCallback, IBackgroundCopyServerCertificateValidationCallback_Vtbl, 0x4cec0d02_def7_4158_813a_c32a46945ff7);
windows_core::imp::interface_hierarchy!(IBackgroundCopyServerCertificateValidationCallback, windows_core::IUnknown);
impl IBackgroundCopyServerCertificateValidationCallback {
    #[cfg(feature = "shared")]
    pub unsafe fn ValidateServerCertificate<P0, P1>(&self, job: P0, file: P1, certdata: &[super::BYTE], certencodingtype: super::DWORD, certstoredata: &[super::BYTE]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IBackgroundCopyJob>,
        P1: windows_core::Param<IBackgroundCopyFile>,
    {
        unsafe { (windows_core::Interface::vtable(self).ValidateServerCertificate)(windows_core::Interface::as_raw(self), job.param().abi(), file.param().abi(), certdata.len().try_into().unwrap(), certdata.as_ptr(), certencodingtype, certstoredata.len().try_into().unwrap(), certstoredata.as_ptr()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyServerCertificateValidationCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub ValidateServerCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::DWORD, *const super::BYTE, super::DWORD, super::DWORD, *const super::BYTE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    ValidateServerCertificate: usize,
}
#[cfg(feature = "shared")]
pub trait IBackgroundCopyServerCertificateValidationCallback_Impl: windows_core::IUnknownImpl {
    fn ValidateServerCertificate(&self, job: windows_core::Ref<IBackgroundCopyJob>, file: windows_core::Ref<IBackgroundCopyFile>, certlength: super::DWORD, certdata: *const super::BYTE, certencodingtype: super::DWORD, certstorelength: super::DWORD, certstoredata: *const super::BYTE) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IBackgroundCopyServerCertificateValidationCallback_Vtbl {
    pub const fn new<Identity: IBackgroundCopyServerCertificateValidationCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ValidateServerCertificate<Identity: IBackgroundCopyServerCertificateValidationCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, job: *mut core::ffi::c_void, file: *mut core::ffi::c_void, certlength: super::DWORD, certdata: *const super::BYTE, certencodingtype: super::DWORD, certstorelength: super::DWORD, certstoredata: *const super::BYTE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyServerCertificateValidationCallback_Impl::ValidateServerCertificate(this, core::mem::transmute_copy(&job), core::mem::transmute_copy(&file), core::mem::transmute_copy(&certlength), core::mem::transmute_copy(&certdata), core::mem::transmute_copy(&certencodingtype), core::mem::transmute_copy(&certstorelength), core::mem::transmute_copy(&certstoredata)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ValidateServerCertificate: ValidateServerCertificate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyServerCertificateValidationCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IBackgroundCopyServerCertificateValidationCallback {}
windows_core::imp::define_interface!(IBitsPeer, IBitsPeer_Vtbl, 0x659cdea2_489e_11d9_a9cd_000d56965251);
windows_core::imp::interface_hierarchy!(IBitsPeer, windows_core::IUnknown);
impl IBitsPeer {
    pub unsafe fn GetPeerName(&self, pname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPeerName)(windows_core::Interface::as_raw(self), pname as _) }
    }
    pub unsafe fn IsAuthenticated(&self, pauth: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsAuthenticated)(windows_core::Interface::as_raw(self), pauth as _) }
    }
    pub unsafe fn IsAvailable(&self, ponline: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsAvailable)(windows_core::Interface::as_raw(self), ponline as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitsPeer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPeerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub IsAuthenticated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IBitsPeer_Impl: windows_core::IUnknownImpl {
    fn GetPeerName(&self, pname: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn IsAuthenticated(&self, pauth: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn IsAvailable(&self, ponline: *mut windows_core::BOOL) -> windows_core::Result<()>;
}
impl IBitsPeer_Vtbl {
    pub const fn new<Identity: IBitsPeer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPeerName<Identity: IBitsPeer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeer_Impl::GetPeerName(this, core::mem::transmute_copy(&pname)).into()
            }
        }
        unsafe extern "system" fn IsAuthenticated<Identity: IBitsPeer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pauth: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeer_Impl::IsAuthenticated(this, core::mem::transmute_copy(&pauth)).into()
            }
        }
        unsafe extern "system" fn IsAvailable<Identity: IBitsPeer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ponline: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeer_Impl::IsAvailable(this, core::mem::transmute_copy(&ponline)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPeerName: GetPeerName::<Identity, OFFSET>,
            IsAuthenticated: IsAuthenticated::<Identity, OFFSET>,
            IsAvailable: IsAvailable::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBitsPeer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBitsPeer {}
windows_core::imp::define_interface!(IBitsPeerCacheAdministration, IBitsPeerCacheAdministration_Vtbl, 0x659cdead_489e_11d9_a9cd_000d56965251);
windows_core::imp::interface_hierarchy!(IBitsPeerCacheAdministration, windows_core::IUnknown);
impl IBitsPeerCacheAdministration {
    #[cfg(feature = "shared")]
    pub unsafe fn GetMaximumCacheSize(&self, pbytes: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMaximumCacheSize)(windows_core::Interface::as_raw(self), pbytes as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetMaximumCacheSize(&self, bytes: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaximumCacheSize)(windows_core::Interface::as_raw(self), bytes) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetMaximumContentAge(&self, pseconds: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMaximumContentAge)(windows_core::Interface::as_raw(self), pseconds as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetMaximumContentAge(&self, seconds: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaximumContentAge)(windows_core::Interface::as_raw(self), seconds) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetConfigurationFlags(&self, pflags: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetConfigurationFlags)(windows_core::Interface::as_raw(self), pflags as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetConfigurationFlags(&self, flags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetConfigurationFlags)(windows_core::Interface::as_raw(self), flags) }
    }
    pub unsafe fn EnumRecords(&self, ppenum: *mut Option<IEnumBitsPeerCacheRecords>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumRecords)(windows_core::Interface::as_raw(self), core::mem::transmute(ppenum)) }
    }
    pub unsafe fn GetRecord(&self, id: *const windows_core::GUID, pprecord: *mut Option<IBitsPeerCacheRecord>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRecord)(windows_core::Interface::as_raw(self), id, core::mem::transmute(pprecord)) }
    }
    pub unsafe fn ClearRecords(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ClearRecords)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn DeleteRecord(&self, id: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteRecord)(windows_core::Interface::as_raw(self), id) }
    }
    pub unsafe fn DeleteUrl<P0>(&self, url: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteUrl)(windows_core::Interface::as_raw(self), url.param().abi()) }
    }
    pub unsafe fn EnumPeers(&self, ppenum: *mut Option<IEnumBitsPeers>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumPeers)(windows_core::Interface::as_raw(self), core::mem::transmute(ppenum)) }
    }
    pub unsafe fn ClearPeers(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ClearPeers)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn DiscoverPeers(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DiscoverPeers)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitsPeerCacheAdministration_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetMaximumCacheSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMaximumCacheSize: usize,
    #[cfg(feature = "shared")]
    pub SetMaximumCacheSize: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetMaximumCacheSize: usize,
    #[cfg(feature = "shared")]
    pub GetMaximumContentAge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMaximumContentAge: usize,
    #[cfg(feature = "shared")]
    pub SetMaximumContentAge: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetMaximumContentAge: usize,
    #[cfg(feature = "shared")]
    pub GetConfigurationFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetConfigurationFlags: usize,
    #[cfg(feature = "shared")]
    pub SetConfigurationFlags: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetConfigurationFlags: usize,
    pub EnumRecords: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRecord: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearRecords: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteRecord: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub DeleteUrl: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub EnumPeers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearPeers: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DiscoverPeers: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IBitsPeerCacheAdministration_Impl: windows_core::IUnknownImpl {
    fn GetMaximumCacheSize(&self, pbytes: *mut super::DWORD) -> windows_core::Result<()>;
    fn SetMaximumCacheSize(&self, bytes: super::DWORD) -> windows_core::Result<()>;
    fn GetMaximumContentAge(&self, pseconds: *mut super::ULONG) -> windows_core::Result<()>;
    fn SetMaximumContentAge(&self, seconds: super::ULONG) -> windows_core::Result<()>;
    fn GetConfigurationFlags(&self, pflags: *mut super::DWORD) -> windows_core::Result<()>;
    fn SetConfigurationFlags(&self, flags: super::DWORD) -> windows_core::Result<()>;
    fn EnumRecords(&self, ppenum: windows_core::OutRef<IEnumBitsPeerCacheRecords>) -> windows_core::Result<()>;
    fn GetRecord(&self, id: *const windows_core::GUID, pprecord: windows_core::OutRef<IBitsPeerCacheRecord>) -> windows_core::Result<()>;
    fn ClearRecords(&self) -> windows_core::Result<()>;
    fn DeleteRecord(&self, id: *const windows_core::GUID) -> windows_core::Result<()>;
    fn DeleteUrl(&self, url: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn EnumPeers(&self, ppenum: windows_core::OutRef<IEnumBitsPeers>) -> windows_core::Result<()>;
    fn ClearPeers(&self) -> windows_core::Result<()>;
    fn DiscoverPeers(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IBitsPeerCacheAdministration_Vtbl {
    pub const fn new<Identity: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMaximumCacheSize<Identity: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbytes: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeerCacheAdministration_Impl::GetMaximumCacheSize(this, core::mem::transmute_copy(&pbytes)).into()
            }
        }
        unsafe extern "system" fn SetMaximumCacheSize<Identity: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bytes: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeerCacheAdministration_Impl::SetMaximumCacheSize(this, core::mem::transmute_copy(&bytes)).into()
            }
        }
        unsafe extern "system" fn GetMaximumContentAge<Identity: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pseconds: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeerCacheAdministration_Impl::GetMaximumContentAge(this, core::mem::transmute_copy(&pseconds)).into()
            }
        }
        unsafe extern "system" fn SetMaximumContentAge<Identity: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeerCacheAdministration_Impl::SetMaximumContentAge(this, core::mem::transmute_copy(&seconds)).into()
            }
        }
        unsafe extern "system" fn GetConfigurationFlags<Identity: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeerCacheAdministration_Impl::GetConfigurationFlags(this, core::mem::transmute_copy(&pflags)).into()
            }
        }
        unsafe extern "system" fn SetConfigurationFlags<Identity: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeerCacheAdministration_Impl::SetConfigurationFlags(this, core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn EnumRecords<Identity: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeerCacheAdministration_Impl::EnumRecords(this, core::mem::transmute_copy(&ppenum)).into()
            }
        }
        unsafe extern "system" fn GetRecord<Identity: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *const windows_core::GUID, pprecord: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeerCacheAdministration_Impl::GetRecord(this, core::mem::transmute_copy(&id), core::mem::transmute_copy(&pprecord)).into()
            }
        }
        unsafe extern "system" fn ClearRecords<Identity: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeerCacheAdministration_Impl::ClearRecords(this).into()
            }
        }
        unsafe extern "system" fn DeleteRecord<Identity: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeerCacheAdministration_Impl::DeleteRecord(this, core::mem::transmute_copy(&id)).into()
            }
        }
        unsafe extern "system" fn DeleteUrl<Identity: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, url: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeerCacheAdministration_Impl::DeleteUrl(this, core::mem::transmute(&url)).into()
            }
        }
        unsafe extern "system" fn EnumPeers<Identity: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeerCacheAdministration_Impl::EnumPeers(this, core::mem::transmute_copy(&ppenum)).into()
            }
        }
        unsafe extern "system" fn ClearPeers<Identity: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeerCacheAdministration_Impl::ClearPeers(this).into()
            }
        }
        unsafe extern "system" fn DiscoverPeers<Identity: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeerCacheAdministration_Impl::DiscoverPeers(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMaximumCacheSize: GetMaximumCacheSize::<Identity, OFFSET>,
            SetMaximumCacheSize: SetMaximumCacheSize::<Identity, OFFSET>,
            GetMaximumContentAge: GetMaximumContentAge::<Identity, OFFSET>,
            SetMaximumContentAge: SetMaximumContentAge::<Identity, OFFSET>,
            GetConfigurationFlags: GetConfigurationFlags::<Identity, OFFSET>,
            SetConfigurationFlags: SetConfigurationFlags::<Identity, OFFSET>,
            EnumRecords: EnumRecords::<Identity, OFFSET>,
            GetRecord: GetRecord::<Identity, OFFSET>,
            ClearRecords: ClearRecords::<Identity, OFFSET>,
            DeleteRecord: DeleteRecord::<Identity, OFFSET>,
            DeleteUrl: DeleteUrl::<Identity, OFFSET>,
            EnumPeers: EnumPeers::<Identity, OFFSET>,
            ClearPeers: ClearPeers::<Identity, OFFSET>,
            DiscoverPeers: DiscoverPeers::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBitsPeerCacheAdministration as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IBitsPeerCacheAdministration {}
windows_core::imp::define_interface!(IBitsPeerCacheRecord, IBitsPeerCacheRecord_Vtbl, 0x659cdeaf_489e_11d9_a9cd_000d56965251);
windows_core::imp::interface_hierarchy!(IBitsPeerCacheRecord, windows_core::IUnknown);
impl IBitsPeerCacheRecord {
    pub unsafe fn GetId(&self, pval: *mut windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetId)(windows_core::Interface::as_raw(self), pval as _) }
    }
    pub unsafe fn GetOriginUrl(&self, pval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOriginUrl)(windows_core::Interface::as_raw(self), pval as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetFileSize(&self, pval: *mut super::UINT64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFileSize)(windows_core::Interface::as_raw(self), pval as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn GetFileModificationTime(&self, pval: *mut super::FILETIME) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFileModificationTime)(windows_core::Interface::as_raw(self), pval as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn GetLastAccessTime(&self, pval: *mut super::FILETIME) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLastAccessTime)(windows_core::Interface::as_raw(self), pval as _) }
    }
    pub unsafe fn IsFileValidated(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsFileValidated)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetFileRanges(&self, prangecount: *mut super::DWORD, ppranges: *mut *mut BG_FILE_RANGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFileRanges)(windows_core::Interface::as_raw(self), prangecount as _, ppranges as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitsPeerCacheRecord_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetOriginUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetFileSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT64) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetFileSize: usize,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub GetFileModificationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::FILETIME) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    GetFileModificationTime: usize,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub GetLastAccessTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::FILETIME) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    GetLastAccessTime: usize,
    pub IsFileValidated: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetFileRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD, *mut *mut BG_FILE_RANGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetFileRanges: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub trait IBitsPeerCacheRecord_Impl: windows_core::IUnknownImpl {
    fn GetId(&self, pval: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetOriginUrl(&self, pval: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetFileSize(&self, pval: *mut super::UINT64) -> windows_core::Result<()>;
    fn GetFileModificationTime(&self, pval: *mut super::FILETIME) -> windows_core::Result<()>;
    fn GetLastAccessTime(&self, pval: *mut super::FILETIME) -> windows_core::Result<()>;
    fn IsFileValidated(&self) -> windows_core::Result<()>;
    fn GetFileRanges(&self, prangecount: *mut super::DWORD, ppranges: *mut *mut BG_FILE_RANGE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl IBitsPeerCacheRecord_Vtbl {
    pub const fn new<Identity: IBitsPeerCacheRecord_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetId<Identity: IBitsPeerCacheRecord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeerCacheRecord_Impl::GetId(this, core::mem::transmute_copy(&pval)).into()
            }
        }
        unsafe extern "system" fn GetOriginUrl<Identity: IBitsPeerCacheRecord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeerCacheRecord_Impl::GetOriginUrl(this, core::mem::transmute_copy(&pval)).into()
            }
        }
        unsafe extern "system" fn GetFileSize<Identity: IBitsPeerCacheRecord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::UINT64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeerCacheRecord_Impl::GetFileSize(this, core::mem::transmute_copy(&pval)).into()
            }
        }
        unsafe extern "system" fn GetFileModificationTime<Identity: IBitsPeerCacheRecord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeerCacheRecord_Impl::GetFileModificationTime(this, core::mem::transmute_copy(&pval)).into()
            }
        }
        unsafe extern "system" fn GetLastAccessTime<Identity: IBitsPeerCacheRecord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeerCacheRecord_Impl::GetLastAccessTime(this, core::mem::transmute_copy(&pval)).into()
            }
        }
        unsafe extern "system" fn IsFileValidated<Identity: IBitsPeerCacheRecord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeerCacheRecord_Impl::IsFileValidated(this).into()
            }
        }
        unsafe extern "system" fn GetFileRanges<Identity: IBitsPeerCacheRecord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prangecount: *mut super::DWORD, ppranges: *mut *mut BG_FILE_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeerCacheRecord_Impl::GetFileRanges(this, core::mem::transmute_copy(&prangecount), core::mem::transmute_copy(&ppranges)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetId: GetId::<Identity, OFFSET>,
            GetOriginUrl: GetOriginUrl::<Identity, OFFSET>,
            GetFileSize: GetFileSize::<Identity, OFFSET>,
            GetFileModificationTime: GetFileModificationTime::<Identity, OFFSET>,
            GetLastAccessTime: GetLastAccessTime::<Identity, OFFSET>,
            IsFileValidated: IsFileValidated::<Identity, OFFSET>,
            GetFileRanges: GetFileRanges::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBitsPeerCacheRecord as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl windows_core::RuntimeName for IBitsPeerCacheRecord {}
windows_core::imp::define_interface!(IBitsTokenOptions, IBitsTokenOptions_Vtbl, 0x9a2584c3_f7d2_457a_9a5e_22b67bffc7d2);
windows_core::imp::interface_hierarchy!(IBitsTokenOptions, windows_core::IUnknown);
impl IBitsTokenOptions {
    #[cfg(feature = "shared")]
    pub unsafe fn SetHelperTokenFlags(&self, usageflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHelperTokenFlags)(windows_core::Interface::as_raw(self), usageflags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetHelperTokenFlags(&self, pflags: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetHelperTokenFlags)(windows_core::Interface::as_raw(self), pflags as _) }
    }
    pub unsafe fn SetHelperToken(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHelperToken)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ClearHelperToken(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ClearHelperToken)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetHelperTokenSid(&self, psid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetHelperTokenSid)(windows_core::Interface::as_raw(self), psid as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitsTokenOptions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub SetHelperTokenFlags: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetHelperTokenFlags: usize,
    #[cfg(feature = "shared")]
    pub GetHelperTokenFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetHelperTokenFlags: usize,
    pub SetHelperToken: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearHelperToken: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHelperTokenSid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IBitsTokenOptions_Impl: windows_core::IUnknownImpl {
    fn SetHelperTokenFlags(&self, usageflags: super::DWORD) -> windows_core::Result<()>;
    fn GetHelperTokenFlags(&self, pflags: *mut super::DWORD) -> windows_core::Result<()>;
    fn SetHelperToken(&self) -> windows_core::Result<()>;
    fn ClearHelperToken(&self) -> windows_core::Result<()>;
    fn GetHelperTokenSid(&self, psid: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IBitsTokenOptions_Vtbl {
    pub const fn new<Identity: IBitsTokenOptions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetHelperTokenFlags<Identity: IBitsTokenOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usageflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsTokenOptions_Impl::SetHelperTokenFlags(this, core::mem::transmute_copy(&usageflags)).into()
            }
        }
        unsafe extern "system" fn GetHelperTokenFlags<Identity: IBitsTokenOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsTokenOptions_Impl::GetHelperTokenFlags(this, core::mem::transmute_copy(&pflags)).into()
            }
        }
        unsafe extern "system" fn SetHelperToken<Identity: IBitsTokenOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsTokenOptions_Impl::SetHelperToken(this).into()
            }
        }
        unsafe extern "system" fn ClearHelperToken<Identity: IBitsTokenOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsTokenOptions_Impl::ClearHelperToken(this).into()
            }
        }
        unsafe extern "system" fn GetHelperTokenSid<Identity: IBitsTokenOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsTokenOptions_Impl::GetHelperTokenSid(this, core::mem::transmute_copy(&psid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetHelperTokenFlags: SetHelperTokenFlags::<Identity, OFFSET>,
            GetHelperTokenFlags: GetHelperTokenFlags::<Identity, OFFSET>,
            SetHelperToken: SetHelperToken::<Identity, OFFSET>,
            ClearHelperToken: ClearHelperToken::<Identity, OFFSET>,
            GetHelperTokenSid: GetHelperTokenSid::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBitsTokenOptions as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IBitsTokenOptions {}
windows_core::imp::define_interface!(IEnumBackgroundCopyFiles, IEnumBackgroundCopyFiles_Vtbl, 0xca51e165_c365_424c_8d41_24aaa4ff3c40);
windows_core::imp::interface_hierarchy!(IEnumBackgroundCopyFiles, windows_core::IUnknown);
impl IEnumBackgroundCopyFiles {
    #[cfg(feature = "shared")]
    pub unsafe fn Next(&self, celt: super::ULONG, rgelt: *mut Option<IBackgroundCopyFile>, pceltfetched: Option<*mut super::ULONG>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(rgelt), pceltfetched.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Skip(&self, celt: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self, ppenum: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), core::mem::transmute(ppenum)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCount(&self, pucount: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), pucount as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumBackgroundCopyFiles_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut *mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Next: usize,
    #[cfg(feature = "shared")]
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Skip: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCount: usize,
}
#[cfg(feature = "shared")]
pub trait IEnumBackgroundCopyFiles_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: super::ULONG, rgelt: *mut Option<IBackgroundCopyFile>, pceltfetched: *mut super::ULONG) -> windows_core::Result<()>;
    fn Skip(&self, celt: super::ULONG) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self, ppenum: windows_core::OutRef<IEnumBackgroundCopyFiles>) -> windows_core::Result<()>;
    fn GetCount(&self, pucount: *mut super::ULONG) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IEnumBackgroundCopyFiles_Vtbl {
    pub const fn new<Identity: IEnumBackgroundCopyFiles_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumBackgroundCopyFiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBackgroundCopyFiles_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumBackgroundCopyFiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBackgroundCopyFiles_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumBackgroundCopyFiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBackgroundCopyFiles_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumBackgroundCopyFiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBackgroundCopyFiles_Impl::Clone(this, core::mem::transmute_copy(&ppenum)).into()
            }
        }
        unsafe extern "system" fn GetCount<Identity: IEnumBackgroundCopyFiles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBackgroundCopyFiles_Impl::GetCount(this, core::mem::transmute_copy(&pucount)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumBackgroundCopyFiles as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IEnumBackgroundCopyFiles {}
windows_core::imp::define_interface!(IEnumBackgroundCopyJobs, IEnumBackgroundCopyJobs_Vtbl, 0x1af4f612_3b71_466f_8f58_7b6f73ac57ad);
windows_core::imp::interface_hierarchy!(IEnumBackgroundCopyJobs, windows_core::IUnknown);
impl IEnumBackgroundCopyJobs {
    #[cfg(feature = "shared")]
    pub unsafe fn Next(&self, celt: super::ULONG, rgelt: *mut Option<IBackgroundCopyJob>, pceltfetched: Option<*mut super::ULONG>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(rgelt), pceltfetched.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Skip(&self, celt: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self, ppenum: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), core::mem::transmute(ppenum)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCount(&self, pucount: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), pucount as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumBackgroundCopyJobs_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut *mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Next: usize,
    #[cfg(feature = "shared")]
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Skip: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCount: usize,
}
#[cfg(feature = "shared")]
pub trait IEnumBackgroundCopyJobs_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: super::ULONG, rgelt: *mut Option<IBackgroundCopyJob>, pceltfetched: *mut super::ULONG) -> windows_core::Result<()>;
    fn Skip(&self, celt: super::ULONG) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self, ppenum: windows_core::OutRef<IEnumBackgroundCopyJobs>) -> windows_core::Result<()>;
    fn GetCount(&self, pucount: *mut super::ULONG) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IEnumBackgroundCopyJobs_Vtbl {
    pub const fn new<Identity: IEnumBackgroundCopyJobs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumBackgroundCopyJobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBackgroundCopyJobs_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumBackgroundCopyJobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBackgroundCopyJobs_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumBackgroundCopyJobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBackgroundCopyJobs_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumBackgroundCopyJobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBackgroundCopyJobs_Impl::Clone(this, core::mem::transmute_copy(&ppenum)).into()
            }
        }
        unsafe extern "system" fn GetCount<Identity: IEnumBackgroundCopyJobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBackgroundCopyJobs_Impl::GetCount(this, core::mem::transmute_copy(&pucount)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumBackgroundCopyJobs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IEnumBackgroundCopyJobs {}
windows_core::imp::define_interface!(IEnumBitsPeerCacheRecords, IEnumBitsPeerCacheRecords_Vtbl, 0x659cdea4_489e_11d9_a9cd_000d56965251);
windows_core::imp::interface_hierarchy!(IEnumBitsPeerCacheRecords, windows_core::IUnknown);
impl IEnumBitsPeerCacheRecords {
    #[cfg(feature = "shared")]
    pub unsafe fn Next(&self, celt: super::ULONG, rgelt: *mut Option<IBitsPeerCacheRecord>, pceltfetched: Option<*mut super::ULONG>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(rgelt), pceltfetched.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Skip(&self, celt: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self, ppenum: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), core::mem::transmute(ppenum)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCount(&self, pucount: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), pucount as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumBitsPeerCacheRecords_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut *mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Next: usize,
    #[cfg(feature = "shared")]
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Skip: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCount: usize,
}
#[cfg(feature = "shared")]
pub trait IEnumBitsPeerCacheRecords_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: super::ULONG, rgelt: *mut Option<IBitsPeerCacheRecord>, pceltfetched: *mut super::ULONG) -> windows_core::Result<()>;
    fn Skip(&self, celt: super::ULONG) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self, ppenum: windows_core::OutRef<IEnumBitsPeerCacheRecords>) -> windows_core::Result<()>;
    fn GetCount(&self, pucount: *mut super::ULONG) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IEnumBitsPeerCacheRecords_Vtbl {
    pub const fn new<Identity: IEnumBitsPeerCacheRecords_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumBitsPeerCacheRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBitsPeerCacheRecords_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumBitsPeerCacheRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBitsPeerCacheRecords_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumBitsPeerCacheRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBitsPeerCacheRecords_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumBitsPeerCacheRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBitsPeerCacheRecords_Impl::Clone(this, core::mem::transmute_copy(&ppenum)).into()
            }
        }
        unsafe extern "system" fn GetCount<Identity: IEnumBitsPeerCacheRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBitsPeerCacheRecords_Impl::GetCount(this, core::mem::transmute_copy(&pucount)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumBitsPeerCacheRecords as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IEnumBitsPeerCacheRecords {}
windows_core::imp::define_interface!(IEnumBitsPeers, IEnumBitsPeers_Vtbl, 0x659cdea5_489e_11d9_a9cd_000d56965251);
windows_core::imp::interface_hierarchy!(IEnumBitsPeers, windows_core::IUnknown);
impl IEnumBitsPeers {
    #[cfg(feature = "shared")]
    pub unsafe fn Next(&self, celt: super::ULONG, rgelt: *mut Option<IBitsPeer>, pceltfetched: Option<*mut super::ULONG>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(rgelt), pceltfetched.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Skip(&self, celt: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self, ppenum: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), core::mem::transmute(ppenum)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCount(&self, pucount: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), pucount as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumBitsPeers_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut *mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Next: usize,
    #[cfg(feature = "shared")]
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Skip: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCount: usize,
}
#[cfg(feature = "shared")]
pub trait IEnumBitsPeers_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: super::ULONG, rgelt: *mut Option<IBitsPeer>, pceltfetched: *mut super::ULONG) -> windows_core::Result<()>;
    fn Skip(&self, celt: super::ULONG) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self, ppenum: windows_core::OutRef<IEnumBitsPeers>) -> windows_core::Result<()>;
    fn GetCount(&self, pucount: *mut super::ULONG) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IEnumBitsPeers_Vtbl {
    pub const fn new<Identity: IEnumBitsPeers_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumBitsPeers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBitsPeers_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumBitsPeers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBitsPeers_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumBitsPeers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBitsPeers_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumBitsPeers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBitsPeers_Impl::Clone(this, core::mem::transmute_copy(&ppenum)).into()
            }
        }
        unsafe extern "system" fn GetCount<Identity: IEnumBitsPeers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBitsPeers_Impl::GetCount(this, core::mem::transmute_copy(&pucount)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumBitsPeers as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IEnumBitsPeers {}
pub type PBG_AUTH_CREDENTIALS = *mut BG_AUTH_CREDENTIALS;
pub type PBG_BASIC_CREDENTIALS = *mut BG_BASIC_CREDENTIALS;
