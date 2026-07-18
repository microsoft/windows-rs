windows_link::link!("rasapi32.dll" "system" fn RasClearConnectionStatistics(hrasconn : HRASCONN) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasClearLinkStatistics(hrasconn : HRASCONN, dwsubentry : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("rasapi32.dll" "system" fn RasConnectionNotificationA(param0 : HRASCONN, param1 : super::HANDLE, param2 : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("rasapi32.dll" "system" fn RasConnectionNotificationW(param0 : HRASCONN, param1 : super::HANDLE, param2 : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("rasapi32.dll" "system" fn RasCreatePhonebookEntryA(param0 : super::HWND, param1 : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("rasapi32.dll" "system" fn RasCreatePhonebookEntryW(param0 : super::HWND, param1 : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasDeleteEntryA(param0 : windows_sys::core::PCSTR, param1 : windows_sys::core::PCSTR) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasDeleteEntryW(param0 : windows_sys::core::PCWSTR, param1 : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasDeleteSubEntryA(pszphonebook : windows_sys::core::PCSTR, pszentry : windows_sys::core::PCSTR, dwsubentryid : u32) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasDeleteSubEntryW(pszphonebook : windows_sys::core::PCWSTR, pszentry : windows_sys::core::PCWSTR, dwsubentryid : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("rasapi32.dll" "system" fn RasDialA(param0 : *const tagRASDIALEXTENSIONS, param1 : windows_sys::core::PCSTR, param2 : *const tagRASDIALPARAMSA, param3 : u32, param4 : *const core::ffi::c_void, param5 : *mut HRASCONN) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("rasapi32.dll" "system" fn RasDialW(param0 : *const tagRASDIALEXTENSIONS, param1 : windows_sys::core::PCWSTR, param2 : *const tagRASDIALPARAMSW, param3 : u32, param4 : *const core::ffi::c_void, param5 : *mut HRASCONN) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("rasapi32.dll" "system" fn RasEditPhonebookEntryA(param0 : super::HWND, param1 : windows_sys::core::PCSTR, param2 : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("rasapi32.dll" "system" fn RasEditPhonebookEntryW(param0 : super::HWND, param1 : windows_sys::core::PCWSTR, param2 : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasEnumAutodialAddressesA(lpprasautodialaddresses : *mut windows_sys::core::PSTR, lpdwcbrasautodialaddresses : *mut u32, lpdwcrasautodialaddresses : *mut u32) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasEnumAutodialAddressesW(lpprasautodialaddresses : *mut windows_sys::core::PWSTR, lpdwcbrasautodialaddresses : *mut u32, lpdwcrasautodialaddresses : *mut u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("rasapi32.dll" "system" fn RasEnumConnectionsA(param0 : *mut tagRASCONNA, param1 : *mut u32, param2 : *mut u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("rasapi32.dll" "system" fn RasEnumConnectionsW(param0 : *mut tagRASCONNW, param1 : *mut u32, param2 : *mut u32) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasEnumDevicesA(param0 : *mut tagRASDEVINFOA, param1 : *mut u32, param2 : *mut u32) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasEnumDevicesW(param0 : *mut tagRASDEVINFOW, param1 : *mut u32, param2 : *mut u32) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasEnumEntriesA(param0 : windows_sys::core::PCSTR, param1 : windows_sys::core::PCSTR, param2 : *mut tagRASENTRYNAMEA, param3 : *mut u32, param4 : *mut u32) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasEnumEntriesW(param0 : windows_sys::core::PCWSTR, param1 : windows_sys::core::PCWSTR, param2 : *mut tagRASENTRYNAMEW, param3 : *mut u32, param4 : *mut u32) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasFreeEapUserIdentityA(praseapuseridentity : *const tagRASEAPUSERIDENTITYA));
windows_link::link!("rasapi32.dll" "system" fn RasFreeEapUserIdentityW(praseapuseridentity : *const tagRASEAPUSERIDENTITYW));
windows_link::link!("rasapi32.dll" "system" fn RasGetAutodialAddressA(param0 : windows_sys::core::PCSTR, param1 : *const u32, param2 : *mut tagRASAUTODIALENTRYA, param3 : *mut u32, param4 : *mut u32) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasGetAutodialAddressW(param0 : windows_sys::core::PCWSTR, param1 : *const u32, param2 : *mut tagRASAUTODIALENTRYW, param3 : *mut u32, param4 : *mut u32) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasGetAutodialEnableA(param0 : u32, param1 : *mut windows_sys::core::BOOL) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasGetAutodialEnableW(param0 : u32, param1 : *mut windows_sys::core::BOOL) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasGetAutodialParamA(param0 : u32, param1 : *mut core::ffi::c_void, param2 : *mut u32) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasGetAutodialParamW(param0 : u32, param1 : *mut core::ffi::c_void, param2 : *mut u32) -> u32);
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
windows_link::link!("rasapi32.dll" "system" fn RasGetConnectStatusA(param0 : HRASCONN, param1 : *mut tagRASCONNSTATUSA) -> u32);
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
windows_link::link!("rasapi32.dll" "system" fn RasGetConnectStatusW(param0 : HRASCONN, param1 : *mut tagRASCONNSTATUSW) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasGetConnectionStatistics(hrasconn : HRASCONN, lpstatistics : *mut RAS_STATS) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasGetCountryInfoA(param0 : *mut RASCTRYINFO, param1 : *mut u32) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasGetCountryInfoW(param0 : *mut RASCTRYINFO, param1 : *mut u32) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasGetCredentialsA(param0 : windows_sys::core::PCSTR, param1 : windows_sys::core::PCSTR, param2 : *mut tagRASCREDENTIALSA) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasGetCredentialsW(param0 : windows_sys::core::PCWSTR, param1 : windows_sys::core::PCWSTR, param2 : *mut tagRASCREDENTIALSW) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasGetCustomAuthDataA(pszphonebook : windows_sys::core::PCSTR, pszentry : windows_sys::core::PCSTR, pbcustomauthdata : *mut u8, pdwsizeofcustomauthdata : *mut u32) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasGetCustomAuthDataW(pszphonebook : windows_sys::core::PCWSTR, pszentry : windows_sys::core::PCWSTR, pbcustomauthdata : *mut u8, pdwsizeofcustomauthdata : *mut u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("rasapi32.dll" "system" fn RasGetEapUserDataA(htoken : super::HANDLE, pszphonebook : windows_sys::core::PCSTR, pszentry : windows_sys::core::PCSTR, pbeapdata : *mut u8, pdwsizeofeapdata : *mut u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("rasapi32.dll" "system" fn RasGetEapUserDataW(htoken : super::HANDLE, pszphonebook : windows_sys::core::PCWSTR, pszentry : windows_sys::core::PCWSTR, pbeapdata : *mut u8, pdwsizeofeapdata : *mut u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("rasapi32.dll" "system" fn RasGetEapUserIdentityA(pszphonebook : windows_sys::core::PCSTR, pszentry : windows_sys::core::PCSTR, dwflags : u32, hwnd : super::HWND, ppraseapuseridentity : *mut *mut tagRASEAPUSERIDENTITYA) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("rasapi32.dll" "system" fn RasGetEapUserIdentityW(pszphonebook : windows_sys::core::PCWSTR, pszentry : windows_sys::core::PCWSTR, dwflags : u32, hwnd : super::HWND, ppraseapuseridentity : *mut *mut tagRASEAPUSERIDENTITYW) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasGetEntryDialParamsA(param0 : windows_sys::core::PCSTR, param1 : *mut tagRASDIALPARAMSA, param2 : *mut windows_sys::core::BOOL) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasGetEntryDialParamsW(param0 : windows_sys::core::PCWSTR, param1 : *mut tagRASDIALPARAMSW, param2 : *mut windows_sys::core::BOOL) -> u32);
#[cfg(feature = "in6addr")]
windows_link::link!("rasapi32.dll" "system" fn RasGetEntryPropertiesA(param0 : windows_sys::core::PCSTR, param1 : windows_sys::core::PCSTR, param2 : *mut tagRASENTRYA, param3 : *mut u32, param4 : *mut u8, param5 : *mut u32) -> u32);
#[cfg(feature = "in6addr")]
windows_link::link!("rasapi32.dll" "system" fn RasGetEntryPropertiesW(param0 : windows_sys::core::PCWSTR, param1 : windows_sys::core::PCWSTR, param2 : *mut tagRASENTRYW, param3 : *mut u32, param4 : *mut u8, param5 : *mut u32) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasGetErrorStringA(resourceid : u32, lpszstring : windows_sys::core::PSTR, inbufsize : u32) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasGetErrorStringW(resourceid : u32, lpszstring : windows_sys::core::PWSTR, inbufsize : u32) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasGetLinkStatistics(hrasconn : HRASCONN, dwsubentry : u32, lpstatistics : *mut RAS_STATS) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasGetPCscf(lpszpcscf : windows_sys::core::PWSTR) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasGetProjectionInfoA(param0 : HRASCONN, param1 : tagRASPROJECTION, param2 : *mut core::ffi::c_void, param3 : *mut u32) -> u32);
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
windows_link::link!("rasapi32.dll" "system" fn RasGetProjectionInfoEx(hrasconn : HRASCONN, prasprojection : *mut RAS_PROJECTION_INFO, lpdwsize : *mut u32) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasGetProjectionInfoW(param0 : HRASCONN, param1 : tagRASPROJECTION, param2 : *mut core::ffi::c_void, param3 : *mut u32) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasGetSubEntryHandleA(param0 : HRASCONN, param1 : u32, param2 : *mut HRASCONN) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasGetSubEntryHandleW(param0 : HRASCONN, param1 : u32, param2 : *mut HRASCONN) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasGetSubEntryPropertiesA(param0 : windows_sys::core::PCSTR, param1 : windows_sys::core::PCSTR, param2 : u32, param3 : *mut tagRASSUBENTRYA, param4 : *mut u32, param5 : *mut u8, param6 : *mut u32) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasGetSubEntryPropertiesW(param0 : windows_sys::core::PCWSTR, param1 : windows_sys::core::PCWSTR, param2 : u32, param3 : *mut tagRASSUBENTRYW, param4 : *mut u32, param5 : *mut u8, param6 : *mut u32) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasHangUpA(param0 : HRASCONN) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasHangUpW(param0 : HRASCONN) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("rasapi32.dll" "system" fn RasInvokeEapUI(param0 : HRASCONN, param1 : u32, param2 : *const tagRASDIALEXTENSIONS, param3 : super::HWND) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasRenameEntryA(param0 : windows_sys::core::PCSTR, param1 : windows_sys::core::PCSTR, param2 : windows_sys::core::PCSTR) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasRenameEntryW(param0 : windows_sys::core::PCWSTR, param1 : windows_sys::core::PCWSTR, param2 : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasSetAutodialAddressA(param0 : windows_sys::core::PCSTR, param1 : u32, param2 : *const tagRASAUTODIALENTRYA, param3 : u32, param4 : u32) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasSetAutodialAddressW(param0 : windows_sys::core::PCWSTR, param1 : u32, param2 : *const tagRASAUTODIALENTRYW, param3 : u32, param4 : u32) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasSetAutodialEnableA(param0 : u32, param1 : windows_sys::core::BOOL) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasSetAutodialEnableW(param0 : u32, param1 : windows_sys::core::BOOL) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasSetAutodialParamA(param0 : u32, param1 : *const core::ffi::c_void, param2 : u32) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasSetAutodialParamW(param0 : u32, param1 : *const core::ffi::c_void, param2 : u32) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasSetCredentialsA(param0 : windows_sys::core::PCSTR, param1 : windows_sys::core::PCSTR, param2 : *const tagRASCREDENTIALSA, param3 : windows_sys::core::BOOL) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasSetCredentialsW(param0 : windows_sys::core::PCWSTR, param1 : windows_sys::core::PCWSTR, param2 : *const tagRASCREDENTIALSW, param3 : windows_sys::core::BOOL) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasSetCustomAuthDataA(pszphonebook : windows_sys::core::PCSTR, pszentry : windows_sys::core::PCSTR, pbcustomauthdata : *const u8, dwsizeofcustomauthdata : u32) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasSetCustomAuthDataW(pszphonebook : windows_sys::core::PCWSTR, pszentry : windows_sys::core::PCWSTR, pbcustomauthdata : *const u8, dwsizeofcustomauthdata : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("rasapi32.dll" "system" fn RasSetEapUserDataA(htoken : super::HANDLE, pszphonebook : windows_sys::core::PCSTR, pszentry : windows_sys::core::PCSTR, pbeapdata : *const u8, dwsizeofeapdata : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("rasapi32.dll" "system" fn RasSetEapUserDataW(htoken : super::HANDLE, pszphonebook : windows_sys::core::PCWSTR, pszentry : windows_sys::core::PCWSTR, pbeapdata : *const u8, dwsizeofeapdata : u32) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasSetEntryDialParamsA(param0 : windows_sys::core::PCSTR, param1 : *const tagRASDIALPARAMSA, param2 : windows_sys::core::BOOL) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasSetEntryDialParamsW(param0 : windows_sys::core::PCWSTR, param1 : *const tagRASDIALPARAMSW, param2 : windows_sys::core::BOOL) -> u32);
#[cfg(feature = "in6addr")]
windows_link::link!("rasapi32.dll" "system" fn RasSetEntryPropertiesA(param0 : windows_sys::core::PCSTR, param1 : windows_sys::core::PCSTR, param2 : *const tagRASENTRYA, param3 : u32, param4 : *const u8, param5 : u32) -> u32);
#[cfg(feature = "in6addr")]
windows_link::link!("rasapi32.dll" "system" fn RasSetEntryPropertiesW(param0 : windows_sys::core::PCWSTR, param1 : windows_sys::core::PCWSTR, param2 : *const tagRASENTRYW, param3 : u32, param4 : *const u8, param5 : u32) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasSetSubEntryPropertiesA(param0 : windows_sys::core::PCSTR, param1 : windows_sys::core::PCSTR, param2 : u32, param3 : *const tagRASSUBENTRYA, param4 : u32, param5 : *const u8, param6 : u32) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasSetSubEntryPropertiesW(param0 : windows_sys::core::PCWSTR, param1 : windows_sys::core::PCWSTR, param2 : u32, param3 : *const tagRASSUBENTRYW, param4 : u32, param5 : *const u8, param6 : u32) -> u32);
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
windows_link::link!("rasapi32.dll" "system" fn RasUpdateConnection(hrasconn : HRASCONN, lprasupdateconn : *const tagRASUPDATECONN) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasValidateEntryNameA(param0 : windows_sys::core::PCSTR, param1 : windows_sys::core::PCSTR) -> u32);
windows_link::link!("rasapi32.dll" "system" fn RasValidateEntryNameW(param0 : windows_sys::core::PCWSTR, param1 : windows_sys::core::PCWSTR) -> u32);
pub const ET_None: u32 = 0;
pub const ET_Optional: u32 = 3;
pub const ET_Require: u32 = 1;
pub const ET_RequireMax: u32 = 2;
pub type HRASCONN = *mut core::ffi::c_void;
pub type IKEV2_ID_PAYLOAD_TYPE = i32;
pub const IKEV2_ID_PAYLOAD_TYPE_DER_ASN1_DN: IKEV2_ID_PAYLOAD_TYPE = 9;
pub const IKEV2_ID_PAYLOAD_TYPE_DER_ASN1_GN: IKEV2_ID_PAYLOAD_TYPE = 10;
pub const IKEV2_ID_PAYLOAD_TYPE_FQDN: IKEV2_ID_PAYLOAD_TYPE = 2;
pub const IKEV2_ID_PAYLOAD_TYPE_ID_IPV6_ADDR: IKEV2_ID_PAYLOAD_TYPE = 5;
pub const IKEV2_ID_PAYLOAD_TYPE_INVALID: IKEV2_ID_PAYLOAD_TYPE = 0;
pub const IKEV2_ID_PAYLOAD_TYPE_IPV4_ADDR: IKEV2_ID_PAYLOAD_TYPE = 1;
pub const IKEV2_ID_PAYLOAD_TYPE_KEY_ID: IKEV2_ID_PAYLOAD_TYPE = 11;
pub const IKEV2_ID_PAYLOAD_TYPE_MAX: IKEV2_ID_PAYLOAD_TYPE = 12;
pub const IKEV2_ID_PAYLOAD_TYPE_RESERVED1: IKEV2_ID_PAYLOAD_TYPE = 4;
pub const IKEV2_ID_PAYLOAD_TYPE_RESERVED2: IKEV2_ID_PAYLOAD_TYPE = 6;
pub const IKEV2_ID_PAYLOAD_TYPE_RESERVED3: IKEV2_ID_PAYLOAD_TYPE = 7;
pub const IKEV2_ID_PAYLOAD_TYPE_RESERVED4: IKEV2_ID_PAYLOAD_TYPE = 8;
pub const IKEV2_ID_PAYLOAD_TYPE_RFC822_ADDR: IKEV2_ID_PAYLOAD_TYPE = 3;
#[cfg(feature = "windef")]
pub type ORASADFUNC = Option<unsafe extern "system" fn(param0: super::HWND, param1: windows_sys::core::PCSTR, param2: u32, param3: *mut u32) -> windows_sys::core::BOOL>;
pub type PFNRASFREEBUFFER = Option<unsafe extern "system" fn(pbufer: *mut u8) -> u32>;
#[cfg(feature = "minwindef")]
pub type PFNRASGETBUFFER = Option<unsafe extern "system" fn(ppbuffer: *mut super::PBYTE, pdwsize: *mut u32) -> u32>;
#[cfg(feature = "winnt")]
pub type PFNRASRECEIVEBUFFER = Option<unsafe extern "system" fn(hport: super::HANDLE, pbuffer: *mut u8, pdwsize: *mut u32, dwtimeout: u32, hevent: super::HANDLE) -> u32>;
#[cfg(feature = "winnt")]
pub type PFNRASRETRIEVEBUFFER = Option<unsafe extern "system" fn(hport: super::HANDLE, pbuffer: *mut u8, pdwsize: *mut u32) -> u32>;
#[cfg(feature = "winnt")]
pub type PFNRASSENDBUFFER = Option<unsafe extern "system" fn(hport: super::HANDLE, pbuffer: *mut u8, dwsize: u32) -> u32>;
#[cfg(feature = "winnt")]
pub type PFNRASSETCOMMSETTINGS = Option<unsafe extern "system" fn(hport: super::HANDLE, prascommsettings: *mut tagRASCOMMSETTINGS, pvreserved: *mut core::ffi::c_void) -> u32>;
pub type PRASDEVSPECIFICINFO = *mut RASDEVSPECIFICINFO;
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
pub type PRASIKEV2_PROJECTION_INFO = *mut RASIKEV2_PROJECTION_INFO;
#[cfg(feature = "inaddr")]
pub type PRASPPP_PROJECTION_INFO = *mut RASPPP_PROJECTION_INFO;
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
pub type PRAS_PROJECTION_INFO = *mut RAS_PROJECTION_INFO;
pub type PRAS_STATS = *mut RAS_STATS;
pub const PROJECTION_INFO_TYPE_IKEv2: RASPROJECTION_INFO_TYPE = 2;
pub const PROJECTION_INFO_TYPE_PPP: RASPROJECTION_INFO_TYPE = 1;
pub const RASADFLG_PositionDlg: u32 = 1;
#[cfg(feature = "windef")]
pub type RASADFUNCA = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR, param1: windows_sys::core::PCSTR, param2: *mut tagRASADPARAMS, param3: *mut u32) -> windows_sys::core::BOOL>;
#[cfg(feature = "windef")]
pub type RASADFUNCW = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: windows_sys::core::PCWSTR, param2: *mut tagRASADPARAMS, param3: *mut u32) -> windows_sys::core::BOOL>;
pub const RASADP_ConnectionQueryTimeout: u32 = 4;
pub const RASADP_DisableConnectionQuery: u32 = 0;
pub const RASADP_FailedConnectionTimeout: u32 = 3;
pub const RASADP_LoginSessionDisable: u32 = 1;
pub const RASADP_SavedAddressesLimit: u32 = 2;
pub type RASAPIVERSION = i32;
pub const RASAPIVERSION_500: RASAPIVERSION = 1;
pub const RASAPIVERSION_501: RASAPIVERSION = 2;
pub const RASAPIVERSION_600: RASAPIVERSION = 3;
pub const RASAPIVERSION_601: RASAPIVERSION = 4;
pub const RASAPIVERSION_CURRENT: u32 = 4;
pub const RASCCPCA_MPPC: u32 = 6;
pub const RASCCPCA_STAC: u32 = 5;
pub const RASCCPO_Compression: u32 = 1;
pub const RASCCPO_Encryption128bit: u32 = 64;
pub const RASCCPO_Encryption40bit: u32 = 32;
pub const RASCCPO_Encryption56bit: u32 = 16;
pub const RASCCPO_HistoryLess: u32 = 2;
pub const RASCF_AllUsers: u32 = 1;
pub const RASCF_GlobalCreds: u32 = 2;
pub const RASCF_OwnerKnown: u32 = 4;
pub const RASCF_OwnerMatch: u32 = 8;
pub const RASCM_DDMPreSharedKey: u32 = 64;
pub const RASCM_DefaultCreds: u32 = 8;
pub const RASCM_Domain: u32 = 4;
pub const RASCM_Password: u32 = 2;
pub const RASCM_PreSharedKey: u32 = 16;
pub const RASCM_ServerPreSharedKey: u32 = 32;
pub const RASCM_UserName: u32 = 1;
pub const RASCN_BandwidthAdded: u32 = 4;
pub const RASCN_BandwidthRemoved: u32 = 8;
pub const RASCN_Connection: u32 = 1;
pub const RASCN_Disconnection: u32 = 2;
pub const RASCN_Dormant: u32 = 16;
pub const RASCN_EPDGPacketArrival: u32 = 64;
pub const RASCN_ReConnection: u32 = 32;
pub const RASCSS_DONE: u32 = 8192;
pub const RASCSS_Dormant: tagRASCONNSUBSTATE = 1;
pub const RASCSS_None: tagRASCONNSUBSTATE = 0;
pub const RASCSS_Reconnected: tagRASCONNSUBSTATE = 8192;
pub const RASCSS_Reconnecting: tagRASCONNSUBSTATE = 2;
pub const RASCS_AllDevicesConnected: tagRASCONNSTATE = 4;
pub const RASCS_ApplySettings: tagRASCONNSTATE = 24;
pub const RASCS_AuthAck: tagRASCONNSTATE = 12;
pub const RASCS_AuthCallback: tagRASCONNSTATE = 8;
pub const RASCS_AuthChangePassword: tagRASCONNSTATE = 9;
pub const RASCS_AuthLinkSpeed: tagRASCONNSTATE = 11;
pub const RASCS_AuthNotify: tagRASCONNSTATE = 6;
pub const RASCS_AuthProject: tagRASCONNSTATE = 10;
pub const RASCS_AuthRetry: tagRASCONNSTATE = 7;
pub const RASCS_Authenticate: tagRASCONNSTATE = 5;
pub const RASCS_Authenticated: tagRASCONNSTATE = 14;
pub const RASCS_CallbackComplete: tagRASCONNSTATE = 20;
pub const RASCS_CallbackSetByCaller: tagRASCONNSTATE = 4098;
pub const RASCS_ConnectDevice: tagRASCONNSTATE = 2;
pub const RASCS_Connected: tagRASCONNSTATE = 8192;
pub const RASCS_DONE: u32 = 8192;
pub const RASCS_DeviceConnected: tagRASCONNSTATE = 3;
pub const RASCS_Disconnected: tagRASCONNSTATE = 8193;
pub const RASCS_Interactive: tagRASCONNSTATE = 4096;
pub const RASCS_InvokeEapUI: tagRASCONNSTATE = 4100;
pub const RASCS_LogonNetwork: tagRASCONNSTATE = 21;
pub const RASCS_OpenPort: tagRASCONNSTATE = 0;
pub const RASCS_PAUSED: u32 = 4096;
pub const RASCS_PasswordExpired: tagRASCONNSTATE = 4099;
pub const RASCS_PortOpened: tagRASCONNSTATE = 1;
pub const RASCS_PrepareForCallback: tagRASCONNSTATE = 15;
pub const RASCS_Projected: tagRASCONNSTATE = 18;
pub const RASCS_ReAuthenticate: tagRASCONNSTATE = 13;
pub const RASCS_RetryAuthentication: tagRASCONNSTATE = 4097;
pub const RASCS_StartAuthentication: tagRASCONNSTATE = 19;
pub const RASCS_SubEntryConnected: tagRASCONNSTATE = 22;
pub const RASCS_SubEntryDisconnected: tagRASCONNSTATE = 23;
pub const RASCS_WaitForCallback: tagRASCONNSTATE = 17;
pub const RASCS_WaitForModemReset: tagRASCONNSTATE = 16;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RASCTRYINFO {
    pub dwSize: u32,
    pub dwCountryID: u32,
    pub dwNextCountryID: u32,
    pub dwCountryCode: u32,
    pub dwCountryNameOffset: u32,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct RASDEVSPECIFICINFO {
    pub dwSize: u32,
    pub pbDevSpecificInfo: *mut u8,
}
#[cfg(target_arch = "x86")]
impl Default for RASDEVSPECIFICINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct RASDEVSPECIFICINFO {
    pub dwSize: u32,
    pub pbDevSpecificInfo: *mut u8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for RASDEVSPECIFICINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RASDIALEVENT: windows_sys::core::PCSTR = windows_sys::core::s!("RasDialEvent");
pub type RASDIALFUNC = Option<unsafe extern "system" fn(param0: u32, param1: tagRASCONNSTATE, param2: u32)>;
pub type RASDIALFUNC1 = Option<unsafe extern "system" fn(param0: HRASCONN, param1: u32, param2: tagRASCONNSTATE, param3: u32, param4: u32)>;
pub type RASDIALFUNC2 = Option<unsafe extern "system" fn(param0: usize, param1: u32, param2: HRASCONN, param3: u32, param4: tagRASCONNSTATE, param5: u32, param6: u32) -> u32>;
pub const RASEAPF_Logon: u32 = 4;
pub const RASEAPF_NonInteractive: u32 = 2;
pub const RASEAPF_Preview: u32 = 8;
pub const RASEDM_DialAll: u32 = 1;
pub const RASEDM_DialAsNeeded: u32 = 2;
pub const RASEO2_AuthTypeIsOtp: u32 = 268435456;
pub const RASEO2_AutoTriggerCapable: u32 = 67108864;
pub const RASEO2_CacheCredentials: u32 = 33554432;
pub const RASEO2_DisableClassBasedStaticRoute: u32 = 524288;
pub const RASEO2_DisableIKENameEkuCheck: u32 = 262144;
pub const RASEO2_DisableMobility: u32 = 2097152;
pub const RASEO2_DisableNbtOverIP: u32 = 64;
pub const RASEO2_DontNegotiateMultilink: u32 = 4;
pub const RASEO2_DontUseRasCredentials: u32 = 8;
pub const RASEO2_IPv4ExplicitMetric: u32 = 65536;
pub const RASEO2_IPv6ExplicitMetric: u32 = 131072;
pub const RASEO2_IPv6RemoteDefaultGateway: u32 = 8192;
pub const RASEO2_IPv6SpecificNameServers: u32 = 4096;
pub const RASEO2_Internet: u32 = 32;
pub const RASEO2_IsAlwaysOn: u32 = 536870912;
pub const RASEO2_IsPrivateNetwork: u32 = 1073741824;
pub const RASEO2_IsThirdPartyProfile: u32 = 134217728;
pub const RASEO2_PlumbIKEv2TSAsRoutes: u32 = 2147483648;
pub const RASEO2_ReconnectIfDropped: u32 = 256;
pub const RASEO2_RegisterIpWithDNS: u32 = 16384;
pub const RASEO2_RequireMachineCertificates: u32 = 4194304;
pub const RASEO2_SecureClientForMSNet: u32 = 2;
pub const RASEO2_SecureFileAndPrint: u32 = 1;
pub const RASEO2_SecureRoutingCompartment: u32 = 1024;
pub const RASEO2_SharePhoneNumbers: u32 = 512;
pub const RASEO2_SpecificIPv6Addr: u32 = 1048576;
pub const RASEO2_UseDNSSuffixForRegistration: u32 = 32768;
pub const RASEO2_UseGlobalDeviceSettings: u32 = 128;
pub const RASEO2_UsePreSharedKey: u32 = 16;
pub const RASEO2_UsePreSharedKeyForIkev2Initiator: u32 = 8388608;
pub const RASEO2_UsePreSharedKeyForIkev2Responder: u32 = 16777216;
pub const RASEO2_UseTypicalSettings: u32 = 2048;
pub const RASEO_Custom: u32 = 1048576;
pub const RASEO_CustomScript: u32 = 2147483648;
pub const RASEO_DisableLcpExtensions: u32 = 32;
pub const RASEO_IpHeaderCompression: u32 = 8;
pub const RASEO_ModemLights: u32 = 256;
pub const RASEO_NetworkLogon: u32 = 8192;
pub const RASEO_PreviewDomain: u32 = 33554432;
pub const RASEO_PreviewPhoneNumber: u32 = 2097152;
pub const RASEO_PreviewUserPw: u32 = 16777216;
pub const RASEO_PromoteAlternates: u32 = 32768;
pub const RASEO_RemoteDefaultGateway: u32 = 16;
pub const RASEO_RequireCHAP: u32 = 134217728;
pub const RASEO_RequireDataEncryption: u32 = 4096;
pub const RASEO_RequireEAP: u32 = 131072;
pub const RASEO_RequireEncryptedPw: u32 = 1024;
pub const RASEO_RequireMsCHAP: u32 = 268435456;
pub const RASEO_RequireMsCHAP2: u32 = 536870912;
pub const RASEO_RequireMsEncryptedPw: u32 = 2048;
pub const RASEO_RequirePAP: u32 = 262144;
pub const RASEO_RequireSPAP: u32 = 524288;
pub const RASEO_RequireW95MSCHAP: u32 = 1073741824;
pub const RASEO_SecureLocalFiles: u32 = 65536;
pub const RASEO_SharedPhoneNumbers: u32 = 8388608;
pub const RASEO_ShowDialingProgress: u32 = 67108864;
pub const RASEO_SpecificIpAddr: u32 = 2;
pub const RASEO_SpecificNameServers: u32 = 4;
pub const RASEO_SwCompression: u32 = 512;
pub const RASEO_TerminalAfterDial: u32 = 128;
pub const RASEO_TerminalBeforeDial: u32 = 64;
pub const RASEO_UseCountryAndAreaCodes: u32 = 1;
pub const RASEO_UseLogonCredentials: u32 = 16384;
pub const RASET_Broadband: u32 = 5;
pub const RASET_Internet: u32 = 4;
pub const RASET_Phone: u32 = 1;
pub const RASET_Vpn: u32 = 2;
pub const RASFP_Ppp: u32 = 1;
pub const RASFP_Ras: u32 = 4;
pub const RASFP_Slip: u32 = 2;
pub const RASIDS_Disabled: u32 = 4294967295;
pub const RASIDS_UseGlobalValue: u32 = 0;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
#[derive(Clone, Copy)]
pub struct RASIKEV2_PROJECTION_INFO {
    pub dwIPv4NegotiationError: u32,
    pub ipv4Address: RASIPV4ADDR,
    pub ipv4ServerAddress: RASIPV4ADDR,
    pub dwIPv6NegotiationError: u32,
    pub ipv6Address: RASIPV6ADDR,
    pub ipv6ServerAddress: RASIPV6ADDR,
    pub dwPrefixLength: u32,
    pub dwAuthenticationProtocol: u32,
    pub dwEapTypeId: u32,
    pub dwFlags: u32,
    pub dwEncryptionMethod: u32,
    pub numIPv4ServerAddresses: u32,
    pub ipv4ServerAddresses: *mut RASIPV4ADDR,
    pub numIPv6ServerAddresses: u32,
    pub ipv6ServerAddresses: *mut RASIPV6ADDR,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
impl Default for RASIKEV2_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
#[derive(Clone, Copy)]
pub struct RASIKEV2_PROJECTION_INFO {
    pub dwIPv4NegotiationError: u32,
    pub ipv4Address: RASIPV4ADDR,
    pub ipv4ServerAddress: RASIPV4ADDR,
    pub dwIPv6NegotiationError: u32,
    pub ipv6Address: RASIPV6ADDR,
    pub ipv6ServerAddress: RASIPV6ADDR,
    pub dwPrefixLength: u32,
    pub dwAuthenticationProtocol: u32,
    pub dwEapTypeId: u32,
    pub dwFlags: u32,
    pub dwEncryptionMethod: u32,
    pub numIPv4ServerAddresses: u32,
    pub ipv4ServerAddresses: *mut RASIPV4ADDR,
    pub numIPv6ServerAddresses: u32,
    pub ipv6ServerAddresses: *mut RASIPV6ADDR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
impl Default for RASIKEV2_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RASIKEv2_AUTH_EAP: u32 = 2;
pub const RASIKEv2_AUTH_MACHINECERTIFICATES: u32 = 1;
pub const RASIKEv2_AUTH_PSK: u32 = 3;
pub const RASIKEv2_FLAGS_BEHIND_NAT: u32 = 2;
pub const RASIKEv2_FLAGS_MOBIKESUPPORTED: u32 = 1;
pub const RASIKEv2_FLAGS_SERVERBEHIND_NAT: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RASIPADDR {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
}
pub const RASIPO_VJ: u32 = 1;
#[cfg(feature = "inaddr")]
pub type RASIPV4ADDR = super::IN_ADDR;
#[cfg(feature = "in6addr")]
pub type RASIPV6ADDR = super::IN6_ADDR;
pub const RASLCPAD_CHAP_MD5: u32 = 5;
pub const RASLCPAD_CHAP_MS: u32 = 128;
pub const RASLCPAD_CHAP_MSV2: u32 = 129;
pub const RASLCPAP_CHAP: u32 = 49699;
pub const RASLCPAP_EAP: u32 = 49703;
pub const RASLCPAP_PAP: u32 = 49187;
pub const RASLCPAP_SPAP: u32 = 49191;
pub const RASLCPO_3_DES: u32 = 16;
pub const RASLCPO_ACFC: u32 = 2;
pub const RASLCPO_AES_128: u32 = 32;
pub const RASLCPO_AES_192: u32 = 128;
pub const RASLCPO_AES_256: u32 = 64;
pub const RASLCPO_DES_56: u32 = 8;
pub const RASLCPO_GCM_AES_128: u32 = 256;
pub const RASLCPO_GCM_AES_192: u32 = 512;
pub const RASLCPO_GCM_AES_256: u32 = 1024;
pub const RASLCPO_PFC: u32 = 1;
pub const RASLCPO_SSHF: u32 = 4;
pub const RASNP_Ip: u32 = 4;
pub const RASNP_Ipv6: u32 = 8;
pub const RASNP_Ipx: u32 = 2;
pub const RASNP_NetBEUI: u32 = 1;
#[repr(C)]
#[cfg(feature = "inaddr")]
#[derive(Clone, Copy)]
pub struct RASPPP_PROJECTION_INFO {
    pub dwIPv4NegotiationError: u32,
    pub ipv4Address: RASIPV4ADDR,
    pub ipv4ServerAddress: RASIPV4ADDR,
    pub dwIPv4Options: u32,
    pub dwIPv4ServerOptions: u32,
    pub dwIPv6NegotiationError: u32,
    pub bInterfaceIdentifier: [u8; 8],
    pub bServerInterfaceIdentifier: [u8; 8],
    pub fBundled: windows_sys::core::BOOL,
    pub fMultilink: windows_sys::core::BOOL,
    pub dwAuthenticationProtocol: u32,
    pub dwAuthenticationData: u32,
    pub dwServerAuthenticationProtocol: u32,
    pub dwServerAuthenticationData: u32,
    pub dwEapTypeId: u32,
    pub dwServerEapTypeId: u32,
    pub dwLcpOptions: u32,
    pub dwLcpServerOptions: u32,
    pub dwCcpError: u32,
    pub dwCcpCompressionAlgorithm: u32,
    pub dwCcpServerCompressionAlgorithm: u32,
    pub dwCcpOptions: u32,
    pub dwCcpServerOptions: u32,
}
#[cfg(feature = "inaddr")]
impl Default for RASPPP_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RASPROJECTION_INFO_TYPE = i32;
pub const RASP_Amb: tagRASPROJECTION = 65536;
pub const RASP_PppCcp: tagRASPROJECTION = 33021;
pub const RASP_PppIp: tagRASPROJECTION = 32801;
pub const RASP_PppIpv6: tagRASPROJECTION = 32855;
pub const RASP_PppIpx: tagRASPROJECTION = 32811;
pub const RASP_PppLcp: tagRASPROJECTION = 49185;
pub const RASP_PppNbf: tagRASPROJECTION = 32831;
pub const RASTUNNELENDPOINT_IPv4: u32 = 1;
pub const RASTUNNELENDPOINT_IPv6: u32 = 2;
pub const RASTUNNELENDPOINT_UNKNOWN: u32 = 0;
pub const RAS_MaxAreaCode: u32 = 10;
pub const RAS_MaxCallbackNumber: u32 = 128;
pub const RAS_MaxDeviceName: u32 = 128;
pub const RAS_MaxDeviceType: u32 = 16;
pub const RAS_MaxDnsSuffix: u32 = 256;
pub const RAS_MaxEntryName: u32 = 256;
pub const RAS_MaxFacilities: u32 = 200;
pub const RAS_MaxIDSize: u32 = 256;
pub const RAS_MaxIpAddress: u32 = 15;
pub const RAS_MaxIpxAddress: u32 = 21;
pub const RAS_MaxPadType: u32 = 32;
pub const RAS_MaxPhoneNumber: u32 = 128;
pub const RAS_MaxReplyMessage: u32 = 1024;
pub const RAS_MaxUserData: u32 = 200;
pub const RAS_MaxX25Address: u32 = 200;
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
#[derive(Clone, Copy)]
pub struct RAS_PROJECTION_INFO {
    pub version: RASAPIVERSION,
    pub r#type: RASPROJECTION_INFO_TYPE,
    pub Anonymous: RAS_PROJECTION_INFO_0,
}
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
impl Default for RAS_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
#[derive(Clone, Copy)]
pub union RAS_PROJECTION_INFO_0 {
    pub ppp: RASPPP_PROJECTION_INFO,
    pub ikev2: RASIKEV2_PROJECTION_INFO,
}
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
impl Default for RAS_PROJECTION_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RAS_STATS {
    pub dwSize: u32,
    pub dwBytesXmited: u32,
    pub dwBytesRcved: u32,
    pub dwFramesXmited: u32,
    pub dwFramesRcved: u32,
    pub dwCrcErr: u32,
    pub dwTimeoutErr: u32,
    pub dwAlignmentErr: u32,
    pub dwHardwareOverrunErr: u32,
    pub dwFramingErr: u32,
    pub dwBufferOverrunErr: u32,
    pub dwCompressionRatioIn: u32,
    pub dwCompressionRatioOut: u32,
    pub dwBps: u32,
    pub dwConnectDuration: u32,
}
pub const RCD_AllUsers: u32 = 1;
pub const RCD_Eap: u32 = 2;
pub const RCD_Logon: u32 = 4;
pub const RCD_SingleUser: u32 = 0;
pub const RDEOPT_CustomDial: u32 = 4096;
pub const RDEOPT_DisableConnectedUI: u32 = 64;
pub const RDEOPT_DisableReconnect: u32 = 256;
pub const RDEOPT_DisableReconnectUI: u32 = 128;
pub const RDEOPT_EapInfoCryptInCapable: u32 = 32768;
pub const RDEOPT_IgnoreModemSpeaker: u32 = 4;
pub const RDEOPT_IgnoreSoftwareCompression: u32 = 16;
pub const RDEOPT_InvokeAutoTriggerCredentialUI: u32 = 16384;
pub const RDEOPT_NoUser: u32 = 512;
pub const RDEOPT_PauseOnScript: u32 = 1024;
pub const RDEOPT_PausedStates: u32 = 2;
pub const RDEOPT_Router: u32 = 2048;
pub const RDEOPT_SetModemSpeaker: u32 = 8;
pub const RDEOPT_SetSoftwareCompression: u32 = 32;
pub const RDEOPT_UseCustomScripting: u32 = 8192;
pub const RDEOPT_UsePrefixSuffix: u32 = 1;
pub const REN_AllUsers: u32 = 1;
pub const REN_User: u32 = 0;
pub type RasCustomDeleteEntryNotifyFn = Option<unsafe extern "system" fn(lpszphonebook: windows_sys::core::PCWSTR, lpszentry: windows_sys::core::PCWSTR, dwflags: u32) -> u32>;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type RasCustomDialFn = Option<unsafe extern "system" fn(hinstdll: super::HINSTANCE, lprasdialextensions: *mut tagRASDIALEXTENSIONS, lpszphonebook: windows_sys::core::PCWSTR, lprasdialparams: *mut tagRASDIALPARAMSA, dwnotifiertype: u32, lpvnotifier: *mut core::ffi::c_void, lphrasconn: *mut HRASCONN, dwflags: u32) -> u32>;
pub type RasCustomHangUpFn = Option<unsafe extern "system" fn(hrasconn: HRASCONN) -> u32>;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
pub type RasCustomScriptExecuteFn = Option<unsafe extern "system" fn(hport: super::HANDLE, lpszphonebook: windows_sys::core::PCWSTR, lpszentryname: windows_sys::core::PCWSTR, pfnrasgetbuffer: PFNRASGETBUFFER, pfnrasfreebuffer: PFNRASFREEBUFFER, pfnrassendbuffer: PFNRASSENDBUFFER, pfnrasreceivebuffer: PFNRASRECEIVEBUFFER, pfnrasretrievebuffer: PFNRASRETRIEVEBUFFER, hwnd: super::HWND, prasdialparams: *mut tagRASDIALPARAMSA, pvreserved: *mut core::ffi::c_void) -> u32>;
pub const VS_Default: u32 = 0;
pub const VS_GREOnly: u32 = 9;
pub const VS_Ikev2First: u32 = 8;
pub const VS_Ikev2Only: u32 = 7;
pub const VS_Ikev2Sstp: u32 = 14;
pub const VS_L2tpFirst: u32 = 4;
pub const VS_L2tpOnly: u32 = 3;
pub const VS_L2tpSstp: u32 = 13;
pub const VS_PptpFirst: u32 = 2;
pub const VS_PptpOnly: u32 = 1;
pub const VS_PptpSstp: u32 = 12;
pub const VS_ProtocolList: u32 = 15;
pub const VS_SstpFirst: u32 = 6;
pub const VS_SstpOnly: u32 = 5;
pub const WM_RASDIALEVENT: u32 = 52429;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct tagRASADPARAMS {
    pub dwSize: u32,
    pub hwndOwner: super::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "windef")]
impl Default for tagRASADPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct tagRASADPARAMS {
    pub dwSize: u32,
    pub hwndOwner: super::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
impl Default for tagRASADPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tagRASAMBA {
    pub dwSize: u32,
    pub dwError: u32,
    pub szNetBiosError: [i8; 17],
    pub bLana: u8,
}
impl Default for tagRASAMBA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tagRASAMBW {
    pub dwSize: u32,
    pub dwError: u32,
    pub szNetBiosError: [u16; 17],
    pub bLana: u8,
}
impl Default for tagRASAMBW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tagRASAUTODIALENTRYA {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwDialingLocation: u32,
    pub szEntry: [i8; 257],
}
impl Default for tagRASAUTODIALENTRYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tagRASAUTODIALENTRYW {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwDialingLocation: u32,
    pub szEntry: [u16; 257],
}
impl Default for tagRASAUTODIALENTRYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct tagRASCOMMSETTINGS {
    pub dwSize: u32,
    pub bParity: u8,
    pub bStop: u8,
    pub bByteSize: u8,
    pub bAlign: u8,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct tagRASCONNA {
    pub dwSize: u32,
    pub hrasconn: HRASCONN,
    pub szEntryName: [i8; 257],
    pub szDeviceType: [i8; 17],
    pub szDeviceName: [i8; 129],
    pub szPhonebook: [i8; 260],
    pub dwSubEntry: u32,
    pub guidEntry: windows_sys::core::GUID,
    pub dwFlags: u32,
    pub luid: super::LUID,
    pub guidCorrelationId: windows_sys::core::GUID,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for tagRASCONNA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct tagRASCONNA {
    pub dwSize: u32,
    pub hrasconn: HRASCONN,
    pub szEntryName: [i8; 257],
    pub szDeviceType: [i8; 17],
    pub szDeviceName: [i8; 129],
    pub szPhonebook: [i8; 260],
    pub dwSubEntry: u32,
    pub guidEntry: windows_sys::core::GUID,
    pub dwFlags: u32,
    pub luid: super::LUID,
    pub guidCorrelationId: windows_sys::core::GUID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for tagRASCONNA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type tagRASCONNSTATE = i32;
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
#[derive(Clone, Copy)]
pub struct tagRASCONNSTATUSA {
    pub dwSize: u32,
    pub rasconnstate: tagRASCONNSTATE,
    pub dwError: u32,
    pub szDeviceType: [i8; 17],
    pub szDeviceName: [i8; 129],
    pub szPhoneNumber: [i8; 129],
    pub localEndPoint: tagRASTUNNELENDPOINT,
    pub remoteEndPoint: tagRASTUNNELENDPOINT,
    pub rasconnsubstate: tagRASCONNSUBSTATE,
}
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
impl Default for tagRASCONNSTATUSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
#[derive(Clone, Copy)]
pub struct tagRASCONNSTATUSW {
    pub dwSize: u32,
    pub rasconnstate: tagRASCONNSTATE,
    pub dwError: u32,
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szPhoneNumber: [u16; 129],
    pub localEndPoint: tagRASTUNNELENDPOINT,
    pub remoteEndPoint: tagRASTUNNELENDPOINT,
    pub rasconnsubstate: tagRASCONNSUBSTATE,
}
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
impl Default for tagRASCONNSTATUSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type tagRASCONNSUBSTATE = i32;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct tagRASCONNW {
    pub dwSize: u32,
    pub hrasconn: HRASCONN,
    pub szEntryName: [u16; 257],
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szPhonebook: [u16; 260],
    pub dwSubEntry: u32,
    pub guidEntry: windows_sys::core::GUID,
    pub dwFlags: u32,
    pub luid: super::LUID,
    pub guidCorrelationId: windows_sys::core::GUID,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for tagRASCONNW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct tagRASCONNW {
    pub dwSize: u32,
    pub hrasconn: HRASCONN,
    pub szEntryName: [u16; 257],
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szPhonebook: [u16; 260],
    pub dwSubEntry: u32,
    pub guidEntry: windows_sys::core::GUID,
    pub dwFlags: u32,
    pub luid: super::LUID,
    pub guidCorrelationId: windows_sys::core::GUID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for tagRASCONNW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tagRASCREDENTIALSA {
    pub dwSize: u32,
    pub dwMask: u32,
    pub szUserName: [i8; 257],
    pub szPassword: [i8; 257],
    pub szDomain: [i8; 16],
}
impl Default for tagRASCREDENTIALSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tagRASCREDENTIALSW {
    pub dwSize: u32,
    pub dwMask: u32,
    pub szUserName: [u16; 257],
    pub szPassword: [u16; 257],
    pub szDomain: [u16; 16],
}
impl Default for tagRASCREDENTIALSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct tagRASCUSTOMSCRIPTEXTENSIONS {
    pub dwSize: u32,
    pub pfnRasSetCommSettings: PFNRASSETCOMMSETTINGS,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct tagRASCUSTOMSCRIPTEXTENSIONS {
    pub dwSize: u32,
    pub pfnRasSetCommSettings: PFNRASSETCOMMSETTINGS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tagRASDEVINFOA {
    pub dwSize: u32,
    pub szDeviceType: [i8; 17],
    pub szDeviceName: [i8; 129],
}
impl Default for tagRASDEVINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tagRASDEVINFOW {
    pub dwSize: u32,
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
}
impl Default for tagRASDEVINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct tagRASDIALEXTENSIONS {
    pub dwSize: u32,
    pub dwfOptions: u32,
    pub hwndParent: super::HWND,
    pub reserved: usize,
    pub reserved1: usize,
    pub RasEapInfo: tagRASEAPINFO,
    pub fSkipPppAuth: windows_sys::core::BOOL,
    pub RasDevSpecificInfo: RASDEVSPECIFICINFO,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "windef")]
impl Default for tagRASDIALEXTENSIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct tagRASDIALEXTENSIONS {
    pub dwSize: u32,
    pub dwfOptions: u32,
    pub hwndParent: super::HWND,
    pub reserved: usize,
    pub reserved1: usize,
    pub RasEapInfo: tagRASEAPINFO,
    pub fSkipPppAuth: windows_sys::core::BOOL,
    pub RasDevSpecificInfo: RASDEVSPECIFICINFO,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
impl Default for tagRASDIALEXTENSIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct tagRASDIALPARAMSA {
    pub dwSize: u32,
    pub szEntryName: [i8; 257],
    pub szPhoneNumber: [i8; 129],
    pub szCallbackNumber: [i8; 129],
    pub szUserName: [i8; 257],
    pub szPassword: [i8; 257],
    pub szDomain: [i8; 16],
    pub dwSubEntry: u32,
    pub dwCallbackId: usize,
    pub dwIfIndex: u32,
    pub szEncPassword: windows_sys::core::PSTR,
}
#[cfg(target_arch = "x86")]
impl Default for tagRASDIALPARAMSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct tagRASDIALPARAMSA {
    pub dwSize: u32,
    pub szEntryName: [i8; 257],
    pub szPhoneNumber: [i8; 129],
    pub szCallbackNumber: [i8; 129],
    pub szUserName: [i8; 257],
    pub szPassword: [i8; 257],
    pub szDomain: [i8; 16],
    pub dwSubEntry: u32,
    pub dwCallbackId: usize,
    pub dwIfIndex: u32,
    pub szEncPassword: windows_sys::core::PSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for tagRASDIALPARAMSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct tagRASDIALPARAMSW {
    pub dwSize: u32,
    pub szEntryName: [u16; 257],
    pub szPhoneNumber: [u16; 129],
    pub szCallbackNumber: [u16; 129],
    pub szUserName: [u16; 257],
    pub szPassword: [u16; 257],
    pub szDomain: [u16; 16],
    pub dwSubEntry: u32,
    pub dwCallbackId: usize,
    pub dwIfIndex: u32,
    pub szEncPassword: windows_sys::core::PWSTR,
}
#[cfg(target_arch = "x86")]
impl Default for tagRASDIALPARAMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct tagRASDIALPARAMSW {
    pub dwSize: u32,
    pub szEntryName: [u16; 257],
    pub szPhoneNumber: [u16; 129],
    pub szCallbackNumber: [u16; 129],
    pub szUserName: [u16; 257],
    pub szPassword: [u16; 257],
    pub szDomain: [u16; 16],
    pub dwSubEntry: u32,
    pub dwCallbackId: usize,
    pub dwIfIndex: u32,
    pub szEncPassword: windows_sys::core::PWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for tagRASDIALPARAMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct tagRASEAPINFO {
    pub dwSizeofEapInfo: u32,
    pub pbEapInfo: *mut u8,
}
#[cfg(target_arch = "x86")]
impl Default for tagRASEAPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct tagRASEAPINFO {
    pub dwSizeofEapInfo: u32,
    pub pbEapInfo: *mut u8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for tagRASEAPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tagRASEAPUSERIDENTITYA {
    pub szUserName: [i8; 257],
    pub dwSizeofEapInfo: u32,
    pub pbEapInfo: [u8; 1],
}
impl Default for tagRASEAPUSERIDENTITYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tagRASEAPUSERIDENTITYW {
    pub szUserName: [u16; 257],
    pub dwSizeofEapInfo: u32,
    pub pbEapInfo: [u8; 1],
}
impl Default for tagRASEAPUSERIDENTITYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "in6addr")]
#[derive(Clone, Copy)]
pub struct tagRASENTRYA {
    pub dwSize: u32,
    pub dwfOptions: u32,
    pub dwCountryID: u32,
    pub dwCountryCode: u32,
    pub szAreaCode: [i8; 11],
    pub szLocalPhoneNumber: [i8; 129],
    pub dwAlternateOffset: u32,
    pub ipaddr: RASIPADDR,
    pub ipaddrDns: RASIPADDR,
    pub ipaddrDnsAlt: RASIPADDR,
    pub ipaddrWins: RASIPADDR,
    pub ipaddrWinsAlt: RASIPADDR,
    pub dwFrameSize: u32,
    pub dwfNetProtocols: u32,
    pub dwFramingProtocol: u32,
    pub szScript: [i8; 260],
    pub szAutodialDll: [i8; 260],
    pub szAutodialFunc: [i8; 260],
    pub szDeviceType: [i8; 17],
    pub szDeviceName: [i8; 129],
    pub szX25PadType: [i8; 33],
    pub szX25Address: [i8; 201],
    pub szX25Facilities: [i8; 201],
    pub szX25UserData: [i8; 201],
    pub dwChannels: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwSubEntries: u32,
    pub dwDialMode: u32,
    pub dwDialExtraPercent: u32,
    pub dwDialExtraSampleSeconds: u32,
    pub dwHangUpExtraPercent: u32,
    pub dwHangUpExtraSampleSeconds: u32,
    pub dwIdleDisconnectSeconds: u32,
    pub dwType: u32,
    pub dwEncryptionType: u32,
    pub dwCustomAuthKey: u32,
    pub guidId: windows_sys::core::GUID,
    pub szCustomDialDll: [i8; 260],
    pub dwVpnStrategy: u32,
    pub dwfOptions2: u32,
    pub dwfOptions3: u32,
    pub szDnsSuffix: [i8; 256],
    pub dwTcpWindowSize: u32,
    pub szPrerequisitePbk: [i8; 260],
    pub szPrerequisiteEntry: [i8; 257],
    pub dwRedialCount: u32,
    pub dwRedialPause: u32,
    pub ipv6addrDns: RASIPV6ADDR,
    pub ipv6addrDnsAlt: RASIPV6ADDR,
    pub dwIPv4InterfaceMetric: u32,
    pub dwIPv6InterfaceMetric: u32,
    pub ipv6addr: RASIPV6ADDR,
    pub dwIPv6PrefixLength: u32,
    pub dwNetworkOutageTime: u32,
    pub szIDi: [i8; 257],
    pub szIDr: [i8; 257],
    pub fIsImsConfig: windows_sys::core::BOOL,
    pub IdiType: IKEV2_ID_PAYLOAD_TYPE,
    pub IdrType: IKEV2_ID_PAYLOAD_TYPE,
    pub fDisableIKEv2Fragmentation: windows_sys::core::BOOL,
}
#[cfg(feature = "in6addr")]
impl Default for tagRASENTRYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tagRASENTRYNAMEA {
    pub dwSize: u32,
    pub szEntryName: [i8; 257],
    pub dwFlags: u32,
    pub szPhonebookPath: [i8; 261],
}
impl Default for tagRASENTRYNAMEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tagRASENTRYNAMEW {
    pub dwSize: u32,
    pub szEntryName: [u16; 257],
    pub dwFlags: u32,
    pub szPhonebookPath: [u16; 261],
}
impl Default for tagRASENTRYNAMEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "in6addr")]
#[derive(Clone, Copy)]
pub struct tagRASENTRYW {
    pub dwSize: u32,
    pub dwfOptions: u32,
    pub dwCountryID: u32,
    pub dwCountryCode: u32,
    pub szAreaCode: [u16; 11],
    pub szLocalPhoneNumber: [u16; 129],
    pub dwAlternateOffset: u32,
    pub ipaddr: RASIPADDR,
    pub ipaddrDns: RASIPADDR,
    pub ipaddrDnsAlt: RASIPADDR,
    pub ipaddrWins: RASIPADDR,
    pub ipaddrWinsAlt: RASIPADDR,
    pub dwFrameSize: u32,
    pub dwfNetProtocols: u32,
    pub dwFramingProtocol: u32,
    pub szScript: [u16; 260],
    pub szAutodialDll: [u16; 260],
    pub szAutodialFunc: [u16; 260],
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szX25PadType: [u16; 33],
    pub szX25Address: [u16; 201],
    pub szX25Facilities: [u16; 201],
    pub szX25UserData: [u16; 201],
    pub dwChannels: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwSubEntries: u32,
    pub dwDialMode: u32,
    pub dwDialExtraPercent: u32,
    pub dwDialExtraSampleSeconds: u32,
    pub dwHangUpExtraPercent: u32,
    pub dwHangUpExtraSampleSeconds: u32,
    pub dwIdleDisconnectSeconds: u32,
    pub dwType: u32,
    pub dwEncryptionType: u32,
    pub dwCustomAuthKey: u32,
    pub guidId: windows_sys::core::GUID,
    pub szCustomDialDll: [u16; 260],
    pub dwVpnStrategy: u32,
    pub dwfOptions2: u32,
    pub dwfOptions3: u32,
    pub szDnsSuffix: [u16; 256],
    pub dwTcpWindowSize: u32,
    pub szPrerequisitePbk: [u16; 260],
    pub szPrerequisiteEntry: [u16; 257],
    pub dwRedialCount: u32,
    pub dwRedialPause: u32,
    pub ipv6addrDns: RASIPV6ADDR,
    pub ipv6addrDnsAlt: RASIPV6ADDR,
    pub dwIPv4InterfaceMetric: u32,
    pub dwIPv6InterfaceMetric: u32,
    pub ipv6addr: RASIPV6ADDR,
    pub dwIPv6PrefixLength: u32,
    pub dwNetworkOutageTime: u32,
    pub szIDi: [u16; 257],
    pub szIDr: [u16; 257],
    pub fIsImsConfig: windows_sys::core::BOOL,
    pub IdiType: IKEV2_ID_PAYLOAD_TYPE,
    pub IdrType: IKEV2_ID_PAYLOAD_TYPE,
    pub fDisableIKEv2Fragmentation: windows_sys::core::BOOL,
}
#[cfg(feature = "in6addr")]
impl Default for tagRASENTRYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tagRASIPXW {
    pub dwSize: u32,
    pub dwError: u32,
    pub szIpxAddress: [u16; 22],
}
impl Default for tagRASIPXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct tagRASPPPCCP {
    pub dwSize: u32,
    pub dwError: u32,
    pub dwCompressionAlgorithm: u32,
    pub dwOptions: u32,
    pub dwServerCompressionAlgorithm: u32,
    pub dwServerOptions: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tagRASPPPIPA {
    pub dwSize: u32,
    pub dwError: u32,
    pub szIpAddress: [i8; 16],
    pub szServerIpAddress: [i8; 16],
    pub dwOptions: u32,
    pub dwServerOptions: u32,
}
impl Default for tagRASPPPIPA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tagRASPPPIPV6 {
    pub dwSize: u32,
    pub dwError: u32,
    pub bLocalInterfaceIdentifier: [u8; 8],
    pub bPeerInterfaceIdentifier: [u8; 8],
    pub bLocalCompressionProtocol: [u8; 2],
    pub bPeerCompressionProtocol: [u8; 2],
}
impl Default for tagRASPPPIPV6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tagRASPPPIPW {
    pub dwSize: u32,
    pub dwError: u32,
    pub szIpAddress: [u16; 16],
    pub szServerIpAddress: [u16; 16],
    pub dwOptions: u32,
    pub dwServerOptions: u32,
}
impl Default for tagRASPPPIPW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tagRASPPPIPXA {
    pub dwSize: u32,
    pub dwError: u32,
    pub szIpxAddress: [i8; 22],
}
impl Default for tagRASPPPIPXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tagRASPPPLCPA {
    pub dwSize: u32,
    pub fBundled: windows_sys::core::BOOL,
    pub dwError: u32,
    pub dwAuthenticationProtocol: u32,
    pub dwAuthenticationData: u32,
    pub dwEapTypeId: u32,
    pub dwServerAuthenticationProtocol: u32,
    pub dwServerAuthenticationData: u32,
    pub dwServerEapTypeId: u32,
    pub fMultilink: windows_sys::core::BOOL,
    pub dwTerminateReason: u32,
    pub dwServerTerminateReason: u32,
    pub szReplyMessage: [i8; 1024],
    pub dwOptions: u32,
    pub dwServerOptions: u32,
}
impl Default for tagRASPPPLCPA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tagRASPPPLCPW {
    pub dwSize: u32,
    pub fBundled: windows_sys::core::BOOL,
    pub dwError: u32,
    pub dwAuthenticationProtocol: u32,
    pub dwAuthenticationData: u32,
    pub dwEapTypeId: u32,
    pub dwServerAuthenticationProtocol: u32,
    pub dwServerAuthenticationData: u32,
    pub dwServerEapTypeId: u32,
    pub fMultilink: windows_sys::core::BOOL,
    pub dwTerminateReason: u32,
    pub dwServerTerminateReason: u32,
    pub szReplyMessage: [u16; 1024],
    pub dwOptions: u32,
    pub dwServerOptions: u32,
}
impl Default for tagRASPPPLCPW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tagRASPPPNBFA {
    pub dwSize: u32,
    pub dwError: u32,
    pub dwNetBiosError: u32,
    pub szNetBiosError: [i8; 17],
    pub szWorkstationName: [i8; 17],
    pub bLana: u8,
}
impl Default for tagRASPPPNBFA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tagRASPPPNBFW {
    pub dwSize: u32,
    pub dwError: u32,
    pub dwNetBiosError: u32,
    pub szNetBiosError: [u16; 17],
    pub szWorkstationName: [u16; 17],
    pub bLana: u8,
}
impl Default for tagRASPPPNBFW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type tagRASPROJECTION = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tagRASSUBENTRYA {
    pub dwSize: u32,
    pub dwfFlags: u32,
    pub szDeviceType: [i8; 17],
    pub szDeviceName: [i8; 129],
    pub szLocalPhoneNumber: [i8; 129],
    pub dwAlternateOffset: u32,
}
impl Default for tagRASSUBENTRYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tagRASSUBENTRYW {
    pub dwSize: u32,
    pub dwfFlags: u32,
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szLocalPhoneNumber: [u16; 129],
    pub dwAlternateOffset: u32,
}
impl Default for tagRASSUBENTRYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
#[derive(Clone, Copy)]
pub struct tagRASTUNNELENDPOINT {
    pub dwType: u32,
    pub Anonymous: tagRASTUNNELENDPOINT_0,
}
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
impl Default for tagRASTUNNELENDPOINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
#[derive(Clone, Copy)]
pub union tagRASTUNNELENDPOINT_0 {
    pub ipv4: RASIPV4ADDR,
    pub ipv6: RASIPV6ADDR,
}
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
impl Default for tagRASTUNNELENDPOINT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
#[derive(Clone, Copy)]
pub struct tagRASUPDATECONN {
    pub version: RASAPIVERSION,
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwIfIndex: u32,
    pub localEndPoint: tagRASTUNNELENDPOINT,
    pub remoteEndPoint: tagRASTUNNELENDPOINT,
}
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
impl Default for tagRASUPDATECONN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
