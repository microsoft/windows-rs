#[cfg(all(feature = "iads", feature = "oaidl"))]
windows_link::link!("activeds.dll" "system" fn ADsBuildEnumerator(padscontainer : *mut core::ffi::c_void, ppenumvariant : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("activeds.dll" "system" fn ADsBuildVarArrayInt(lpdwobjecttypes : super::LPDWORD, dwobjecttypes : super::DWORD, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("activeds.dll" "system" fn ADsBuildVarArrayStr(lpppathnames : *const windows_sys::core::PCWSTR, dwpathnames : super::DWORD, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("activeds.dll" "system" fn ADsDecodeBinaryData(szsrcdata : windows_sys::core::PCWSTR, ppbdestdata : *mut super::PBYTE, pdwdestlen : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("activeds.dll" "system" fn ADsEncodeBinaryData(pbsrcdata : super::PBYTE, dwsrclen : super::DWORD, ppszdestdata : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("activeds.dll" "system" fn ADsEnumerateNext(penumvariant : *mut core::ffi::c_void, celements : super::ULONG, pvar : *mut super::VARIANT, pcelementsfetched : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "oaidl")]
windows_link::link!("activeds.dll" "system" fn ADsFreeEnumerator(penumvariant : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("activeds.dll" "system" fn ADsGetLastError(lperror : super::LPDWORD, lperrorbuf : windows_sys::core::PWSTR, dwerrorbuflen : super::DWORD, lpnamebuf : windows_sys::core::PWSTR, dwnamebuflen : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "guiddef")]
windows_link::link!("activeds.dll" "system" fn ADsGetObject(lpszpathname : windows_sys::core::PCWSTR, riid : *const super::IID, ppobject : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "shared"))]
windows_link::link!("activeds.dll" "system" fn ADsOpenObject(lpszpathname : windows_sys::core::PCWSTR, lpszusername : windows_sys::core::PCWSTR, lpszpassword : windows_sys::core::PCWSTR, dwreserved : super::DWORD, riid : *const super::IID, ppobject : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("activeds.dll" "system" fn ADsSetLastError(dwerr : super::DWORD, pszerror : windows_sys::core::PCWSTR, pszprovider : windows_sys::core::PCWSTR));
#[cfg(all(feature = "iads", feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("activeds.dll" "system" fn AdsFreeAdsValues(padsvalues : super::PADSVALUE, dwnumvalues : super::DWORD));
#[cfg(all(feature = "iads", feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("activeds.dll" "system" fn AdsTypeToPropVariant(padsvalues : super::PADSVALUE, dwnumvalues : super::DWORD, pvariant : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("activeds.dll" "system" fn AllocADsMem(cb : super::DWORD) -> super::LPVOID);
windows_link::link!("activeds.dll" "system" fn AllocADsStr(pstr : windows_sys::core::PCWSTR) -> windows_sys::core::PWSTR);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("activeds.dll" "system" fn BinarySDToSecurityDescriptor(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, pvarsec : *mut super::VARIANT, pszservername : windows_sys::core::PCWSTR, username : windows_sys::core::PCWSTR, password : windows_sys::core::PCWSTR, dwflags : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "minwindef")]
windows_link::link!("activeds.dll" "system" fn FreeADsMem(pmem : super::LPVOID) -> windows_sys::core::BOOL);
windows_link::link!("activeds.dll" "system" fn FreeADsStr(pstr : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "iads", feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("activeds.dll" "system" fn PropVariantToAdsType(pvariant : *mut super::VARIANT, dwnumvariant : super::DWORD, ppadsvalues : *mut super::PADSVALUE, pdwnumvalues : super::PDWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("activeds.dll" "system" fn ReallocADsMem(poldmem : super::LPVOID, cbold : super::DWORD, cbnew : super::DWORD) -> super::LPVOID);
windows_link::link!("activeds.dll" "system" fn ReallocADsStr(ppstr : *mut windows_sys::core::PWSTR, pstr : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("activeds.dll" "system" fn SecurityDescriptorToBinarySD(vvarsecdes : super::VARIANT, ppsecuritydescriptor : *mut super::PSECURITY_DESCRIPTOR, pdwsdlength : super::PDWORD, pszservername : windows_sys::core::PCWSTR, username : windows_sys::core::PCWSTR, password : windows_sys::core::PCWSTR, dwflags : super::DWORD) -> windows_sys::core::HRESULT);
