#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct AsyncIBackgroundCopyCallback(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
pub struct BG_AUTH_CREDENTIALS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct BG_AUTH_CREDENTIALS_UNION(i32);
pub struct BG_AUTH_SCHEME(i32);
pub struct BG_AUTH_TARGET(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct BG_BASIC_CREDENTIALS(i32);
pub struct BG_CERT_STORE_LOCATION(i32);
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_COPY_FILE_ALL: u32 = 15u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_COPY_FILE_DACL: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_COPY_FILE_GROUP: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_COPY_FILE_OWNER: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_COPY_FILE_SACL: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_DISABLE_BRANCH_CACHE: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_ENABLE_PEERCACHING_CLIENT: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_ENABLE_PEERCACHING_SERVER: u32 = 2u32;
pub struct BG_ERROR_CONTEXT(i32);
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_APP_PACKAGE_NOT_FOUND: i32 = -2145386390i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_APP_PACKAGE_SCENARIO_NOT_SUPPORTED: i32 = -2145386389i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_BLOCKED_BY_BACKGROUND_ACCESS_POLICY: i32 = -2145386386i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_BLOCKED_BY_BATTERY_POLICY: i32 = -2145386393i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_BLOCKED_BY_BATTERY_SAVER: i32 = -2145386392i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_BLOCKED_BY_COST_TRANSFER_POLICY: i32 = -2145386407i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_BLOCKED_BY_GAME_MODE: i32 = -2145386385i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_BLOCKED_BY_POLICY: i32 = -2145386434i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_BLOCKED_BY_SYSTEM_POLICY: i32 = -2145386384i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_BUSYCACHERECORD: i32 = -2145386424i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_CLIENT_SERVER_PROTOCOL_MISMATCH: i32 = -2145386462i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_COMMIT_IN_PROGRESS: i32 = -2145386429i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_CONNECTION_CLOSED: i32 = -2145386450i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_CONNECT_FAILURE: i32 = -2145386451i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_DATABASE_CORRUPT: i32 = -2145386388i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_DESTINATION_LOCKED: i32 = -2145386483i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_DISCOVERY_IN_PROGRESS: i32 = -2145386428i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_EMPTY: i32 = -2145386493i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_ERROR_CONTEXT_GENERAL_QUEUE_MANAGER: i32 = -2145386488i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_ERROR_CONTEXT_GENERAL_TRANSPORT: i32 = -2145386485i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_ERROR_CONTEXT_LOCAL_FILE: i32 = -2145386487i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_ERROR_CONTEXT_QUEUE_MANAGER_NOTIFICATION: i32 = -2145386484i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_ERROR_CONTEXT_REMOTE_APPLICATION: i32 = -2145386466i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_ERROR_CONTEXT_REMOTE_FILE: i32 = -2145386486i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_ERROR_CONTEXT_SERVER_CERTIFICATE_CALLBACK: i32 = -2145386378i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_ERROR_CONTEXT_UNKNOWN: i32 = -2145386489i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_ERROR_INFORMATION_UNAVAILABLE: i32 = -2145386481i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_FILE_NOT_AVAILABLE: i32 = -2145386492i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_FILE_NOT_FOUND: i32 = -2145386455i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_100: i32 = -2145845148i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_101: i32 = -2145845147i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_200: i32 = -2145845048i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_201: i32 = -2145845047i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_202: i32 = -2145845046i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_203: i32 = -2145845045i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_204: i32 = -2145845044i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_205: i32 = -2145845043i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_206: i32 = -2145845042i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_300: i32 = -2145844948i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_301: i32 = -2145844947i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_302: i32 = -2145844946i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_303: i32 = -2145844945i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_304: i32 = -2145844944i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_305: i32 = -2145844943i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_307: i32 = -2145844941i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_400: i32 = -2145844848i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_401: i32 = -2145844847i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_402: i32 = -2145844846i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_403: i32 = -2145844845i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_404: i32 = -2145844844i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_405: i32 = -2145844843i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_406: i32 = -2145844842i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_407: i32 = -2145844841i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_408: i32 = -2145844840i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_409: i32 = -2145844839i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_410: i32 = -2145844838i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_411: i32 = -2145844837i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_412: i32 = -2145844836i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_413: i32 = -2145844835i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_414: i32 = -2145844834i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_415: i32 = -2145844833i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_416: i32 = -2145844832i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_417: i32 = -2145844831i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_449: i32 = -2145844799i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_500: i32 = -2145844748i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_501: i32 = -2145844747i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_502: i32 = -2145844746i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_503: i32 = -2145844745i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_504: i32 = -2145844744i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_HTTP_ERROR_505: i32 = -2145844743i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_INSUFFICIENT_HTTP_SUPPORT: i32 = -2145386478i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_INSUFFICIENT_RANGE_SUPPORT: i32 = -2145386477i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_INVALID_AUTH_SCHEME: i32 = -2145386456i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_INVALID_AUTH_TARGET: i32 = -2145386457i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_INVALID_CREDENTIALS: i32 = -2145386432i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_INVALID_HASH_ALGORITHM: i32 = -2145386431i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_INVALID_PROXY_INFO: i32 = -2145386433i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_INVALID_RANGE: i32 = -2145386453i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_INVALID_SERVER_RESPONSE: i32 = -2145386469i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_INVALID_STATE: i32 = -2145386494i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_LOCAL_FILE_CHANGED: i32 = -2145386467i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_MAXDOWNLOAD_TIMEOUT: i32 = -2145386412i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_MAX_DOWNLOAD_SIZE_INVALID_VALUE: i32 = -2145386397i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_MAX_DOWNLOAD_SIZE_LIMIT_REACHED: i32 = -2145386396i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_MISSING_FILE_SIZE: i32 = -2145386479i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_NETWORK_DISCONNECTED: i32 = -2145386480i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_NEW_OWNER_DIFF_MAPPING: i32 = -2145386475i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_NEW_OWNER_NO_FILE_ACCESS: i32 = -2145386474i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_NOT_FOUND: i32 = -2145386495i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_NOT_SUPPORTED_WITH_CUSTOM_HTTP_METHOD: i32 = -2145386383i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_NO_PROGRESS: i32 = -2145386460i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_OVERLAPPING_RANGES: i32 = -2145386452i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_PASSWORD_TOO_LARGE: i32 = -2145386458i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_PEERCACHING_DISABLED: i32 = -2145386425i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_PROPERTY_SUPPORTED_FOR_DOWNLOAD_JOBS_ONLY: i32 = -2145386400i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_PROTOCOL_NOT_AVAILABLE: i32 = -2145386491i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_PROXY_BYPASS_LIST_TOO_LARGE: i32 = -2145386471i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_PROXY_LIST_TOO_LARGE: i32 = -2145386472i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_RANDOM_ACCESS_NOT_SUPPORTED: i32 = -2145386387i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_READ_ONLY_PROPERTY: i32 = -2145386408i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_READ_ONLY_PROPERTY_AFTER_ADDFILE: i32 = -2145386399i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_READ_ONLY_PROPERTY_AFTER_RESUME: i32 = -2145386398i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_READ_ONLY_WHEN_JOB_ACTIVE: i32 = -2145386379i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_RECORD_DELETED: i32 = -2145386430i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_REMOTE_FILE_CHANGED: i32 = -2145386381i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_REMOTE_NOT_SUPPORTED: i32 = -2145386476i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_SERVER_CERT_VALIDATION_INTERFACE_REQUIRED: i32 = -2145386380i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_SERVER_EXECUTE_ENABLE: i32 = -2145386461i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_SESSION_NOT_FOUND: i32 = -2145386465i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_STANDBY_MODE: i32 = -2145386395i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_STRING_TOO_LONG: i32 = -2145386463i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_TEST_OPTION_BLOCKED_DOWNLOAD: i32 = -2145386426i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_TOKEN_REQUIRED: i32 = -2145386410i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_TOO_LARGE: i32 = -2145386464i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_TOO_MANY_FILES: i32 = -2145386468i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_TOO_MANY_FILES_IN_JOB: i32 = -2145386415i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_TOO_MANY_JOBS_PER_MACHINE: i32 = -2145386416i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_TOO_MANY_JOBS_PER_USER: i32 = -2145386423i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_TOO_MANY_RANGES_IN_FILE: i32 = -2145386414i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_UNKNOWN_PROPERTY_ID: i32 = -2145386409i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_UNSUPPORTED_JOB_CONFIGURATION: i32 = -2145386382i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_UPNP_ERROR: i32 = -2145386427i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_USERNAME_TOO_LARGE: i32 = -2145386459i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_USE_STORED_CREDENTIALS_NOT_SUPPORTED: i32 = -2145386394i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_VALIDATION_FAILED: i32 = -2145386413i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_VOLUME_CHANGED: i32 = -2145386482i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_E_WATCHDOG_TIMEOUT: i32 = -2145386391i32;
#[cfg(feature = "Win32_Foundation")]
pub struct BG_FILE_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct BG_FILE_PROGRESS(i32);
pub struct BG_FILE_RANGE(i32);
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_HTTP_REDIRECT_POLICY_ALLOW_HTTPS_TO_HTTP: u32 = 2048u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_HTTP_REDIRECT_POLICY_ALLOW_REPORT: u32 = 256u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_HTTP_REDIRECT_POLICY_ALLOW_SILENT: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_HTTP_REDIRECT_POLICY_DISALLOW: u32 = 512u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_HTTP_REDIRECT_POLICY_MASK: u32 = 1792u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_JOB_DISABLE_BRANCH_CACHE: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_JOB_ENABLE_PEERCACHING_CLIENT: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_JOB_ENABLE_PEERCACHING_SERVER: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_JOB_ENUM_ALL_USERS: u32 = 1u32;
pub struct BG_JOB_PRIORITY(i32);
pub struct BG_JOB_PROGRESS(i32);
pub struct BG_JOB_PROXY_USAGE(i32);
pub struct BG_JOB_REPLY_PROGRESS(i32);
pub struct BG_JOB_STATE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct BG_JOB_TIMES(i32);
pub struct BG_JOB_TYPE(i32);
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_NOTIFY_DISABLE: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_NOTIFY_FILE_RANGES_TRANSFERRED: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_NOTIFY_FILE_TRANSFERRED: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_NOTIFY_JOB_ERROR: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_NOTIFY_JOB_MODIFICATION: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_NOTIFY_JOB_TRANSFERRED: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_SSL_ENABLE_CRL_CHECK: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_SSL_IGNORE_CERT_CN_INVALID: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_SSL_IGNORE_CERT_DATE_INVALID: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_SSL_IGNORE_CERT_WRONG_USAGE: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_SSL_IGNORE_UNKNOWN_CA: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_S_ERROR_CONTEXT_NONE: i32 = 2097158i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_S_OVERRIDDEN_BY_POLICY: i32 = 2097237i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_S_PARTIAL_COMPLETE: i32 = 2097175i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_S_PROXY_CHANGED: i32 = 2097194i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BG_S_UNABLE_TO_DELETE_FILES: i32 = 2097178i32;
pub struct BG_TOKEN(i32);
pub struct BITSExtensionSetupFactory(i32);
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BITS_COST_OPTION_IGNORE_CONGESTION: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BITS_COST_STATE_BELOW_CAP: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BITS_COST_STATE_CAPPED_USAGE_UNKNOWN: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BITS_COST_STATE_NEAR_CAP: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BITS_COST_STATE_OVERCAP_CHARGED: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BITS_COST_STATE_OVERCAP_THROTTLED: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BITS_COST_STATE_RESERVED: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BITS_COST_STATE_ROAMING: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BITS_COST_STATE_UNRESTRICTED: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BITS_COST_STATE_USAGE_BASED: u32 = 64u32;
pub struct BITS_FILE_PROPERTY_ID(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct BITS_FILE_PROPERTY_VALUE(i32);
pub struct BITS_JOB_PROPERTY_ID(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct BITS_JOB_PROPERTY_VALUE(i32);
pub struct BITS_JOB_TRANSFER_POLICY(i32);
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BITS_MC_FAILED_TO_START: i32 = -2145828856i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BITS_MC_FATAL_IGD_ERROR: i32 = -2145828855i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BITS_MC_FILE_DELETION_FAILED: i32 = -2145828863i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BITS_MC_FILE_DELETION_FAILED_MORE: i32 = -2145828862i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BITS_MC_JOB_CANCELLED: i32 = -2145828864i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BITS_MC_JOB_NOTIFICATION_FAILURE: i32 = -2145828858i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BITS_MC_JOB_PROPERTY_CHANGE: i32 = -2145828861i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BITS_MC_JOB_SCAVENGED: i32 = -2145828859i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BITS_MC_JOB_TAKE_OWNERSHIP: i32 = -2145828860i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BITS_MC_PEERCACHING_PORT: i32 = -2145828854i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BITS_MC_STATE_FILE_CORRUPT: i32 = -2145828857i32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const BITS_MC_WSD_PORT: i32 = -2145828853i32;
pub struct BackgroundCopyManager(i32);
pub struct BackgroundCopyManager10_1(i32);
pub struct BackgroundCopyManager10_2(i32);
pub struct BackgroundCopyManager10_3(i32);
pub struct BackgroundCopyManager1_5(i32);
pub struct BackgroundCopyManager2_0(i32);
pub struct BackgroundCopyManager2_5(i32);
pub struct BackgroundCopyManager3_0(i32);
pub struct BackgroundCopyManager4_0(i32);
pub struct BackgroundCopyManager5_0(i32);
pub struct BackgroundCopyQMgr(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct FILESETINFO(i32);
pub struct GROUPPROP(i32);
pub struct IBITSExtensionSetup(pub *mut ::core::ffi::c_void);
pub struct IBITSExtensionSetupFactory(pub *mut ::core::ffi::c_void);
pub struct IBackgroundCopyCallback(pub *mut ::core::ffi::c_void);
pub struct IBackgroundCopyCallback1(pub *mut ::core::ffi::c_void);
pub struct IBackgroundCopyCallback2(pub *mut ::core::ffi::c_void);
pub struct IBackgroundCopyCallback3(pub *mut ::core::ffi::c_void);
pub struct IBackgroundCopyError(pub *mut ::core::ffi::c_void);
pub struct IBackgroundCopyFile(pub *mut ::core::ffi::c_void);
pub struct IBackgroundCopyFile2(pub *mut ::core::ffi::c_void);
pub struct IBackgroundCopyFile3(pub *mut ::core::ffi::c_void);
pub struct IBackgroundCopyFile4(pub *mut ::core::ffi::c_void);
pub struct IBackgroundCopyFile5(pub *mut ::core::ffi::c_void);
pub struct IBackgroundCopyFile6(pub *mut ::core::ffi::c_void);
pub struct IBackgroundCopyGroup(pub *mut ::core::ffi::c_void);
pub struct IBackgroundCopyJob(pub *mut ::core::ffi::c_void);
pub struct IBackgroundCopyJob1(pub *mut ::core::ffi::c_void);
pub struct IBackgroundCopyJob2(pub *mut ::core::ffi::c_void);
pub struct IBackgroundCopyJob3(pub *mut ::core::ffi::c_void);
pub struct IBackgroundCopyJob4(pub *mut ::core::ffi::c_void);
pub struct IBackgroundCopyJob5(pub *mut ::core::ffi::c_void);
pub struct IBackgroundCopyJobHttpOptions(pub *mut ::core::ffi::c_void);
pub struct IBackgroundCopyJobHttpOptions2(pub *mut ::core::ffi::c_void);
pub struct IBackgroundCopyJobHttpOptions3(pub *mut ::core::ffi::c_void);
pub struct IBackgroundCopyManager(pub *mut ::core::ffi::c_void);
pub struct IBackgroundCopyQMgr(pub *mut ::core::ffi::c_void);
pub struct IBackgroundCopyServerCertificateValidationCallback(pub *mut ::core::ffi::c_void);
pub struct IBitsPeer(pub *mut ::core::ffi::c_void);
pub struct IBitsPeerCacheAdministration(pub *mut ::core::ffi::c_void);
pub struct IBitsPeerCacheRecord(pub *mut ::core::ffi::c_void);
pub struct IBitsTokenOptions(pub *mut ::core::ffi::c_void);
pub struct IEnumBackgroundCopyFiles(pub *mut ::core::ffi::c_void);
pub struct IEnumBackgroundCopyGroups(pub *mut ::core::ffi::c_void);
pub struct IEnumBackgroundCopyJobs(pub *mut ::core::ffi::c_void);
pub struct IEnumBackgroundCopyJobs1(pub *mut ::core::ffi::c_void);
pub struct IEnumBitsPeerCacheRecords(pub *mut ::core::ffi::c_void);
pub struct IEnumBitsPeers(pub *mut ::core::ffi::c_void);
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const QM_E_DOWNLOADER_UNAVAILABLE: u32 = 2164264963u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const QM_E_INVALID_STATE: u32 = 2164264961u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const QM_E_ITEM_NOT_FOUND: u32 = 2164264964u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const QM_E_SERVICE_UNAVAILABLE: u32 = 2164264962u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const QM_NOTIFY_DISABLE_NOTIFY: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const QM_NOTIFY_FILE_DONE: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const QM_NOTIFY_GROUP_DONE: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const QM_NOTIFY_JOB_DONE: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const QM_NOTIFY_USE_PROGRESSEX: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const QM_PROGRESS_PERCENT_DONE: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const QM_PROGRESS_SIZE_DONE: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const QM_PROGRESS_TIME_DONE: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const QM_PROTOCOL_CUSTOM: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const QM_PROTOCOL_FTP: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const QM_PROTOCOL_HTTP: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const QM_PROTOCOL_SMB: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const QM_STATUS_FILE_COMPLETE: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const QM_STATUS_FILE_INCOMPLETE: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const QM_STATUS_GROUP_COMPLETE: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const QM_STATUS_GROUP_ERROR: u32 = 512u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const QM_STATUS_GROUP_FOREGROUND: u32 = 1024u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const QM_STATUS_GROUP_INCOMPLETE: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const QM_STATUS_GROUP_SUSPENDED: u32 = 256u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const QM_STATUS_JOB_COMPLETE: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const QM_STATUS_JOB_ERROR: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const QM_STATUS_JOB_FOREGROUND: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_BackgroundIntelligentTransferService`*"]
pub const QM_STATUS_JOB_INCOMPLETE: u32 = 8u32;
