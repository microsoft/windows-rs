#[cfg(feature = "shared")]
#[inline]
pub unsafe fn BCryptAddContextFunction<P1, P3>(dwtable: super::ULONG, pszcontext: P1, dwinterface: super::ULONG, pszfunction: P3, dwposition: super::ULONG) -> windows_core::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptAddContextFunction(dwtable : super::ULONG, pszcontext : windows_core::PCWSTR, dwinterface : super::ULONG, pszfunction : windows_core::PCWSTR, dwposition : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptAddContextFunction(dwtable, pszcontext.param().abi(), dwinterface, pszfunction.param().abi(), dwposition) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn BCryptCloseAlgorithmProvider(halgorithm: BCRYPT_ALG_HANDLE, dwflags: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptCloseAlgorithmProvider(halgorithm : BCRYPT_ALG_HANDLE, dwflags : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptCloseAlgorithmProvider(halgorithm as _, dwflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn BCryptConfigureContext<P1>(dwtable: super::ULONG, pszcontext: P1, pconfig: PCRYPT_CONTEXT_CONFIG) -> windows_core::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptConfigureContext(dwtable : super::ULONG, pszcontext : windows_core::PCWSTR, pconfig : PCRYPT_CONTEXT_CONFIG) -> windows_core::NTSTATUS);
    unsafe { BCryptConfigureContext(dwtable, pszcontext.param().abi(), pconfig) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn BCryptConfigureContextFunction<P1, P3>(dwtable: super::ULONG, pszcontext: P1, dwinterface: super::ULONG, pszfunction: P3, pconfig: PCRYPT_CONTEXT_FUNCTION_CONFIG) -> windows_core::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptConfigureContextFunction(dwtable : super::ULONG, pszcontext : windows_core::PCWSTR, dwinterface : super::ULONG, pszfunction : windows_core::PCWSTR, pconfig : PCRYPT_CONTEXT_FUNCTION_CONFIG) -> windows_core::NTSTATUS);
    unsafe { BCryptConfigureContextFunction(dwtable, pszcontext.param().abi(), dwinterface, pszfunction.param().abi(), pconfig) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn BCryptCreateContext<P1>(dwtable: super::ULONG, pszcontext: P1, pconfig: Option<PCRYPT_CONTEXT_CONFIG>) -> windows_core::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptCreateContext(dwtable : super::ULONG, pszcontext : windows_core::PCWSTR, pconfig : PCRYPT_CONTEXT_CONFIG) -> windows_core::NTSTATUS);
    unsafe { BCryptCreateContext(dwtable, pszcontext.param().abi(), pconfig.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn BCryptCreateHash(halgorithm: BCRYPT_ALG_HANDLE, phhash: *mut BCRYPT_HASH_HANDLE, pbhashobject: Option<super::PUCHAR>, cbhashobject: super::ULONG, pbsecret: Option<&[u8]>, dwflags: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptCreateHash(halgorithm : BCRYPT_ALG_HANDLE, phhash : *mut BCRYPT_HASH_HANDLE, pbhashobject : super::PUCHAR, cbhashobject : super::ULONG, pbsecret : super::PUCHAR, cbsecret : super::ULONG, dwflags : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptCreateHash(halgorithm as _, phhash as _, pbhashobject.unwrap_or(core::mem::zeroed()) as _, cbhashobject, core::mem::transmute(pbsecret.map_or(core::ptr::null(), |slice| slice.as_ptr())), pbsecret.map_or(0, |slice| slice.len().try_into().unwrap()), dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn BCryptCreateMultiHash(halgorithm: BCRYPT_ALG_HANDLE, phhash: *mut BCRYPT_HASH_HANDLE, nhashes: super::ULONG, pbhashobject: Option<super::PUCHAR>, cbhashobject: super::ULONG, pbsecret: Option<&[u8]>, dwflags: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptCreateMultiHash(halgorithm : BCRYPT_ALG_HANDLE, phhash : *mut BCRYPT_HASH_HANDLE, nhashes : super::ULONG, pbhashobject : super::PUCHAR, cbhashobject : super::ULONG, pbsecret : super::PUCHAR, cbsecret : super::ULONG, dwflags : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptCreateMultiHash(halgorithm as _, phhash as _, nhashes, pbhashobject.unwrap_or(core::mem::zeroed()) as _, cbhashobject, core::mem::transmute(pbsecret.map_or(core::ptr::null(), |slice| slice.as_ptr())), pbsecret.map_or(0, |slice| slice.len().try_into().unwrap()), dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn BCryptDecapsulate(hkey: BCRYPT_KEY_HANDLE, pbciphertext: &[u8], pbsecretkey: Option<super::PUCHAR>, cbsecretkey: super::ULONG, pcbsecretkey: *mut super::ULONG, dwflags: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptDecapsulate(hkey : BCRYPT_KEY_HANDLE, pbciphertext : super::PUCHAR, cbciphertext : super::ULONG, pbsecretkey : super::PUCHAR, cbsecretkey : super::ULONG, pcbsecretkey : *mut super::ULONG, dwflags : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptDecapsulate(hkey, core::mem::transmute(pbciphertext.as_ptr()), pbciphertext.len().try_into().unwrap(), pbsecretkey.unwrap_or(core::mem::zeroed()) as _, cbsecretkey, pcbsecretkey as _, dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn BCryptDecrypt(hkey: BCRYPT_KEY_HANDLE, pbinput: Option<&[u8]>, ppaddinginfo: Option<*const core::ffi::c_void>, pbiv: Option<&mut [u8]>, pboutput: Option<super::PUCHAR>, cboutput: super::ULONG, pcbresult: *mut super::ULONG, dwflags: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptDecrypt(hkey : BCRYPT_KEY_HANDLE, pbinput : super::PUCHAR, cbinput : super::ULONG, ppaddinginfo : *const core::ffi::c_void, pbiv : super::PUCHAR, cbiv : super::ULONG, pboutput : super::PUCHAR, cboutput : super::ULONG, pcbresult : *mut super::ULONG, dwflags : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptDecrypt(hkey as _, core::mem::transmute(pbinput.map_or(core::ptr::null(), |slice| slice.as_ptr())), pbinput.map_or(0, |slice| slice.len().try_into().unwrap()), ppaddinginfo.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pbiv.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), pbiv.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pboutput.unwrap_or(core::mem::zeroed()) as _, cboutput, pcbresult as _, dwflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn BCryptDeleteContext<P1>(dwtable: super::ULONG, pszcontext: P1) -> windows_core::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptDeleteContext(dwtable : super::ULONG, pszcontext : windows_core::PCWSTR) -> windows_core::NTSTATUS);
    unsafe { BCryptDeleteContext(dwtable, pszcontext.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn BCryptDeriveKey<P1>(hsharedsecret: BCRYPT_SECRET_HANDLE, pwszkdf: P1, pparameterlist: Option<*const BCryptBufferDesc>, pbderivedkey: Option<super::PUCHAR>, cbderivedkey: super::ULONG, pcbresult: *mut super::ULONG, dwflags: super::ULONG) -> windows_core::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptDeriveKey(hsharedsecret : BCRYPT_SECRET_HANDLE, pwszkdf : windows_core::PCWSTR, pparameterlist : *const BCryptBufferDesc, pbderivedkey : super::PUCHAR, cbderivedkey : super::ULONG, pcbresult : *mut super::ULONG, dwflags : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptDeriveKey(hsharedsecret, pwszkdf.param().abi(), pparameterlist.unwrap_or(core::mem::zeroed()) as _, pbderivedkey.unwrap_or(core::mem::zeroed()) as _, cbderivedkey, pcbresult as _, dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn BCryptDeriveKeyCapi(hhash: BCRYPT_HASH_HANDLE, htargetalg: Option<BCRYPT_ALG_HANDLE>, pbderivedkey: super::PUCHAR, cbderivedkey: super::ULONG, dwflags: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptDeriveKeyCapi(hhash : BCRYPT_HASH_HANDLE, htargetalg : BCRYPT_ALG_HANDLE, pbderivedkey : super::PUCHAR, cbderivedkey : super::ULONG, dwflags : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptDeriveKeyCapi(hhash, htargetalg.unwrap_or(core::mem::zeroed()) as _, pbderivedkey as _, cbderivedkey, dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn BCryptDeriveKeyPBKDF2(hprf: BCRYPT_ALG_HANDLE, pbpassword: Option<&[u8]>, pbsalt: Option<&[u8]>, citerations: super::ULONGLONG, pbderivedkey: super::PUCHAR, cbderivedkey: super::ULONG, dwflags: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptDeriveKeyPBKDF2(hprf : BCRYPT_ALG_HANDLE, pbpassword : super::PUCHAR, cbpassword : super::ULONG, pbsalt : super::PUCHAR, cbsalt : super::ULONG, citerations : super::ULONGLONG, pbderivedkey : super::PUCHAR, cbderivedkey : super::ULONG, dwflags : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptDeriveKeyPBKDF2(hprf, core::mem::transmute(pbpassword.map_or(core::ptr::null(), |slice| slice.as_ptr())), pbpassword.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pbsalt.map_or(core::ptr::null(), |slice| slice.as_ptr())), pbsalt.map_or(0, |slice| slice.len().try_into().unwrap()), citerations, pbderivedkey as _, cbderivedkey, dwflags) }
}
#[inline]
pub unsafe fn BCryptDestroyHash(hhash: BCRYPT_HASH_HANDLE) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptDestroyHash(hhash : BCRYPT_HASH_HANDLE) -> windows_core::NTSTATUS);
    unsafe { BCryptDestroyHash(hhash as _) }
}
#[inline]
pub unsafe fn BCryptDestroyKey(hkey: BCRYPT_KEY_HANDLE) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptDestroyKey(hkey : BCRYPT_KEY_HANDLE) -> windows_core::NTSTATUS);
    unsafe { BCryptDestroyKey(hkey as _) }
}
#[inline]
pub unsafe fn BCryptDestroySecret(hsecret: BCRYPT_SECRET_HANDLE) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptDestroySecret(hsecret : BCRYPT_SECRET_HANDLE) -> windows_core::NTSTATUS);
    unsafe { BCryptDestroySecret(hsecret as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn BCryptDuplicateHash(hhash: BCRYPT_HASH_HANDLE, phnewhash: *mut BCRYPT_HASH_HANDLE, pbhashobject: Option<super::PUCHAR>, cbhashobject: super::ULONG, dwflags: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptDuplicateHash(hhash : BCRYPT_HASH_HANDLE, phnewhash : *mut BCRYPT_HASH_HANDLE, pbhashobject : super::PUCHAR, cbhashobject : super::ULONG, dwflags : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptDuplicateHash(hhash, phnewhash as _, pbhashobject.unwrap_or(core::mem::zeroed()) as _, cbhashobject, dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn BCryptDuplicateKey(hkey: BCRYPT_KEY_HANDLE, phnewkey: *mut BCRYPT_KEY_HANDLE, pbkeyobject: Option<super::PUCHAR>, cbkeyobject: super::ULONG, dwflags: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptDuplicateKey(hkey : BCRYPT_KEY_HANDLE, phnewkey : *mut BCRYPT_KEY_HANDLE, pbkeyobject : super::PUCHAR, cbkeyobject : super::ULONG, dwflags : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptDuplicateKey(hkey, phnewkey as _, pbkeyobject.unwrap_or(core::mem::zeroed()) as _, cbkeyobject, dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn BCryptEncapsulate(hkey: BCRYPT_KEY_HANDLE, pbsecretkey: Option<super::PUCHAR>, cbsecretkey: super::ULONG, pcbsecretkey: *mut super::ULONG, pbciphertext: Option<super::PUCHAR>, cbciphertext: super::ULONG, pcbciphertext: *mut super::ULONG, dwflags: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptEncapsulate(hkey : BCRYPT_KEY_HANDLE, pbsecretkey : super::PUCHAR, cbsecretkey : super::ULONG, pcbsecretkey : *mut super::ULONG, pbciphertext : super::PUCHAR, cbciphertext : super::ULONG, pcbciphertext : *mut super::ULONG, dwflags : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptEncapsulate(hkey, pbsecretkey.unwrap_or(core::mem::zeroed()) as _, cbsecretkey, pcbsecretkey as _, pbciphertext.unwrap_or(core::mem::zeroed()) as _, cbciphertext, pcbciphertext as _, dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn BCryptEncrypt(hkey: BCRYPT_KEY_HANDLE, pbinput: Option<&[u8]>, ppaddinginfo: Option<*const core::ffi::c_void>, pbiv: Option<&mut [u8]>, pboutput: Option<super::PUCHAR>, cboutput: super::ULONG, pcbresult: *mut super::ULONG, dwflags: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptEncrypt(hkey : BCRYPT_KEY_HANDLE, pbinput : super::PUCHAR, cbinput : super::ULONG, ppaddinginfo : *const core::ffi::c_void, pbiv : super::PUCHAR, cbiv : super::ULONG, pboutput : super::PUCHAR, cboutput : super::ULONG, pcbresult : *mut super::ULONG, dwflags : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptEncrypt(hkey as _, core::mem::transmute(pbinput.map_or(core::ptr::null(), |slice| slice.as_ptr())), pbinput.map_or(0, |slice| slice.len().try_into().unwrap()), ppaddinginfo.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pbiv.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), pbiv.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pboutput.unwrap_or(core::mem::zeroed()) as _, cboutput, pcbresult as _, dwflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn BCryptEnumAlgorithms(dwalgoperations: super::ULONG, palgcount: *mut super::ULONG, ppalglist: *mut *mut BCRYPT_ALGORITHM_IDENTIFIER, dwflags: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptEnumAlgorithms(dwalgoperations : super::ULONG, palgcount : *mut super::ULONG, ppalglist : *mut *mut BCRYPT_ALGORITHM_IDENTIFIER, dwflags : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptEnumAlgorithms(dwalgoperations, palgcount as _, ppalglist as _, dwflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn BCryptEnumContextFunctionProviders<P1, P3>(dwtable: super::ULONG, pszcontext: P1, dwinterface: super::ULONG, pszfunction: P3, pcbbuffer: *mut super::ULONG, ppbuffer: Option<*mut PCRYPT_CONTEXT_FUNCTION_PROVIDERS>) -> windows_core::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptEnumContextFunctionProviders(dwtable : super::ULONG, pszcontext : windows_core::PCWSTR, dwinterface : super::ULONG, pszfunction : windows_core::PCWSTR, pcbbuffer : *mut super::ULONG, ppbuffer : *mut PCRYPT_CONTEXT_FUNCTION_PROVIDERS) -> windows_core::NTSTATUS);
    unsafe { BCryptEnumContextFunctionProviders(dwtable, pszcontext.param().abi(), dwinterface, pszfunction.param().abi(), pcbbuffer as _, ppbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn BCryptEnumContextFunctions<P1>(dwtable: super::ULONG, pszcontext: P1, dwinterface: super::ULONG, pcbbuffer: *mut super::ULONG, ppbuffer: Option<*mut PCRYPT_CONTEXT_FUNCTIONS>) -> windows_core::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptEnumContextFunctions(dwtable : super::ULONG, pszcontext : windows_core::PCWSTR, dwinterface : super::ULONG, pcbbuffer : *mut super::ULONG, ppbuffer : *mut PCRYPT_CONTEXT_FUNCTIONS) -> windows_core::NTSTATUS);
    unsafe { BCryptEnumContextFunctions(dwtable, pszcontext.param().abi(), dwinterface, pcbbuffer as _, ppbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn BCryptEnumContexts(dwtable: super::ULONG, pcbbuffer: *mut super::ULONG, ppbuffer: Option<*mut PCRYPT_CONTEXTS>) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptEnumContexts(dwtable : super::ULONG, pcbbuffer : *mut super::ULONG, ppbuffer : *mut PCRYPT_CONTEXTS) -> windows_core::NTSTATUS);
    unsafe { BCryptEnumContexts(dwtable, pcbbuffer as _, ppbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn BCryptEnumProviders<P0>(pszalgid: P0, pimplcount: *mut super::ULONG, ppimpllist: *mut *mut BCRYPT_PROVIDER_NAME, dwflags: super::ULONG) -> windows_core::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptEnumProviders(pszalgid : windows_core::PCWSTR, pimplcount : *mut super::ULONG, ppimpllist : *mut *mut BCRYPT_PROVIDER_NAME, dwflags : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptEnumProviders(pszalgid.param().abi(), pimplcount as _, ppimpllist as _, dwflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn BCryptEnumRegisteredProviders(pcbbuffer: *mut super::ULONG, ppbuffer: Option<*mut PCRYPT_PROVIDERS>) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptEnumRegisteredProviders(pcbbuffer : *mut super::ULONG, ppbuffer : *mut PCRYPT_PROVIDERS) -> windows_core::NTSTATUS);
    unsafe { BCryptEnumRegisteredProviders(pcbbuffer as _, ppbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn BCryptExportKey<P2>(hkey: BCRYPT_KEY_HANDLE, hexportkey: Option<BCRYPT_KEY_HANDLE>, pszblobtype: P2, pboutput: Option<super::PUCHAR>, cboutput: super::ULONG, pcbresult: *mut super::ULONG, dwflags: super::ULONG) -> windows_core::NTSTATUS
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptExportKey(hkey : BCRYPT_KEY_HANDLE, hexportkey : BCRYPT_KEY_HANDLE, pszblobtype : windows_core::PCWSTR, pboutput : super::PUCHAR, cboutput : super::ULONG, pcbresult : *mut super::ULONG, dwflags : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptExportKey(hkey, hexportkey.unwrap_or(core::mem::zeroed()) as _, pszblobtype.param().abi(), pboutput.unwrap_or(core::mem::zeroed()) as _, cboutput, pcbresult as _, dwflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn BCryptFinalizeKeyPair(hkey: BCRYPT_KEY_HANDLE, dwflags: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptFinalizeKeyPair(hkey : BCRYPT_KEY_HANDLE, dwflags : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptFinalizeKeyPair(hkey as _, dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn BCryptFinishHash(hhash: BCRYPT_HASH_HANDLE, pboutput: super::PUCHAR, cboutput: super::ULONG, dwflags: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptFinishHash(hhash : BCRYPT_HASH_HANDLE, pboutput : super::PUCHAR, cboutput : super::ULONG, dwflags : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptFinishHash(hhash as _, pboutput as _, cboutput, dwflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn BCryptFreeBuffer(pvbuffer: super::PVOID) {
    windows_core::link!("bcrypt.dll" "system" fn BCryptFreeBuffer(pvbuffer : super::PVOID));
    unsafe { BCryptFreeBuffer(pvbuffer) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn BCryptGenRandom(halgorithm: Option<BCRYPT_ALG_HANDLE>, pbbuffer: super::PUCHAR, cbbuffer: super::ULONG, dwflags: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptGenRandom(halgorithm : BCRYPT_ALG_HANDLE, pbbuffer : super::PUCHAR, cbbuffer : super::ULONG, dwflags : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptGenRandom(halgorithm.unwrap_or(core::mem::zeroed()) as _, pbbuffer as _, cbbuffer, dwflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn BCryptGenerateKeyPair(halgorithm: BCRYPT_ALG_HANDLE, phkey: *mut BCRYPT_KEY_HANDLE, dwlength: super::ULONG, dwflags: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptGenerateKeyPair(halgorithm : BCRYPT_ALG_HANDLE, phkey : *mut BCRYPT_KEY_HANDLE, dwlength : super::ULONG, dwflags : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptGenerateKeyPair(halgorithm as _, phkey as _, dwlength, dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn BCryptGenerateSymmetricKey(halgorithm: BCRYPT_ALG_HANDLE, phkey: *mut BCRYPT_KEY_HANDLE, pbkeyobject: Option<super::PUCHAR>, cbkeyobject: super::ULONG, pbsecret: &[u8], dwflags: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptGenerateSymmetricKey(halgorithm : BCRYPT_ALG_HANDLE, phkey : *mut BCRYPT_KEY_HANDLE, pbkeyobject : super::PUCHAR, cbkeyobject : super::ULONG, pbsecret : super::PUCHAR, cbsecret : super::ULONG, dwflags : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptGenerateSymmetricKey(halgorithm as _, phkey as _, pbkeyobject.unwrap_or(core::mem::zeroed()) as _, cbkeyobject, core::mem::transmute(pbsecret.as_ptr()), pbsecret.len().try_into().unwrap(), dwflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn BCryptGetFipsAlgorithmMode(pfenabled: *mut super::BOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptGetFipsAlgorithmMode(pfenabled : *mut super::BOOLEAN) -> windows_core::NTSTATUS);
    unsafe { BCryptGetFipsAlgorithmMode(pfenabled as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn BCryptGetProperty<P1>(hobject: BCRYPT_HANDLE, pszproperty: P1, pboutput: Option<super::PUCHAR>, cboutput: super::ULONG, pcbresult: *mut super::ULONG, dwflags: super::ULONG) -> windows_core::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptGetProperty(hobject : BCRYPT_HANDLE, pszproperty : windows_core::PCWSTR, pboutput : super::PUCHAR, cboutput : super::ULONG, pcbresult : *mut super::ULONG, dwflags : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptGetProperty(hobject, pszproperty.param().abi(), pboutput.unwrap_or(core::mem::zeroed()) as _, cboutput, pcbresult as _, dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn BCryptHash(halgorithm: BCRYPT_ALG_HANDLE, pbsecret: Option<&[u8]>, pbinput: &[u8], pboutput: super::PUCHAR, cboutput: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptHash(halgorithm : BCRYPT_ALG_HANDLE, pbsecret : super::PUCHAR, cbsecret : super::ULONG, pbinput : super::PUCHAR, cbinput : super::ULONG, pboutput : super::PUCHAR, cboutput : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptHash(halgorithm as _, core::mem::transmute(pbsecret.map_or(core::ptr::null(), |slice| slice.as_ptr())), pbsecret.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pbinput.as_ptr()), pbinput.len().try_into().unwrap(), pboutput as _, cboutput) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn BCryptHashData(hhash: BCRYPT_HASH_HANDLE, pbinput: &[u8], dwflags: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptHashData(hhash : BCRYPT_HASH_HANDLE, pbinput : super::PUCHAR, cbinput : super::ULONG, dwflags : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptHashData(hhash as _, core::mem::transmute(pbinput.as_ptr()), pbinput.len().try_into().unwrap(), dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn BCryptImportKey<P2>(halgorithm: BCRYPT_ALG_HANDLE, himportkey: Option<BCRYPT_KEY_HANDLE>, pszblobtype: P2, phkey: *mut BCRYPT_KEY_HANDLE, pbkeyobject: Option<super::PUCHAR>, cbkeyobject: super::ULONG, pbinput: &[u8], dwflags: super::ULONG) -> windows_core::NTSTATUS
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptImportKey(halgorithm : BCRYPT_ALG_HANDLE, himportkey : BCRYPT_KEY_HANDLE, pszblobtype : windows_core::PCWSTR, phkey : *mut BCRYPT_KEY_HANDLE, pbkeyobject : super::PUCHAR, cbkeyobject : super::ULONG, pbinput : super::PUCHAR, cbinput : super::ULONG, dwflags : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptImportKey(halgorithm, himportkey.unwrap_or(core::mem::zeroed()) as _, pszblobtype.param().abi(), phkey as _, pbkeyobject.unwrap_or(core::mem::zeroed()) as _, cbkeyobject, core::mem::transmute(pbinput.as_ptr()), pbinput.len().try_into().unwrap(), dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn BCryptImportKeyPair<P2>(halgorithm: BCRYPT_ALG_HANDLE, himportkey: Option<BCRYPT_KEY_HANDLE>, pszblobtype: P2, phkey: *mut BCRYPT_KEY_HANDLE, pbinput: &[u8], dwflags: super::ULONG) -> windows_core::NTSTATUS
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptImportKeyPair(halgorithm : BCRYPT_ALG_HANDLE, himportkey : BCRYPT_KEY_HANDLE, pszblobtype : windows_core::PCWSTR, phkey : *mut BCRYPT_KEY_HANDLE, pbinput : super::PUCHAR, cbinput : super::ULONG, dwflags : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptImportKeyPair(halgorithm, himportkey.unwrap_or(core::mem::zeroed()) as _, pszblobtype.param().abi(), phkey as _, core::mem::transmute(pbinput.as_ptr()), pbinput.len().try_into().unwrap(), dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn BCryptKeyDerivation(hkey: BCRYPT_KEY_HANDLE, pparameterlist: Option<*const BCryptBufferDesc>, pbderivedkey: super::PUCHAR, cbderivedkey: super::ULONG, pcbresult: *mut super::ULONG, dwflags: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptKeyDerivation(hkey : BCRYPT_KEY_HANDLE, pparameterlist : *const BCryptBufferDesc, pbderivedkey : super::PUCHAR, cbderivedkey : super::ULONG, pcbresult : *mut super::ULONG, dwflags : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptKeyDerivation(hkey, pparameterlist.unwrap_or(core::mem::zeroed()) as _, pbderivedkey as _, cbderivedkey, pcbresult as _, dwflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn BCryptOpenAlgorithmProvider<P1, P2>(phalgorithm: *mut BCRYPT_ALG_HANDLE, pszalgid: P1, pszimplementation: P2, dwflags: super::ULONG) -> windows_core::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptOpenAlgorithmProvider(phalgorithm : *mut BCRYPT_ALG_HANDLE, pszalgid : windows_core::PCWSTR, pszimplementation : windows_core::PCWSTR, dwflags : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptOpenAlgorithmProvider(phalgorithm as _, pszalgid.param().abi(), pszimplementation.param().abi(), dwflags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn BCryptProcessMultiOperations(hobject: BCRYPT_HANDLE, operationtype: BCRYPT_MULTI_OPERATION_TYPE, poperations: super::PVOID, cboperations: super::ULONG, dwflags: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptProcessMultiOperations(hobject : BCRYPT_HANDLE, operationtype : BCRYPT_MULTI_OPERATION_TYPE, poperations : super::PVOID, cboperations : super::ULONG, dwflags : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptProcessMultiOperations(hobject as _, operationtype, poperations, cboperations, dwflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn BCryptQueryContextConfiguration<P1>(dwtable: super::ULONG, pszcontext: P1, pcbbuffer: *mut super::ULONG, ppbuffer: Option<*mut PCRYPT_CONTEXT_CONFIG>) -> windows_core::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptQueryContextConfiguration(dwtable : super::ULONG, pszcontext : windows_core::PCWSTR, pcbbuffer : *mut super::ULONG, ppbuffer : *mut PCRYPT_CONTEXT_CONFIG) -> windows_core::NTSTATUS);
    unsafe { BCryptQueryContextConfiguration(dwtable, pszcontext.param().abi(), pcbbuffer as _, ppbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn BCryptQueryContextFunctionConfiguration<P1, P3>(dwtable: super::ULONG, pszcontext: P1, dwinterface: super::ULONG, pszfunction: P3, pcbbuffer: *mut super::ULONG, ppbuffer: Option<*mut PCRYPT_CONTEXT_FUNCTION_CONFIG>) -> windows_core::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptQueryContextFunctionConfiguration(dwtable : super::ULONG, pszcontext : windows_core::PCWSTR, dwinterface : super::ULONG, pszfunction : windows_core::PCWSTR, pcbbuffer : *mut super::ULONG, ppbuffer : *mut PCRYPT_CONTEXT_FUNCTION_CONFIG) -> windows_core::NTSTATUS);
    unsafe { BCryptQueryContextFunctionConfiguration(dwtable, pszcontext.param().abi(), dwinterface, pszfunction.param().abi(), pcbbuffer as _, ppbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn BCryptQueryContextFunctionProperty<P1, P3, P4>(dwtable: super::ULONG, pszcontext: P1, dwinterface: super::ULONG, pszfunction: P3, pszproperty: P4, pcbvalue: *mut super::ULONG, ppbvalue: Option<*mut super::PUCHAR>) -> windows_core::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptQueryContextFunctionProperty(dwtable : super::ULONG, pszcontext : windows_core::PCWSTR, dwinterface : super::ULONG, pszfunction : windows_core::PCWSTR, pszproperty : windows_core::PCWSTR, pcbvalue : *mut super::ULONG, ppbvalue : *mut super::PUCHAR) -> windows_core::NTSTATUS);
    unsafe { BCryptQueryContextFunctionProperty(dwtable, pszcontext.param().abi(), dwinterface, pszfunction.param().abi(), pszproperty.param().abi(), pcbvalue as _, ppbvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn BCryptQueryProviderRegistration<P0>(pszprovider: P0, dwmode: super::ULONG, dwinterface: super::ULONG, pcbbuffer: *mut super::ULONG, ppbuffer: Option<*mut PCRYPT_PROVIDER_REG>) -> windows_core::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptQueryProviderRegistration(pszprovider : windows_core::PCWSTR, dwmode : super::ULONG, dwinterface : super::ULONG, pcbbuffer : *mut super::ULONG, ppbuffer : *mut PCRYPT_PROVIDER_REG) -> windows_core::NTSTATUS);
    unsafe { BCryptQueryProviderRegistration(pszprovider.param().abi(), dwmode, dwinterface, pcbbuffer as _, ppbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn BCryptRegisterConfigChangeNotify(phevent: *mut super::HANDLE) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptRegisterConfigChangeNotify(phevent : *mut super::HANDLE) -> windows_core::NTSTATUS);
    unsafe { BCryptRegisterConfigChangeNotify(phevent as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn BCryptRemoveContextFunction<P1, P3>(dwtable: super::ULONG, pszcontext: P1, dwinterface: super::ULONG, pszfunction: P3) -> windows_core::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptRemoveContextFunction(dwtable : super::ULONG, pszcontext : windows_core::PCWSTR, dwinterface : super::ULONG, pszfunction : windows_core::PCWSTR) -> windows_core::NTSTATUS);
    unsafe { BCryptRemoveContextFunction(dwtable, pszcontext.param().abi(), dwinterface, pszfunction.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn BCryptResolveProviders<P0, P2, P3>(pszcontext: P0, dwinterface: Option<super::ULONG>, pszfunction: P2, pszprovider: P3, dwmode: super::ULONG, dwflags: super::ULONG, pcbbuffer: *mut super::ULONG, ppbuffer: Option<*mut PCRYPT_PROVIDER_REFS>) -> windows_core::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptResolveProviders(pszcontext : windows_core::PCWSTR, dwinterface : super::ULONG, pszfunction : windows_core::PCWSTR, pszprovider : windows_core::PCWSTR, dwmode : super::ULONG, dwflags : super::ULONG, pcbbuffer : *mut super::ULONG, ppbuffer : *mut PCRYPT_PROVIDER_REFS) -> windows_core::NTSTATUS);
    unsafe { BCryptResolveProviders(pszcontext.param().abi(), dwinterface.unwrap_or(core::mem::zeroed()) as _, pszfunction.param().abi(), pszprovider.param().abi(), dwmode, dwflags, pcbbuffer as _, ppbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn BCryptSecretAgreement(hprivkey: BCRYPT_KEY_HANDLE, hpubkey: BCRYPT_KEY_HANDLE, phagreedsecret: *mut BCRYPT_SECRET_HANDLE, dwflags: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptSecretAgreement(hprivkey : BCRYPT_KEY_HANDLE, hpubkey : BCRYPT_KEY_HANDLE, phagreedsecret : *mut BCRYPT_SECRET_HANDLE, dwflags : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptSecretAgreement(hprivkey, hpubkey, phagreedsecret as _, dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn BCryptSetContextFunctionProperty<P1, P3, P4>(dwtable: super::ULONG, pszcontext: P1, dwinterface: super::ULONG, pszfunction: P3, pszproperty: P4, pbvalue: Option<&[u8]>) -> windows_core::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptSetContextFunctionProperty(dwtable : super::ULONG, pszcontext : windows_core::PCWSTR, dwinterface : super::ULONG, pszfunction : windows_core::PCWSTR, pszproperty : windows_core::PCWSTR, cbvalue : super::ULONG, pbvalue : super::PUCHAR) -> windows_core::NTSTATUS);
    unsafe { BCryptSetContextFunctionProperty(dwtable, pszcontext.param().abi(), dwinterface, pszfunction.param().abi(), pszproperty.param().abi(), pbvalue.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pbvalue.map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn BCryptSetProperty<P1>(hobject: BCRYPT_HANDLE, pszproperty: P1, pbinput: &[u8], dwflags: super::ULONG) -> windows_core::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("bcrypt.dll" "system" fn BCryptSetProperty(hobject : BCRYPT_HANDLE, pszproperty : windows_core::PCWSTR, pbinput : super::PUCHAR, cbinput : super::ULONG, dwflags : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptSetProperty(hobject as _, pszproperty.param().abi(), core::mem::transmute(pbinput.as_ptr()), pbinput.len().try_into().unwrap(), dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn BCryptSignHash(hkey: BCRYPT_KEY_HANDLE, ppaddinginfo: Option<*const core::ffi::c_void>, pbinput: &[u8], pboutput: Option<super::PUCHAR>, cboutput: super::ULONG, pcbresult: *mut super::ULONG, dwflags: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptSignHash(hkey : BCRYPT_KEY_HANDLE, ppaddinginfo : *const core::ffi::c_void, pbinput : super::PUCHAR, cbinput : super::ULONG, pboutput : super::PUCHAR, cboutput : super::ULONG, pcbresult : *mut super::ULONG, dwflags : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptSignHash(hkey, ppaddinginfo.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pbinput.as_ptr()), pbinput.len().try_into().unwrap(), pboutput.unwrap_or(core::mem::zeroed()) as _, cboutput, pcbresult as _, dwflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn BCryptUnregisterConfigChangeNotify(hevent: super::HANDLE) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptUnregisterConfigChangeNotify(hevent : super::HANDLE) -> windows_core::NTSTATUS);
    unsafe { BCryptUnregisterConfigChangeNotify(hevent) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn BCryptVerifySignature(hkey: BCRYPT_KEY_HANDLE, ppaddinginfo: Option<*const core::ffi::c_void>, pbhash: &[u8], pbsignature: &[u8], dwflags: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("bcrypt.dll" "system" fn BCryptVerifySignature(hkey : BCRYPT_KEY_HANDLE, ppaddinginfo : *const core::ffi::c_void, pbhash : super::PUCHAR, cbhash : super::ULONG, pbsignature : super::PUCHAR, cbsignature : super::ULONG, dwflags : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { BCryptVerifySignature(hkey, ppaddinginfo.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pbhash.as_ptr()), pbhash.len().try_into().unwrap(), core::mem::transmute(pbsignature.as_ptr()), pbsignature.len().try_into().unwrap(), dwflags) }
}
pub const BCRYPTBUFFER_VERSION: i32 = 0;
pub const BCRYPT_3DES_112_ALGORITHM: windows_core::PCWSTR = windows_core::w!("3DES_112");
pub const BCRYPT_3DES_112_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 369 as _;
pub const BCRYPT_3DES_112_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 401 as _;
pub const BCRYPT_3DES_112_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 385 as _;
pub const BCRYPT_3DES_ALGORITHM: windows_core::PCWSTR = windows_core::w!("3DES");
pub const BCRYPT_3DES_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 321 as _;
pub const BCRYPT_3DES_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 353 as _;
pub const BCRYPT_3DES_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 337 as _;
pub const BCRYPT_AES_ALGORITHM: windows_core::PCWSTR = windows_core::w!("AES");
pub const BCRYPT_AES_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 417 as _;
pub const BCRYPT_AES_CCM_ALG_HANDLE: BCRYPT_ALG_HANDLE = 465 as _;
pub const BCRYPT_AES_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 449 as _;
pub const BCRYPT_AES_CMAC_ALGORITHM: windows_core::PCWSTR = windows_core::w!("AES-CMAC");
pub const BCRYPT_AES_CMAC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 257 as _;
pub const BCRYPT_AES_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 433 as _;
pub const BCRYPT_AES_GCM_ALG_HANDLE: BCRYPT_ALG_HANDLE = 481 as _;
pub const BCRYPT_AES_GMAC_ALGORITHM: windows_core::PCWSTR = windows_core::w!("AES-GMAC");
pub const BCRYPT_AES_GMAC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 273 as _;
pub const BCRYPT_AES_WRAP_KEY_BLOB: windows_core::PCWSTR = windows_core::w!("Rfc3565KeyWrapBlob");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_ALGORITHM_IDENTIFIER {
    pub pszName: windows_core::PWSTR,
    pub dwClass: super::ULONG,
    pub dwFlags: super::ULONG,
}
pub const BCRYPT_ALGORITHM_NAME: windows_core::PCWSTR = windows_core::w!("AlgorithmName");
pub type BCRYPT_ALG_HANDLE = *mut core::ffi::c_void;
pub const BCRYPT_ALG_HANDLE_HMAC_FLAG: i32 = 8;
pub const BCRYPT_ASYMMETRIC_ENCRYPTION_INTERFACE: i32 = 3;
pub const BCRYPT_ASYMMETRIC_ENCRYPTION_OPERATION: i32 = 4;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO {
    pub cbSize: super::ULONG,
    pub dwInfoVersion: super::ULONG,
    pub pbNonce: super::PUCHAR,
    pub cbNonce: super::ULONG,
    pub pbAuthData: super::PUCHAR,
    pub cbAuthData: super::ULONG,
    pub pbTag: super::PUCHAR,
    pub cbTag: super::ULONG,
    pub pbMacContext: super::PUCHAR,
    pub cbMacContext: super::ULONG,
    pub cbAAD: super::ULONG,
    pub cbData: super::ULONGLONG,
    pub dwFlags: super::ULONG,
}
pub const BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO_VERSION: i32 = 1;
pub const BCRYPT_AUTH_MODE_CHAIN_CALLS_FLAG: i32 = 1;
pub const BCRYPT_AUTH_MODE_IN_PROGRESS_FLAG: i32 = 2;
pub const BCRYPT_AUTH_TAG_LENGTH: windows_core::PCWSTR = windows_core::w!("AuthTagLength");
#[cfg(feature = "shared")]
pub type BCRYPT_AUTH_TAG_LENGTHS_STRUCT = BCRYPT_KEY_LENGTHS_STRUCT;
pub const BCRYPT_BLOCK_LENGTH: windows_core::PCWSTR = windows_core::w!("BlockLength");
pub const BCRYPT_BLOCK_PADDING: i32 = 1;
pub const BCRYPT_BLOCK_SIZE_LIST: windows_core::PCWSTR = windows_core::w!("BlockSizeList");
pub const BCRYPT_BUFFERS_LOCKED_FLAG: i32 = 64;
pub const BCRYPT_CAPI_AES_FLAG: i32 = 16;
pub const BCRYPT_CAPI_KDF_ALGORITHM: windows_core::PCWSTR = windows_core::w!("CAPI_KDF");
pub const BCRYPT_CAPI_KDF_ALG_HANDLE: BCRYPT_ALG_HANDLE = 801 as _;
pub const BCRYPT_CHACHA20_POLY1305_ALGORITHM: windows_core::PCWSTR = windows_core::w!("CHACHA20_POLY1305");
pub const BCRYPT_CHACHA20_POLY1305_ALG_HANDLE: BCRYPT_ALG_HANDLE = 929 as _;
pub const BCRYPT_CHAINING_MODE: windows_core::PCWSTR = windows_core::w!("ChainingMode");
pub const BCRYPT_CHAIN_MODE_CBC: windows_core::PCWSTR = windows_core::w!("ChainingModeCBC");
pub const BCRYPT_CHAIN_MODE_CCM: windows_core::PCWSTR = windows_core::w!("ChainingModeCCM");
pub const BCRYPT_CHAIN_MODE_CFB: windows_core::PCWSTR = windows_core::w!("ChainingModeCFB");
pub const BCRYPT_CHAIN_MODE_ECB: windows_core::PCWSTR = windows_core::w!("ChainingModeECB");
pub const BCRYPT_CHAIN_MODE_GCM: windows_core::PCWSTR = windows_core::w!("ChainingModeGCM");
pub const BCRYPT_CHAIN_MODE_NA: windows_core::PCWSTR = windows_core::w!("ChainingModeN/A");
pub const BCRYPT_CIPHER_INTERFACE: i32 = 1;
pub const BCRYPT_CIPHER_OPERATION: i32 = 1;
pub const BCRYPT_CSHAKE128_ALGORITHM: windows_core::PCWSTR = windows_core::w!("CSHAKE128");
pub const BCRYPT_CSHAKE128_ALG_HANDLE: BCRYPT_ALG_HANDLE = 1041 as _;
pub const BCRYPT_CSHAKE256_ALGORITHM: windows_core::PCWSTR = windows_core::w!("CSHAKE256");
pub const BCRYPT_CSHAKE256_ALG_HANDLE: BCRYPT_ALG_HANDLE = 1057 as _;
pub const BCRYPT_CUSTOMIZATION_STRING: windows_core::PCWSTR = windows_core::w!("CustomizationString");
pub const BCRYPT_DESX_ALGORITHM: windows_core::PCWSTR = windows_core::w!("DESX");
pub const BCRYPT_DESX_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 545 as _;
pub const BCRYPT_DESX_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 577 as _;
pub const BCRYPT_DESX_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 561 as _;
pub const BCRYPT_DES_ALGORITHM: windows_core::PCWSTR = windows_core::w!("DES");
pub const BCRYPT_DES_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 497 as _;
pub const BCRYPT_DES_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 529 as _;
pub const BCRYPT_DES_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 513 as _;
pub const BCRYPT_DH_ALGORITHM: windows_core::PCWSTR = windows_core::w!("DH");
pub const BCRYPT_DH_ALG_HANDLE: BCRYPT_ALG_HANDLE = 641 as _;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_DH_KEY_BLOB {
    pub dwMagic: super::ULONG,
    pub cbKey: super::ULONG,
}
pub const BCRYPT_DH_PARAMETERS: windows_core::PCWSTR = windows_core::w!("DHParameters");
pub const BCRYPT_DH_PARAMETERS_MAGIC: i32 = 1297107012;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_DH_PARAMETER_HEADER {
    pub cbLength: super::ULONG,
    pub dwMagic: super::ULONG,
    pub cbKeyLength: super::ULONG,
}
pub const BCRYPT_DH_PRIVATE_BLOB: windows_core::PCWSTR = windows_core::w!("DHPRIVATEBLOB");
pub const BCRYPT_DH_PRIVATE_MAGIC: i32 = 1448101956;
pub const BCRYPT_DH_PUBLIC_BLOB: windows_core::PCWSTR = windows_core::w!("DHPUBLICBLOB");
pub const BCRYPT_DH_PUBLIC_MAGIC: i32 = 1112557636;
pub const BCRYPT_DSA_ALGORITHM: windows_core::PCWSTR = windows_core::w!("DSA");
pub const BCRYPT_DSA_ALG_HANDLE: BCRYPT_ALG_HANDLE = 721 as _;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BCRYPT_DSA_KEY_BLOB {
    pub dwMagic: super::ULONG,
    pub cbKey: super::ULONG,
    pub Count: [super::UCHAR; 4],
    pub Seed: [super::UCHAR; 20],
    pub q: [super::UCHAR; 20],
}
#[cfg(feature = "shared")]
impl Default for BCRYPT_DSA_KEY_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BCRYPT_DSA_KEY_BLOB_V2 {
    pub dwMagic: super::ULONG,
    pub cbKey: super::ULONG,
    pub hashAlgorithm: HASHALGORITHM_ENUM,
    pub standardVersion: DSAFIPSVERSION_ENUM,
    pub cbSeedLength: super::ULONG,
    pub cbGroupSize: super::ULONG,
    pub Count: [super::UCHAR; 4],
}
#[cfg(feature = "shared")]
impl Default for BCRYPT_DSA_KEY_BLOB_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BCRYPT_DSA_PARAMETERS: windows_core::PCWSTR = windows_core::w!("DSAParameters");
pub const BCRYPT_DSA_PARAMETERS_MAGIC: i32 = 1297109828;
pub const BCRYPT_DSA_PARAMETERS_MAGIC_V2: i32 = 843927620;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BCRYPT_DSA_PARAMETER_HEADER {
    pub cbLength: super::ULONG,
    pub dwMagic: super::ULONG,
    pub cbKeyLength: super::ULONG,
    pub Count: [super::UCHAR; 4],
    pub Seed: [super::UCHAR; 20],
    pub q: [super::UCHAR; 20],
}
#[cfg(feature = "shared")]
impl Default for BCRYPT_DSA_PARAMETER_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BCRYPT_DSA_PARAMETER_HEADER_V2 {
    pub cbLength: super::ULONG,
    pub dwMagic: super::ULONG,
    pub cbKeyLength: super::ULONG,
    pub hashAlgorithm: HASHALGORITHM_ENUM,
    pub standardVersion: DSAFIPSVERSION_ENUM,
    pub cbSeedLength: super::ULONG,
    pub cbGroupSize: super::ULONG,
    pub Count: [super::UCHAR; 4],
}
#[cfg(feature = "shared")]
impl Default for BCRYPT_DSA_PARAMETER_HEADER_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BCRYPT_DSA_PRIVATE_BLOB: windows_core::PCWSTR = windows_core::w!("DSAPRIVATEBLOB");
pub const BCRYPT_DSA_PRIVATE_MAGIC: i32 = 1448104772;
pub const BCRYPT_DSA_PRIVATE_MAGIC_V2: i32 = 844517444;
pub const BCRYPT_DSA_PUBLIC_BLOB: windows_core::PCWSTR = windows_core::w!("DSAPUBLICBLOB");
pub const BCRYPT_DSA_PUBLIC_MAGIC: i32 = 1112560452;
pub const BCRYPT_DSA_PUBLIC_MAGIC_V2: i32 = 843206724;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_ECCFULLKEY_BLOB {
    pub dwMagic: super::ULONG,
    pub dwVersion: super::ULONG,
    pub dwCurveType: ECC_CURVE_TYPE_ENUM,
    pub dwCurveGenerationAlgId: ECC_CURVE_ALG_ID_ENUM,
    pub cbFieldLength: super::ULONG,
    pub cbSubgroupOrder: super::ULONG,
    pub cbCofactor: super::ULONG,
    pub cbSeed: super::ULONG,
}
pub const BCRYPT_ECCFULLPRIVATE_BLOB: windows_core::PCWSTR = windows_core::w!("ECCFULLPRIVATEBLOB");
pub const BCRYPT_ECCFULLPUBLIC_BLOB: windows_core::PCWSTR = windows_core::w!("ECCFULLPUBLICBLOB");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_ECCKEY_BLOB {
    pub dwMagic: super::ULONG,
    pub cbKey: super::ULONG,
}
pub const BCRYPT_ECCPRIVATE_BLOB: windows_core::PCWSTR = windows_core::w!("ECCPRIVATEBLOB");
pub const BCRYPT_ECCPUBLIC_BLOB: windows_core::PCWSTR = windows_core::w!("ECCPUBLICBLOB");
pub const BCRYPT_ECC_CURVE_25519: windows_core::PCWSTR = windows_core::w!("curve25519");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP160R1: windows_core::PCWSTR = windows_core::w!("brainpoolP160r1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP160T1: windows_core::PCWSTR = windows_core::w!("brainpoolP160t1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP192R1: windows_core::PCWSTR = windows_core::w!("brainpoolP192r1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP192T1: windows_core::PCWSTR = windows_core::w!("brainpoolP192t1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP224R1: windows_core::PCWSTR = windows_core::w!("brainpoolP224r1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP224T1: windows_core::PCWSTR = windows_core::w!("brainpoolP224t1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP256R1: windows_core::PCWSTR = windows_core::w!("brainpoolP256r1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP256T1: windows_core::PCWSTR = windows_core::w!("brainpoolP256t1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP320R1: windows_core::PCWSTR = windows_core::w!("brainpoolP320r1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP320T1: windows_core::PCWSTR = windows_core::w!("brainpoolP320t1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP384R1: windows_core::PCWSTR = windows_core::w!("brainpoolP384r1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP384T1: windows_core::PCWSTR = windows_core::w!("brainpoolP384t1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP512R1: windows_core::PCWSTR = windows_core::w!("brainpoolP512r1");
pub const BCRYPT_ECC_CURVE_BRAINPOOLP512T1: windows_core::PCWSTR = windows_core::w!("brainpoolP512t1");
pub const BCRYPT_ECC_CURVE_EC192WAPI: windows_core::PCWSTR = windows_core::w!("ec192wapi");
pub const BCRYPT_ECC_CURVE_NAME: windows_core::PCWSTR = windows_core::w!("ECCCurveName");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_ECC_CURVE_NAMES {
    pub dwEccCurveNames: super::ULONG,
    pub pEccCurveNames: *mut windows_core::PWSTR,
}
pub const BCRYPT_ECC_CURVE_NAME_LIST: windows_core::PCWSTR = windows_core::w!("ECCCurveNameList");
pub const BCRYPT_ECC_CURVE_NISTP192: windows_core::PCWSTR = windows_core::w!("nistP192");
pub const BCRYPT_ECC_CURVE_NISTP224: windows_core::PCWSTR = windows_core::w!("nistP224");
pub const BCRYPT_ECC_CURVE_NISTP256: windows_core::PCWSTR = windows_core::w!("nistP256");
pub const BCRYPT_ECC_CURVE_NISTP384: windows_core::PCWSTR = windows_core::w!("nistP384");
pub const BCRYPT_ECC_CURVE_NISTP521: windows_core::PCWSTR = windows_core::w!("nistP521");
pub const BCRYPT_ECC_CURVE_NUMSP256T1: windows_core::PCWSTR = windows_core::w!("numsP256t1");
pub const BCRYPT_ECC_CURVE_NUMSP384T1: windows_core::PCWSTR = windows_core::w!("numsP384t1");
pub const BCRYPT_ECC_CURVE_NUMSP512T1: windows_core::PCWSTR = windows_core::w!("numsP512t1");
pub const BCRYPT_ECC_CURVE_SECP160K1: windows_core::PCWSTR = windows_core::w!("secP160k1");
pub const BCRYPT_ECC_CURVE_SECP160R1: windows_core::PCWSTR = windows_core::w!("secP160r1");
pub const BCRYPT_ECC_CURVE_SECP160R2: windows_core::PCWSTR = windows_core::w!("secP160r2");
pub const BCRYPT_ECC_CURVE_SECP192K1: windows_core::PCWSTR = windows_core::w!("secP192k1");
pub const BCRYPT_ECC_CURVE_SECP192R1: windows_core::PCWSTR = windows_core::w!("secP192r1");
pub const BCRYPT_ECC_CURVE_SECP224K1: windows_core::PCWSTR = windows_core::w!("secP224k1");
pub const BCRYPT_ECC_CURVE_SECP224R1: windows_core::PCWSTR = windows_core::w!("secP224r1");
pub const BCRYPT_ECC_CURVE_SECP256K1: windows_core::PCWSTR = windows_core::w!("secP256k1");
pub const BCRYPT_ECC_CURVE_SECP256R1: windows_core::PCWSTR = windows_core::w!("secP256r1");
pub const BCRYPT_ECC_CURVE_SECP384R1: windows_core::PCWSTR = windows_core::w!("secP384r1");
pub const BCRYPT_ECC_CURVE_SECP521R1: windows_core::PCWSTR = windows_core::w!("secP521r1");
pub const BCRYPT_ECC_CURVE_WTLS12: windows_core::PCWSTR = windows_core::w!("wtls12");
pub const BCRYPT_ECC_CURVE_WTLS7: windows_core::PCWSTR = windows_core::w!("wtls7");
pub const BCRYPT_ECC_CURVE_WTLS9: windows_core::PCWSTR = windows_core::w!("wtls9");
pub const BCRYPT_ECC_CURVE_X962P192V1: windows_core::PCWSTR = windows_core::w!("x962P192v1");
pub const BCRYPT_ECC_CURVE_X962P192V2: windows_core::PCWSTR = windows_core::w!("x962P192v2");
pub const BCRYPT_ECC_CURVE_X962P192V3: windows_core::PCWSTR = windows_core::w!("x962P192v3");
pub const BCRYPT_ECC_CURVE_X962P239V1: windows_core::PCWSTR = windows_core::w!("x962P239v1");
pub const BCRYPT_ECC_CURVE_X962P239V2: windows_core::PCWSTR = windows_core::w!("x962P239v2");
pub const BCRYPT_ECC_CURVE_X962P239V3: windows_core::PCWSTR = windows_core::w!("x962P239v3");
pub const BCRYPT_ECC_CURVE_X962P256V1: windows_core::PCWSTR = windows_core::w!("x962P256v1");
pub const BCRYPT_ECC_FULLKEY_BLOB_V1: i32 = 1;
pub const BCRYPT_ECC_PARAMETERS: windows_core::PCWSTR = windows_core::w!("ECCParameters");
pub const BCRYPT_ECC_PARAMETERS_MAGIC: i32 = 1346585413;
pub const BCRYPT_ECC_PRIME_MONTGOMERY_CURVE: ECC_CURVE_TYPE_ENUM = 3;
pub const BCRYPT_ECC_PRIME_SHORT_WEIERSTRASS_CURVE: ECC_CURVE_TYPE_ENUM = 1;
pub const BCRYPT_ECC_PRIME_TWISTED_EDWARDS_CURVE: ECC_CURVE_TYPE_ENUM = 2;
pub const BCRYPT_ECDH_ALGORITHM: windows_core::PCWSTR = windows_core::w!("ECDH");
pub const BCRYPT_ECDH_ALG_HANDLE: BCRYPT_ALG_HANDLE = 657 as _;
pub const BCRYPT_ECDH_P256_ALGORITHM: windows_core::PCWSTR = windows_core::w!("ECDH_P256");
pub const BCRYPT_ECDH_P256_ALG_HANDLE: BCRYPT_ALG_HANDLE = 673 as _;
pub const BCRYPT_ECDH_P384_ALGORITHM: windows_core::PCWSTR = windows_core::w!("ECDH_P384");
pub const BCRYPT_ECDH_P384_ALG_HANDLE: BCRYPT_ALG_HANDLE = 689 as _;
pub const BCRYPT_ECDH_P521_ALGORITHM: windows_core::PCWSTR = windows_core::w!("ECDH_P521");
pub const BCRYPT_ECDH_P521_ALG_HANDLE: BCRYPT_ALG_HANDLE = 705 as _;
pub const BCRYPT_ECDH_PRIVATE_GENERIC_MAGIC: i32 = 1447772997;
pub const BCRYPT_ECDH_PRIVATE_P256_MAGIC: i32 = 843793221;
pub const BCRYPT_ECDH_PRIVATE_P384_MAGIC: i32 = 877347653;
pub const BCRYPT_ECDH_PRIVATE_P521_MAGIC: i32 = 910902085;
pub const BCRYPT_ECDH_PUBLIC_GENERIC_MAGIC: i32 = 1347109701;
pub const BCRYPT_ECDH_PUBLIC_P256_MAGIC: i32 = 827016005;
pub const BCRYPT_ECDH_PUBLIC_P384_MAGIC: i32 = 860570437;
pub const BCRYPT_ECDH_PUBLIC_P521_MAGIC: i32 = 894124869;
pub const BCRYPT_ECDSA_ALGORITHM: windows_core::PCWSTR = windows_core::w!("ECDSA");
pub const BCRYPT_ECDSA_ALG_HANDLE: BCRYPT_ALG_HANDLE = 241 as _;
pub const BCRYPT_ECDSA_P256_ALGORITHM: windows_core::PCWSTR = windows_core::w!("ECDSA_P256");
pub const BCRYPT_ECDSA_P256_ALG_HANDLE: BCRYPT_ALG_HANDLE = 737 as _;
pub const BCRYPT_ECDSA_P384_ALGORITHM: windows_core::PCWSTR = windows_core::w!("ECDSA_P384");
pub const BCRYPT_ECDSA_P384_ALG_HANDLE: BCRYPT_ALG_HANDLE = 753 as _;
pub const BCRYPT_ECDSA_P521_ALGORITHM: windows_core::PCWSTR = windows_core::w!("ECDSA_P521");
pub const BCRYPT_ECDSA_P521_ALG_HANDLE: BCRYPT_ALG_HANDLE = 769 as _;
pub const BCRYPT_ECDSA_PRIVATE_GENERIC_MAGIC: i32 = 1447314245;
pub const BCRYPT_ECDSA_PRIVATE_P256_MAGIC: i32 = 844317509;
pub const BCRYPT_ECDSA_PRIVATE_P384_MAGIC: i32 = 877871941;
pub const BCRYPT_ECDSA_PRIVATE_P521_MAGIC: i32 = 911426373;
pub const BCRYPT_ECDSA_PUBLIC_GENERIC_MAGIC: i32 = 1346650949;
pub const BCRYPT_ECDSA_PUBLIC_P256_MAGIC: i32 = 827540293;
pub const BCRYPT_ECDSA_PUBLIC_P384_MAGIC: i32 = 861094725;
pub const BCRYPT_ECDSA_PUBLIC_P521_MAGIC: i32 = 894649157;
pub const BCRYPT_EFFECTIVE_KEY_LENGTH: windows_core::PCWSTR = windows_core::w!("EffectiveKeyLength");
pub const BCRYPT_ENABLE_INCOMPATIBLE_FIPS_CHECKS: i32 = 256;
pub const BCRYPT_EXTENDED_KEYSIZE: i32 = 128;
pub const BCRYPT_FUNCTION_NAME_STRING: windows_core::PCWSTR = windows_core::w!("FunctionNameString");
pub const BCRYPT_GENERATE_IV: i32 = 32;
pub const BCRYPT_GLOBAL_PARAMETERS: windows_core::PCWSTR = windows_core::w!("SecretAgreementParam");
pub type BCRYPT_HANDLE = *mut core::ffi::c_void;
pub const BCRYPT_HASH_BLOCK_LENGTH: windows_core::PCWSTR = windows_core::w!("HashBlockLength");
pub const BCRYPT_HASH_DONT_RESET_FLAG: i32 = 1;
pub type BCRYPT_HASH_HANDLE = *mut core::ffi::c_void;
pub const BCRYPT_HASH_INTERFACE: i32 = 2;
pub const BCRYPT_HASH_INTERFACE_MAJORVERSION_2: i32 = 2;
pub const BCRYPT_HASH_LENGTH: windows_core::PCWSTR = windows_core::w!("HashDigestLength");
pub const BCRYPT_HASH_OID_LIST: windows_core::PCWSTR = windows_core::w!("HashOIDList");
pub const BCRYPT_HASH_OPERATION: i32 = 2;
pub const BCRYPT_HASH_OPERATION_FINISH_HASH: BCRYPT_HASH_OPERATION_TYPE = 2;
pub const BCRYPT_HASH_OPERATION_HASH_DATA: BCRYPT_HASH_OPERATION_TYPE = 1;
pub type BCRYPT_HASH_OPERATION_TYPE = i32;
pub const BCRYPT_HASH_REUSABLE_FLAG: i32 = 32;
pub const BCRYPT_HKDF_ALGORITHM: windows_core::PCWSTR = windows_core::w!("HKDF");
pub const BCRYPT_HKDF_ALG_HANDLE: BCRYPT_ALG_HANDLE = 913 as _;
pub const BCRYPT_HKDF_HASH_ALGORITHM: windows_core::PCWSTR = windows_core::w!("HkdfHashAlgorithm");
pub const BCRYPT_HKDF_PRK_AND_FINALIZE: windows_core::PCWSTR = windows_core::w!("HkdfPrkAndFinalize");
pub const BCRYPT_HKDF_SALT_AND_FINALIZE: windows_core::PCWSTR = windows_core::w!("HkdfSaltAndFinalize");
pub const BCRYPT_HMAC_MD2_ALG_HANDLE: BCRYPT_ALG_HANDLE = 289 as _;
pub const BCRYPT_HMAC_MD4_ALG_HANDLE: BCRYPT_ALG_HANDLE = 305 as _;
pub const BCRYPT_HMAC_MD5_ALG_HANDLE: BCRYPT_ALG_HANDLE = 145 as _;
pub const BCRYPT_HMAC_SHA1_ALG_HANDLE: BCRYPT_ALG_HANDLE = 161 as _;
pub const BCRYPT_HMAC_SHA256_ALG_HANDLE: BCRYPT_ALG_HANDLE = 177 as _;
pub const BCRYPT_HMAC_SHA384_ALG_HANDLE: BCRYPT_ALG_HANDLE = 193 as _;
pub const BCRYPT_HMAC_SHA3_256_ALG_HANDLE: BCRYPT_ALG_HANDLE = 993 as _;
pub const BCRYPT_HMAC_SHA3_384_ALG_HANDLE: BCRYPT_ALG_HANDLE = 1009 as _;
pub const BCRYPT_HMAC_SHA3_512_ALG_HANDLE: BCRYPT_ALG_HANDLE = 1025 as _;
pub const BCRYPT_HMAC_SHA512_ALG_HANDLE: BCRYPT_ALG_HANDLE = 209 as _;
pub const BCRYPT_INITIALIZATION_VECTOR: windows_core::PCWSTR = windows_core::w!("IV");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_INTERFACE_VERSION {
    pub MajorVersion: super::USHORT,
    pub MinorVersion: super::USHORT,
}
pub const BCRYPT_IS_IFX_TPM_WEAK_KEY: windows_core::PCWSTR = windows_core::w!("IsIfxTpmWeakKey");
pub const BCRYPT_IS_KEYED_HASH: windows_core::PCWSTR = windows_core::w!("IsKeyedHash");
pub const BCRYPT_IS_REUSABLE_HASH: windows_core::PCWSTR = windows_core::w!("IsReusableHash");
pub const BCRYPT_KDF_HASH: windows_core::PCWSTR = windows_core::w!("HASH");
pub const BCRYPT_KDF_HKDF: windows_core::PCWSTR = windows_core::w!("HKDF");
pub const BCRYPT_KDF_HMAC: windows_core::PCWSTR = windows_core::w!("HMAC");
pub const BCRYPT_KDF_RAW_SECRET: windows_core::PCWSTR = windows_core::w!("TRUNCATE");
pub const BCRYPT_KDF_SP80056A_CONCAT: windows_core::PCWSTR = windows_core::w!("SP800_56A_CONCAT");
pub const BCRYPT_KDF_TLS_PRF: windows_core::PCWSTR = windows_core::w!("TLS_PRF");
pub const BCRYPT_KEM_CIPHERTEXT_LENGTH: windows_core::PCWSTR = windows_core::w!("KEMCiphertextLength");
pub const BCRYPT_KEM_SHARED_SECRET_LENGTH: windows_core::PCWSTR = windows_core::w!("KEMSharedSecretLength");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_KEY_BLOB {
    pub Magic: super::ULONG,
}
pub const BCRYPT_KEY_DATA_BLOB: windows_core::PCWSTR = windows_core::w!("KeyDataBlob");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_KEY_DATA_BLOB_HEADER {
    pub dwMagic: super::ULONG,
    pub dwVersion: super::ULONG,
    pub cbKeyData: super::ULONG,
}
pub const BCRYPT_KEY_DATA_BLOB_MAGIC: i32 = 1296188491;
pub const BCRYPT_KEY_DATA_BLOB_VERSION1: i32 = 1;
pub const BCRYPT_KEY_DERIVATION_INTERFACE: i32 = 7;
pub const BCRYPT_KEY_DERIVATION_OPERATION: i32 = 64;
pub const BCRYPT_KEY_ENCAPSULATION_INTERFACE: i32 = 8;
pub const BCRYPT_KEY_ENCAPSULATION_OPERATION: i32 = 128;
pub type BCRYPT_KEY_HANDLE = *mut core::ffi::c_void;
pub const BCRYPT_KEY_LENGTH: windows_core::PCWSTR = windows_core::w!("KeyLength");
pub const BCRYPT_KEY_LENGTHS: windows_core::PCWSTR = windows_core::w!("KeyLengths");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_KEY_LENGTHS_STRUCT {
    pub dwMinLength: super::ULONG,
    pub dwMaxLength: super::ULONG,
    pub dwIncrement: super::ULONG,
}
pub const BCRYPT_KEY_OBJECT_LENGTH: windows_core::PCWSTR = windows_core::w!("KeyObjectLength");
pub const BCRYPT_KEY_STRENGTH: windows_core::PCWSTR = windows_core::w!("KeyStrength");
pub const BCRYPT_KEY_VALIDATION_RANGE: i32 = 16;
pub const BCRYPT_KEY_VALIDATION_RANGE_AND_ORDER: i32 = 24;
pub const BCRYPT_KEY_VALIDATION_REGENERATE: i32 = 32;
pub const BCRYPT_KMAC128_ALGORITHM: windows_core::PCWSTR = windows_core::w!("KMAC128");
pub const BCRYPT_KMAC128_ALG_HANDLE: BCRYPT_ALG_HANDLE = 1073 as _;
pub const BCRYPT_KMAC256_ALGORITHM: windows_core::PCWSTR = windows_core::w!("KMAC256");
pub const BCRYPT_KMAC256_ALG_HANDLE: BCRYPT_ALG_HANDLE = 1089 as _;
pub const BCRYPT_LMS_ALGORITHM: windows_core::PCWSTR = windows_core::w!("LMS");
pub const BCRYPT_LMS_PUBLIC_MAGIC: i32 = 1263553868;
pub const BCRYPT_MD2_ALGORITHM: windows_core::PCWSTR = windows_core::w!("MD2");
pub const BCRYPT_MD2_ALG_HANDLE: BCRYPT_ALG_HANDLE = 1 as _;
pub const BCRYPT_MD4_ALGORITHM: windows_core::PCWSTR = windows_core::w!("MD4");
pub const BCRYPT_MD4_ALG_HANDLE: BCRYPT_ALG_HANDLE = 17 as _;
pub const BCRYPT_MD5_ALGORITHM: windows_core::PCWSTR = windows_core::w!("MD5");
pub const BCRYPT_MD5_ALG_HANDLE: BCRYPT_ALG_HANDLE = 33 as _;
pub const BCRYPT_MESSAGE_BLOCK_LENGTH: windows_core::PCWSTR = windows_core::w!("MessageBlockLength");
pub const BCRYPT_MLDSA_ALGORITHM: windows_core::PCWSTR = windows_core::w!("ML-DSA");
pub const BCRYPT_MLDSA_ALG_HANDLE: BCRYPT_ALG_HANDLE = 1137 as _;
pub const BCRYPT_MLDSA_EXTERNAL_MU: i32 = 64;
pub const BCRYPT_MLDSA_PARAMETER_SET_44: windows_core::PCWSTR = windows_core::w!("44");
pub const BCRYPT_MLDSA_PARAMETER_SET_65: windows_core::PCWSTR = windows_core::w!("65");
pub const BCRYPT_MLDSA_PARAMETER_SET_87: windows_core::PCWSTR = windows_core::w!("87");
pub const BCRYPT_MLDSA_PRIVATE_MAGIC: i32 = 1263752004;
pub const BCRYPT_MLDSA_PRIVATE_SEED_MAGIC: i32 = 1397969732;
pub const BCRYPT_MLDSA_PUBLIC_MAGIC: i32 = 1263555396;
pub const BCRYPT_MLKEM_ALGORITHM: windows_core::PCWSTR = windows_core::w!("ML-KEM");
pub const BCRYPT_MLKEM_ALG_HANDLE: BCRYPT_ALG_HANDLE = 1153 as _;
pub const BCRYPT_MLKEM_DECAPSULATION_BLOB: windows_core::PCWSTR = windows_core::w!("MLKEMPRIVATEBLOB");
pub const BCRYPT_MLKEM_ENCAPSULATION_BLOB: windows_core::PCWSTR = windows_core::w!("MLKEMPUBLICBLOB");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_MLKEM_KEY_BLOB {
    pub dwMagic: super::ULONG,
    pub cbParameterSet: super::ULONG,
    pub cbKey: super::ULONG,
}
pub const BCRYPT_MLKEM_PARAMETER_SET_1024: windows_core::PCWSTR = windows_core::w!("1024");
pub const BCRYPT_MLKEM_PARAMETER_SET_512: windows_core::PCWSTR = windows_core::w!("512");
pub const BCRYPT_MLKEM_PARAMETER_SET_768: windows_core::PCWSTR = windows_core::w!("768");
pub const BCRYPT_MLKEM_PRIVATE_BLOB: windows_core::PCWSTR = windows_core::w!("MLKEMPRIVATEBLOB");
pub const BCRYPT_MLKEM_PRIVATE_MAGIC: i32 = 1380666445;
pub const BCRYPT_MLKEM_PRIVATE_SEED_BLOB: windows_core::PCWSTR = windows_core::w!("MLKEMPRIVATESEEDBLOB");
pub const BCRYPT_MLKEM_PRIVATE_SEED_MAGIC: i32 = 1397443661;
pub const BCRYPT_MLKEM_PUBLIC_BLOB: windows_core::PCWSTR = windows_core::w!("MLKEMPUBLICBLOB");
pub const BCRYPT_MLKEM_PUBLIC_MAGIC: i32 = 1347112013;
pub const BCRYPT_MULTI_FLAG: i32 = 64;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_MULTI_HASH_OPERATION {
    pub iHash: super::ULONG,
    pub hashOperation: BCRYPT_HASH_OPERATION_TYPE,
    pub pbBuffer: super::PUCHAR,
    pub cbBuffer: super::ULONG,
}
pub const BCRYPT_MULTI_OBJECT_LENGTH: windows_core::PCWSTR = windows_core::w!("MultiObjectLength");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_MULTI_OBJECT_LENGTH_STRUCT {
    pub cbPerObject: super::ULONG,
    pub cbPerElement: super::ULONG,
}
pub type BCRYPT_MULTI_OPERATION_TYPE = i32;
pub const BCRYPT_NO_CURVE_GENERATION_ALG_ID: ECC_CURVE_ALG_ID_ENUM = 0;
pub const BCRYPT_NO_KEY_VALIDATION: i32 = 8;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_OAEP_PADDING_INFO {
    pub pszAlgId: windows_core::PCWSTR,
    pub pbLabel: super::PUCHAR,
    pub cbLabel: super::ULONG,
}
pub const BCRYPT_OBJECT_ALIGNMENT: i32 = 16;
pub const BCRYPT_OBJECT_LENGTH: windows_core::PCWSTR = windows_core::w!("ObjectLength");
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_OID {
    pub cbOID: super::ULONG,
    pub pbOID: super::PUCHAR,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_OID_LIST {
    pub dwOIDCount: super::ULONG,
    pub pOIDs: *mut BCRYPT_OID,
}
pub const BCRYPT_OPAQUE_KEY_BLOB: windows_core::PCWSTR = windows_core::w!("OpaqueKeyBlob");
pub const BCRYPT_OPERATION_TYPE_HASH: BCRYPT_MULTI_OPERATION_TYPE = 1;
pub const BCRYPT_PADDING_SCHEMES: windows_core::PCWSTR = windows_core::w!("PaddingSchemes");
pub const BCRYPT_PAD_NONE: i32 = 1;
pub const BCRYPT_PAD_OAEP: i32 = 4;
pub const BCRYPT_PAD_PKCS1: i32 = 2;
pub const BCRYPT_PAD_PKCS1_OPTIONAL_HASH_OID: i32 = 16;
pub const BCRYPT_PAD_PQDSA: i32 = 32;
pub const BCRYPT_PAD_PSS: i32 = 8;
pub const BCRYPT_PARAMETER_SET_NAME: windows_core::PCWSTR = windows_core::w!("ParameterSetName");
pub const BCRYPT_PBKDF2_ALGORITHM: windows_core::PCWSTR = windows_core::w!("PBKDF2");
pub const BCRYPT_PBKDF2_ALG_HANDLE: BCRYPT_ALG_HANDLE = 817 as _;
pub const BCRYPT_PCP_PLATFORM_TYPE_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_PLATFORM_TYPE");
pub const BCRYPT_PCP_PROVIDER_VERSION_PROPERTY: windows_core::PCWSTR = windows_core::w!("PCP_PROVIDER_VERSION");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_PKCS11_RSA_AES_WRAP_BLOB {
    pub dwMagic: super::ULONG,
    pub cbKey: super::ULONG,
    pub cbPaddingAlgId: super::ULONG,
    pub cbPaddingLabel: super::ULONG,
}
pub const BCRYPT_PKCS11_RSA_AES_WRAP_BLOB_MAGIC: i32 = 1464877394;
pub const BCRYPT_PKCS11_RSA_AES_WRAP_KEY_BLOB: windows_core::PCWSTR = windows_core::w!("PKCS11RsaAesWrapBlob");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_PKCS1_PADDING_INFO {
    pub pszAlgId: windows_core::PCWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_PQDSA_KEY_BLOB {
    pub dwMagic: super::ULONG,
    pub cbParameterSet: super::ULONG,
    pub cbKey: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_PQDSA_PADDING_INFO {
    pub pbCtx: super::PUCHAR,
    pub cbCtx: super::ULONG,
    pub pszPrehashAlgId: windows_core::PCWSTR,
}
pub const BCRYPT_PQDSA_PRIVATE_BLOB: windows_core::PCWSTR = windows_core::w!("PQDSAPRIVATEBLOB");
pub const BCRYPT_PQDSA_PRIVATE_SEED_BLOB: windows_core::PCWSTR = windows_core::w!("PQDSAPRIVATESEEDBLOB");
pub const BCRYPT_PQDSA_PUBLIC_BLOB: windows_core::PCWSTR = windows_core::w!("PQDSAPUBLICBLOB");
pub const BCRYPT_PRIMITIVE_TYPE: windows_core::PCWSTR = windows_core::w!("PrimitiveType");
pub const BCRYPT_PRIVATE_KEY: windows_core::PCWSTR = windows_core::w!("PrivKeyVal");
pub const BCRYPT_PRIVATE_KEY_BLOB: windows_core::PCWSTR = windows_core::w!("PRIVATEBLOB");
pub const BCRYPT_PRIVATE_KEY_FLAG: i32 = 2;
pub const BCRYPT_PROVIDER_HANDLE: windows_core::PCWSTR = windows_core::w!("ProviderHandle");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_PROVIDER_NAME {
    pub pszProviderName: windows_core::PWSTR,
}
pub const BCRYPT_PROV_DISPATCH: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_PSS_PADDING_INFO {
    pub pszAlgId: windows_core::PCWSTR,
    pub cbSalt: super::ULONG,
}
pub const BCRYPT_PUBLIC_KEY_BLOB: windows_core::PCWSTR = windows_core::w!("PUBLICBLOB");
pub const BCRYPT_PUBLIC_KEY_FLAG: i32 = 1;
pub const BCRYPT_PUBLIC_KEY_LENGTH: windows_core::PCWSTR = windows_core::w!("PublicKeyLength");
pub const BCRYPT_RC2_ALGORITHM: windows_core::PCWSTR = windows_core::w!("RC2");
pub const BCRYPT_RC2_CBC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 593 as _;
pub const BCRYPT_RC2_CFB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 625 as _;
pub const BCRYPT_RC2_ECB_ALG_HANDLE: BCRYPT_ALG_HANDLE = 609 as _;
pub const BCRYPT_RC4_ALGORITHM: windows_core::PCWSTR = windows_core::w!("RC4");
pub const BCRYPT_RC4_ALG_HANDLE: BCRYPT_ALG_HANDLE = 113 as _;
pub const BCRYPT_RNG_ALGORITHM: windows_core::PCWSTR = windows_core::w!("RNG");
pub const BCRYPT_RNG_ALG_HANDLE: BCRYPT_ALG_HANDLE = 129 as _;
pub const BCRYPT_RNG_DUAL_EC_ALGORITHM: windows_core::PCWSTR = windows_core::w!("DUALECRNG");
pub const BCRYPT_RNG_FIPS186_DSA_ALGORITHM: windows_core::PCWSTR = windows_core::w!("FIPS186DSARNG");
pub const BCRYPT_RNG_INTERFACE: i32 = 6;
pub const BCRYPT_RNG_OPERATION: i32 = 32;
pub const BCRYPT_RNG_USE_ENTROPY_IN_BUFFER: i32 = 1;
pub const BCRYPT_RSAFULLPRIVATE_BLOB: windows_core::PCWSTR = windows_core::w!("RSAFULLPRIVATEBLOB");
pub const BCRYPT_RSAFULLPRIVATE_MAGIC: i32 = 859919186;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCRYPT_RSAKEY_BLOB {
    pub Magic: super::ULONG,
    pub BitLength: super::ULONG,
    pub cbPublicExp: super::ULONG,
    pub cbModulus: super::ULONG,
    pub cbPrime1: super::ULONG,
    pub cbPrime2: super::ULONG,
}
pub const BCRYPT_RSAPRIVATE_BLOB: windows_core::PCWSTR = windows_core::w!("RSAPRIVATEBLOB");
pub const BCRYPT_RSAPRIVATE_MAGIC: i32 = 843141970;
pub const BCRYPT_RSAPUBLIC_BLOB: windows_core::PCWSTR = windows_core::w!("RSAPUBLICBLOB");
pub const BCRYPT_RSAPUBLIC_MAGIC: i32 = 826364754;
pub const BCRYPT_RSA_ALGORITHM: windows_core::PCWSTR = windows_core::w!("RSA");
pub const BCRYPT_RSA_ALG_HANDLE: BCRYPT_ALG_HANDLE = 225 as _;
pub const BCRYPT_RSA_SIGN_ALGORITHM: windows_core::PCWSTR = windows_core::w!("RSA_SIGN");
pub const BCRYPT_RSA_SIGN_ALG_HANDLE: BCRYPT_ALG_HANDLE = 785 as _;
pub const BCRYPT_SECRET_AGREEMENT_INTERFACE: i32 = 4;
pub const BCRYPT_SECRET_AGREEMENT_OPERATION: i32 = 8;
pub type BCRYPT_SECRET_HANDLE = *mut core::ffi::c_void;
pub const BCRYPT_SHA1_ALGORITHM: windows_core::PCWSTR = windows_core::w!("SHA1");
pub const BCRYPT_SHA1_ALG_HANDLE: BCRYPT_ALG_HANDLE = 49 as _;
pub const BCRYPT_SHA256_ALGORITHM: windows_core::PCWSTR = windows_core::w!("SHA256");
pub const BCRYPT_SHA256_ALG_HANDLE: BCRYPT_ALG_HANDLE = 65 as _;
pub const BCRYPT_SHA384_ALGORITHM: windows_core::PCWSTR = windows_core::w!("SHA384");
pub const BCRYPT_SHA384_ALG_HANDLE: BCRYPT_ALG_HANDLE = 81 as _;
pub const BCRYPT_SHA3_256_ALGORITHM: windows_core::PCWSTR = windows_core::w!("SHA3-256");
pub const BCRYPT_SHA3_256_ALG_HANDLE: BCRYPT_ALG_HANDLE = 945 as _;
pub const BCRYPT_SHA3_384_ALGORITHM: windows_core::PCWSTR = windows_core::w!("SHA3-384");
pub const BCRYPT_SHA3_384_ALG_HANDLE: BCRYPT_ALG_HANDLE = 961 as _;
pub const BCRYPT_SHA3_512_ALGORITHM: windows_core::PCWSTR = windows_core::w!("SHA3-512");
pub const BCRYPT_SHA3_512_ALG_HANDLE: BCRYPT_ALG_HANDLE = 977 as _;
pub const BCRYPT_SHA512_ALGORITHM: windows_core::PCWSTR = windows_core::w!("SHA512");
pub const BCRYPT_SHA512_ALG_HANDLE: BCRYPT_ALG_HANDLE = 97 as _;
pub const BCRYPT_SHAKE128_ALGORITHM: windows_core::PCWSTR = windows_core::w!("SHAKE128");
pub const BCRYPT_SHAKE128_ALG_HANDLE: BCRYPT_ALG_HANDLE = 1105 as _;
pub const BCRYPT_SHAKE256_ALGORITHM: windows_core::PCWSTR = windows_core::w!("SHAKE256");
pub const BCRYPT_SHAKE256_ALG_HANDLE: BCRYPT_ALG_HANDLE = 1121 as _;
pub const BCRYPT_SIGNATURE_INTERFACE: i32 = 5;
pub const BCRYPT_SIGNATURE_LENGTH: windows_core::PCWSTR = windows_core::w!("SignatureLength");
pub const BCRYPT_SIGNATURE_OPERATION: i32 = 16;
pub const BCRYPT_SLHDSA_ALGORITHM: windows_core::PCWSTR = windows_core::w!("SLH-DSA");
pub const BCRYPT_SLHDSA_PARAMETER_SET_SHA2_128F: windows_core::PCWSTR = windows_core::w!("SHA2-128f");
pub const BCRYPT_SLHDSA_PARAMETER_SET_SHA2_128S: windows_core::PCWSTR = windows_core::w!("SHA2-128s");
pub const BCRYPT_SLHDSA_PARAMETER_SET_SHA2_192F: windows_core::PCWSTR = windows_core::w!("SHA2-192f");
pub const BCRYPT_SLHDSA_PARAMETER_SET_SHA2_192S: windows_core::PCWSTR = windows_core::w!("SHA2-192s");
pub const BCRYPT_SLHDSA_PARAMETER_SET_SHA2_256F: windows_core::PCWSTR = windows_core::w!("SHA2-256f");
pub const BCRYPT_SLHDSA_PARAMETER_SET_SHA2_256S: windows_core::PCWSTR = windows_core::w!("SHA2-256s");
pub const BCRYPT_SLHDSA_PARAMETER_SET_SHAKE_128F: windows_core::PCWSTR = windows_core::w!("SHAKE-128f");
pub const BCRYPT_SLHDSA_PARAMETER_SET_SHAKE_128S: windows_core::PCWSTR = windows_core::w!("SHAKE-128s");
pub const BCRYPT_SLHDSA_PARAMETER_SET_SHAKE_192F: windows_core::PCWSTR = windows_core::w!("SHAKE-192f");
pub const BCRYPT_SLHDSA_PARAMETER_SET_SHAKE_192S: windows_core::PCWSTR = windows_core::w!("SHAKE-192s");
pub const BCRYPT_SLHDSA_PARAMETER_SET_SHAKE_256F: windows_core::PCWSTR = windows_core::w!("SHAKE-256f");
pub const BCRYPT_SLHDSA_PARAMETER_SET_SHAKE_256S: windows_core::PCWSTR = windows_core::w!("SHAKE-256s");
pub const BCRYPT_SLHDSA_PRIVATE_MAGIC: i32 = 1263749203;
pub const BCRYPT_SLHDSA_PUBLIC_MAGIC: i32 = 1263552595;
pub const BCRYPT_SP800108_CTR_HMAC_ALGORITHM: windows_core::PCWSTR = windows_core::w!("SP800_108_CTR_HMAC");
pub const BCRYPT_SP800108_CTR_HMAC_ALG_HANDLE: BCRYPT_ALG_HANDLE = 833 as _;
pub const BCRYPT_SP80056A_CONCAT_ALGORITHM: windows_core::PCWSTR = windows_core::w!("SP800_56A_CONCAT");
pub const BCRYPT_SP80056A_CONCAT_ALG_HANDLE: BCRYPT_ALG_HANDLE = 849 as _;
pub const BCRYPT_SUPPORTED_PAD_OAEP: i32 = 8;
pub const BCRYPT_SUPPORTED_PAD_PKCS1_ENC: i32 = 2;
pub const BCRYPT_SUPPORTED_PAD_PKCS1_SIG: i32 = 4;
pub const BCRYPT_SUPPORTED_PAD_PSS: i32 = 16;
pub const BCRYPT_SUPPORTED_PAD_ROUTER: i32 = 1;
pub const BCRYPT_TLS1_1_KDF_ALGORITHM: windows_core::PCWSTR = windows_core::w!("TLS1_1_KDF");
pub const BCRYPT_TLS1_1_KDF_ALG_HANDLE: BCRYPT_ALG_HANDLE = 865 as _;
pub const BCRYPT_TLS1_2_KDF_ALGORITHM: windows_core::PCWSTR = windows_core::w!("TLS1_2_KDF");
pub const BCRYPT_TLS1_2_KDF_ALG_HANDLE: BCRYPT_ALG_HANDLE = 881 as _;
pub const BCRYPT_TLS_CBC_HMAC_VERIFY_FLAG: i32 = 4;
pub const BCRYPT_USE_SYSTEM_PREFERRED_RNG: i32 = 2;
pub const BCRYPT_XMSS_ALGORITHM: windows_core::PCWSTR = windows_core::w!("XMSS");
pub const BCRYPT_XMSS_PUBLIC_MAGIC: i32 = 1263553880;
pub const BCRYPT_XTS_AES_ALGORITHM: windows_core::PCWSTR = windows_core::w!("XTS-AES");
pub const BCRYPT_XTS_AES_ALG_HANDLE: BCRYPT_ALG_HANDLE = 897 as _;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCryptBuffer {
    pub cbBuffer: super::ULONG,
    pub BufferType: super::ULONG,
    pub pvBuffer: super::PVOID,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BCryptBufferDesc {
    pub ulVersion: super::ULONG,
    pub cBuffers: super::ULONG,
    pub pBuffers: PBCryptBuffer,
}
pub const CRYPT_ALL_FUNCTIONS: i32 = 1;
pub const CRYPT_ALL_PROVIDERS: i32 = 2;
pub const CRYPT_ANY: i32 = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_CONTEXTS {
    pub cContexts: super::ULONG,
    pub rgpszContexts: *mut windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_CONTEXT_CONFIG {
    pub dwFlags: super::ULONG,
    pub dwReserved: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_CONTEXT_FUNCTIONS {
    pub cFunctions: super::ULONG,
    pub rgpszFunctions: *mut windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_CONTEXT_FUNCTION_CONFIG {
    pub dwFlags: super::ULONG,
    pub dwReserved: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_CONTEXT_FUNCTION_PROVIDERS {
    pub cProviders: super::ULONG,
    pub rgpszProviders: *mut windows_core::PWSTR,
}
pub const CRYPT_DEFAULT_CONTEXT: windows_core::PCWSTR = windows_core::w!("Default");
pub const CRYPT_DOMAIN: i32 = 2;
pub const CRYPT_EXCLUSIVE: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_IMAGE_REF {
    pub pszImage: windows_core::PWSTR,
    pub dwFlags: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_IMAGE_REG {
    pub pszImage: windows_core::PWSTR,
    pub cInterfaces: super::ULONG,
    pub rgpInterfaces: *mut PCRYPT_INTERFACE_REG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_INTERFACE_REG {
    pub dwInterface: super::ULONG,
    pub dwFlags: super::ULONG,
    pub cFunctions: super::ULONG,
    pub rgpszFunctions: *mut windows_core::PWSTR,
}
pub const CRYPT_KM: i32 = 2;
pub const CRYPT_LOCAL: i32 = 1;
pub const CRYPT_MIN_DEPENDENCIES: i32 = 1;
pub const CRYPT_MM: i32 = 3;
pub const CRYPT_OVERRIDE: i32 = 65536;
pub const CRYPT_OVERWRITE: i32 = 1;
pub const CRYPT_PRIORITY_BOTTOM: u32 = 4294967295;
pub const CRYPT_PRIORITY_TOP: i32 = 0;
pub const CRYPT_PROCESS_ISOLATE: i32 = 65536;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_PROPERTY_REF {
    pub pszProperty: windows_core::PWSTR,
    pub cbValue: super::ULONG,
    pub pbValue: super::PUCHAR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_PROVIDERS {
    pub cProviders: super::ULONG,
    pub rgpszProviders: *mut windows_core::PWSTR,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_PROVIDER_REF {
    pub dwInterface: super::ULONG,
    pub pszFunction: windows_core::PWSTR,
    pub pszProvider: windows_core::PWSTR,
    pub cProperties: super::ULONG,
    pub rgpProperties: *mut PCRYPT_PROPERTY_REF,
    pub pUM: PCRYPT_IMAGE_REF,
    pub pKM: PCRYPT_IMAGE_REF,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_PROVIDER_REFS {
    pub cProviders: super::ULONG,
    pub rgpProviders: *mut PCRYPT_PROVIDER_REF,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CRYPT_PROVIDER_REG {
    pub cAliases: super::ULONG,
    pub rgpszAliases: *mut windows_core::PWSTR,
    pub pUM: PCRYPT_IMAGE_REG,
    pub pKM: PCRYPT_IMAGE_REG,
}
pub const CRYPT_UM: i32 = 1;
pub type DSAFIPSVERSION_ENUM = i32;
pub const DSA_FIPS186_2: DSAFIPSVERSION_ENUM = 0;
pub const DSA_FIPS186_3: DSAFIPSVERSION_ENUM = 1;
pub const DSA_HASH_ALGORITHM_SHA1: HASHALGORITHM_ENUM = 0;
pub const DSA_HASH_ALGORITHM_SHA256: HASHALGORITHM_ENUM = 1;
pub const DSA_HASH_ALGORITHM_SHA512: HASHALGORITHM_ENUM = 2;
pub type ECC_CURVE_ALG_ID_ENUM = i32;
pub type ECC_CURVE_TYPE_ENUM = i32;
pub type HASHALGORITHM_ENUM = i32;
pub const KDF_ALGORITHMID: i32 = 8;
pub const KDF_CONTEXT: i32 = 14;
pub const KDF_GENERIC_PARAMETER: i32 = 17;
pub const KDF_HASH_ALGORITHM: i32 = 0;
pub const KDF_HKDF_INFO: i32 = 20;
pub const KDF_HKDF_SALT: i32 = 19;
pub const KDF_HMAC_KEY: i32 = 3;
pub const KDF_ITERATION_COUNT: i32 = 16;
pub const KDF_KEYBITLENGTH: i32 = 18;
pub const KDF_LABEL: i32 = 13;
pub const KDF_PARTYUINFO: i32 = 9;
pub const KDF_PARTYVINFO: i32 = 10;
pub const KDF_SALT: i32 = 15;
pub const KDF_SECRET_APPEND: i32 = 2;
pub const KDF_SECRET_HANDLE: i32 = 6;
pub const KDF_SECRET_PREPEND: i32 = 1;
pub const KDF_SUPPPRIVINFO: i32 = 12;
pub const KDF_SUPPPUBINFO: i32 = 11;
pub const KDF_TLS_PRF_LABEL: i32 = 4;
pub const KDF_TLS_PRF_PROTOCOL: i32 = 7;
pub const KDF_TLS_PRF_SEED: i32 = 5;
pub const KDF_USE_SECRET_AS_HMAC_KEY_FLAG: i32 = 1;
pub const LEGACY_DH_PRIVATE_BLOB: windows_core::PCWSTR = windows_core::w!("CAPIDHPRIVATEBLOB");
pub const LEGACY_DH_PUBLIC_BLOB: windows_core::PCWSTR = windows_core::w!("CAPIDHPUBLICBLOB");
pub const LEGACY_DSA_PRIVATE_BLOB: windows_core::PCWSTR = windows_core::w!("CAPIDSAPRIVATEBLOB");
pub const LEGACY_DSA_PUBLIC_BLOB: windows_core::PCWSTR = windows_core::w!("CAPIDSAPUBLICBLOB");
pub const LEGACY_DSA_V2_PRIVATE_BLOB: windows_core::PCWSTR = windows_core::w!("V2CAPIDSAPRIVATEBLOB");
pub const LEGACY_DSA_V2_PUBLIC_BLOB: windows_core::PCWSTR = windows_core::w!("V2CAPIDSAPUBLICBLOB");
pub const LEGACY_RSAPRIVATE_BLOB: windows_core::PCWSTR = windows_core::w!("CAPIPRIVATEBLOB");
pub const LEGACY_RSAPUBLIC_BLOB: windows_core::PCWSTR = windows_core::w!("CAPIPUBLICBLOB");
pub const MAX_HYBRID_KEY_EXCHANGE_BLOBS: i32 = 4;
pub const MS_PLATFORM_CRYPTO_PROVIDER: windows_core::PCWSTR = windows_core::w!("Microsoft Platform Crypto Provider");
pub const MS_PRIMITIVE_PROVIDER: windows_core::PCWSTR = windows_core::w!("Microsoft Primitive Provider");
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PBCRYPT_AUTHENTICATED_CIPHER_MODE_INFO = *mut BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO;
#[cfg(feature = "shared")]
pub type PBCRYPT_DH_KEY_BLOB = *mut BCRYPT_DH_KEY_BLOB;
#[cfg(feature = "shared")]
pub type PBCRYPT_DSA_KEY_BLOB = *mut BCRYPT_DSA_KEY_BLOB;
#[cfg(feature = "shared")]
pub type PBCRYPT_DSA_KEY_BLOB_V2 = *mut BCRYPT_DSA_KEY_BLOB_V2;
#[cfg(feature = "shared")]
pub type PBCRYPT_ECCFULLKEY_BLOB = *mut BCRYPT_ECCFULLKEY_BLOB;
#[cfg(feature = "shared")]
pub type PBCRYPT_ECCKEY_BLOB = *mut BCRYPT_ECCKEY_BLOB;
#[cfg(feature = "shared")]
pub type PBCRYPT_INTERFACE_VERSION = *mut BCRYPT_INTERFACE_VERSION;
#[cfg(feature = "shared")]
pub type PBCRYPT_KEY_DATA_BLOB_HEADER = *mut BCRYPT_KEY_DATA_BLOB_HEADER;
#[cfg(feature = "shared")]
pub type PBCRYPT_MLKEM_KEY_BLOB = *mut BCRYPT_MLKEM_KEY_BLOB;
#[cfg(feature = "shared")]
pub type PBCRYPT_PKCS11_RSA_AES_WRAP_BLOB = *mut BCRYPT_PKCS11_RSA_AES_WRAP_BLOB;
#[cfg(feature = "shared")]
pub type PBCRYPT_PQDSA_KEY_BLOB = *mut BCRYPT_PQDSA_KEY_BLOB;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PBCryptBuffer = *mut BCryptBuffer;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PBCryptBufferDesc = *mut BCryptBufferDesc;
#[cfg(feature = "shared")]
pub type PCRYPT_CONTEXTS = *mut CRYPT_CONTEXTS;
#[cfg(feature = "shared")]
pub type PCRYPT_CONTEXT_CONFIG = *mut CRYPT_CONTEXT_CONFIG;
#[cfg(feature = "shared")]
pub type PCRYPT_CONTEXT_FUNCTIONS = *mut CRYPT_CONTEXT_FUNCTIONS;
#[cfg(feature = "shared")]
pub type PCRYPT_CONTEXT_FUNCTION_CONFIG = *mut CRYPT_CONTEXT_FUNCTION_CONFIG;
#[cfg(feature = "shared")]
pub type PCRYPT_CONTEXT_FUNCTION_PROVIDERS = *mut CRYPT_CONTEXT_FUNCTION_PROVIDERS;
#[cfg(feature = "shared")]
pub type PCRYPT_IMAGE_REF = *mut CRYPT_IMAGE_REF;
#[cfg(feature = "shared")]
pub type PCRYPT_IMAGE_REG = *mut CRYPT_IMAGE_REG;
#[cfg(feature = "shared")]
pub type PCRYPT_INTERFACE_REG = *mut CRYPT_INTERFACE_REG;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PCRYPT_PROPERTY_REF = *mut CRYPT_PROPERTY_REF;
#[cfg(feature = "shared")]
pub type PCRYPT_PROVIDERS = *mut CRYPT_PROVIDERS;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PCRYPT_PROVIDER_REF = *mut CRYPT_PROVIDER_REF;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PCRYPT_PROVIDER_REFS = *mut CRYPT_PROVIDER_REFS;
#[cfg(feature = "shared")]
pub type PCRYPT_PROVIDER_REG = *mut CRYPT_PROVIDER_REG;
#[cfg(feature = "shared")]
pub type PSSL_ECCKEY_BLOB = *mut SSL_ECCKEY_BLOB;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PTLS_HYBRID_KEYEXCHANGE_BLOB = *mut TLS_HYBRID_KEYEXCHANGE_BLOB;
#[cfg(feature = "shared")]
pub type PTLS_KEM_CIPHERTEXT_BLOB = *mut TLS_KEM_CIPHERTEXT_BLOB;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SSL_ECCKEY_BLOB {
    pub dwCurveType: super::ULONG,
    pub cbKey: super::ULONG,
}
pub const SSL_ECCPUBLIC_BLOB: windows_core::PCWSTR = windows_core::w!("SSLECCPUBLICBLOB");
pub const TLS_13_PRE_SHARED_KEY: windows_core::PCWSTR = windows_core::w!("TLS13PRESHAREDKEY");
pub const TLS_HYBRID_KEYEXCHANGE: windows_core::PCWSTR = windows_core::w!("TLS_HYBRID_KEYEXCHANGE");
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TLS_HYBRID_KEYEXCHANGE_BLOB {
    pub dwMagic: super::ULONG,
    pub dwKeyType: super::ULONG,
    pub NumBlobs: super::UCHAR,
    pub rgwszBlobTypes: [windows_core::PCWSTR; 4],
    pub rgbBlobs: [super::PUCHAR; 4],
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl Default for TLS_HYBRID_KEYEXCHANGE_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TLS_HYBRID_KEYEXCHANGE_MAGIC: i32 = 1213418580;
pub const TLS_KEM_CIPHERTEXT: windows_core::PCWSTR = windows_core::w!("TLS_KEM_CIPHER_TEXT");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TLS_KEM_CIPHERTEXT_BLOB {
    pub dwMagic: super::ULONG,
    pub cbLength: super::ULONG,
}
pub const TLS_KEM_CIPHERTEXT_MAGIC: i32 = 1129008205;
