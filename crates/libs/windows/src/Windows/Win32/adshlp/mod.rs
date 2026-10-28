#[cfg(all(feature = "iads", feature = "oaidl"))]
#[inline]
pub unsafe fn ADsBuildEnumerator<P0>(padscontainer: P0, ppenumvariant: *mut Option<super::IEnumVARIANT>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IADsContainer>,
{
    windows_core::link!("activeds.dll" "system" fn ADsBuildEnumerator(padscontainer : *mut core::ffi::c_void, ppenumvariant : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ADsBuildEnumerator(padscontainer.param().abi(), ppenumvariant as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn ADsBuildVarArrayInt(lpdwobjecttypes: super::LPDWORD, dwobjecttypes: super::DWORD, pvar: *mut super::VARIANT) -> windows_core::HRESULT {
    windows_core::link!("activeds.dll" "system" fn ADsBuildVarArrayInt(lpdwobjecttypes : super::LPDWORD, dwobjecttypes : super::DWORD, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe { ADsBuildVarArrayInt(lpdwobjecttypes as _, dwobjecttypes, pvar as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn ADsBuildVarArrayStr(lpppathnames: &[windows_core::PCWSTR], pvar: *mut super::VARIANT) -> windows_core::HRESULT {
    windows_core::link!("activeds.dll" "system" fn ADsBuildVarArrayStr(lpppathnames : *const windows_core::PCWSTR, dwpathnames : super::DWORD, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe { ADsBuildVarArrayStr(lpppathnames.as_ptr(), lpppathnames.len().try_into().unwrap(), pvar as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn ADsDecodeBinaryData<P0>(szsrcdata: P0, ppbdestdata: *mut super::PBYTE, pdwdestlen: *mut super::ULONG) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("activeds.dll" "system" fn ADsDecodeBinaryData(szsrcdata : windows_core::PCWSTR, ppbdestdata : *mut super::PBYTE, pdwdestlen : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { ADsDecodeBinaryData(szsrcdata.param().abi(), ppbdestdata as _, pdwdestlen as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn ADsEncodeBinaryData(pbsrcdata: super::PBYTE, dwsrclen: super::DWORD) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("activeds.dll" "system" fn ADsEncodeBinaryData(pbsrcdata : super::PBYTE, dwsrclen : super::DWORD, ppszdestdata : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        ADsEncodeBinaryData(pbsrcdata as _, dwsrclen, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn ADsEnumerateNext<P0>(penumvariant: P0, celements: super::ULONG, pvar: *mut super::VARIANT, pcelementsfetched: *mut super::ULONG) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IEnumVARIANT>,
{
    windows_core::link!("activeds.dll" "system" fn ADsEnumerateNext(penumvariant : *mut core::ffi::c_void, celements : super::ULONG, pvar : *mut super::VARIANT, pcelementsfetched : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { ADsEnumerateNext(penumvariant.param().abi(), celements, pvar as _, pcelementsfetched as _) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn ADsFreeEnumerator<P0>(penumvariant: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IEnumVARIANT>,
{
    windows_core::link!("activeds.dll" "system" fn ADsFreeEnumerator(penumvariant : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ADsFreeEnumerator(penumvariant.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn ADsGetLastError(lperror: super::LPDWORD, lperrorbuf: windows_core::PWSTR, dwerrorbuflen: super::DWORD, lpnamebuf: windows_core::PWSTR, dwnamebuflen: super::DWORD) -> windows_core::HRESULT {
    windows_core::link!("activeds.dll" "system" fn ADsGetLastError(lperror : super::LPDWORD, lperrorbuf : windows_core::PWSTR, dwerrorbuflen : super::DWORD, lpnamebuf : windows_core::PWSTR, dwnamebuflen : super::DWORD) -> windows_core::HRESULT);
    unsafe { ADsGetLastError(lperror as _, lperrorbuf, dwerrorbuflen, lpnamebuf, dwnamebuflen) }
}
#[cfg(feature = "guiddef")]
#[inline]
pub unsafe fn ADsGetObject<P0>(lpszpathname: P0, riid: *const super::IID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("activeds.dll" "system" fn ADsGetObject(lpszpathname : windows_core::PCWSTR, riid : *const super::IID, ppobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ADsGetObject(lpszpathname.param().abi(), riid, ppobject as _) }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[inline]
pub unsafe fn ADsOpenObject<P0, P1, P2>(lpszpathname: P0, lpszusername: P1, lpszpassword: P2, dwreserved: super::DWORD, riid: *const super::IID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("activeds.dll" "system" fn ADsOpenObject(lpszpathname : windows_core::PCWSTR, lpszusername : windows_core::PCWSTR, lpszpassword : windows_core::PCWSTR, dwreserved : super::DWORD, riid : *const super::IID, ppobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ADsOpenObject(lpszpathname.param().abi(), lpszusername.param().abi(), lpszpassword.param().abi(), dwreserved, riid, ppobject as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn ADsSetLastError<P1, P2>(dwerr: super::DWORD, pszerror: P1, pszprovider: P2)
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("activeds.dll" "system" fn ADsSetLastError(dwerr : super::DWORD, pszerror : windows_core::PCWSTR, pszprovider : windows_core::PCWSTR));
    unsafe { ADsSetLastError(dwerr, pszerror.param().abi(), pszprovider.param().abi()) }
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
pub unsafe fn AllocADsStr<P0>(pstr: P0) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("activeds.dll" "system" fn AllocADsStr(pstr : windows_core::PCWSTR) -> windows_core::PWSTR);
    unsafe { AllocADsStr(pstr.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn BinarySDToSecurityDescriptor<P2, P3, P4>(psecuritydescriptor: super::PSECURITY_DESCRIPTOR, pvarsec: *mut super::VARIANT, pszservername: P2, username: P3, password: P4, dwflags: super::DWORD) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("activeds.dll" "system" fn BinarySDToSecurityDescriptor(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, pvarsec : *mut super::VARIANT, pszservername : windows_core::PCWSTR, username : windows_core::PCWSTR, password : windows_core::PCWSTR, dwflags : super::DWORD) -> windows_core::HRESULT);
    unsafe { BinarySDToSecurityDescriptor(psecuritydescriptor as _, pvarsec as _, pszservername.param().abi(), username.param().abi(), password.param().abi(), dwflags) }
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
pub unsafe fn SecurityDescriptorToBinarySD<P3, P4, P5>(vvarsecdes: &super::VARIANT, ppsecuritydescriptor: *mut super::PSECURITY_DESCRIPTOR, pdwsdlength: super::PDWORD, pszservername: P3, username: P4, password: P5, dwflags: super::DWORD) -> windows_core::HRESULT
where
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("activeds.dll" "system" fn SecurityDescriptorToBinarySD(vvarsecdes : super::VARIANT, ppsecuritydescriptor : *mut super::PSECURITY_DESCRIPTOR, pdwsdlength : super::PDWORD, pszservername : windows_core::PCWSTR, username : windows_core::PCWSTR, password : windows_core::PCWSTR, dwflags : super::DWORD) -> windows_core::HRESULT);
    unsafe { SecurityDescriptorToBinarySD(core::mem::transmute_copy(vvarsecdes), ppsecuritydescriptor as _, pdwsdlength as _, pszservername.param().abi(), username.param().abi(), password.param().abi(), dwflags) }
}
