#[cfg(all(feature = "iads", feature = "oaidl"))]
#[inline]
pub unsafe fn ADsBuildEnumerator<P0>(padscontainer: P0) -> windows_core::Result<super::IEnumVARIANT>
where
    P0: windows_core::Param<super::IADsContainer>,
{
    windows_core::link!("activeds.dll" "system" fn ADsBuildEnumerator(padscontainer : *mut core::ffi::c_void, ppenumvariant : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        ADsBuildEnumerator(padscontainer.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn ADsBuildVarArrayInt(lpdwobjecttypes: super::LPDWORD, dwobjecttypes: u32) -> windows_core::Result<super::VARIANT> {
    windows_core::link!("activeds.dll" "system" fn ADsBuildVarArrayInt(lpdwobjecttypes : super::LPDWORD, dwobjecttypes : u32, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        ADsBuildVarArrayInt(lpdwobjecttypes, dwobjecttypes, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn ADsBuildVarArrayStr(lpppathnames: &[windows_core::PCWSTR]) -> windows_core::Result<super::VARIANT> {
    windows_core::link!("activeds.dll" "system" fn ADsBuildVarArrayStr(lpppathnames : *const windows_core::PCWSTR, dwpathnames : u32, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        ADsBuildVarArrayStr(lpppathnames.as_ptr(), lpppathnames.len().try_into().unwrap(), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ADsDecodeBinaryData<P0>(szsrcdata: P0, ppbdestdata: *mut super::PBYTE, pdwdestlen: *mut u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("activeds.dll" "system" fn ADsDecodeBinaryData(szsrcdata : windows_core::PCWSTR, ppbdestdata : *mut super::PBYTE, pdwdestlen : *mut u32) -> windows_core::HRESULT);
    unsafe { ADsDecodeBinaryData(szsrcdata.param().abi(), ppbdestdata as _, pdwdestlen as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ADsEncodeBinaryData(pbsrcdata: super::PBYTE, dwsrclen: u32) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("activeds.dll" "system" fn ADsEncodeBinaryData(pbsrcdata : super::PBYTE, dwsrclen : u32, ppszdestdata : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        ADsEncodeBinaryData(pbsrcdata, dwsrclen, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn ADsEnumerateNext<P0>(penumvariant: P0, celements: u32, pvar: *mut super::VARIANT, pcelementsfetched: *mut u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IEnumVARIANT>,
{
    windows_core::link!("activeds.dll" "system" fn ADsEnumerateNext(penumvariant : *mut core::ffi::c_void, celements : u32, pvar : *mut super::VARIANT, pcelementsfetched : *mut u32) -> windows_core::HRESULT);
    unsafe { ADsEnumerateNext(penumvariant.param().abi(), celements, pvar, pcelementsfetched as _) }
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
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ADsGetLastError(lperror: super::LPDWORD, lperrorbuf: windows_core::PWSTR, dwerrorbuflen: u32, lpnamebuf: windows_core::PWSTR, dwnamebuflen: u32) -> windows_core::HRESULT {
    windows_core::link!("activeds.dll" "system" fn ADsGetLastError(lperror : super::LPDWORD, lperrorbuf : windows_core::PWSTR, dwerrorbuflen : u32, lpnamebuf : windows_core::PWSTR, dwnamebuflen : u32) -> windows_core::HRESULT);
    unsafe { ADsGetLastError(lperror as _, lperrorbuf, dwerrorbuflen, lpnamebuf, dwnamebuflen) }
}
#[inline]
pub unsafe fn ADsGetObject<P0>(lpszpathname: P0, riid: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("activeds.dll" "system" fn ADsGetObject(lpszpathname : windows_core::PCWSTR, riid : *const windows_core::GUID, ppobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ADsGetObject(lpszpathname.param().abi(), riid, ppobject as _) }
}
#[inline]
pub unsafe fn ADsOpenObject<P0, P1, P2>(lpszpathname: P0, lpszusername: P1, lpszpassword: P2, dwreserved: u32, riid: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("activeds.dll" "system" fn ADsOpenObject(lpszpathname : windows_core::PCWSTR, lpszusername : windows_core::PCWSTR, lpszpassword : windows_core::PCWSTR, dwreserved : u32, riid : *const windows_core::GUID, ppobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ADsOpenObject(lpszpathname.param().abi(), lpszusername.param().abi(), lpszpassword.param().abi(), dwreserved, riid, ppobject as _) }
}
#[inline]
pub unsafe fn ADsSetLastError<P1, P2>(dwerr: u32, pszerror: P1, pszprovider: P2)
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("activeds.dll" "system" fn ADsSetLastError(dwerr : u32, pszerror : windows_core::PCWSTR, pszprovider : windows_core::PCWSTR));
    unsafe { ADsSetLastError(dwerr, pszerror.param().abi(), pszprovider.param().abi()) }
}
#[cfg(all(feature = "iads", feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn AdsFreeAdsValues(padsvalues: super::PADSVALUE, dwnumvalues: u32) {
    windows_core::link!("activeds.dll" "system" fn AdsFreeAdsValues(padsvalues : super::PADSVALUE, dwnumvalues : u32));
    unsafe { AdsFreeAdsValues(padsvalues, dwnumvalues) }
}
#[cfg(all(feature = "iads", feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn AdsTypeToPropVariant(padsvalues: super::PADSVALUE, dwnumvalues: u32) -> windows_core::Result<super::VARIANT> {
    windows_core::link!("activeds.dll" "system" fn AdsTypeToPropVariant(padsvalues : super::PADSVALUE, dwnumvalues : u32, pvariant : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        AdsTypeToPropVariant(padsvalues, dwnumvalues, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[inline]
pub unsafe fn AllocADsMem(cb: u32) -> *mut core::ffi::c_void {
    windows_core::link!("activeds.dll" "system" fn AllocADsMem(cb : u32) -> *mut core::ffi::c_void);
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
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn BinarySDToSecurityDescriptor<P2, P3, P4>(psecuritydescriptor: super::PSECURITY_DESCRIPTOR, pvarsec: *mut super::VARIANT, pszservername: P2, username: P3, password: P4, dwflags: u32) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("activeds.dll" "system" fn BinarySDToSecurityDescriptor(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, pvarsec : *mut super::VARIANT, pszservername : windows_core::PCWSTR, username : windows_core::PCWSTR, password : windows_core::PCWSTR, dwflags : u32) -> windows_core::HRESULT);
    unsafe { BinarySDToSecurityDescriptor(psecuritydescriptor, pvarsec, pszservername.param().abi(), username.param().abi(), password.param().abi(), dwflags) }
}
#[inline]
pub unsafe fn FreeADsMem(pmem: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("activeds.dll" "system" fn FreeADsMem(pmem : *mut core::ffi::c_void) -> windows_core::BOOL);
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
#[cfg(all(feature = "iads", feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantToAdsType(pvariant: *mut super::VARIANT, dwnumvariant: u32, ppadsvalues: *mut super::PADSVALUE, pdwnumvalues: super::PDWORD) -> windows_core::HRESULT {
    windows_core::link!("activeds.dll" "system" fn PropVariantToAdsType(pvariant : *mut super::VARIANT, dwnumvariant : u32, ppadsvalues : *mut super::PADSVALUE, pdwnumvalues : super::PDWORD) -> windows_core::HRESULT);
    unsafe { PropVariantToAdsType(pvariant, dwnumvariant, ppadsvalues as _, pdwnumvalues) }
}
#[inline]
pub unsafe fn ReallocADsMem(poldmem: *mut core::ffi::c_void, cbold: u32, cbnew: u32) -> *mut core::ffi::c_void {
    windows_core::link!("activeds.dll" "system" fn ReallocADsMem(poldmem : *mut core::ffi::c_void, cbold : u32, cbnew : u32) -> *mut core::ffi::c_void);
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
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn SecurityDescriptorToBinarySD<P3, P4, P5>(vvarsecdes: &super::VARIANT, ppsecuritydescriptor: *mut super::PSECURITY_DESCRIPTOR, pdwsdlength: super::PDWORD, pszservername: P3, username: P4, password: P5, dwflags: u32) -> windows_core::HRESULT
where
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("activeds.dll" "system" fn SecurityDescriptorToBinarySD(vvarsecdes : super::VARIANT, ppsecuritydescriptor : *mut super::PSECURITY_DESCRIPTOR, pdwsdlength : super::PDWORD, pszservername : windows_core::PCWSTR, username : windows_core::PCWSTR, password : windows_core::PCWSTR, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SecurityDescriptorToBinarySD(core::mem::transmute_copy(vvarsecdes), ppsecuritydescriptor as _, pdwsdlength, pszservername.param().abi(), username.param().abi(), password.param().abi(), dwflags) }
}
