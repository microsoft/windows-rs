#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("ntdsapi.dll" "system" fn DsCrackSpn2A(pszspn : windows_sys::core::PCSTR, cspn : super::DWORD, pcserviceclass : super::LPDWORD, serviceclass : windows_sys::core::PSTR, pcservicename : super::LPDWORD, servicename : windows_sys::core::PSTR, pcinstancename : super::LPDWORD, instancename : windows_sys::core::PSTR, pinstanceport : *mut super::USHORT) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("ntdsapi.dll" "system" fn DsCrackSpn2W(pszspn : windows_sys::core::PCWSTR, cspn : super::DWORD, pcserviceclass : *mut super::DWORD, serviceclass : windows_sys::core::PWSTR, pcservicename : *mut super::DWORD, servicename : windows_sys::core::PWSTR, pcinstancename : *mut super::DWORD, instancename : windows_sys::core::PWSTR, pinstanceport : *mut super::USHORT) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("ntdsapi.dll" "system" fn DsCrackSpn3W(pszspn : windows_sys::core::PCWSTR, cspn : super::DWORD, pchostname : *mut super::DWORD, hostname : windows_sys::core::PWSTR, pcinstancename : *mut super::DWORD, instancename : windows_sys::core::PWSTR, pportnumber : *mut super::USHORT, pcdomainname : *mut super::DWORD, domainname : windows_sys::core::PWSTR, pcrealmname : *mut super::DWORD, realmname : windows_sys::core::PWSTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("ntdsapi.dll" "system" fn DsCrackSpn4W(pszspn : windows_sys::core::PCWSTR, cspn : super::DWORD, pchostname : *mut super::DWORD, hostname : windows_sys::core::PWSTR, pcinstancename : *mut super::DWORD, instancename : windows_sys::core::PWSTR, pcportname : *mut super::DWORD, portname : windows_sys::core::PWSTR, pcdomainname : *mut super::DWORD, domainname : windows_sys::core::PWSTR, pcrealmname : *mut super::DWORD, realmname : windows_sys::core::PWSTR) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("ntdsapi.dll" "system" fn DsCrackSpnA(pszspn : windows_sys::core::PCSTR, pcserviceclass : super::LPDWORD, serviceclass : windows_sys::core::PSTR, pcservicename : super::LPDWORD, servicename : windows_sys::core::PSTR, pcinstancename : super::LPDWORD, instancename : windows_sys::core::PSTR, pinstanceport : *mut super::USHORT) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("ntdsapi.dll" "system" fn DsCrackSpnW(pszspn : windows_sys::core::PCWSTR, pcserviceclass : *mut super::DWORD, serviceclass : windows_sys::core::PWSTR, pcservicename : *mut super::DWORD, servicename : windows_sys::core::PWSTR, pcinstancename : *mut super::DWORD, instancename : windows_sys::core::PWSTR, pinstanceport : *mut super::USHORT) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("ntdsapi.dll" "system" fn DsCrackUnquotedMangledRdnA(pszrdn : windows_sys::core::PCSTR, cchrdn : super::DWORD, pguid : *mut windows_sys::core::GUID, pedsmanglefor : *mut DS_MANGLE_FOR) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("ntdsapi.dll" "system" fn DsCrackUnquotedMangledRdnW(pszrdn : windows_sys::core::PCWSTR, cchrdn : super::DWORD, pguid : *mut windows_sys::core::GUID, pedsmanglefor : *mut DS_MANGLE_FOR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdsapi.dll" "system" fn DsGetRdnW(ppdn : *mut super::LPCWCH, pcdn : *mut super::DWORD, ppkey : *mut super::LPCWCH, pckey : *mut super::DWORD, ppval : *mut super::LPCWCH, pcval : *mut super::DWORD) -> super::DWORD);
windows_link::link!("ntdsapi.dll" "system" fn DsIsMangledDnA(pszdn : windows_sys::core::PCSTR, edsmanglefor : DS_MANGLE_FOR) -> windows_sys::core::BOOL);
windows_link::link!("ntdsapi.dll" "system" fn DsIsMangledDnW(pszdn : windows_sys::core::PCWSTR, edsmanglefor : DS_MANGLE_FOR) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("ntdsapi.dll" "system" fn DsIsMangledRdnValueA(pszrdn : windows_sys::core::PCSTR, crdn : super::DWORD, edsmanglefordesired : DS_MANGLE_FOR) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("ntdsapi.dll" "system" fn DsIsMangledRdnValueW(pszrdn : windows_sys::core::PCWSTR, crdn : super::DWORD, edsmanglefordesired : DS_MANGLE_FOR) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("ntdsapi.dll" "system" fn DsMakeSpnA(serviceclass : windows_sys::core::PCSTR, servicename : windows_sys::core::PCSTR, instancename : windows_sys::core::PCSTR, instanceport : super::USHORT, referrer : windows_sys::core::PCSTR, pcspnlength : *mut super::DWORD, pszspn : windows_sys::core::PSTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("ntdsapi.dll" "system" fn DsMakeSpnW(serviceclass : windows_sys::core::PCWSTR, servicename : windows_sys::core::PCWSTR, instancename : windows_sys::core::PCWSTR, instanceport : super::USHORT, referrer : windows_sys::core::PCWSTR, pcspnlength : *mut super::DWORD, pszspn : windows_sys::core::PWSTR) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdsapi.dll" "system" fn DsQuoteRdnValueA(cunquotedrdnvaluelength : super::DWORD, psunquotedrdnvalue : super::LPCCH, pcquotedrdnvaluelength : *mut super::DWORD, psquotedrdnvalue : super::LPCH) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdsapi.dll" "system" fn DsQuoteRdnValueW(cunquotedrdnvaluelength : super::DWORD, psunquotedrdnvalue : super::LPCWCH, pcquotedrdnvaluelength : *mut super::DWORD, psquotedrdnvalue : super::LPWCH) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdsapi.dll" "system" fn DsUnquoteRdnValueA(cquotedrdnvaluelength : super::DWORD, psquotedrdnvalue : super::LPCCH, pcunquotedrdnvaluelength : *mut super::DWORD, psunquotedrdnvalue : super::LPCH) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdsapi.dll" "system" fn DsUnquoteRdnValueW(cquotedrdnvaluelength : super::DWORD, psquotedrdnvalue : super::LPCWCH, pcunquotedrdnvaluelength : *mut super::DWORD, psunquotedrdnvalue : super::LPWCH) -> super::DWORD);
pub type DS_MANGLE_FOR = i32;
pub const DS_MANGLE_OBJECT_RDN_FOR_DELETION: DS_MANGLE_FOR = 1;
pub const DS_MANGLE_OBJECT_RDN_FOR_NAME_CONFLICT: DS_MANGLE_FOR = 2;
pub const DS_MANGLE_UNKNOWN: DS_MANGLE_FOR = 0;
