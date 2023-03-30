#[doc = "*Required features: `\"Win32_System_Environment\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CallEnclave<P0>(lproutine: isize, lpparameter: *const ::core::ffi::c_void, fwaitforthread: P0, lpreturnvalue: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "vertdll.dll""system" fn CallEnclave ( lproutine : isize , lpparameter : *const ::core::ffi::c_void , fwaitforthread : super::super::Foundation:: BOOL , lpreturnvalue : *mut *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    CallEnclave(lproutine, lpparameter, fwaitforthread.into_param().abi(), lpreturnvalue)
}
#[doc = "*Required features: `\"Win32_System_Environment\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateEnclave<P0>(hprocess: P0, lpaddress: ::core::option::Option<*const ::core::ffi::c_void>, dwsize: usize, dwinitialcommitment: usize, flenclavetype: u32, lpenclaveinformation: *const ::core::ffi::c_void, dwinfolength: u32, lpenclaveerror: ::core::option::Option<*mut u32>) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateEnclave ( hprocess : super::super::Foundation:: HANDLE , lpaddress : *const ::core::ffi::c_void , dwsize : usize , dwinitialcommitment : usize , flenclavetype : u32 , lpenclaveinformation : *const ::core::ffi::c_void , dwinfolength : u32 , lpenclaveerror : *mut u32 ) -> *mut ::core::ffi::c_void );
    CreateEnclave(hprocess.into_param().abi(), ::core::mem::transmute(lpaddress.unwrap_or(::std::ptr::null())), dwsize, dwinitialcommitment, flenclavetype, lpenclaveinformation, dwinfolength, ::core::mem::transmute(lpenclaveerror.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Environment\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateEnvironmentBlock<P0, P1>(lpenvironment: *mut *mut ::core::ffi::c_void, htoken: P0, binherit: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "userenv.dll""system" fn CreateEnvironmentBlock ( lpenvironment : *mut *mut ::core::ffi::c_void , htoken : super::super::Foundation:: HANDLE , binherit : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    CreateEnvironmentBlock(lpenvironment, htoken.into_param().abi(), binherit.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Environment\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteEnclave(lpaddress: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "api-ms-win-core-enclave-l1-1-1.dll""system" fn DeleteEnclave ( lpaddress : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    DeleteEnclave(lpaddress)
}
#[doc = "*Required features: `\"Win32_System_Environment\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DestroyEnvironmentBlock(lpenvironment: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "userenv.dll""system" fn DestroyEnvironmentBlock ( lpenvironment : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    DestroyEnvironmentBlock(lpenvironment)
}
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
#[inline]
pub unsafe fn EnclaveGetAttestationReport(enclavedata: ::core::option::Option<*const u8>, report: ::core::option::Option<*mut ::core::ffi::c_void>, buffersize: u32, outputsize: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "vertdll.dll""system" fn EnclaveGetAttestationReport ( enclavedata : *const u8 , report : *mut ::core::ffi::c_void , buffersize : u32 , outputsize : *mut u32 ) -> ::windows::core::HRESULT );
    EnclaveGetAttestationReport(::core::mem::transmute(enclavedata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(report.unwrap_or(::std::ptr::null_mut())), buffersize, outputsize).ok()
}
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
#[inline]
pub unsafe fn EnclaveGetEnclaveInformation(informationsize: u32, enclaveinformation: *mut ENCLAVE_INFORMATION) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "vertdll.dll""system" fn EnclaveGetEnclaveInformation ( informationsize : u32 , enclaveinformation : *mut ENCLAVE_INFORMATION ) -> ::windows::core::HRESULT );
    EnclaveGetEnclaveInformation(informationsize, enclaveinformation).ok()
}
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
#[inline]
pub unsafe fn EnclaveSealData(datatoencrypt: *const ::core::ffi::c_void, datatoencryptsize: u32, identitypolicy: ENCLAVE_SEALING_IDENTITY_POLICY, runtimepolicy: u32, protectedblob: ::core::option::Option<*mut ::core::ffi::c_void>, buffersize: u32, protectedblobsize: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "vertdll.dll""system" fn EnclaveSealData ( datatoencrypt : *const ::core::ffi::c_void , datatoencryptsize : u32 , identitypolicy : ENCLAVE_SEALING_IDENTITY_POLICY , runtimepolicy : u32 , protectedblob : *mut ::core::ffi::c_void , buffersize : u32 , protectedblobsize : *mut u32 ) -> ::windows::core::HRESULT );
    EnclaveSealData(datatoencrypt, datatoencryptsize, identitypolicy, runtimepolicy, ::core::mem::transmute(protectedblob.unwrap_or(::std::ptr::null_mut())), buffersize, protectedblobsize).ok()
}
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
#[inline]
pub unsafe fn EnclaveUnsealData(protectedblob: *const ::core::ffi::c_void, protectedblobsize: u32, decrypteddata: ::core::option::Option<*mut ::core::ffi::c_void>, buffersize: u32, decrypteddatasize: *mut u32, sealingidentity: ::core::option::Option<*mut ENCLAVE_IDENTITY>, unsealingflags: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "vertdll.dll""system" fn EnclaveUnsealData ( protectedblob : *const ::core::ffi::c_void , protectedblobsize : u32 , decrypteddata : *mut ::core::ffi::c_void , buffersize : u32 , decrypteddatasize : *mut u32 , sealingidentity : *mut ENCLAVE_IDENTITY , unsealingflags : *mut u32 ) -> ::windows::core::HRESULT );
    EnclaveUnsealData(protectedblob, protectedblobsize, ::core::mem::transmute(decrypteddata.unwrap_or(::std::ptr::null_mut())), buffersize, decrypteddatasize, ::core::mem::transmute(sealingidentity.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(unsealingflags.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
#[inline]
pub unsafe fn EnclaveVerifyAttestationReport(enclavetype: u32, report: *const ::core::ffi::c_void, reportsize: u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "vertdll.dll""system" fn EnclaveVerifyAttestationReport ( enclavetype : u32 , report : *const ::core::ffi::c_void , reportsize : u32 ) -> ::windows::core::HRESULT );
    EnclaveVerifyAttestationReport(enclavetype, report, reportsize).ok()
}
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
#[inline]
pub unsafe fn ExpandEnvironmentStringsA<P0>(lpsrc: P0, lpdst: ::core::option::Option<&mut [u8]>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn ExpandEnvironmentStringsA ( lpsrc : ::windows::core::PCSTR , lpdst : ::windows::core::PSTR , nsize : u32 ) -> u32 );
    ExpandEnvironmentStringsA(lpsrc.into_param().abi(), ::core::mem::transmute(lpdst.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpdst.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_System_Environment\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExpandEnvironmentStringsForUserA<P0, P1>(htoken: P0, lpsrc: P1, lpdest: &mut [u8]) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "userenv.dll""system" fn ExpandEnvironmentStringsForUserA ( htoken : super::super::Foundation:: HANDLE , lpsrc : ::windows::core::PCSTR , lpdest : ::windows::core::PSTR , dwsize : u32 ) -> super::super::Foundation:: BOOL );
    ExpandEnvironmentStringsForUserA(htoken.into_param().abi(), lpsrc.into_param().abi(), ::core::mem::transmute(lpdest.as_ptr()), lpdest.len() as _)
}
#[doc = "*Required features: `\"Win32_System_Environment\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExpandEnvironmentStringsForUserW<P0, P1>(htoken: P0, lpsrc: P1, lpdest: &mut [u16]) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "userenv.dll""system" fn ExpandEnvironmentStringsForUserW ( htoken : super::super::Foundation:: HANDLE , lpsrc : ::windows::core::PCWSTR , lpdest : ::windows::core::PWSTR , dwsize : u32 ) -> super::super::Foundation:: BOOL );
    ExpandEnvironmentStringsForUserW(htoken.into_param().abi(), lpsrc.into_param().abi(), ::core::mem::transmute(lpdest.as_ptr()), lpdest.len() as _)
}
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
#[inline]
pub unsafe fn ExpandEnvironmentStringsW<P0>(lpsrc: P0, lpdst: ::core::option::Option<&mut [u16]>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn ExpandEnvironmentStringsW ( lpsrc : ::windows::core::PCWSTR , lpdst : ::windows::core::PWSTR , nsize : u32 ) -> u32 );
    ExpandEnvironmentStringsW(lpsrc.into_param().abi(), ::core::mem::transmute(lpdst.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpdst.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_System_Environment\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeEnvironmentStringsA<P0>(penv: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FreeEnvironmentStringsA ( penv : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    FreeEnvironmentStringsA(penv.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Environment\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeEnvironmentStringsW<P0>(penv: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FreeEnvironmentStringsW ( penv : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    FreeEnvironmentStringsW(penv.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
#[inline]
pub unsafe fn GetCommandLineA() -> ::windows::core::PCSTR {
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetCommandLineA ( ) -> ::windows::core::PCSTR );
    GetCommandLineA()
}
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
#[inline]
pub unsafe fn GetCommandLineW() -> ::windows::core::PCWSTR {
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetCommandLineW ( ) -> ::windows::core::PCWSTR );
    GetCommandLineW()
}
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
#[inline]
pub unsafe fn GetCurrentDirectoryA(lpbuffer: ::core::option::Option<&mut [u8]>) -> u32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetCurrentDirectoryA ( nbufferlength : u32 , lpbuffer : ::windows::core::PSTR ) -> u32 );
    GetCurrentDirectoryA(lpbuffer.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
#[inline]
pub unsafe fn GetCurrentDirectoryW(lpbuffer: ::core::option::Option<&mut [u16]>) -> u32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetCurrentDirectoryW ( nbufferlength : u32 , lpbuffer : ::windows::core::PWSTR ) -> u32 );
    GetCurrentDirectoryW(lpbuffer.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
#[inline]
pub unsafe fn GetEnvironmentStrings() -> ::windows::core::PSTR {
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetEnvironmentStrings ( ) -> ::windows::core::PSTR );
    GetEnvironmentStrings()
}
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
#[inline]
pub unsafe fn GetEnvironmentStringsW() -> ::windows::core::PWSTR {
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetEnvironmentStringsW ( ) -> ::windows::core::PWSTR );
    GetEnvironmentStringsW()
}
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
#[inline]
pub unsafe fn GetEnvironmentVariableA<P0>(lpname: P0, lpbuffer: ::core::option::Option<&mut [u8]>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetEnvironmentVariableA ( lpname : ::windows::core::PCSTR , lpbuffer : ::windows::core::PSTR , nsize : u32 ) -> u32 );
    GetEnvironmentVariableA(lpname.into_param().abi(), ::core::mem::transmute(lpbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
#[inline]
pub unsafe fn GetEnvironmentVariableW<P0>(lpname: P0, lpbuffer: ::core::option::Option<&mut [u16]>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetEnvironmentVariableW ( lpname : ::windows::core::PCWSTR , lpbuffer : ::windows::core::PWSTR , nsize : u32 ) -> u32 );
    GetEnvironmentVariableW(lpname.into_param().abi(), ::core::mem::transmute(lpbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_System_Environment\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitializeEnclave<P0>(hprocess: P0, lpaddress: *const ::core::ffi::c_void, lpenclaveinformation: *const ::core::ffi::c_void, dwinfolength: u32, lpenclaveerror: ::core::option::Option<*mut u32>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn InitializeEnclave ( hprocess : super::super::Foundation:: HANDLE , lpaddress : *const ::core::ffi::c_void , lpenclaveinformation : *const ::core::ffi::c_void , dwinfolength : u32 , lpenclaveerror : *mut u32 ) -> super::super::Foundation:: BOOL );
    InitializeEnclave(hprocess.into_param().abi(), lpaddress, lpenclaveinformation, dwinfolength, ::core::mem::transmute(lpenclaveerror.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Environment\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsEnclaveTypeSupported(flenclavetype: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn IsEnclaveTypeSupported ( flenclavetype : u32 ) -> super::super::Foundation:: BOOL );
    IsEnclaveTypeSupported(flenclavetype)
}
#[doc = "*Required features: `\"Win32_System_Environment\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadEnclaveData<P0>(hprocess: P0, lpaddress: *const ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, nsize: usize, flprotect: u32, lppageinformation: *const ::core::ffi::c_void, dwinfolength: u32, lpnumberofbyteswritten: *mut usize, lpenclaveerror: ::core::option::Option<*mut u32>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn LoadEnclaveData ( hprocess : super::super::Foundation:: HANDLE , lpaddress : *const ::core::ffi::c_void , lpbuffer : *const ::core::ffi::c_void , nsize : usize , flprotect : u32 , lppageinformation : *const ::core::ffi::c_void , dwinfolength : u32 , lpnumberofbyteswritten : *mut usize , lpenclaveerror : *mut u32 ) -> super::super::Foundation:: BOOL );
    LoadEnclaveData(hprocess.into_param().abi(), lpaddress, lpbuffer, nsize, flprotect, lppageinformation, dwinfolength, lpnumberofbyteswritten, ::core::mem::transmute(lpenclaveerror.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Environment\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadEnclaveImageA<P0>(lpenclaveaddress: *const ::core::ffi::c_void, lpimagename: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "api-ms-win-core-enclave-l1-1-1.dll""system" fn LoadEnclaveImageA ( lpenclaveaddress : *const ::core::ffi::c_void , lpimagename : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    LoadEnclaveImageA(lpenclaveaddress, lpimagename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Environment\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadEnclaveImageW<P0>(lpenclaveaddress: *const ::core::ffi::c_void, lpimagename: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "api-ms-win-core-enclave-l1-1-1.dll""system" fn LoadEnclaveImageW ( lpenclaveaddress : *const ::core::ffi::c_void , lpimagename : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    LoadEnclaveImageW(lpenclaveaddress, lpimagename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Environment\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NeedCurrentDirectoryForExePathA<P0>(exename: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn NeedCurrentDirectoryForExePathA ( exename : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    NeedCurrentDirectoryForExePathA(exename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Environment\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NeedCurrentDirectoryForExePathW<P0>(exename: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn NeedCurrentDirectoryForExePathW ( exename : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    NeedCurrentDirectoryForExePathW(exename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Environment\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetCurrentDirectoryA<P0>(lppathname: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetCurrentDirectoryA ( lppathname : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    SetCurrentDirectoryA(lppathname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Environment\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetCurrentDirectoryW<P0>(lppathname: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetCurrentDirectoryW ( lppathname : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    SetCurrentDirectoryW(lppathname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Environment\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetEnvironmentStringsW<P0>(newenvironment: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetEnvironmentStringsW ( newenvironment : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    SetEnvironmentStringsW(newenvironment.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Environment\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetEnvironmentVariableA<P0, P1>(lpname: P0, lpvalue: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetEnvironmentVariableA ( lpname : ::windows::core::PCSTR , lpvalue : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    SetEnvironmentVariableA(lpname.into_param().abi(), lpvalue.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Environment\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetEnvironmentVariableW<P0, P1>(lpname: P0, lpvalue: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetEnvironmentVariableW ( lpname : ::windows::core::PCWSTR , lpvalue : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    SetEnvironmentVariableW(lpname.into_param().abi(), lpvalue.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Environment\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TerminateEnclave<P0>(lpaddress: *const ::core::ffi::c_void, fwait: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "vertdll.dll""system" fn TerminateEnclave ( lpaddress : *const ::core::ffi::c_void , fwait : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    TerminateEnclave(lpaddress, fwait.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub const ENCLAVE_FLAG_DYNAMIC_DEBUG_ACTIVE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub const ENCLAVE_FLAG_DYNAMIC_DEBUG_ENABLED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub const ENCLAVE_FLAG_FULL_DEBUG_ENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub const ENCLAVE_REPORT_DATA_LENGTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub const ENCLAVE_RUNTIME_POLICY_ALLOW_DYNAMIC_DEBUG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub const ENCLAVE_RUNTIME_POLICY_ALLOW_FULL_DEBUG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub const ENCLAVE_UNSEAL_FLAG_STALE_KEY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub const ENCLAVE_VBS_BASIC_KEY_FLAG_DEBUG_KEY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub const ENCLAVE_VBS_BASIC_KEY_FLAG_FAMILY_ID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub const ENCLAVE_VBS_BASIC_KEY_FLAG_IMAGE_ID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub const ENCLAVE_VBS_BASIC_KEY_FLAG_MEASUREMENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub const VBS_ENCLAVE_REPORT_PKG_HEADER_VERSION_CURRENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub const VBS_ENCLAVE_REPORT_SIGNATURE_SCHEME_SHA256_RSA_PSS_SHA256: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub const VBS_ENCLAVE_REPORT_VERSION_CURRENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub const VBS_ENCLAVE_VARDATA_INVALID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub const VBS_ENCLAVE_VARDATA_MODULE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ENCLAVE_SEALING_IDENTITY_POLICY(pub i32);
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub const ENCLAVE_IDENTITY_POLICY_SEAL_INVALID: ENCLAVE_SEALING_IDENTITY_POLICY = ENCLAVE_SEALING_IDENTITY_POLICY(0i32);
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub const ENCLAVE_IDENTITY_POLICY_SEAL_EXACT_CODE: ENCLAVE_SEALING_IDENTITY_POLICY = ENCLAVE_SEALING_IDENTITY_POLICY(1i32);
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub const ENCLAVE_IDENTITY_POLICY_SEAL_SAME_PRIMARY_CODE: ENCLAVE_SEALING_IDENTITY_POLICY = ENCLAVE_SEALING_IDENTITY_POLICY(2i32);
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub const ENCLAVE_IDENTITY_POLICY_SEAL_SAME_IMAGE: ENCLAVE_SEALING_IDENTITY_POLICY = ENCLAVE_SEALING_IDENTITY_POLICY(3i32);
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub const ENCLAVE_IDENTITY_POLICY_SEAL_SAME_FAMILY: ENCLAVE_SEALING_IDENTITY_POLICY = ENCLAVE_SEALING_IDENTITY_POLICY(4i32);
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub const ENCLAVE_IDENTITY_POLICY_SEAL_SAME_AUTHOR: ENCLAVE_SEALING_IDENTITY_POLICY = ENCLAVE_SEALING_IDENTITY_POLICY(5i32);
impl ::core::marker::Copy for ENCLAVE_SEALING_IDENTITY_POLICY {}
impl ::core::clone::Clone for ENCLAVE_SEALING_IDENTITY_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ENCLAVE_SEALING_IDENTITY_POLICY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ENCLAVE_SEALING_IDENTITY_POLICY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ENCLAVE_SEALING_IDENTITY_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENCLAVE_SEALING_IDENTITY_POLICY").field(&self.0).finish()
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub struct ENCLAVE_IDENTITY {
    pub OwnerId: [u8; 32],
    pub UniqueId: [u8; 32],
    pub AuthorId: [u8; 32],
    pub FamilyId: [u8; 16],
    pub ImageId: [u8; 16],
    pub EnclaveSvn: u32,
    pub SecureKernelSvn: u32,
    pub PlatformSvn: u32,
    pub Flags: u32,
    pub SigningLevel: u32,
    pub EnclaveType: u32,
}
impl ::core::marker::Copy for ENCLAVE_IDENTITY {}
impl ::core::clone::Clone for ENCLAVE_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for ENCLAVE_IDENTITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for ENCLAVE_IDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub struct ENCLAVE_INFORMATION {
    pub EnclaveType: u32,
    pub Reserved: u32,
    pub BaseAddress: *mut ::core::ffi::c_void,
    pub Size: usize,
    pub Identity: ENCLAVE_IDENTITY,
}
impl ::core::marker::Copy for ENCLAVE_INFORMATION {}
impl ::core::clone::Clone for ENCLAVE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for ENCLAVE_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for ENCLAVE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub struct ENCLAVE_VBS_BASIC_KEY_REQUEST {
    pub RequestSize: u32,
    pub Flags: u32,
    pub EnclaveSVN: u32,
    pub SystemKeyID: u32,
    pub CurrentSystemKeyID: u32,
}
impl ::core::marker::Copy for ENCLAVE_VBS_BASIC_KEY_REQUEST {}
impl ::core::clone::Clone for ENCLAVE_VBS_BASIC_KEY_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ENCLAVE_VBS_BASIC_KEY_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCLAVE_VBS_BASIC_KEY_REQUEST").field("RequestSize", &self.RequestSize).field("Flags", &self.Flags).field("EnclaveSVN", &self.EnclaveSVN).field("SystemKeyID", &self.SystemKeyID).field("CurrentSystemKeyID", &self.CurrentSystemKeyID).finish()
    }
}
impl ::windows::core::TypeKind for ENCLAVE_VBS_BASIC_KEY_REQUEST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ENCLAVE_VBS_BASIC_KEY_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.RequestSize == other.RequestSize && self.Flags == other.Flags && self.EnclaveSVN == other.EnclaveSVN && self.SystemKeyID == other.SystemKeyID && self.CurrentSystemKeyID == other.CurrentSystemKeyID
    }
}
impl ::core::cmp::Eq for ENCLAVE_VBS_BASIC_KEY_REQUEST {}
impl ::core::default::Default for ENCLAVE_VBS_BASIC_KEY_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub struct VBS_BASIC_ENCLAVE_EXCEPTION_AMD64 {
    pub ExceptionCode: u32,
    pub NumberParameters: u32,
    pub ExceptionInformation: [usize; 3],
    pub ExceptionRAX: usize,
    pub ExceptionRCX: usize,
    pub ExceptionRIP: usize,
    pub ExceptionRFLAGS: usize,
    pub ExceptionRSP: usize,
}
impl ::core::marker::Copy for VBS_BASIC_ENCLAVE_EXCEPTION_AMD64 {}
impl ::core::clone::Clone for VBS_BASIC_ENCLAVE_EXCEPTION_AMD64 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VBS_BASIC_ENCLAVE_EXCEPTION_AMD64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBS_BASIC_ENCLAVE_EXCEPTION_AMD64").field("ExceptionCode", &self.ExceptionCode).field("NumberParameters", &self.NumberParameters).field("ExceptionInformation", &self.ExceptionInformation).field("ExceptionRAX", &self.ExceptionRAX).field("ExceptionRCX", &self.ExceptionRCX).field("ExceptionRIP", &self.ExceptionRIP).field("ExceptionRFLAGS", &self.ExceptionRFLAGS).field("ExceptionRSP", &self.ExceptionRSP).finish()
    }
}
impl ::windows::core::TypeKind for VBS_BASIC_ENCLAVE_EXCEPTION_AMD64 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VBS_BASIC_ENCLAVE_EXCEPTION_AMD64 {
    fn eq(&self, other: &Self) -> bool {
        self.ExceptionCode == other.ExceptionCode && self.NumberParameters == other.NumberParameters && self.ExceptionInformation == other.ExceptionInformation && self.ExceptionRAX == other.ExceptionRAX && self.ExceptionRCX == other.ExceptionRCX && self.ExceptionRIP == other.ExceptionRIP && self.ExceptionRFLAGS == other.ExceptionRFLAGS && self.ExceptionRSP == other.ExceptionRSP
    }
}
impl ::core::cmp::Eq for VBS_BASIC_ENCLAVE_EXCEPTION_AMD64 {}
impl ::core::default::Default for VBS_BASIC_ENCLAVE_EXCEPTION_AMD64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub struct VBS_BASIC_ENCLAVE_SYSCALL_PAGE {
    pub ReturnFromEnclave: VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_ENCLAVE,
    pub ReturnFromException: VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_EXCEPTION,
    pub TerminateThread: VBS_BASIC_ENCLAVE_BASIC_CALL_TERMINATE_THREAD,
    pub InterruptThread: VBS_BASIC_ENCLAVE_BASIC_CALL_INTERRUPT_THREAD,
    pub CommitPages: VBS_BASIC_ENCLAVE_BASIC_CALL_COMMIT_PAGES,
    pub DecommitPages: VBS_BASIC_ENCLAVE_BASIC_CALL_DECOMMIT_PAGES,
    pub ProtectPages: VBS_BASIC_ENCLAVE_BASIC_CALL_PROTECT_PAGES,
    pub CreateThread: VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD,
    pub GetEnclaveInformation: VBS_BASIC_ENCLAVE_BASIC_CALL_GET_ENCLAVE_INFORMATION,
    pub GenerateKey: VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_KEY,
    pub GenerateReport: VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_REPORT,
    pub VerifyReport: VBS_BASIC_ENCLAVE_BASIC_CALL_VERIFY_REPORT,
    pub GenerateRandomData: VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_RANDOM_DATA,
}
impl ::core::marker::Copy for VBS_BASIC_ENCLAVE_SYSCALL_PAGE {}
impl ::core::clone::Clone for VBS_BASIC_ENCLAVE_SYSCALL_PAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VBS_BASIC_ENCLAVE_SYSCALL_PAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBS_BASIC_ENCLAVE_SYSCALL_PAGE").finish()
    }
}
impl ::windows::core::TypeKind for VBS_BASIC_ENCLAVE_SYSCALL_PAGE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for VBS_BASIC_ENCLAVE_SYSCALL_PAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub struct VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {
    pub ThreadContext: [u32; 4],
    pub EntryPoint: u32,
    pub StackPointer: u32,
    pub ExceptionEntryPoint: u32,
    pub ExceptionStack: u32,
    pub ExceptionActive: u32,
}
impl ::core::marker::Copy for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {}
impl ::core::clone::Clone for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32").field("ThreadContext", &self.ThreadContext).field("EntryPoint", &self.EntryPoint).field("StackPointer", &self.StackPointer).field("ExceptionEntryPoint", &self.ExceptionEntryPoint).field("ExceptionStack", &self.ExceptionStack).field("ExceptionActive", &self.ExceptionActive).finish()
    }
}
impl ::windows::core::TypeKind for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {
    fn eq(&self, other: &Self) -> bool {
        self.ThreadContext == other.ThreadContext && self.EntryPoint == other.EntryPoint && self.StackPointer == other.StackPointer && self.ExceptionEntryPoint == other.ExceptionEntryPoint && self.ExceptionStack == other.ExceptionStack && self.ExceptionActive == other.ExceptionActive
    }
}
impl ::core::cmp::Eq for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {}
impl ::core::default::Default for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub struct VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {
    pub ThreadContext: [u64; 4],
    pub EntryPoint: u64,
    pub StackPointer: u64,
    pub ExceptionEntryPoint: u64,
    pub ExceptionStack: u64,
    pub ExceptionActive: u32,
}
impl ::core::marker::Copy for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {}
impl ::core::clone::Clone for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64").field("ThreadContext", &self.ThreadContext).field("EntryPoint", &self.EntryPoint).field("StackPointer", &self.StackPointer).field("ExceptionEntryPoint", &self.ExceptionEntryPoint).field("ExceptionStack", &self.ExceptionStack).field("ExceptionActive", &self.ExceptionActive).finish()
    }
}
impl ::windows::core::TypeKind for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {
    fn eq(&self, other: &Self) -> bool {
        self.ThreadContext == other.ThreadContext && self.EntryPoint == other.EntryPoint && self.StackPointer == other.StackPointer && self.ExceptionEntryPoint == other.ExceptionEntryPoint && self.ExceptionStack == other.ExceptionStack && self.ExceptionActive == other.ExceptionActive
    }
}
impl ::core::cmp::Eq for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {}
impl ::core::default::Default for VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub struct VBS_ENCLAVE_REPORT {
    pub ReportSize: u32,
    pub ReportVersion: u32,
    pub EnclaveData: [u8; 64],
    pub EnclaveIdentity: ENCLAVE_IDENTITY,
}
impl ::core::marker::Copy for VBS_ENCLAVE_REPORT {}
impl ::core::clone::Clone for VBS_ENCLAVE_REPORT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for VBS_ENCLAVE_REPORT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for VBS_ENCLAVE_REPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub struct VBS_ENCLAVE_REPORT_MODULE {
    pub Header: VBS_ENCLAVE_REPORT_VARDATA_HEADER,
    pub UniqueId: [u8; 32],
    pub AuthorId: [u8; 32],
    pub FamilyId: [u8; 16],
    pub ImageId: [u8; 16],
    pub Svn: u32,
    pub ModuleName: [u16; 1],
}
impl ::core::marker::Copy for VBS_ENCLAVE_REPORT_MODULE {}
impl ::core::clone::Clone for VBS_ENCLAVE_REPORT_MODULE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for VBS_ENCLAVE_REPORT_MODULE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for VBS_ENCLAVE_REPORT_MODULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub struct VBS_ENCLAVE_REPORT_PKG_HEADER {
    pub PackageSize: u32,
    pub Version: u32,
    pub SignatureScheme: u32,
    pub SignedStatementSize: u32,
    pub SignatureSize: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for VBS_ENCLAVE_REPORT_PKG_HEADER {}
impl ::core::clone::Clone for VBS_ENCLAVE_REPORT_PKG_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for VBS_ENCLAVE_REPORT_PKG_HEADER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for VBS_ENCLAVE_REPORT_PKG_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub struct VBS_ENCLAVE_REPORT_VARDATA_HEADER {
    pub DataType: u32,
    pub Size: u32,
}
impl ::core::marker::Copy for VBS_ENCLAVE_REPORT_VARDATA_HEADER {}
impl ::core::clone::Clone for VBS_ENCLAVE_REPORT_VARDATA_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for VBS_ENCLAVE_REPORT_VARDATA_HEADER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for VBS_ENCLAVE_REPORT_VARDATA_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_COMMIT_PAGES = ::core::option::Option<unsafe extern "system" fn(enclaveaddress: *const ::core::ffi::c_void, numberofbytes: usize, sourceaddress: *const ::core::ffi::c_void, pageprotection: u32) -> i32>;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD = ::core::option::Option<unsafe extern "system" fn(threaddescriptor: *const VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64) -> i32>;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
#[cfg(target_arch = "x86")]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD = ::core::option::Option<unsafe extern "system" fn(threaddescriptor: *const VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32) -> i32>;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_DECOMMIT_PAGES = ::core::option::Option<unsafe extern "system" fn(enclaveaddress: *const ::core::ffi::c_void, numberofbytes: usize) -> i32>;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_KEY = ::core::option::Option<unsafe extern "system" fn(keyrequest: *mut ENCLAVE_VBS_BASIC_KEY_REQUEST, requestedkeysize: u32, returnedkey: *mut u8) -> i32>;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_RANDOM_DATA = ::core::option::Option<unsafe extern "system" fn(buffer: *mut u8, numberofbytes: u32, generation: *mut u64) -> i32>;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_GENERATE_REPORT = ::core::option::Option<unsafe extern "system" fn(enclavedata: *const u8, report: *mut ::core::ffi::c_void, buffersize: u32, outputsize: *mut u32) -> i32>;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_GET_ENCLAVE_INFORMATION = ::core::option::Option<unsafe extern "system" fn(enclaveinfo: *mut ENCLAVE_INFORMATION) -> i32>;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_INTERRUPT_THREAD = ::core::option::Option<unsafe extern "system" fn(threaddescriptor: *const VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64) -> i32>;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
#[cfg(target_arch = "x86")]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_INTERRUPT_THREAD = ::core::option::Option<unsafe extern "system" fn(threaddescriptor: *const VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32) -> i32>;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_PROTECT_PAGES = ::core::option::Option<unsafe extern "system" fn(enclaveaddress: *const ::core::ffi::c_void, numberofytes: usize, pageprotection: u32) -> i32>;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_ENCLAVE = ::core::option::Option<unsafe extern "system" fn(returnvalue: usize) -> ()>;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
#[cfg(target_arch = "x86_64")]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_EXCEPTION = ::core::option::Option<unsafe extern "system" fn(exceptionrecord: *const VBS_BASIC_ENCLAVE_EXCEPTION_AMD64) -> i32>;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_RETURN_FROM_EXCEPTION = ::core::option::Option<unsafe extern "system" fn(exceptionrecord: *const ::core::ffi::c_void) -> i32>;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_TERMINATE_THREAD = ::core::option::Option<unsafe extern "system" fn(threaddescriptor: *const VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR64) -> i32>;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
#[cfg(target_arch = "x86")]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_TERMINATE_THREAD = ::core::option::Option<unsafe extern "system" fn(threaddescriptor: *const VBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR32) -> i32>;
#[doc = "*Required features: `\"Win32_System_Environment\"`*"]
pub type VBS_BASIC_ENCLAVE_BASIC_CALL_VERIFY_REPORT = ::core::option::Option<unsafe extern "system" fn(report: *const ::core::ffi::c_void, reportsize: u32) -> i32>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
