#[cfg(all(feature = "iads", feature = "oaidl"))]
#[inline]
pub unsafe fn ADsBuildEnumerator(padscontainer: &Option<super::IADsContainer>, ppenumvariant: *mut Option<super::IEnumVARIANT>) -> windows_core::HRESULT {
    windows_core::link!("activeds.dll" "system" fn ADsBuildEnumerator(padscontainer : *mut core::ffi::c_void, ppenumvariant : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ADsBuildEnumerator(core::mem::transmute_copy(padscontainer), ppenumvariant as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn ADsBuildVarArrayInt(lpdwobjecttypes: super::LPDWORD, dwobjecttypes: super::DWORD, pvar: *mut super::VARIANT) -> windows_core::HRESULT {
    windows_core::link!("activeds.dll" "system" fn ADsBuildVarArrayInt(lpdwobjecttypes : super::LPDWORD, dwobjecttypes : super::DWORD, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe { ADsBuildVarArrayInt(lpdwobjecttypes as _, dwobjecttypes, pvar as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn ADsBuildVarArrayStr(lpppathnames: *const windows_core::PCWSTR, dwpathnames: super::DWORD, pvar: *mut super::VARIANT) -> windows_core::HRESULT {
    windows_core::link!("activeds.dll" "system" fn ADsBuildVarArrayStr(lpppathnames : *const windows_core::PCWSTR, dwpathnames : super::DWORD, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe { ADsBuildVarArrayStr(lpppathnames, dwpathnames, pvar as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn ADsDecodeBinaryData(szsrcdata: &windows_core::PCWSTR, ppbdestdata: *mut super::PBYTE, pdwdestlen: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("activeds.dll" "system" fn ADsDecodeBinaryData(szsrcdata : windows_core::PCWSTR, ppbdestdata : *mut super::PBYTE, pdwdestlen : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { ADsDecodeBinaryData(core::mem::transmute_copy(szsrcdata), ppbdestdata as _, pdwdestlen as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn ADsEncodeBinaryData(pbsrcdata: super::PBYTE, dwsrclen: super::DWORD, ppszdestdata: *mut windows_core::PWSTR) -> windows_core::HRESULT {
    windows_core::link!("activeds.dll" "system" fn ADsEncodeBinaryData(pbsrcdata : super::PBYTE, dwsrclen : super::DWORD, ppszdestdata : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { ADsEncodeBinaryData(pbsrcdata as _, dwsrclen, ppszdestdata as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn ADsEnumerateNext(penumvariant: &Option<super::IEnumVARIANT>, celements: super::ULONG, pvar: *mut super::VARIANT, pcelementsfetched: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("activeds.dll" "system" fn ADsEnumerateNext(penumvariant : *mut core::ffi::c_void, celements : super::ULONG, pvar : *mut super::VARIANT, pcelementsfetched : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { ADsEnumerateNext(core::mem::transmute_copy(penumvariant), celements, pvar as _, pcelementsfetched as _) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn ADsFreeEnumerator(penumvariant: &Option<super::IEnumVARIANT>) -> windows_core::HRESULT {
    windows_core::link!("activeds.dll" "system" fn ADsFreeEnumerator(penumvariant : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ADsFreeEnumerator(core::mem::transmute_copy(penumvariant)) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn ADsGetLastError(lperror: super::LPDWORD, lperrorbuf: windows_core::PWSTR, dwerrorbuflen: super::DWORD, lpnamebuf: windows_core::PWSTR, dwnamebuflen: super::DWORD) -> windows_core::HRESULT {
    windows_core::link!("activeds.dll" "system" fn ADsGetLastError(lperror : super::LPDWORD, lperrorbuf : windows_core::PWSTR, dwerrorbuflen : super::DWORD, lpnamebuf : windows_core::PWSTR, dwnamebuflen : super::DWORD) -> windows_core::HRESULT);
    unsafe { ADsGetLastError(lperror as _, lperrorbuf, dwerrorbuflen, lpnamebuf, dwnamebuflen) }
}
#[cfg(feature = "guiddef")]
#[inline]
pub unsafe fn ADsGetObject(lpszpathname: &windows_core::PCWSTR, riid: *const super::IID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("activeds.dll" "system" fn ADsGetObject(lpszpathname : windows_core::PCWSTR, riid : *const super::IID, ppobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ADsGetObject(core::mem::transmute_copy(lpszpathname), riid as _, ppobject as _) }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[inline]
pub unsafe fn ADsOpenObject(lpszpathname: &windows_core::PCWSTR, lpszusername: &windows_core::PCWSTR, lpszpassword: &windows_core::PCWSTR, dwreserved: super::DWORD, riid: *const super::IID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("activeds.dll" "system" fn ADsOpenObject(lpszpathname : windows_core::PCWSTR, lpszusername : windows_core::PCWSTR, lpszpassword : windows_core::PCWSTR, dwreserved : super::DWORD, riid : *const super::IID, ppobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ADsOpenObject(core::mem::transmute_copy(lpszpathname), core::mem::transmute_copy(lpszusername), core::mem::transmute_copy(lpszpassword), dwreserved, riid as _, ppobject as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn ADsSetLastError(dwerr: super::DWORD, pszerror: &windows_core::PCWSTR, pszprovider: &windows_core::PCWSTR) {
    windows_core::link!("activeds.dll" "system" fn ADsSetLastError(dwerr : super::DWORD, pszerror : windows_core::PCWSTR, pszprovider : windows_core::PCWSTR));
    unsafe { ADsSetLastError(dwerr, core::mem::transmute_copy(pszerror), core::mem::transmute_copy(pszprovider)) }
}
#[cfg(all(feature = "iads", feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AdsFreeAdsValues(padsvalues: super::PADSVALUE, dwnumvalues: super::DWORD) {
    windows_core::link!("activeds.dll" "system" fn AdsFreeAdsValues(padsvalues : super::PADSVALUE, dwnumvalues : super::DWORD));
    unsafe { AdsFreeAdsValues(padsvalues as _, dwnumvalues) }
}
#[cfg(all(feature = "iads", feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn AdsTypeToPropVariant(padsvalues: super::PADSVALUE, dwnumvalues: super::DWORD, pvariant: *mut super::VARIANT) -> windows_core::HRESULT {
    windows_core::link!("activeds.dll" "system" fn AdsTypeToPropVariant(padsvalues : super::PADSVALUE, dwnumvalues : super::DWORD, pvariant : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe { AdsTypeToPropVariant(padsvalues as _, dwnumvalues, pvariant as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn AllocADsMem(cb: super::DWORD) -> super::LPVOID {
    windows_core::link!("activeds.dll" "system" fn AllocADsMem(cb : super::DWORD) -> super::LPVOID);
    unsafe { AllocADsMem(cb) }
}
#[inline]
pub unsafe fn AllocADsStr(pstr: &windows_core::PCWSTR) -> windows_core::PWSTR {
    windows_core::link!("activeds.dll" "system" fn AllocADsStr(pstr : windows_core::PCWSTR) -> windows_core::PWSTR);
    unsafe { AllocADsStr(core::mem::transmute_copy(pstr)) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn BinarySDToSecurityDescriptor(psecuritydescriptor: super::PSECURITY_DESCRIPTOR, pvarsec: *mut super::VARIANT, pszservername: &windows_core::PCWSTR, username: &windows_core::PCWSTR, password: &windows_core::PCWSTR, dwflags: super::DWORD) -> windows_core::HRESULT {
    windows_core::link!("activeds.dll" "system" fn BinarySDToSecurityDescriptor(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, pvarsec : *mut super::VARIANT, pszservername : windows_core::PCWSTR, username : windows_core::PCWSTR, password : windows_core::PCWSTR, dwflags : super::DWORD) -> windows_core::HRESULT);
    unsafe { BinarySDToSecurityDescriptor(psecuritydescriptor as _, pvarsec as _, core::mem::transmute_copy(pszservername), core::mem::transmute_copy(username), core::mem::transmute_copy(password), dwflags) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn FreeADsMem(pmem: super::LPVOID) -> windows_core::BOOL {
    windows_core::link!("activeds.dll" "system" fn FreeADsMem(pmem : super::LPVOID) -> windows_core::BOOL);
    unsafe { FreeADsMem(pmem as _) }
}
#[inline]
pub unsafe fn FreeADsStr<P0>(pstr: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("activeds.dll" "system" fn FreeADsStr(pstr : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { FreeADsStr(pstr.param().abi()) }
}
#[cfg(all(feature = "iads", feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToAdsType(pvariant: *mut super::VARIANT, dwnumvariant: super::DWORD, ppadsvalues: *mut super::PADSVALUE, pdwnumvalues: super::PDWORD) -> windows_core::HRESULT {
    windows_core::link!("activeds.dll" "system" fn PropVariantToAdsType(pvariant : *mut super::VARIANT, dwnumvariant : super::DWORD, ppadsvalues : *mut super::PADSVALUE, pdwnumvalues : super::PDWORD) -> windows_core::HRESULT);
    unsafe { PropVariantToAdsType(pvariant as _, dwnumvariant, ppadsvalues as _, pdwnumvalues as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn ReallocADsMem(poldmem: super::LPVOID, cbold: super::DWORD, cbnew: super::DWORD) -> super::LPVOID {
    windows_core::link!("activeds.dll" "system" fn ReallocADsMem(poldmem : super::LPVOID, cbold : super::DWORD, cbnew : super::DWORD) -> super::LPVOID);
    unsafe { ReallocADsMem(poldmem as _, cbold, cbnew) }
}
#[inline]
pub unsafe fn ReallocADsStr<P1>(ppstr: *mut windows_core::PWSTR, pstr: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("activeds.dll" "system" fn ReallocADsStr(ppstr : *mut windows_core::PWSTR, pstr : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { ReallocADsStr(ppstr as _, pstr.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn SecurityDescriptorToBinarySD(vvarsecdes: &super::VARIANT, ppsecuritydescriptor: *mut super::PSECURITY_DESCRIPTOR, pdwsdlength: super::PDWORD, pszservername: &windows_core::PCWSTR, username: &windows_core::PCWSTR, password: &windows_core::PCWSTR, dwflags: super::DWORD) -> windows_core::HRESULT {
    windows_core::link!("activeds.dll" "system" fn SecurityDescriptorToBinarySD(vvarsecdes : super::VARIANT, ppsecuritydescriptor : *mut super::PSECURITY_DESCRIPTOR, pdwsdlength : super::PDWORD, pszservername : windows_core::PCWSTR, username : windows_core::PCWSTR, password : windows_core::PCWSTR, dwflags : super::DWORD) -> windows_core::HRESULT);
    unsafe { SecurityDescriptorToBinarySD(core::mem::transmute_copy(vvarsecdes), ppsecuritydescriptor as _, pdwsdlength as _, core::mem::transmute_copy(pszservername), core::mem::transmute_copy(username), core::mem::transmute_copy(password), dwflags) }
}
