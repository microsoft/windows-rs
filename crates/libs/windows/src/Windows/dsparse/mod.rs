#[inline]
pub unsafe fn DsCrackSpn2A(pszspn: &[u8], pcserviceclass: Option<*mut u32>, serviceclass: Option<windows_core::PSTR>, pcservicename: Option<*mut u32>, servicename: Option<windows_core::PSTR>, pcinstancename: Option<*mut u32>, instancename: Option<windows_core::PSTR>, pinstanceport: Option<*mut u16>) -> u32 {
    windows_core::link!("ntdsapi.dll" "system" fn DsCrackSpn2A(pszspn : windows_core::PCSTR, cspn : u32, pcserviceclass : *mut u32, serviceclass : windows_core::PSTR, pcservicename : *mut u32, servicename : windows_core::PSTR, pcinstancename : *mut u32, instancename : windows_core::PSTR, pinstanceport : *mut u16) -> u32);
    unsafe { DsCrackSpn2A(core::mem::transmute(pszspn.as_ptr()), pszspn.len().try_into().unwrap(), pcserviceclass.unwrap_or(core::mem::zeroed()) as _, serviceclass.unwrap_or(core::mem::zeroed()) as _, pcservicename.unwrap_or(core::mem::zeroed()) as _, servicename.unwrap_or(core::mem::zeroed()) as _, pcinstancename.unwrap_or(core::mem::zeroed()) as _, instancename.unwrap_or(core::mem::zeroed()) as _, pinstanceport.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DsCrackSpn2W(pszspn: &[u16], pcserviceclass: Option<*mut u32>, serviceclass: Option<windows_core::PWSTR>, pcservicename: Option<*mut u32>, servicename: Option<windows_core::PWSTR>, pcinstancename: Option<*mut u32>, instancename: Option<windows_core::PWSTR>, pinstanceport: Option<*mut u16>) -> u32 {
    windows_core::link!("ntdsapi.dll" "system" fn DsCrackSpn2W(pszspn : windows_core::PCWSTR, cspn : u32, pcserviceclass : *mut u32, serviceclass : windows_core::PWSTR, pcservicename : *mut u32, servicename : windows_core::PWSTR, pcinstancename : *mut u32, instancename : windows_core::PWSTR, pinstanceport : *mut u16) -> u32);
    unsafe { DsCrackSpn2W(core::mem::transmute(pszspn.as_ptr()), pszspn.len().try_into().unwrap(), pcserviceclass.unwrap_or(core::mem::zeroed()) as _, serviceclass.unwrap_or(core::mem::zeroed()) as _, pcservicename.unwrap_or(core::mem::zeroed()) as _, servicename.unwrap_or(core::mem::zeroed()) as _, pcinstancename.unwrap_or(core::mem::zeroed()) as _, instancename.unwrap_or(core::mem::zeroed()) as _, pinstanceport.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DsCrackSpn3W<P0>(pszspn: P0, cspn: u32, pchostname: *mut u32, hostname: windows_core::PWSTR, pcinstancename: *mut u32, instancename: windows_core::PWSTR, pportnumber: *mut u16, pcdomainname: *mut u32, domainname: windows_core::PWSTR, pcrealmname: *mut u32, realmname: windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsCrackSpn3W(pszspn : windows_core::PCWSTR, cspn : u32, pchostname : *mut u32, hostname : windows_core::PWSTR, pcinstancename : *mut u32, instancename : windows_core::PWSTR, pportnumber : *mut u16, pcdomainname : *mut u32, domainname : windows_core::PWSTR, pcrealmname : *mut u32, realmname : windows_core::PWSTR) -> u32);
    unsafe { DsCrackSpn3W(pszspn.param().abi(), cspn, pchostname as _, core::mem::transmute(hostname), pcinstancename as _, core::mem::transmute(instancename), pportnumber as _, pcdomainname as _, core::mem::transmute(domainname), pcrealmname as _, core::mem::transmute(realmname)) }
}
#[inline]
pub unsafe fn DsCrackSpn4W<P0>(pszspn: P0, cspn: u32, pchostname: *mut u32, hostname: windows_core::PWSTR, pcinstancename: *mut u32, instancename: windows_core::PWSTR, pcportname: *mut u32, portname: windows_core::PWSTR, pcdomainname: *mut u32, domainname: windows_core::PWSTR, pcrealmname: *mut u32, realmname: windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsCrackSpn4W(pszspn : windows_core::PCWSTR, cspn : u32, pchostname : *mut u32, hostname : windows_core::PWSTR, pcinstancename : *mut u32, instancename : windows_core::PWSTR, pcportname : *mut u32, portname : windows_core::PWSTR, pcdomainname : *mut u32, domainname : windows_core::PWSTR, pcrealmname : *mut u32, realmname : windows_core::PWSTR) -> u32);
    unsafe { DsCrackSpn4W(pszspn.param().abi(), cspn, pchostname as _, core::mem::transmute(hostname), pcinstancename as _, core::mem::transmute(instancename), pcportname as _, core::mem::transmute(portname), pcdomainname as _, core::mem::transmute(domainname), pcrealmname as _, core::mem::transmute(realmname)) }
}
#[inline]
pub unsafe fn DsCrackSpnA<P0>(pszspn: P0, pcserviceclass: Option<*mut u32>, serviceclass: Option<windows_core::PSTR>, pcservicename: Option<*mut u32>, servicename: Option<windows_core::PSTR>, pcinstancename: Option<*mut u32>, instancename: Option<windows_core::PSTR>, pinstanceport: Option<*mut u16>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsCrackSpnA(pszspn : windows_core::PCSTR, pcserviceclass : *mut u32, serviceclass : windows_core::PSTR, pcservicename : *mut u32, servicename : windows_core::PSTR, pcinstancename : *mut u32, instancename : windows_core::PSTR, pinstanceport : *mut u16) -> u32);
    unsafe { DsCrackSpnA(pszspn.param().abi(), pcserviceclass.unwrap_or(core::mem::zeroed()) as _, serviceclass.unwrap_or(core::mem::zeroed()) as _, pcservicename.unwrap_or(core::mem::zeroed()) as _, servicename.unwrap_or(core::mem::zeroed()) as _, pcinstancename.unwrap_or(core::mem::zeroed()) as _, instancename.unwrap_or(core::mem::zeroed()) as _, pinstanceport.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DsCrackSpnW<P0>(pszspn: P0, pcserviceclass: Option<*mut u32>, serviceclass: Option<windows_core::PWSTR>, pcservicename: Option<*mut u32>, servicename: Option<windows_core::PWSTR>, pcinstancename: Option<*mut u32>, instancename: Option<windows_core::PWSTR>, pinstanceport: Option<*mut u16>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsCrackSpnW(pszspn : windows_core::PCWSTR, pcserviceclass : *mut u32, serviceclass : windows_core::PWSTR, pcservicename : *mut u32, servicename : windows_core::PWSTR, pcinstancename : *mut u32, instancename : windows_core::PWSTR, pinstanceport : *mut u16) -> u32);
    unsafe { DsCrackSpnW(pszspn.param().abi(), pcserviceclass.unwrap_or(core::mem::zeroed()) as _, serviceclass.unwrap_or(core::mem::zeroed()) as _, pcservicename.unwrap_or(core::mem::zeroed()) as _, servicename.unwrap_or(core::mem::zeroed()) as _, pcinstancename.unwrap_or(core::mem::zeroed()) as _, instancename.unwrap_or(core::mem::zeroed()) as _, pinstanceport.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DsCrackUnquotedMangledRdnA(pszrdn: &[u8], pguid: Option<*mut windows_core::GUID>, pedsmanglefor: Option<*mut DS_MANGLE_FOR>) -> windows_core::BOOL {
    windows_core::link!("ntdsapi.dll" "system" fn DsCrackUnquotedMangledRdnA(pszrdn : windows_core::PCSTR, cchrdn : u32, pguid : *mut windows_core::GUID, pedsmanglefor : *mut DS_MANGLE_FOR) -> windows_core::BOOL);
    unsafe { DsCrackUnquotedMangledRdnA(core::mem::transmute(pszrdn.as_ptr()), pszrdn.len().try_into().unwrap(), pguid.unwrap_or(core::mem::zeroed()) as _, pedsmanglefor.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DsCrackUnquotedMangledRdnW(pszrdn: &[u16], pguid: Option<*mut windows_core::GUID>, pedsmanglefor: Option<*mut DS_MANGLE_FOR>) -> windows_core::BOOL {
    windows_core::link!("ntdsapi.dll" "system" fn DsCrackUnquotedMangledRdnW(pszrdn : windows_core::PCWSTR, cchrdn : u32, pguid : *mut windows_core::GUID, pedsmanglefor : *mut DS_MANGLE_FOR) -> windows_core::BOOL);
    unsafe { DsCrackUnquotedMangledRdnW(core::mem::transmute(pszrdn.as_ptr()), pszrdn.len().try_into().unwrap(), pguid.unwrap_or(core::mem::zeroed()) as _, pedsmanglefor.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DsGetRdnW(ppdn: *mut super::winnt::LPCWCH, pcdn: *mut u32, ppkey: *mut super::winnt::LPCWCH, pckey: *mut u32, ppval: *mut super::winnt::LPCWCH, pcval: *mut u32) -> u32 {
    windows_core::link!("ntdsapi.dll" "system" fn DsGetRdnW(ppdn : *mut super::winnt::LPCWCH, pcdn : *mut u32, ppkey : *mut super::winnt::LPCWCH, pckey : *mut u32, ppval : *mut super::winnt::LPCWCH, pcval : *mut u32) -> u32);
    unsafe { DsGetRdnW(ppdn as _, pcdn as _, ppkey as _, pckey as _, ppval as _, pcval as _) }
}
#[inline]
pub unsafe fn DsIsMangledDnA<P0>(pszdn: P0, edsmanglefor: DS_MANGLE_FOR) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsIsMangledDnA(pszdn : windows_core::PCSTR, edsmanglefor : DS_MANGLE_FOR) -> windows_core::BOOL);
    unsafe { DsIsMangledDnA(pszdn.param().abi(), edsmanglefor) }
}
#[inline]
pub unsafe fn DsIsMangledDnW<P0>(pszdn: P0, edsmanglefor: DS_MANGLE_FOR) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsIsMangledDnW(pszdn : windows_core::PCWSTR, edsmanglefor : DS_MANGLE_FOR) -> windows_core::BOOL);
    unsafe { DsIsMangledDnW(pszdn.param().abi(), edsmanglefor) }
}
#[inline]
pub unsafe fn DsIsMangledRdnValueA(pszrdn: &[u8], edsmanglefordesired: DS_MANGLE_FOR) -> windows_core::BOOL {
    windows_core::link!("ntdsapi.dll" "system" fn DsIsMangledRdnValueA(pszrdn : windows_core::PCSTR, crdn : u32, edsmanglefordesired : DS_MANGLE_FOR) -> windows_core::BOOL);
    unsafe { DsIsMangledRdnValueA(core::mem::transmute(pszrdn.as_ptr()), pszrdn.len().try_into().unwrap(), edsmanglefordesired) }
}
#[inline]
pub unsafe fn DsIsMangledRdnValueW(pszrdn: &[u16], edsmanglefordesired: DS_MANGLE_FOR) -> windows_core::BOOL {
    windows_core::link!("ntdsapi.dll" "system" fn DsIsMangledRdnValueW(pszrdn : windows_core::PCWSTR, crdn : u32, edsmanglefordesired : DS_MANGLE_FOR) -> windows_core::BOOL);
    unsafe { DsIsMangledRdnValueW(core::mem::transmute(pszrdn.as_ptr()), pszrdn.len().try_into().unwrap(), edsmanglefordesired) }
}
#[inline]
pub unsafe fn DsMakeSpnA<P0, P1, P2, P4>(serviceclass: P0, servicename: P1, instancename: P2, instanceport: u16, referrer: P4, pcspnlength: *mut u32, pszspn: Option<windows_core::PSTR>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsMakeSpnA(serviceclass : windows_core::PCSTR, servicename : windows_core::PCSTR, instancename : windows_core::PCSTR, instanceport : u16, referrer : windows_core::PCSTR, pcspnlength : *mut u32, pszspn : windows_core::PSTR) -> u32);
    unsafe { DsMakeSpnA(serviceclass.param().abi(), servicename.param().abi(), instancename.param().abi(), instanceport, referrer.param().abi(), pcspnlength as _, pszspn.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DsMakeSpnW<P0, P1, P2, P4>(serviceclass: P0, servicename: P1, instancename: P2, instanceport: u16, referrer: P4, pcspnlength: *mut u32, pszspn: Option<windows_core::PWSTR>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdsapi.dll" "system" fn DsMakeSpnW(serviceclass : windows_core::PCWSTR, servicename : windows_core::PCWSTR, instancename : windows_core::PCWSTR, instanceport : u16, referrer : windows_core::PCWSTR, pcspnlength : *mut u32, pszspn : windows_core::PWSTR) -> u32);
    unsafe { DsMakeSpnW(serviceclass.param().abi(), servicename.param().abi(), instancename.param().abi(), instanceport, referrer.param().abi(), pcspnlength as _, pszspn.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DsQuoteRdnValueA(psunquotedrdnvalue: &[i8], pcquotedrdnvaluelength: *mut u32, psquotedrdnvalue: *mut i8) -> u32 {
    windows_core::link!("ntdsapi.dll" "system" fn DsQuoteRdnValueA(cunquotedrdnvaluelength : u32, psunquotedrdnvalue : *const i8, pcquotedrdnvaluelength : *mut u32, psquotedrdnvalue : *mut i8) -> u32);
    unsafe { DsQuoteRdnValueA(psunquotedrdnvalue.len().try_into().unwrap(), psunquotedrdnvalue.as_ptr(), pcquotedrdnvaluelength as _, psquotedrdnvalue as _) }
}
#[inline]
pub unsafe fn DsQuoteRdnValueW(psunquotedrdnvalue: &[u16], pcquotedrdnvaluelength: *mut u32, psquotedrdnvalue: *mut u16) -> u32 {
    windows_core::link!("ntdsapi.dll" "system" fn DsQuoteRdnValueW(cunquotedrdnvaluelength : u32, psunquotedrdnvalue : *const u16, pcquotedrdnvaluelength : *mut u32, psquotedrdnvalue : *mut u16) -> u32);
    unsafe { DsQuoteRdnValueW(psunquotedrdnvalue.len().try_into().unwrap(), psunquotedrdnvalue.as_ptr(), pcquotedrdnvaluelength as _, psquotedrdnvalue as _) }
}
#[inline]
pub unsafe fn DsUnquoteRdnValueA(psquotedrdnvalue: &[i8], pcunquotedrdnvaluelength: *mut u32, psunquotedrdnvalue: *mut i8) -> u32 {
    windows_core::link!("ntdsapi.dll" "system" fn DsUnquoteRdnValueA(cquotedrdnvaluelength : u32, psquotedrdnvalue : *const i8, pcunquotedrdnvaluelength : *mut u32, psunquotedrdnvalue : *mut i8) -> u32);
    unsafe { DsUnquoteRdnValueA(psquotedrdnvalue.len().try_into().unwrap(), psquotedrdnvalue.as_ptr(), pcunquotedrdnvaluelength as _, psunquotedrdnvalue as _) }
}
#[inline]
pub unsafe fn DsUnquoteRdnValueW(psquotedrdnvalue: &[u16], pcunquotedrdnvaluelength: *mut u32, psunquotedrdnvalue: *mut u16) -> u32 {
    windows_core::link!("ntdsapi.dll" "system" fn DsUnquoteRdnValueW(cquotedrdnvaluelength : u32, psquotedrdnvalue : *const u16, pcunquotedrdnvaluelength : *mut u32, psunquotedrdnvalue : *mut u16) -> u32);
    unsafe { DsUnquoteRdnValueW(psquotedrdnvalue.len().try_into().unwrap(), psquotedrdnvalue.as_ptr(), pcunquotedrdnvaluelength as _, psunquotedrdnvalue as _) }
}
pub type DS_MANGLE_FOR = i32;
pub const DS_MANGLE_OBJECT_RDN_FOR_DELETION: DS_MANGLE_FOR = 1;
pub const DS_MANGLE_OBJECT_RDN_FOR_NAME_CONFLICT: DS_MANGLE_FOR = 2;
pub const DS_MANGLE_UNKNOWN: DS_MANGLE_FOR = 0;
