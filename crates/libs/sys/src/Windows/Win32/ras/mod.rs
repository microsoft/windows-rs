#[cfg(feature = "shared")]
windows_link::link!("rasapi32.dll" "system" fn RasClearConnectionStatistics(hrasconn : HRASCONN) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rasapi32.dll" "system" fn RasClearLinkStatistics(hrasconn : HRASCONN, dwsubentry : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("rasapi32.dll" "system" fn RasConnectionNotificationA(param0 : HRASCONN, param1 : super::HANDLE, param2 : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("rasapi32.dll" "system" fn RasConnectionNotificationW(param0 : HRASCONN, param1 : super::HANDLE, param2 : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("rasapi32.dll" "system" fn RasCreatePhonebookEntryA(param0 : super::HWND, param1 : windows_sys::core::PCSTR) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("rasapi32.dll" "system" fn RasCreatePhonebookEntryW(param0 : super::HWND, param1 : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rasapi32.dll" "system" fn RasDeleteEntryA(param0 : windows_sys::core::PCSTR, param1 : windows_sys::core::PCSTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rasapi32.dll" "system" fn RasDeleteEntryW(param0 : windows_sys::core::PCWSTR, param1 : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rasapi32.dll" "system" fn RasDeleteSubEntryA(pszphonebook : windows_sys::core::PCSTR, pszentry : windows_sys::core::PCSTR, dwsubentryid : super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rasapi32.dll" "system" fn RasDeleteSubEntryW(pszphonebook : windows_sys::core::PCWSTR, pszentry : windows_sys::core::PCWSTR, dwsubentryid : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("rasapi32.dll" "system" fn RasDialA(param0 : *const tagRASDIALEXTENSIONS, param1 : windows_sys::core::PCSTR, param2 : *const tagRASDIALPARAMSA, param3 : super::DWORD, param4 : super::LPVOID, param5 : *mut HRASCONN) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("rasapi32.dll" "system" fn RasDialW(param0 : *const tagRASDIALEXTENSIONS, param1 : windows_sys::core::PCWSTR, param2 : *const tagRASDIALPARAMSW, param3 : super::DWORD, param4 : super::LPVOID, param5 : *mut HRASCONN) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("rasapi32.dll" "system" fn RasEditPhonebookEntryA(param0 : super::HWND, param1 : windows_sys::core::PCSTR, param2 : windows_sys::core::PCSTR) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("rasapi32.dll" "system" fn RasEditPhonebookEntryW(param0 : super::HWND, param1 : windows_sys::core::PCWSTR, param2 : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("rasapi32.dll" "system" fn RasEnumAutodialAddressesA(lpprasautodialaddresses : *mut windows_sys::core::PSTR, lpdwcbrasautodialaddresses : super::LPDWORD, lpdwcrasautodialaddresses : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("rasapi32.dll" "system" fn RasEnumAutodialAddressesW(lpprasautodialaddresses : *mut windows_sys::core::PWSTR, lpdwcbrasautodialaddresses : super::LPDWORD, lpdwcrasautodialaddresses : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("rasapi32.dll" "system" fn RasEnumConnectionsA(param0 : *mut tagRASCONNA, param1 : super::LPDWORD, param2 : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("rasapi32.dll" "system" fn RasEnumConnectionsW(param0 : *mut tagRASCONNW, param1 : super::LPDWORD, param2 : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("rasapi32.dll" "system" fn RasEnumDevicesA(param0 : *mut tagRASDEVINFOA, param1 : super::LPDWORD, param2 : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("rasapi32.dll" "system" fn RasEnumDevicesW(param0 : *mut tagRASDEVINFOW, param1 : super::LPDWORD, param2 : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("rasapi32.dll" "system" fn RasEnumEntriesA(param0 : windows_sys::core::PCSTR, param1 : windows_sys::core::PCSTR, param2 : *mut tagRASENTRYNAMEA, param3 : super::LPDWORD, param4 : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("rasapi32.dll" "system" fn RasEnumEntriesW(param0 : windows_sys::core::PCWSTR, param1 : windows_sys::core::PCWSTR, param2 : *mut tagRASENTRYNAMEW, param3 : super::LPDWORD, param4 : super::LPDWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rasapi32.dll" "system" fn RasFreeEapUserIdentityA(praseapuseridentity : *const tagRASEAPUSERIDENTITYA));
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("rasapi32.dll" "system" fn RasFreeEapUserIdentityW(praseapuseridentity : *const tagRASEAPUSERIDENTITYW));
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("rasapi32.dll" "system" fn RasGetAutodialAddressA(param0 : windows_sys::core::PCSTR, param1 : super::LPDWORD, param2 : *mut tagRASAUTODIALENTRYA, param3 : super::LPDWORD, param4 : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("rasapi32.dll" "system" fn RasGetAutodialAddressW(param0 : windows_sys::core::PCWSTR, param1 : super::LPDWORD, param2 : *mut tagRASAUTODIALENTRYW, param3 : super::LPDWORD, param4 : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("rasapi32.dll" "system" fn RasGetAutodialEnableA(param0 : super::DWORD, param1 : super::LPBOOL) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("rasapi32.dll" "system" fn RasGetAutodialEnableW(param0 : super::DWORD, param1 : super::LPBOOL) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("rasapi32.dll" "system" fn RasGetAutodialParamA(param0 : super::DWORD, param1 : super::LPVOID, param2 : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("rasapi32.dll" "system" fn RasGetAutodialParamW(param0 : super::DWORD, param1 : super::LPVOID, param2 : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared"))]
windows_link::link!("rasapi32.dll" "system" fn RasGetConnectStatusA(param0 : HRASCONN, param1 : *mut tagRASCONNSTATUSA) -> super::DWORD);
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared", feature = "winnt"))]
windows_link::link!("rasapi32.dll" "system" fn RasGetConnectStatusW(param0 : HRASCONN, param1 : *mut tagRASCONNSTATUSW) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rasapi32.dll" "system" fn RasGetConnectionStatistics(hrasconn : HRASCONN, lpstatistics : *mut RAS_STATS) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("rasapi32.dll" "system" fn RasGetCountryInfoA(param0 : *mut RASCTRYINFO, param1 : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("rasapi32.dll" "system" fn RasGetCountryInfoW(param0 : *mut RASCTRYINFO, param1 : super::LPDWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rasapi32.dll" "system" fn RasGetCredentialsA(param0 : windows_sys::core::PCSTR, param1 : windows_sys::core::PCSTR, param2 : *mut tagRASCREDENTIALSA) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("rasapi32.dll" "system" fn RasGetCredentialsW(param0 : windows_sys::core::PCWSTR, param1 : windows_sys::core::PCWSTR, param2 : *mut tagRASCREDENTIALSW) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rasapi32.dll" "system" fn RasGetCustomAuthDataA(pszphonebook : windows_sys::core::PCSTR, pszentry : windows_sys::core::PCSTR, pbcustomauthdata : *mut super::BYTE, pdwsizeofcustomauthdata : *mut super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rasapi32.dll" "system" fn RasGetCustomAuthDataW(pszphonebook : windows_sys::core::PCWSTR, pszentry : windows_sys::core::PCWSTR, pbcustomauthdata : *mut super::BYTE, pdwsizeofcustomauthdata : *mut super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("rasapi32.dll" "system" fn RasGetEapUserDataA(htoken : super::HANDLE, pszphonebook : windows_sys::core::PCSTR, pszentry : windows_sys::core::PCSTR, pbeapdata : *mut super::BYTE, pdwsizeofeapdata : *mut super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("rasapi32.dll" "system" fn RasGetEapUserDataW(htoken : super::HANDLE, pszphonebook : windows_sys::core::PCWSTR, pszentry : windows_sys::core::PCWSTR, pbeapdata : *mut super::BYTE, pdwsizeofeapdata : *mut super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("rasapi32.dll" "system" fn RasGetEapUserIdentityA(pszphonebook : windows_sys::core::PCSTR, pszentry : windows_sys::core::PCSTR, dwflags : super::DWORD, hwnd : super::HWND, ppraseapuseridentity : *mut *mut tagRASEAPUSERIDENTITYA) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("rasapi32.dll" "system" fn RasGetEapUserIdentityW(pszphonebook : windows_sys::core::PCWSTR, pszentry : windows_sys::core::PCWSTR, dwflags : super::DWORD, hwnd : super::HWND, ppraseapuseridentity : *mut *mut tagRASEAPUSERIDENTITYW) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("rasapi32.dll" "system" fn RasGetEntryDialParamsA(param0 : windows_sys::core::PCSTR, param1 : *mut tagRASDIALPARAMSA, param2 : super::LPBOOL) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("rasapi32.dll" "system" fn RasGetEntryDialParamsW(param0 : windows_sys::core::PCWSTR, param1 : *mut tagRASDIALPARAMSW, param2 : super::LPBOOL) -> super::DWORD);
#[cfg(all(feature = "in6addr", feature = "minwindef", feature = "shared"))]
windows_link::link!("rasapi32.dll" "system" fn RasGetEntryPropertiesA(param0 : windows_sys::core::PCSTR, param1 : windows_sys::core::PCSTR, param2 : *mut tagRASENTRYA, param3 : super::LPDWORD, param4 : super::LPBYTE, param5 : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "in6addr", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("rasapi32.dll" "system" fn RasGetEntryPropertiesW(param0 : windows_sys::core::PCWSTR, param1 : windows_sys::core::PCWSTR, param2 : *mut tagRASENTRYW, param3 : super::LPDWORD, param4 : super::LPBYTE, param5 : super::LPDWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rasapi32.dll" "system" fn RasGetErrorStringA(resourceid : super::UINT, lpszstring : windows_sys::core::PSTR, inbufsize : super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rasapi32.dll" "system" fn RasGetErrorStringW(resourceid : super::UINT, lpszstring : windows_sys::core::PWSTR, inbufsize : super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rasapi32.dll" "system" fn RasGetLinkStatistics(hrasconn : HRASCONN, dwsubentry : super::DWORD, lpstatistics : *mut RAS_STATS) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rasapi32.dll" "system" fn RasGetPCscf(lpszpcscf : windows_sys::core::PWSTR) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("rasapi32.dll" "system" fn RasGetProjectionInfoA(param0 : HRASCONN, param1 : tagRASPROJECTION, param2 : super::LPVOID, param3 : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "minwindef", feature = "shared"))]
windows_link::link!("rasapi32.dll" "system" fn RasGetProjectionInfoEx(hrasconn : HRASCONN, prasprojection : PRAS_PROJECTION_INFO, lpdwsize : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("rasapi32.dll" "system" fn RasGetProjectionInfoW(param0 : HRASCONN, param1 : tagRASPROJECTION, param2 : super::LPVOID, param3 : super::LPDWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rasapi32.dll" "system" fn RasGetSubEntryHandleA(param0 : HRASCONN, param1 : super::DWORD, param2 : *mut HRASCONN) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rasapi32.dll" "system" fn RasGetSubEntryHandleW(param0 : HRASCONN, param1 : super::DWORD, param2 : *mut HRASCONN) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("rasapi32.dll" "system" fn RasGetSubEntryPropertiesA(param0 : windows_sys::core::PCSTR, param1 : windows_sys::core::PCSTR, param2 : super::DWORD, param3 : *mut tagRASSUBENTRYA, param4 : super::LPDWORD, param5 : super::LPBYTE, param6 : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("rasapi32.dll" "system" fn RasGetSubEntryPropertiesW(param0 : windows_sys::core::PCWSTR, param1 : windows_sys::core::PCWSTR, param2 : super::DWORD, param3 : *mut tagRASSUBENTRYW, param4 : super::LPDWORD, param5 : super::LPBYTE, param6 : super::LPDWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rasapi32.dll" "system" fn RasHangUpA(param0 : HRASCONN) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rasapi32.dll" "system" fn RasHangUpW(param0 : HRASCONN) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("rasapi32.dll" "system" fn RasInvokeEapUI(param0 : HRASCONN, param1 : super::DWORD, param2 : *const tagRASDIALEXTENSIONS, param3 : super::HWND) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rasapi32.dll" "system" fn RasRenameEntryA(param0 : windows_sys::core::PCSTR, param1 : windows_sys::core::PCSTR, param2 : windows_sys::core::PCSTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rasapi32.dll" "system" fn RasRenameEntryW(param0 : windows_sys::core::PCWSTR, param1 : windows_sys::core::PCWSTR, param2 : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rasapi32.dll" "system" fn RasSetAutodialAddressA(param0 : windows_sys::core::PCSTR, param1 : super::DWORD, param2 : *const tagRASAUTODIALENTRYA, param3 : super::DWORD, param4 : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("rasapi32.dll" "system" fn RasSetAutodialAddressW(param0 : windows_sys::core::PCWSTR, param1 : super::DWORD, param2 : *const tagRASAUTODIALENTRYW, param3 : super::DWORD, param4 : super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rasapi32.dll" "system" fn RasSetAutodialEnableA(param0 : super::DWORD, param1 : windows_sys::core::BOOL) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rasapi32.dll" "system" fn RasSetAutodialEnableW(param0 : super::DWORD, param1 : windows_sys::core::BOOL) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("rasapi32.dll" "system" fn RasSetAutodialParamA(param0 : super::DWORD, param1 : super::LPVOID, param2 : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("rasapi32.dll" "system" fn RasSetAutodialParamW(param0 : super::DWORD, param1 : super::LPVOID, param2 : super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rasapi32.dll" "system" fn RasSetCredentialsA(param0 : windows_sys::core::PCSTR, param1 : windows_sys::core::PCSTR, param2 : *const tagRASCREDENTIALSA, param3 : windows_sys::core::BOOL) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("rasapi32.dll" "system" fn RasSetCredentialsW(param0 : windows_sys::core::PCWSTR, param1 : windows_sys::core::PCWSTR, param2 : *const tagRASCREDENTIALSW, param3 : windows_sys::core::BOOL) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rasapi32.dll" "system" fn RasSetCustomAuthDataA(pszphonebook : windows_sys::core::PCSTR, pszentry : windows_sys::core::PCSTR, pbcustomauthdata : *const super::BYTE, dwsizeofcustomauthdata : super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rasapi32.dll" "system" fn RasSetCustomAuthDataW(pszphonebook : windows_sys::core::PCWSTR, pszentry : windows_sys::core::PCWSTR, pbcustomauthdata : *const super::BYTE, dwsizeofcustomauthdata : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("rasapi32.dll" "system" fn RasSetEapUserDataA(htoken : super::HANDLE, pszphonebook : windows_sys::core::PCSTR, pszentry : windows_sys::core::PCSTR, pbeapdata : *const super::BYTE, dwsizeofeapdata : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("rasapi32.dll" "system" fn RasSetEapUserDataW(htoken : super::HANDLE, pszphonebook : windows_sys::core::PCWSTR, pszentry : windows_sys::core::PCWSTR, pbeapdata : *const super::BYTE, dwsizeofeapdata : super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rasapi32.dll" "system" fn RasSetEntryDialParamsA(param0 : windows_sys::core::PCSTR, param1 : *const tagRASDIALPARAMSA, param2 : windows_sys::core::BOOL) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("rasapi32.dll" "system" fn RasSetEntryDialParamsW(param0 : windows_sys::core::PCWSTR, param1 : *const tagRASDIALPARAMSW, param2 : windows_sys::core::BOOL) -> super::DWORD);
#[cfg(all(feature = "in6addr", feature = "shared"))]
windows_link::link!("rasapi32.dll" "system" fn RasSetEntryPropertiesA(param0 : windows_sys::core::PCSTR, param1 : windows_sys::core::PCSTR, param2 : *const tagRASENTRYA, param3 : super::DWORD, param4 : super::LPBYTE, param5 : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "in6addr", feature = "shared", feature = "winnt"))]
windows_link::link!("rasapi32.dll" "system" fn RasSetEntryPropertiesW(param0 : windows_sys::core::PCWSTR, param1 : windows_sys::core::PCWSTR, param2 : *const tagRASENTRYW, param3 : super::DWORD, param4 : super::LPBYTE, param5 : super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rasapi32.dll" "system" fn RasSetSubEntryPropertiesA(param0 : windows_sys::core::PCSTR, param1 : windows_sys::core::PCSTR, param2 : super::DWORD, param3 : *const tagRASSUBENTRYA, param4 : super::DWORD, param5 : super::LPBYTE, param6 : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("rasapi32.dll" "system" fn RasSetSubEntryPropertiesW(param0 : windows_sys::core::PCWSTR, param1 : windows_sys::core::PCWSTR, param2 : super::DWORD, param3 : *const tagRASSUBENTRYW, param4 : super::DWORD, param5 : super::LPBYTE, param6 : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared"))]
windows_link::link!("rasapi32.dll" "system" fn RasUpdateConnection(hrasconn : HRASCONN, lprasupdateconn : *const tagRASUPDATECONN) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rasapi32.dll" "system" fn RasValidateEntryNameA(param0 : windows_sys::core::PCSTR, param1 : windows_sys::core::PCSTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rasapi32.dll" "system" fn RasValidateEntryNameW(param0 : windows_sys::core::PCWSTR, param1 : windows_sys::core::PCWSTR) -> super::DWORD);
pub const ET_None: i32 = 0;
pub const ET_Optional: i32 = 3;
pub const ET_Require: i32 = 1;
pub const ET_RequireMax: i32 = 2;
pub type HRASCONN = *mut HRASCONN__;
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct HRASCONN__ {
    pub unused: i32,
}
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
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type ORASADFUNC = Option<unsafe extern "system" fn(param0: super::HWND, param1: windows_sys::core::PSTR, param2: super::DWORD, param3: super::LPDWORD) -> windows_sys::core::BOOL>;
#[cfg(feature = "shared")]
pub type PFNRASFREEBUFFER = Option<unsafe extern "system" fn(pbufer: super::PBYTE) -> super::DWORD>;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PFNRASGETBUFFER = Option<unsafe extern "system" fn(ppbuffer: *mut super::PBYTE, pdwsize: super::PDWORD) -> super::DWORD>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PFNRASRECEIVEBUFFER = Option<unsafe extern "system" fn(hport: super::HANDLE, pbuffer: super::PBYTE, pdwsize: super::PDWORD, dwtimeout: super::DWORD, hevent: super::HANDLE) -> super::DWORD>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PFNRASRETRIEVEBUFFER = Option<unsafe extern "system" fn(hport: super::HANDLE, pbuffer: super::PBYTE, pdwsize: super::PDWORD) -> super::DWORD>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFNRASSENDBUFFER = Option<unsafe extern "system" fn(hport: super::HANDLE, pbuffer: super::PBYTE, dwsize: super::DWORD) -> super::DWORD>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFNRASSETCOMMSETTINGS = Option<unsafe extern "system" fn(hport: super::HANDLE, prascommsettings: *mut tagRASCOMMSETTINGS, pvreserved: super::PVOID) -> super::DWORD>;
#[cfg(feature = "shared")]
pub type PRASDEVSPECIFICINFO = *mut RASDEVSPECIFICINFO;
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared"))]
pub type PRASIKEV2_PROJECTION_INFO = *mut RASIKEV2_PROJECTION_INFO;
#[cfg(all(feature = "inaddr", feature = "shared"))]
pub type PRASPPP_PROJECTION_INFO = *mut RASPPP_PROJECTION_INFO;
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared"))]
pub type PRAS_PROJECTION_INFO = *mut RAS_PROJECTION_INFO;
#[cfg(feature = "shared")]
pub type PRAS_STATS = *mut RAS_STATS;
pub const PROJECTION_INFO_TYPE_IKEv2: RASPROJECTION_INFO_TYPE = 2;
pub const PROJECTION_INFO_TYPE_PPP: RASPROJECTION_INFO_TYPE = 1;
pub const RASADFLG_PositionDlg: i32 = 1;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type RASADFUNCA = Option<unsafe extern "system" fn(param0: windows_sys::core::PSTR, param1: windows_sys::core::PSTR, param2: *mut tagRASADPARAMS, param3: super::LPDWORD) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type RASADFUNCW = Option<unsafe extern "system" fn(param0: windows_sys::core::PWSTR, param1: windows_sys::core::PWSTR, param2: *mut tagRASADPARAMS, param3: super::LPDWORD) -> windows_sys::core::BOOL>;
pub const RASADP_ConnectionQueryTimeout: i32 = 4;
pub const RASADP_DisableConnectionQuery: i32 = 0;
pub const RASADP_FailedConnectionTimeout: i32 = 3;
pub const RASADP_LoginSessionDisable: i32 = 1;
pub const RASADP_SavedAddressesLimit: i32 = 2;
pub type RASAPIVERSION = i32;
pub const RASAPIVERSION_500: RASAPIVERSION = 1;
pub const RASAPIVERSION_501: RASAPIVERSION = 2;
pub const RASAPIVERSION_600: RASAPIVERSION = 3;
pub const RASAPIVERSION_601: RASAPIVERSION = 4;
pub const RASAPIVERSION_CURRENT: RASAPIVERSION = 4;
pub const RASCCPO_Compression: i32 = 1;
pub const RASCCPO_Encryption128bit: i32 = 64;
pub const RASCCPO_Encryption40bit: i32 = 32;
pub const RASCCPO_Encryption56bit: i32 = 16;
pub const RASCCPO_HistoryLess: i32 = 2;
pub const RASCF_AllUsers: i32 = 1;
pub const RASCF_GlobalCreds: i32 = 2;
pub const RASCF_OwnerKnown: i32 = 4;
pub const RASCF_OwnerMatch: i32 = 8;
pub const RASCM_DDMPreSharedKey: i32 = 64;
pub const RASCM_DefaultCreds: i32 = 8;
pub const RASCM_Domain: i32 = 4;
pub const RASCM_Password: i32 = 2;
pub const RASCM_PreSharedKey: i32 = 16;
pub const RASCM_ServerPreSharedKey: i32 = 32;
pub const RASCM_UserName: i32 = 1;
pub const RASCN_BandwidthAdded: i32 = 4;
pub const RASCN_BandwidthRemoved: i32 = 8;
pub const RASCN_Connection: i32 = 1;
pub const RASCN_Disconnection: i32 = 2;
pub const RASCN_Dormant: i32 = 16;
pub const RASCN_EPDGPacketArrival: i32 = 64;
pub const RASCN_ReConnection: i32 = 32;
pub const RASCSS_DONE: i32 = 8192;
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
pub const RASCS_DONE: i32 = 8192;
pub const RASCS_DeviceConnected: tagRASCONNSTATE = 3;
pub const RASCS_Disconnected: tagRASCONNSTATE = 8193;
pub const RASCS_Interactive: tagRASCONNSTATE = 4096;
pub const RASCS_InvokeEapUI: tagRASCONNSTATE = 4100;
pub const RASCS_LogonNetwork: tagRASCONNSTATE = 21;
pub const RASCS_OpenPort: tagRASCONNSTATE = 0;
pub const RASCS_PAUSED: i32 = 4096;
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
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct RASCTRYINFO {
    pub dwSize: super::DWORD,
    pub dwCountryID: super::DWORD,
    pub dwNextCountryID: super::DWORD,
    pub dwCountryCode: super::DWORD,
    pub dwCountryNameOffset: super::DWORD,
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct RASDEVSPECIFICINFO {
    pub dwSize: super::DWORD,
    pub pbDevSpecificInfo: *mut super::BYTE,
}
pub const RASDIALEVENT: windows_sys::core::PCSTR = windows_sys::core::s!("RasDialEvent");
#[cfg(feature = "shared")]
pub type RASDIALFUNC = Option<unsafe extern "system" fn(param0: super::UINT, param1: tagRASCONNSTATE, param2: super::DWORD)>;
#[cfg(feature = "shared")]
pub type RASDIALFUNC1 = Option<unsafe extern "system" fn(param0: HRASCONN, param1: super::UINT, param2: tagRASCONNSTATE, param3: super::DWORD, param4: super::DWORD)>;
#[cfg(feature = "shared")]
pub type RASDIALFUNC2 = Option<unsafe extern "system" fn(param0: super::ULONG_PTR, param1: super::DWORD, param2: HRASCONN, param3: super::UINT, param4: tagRASCONNSTATE, param5: super::DWORD, param6: super::DWORD) -> super::DWORD>;
pub const RASDT_Atm: windows_sys::core::PCSTR = windows_sys::core::s!("ATM");
pub const RASDT_FrameRelay: windows_sys::core::PCSTR = windows_sys::core::s!("FRAMERELAY");
pub const RASDT_Generic: windows_sys::core::PCSTR = windows_sys::core::s!("GENERIC");
pub const RASDT_Irda: windows_sys::core::PCSTR = windows_sys::core::s!("IRDA");
pub const RASDT_Isdn: windows_sys::core::PCSTR = windows_sys::core::s!("isdn");
pub const RASDT_Modem: windows_sys::core::PCSTR = windows_sys::core::s!("modem");
pub const RASDT_PPPoE: windows_sys::core::PCSTR = windows_sys::core::s!("PPPoE");
pub const RASDT_Pad: windows_sys::core::PCSTR = windows_sys::core::s!("pad");
pub const RASDT_Parallel: windows_sys::core::PCSTR = windows_sys::core::s!("PARALLEL");
pub const RASDT_SW56: windows_sys::core::PCSTR = windows_sys::core::s!("SW56");
pub const RASDT_Serial: windows_sys::core::PCSTR = windows_sys::core::s!("SERIAL");
pub const RASDT_Sonet: windows_sys::core::PCSTR = windows_sys::core::s!("SONET");
pub const RASDT_Vpn: windows_sys::core::PCSTR = windows_sys::core::s!("vpn");
pub const RASDT_X25: windows_sys::core::PCSTR = windows_sys::core::s!("x25");
pub const RASEAPF_Logon: i32 = 4;
pub const RASEAPF_NonInteractive: i32 = 2;
pub const RASEAPF_Preview: i32 = 8;
pub const RASEDM_DialAll: i32 = 1;
pub const RASEDM_DialAsNeeded: i32 = 2;
pub const RASEO2_AuthTypeIsOtp: i32 = 268435456;
pub const RASEO2_AutoTriggerCapable: i32 = 67108864;
pub const RASEO2_CacheCredentials: i32 = 33554432;
pub const RASEO2_DisableClassBasedStaticRoute: i32 = 524288;
pub const RASEO2_DisableIKENameEkuCheck: i32 = 262144;
pub const RASEO2_DisableMobility: i32 = 2097152;
pub const RASEO2_DisableNbtOverIP: i32 = 64;
pub const RASEO2_DontNegotiateMultilink: i32 = 4;
pub const RASEO2_DontUseRasCredentials: i32 = 8;
pub const RASEO2_IPv4ExplicitMetric: i32 = 65536;
pub const RASEO2_IPv6ExplicitMetric: i32 = 131072;
pub const RASEO2_IPv6RemoteDefaultGateway: i32 = 8192;
pub const RASEO2_IPv6SpecificNameServers: i32 = 4096;
pub const RASEO2_Internet: i32 = 32;
pub const RASEO2_IsAlwaysOn: i32 = 536870912;
pub const RASEO2_IsPrivateNetwork: i32 = 1073741824;
pub const RASEO2_IsThirdPartyProfile: i32 = 134217728;
pub const RASEO2_PlumbIKEv2TSAsRoutes: u32 = 2147483648;
pub const RASEO2_ReconnectIfDropped: i32 = 256;
pub const RASEO2_RegisterIpWithDNS: i32 = 16384;
pub const RASEO2_RequireMachineCertificates: i32 = 4194304;
pub const RASEO2_SecureClientForMSNet: i32 = 2;
pub const RASEO2_SecureFileAndPrint: i32 = 1;
pub const RASEO2_SecureRoutingCompartment: i32 = 1024;
pub const RASEO2_SharePhoneNumbers: i32 = 512;
pub const RASEO2_SpecificIPv6Addr: i32 = 1048576;
pub const RASEO2_UseDNSSuffixForRegistration: i32 = 32768;
pub const RASEO2_UseGlobalDeviceSettings: i32 = 128;
pub const RASEO2_UsePreSharedKey: i32 = 16;
pub const RASEO2_UsePreSharedKeyForIkev2Initiator: i32 = 8388608;
pub const RASEO2_UsePreSharedKeyForIkev2Responder: i32 = 16777216;
pub const RASEO2_UseTypicalSettings: i32 = 2048;
pub const RASEO_Custom: i32 = 1048576;
pub const RASEO_CustomScript: u32 = 2147483648;
pub const RASEO_DisableLcpExtensions: i32 = 32;
pub const RASEO_IpHeaderCompression: i32 = 8;
pub const RASEO_ModemLights: i32 = 256;
pub const RASEO_NetworkLogon: i32 = 8192;
pub const RASEO_PreviewDomain: i32 = 33554432;
pub const RASEO_PreviewPhoneNumber: i32 = 2097152;
pub const RASEO_PreviewUserPw: i32 = 16777216;
pub const RASEO_PromoteAlternates: i32 = 32768;
pub const RASEO_RemoteDefaultGateway: i32 = 16;
pub const RASEO_RequireCHAP: i32 = 134217728;
pub const RASEO_RequireDataEncryption: i32 = 4096;
pub const RASEO_RequireEAP: i32 = 131072;
pub const RASEO_RequireEncryptedPw: i32 = 1024;
pub const RASEO_RequireMsCHAP: i32 = 268435456;
pub const RASEO_RequireMsCHAP2: i32 = 536870912;
pub const RASEO_RequireMsEncryptedPw: i32 = 2048;
pub const RASEO_RequirePAP: i32 = 262144;
pub const RASEO_RequireSPAP: i32 = 524288;
pub const RASEO_RequireW95MSCHAP: i32 = 1073741824;
pub const RASEO_SecureLocalFiles: i32 = 65536;
pub const RASEO_SharedPhoneNumbers: i32 = 8388608;
pub const RASEO_ShowDialingProgress: i32 = 67108864;
pub const RASEO_SpecificIpAddr: i32 = 2;
pub const RASEO_SpecificNameServers: i32 = 4;
pub const RASEO_SwCompression: i32 = 512;
pub const RASEO_TerminalAfterDial: i32 = 128;
pub const RASEO_TerminalBeforeDial: i32 = 64;
pub const RASEO_UseCountryAndAreaCodes: i32 = 1;
pub const RASEO_UseLogonCredentials: i32 = 16384;
pub const RASET_Broadband: i32 = 5;
pub const RASET_Internet: i32 = 4;
pub const RASET_Phone: i32 = 1;
pub const RASET_Vpn: i32 = 2;
pub const RASFP_Ppp: i32 = 1;
pub const RASFP_Ras: i32 = 4;
pub const RASFP_Slip: i32 = 2;
pub const RASIDS_Disabled: u32 = 4294967295;
pub const RASIDS_UseGlobalValue: i32 = 0;
#[repr(C, packed(4))]
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct RASIKEV2_PROJECTION_INFO {
    pub dwIPv4NegotiationError: super::DWORD,
    pub ipv4Address: RASIPV4ADDR,
    pub ipv4ServerAddress: RASIPV4ADDR,
    pub dwIPv6NegotiationError: super::DWORD,
    pub ipv6Address: RASIPV6ADDR,
    pub ipv6ServerAddress: RASIPV6ADDR,
    pub dwPrefixLength: super::DWORD,
    pub dwAuthenticationProtocol: super::DWORD,
    pub dwEapTypeId: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwEncryptionMethod: super::DWORD,
    pub numIPv4ServerAddresses: super::DWORD,
    pub ipv4ServerAddresses: *mut RASIPV4ADDR,
    pub numIPv6ServerAddresses: super::DWORD,
    pub ipv6ServerAddresses: *mut RASIPV6ADDR,
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared"))]
impl Default for RASIKEV2_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RASIKEv2_AUTH_EAP: i32 = 2;
pub const RASIKEv2_AUTH_MACHINECERTIFICATES: i32 = 1;
pub const RASIKEv2_AUTH_PSK: i32 = 3;
pub const RASIKEv2_FLAGS_BEHIND_NAT: i32 = 2;
pub const RASIKEv2_FLAGS_MOBIKESUPPORTED: i32 = 1;
pub const RASIKEv2_FLAGS_SERVERBEHIND_NAT: i32 = 4;
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct RASIPADDR {
    pub a: super::BYTE,
    pub b: super::BYTE,
    pub c: super::BYTE,
    pub d: super::BYTE,
}
pub const RASIPO_VJ: i32 = 1;
#[cfg(all(feature = "inaddr", feature = "shared"))]
pub type RASIPV4ADDR = super::IN_ADDR;
#[cfg(all(feature = "in6addr", feature = "shared"))]
pub type RASIPV6ADDR = super::IN6_ADDR;
pub const RASLCPAD_CHAP_MD5: i32 = 5;
pub const RASLCPAD_CHAP_MS: i32 = 128;
pub const RASLCPAD_CHAP_MSV2: i32 = 129;
pub const RASLCPAP_CHAP: i32 = 49699;
pub const RASLCPAP_EAP: i32 = 49703;
pub const RASLCPAP_PAP: i32 = 49187;
pub const RASLCPAP_SPAP: i32 = 49191;
pub const RASLCPO_3_DES: i32 = 16;
pub const RASLCPO_ACFC: i32 = 2;
pub const RASLCPO_AES_128: i32 = 32;
pub const RASLCPO_AES_192: i32 = 128;
pub const RASLCPO_AES_256: i32 = 64;
pub const RASLCPO_DES_56: i32 = 8;
pub const RASLCPO_GCM_AES_128: i32 = 256;
pub const RASLCPO_GCM_AES_192: i32 = 512;
pub const RASLCPO_GCM_AES_256: i32 = 1024;
pub const RASLCPO_PFC: i32 = 1;
pub const RASLCPO_SSHF: i32 = 4;
pub const RASNP_Ip: i32 = 4;
pub const RASNP_Ipv6: i32 = 8;
pub const RASNP_Ipx: i32 = 2;
pub const RASNP_NetBEUI: i32 = 1;
#[repr(C, packed(4))]
#[cfg(all(feature = "inaddr", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct RASPPP_PROJECTION_INFO {
    pub dwIPv4NegotiationError: super::DWORD,
    pub ipv4Address: RASIPV4ADDR,
    pub ipv4ServerAddress: RASIPV4ADDR,
    pub dwIPv4Options: super::DWORD,
    pub dwIPv4ServerOptions: super::DWORD,
    pub dwIPv6NegotiationError: super::DWORD,
    pub bInterfaceIdentifier: [super::BYTE; 8],
    pub bServerInterfaceIdentifier: [super::BYTE; 8],
    pub fBundled: windows_sys::core::BOOL,
    pub fMultilink: windows_sys::core::BOOL,
    pub dwAuthenticationProtocol: super::DWORD,
    pub dwAuthenticationData: super::DWORD,
    pub dwServerAuthenticationProtocol: super::DWORD,
    pub dwServerAuthenticationData: super::DWORD,
    pub dwEapTypeId: super::DWORD,
    pub dwServerEapTypeId: super::DWORD,
    pub dwLcpOptions: super::DWORD,
    pub dwLcpServerOptions: super::DWORD,
    pub dwCcpError: super::DWORD,
    pub dwCcpCompressionAlgorithm: super::DWORD,
    pub dwCcpServerCompressionAlgorithm: super::DWORD,
    pub dwCcpOptions: super::DWORD,
    pub dwCcpServerOptions: super::DWORD,
}
#[cfg(all(feature = "inaddr", feature = "shared"))]
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
pub const RASTUNNELENDPOINT_IPv4: i32 = 1;
pub const RASTUNNELENDPOINT_IPv6: i32 = 2;
pub const RASTUNNELENDPOINT_UNKNOWN: i32 = 0;
pub const RAS_MaxAreaCode: i32 = 10;
pub const RAS_MaxCallbackNumber: i32 = 128;
pub const RAS_MaxDeviceName: i32 = 128;
pub const RAS_MaxDeviceType: i32 = 16;
pub const RAS_MaxDnsSuffix: i32 = 256;
pub const RAS_MaxEntryName: i32 = 256;
pub const RAS_MaxFacilities: i32 = 200;
pub const RAS_MaxIDSize: i32 = 256;
pub const RAS_MaxIpAddress: i32 = 15;
pub const RAS_MaxIpxAddress: i32 = 21;
pub const RAS_MaxPadType: i32 = 32;
pub const RAS_MaxPhoneNumber: i32 = 128;
pub const RAS_MaxReplyMessage: i32 = 1024;
pub const RAS_MaxUserData: i32 = 200;
pub const RAS_MaxX25Address: i32 = 200;
#[repr(C, packed(4))]
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct RAS_PROJECTION_INFO {
    pub version: RASAPIVERSION,
    pub r#type: RASPROJECTION_INFO_TYPE,
    pub Anonymous: RAS_PROJECTION_INFO_0,
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared"))]
impl Default for RAS_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared"))]
#[derive(Clone, Copy)]
pub union RAS_PROJECTION_INFO_0 {
    pub ppp: RASPPP_PROJECTION_INFO,
    pub ikev2: RASIKEV2_PROJECTION_INFO,
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared"))]
impl Default for RAS_PROJECTION_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct RAS_STATS {
    pub dwSize: super::DWORD,
    pub dwBytesXmited: super::DWORD,
    pub dwBytesRcved: super::DWORD,
    pub dwFramesXmited: super::DWORD,
    pub dwFramesRcved: super::DWORD,
    pub dwCrcErr: super::DWORD,
    pub dwTimeoutErr: super::DWORD,
    pub dwAlignmentErr: super::DWORD,
    pub dwHardwareOverrunErr: super::DWORD,
    pub dwFramingErr: super::DWORD,
    pub dwBufferOverrunErr: super::DWORD,
    pub dwCompressionRatioIn: super::DWORD,
    pub dwCompressionRatioOut: super::DWORD,
    pub dwBps: super::DWORD,
    pub dwConnectDuration: super::DWORD,
}
pub const RCD_AllUsers: i32 = 1;
pub const RCD_Eap: i32 = 2;
pub const RCD_Logon: i32 = 4;
pub const RCD_SingleUser: i32 = 0;
pub const RDEOPT_CustomDial: i32 = 4096;
pub const RDEOPT_DisableConnectedUI: i32 = 64;
pub const RDEOPT_DisableReconnect: i32 = 256;
pub const RDEOPT_DisableReconnectUI: i32 = 128;
pub const RDEOPT_EapInfoCryptInCapable: i32 = 32768;
pub const RDEOPT_IgnoreModemSpeaker: i32 = 4;
pub const RDEOPT_IgnoreSoftwareCompression: i32 = 16;
pub const RDEOPT_InvokeAutoTriggerCredentialUI: i32 = 16384;
pub const RDEOPT_NoUser: i32 = 512;
pub const RDEOPT_PauseOnScript: i32 = 1024;
pub const RDEOPT_PausedStates: i32 = 2;
pub const RDEOPT_Router: i32 = 2048;
pub const RDEOPT_SetModemSpeaker: i32 = 8;
pub const RDEOPT_SetSoftwareCompression: i32 = 32;
pub const RDEOPT_UseCustomScripting: i32 = 8192;
pub const RDEOPT_UsePrefixSuffix: i32 = 1;
pub const REN_AllUsers: i32 = 1;
pub const REN_User: i32 = 0;
#[cfg(feature = "shared")]
pub type RasCustomDeleteEntryNotifyFn = Option<unsafe extern "system" fn(lpszphonebook: windows_sys::core::PCWSTR, lpszentry: windows_sys::core::PCWSTR, dwflags: super::DWORD) -> super::DWORD>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type RasCustomDialFn = Option<unsafe extern "system" fn(hinstdll: super::HINSTANCE, lprasdialextensions: *mut tagRASDIALEXTENSIONS, lpszphonebook: windows_sys::core::PCWSTR, lprasdialparams: *mut tagRASDIALPARAMSA, dwnotifiertype: super::DWORD, lpvnotifier: super::LPVOID, lphrasconn: *mut HRASCONN, dwflags: super::DWORD) -> super::DWORD>;
#[cfg(feature = "shared")]
pub type RasCustomHangUpFn = Option<unsafe extern "system" fn(hrasconn: HRASCONN) -> super::DWORD>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
pub type RasCustomScriptExecuteFn = Option<unsafe extern "system" fn(hport: super::HANDLE, lpszphonebook: windows_sys::core::PCWSTR, lpszentryname: windows_sys::core::PCWSTR, pfnrasgetbuffer: PFNRASGETBUFFER, pfnrasfreebuffer: PFNRASFREEBUFFER, pfnrassendbuffer: PFNRASSENDBUFFER, pfnrasreceivebuffer: PFNRASRECEIVEBUFFER, pfnrasretrievebuffer: PFNRASRETRIEVEBUFFER, hwnd: super::HWND, prasdialparams: *mut tagRASDIALPARAMSA, pvreserved: super::PVOID) -> super::DWORD>;
pub const VS_Default: i32 = 0;
pub const VS_GREOnly: i32 = 9;
pub const VS_Ikev2First: i32 = 8;
pub const VS_Ikev2Only: i32 = 7;
pub const VS_Ikev2Sstp: i32 = 14;
pub const VS_L2tpFirst: i32 = 4;
pub const VS_L2tpOnly: i32 = 3;
pub const VS_L2tpSstp: i32 = 13;
pub const VS_PptpFirst: i32 = 2;
pub const VS_PptpOnly: i32 = 1;
pub const VS_PptpSstp: i32 = 12;
pub const VS_ProtocolList: i32 = 15;
pub const VS_SstpFirst: i32 = 6;
pub const VS_SstpOnly: i32 = 5;
pub const WM_RASDIALEVENT: i32 = 52429;
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct tagRASADPARAMS {
    pub dwSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub dwFlags: super::DWORD,
    pub xDlg: super::LONG,
    pub yDlg: super::LONG,
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct tagRASAMBA {
    pub dwSize: super::DWORD,
    pub dwError: super::DWORD,
    pub szNetBiosError: [super::CHAR; 17],
    pub bLana: super::BYTE,
}
#[cfg(feature = "shared")]
impl Default for tagRASAMBA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct tagRASAMBW {
    pub dwSize: super::DWORD,
    pub dwError: super::DWORD,
    pub szNetBiosError: [super::WCHAR; 17],
    pub bLana: super::BYTE,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for tagRASAMBW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct tagRASAUTODIALENTRYA {
    pub dwSize: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwDialingLocation: super::DWORD,
    pub szEntry: [super::CHAR; 257],
}
#[cfg(feature = "shared")]
impl Default for tagRASAUTODIALENTRYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct tagRASAUTODIALENTRYW {
    pub dwSize: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwDialingLocation: super::DWORD,
    pub szEntry: [super::WCHAR; 257],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for tagRASAUTODIALENTRYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct tagRASCOMMSETTINGS {
    pub dwSize: super::DWORD,
    pub bParity: super::BYTE,
    pub bStop: super::BYTE,
    pub bByteSize: super::BYTE,
    pub bAlign: super::BYTE,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct tagRASCONNA {
    pub dwSize: super::DWORD,
    pub hrasconn: HRASCONN,
    pub szEntryName: [super::CHAR; 257],
    pub szDeviceType: [super::CHAR; 17],
    pub szDeviceName: [super::CHAR; 129],
    pub szPhonebook: [super::CHAR; 260],
    pub dwSubEntry: super::DWORD,
    pub guidEntry: windows_sys::core::GUID,
    pub dwFlags: super::DWORD,
    pub luid: super::LUID,
    pub guidCorrelationId: windows_sys::core::GUID,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for tagRASCONNA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type tagRASCONNSTATE = i32;
#[repr(C, packed(4))]
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct tagRASCONNSTATUSA {
    pub dwSize: super::DWORD,
    pub rasconnstate: tagRASCONNSTATE,
    pub dwError: super::DWORD,
    pub szDeviceType: [super::CHAR; 17],
    pub szDeviceName: [super::CHAR; 129],
    pub szPhoneNumber: [super::CHAR; 129],
    pub localEndPoint: tagRASTUNNELENDPOINT,
    pub remoteEndPoint: tagRASTUNNELENDPOINT,
    pub rasconnsubstate: tagRASCONNSUBSTATE,
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared"))]
impl Default for tagRASCONNSTATUSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct tagRASCONNSTATUSW {
    pub dwSize: super::DWORD,
    pub rasconnstate: tagRASCONNSTATE,
    pub dwError: super::DWORD,
    pub szDeviceType: [super::WCHAR; 17],
    pub szDeviceName: [super::WCHAR; 129],
    pub szPhoneNumber: [super::WCHAR; 129],
    pub localEndPoint: tagRASTUNNELENDPOINT,
    pub remoteEndPoint: tagRASTUNNELENDPOINT,
    pub rasconnsubstate: tagRASCONNSUBSTATE,
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared", feature = "winnt"))]
impl Default for tagRASCONNSTATUSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type tagRASCONNSUBSTATE = i32;
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct tagRASCONNW {
    pub dwSize: super::DWORD,
    pub hrasconn: HRASCONN,
    pub szEntryName: [super::WCHAR; 257],
    pub szDeviceType: [super::WCHAR; 17],
    pub szDeviceName: [super::WCHAR; 129],
    pub szPhonebook: [super::WCHAR; 260],
    pub dwSubEntry: super::DWORD,
    pub guidEntry: windows_sys::core::GUID,
    pub dwFlags: super::DWORD,
    pub luid: super::LUID,
    pub guidCorrelationId: windows_sys::core::GUID,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for tagRASCONNW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct tagRASCREDENTIALSA {
    pub dwSize: super::DWORD,
    pub dwMask: super::DWORD,
    pub szUserName: [super::CHAR; 257],
    pub szPassword: [super::CHAR; 257],
    pub szDomain: [super::CHAR; 16],
}
#[cfg(feature = "shared")]
impl Default for tagRASCREDENTIALSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct tagRASCREDENTIALSW {
    pub dwSize: super::DWORD,
    pub dwMask: super::DWORD,
    pub szUserName: [super::WCHAR; 257],
    pub szPassword: [super::WCHAR; 257],
    pub szDomain: [super::WCHAR; 16],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for tagRASCREDENTIALSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct tagRASCUSTOMSCRIPTEXTENSIONS {
    pub dwSize: super::DWORD,
    pub pfnRasSetCommSettings: PFNRASSETCOMMSETTINGS,
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct tagRASDEVINFOA {
    pub dwSize: super::DWORD,
    pub szDeviceType: [super::CHAR; 17],
    pub szDeviceName: [super::CHAR; 129],
}
#[cfg(feature = "shared")]
impl Default for tagRASDEVINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct tagRASDEVINFOW {
    pub dwSize: super::DWORD,
    pub szDeviceType: [super::WCHAR; 17],
    pub szDeviceName: [super::WCHAR; 129],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for tagRASDEVINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct tagRASDIALEXTENSIONS {
    pub dwSize: super::DWORD,
    pub dwfOptions: super::DWORD,
    pub hwndParent: super::HWND,
    pub reserved: super::ULONG_PTR,
    pub reserved1: super::ULONG_PTR,
    pub RasEapInfo: tagRASEAPINFO,
    pub fSkipPppAuth: windows_sys::core::BOOL,
    pub RasDevSpecificInfo: RASDEVSPECIFICINFO,
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct tagRASDIALPARAMSA {
    pub dwSize: super::DWORD,
    pub szEntryName: [super::CHAR; 257],
    pub szPhoneNumber: [super::CHAR; 129],
    pub szCallbackNumber: [super::CHAR; 129],
    pub szUserName: [super::CHAR; 257],
    pub szPassword: [super::CHAR; 257],
    pub szDomain: [super::CHAR; 16],
    pub dwSubEntry: super::DWORD,
    pub dwCallbackId: super::ULONG_PTR,
    pub dwIfIndex: super::DWORD,
    pub szEncPassword: windows_sys::core::PSTR,
}
#[cfg(feature = "shared")]
impl Default for tagRASDIALPARAMSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct tagRASDIALPARAMSW {
    pub dwSize: super::DWORD,
    pub szEntryName: [super::WCHAR; 257],
    pub szPhoneNumber: [super::WCHAR; 129],
    pub szCallbackNumber: [super::WCHAR; 129],
    pub szUserName: [super::WCHAR; 257],
    pub szPassword: [super::WCHAR; 257],
    pub szDomain: [super::WCHAR; 16],
    pub dwSubEntry: super::DWORD,
    pub dwCallbackId: super::ULONG_PTR,
    pub dwIfIndex: super::DWORD,
    pub szEncPassword: windows_sys::core::PWSTR,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for tagRASDIALPARAMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct tagRASEAPINFO {
    pub dwSizeofEapInfo: super::DWORD,
    pub pbEapInfo: *mut super::BYTE,
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct tagRASEAPUSERIDENTITYA {
    pub szUserName: [super::CHAR; 257],
    pub dwSizeofEapInfo: super::DWORD,
    pub pbEapInfo: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for tagRASEAPUSERIDENTITYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct tagRASEAPUSERIDENTITYW {
    pub szUserName: [super::WCHAR; 257],
    pub dwSizeofEapInfo: super::DWORD,
    pub pbEapInfo: [super::BYTE; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for tagRASEAPUSERIDENTITYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "in6addr", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct tagRASENTRYA {
    pub dwSize: super::DWORD,
    pub dwfOptions: super::DWORD,
    pub dwCountryID: super::DWORD,
    pub dwCountryCode: super::DWORD,
    pub szAreaCode: [super::CHAR; 11],
    pub szLocalPhoneNumber: [super::CHAR; 129],
    pub dwAlternateOffset: super::DWORD,
    pub ipaddr: RASIPADDR,
    pub ipaddrDns: RASIPADDR,
    pub ipaddrDnsAlt: RASIPADDR,
    pub ipaddrWins: RASIPADDR,
    pub ipaddrWinsAlt: RASIPADDR,
    pub dwFrameSize: super::DWORD,
    pub dwfNetProtocols: super::DWORD,
    pub dwFramingProtocol: super::DWORD,
    pub szScript: [super::CHAR; 260],
    pub szAutodialDll: [super::CHAR; 260],
    pub szAutodialFunc: [super::CHAR; 260],
    pub szDeviceType: [super::CHAR; 17],
    pub szDeviceName: [super::CHAR; 129],
    pub szX25PadType: [super::CHAR; 33],
    pub szX25Address: [super::CHAR; 201],
    pub szX25Facilities: [super::CHAR; 201],
    pub szX25UserData: [super::CHAR; 201],
    pub dwChannels: super::DWORD,
    pub dwReserved1: super::DWORD,
    pub dwReserved2: super::DWORD,
    pub dwSubEntries: super::DWORD,
    pub dwDialMode: super::DWORD,
    pub dwDialExtraPercent: super::DWORD,
    pub dwDialExtraSampleSeconds: super::DWORD,
    pub dwHangUpExtraPercent: super::DWORD,
    pub dwHangUpExtraSampleSeconds: super::DWORD,
    pub dwIdleDisconnectSeconds: super::DWORD,
    pub dwType: super::DWORD,
    pub dwEncryptionType: super::DWORD,
    pub dwCustomAuthKey: super::DWORD,
    pub guidId: windows_sys::core::GUID,
    pub szCustomDialDll: [super::CHAR; 260],
    pub dwVpnStrategy: super::DWORD,
    pub dwfOptions2: super::DWORD,
    pub dwfOptions3: super::DWORD,
    pub szDnsSuffix: [super::CHAR; 256],
    pub dwTcpWindowSize: super::DWORD,
    pub szPrerequisitePbk: [super::CHAR; 260],
    pub szPrerequisiteEntry: [super::CHAR; 257],
    pub dwRedialCount: super::DWORD,
    pub dwRedialPause: super::DWORD,
    pub ipv6addrDns: RASIPV6ADDR,
    pub ipv6addrDnsAlt: RASIPV6ADDR,
    pub dwIPv4InterfaceMetric: super::DWORD,
    pub dwIPv6InterfaceMetric: super::DWORD,
    pub ipv6addr: RASIPV6ADDR,
    pub dwIPv6PrefixLength: super::DWORD,
    pub dwNetworkOutageTime: super::DWORD,
    pub szIDi: [super::CHAR; 257],
    pub szIDr: [super::CHAR; 257],
    pub fIsImsConfig: windows_sys::core::BOOL,
    pub IdiType: IKEV2_ID_PAYLOAD_TYPE,
    pub IdrType: IKEV2_ID_PAYLOAD_TYPE,
    pub fDisableIKEv2Fragmentation: windows_sys::core::BOOL,
}
#[cfg(all(feature = "in6addr", feature = "shared"))]
impl Default for tagRASENTRYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct tagRASENTRYNAMEA {
    pub dwSize: super::DWORD,
    pub szEntryName: [super::CHAR; 257],
    pub dwFlags: super::DWORD,
    pub szPhonebookPath: [super::CHAR; 261],
}
#[cfg(feature = "shared")]
impl Default for tagRASENTRYNAMEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct tagRASENTRYNAMEW {
    pub dwSize: super::DWORD,
    pub szEntryName: [super::WCHAR; 257],
    pub dwFlags: super::DWORD,
    pub szPhonebookPath: [super::WCHAR; 261],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for tagRASENTRYNAMEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "in6addr", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct tagRASENTRYW {
    pub dwSize: super::DWORD,
    pub dwfOptions: super::DWORD,
    pub dwCountryID: super::DWORD,
    pub dwCountryCode: super::DWORD,
    pub szAreaCode: [super::WCHAR; 11],
    pub szLocalPhoneNumber: [super::WCHAR; 129],
    pub dwAlternateOffset: super::DWORD,
    pub ipaddr: RASIPADDR,
    pub ipaddrDns: RASIPADDR,
    pub ipaddrDnsAlt: RASIPADDR,
    pub ipaddrWins: RASIPADDR,
    pub ipaddrWinsAlt: RASIPADDR,
    pub dwFrameSize: super::DWORD,
    pub dwfNetProtocols: super::DWORD,
    pub dwFramingProtocol: super::DWORD,
    pub szScript: [super::WCHAR; 260],
    pub szAutodialDll: [super::WCHAR; 260],
    pub szAutodialFunc: [super::WCHAR; 260],
    pub szDeviceType: [super::WCHAR; 17],
    pub szDeviceName: [super::WCHAR; 129],
    pub szX25PadType: [super::WCHAR; 33],
    pub szX25Address: [super::WCHAR; 201],
    pub szX25Facilities: [super::WCHAR; 201],
    pub szX25UserData: [super::WCHAR; 201],
    pub dwChannels: super::DWORD,
    pub dwReserved1: super::DWORD,
    pub dwReserved2: super::DWORD,
    pub dwSubEntries: super::DWORD,
    pub dwDialMode: super::DWORD,
    pub dwDialExtraPercent: super::DWORD,
    pub dwDialExtraSampleSeconds: super::DWORD,
    pub dwHangUpExtraPercent: super::DWORD,
    pub dwHangUpExtraSampleSeconds: super::DWORD,
    pub dwIdleDisconnectSeconds: super::DWORD,
    pub dwType: super::DWORD,
    pub dwEncryptionType: super::DWORD,
    pub dwCustomAuthKey: super::DWORD,
    pub guidId: windows_sys::core::GUID,
    pub szCustomDialDll: [super::WCHAR; 260],
    pub dwVpnStrategy: super::DWORD,
    pub dwfOptions2: super::DWORD,
    pub dwfOptions3: super::DWORD,
    pub szDnsSuffix: [super::WCHAR; 256],
    pub dwTcpWindowSize: super::DWORD,
    pub szPrerequisitePbk: [super::WCHAR; 260],
    pub szPrerequisiteEntry: [super::WCHAR; 257],
    pub dwRedialCount: super::DWORD,
    pub dwRedialPause: super::DWORD,
    pub ipv6addrDns: RASIPV6ADDR,
    pub ipv6addrDnsAlt: RASIPV6ADDR,
    pub dwIPv4InterfaceMetric: super::DWORD,
    pub dwIPv6InterfaceMetric: super::DWORD,
    pub ipv6addr: RASIPV6ADDR,
    pub dwIPv6PrefixLength: super::DWORD,
    pub dwNetworkOutageTime: super::DWORD,
    pub szIDi: [super::WCHAR; 257],
    pub szIDr: [super::WCHAR; 257],
    pub fIsImsConfig: windows_sys::core::BOOL,
    pub IdiType: IKEV2_ID_PAYLOAD_TYPE,
    pub IdrType: IKEV2_ID_PAYLOAD_TYPE,
    pub fDisableIKEv2Fragmentation: windows_sys::core::BOOL,
}
#[cfg(all(feature = "in6addr", feature = "shared", feature = "winnt"))]
impl Default for tagRASENTRYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct tagRASIPXW {
    pub dwSize: super::DWORD,
    pub dwError: super::DWORD,
    pub szIpxAddress: [super::WCHAR; 22],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for tagRASIPXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct tagRASPPPCCP {
    pub dwSize: super::DWORD,
    pub dwError: super::DWORD,
    pub dwCompressionAlgorithm: super::DWORD,
    pub dwOptions: super::DWORD,
    pub dwServerCompressionAlgorithm: super::DWORD,
    pub dwServerOptions: super::DWORD,
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct tagRASPPPIPA {
    pub dwSize: super::DWORD,
    pub dwError: super::DWORD,
    pub szIpAddress: [super::CHAR; 16],
    pub szServerIpAddress: [super::CHAR; 16],
    pub dwOptions: super::DWORD,
    pub dwServerOptions: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for tagRASPPPIPA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct tagRASPPPIPV6 {
    pub dwSize: super::DWORD,
    pub dwError: super::DWORD,
    pub bLocalInterfaceIdentifier: [super::BYTE; 8],
    pub bPeerInterfaceIdentifier: [super::BYTE; 8],
    pub bLocalCompressionProtocol: [super::BYTE; 2],
    pub bPeerCompressionProtocol: [super::BYTE; 2],
}
#[cfg(feature = "shared")]
impl Default for tagRASPPPIPV6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct tagRASPPPIPW {
    pub dwSize: super::DWORD,
    pub dwError: super::DWORD,
    pub szIpAddress: [super::WCHAR; 16],
    pub szServerIpAddress: [super::WCHAR; 16],
    pub dwOptions: super::DWORD,
    pub dwServerOptions: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for tagRASPPPIPW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct tagRASPPPIPXA {
    pub dwSize: super::DWORD,
    pub dwError: super::DWORD,
    pub szIpxAddress: [super::CHAR; 22],
}
#[cfg(feature = "shared")]
impl Default for tagRASPPPIPXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct tagRASPPPLCPA {
    pub dwSize: super::DWORD,
    pub fBundled: windows_sys::core::BOOL,
    pub dwError: super::DWORD,
    pub dwAuthenticationProtocol: super::DWORD,
    pub dwAuthenticationData: super::DWORD,
    pub dwEapTypeId: super::DWORD,
    pub dwServerAuthenticationProtocol: super::DWORD,
    pub dwServerAuthenticationData: super::DWORD,
    pub dwServerEapTypeId: super::DWORD,
    pub fMultilink: windows_sys::core::BOOL,
    pub dwTerminateReason: super::DWORD,
    pub dwServerTerminateReason: super::DWORD,
    pub szReplyMessage: [super::CHAR; 1024],
    pub dwOptions: super::DWORD,
    pub dwServerOptions: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for tagRASPPPLCPA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct tagRASPPPLCPW {
    pub dwSize: super::DWORD,
    pub fBundled: windows_sys::core::BOOL,
    pub dwError: super::DWORD,
    pub dwAuthenticationProtocol: super::DWORD,
    pub dwAuthenticationData: super::DWORD,
    pub dwEapTypeId: super::DWORD,
    pub dwServerAuthenticationProtocol: super::DWORD,
    pub dwServerAuthenticationData: super::DWORD,
    pub dwServerEapTypeId: super::DWORD,
    pub fMultilink: windows_sys::core::BOOL,
    pub dwTerminateReason: super::DWORD,
    pub dwServerTerminateReason: super::DWORD,
    pub szReplyMessage: [super::WCHAR; 1024],
    pub dwOptions: super::DWORD,
    pub dwServerOptions: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for tagRASPPPLCPW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct tagRASPPPNBFA {
    pub dwSize: super::DWORD,
    pub dwError: super::DWORD,
    pub dwNetBiosError: super::DWORD,
    pub szNetBiosError: [super::CHAR; 17],
    pub szWorkstationName: [super::CHAR; 17],
    pub bLana: super::BYTE,
}
#[cfg(feature = "shared")]
impl Default for tagRASPPPNBFA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct tagRASPPPNBFW {
    pub dwSize: super::DWORD,
    pub dwError: super::DWORD,
    pub dwNetBiosError: super::DWORD,
    pub szNetBiosError: [super::WCHAR; 17],
    pub szWorkstationName: [super::WCHAR; 17],
    pub bLana: super::BYTE,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for tagRASPPPNBFW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type tagRASPROJECTION = i32;
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct tagRASSUBENTRYA {
    pub dwSize: super::DWORD,
    pub dwfFlags: super::DWORD,
    pub szDeviceType: [super::CHAR; 17],
    pub szDeviceName: [super::CHAR; 129],
    pub szLocalPhoneNumber: [super::CHAR; 129],
    pub dwAlternateOffset: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for tagRASSUBENTRYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct tagRASSUBENTRYW {
    pub dwSize: super::DWORD,
    pub dwfFlags: super::DWORD,
    pub szDeviceType: [super::WCHAR; 17],
    pub szDeviceName: [super::WCHAR; 129],
    pub szLocalPhoneNumber: [super::WCHAR; 129],
    pub dwAlternateOffset: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for tagRASSUBENTRYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct tagRASTUNNELENDPOINT {
    pub dwType: super::DWORD,
    pub Anonymous: tagRASTUNNELENDPOINT_0,
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared"))]
impl Default for tagRASTUNNELENDPOINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared"))]
#[derive(Clone, Copy)]
pub union tagRASTUNNELENDPOINT_0 {
    pub ipv4: RASIPV4ADDR,
    pub ipv6: RASIPV6ADDR,
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared"))]
impl Default for tagRASTUNNELENDPOINT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct tagRASUPDATECONN {
    pub version: RASAPIVERSION,
    pub dwSize: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwIfIndex: super::DWORD,
    pub localEndPoint: tagRASTUNNELENDPOINT,
    pub remoteEndPoint: tagRASTUNNELENDPOINT,
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "shared"))]
impl Default for tagRASUPDATECONN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
