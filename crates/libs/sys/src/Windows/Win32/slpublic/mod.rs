windows_link::link!("slcext.dll" "system" fn SLAcquireGenuineTicket(ppticketblob : *mut *mut core::ffi::c_void, pcbticketblob : *mut u32, pwsztemplateid : windows_sys::core::PCWSTR, pwszserverurl : windows_sys::core::PCWSTR, pwszclienttoken : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("slcext.dll" "system" fn SLActivateProduct(hslc : HSLC, pproductskuid : *const SLID, cbappspecificdata : u32, pvappspecificdata : *const core::ffi::c_void, pactivationinfo : *const SL_ACTIVATION_INFO_HEADER, pwszproxyserver : windows_sys::core::PCWSTR, wproxyport : u16) -> windows_sys::core::HRESULT);
windows_link::link!("slc.dll" "system" fn SLClose(hslc : HSLC) -> windows_sys::core::HRESULT);
windows_link::link!("slc.dll" "system" fn SLConsumeRight(hslc : HSLC, pappid : *const SLID, pproductskuid : *const SLID, pwszrightname : windows_sys::core::PCWSTR, pvreserved : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("slc.dll" "system" fn SLDepositOfflineConfirmationId(hslc : HSLC, pproductskuid : *const SLID, pwszinstallationid : windows_sys::core::PCWSTR, pwszconfirmationid : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("slc.dll" "system" fn SLDepositOfflineConfirmationIdEx(hslc : HSLC, pproductskuid : *const SLID, pactivationinfo : *const SL_ACTIVATION_INFO_HEADER, pwszinstallationid : windows_sys::core::PCWSTR, pwszconfirmationid : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("slc.dll" "system" fn SLFireEvent(hslc : HSLC, pwszeventid : windows_sys::core::PCWSTR, papplicationid : *const SLID) -> windows_sys::core::HRESULT);
windows_link::link!("slc.dll" "system" fn SLGenerateOfflineInstallationId(hslc : HSLC, pproductskuid : *const SLID, ppwszinstallationid : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("slc.dll" "system" fn SLGenerateOfflineInstallationIdEx(hslc : HSLC, pproductskuid : *const SLID, pactivationinfo : *const SL_ACTIVATION_INFO_HEADER, ppwszinstallationid : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("slc.dll" "system" fn SLGetApplicationInformation(hslc : HSLC, papplicationid : *const SLID, pwszvaluename : windows_sys::core::PCWSTR, pedatatype : *mut SLDATATYPE, pcbvalue : *mut u32, ppbvalue : *mut super::minwindef::PBYTE) -> windows_sys::core::HRESULT);
windows_link::link!("slc.dll" "system" fn SLGetGenuineInformation(pqueryid : *const SLID, pwszvaluename : windows_sys::core::PCWSTR, pedatatype : *mut SLDATATYPE, pcbvalue : *mut u32, ppbvalue : *mut *mut u8) -> windows_sys::core::HRESULT);
windows_link::link!("slc.dll" "system" fn SLGetInstalledProductKeyIds(hslc : HSLC, pproductskuid : *const SLID, pnproductkeyids : *mut u32, ppproductkeyids : *mut *mut SLID) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("slc.dll" "system" fn SLGetLicense(hslc : HSLC, plicensefileid : *const SLID, pcblicensefile : *mut u32, ppblicensefile : *mut super::minwindef::PBYTE) -> windows_sys::core::HRESULT);
windows_link::link!("slc.dll" "system" fn SLGetLicenseFileId(hslc : HSLC, cblicenseblob : u32, pblicenseblob : *const u8, plicensefileid : *mut SLID) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("slc.dll" "system" fn SLGetLicenseInformation(hslc : HSLC, psllicenseid : *const SLID, pwszvaluename : windows_sys::core::PCWSTR, pedatatype : *mut SLDATATYPE, pcbvalue : *mut u32, ppbvalue : *mut super::minwindef::PBYTE) -> windows_sys::core::HRESULT);
windows_link::link!("slc.dll" "system" fn SLGetLicensingStatusInformation(hslc : HSLC, pappid : *const SLID, pproductskuid : *const SLID, pwszrightname : windows_sys::core::PCWSTR, pnstatuscount : *mut u32, pplicensingstatus : *mut *mut SL_LICENSING_STATUS) -> windows_sys::core::HRESULT);
windows_link::link!("slc.dll" "system" fn SLGetPKeyId(hslc : HSLC, pwszpkeyalgorithm : windows_sys::core::PCWSTR, pwszpkeystring : windows_sys::core::PCWSTR, cbpkeyspecificdata : u32, pbpkeyspecificdata : *const u8, ppkeyid : *mut SLID) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("slc.dll" "system" fn SLGetPKeyInformation(hslc : HSLC, ppkeyid : *const SLID, pwszvaluename : windows_sys::core::PCWSTR, pedatatype : *mut SLDATATYPE, pcbvalue : *mut u32, ppbvalue : *mut super::minwindef::PBYTE) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("slc.dll" "system" fn SLGetPolicyInformation(hslc : HSLC, pwszvaluename : windows_sys::core::PCWSTR, pedatatype : *mut SLDATATYPE, pcbvalue : *mut u32, ppbvalue : *mut super::minwindef::PBYTE) -> windows_sys::core::HRESULT);
windows_link::link!("slc.dll" "system" fn SLGetPolicyInformationDWORD(hslc : HSLC, pwszvaluename : windows_sys::core::PCWSTR, pdwvalue : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("slc.dll" "system" fn SLGetProductSkuInformation(hslc : HSLC, pproductskuid : *const SLID, pwszvaluename : windows_sys::core::PCWSTR, pedatatype : *mut SLDATATYPE, pcbvalue : *mut u32, ppbvalue : *mut super::minwindef::PBYTE) -> windows_sys::core::HRESULT);
windows_link::link!("slcext.dll" "system" fn SLGetReferralInformation(hslc : HSLC, ereferraltype : SLREFERRALTYPE, pskuorappid : *const SLID, pwszvaluename : windows_sys::core::PCWSTR, ppwszvalue : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("slc.dll" "system" fn SLGetSLIDList(hslc : HSLC, equeryidtype : SLIDTYPE, pqueryid : *const SLID, ereturnidtype : SLIDTYPE, pnreturnids : *mut u32, ppreturnids : *mut *mut SLID) -> windows_sys::core::HRESULT);
windows_link::link!("slcext.dll" "system" fn SLGetServerStatus(pwszserverurl : windows_sys::core::PCWSTR, pwszacquisitiontype : windows_sys::core::PCWSTR, pwszproxyserver : windows_sys::core::PCWSTR, wproxyport : u16, phrstatus : *mut windows_sys::core::HRESULT) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("slc.dll" "system" fn SLGetServiceInformation(hslc : HSLC, pwszvaluename : windows_sys::core::PCWSTR, pedatatype : *mut SLDATATYPE, pcbvalue : *mut u32, ppbvalue : *mut super::minwindef::PBYTE) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("slc.dll" "system" fn SLGetWindowsInformation(pwszvaluename : windows_sys::core::PCWSTR, pedatatype : *mut SLDATATYPE, pcbvalue : *mut u32, ppbvalue : *mut super::minwindef::PBYTE) -> windows_sys::core::HRESULT);
windows_link::link!("slc.dll" "system" fn SLGetWindowsInformationDWORD(pwszvaluename : windows_sys::core::PCWSTR, pdwvalue : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("slc.dll" "system" fn SLInstallLicense(hslc : HSLC, cblicenseblob : u32, pblicenseblob : *const u8, plicensefileid : *mut SLID) -> windows_sys::core::HRESULT);
windows_link::link!("slc.dll" "system" fn SLInstallProofOfPurchase(hslc : HSLC, pwszpkeyalgorithm : windows_sys::core::PCWSTR, pwszpkeystring : windows_sys::core::PCWSTR, cbpkeyspecificdata : u32, pbpkeyspecificdata : *const u8, ppkeyid : *mut SLID) -> windows_sys::core::HRESULT);
windows_link::link!("slwga.dll" "system" fn SLIsGenuineLocal(pappid : *const SLID, pgenuinestate : *mut SL_GENUINE_STATE, puioptions : *mut SL_NONGENUINE_UI_OPTIONS) -> windows_sys::core::HRESULT);
windows_link::link!("slc.dll" "system" fn SLOpen(phslc : *mut HSLC) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-slapi-l1-1-0.dll" "system" fn SLQueryLicenseValueFromApp(valuename : windows_sys::core::PCWSTR, valuetype : *mut u32, databuffer : *mut core::ffi::c_void, datasize : u32, resultdatasize : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("slc.dll" "system" fn SLRegisterEvent(hslc : HSLC, pwszeventid : windows_sys::core::PCWSTR, papplicationid : *const SLID, hevent : super::winnt::HANDLE) -> windows_sys::core::HRESULT);
windows_link::link!("slc.dll" "system" fn SLSetCurrentProductKey(hslc : HSLC, pproductskuid : *const SLID, pproductkeyid : *const SLID) -> windows_sys::core::HRESULT);
windows_link::link!("slc.dll" "system" fn SLSetGenuineInformation(pqueryid : *const SLID, pwszvaluename : windows_sys::core::PCWSTR, edatatype : SLDATATYPE, cbvalue : u32, pbvalue : *const u8) -> windows_sys::core::HRESULT);
windows_link::link!("slc.dll" "system" fn SLUninstallLicense(hslc : HSLC, plicensefileid : *const SLID) -> windows_sys::core::HRESULT);
windows_link::link!("slc.dll" "system" fn SLUninstallProofOfPurchase(hslc : HSLC, ppkeyid : *const SLID) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("slc.dll" "system" fn SLUnregisterEvent(hslc : HSLC, pwszeventid : windows_sys::core::PCWSTR, papplicationid : *const SLID, hevent : super::winnt::HANDLE) -> windows_sys::core::HRESULT);
pub type HSLC = *mut core::ffi::c_void;
pub type HSLP = *mut core::ffi::c_void;
pub const ID_CAP_SLAPI: windows_sys::core::PCWSTR = windows_sys::core::w!("slapiQueryLicenseValue");
pub type PSL_SYSTEM_POLICY_INFORMATION = *mut SL_SYSTEM_POLICY_INFORMATION;
pub type SLDATATYPE = i32;
pub type SLID = windows_sys::core::GUID;
pub type SLIDTYPE = i32;
pub type SLLICENSINGSTATUS = i32;
pub type SLREFERRALTYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SL_ACTIVATION_INFO_HEADER {
    pub cbSize: u32,
    pub r#type: SL_ACTIVATION_TYPE,
}
pub type SL_ACTIVATION_TYPE = i32;
pub const SL_ACTIVATION_TYPE_ACTIVE_DIRECTORY: SL_ACTIVATION_TYPE = 1;
pub const SL_ACTIVATION_TYPE_DEFAULT: SL_ACTIVATION_TYPE = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SL_AD_ACTIVATION_INFO {
    pub header: SL_ACTIVATION_INFO_HEADER,
    pub pwszProductKey: windows_sys::core::PCWSTR,
    pub pwszActivationObjectName: windows_sys::core::PCWSTR,
}
impl Default for SL_AD_ACTIVATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SL_DATA_BINARY: SLDATATYPE = 3;
pub const SL_DATA_DWORD: SLDATATYPE = 4;
pub const SL_DATA_MULTI_SZ: SLDATATYPE = 7;
pub const SL_DATA_NONE: SLDATATYPE = 0;
pub const SL_DATA_SUM: SLDATATYPE = 100;
pub const SL_DATA_SZ: SLDATATYPE = 1;
pub const SL_DEFAULT_MIGRATION_ENCRYPTOR_URI: windows_sys::core::PCWSTR = windows_sys::core::w!("msft:spp/migrationencryptor/tokenact/1.0");
pub const SL_EVENT_LICENSING_STATE_CHANGED: windows_sys::core::PCWSTR = windows_sys::core::w!("msft:rm/event/licensingstatechanged");
pub const SL_EVENT_POLICY_CHANGED: windows_sys::core::PCWSTR = windows_sys::core::w!("msft:rm/event/policychanged");
pub const SL_EVENT_USER_NOTIFICATION: windows_sys::core::PCWSTR = windows_sys::core::w!("msft:rm/event/usernotification");
pub type SL_GENUINE_STATE = i32;
pub const SL_GEN_STATE_INVALID_LICENSE: SL_GENUINE_STATE = 1;
pub const SL_GEN_STATE_IS_GENUINE: SL_GENUINE_STATE = 0;
pub const SL_GEN_STATE_LAST: SL_GENUINE_STATE = 4;
pub const SL_GEN_STATE_OFFLINE: SL_GENUINE_STATE = 3;
pub const SL_GEN_STATE_TAMPERED: SL_GENUINE_STATE = 2;
pub const SL_ID_ALL_LICENSES: SLIDTYPE = 5;
pub const SL_ID_ALL_LICENSE_FILES: SLIDTYPE = 6;
pub const SL_ID_APPLICATION: SLIDTYPE = 0;
pub const SL_ID_LAST: SLIDTYPE = 8;
pub const SL_ID_LICENSE: SLIDTYPE = 3;
pub const SL_ID_LICENSE_FILE: SLIDTYPE = 2;
pub const SL_ID_PKEY: SLIDTYPE = 4;
pub const SL_ID_PRODUCT_SKU: SLIDTYPE = 1;
pub const SL_ID_STORE_TOKEN: SLIDTYPE = 7;
pub const SL_INFO_KEY_ACTIVE_PLUGINS: windows_sys::core::PCWSTR = windows_sys::core::w!("ActivePlugins");
pub const SL_INFO_KEY_AUTHOR: windows_sys::core::PCWSTR = windows_sys::core::w!("Author");
pub const SL_INFO_KEY_BIOS_OA2_MINOR_VERSION: windows_sys::core::PCWSTR = windows_sys::core::w!("BiosOA2MinorVersion");
pub const SL_INFO_KEY_BIOS_PKEY: windows_sys::core::PCWSTR = windows_sys::core::w!("BiosProductKey");
pub const SL_INFO_KEY_BIOS_PKEY_DESCRIPTION: windows_sys::core::PCWSTR = windows_sys::core::w!("BiosProductKeyDescription");
pub const SL_INFO_KEY_BIOS_PKEY_PKPN: windows_sys::core::PCWSTR = windows_sys::core::w!("BiosProductKeyPkPn");
pub const SL_INFO_KEY_BIOS_SLIC_STATE: windows_sys::core::PCWSTR = windows_sys::core::w!("BiosSlicState");
pub const SL_INFO_KEY_CHANNEL: windows_sys::core::PCWSTR = windows_sys::core::w!("Channel");
pub const SL_INFO_KEY_DESCRIPTION: windows_sys::core::PCWSTR = windows_sys::core::w!("Description");
pub const SL_INFO_KEY_DIGITAL_PID: windows_sys::core::PCWSTR = windows_sys::core::w!("DigitalPID");
pub const SL_INFO_KEY_DIGITAL_PID2: windows_sys::core::PCWSTR = windows_sys::core::w!("DigitalPID2");
pub const SL_INFO_KEY_IS_KMS: windows_sys::core::PCWSTR = windows_sys::core::w!("IsKeyManagementService");
pub const SL_INFO_KEY_IS_PRS: windows_sys::core::PCWSTR = windows_sys::core::w!("IsPRS");
pub const SL_INFO_KEY_KMS_CURRENT_COUNT: windows_sys::core::PCWSTR = windows_sys::core::w!("KeyManagementServiceCurrentCount");
pub const SL_INFO_KEY_KMS_FAILED_REQUESTS: windows_sys::core::PCWSTR = windows_sys::core::w!("KeyManagementServiceFailedRequests");
pub const SL_INFO_KEY_KMS_LICENSED_REQUESTS: windows_sys::core::PCWSTR = windows_sys::core::w!("KeyManagementServiceLicensedRequests");
pub const SL_INFO_KEY_KMS_NON_GENUINE_GRACE_REQUESTS: windows_sys::core::PCWSTR = windows_sys::core::w!("KeyManagementServiceNonGenuineGraceRequests");
pub const SL_INFO_KEY_KMS_NOTIFICATION_REQUESTS: windows_sys::core::PCWSTR = windows_sys::core::w!("KeyManagementServiceNotificationRequests");
pub const SL_INFO_KEY_KMS_OOB_GRACE_REQUESTS: windows_sys::core::PCWSTR = windows_sys::core::w!("KeyManagementServiceOOBGraceRequests");
pub const SL_INFO_KEY_KMS_OOT_GRACE_REQUESTS: windows_sys::core::PCWSTR = windows_sys::core::w!("KeyManagementServiceOOTGraceRequests");
pub const SL_INFO_KEY_KMS_REQUIRED_CLIENT_COUNT: windows_sys::core::PCWSTR = windows_sys::core::w!("KeyManagementServiceRequiredClientCount");
pub const SL_INFO_KEY_KMS_TOTAL_REQUESTS: windows_sys::core::PCWSTR = windows_sys::core::w!("KeyManagementServiceTotalRequests");
pub const SL_INFO_KEY_KMS_UNLICENSED_REQUESTS: windows_sys::core::PCWSTR = windows_sys::core::w!("KeyManagementServiceUnlicensedRequests");
pub const SL_INFO_KEY_LICENSE_TYPE: windows_sys::core::PCWSTR = windows_sys::core::w!("LicenseType");
pub const SL_INFO_KEY_LICENSOR_URL: windows_sys::core::PCWSTR = windows_sys::core::w!("LicensorUrl");
pub const SL_INFO_KEY_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("Name");
pub const SL_INFO_KEY_PARTIAL_PRODUCT_KEY: windows_sys::core::PCWSTR = windows_sys::core::w!("PartialProductKey");
pub const SL_INFO_KEY_PRODUCT_KEY_ACTIVATION_URL: windows_sys::core::PCWSTR = windows_sys::core::w!("PKCURL");
pub const SL_INFO_KEY_PRODUCT_SKU_ID: windows_sys::core::PCWSTR = windows_sys::core::w!("ProductSkuId");
pub const SL_INFO_KEY_RIGHT_ACCOUNT_ACTIVATION_URL: windows_sys::core::PCWSTR = windows_sys::core::w!("RACURL");
pub const SL_INFO_KEY_SECURE_PROCESSOR_ACTIVATION_URL: windows_sys::core::PCWSTR = windows_sys::core::w!("SPCURL");
pub const SL_INFO_KEY_SECURE_STORE_ID: windows_sys::core::PCWSTR = windows_sys::core::w!("SecureStoreId");
pub const SL_INFO_KEY_SYSTEM_STATE: windows_sys::core::PCWSTR = windows_sys::core::w!("SystemState");
pub const SL_INFO_KEY_USE_LICENSE_ACTIVATION_URL: windows_sys::core::PCWSTR = windows_sys::core::w!("EULURL");
pub const SL_INFO_KEY_VERSION: windows_sys::core::PCWSTR = windows_sys::core::w!("Version");
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SL_LICENSING_STATUS {
    pub SkuId: SLID,
    pub eStatus: SLLICENSINGSTATUS,
    pub dwGraceTime: u32,
    pub dwTotalGraceDays: u32,
    pub hrReason: windows_sys::core::HRESULT,
    pub qwValidityExpiration: u64,
}
pub const SL_LICENSING_STATUS_IN_GRACE_PERIOD: SLLICENSINGSTATUS = 2;
pub const SL_LICENSING_STATUS_LAST: SLLICENSINGSTATUS = 4;
pub const SL_LICENSING_STATUS_LICENSED: SLLICENSINGSTATUS = 1;
pub const SL_LICENSING_STATUS_NOTIFICATION: SLLICENSINGSTATUS = 3;
pub const SL_LICENSING_STATUS_UNLICENSED: SLLICENSINGSTATUS = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SL_NONGENUINE_UI_OPTIONS {
    pub cbSize: u32,
    pub pComponentId: *const SLID,
    pub hResultUI: windows_sys::core::HRESULT,
}
impl Default for SL_NONGENUINE_UI_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SL_PKEY_DETECT: windows_sys::core::PCWSTR = windows_sys::core::w!("msft:rm/algorithm/pkey/detect");
pub const SL_PKEY_MS2005: windows_sys::core::PCWSTR = windows_sys::core::w!("msft:rm/algorithm/pkey/2005");
pub const SL_PKEY_MS2009: windows_sys::core::PCWSTR = windows_sys::core::w!("msft:rm/algorithm/pkey/2009");
pub const SL_POLICY_EVALUATION_MODE_ENABLED: windows_sys::core::PCWSTR = windows_sys::core::w!("Security-SPP-EvaluationModeEnabled");
pub const SL_PROP_ACTIVATION_VALIDATION_IN_PROGRESS: windows_sys::core::PCWSTR = windows_sys::core::w!("SL_ACTIVATION_VALIDATION_IN_PROGRESS");
pub const SL_PROP_BRT_COMMIT: windows_sys::core::PCWSTR = windows_sys::core::w!("SL_BRT_COMMIT");
pub const SL_PROP_BRT_DATA: windows_sys::core::PCWSTR = windows_sys::core::w!("SL_BRT_DATA");
pub const SL_PROP_GENUINE_RESULT: windows_sys::core::PCWSTR = windows_sys::core::w!("SL_GENUINE_RESULT");
pub const SL_PROP_GET_GENUINE_AUTHZ: windows_sys::core::PCWSTR = windows_sys::core::w!("SL_GET_GENUINE_AUTHZ");
pub const SL_PROP_GET_GENUINE_SERVER_AUTHZ: windows_sys::core::PCWSTR = windows_sys::core::w!("SL_GET_GENUINE_SERVER_AUTHZ");
pub const SL_PROP_LAST_ACT_ATTEMPT_HRESULT: windows_sys::core::PCWSTR = windows_sys::core::w!("SL_LAST_ACT_ATTEMPT_HRESULT");
pub const SL_PROP_LAST_ACT_ATTEMPT_SERVER_FLAGS: windows_sys::core::PCWSTR = windows_sys::core::w!("SL_LAST_ACT_ATTEMPT_SERVER_FLAGS");
pub const SL_PROP_LAST_ACT_ATTEMPT_TIME: windows_sys::core::PCWSTR = windows_sys::core::w!("SL_LAST_ACT_ATTEMPT_TIME");
pub const SL_PROP_NONGENUINE_GRACE_FLAG: windows_sys::core::PCWSTR = windows_sys::core::w!("SL_NONGENUINE_GRACE_FLAG");
pub const SL_REARM_REBOOT_REQUIRED: u32 = 1;
pub const SL_REFERRALTYPE_APPID: SLREFERRALTYPE = 1;
pub const SL_REFERRALTYPE_BEST_MATCH: SLREFERRALTYPE = 4;
pub const SL_REFERRALTYPE_OVERRIDE_APPID: SLREFERRALTYPE = 3;
pub const SL_REFERRALTYPE_OVERRIDE_SKUID: SLREFERRALTYPE = 2;
pub const SL_REFERRALTYPE_SKUID: SLREFERRALTYPE = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SL_SYSTEM_POLICY_INFORMATION {
    pub Reserved1: [*mut core::ffi::c_void; 2],
    pub Reserved2: [u32; 3],
}
impl Default for SL_SYSTEM_POLICY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SL_SYSTEM_STATE_REBOOT_POLICY_FOUND: u32 = 1;
pub const SL_SYSTEM_STATE_TAMPERED: u32 = 2;
pub const SPP_MIGRATION_GATHER_ACTIVATED_WINDOWS_STATE: u32 = 2;
pub const SPP_MIGRATION_GATHER_ALL: u32 = 4294967295;
pub const SPP_MIGRATION_GATHER_MIGRATABLE_APPS: u32 = 1;
