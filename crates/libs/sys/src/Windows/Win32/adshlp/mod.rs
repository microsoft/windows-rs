#[cfg(all(feature = "iads", feature = "oaidl"))]
windows_link::link!("activeds.dll" "system" fn ADsBuildEnumerator(padscontainer : *mut core::ffi::c_void, ppenumvariant : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("activeds.dll" "system" fn ADsBuildVarArrayInt(lpdwobjecttypes : *mut u32, dwobjecttypes : u32, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("activeds.dll" "system" fn ADsBuildVarArrayStr(lpppathnames : *const windows_sys::core::PCWSTR, dwpathnames : u32, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(feature = "minwindef")]
windows_link::link!("activeds.dll" "system" fn ADsDecodeBinaryData(szsrcdata : windows_sys::core::PCWSTR, ppbdestdata : *mut super::PBYTE, pdwdestlen : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("activeds.dll" "system" fn ADsEncodeBinaryData(pbsrcdata : *mut u8, dwsrclen : u32, ppszdestdata : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("activeds.dll" "system" fn ADsEnumerateNext(penumvariant : *mut core::ffi::c_void, celements : u32, pvar : *mut super::VARIANT, pcelementsfetched : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "oaidl")]
windows_link::link!("activeds.dll" "system" fn ADsFreeEnumerator(penumvariant : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("activeds.dll" "system" fn ADsGetLastError(lperror : *mut u32, lperrorbuf : windows_sys::core::PWSTR, dwerrorbuflen : u32, lpnamebuf : windows_sys::core::PWSTR, dwnamebuflen : u32) -> windows_sys::core::HRESULT);
windows_link::link!("activeds.dll" "system" fn ADsGetObject(lpszpathname : windows_sys::core::PCWSTR, riid : *const windows_sys::core::GUID, ppobject : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("activeds.dll" "system" fn ADsOpenObject(lpszpathname : windows_sys::core::PCWSTR, lpszusername : windows_sys::core::PCWSTR, lpszpassword : windows_sys::core::PCWSTR, dwreserved : u32, riid : *const windows_sys::core::GUID, ppobject : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("activeds.dll" "system" fn ADsSetLastError(dwerr : u32, pszerror : windows_sys::core::PCWSTR, pszprovider : windows_sys::core::PCWSTR));
#[cfg(all(feature = "iads", feature = "minwinbase", feature = "minwindef"))]
windows_link::link!("activeds.dll" "system" fn AdsFreeAdsValues(padsvalues : *mut super::ADSVALUE, dwnumvalues : u32));
#[cfg(all(feature = "iads", feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("activeds.dll" "system" fn AdsTypeToPropVariant(padsvalues : *mut super::ADSVALUE, dwnumvalues : u32, pvariant : *mut super::VARIANT) -> windows_sys::core::HRESULT);
windows_link::link!("activeds.dll" "system" fn AllocADsMem(cb : u32) -> *mut core::ffi::c_void);
windows_link::link!("activeds.dll" "system" fn AllocADsStr(pstr : windows_sys::core::PCWSTR) -> windows_sys::core::PWSTR);
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("activeds.dll" "system" fn BinarySDToSecurityDescriptor(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, pvarsec : *mut super::VARIANT, pszservername : windows_sys::core::PCWSTR, username : windows_sys::core::PCWSTR, password : windows_sys::core::PCWSTR, dwflags : u32) -> windows_sys::core::HRESULT);
windows_link::link!("activeds.dll" "system" fn FreeADsMem(pmem : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("activeds.dll" "system" fn FreeADsStr(pstr : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "iads", feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("activeds.dll" "system" fn PropVariantToAdsType(pvariant : *mut super::VARIANT, dwnumvariant : u32, ppadsvalues : *mut super::PADSVALUE, pdwnumvalues : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("activeds.dll" "system" fn ReallocADsMem(poldmem : *mut core::ffi::c_void, cbold : u32, cbnew : u32) -> *mut core::ffi::c_void);
windows_link::link!("activeds.dll" "system" fn ReallocADsStr(ppstr : *mut windows_sys::core::PWSTR, pstr : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("activeds.dll" "system" fn SecurityDescriptorToBinarySD(vvarsecdes : super::VARIANT, ppsecuritydescriptor : *mut super::PSECURITY_DESCRIPTOR, pdwsdlength : *mut u32, pszservername : windows_sys::core::PCWSTR, username : windows_sys::core::PCWSTR, password : windows_sys::core::PCWSTR, dwflags : u32) -> windows_sys::core::HRESULT);
