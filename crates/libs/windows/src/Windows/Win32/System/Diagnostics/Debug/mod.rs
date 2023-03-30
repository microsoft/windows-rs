#[cfg(feature = "Win32_System_Diagnostics_Debug_ActiveScript")]
pub mod ActiveScript;
#[cfg(feature = "Win32_System_Diagnostics_Debug_Extensions")]
pub mod Extensions;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn AddVectoredContinueHandler(first: u32, handler: PVECTORED_EXCEPTION_HANDLER) -> *mut ::core::ffi::c_void {
    ::windows_targets::link ! ( "kernel32.dll""system" fn AddVectoredContinueHandler ( first : u32 , handler : PVECTORED_EXCEPTION_HANDLER ) -> *mut ::core::ffi::c_void );
    AddVectoredContinueHandler(first, handler)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn AddVectoredExceptionHandler(first: u32, handler: PVECTORED_EXCEPTION_HANDLER) -> *mut ::core::ffi::c_void {
    ::windows_targets::link ! ( "kernel32.dll""system" fn AddVectoredExceptionHandler ( first : u32 , handler : PVECTORED_EXCEPTION_HANDLER ) -> *mut ::core::ffi::c_void );
    AddVectoredExceptionHandler(first, handler)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Beep(dwfreq: u32, dwduration: u32) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn Beep ( dwfreq : u32 , dwduration : u32 ) -> super::super::super::Foundation:: BOOL );
    Beep(dwfreq, dwduration)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BindImage<P0, P1, P2>(imagename: P0, dllpath: P1, symbolpath: P2) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "imagehlp.dll""system" fn BindImage ( imagename : ::windows::core::PCSTR , dllpath : ::windows::core::PCSTR , symbolpath : ::windows::core::PCSTR ) -> super::super::super::Foundation:: BOOL );
    BindImage(imagename.into_param().abi(), dllpath.into_param().abi(), symbolpath.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BindImageEx<P0, P1, P2>(flags: u32, imagename: P0, dllpath: P1, symbolpath: P2, statusroutine: PIMAGEHLP_STATUS_ROUTINE) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "imagehlp.dll""system" fn BindImageEx ( flags : u32 , imagename : ::windows::core::PCSTR , dllpath : ::windows::core::PCSTR , symbolpath : ::windows::core::PCSTR , statusroutine : PIMAGEHLP_STATUS_ROUTINE ) -> super::super::super::Foundation:: BOOL );
    BindImageEx(flags, imagename.into_param().abi(), dllpath.into_param().abi(), symbolpath.into_param().abi(), statusroutine)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckRemoteDebuggerPresent<P0>(hprocess: P0, pbdebuggerpresent: *mut super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CheckRemoteDebuggerPresent ( hprocess : super::super::super::Foundation:: HANDLE , pbdebuggerpresent : *mut super::super::super::Foundation:: BOOL ) -> super::super::super::Foundation:: BOOL );
    CheckRemoteDebuggerPresent(hprocess.into_param().abi(), pbdebuggerpresent)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_SystemInformation")]
#[inline]
pub unsafe fn CheckSumMappedFile(baseaddress: *const ::core::ffi::c_void, filelength: u32, headersum: *mut u32, checksum: *mut u32) -> *mut IMAGE_NT_HEADERS64 {
    ::windows_targets::link ! ( "imagehlp.dll""system" fn CheckSumMappedFile ( baseaddress : *const ::core::ffi::c_void , filelength : u32 , headersum : *mut u32 , checksum : *mut u32 ) -> *mut IMAGE_NT_HEADERS64 );
    CheckSumMappedFile(baseaddress, filelength, headersum, checksum)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_SystemInformation")]
#[inline]
pub unsafe fn CheckSumMappedFile(baseaddress: *const ::core::ffi::c_void, filelength: u32, headersum: *mut u32, checksum: *mut u32) -> *mut IMAGE_NT_HEADERS32 {
    ::windows_targets::link ! ( "imagehlp.dll""system" fn CheckSumMappedFile ( baseaddress : *const ::core::ffi::c_void , filelength : u32 , headersum : *mut u32 , checksum : *mut u32 ) -> *mut IMAGE_NT_HEADERS32 );
    CheckSumMappedFile(baseaddress, filelength, headersum, checksum)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn CloseThreadWaitChainSession(wcthandle: *const ::core::ffi::c_void) {
    ::windows_targets::link ! ( "advapi32.dll""system" fn CloseThreadWaitChainSession ( wcthandle : *const ::core::ffi::c_void ) -> ( ) );
    CloseThreadWaitChainSession(wcthandle)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ContinueDebugEvent<P0>(dwprocessid: u32, dwthreadid: u32, dwcontinuestatus: P0) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::NTSTATUS>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn ContinueDebugEvent ( dwprocessid : u32 , dwthreadid : u32 , dwcontinuestatus : super::super::super::Foundation:: NTSTATUS ) -> super::super::super::Foundation:: BOOL );
    ContinueDebugEvent(dwprocessid, dwthreadid, dwcontinuestatus.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn CopyContext(destination: *mut CONTEXT, contextflags: u32, source: *const CONTEXT) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn CopyContext ( destination : *mut CONTEXT , contextflags : u32 , source : *const CONTEXT ) -> super::super::super::Foundation:: BOOL );
    CopyContext(destination, contextflags, source)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DbgHelpCreateUserDump<P0>(filename: P0, callback: PDBGHELP_CREATE_USER_DUMP_CALLBACK, userdata: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn DbgHelpCreateUserDump ( filename : ::windows::core::PCSTR , callback : PDBGHELP_CREATE_USER_DUMP_CALLBACK , userdata : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    DbgHelpCreateUserDump(filename.into_param().abi(), callback, ::core::mem::transmute(userdata.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DbgHelpCreateUserDumpW<P0>(filename: P0, callback: PDBGHELP_CREATE_USER_DUMP_CALLBACK, userdata: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn DbgHelpCreateUserDumpW ( filename : ::windows::core::PCWSTR , callback : PDBGHELP_CREATE_USER_DUMP_CALLBACK , userdata : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    DbgHelpCreateUserDumpW(filename.into_param().abi(), callback, ::core::mem::transmute(userdata.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DebugActiveProcess(dwprocessid: u32) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn DebugActiveProcess ( dwprocessid : u32 ) -> super::super::super::Foundation:: BOOL );
    DebugActiveProcess(dwprocessid)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DebugActiveProcessStop(dwprocessid: u32) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn DebugActiveProcessStop ( dwprocessid : u32 ) -> super::super::super::Foundation:: BOOL );
    DebugActiveProcessStop(dwprocessid)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn DebugBreak() {
    ::windows_targets::link ! ( "kernel32.dll""system" fn DebugBreak ( ) -> ( ) );
    DebugBreak()
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DebugBreakProcess<P0>(process: P0) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn DebugBreakProcess ( process : super::super::super::Foundation:: HANDLE ) -> super::super::super::Foundation:: BOOL );
    DebugBreakProcess(process.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DebugSetProcessKillOnExit<P0>(killonexit: P0) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn DebugSetProcessKillOnExit ( killonexit : super::super::super::Foundation:: BOOL ) -> super::super::super::Foundation:: BOOL );
    DebugSetProcessKillOnExit(killonexit.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn DecodePointer(ptr: ::core::option::Option<*const ::core::ffi::c_void>) -> *mut ::core::ffi::c_void {
    ::windows_targets::link ! ( "kernel32.dll""system" fn DecodePointer ( ptr : *const ::core::ffi::c_void ) -> *mut ::core::ffi::c_void );
    DecodePointer(::core::mem::transmute(ptr.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DecodeRemotePointer<P0>(processhandle: P0, ptr: ::core::option::Option<*const ::core::ffi::c_void>, decodedptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "api-ms-win-core-util-l1-1-1.dll""system" fn DecodeRemotePointer ( processhandle : super::super::super::Foundation:: HANDLE , ptr : *const ::core::ffi::c_void , decodedptr : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    DecodeRemotePointer(processhandle.into_param().abi(), ::core::mem::transmute(ptr.unwrap_or(::std::ptr::null())), decodedptr).ok()
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn DecodeSystemPointer(ptr: ::core::option::Option<*const ::core::ffi::c_void>) -> *mut ::core::ffi::c_void {
    ::windows_targets::link ! ( "kernel32.dll""system" fn DecodeSystemPointer ( ptr : *const ::core::ffi::c_void ) -> *mut ::core::ffi::c_void );
    DecodeSystemPointer(::core::mem::transmute(ptr.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn EncodePointer(ptr: ::core::option::Option<*const ::core::ffi::c_void>) -> *mut ::core::ffi::c_void {
    ::windows_targets::link ! ( "kernel32.dll""system" fn EncodePointer ( ptr : *const ::core::ffi::c_void ) -> *mut ::core::ffi::c_void );
    EncodePointer(::core::mem::transmute(ptr.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EncodeRemotePointer<P0>(processhandle: P0, ptr: ::core::option::Option<*const ::core::ffi::c_void>, encodedptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "api-ms-win-core-util-l1-1-1.dll""system" fn EncodeRemotePointer ( processhandle : super::super::super::Foundation:: HANDLE , ptr : *const ::core::ffi::c_void , encodedptr : *mut *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    EncodeRemotePointer(processhandle.into_param().abi(), ::core::mem::transmute(ptr.unwrap_or(::std::ptr::null())), encodedptr).ok()
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn EncodeSystemPointer(ptr: ::core::option::Option<*const ::core::ffi::c_void>) -> *mut ::core::ffi::c_void {
    ::windows_targets::link ! ( "kernel32.dll""system" fn EncodeSystemPointer ( ptr : *const ::core::ffi::c_void ) -> *mut ::core::ffi::c_void );
    EncodeSystemPointer(::core::mem::transmute(ptr.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumDirTree<P0, P1, P2>(hprocess: P0, rootpath: P1, inputpathname: P2, outputpathbuffer: ::windows::core::PSTR, cb: PENUMDIRTREE_CALLBACK, data: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn EnumDirTree ( hprocess : super::super::super::Foundation:: HANDLE , rootpath : ::windows::core::PCSTR , inputpathname : ::windows::core::PCSTR , outputpathbuffer : ::windows::core::PSTR , cb : PENUMDIRTREE_CALLBACK , data : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    EnumDirTree(hprocess.into_param().abi(), rootpath.into_param().abi(), inputpathname.into_param().abi(), ::core::mem::transmute(outputpathbuffer), cb, ::core::mem::transmute(data.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumDirTreeW<P0, P1, P2>(hprocess: P0, rootpath: P1, inputpathname: P2, outputpathbuffer: ::windows::core::PWSTR, cb: PENUMDIRTREE_CALLBACKW, data: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn EnumDirTreeW ( hprocess : super::super::super::Foundation:: HANDLE , rootpath : ::windows::core::PCWSTR , inputpathname : ::windows::core::PCWSTR , outputpathbuffer : ::windows::core::PWSTR , cb : PENUMDIRTREE_CALLBACKW , data : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    EnumDirTreeW(hprocess.into_param().abi(), rootpath.into_param().abi(), inputpathname.into_param().abi(), ::core::mem::transmute(outputpathbuffer), cb, ::core::mem::transmute(data.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumerateLoadedModules<P0>(hprocess: P0, enumloadedmodulescallback: PENUMLOADED_MODULES_CALLBACK, usercontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn EnumerateLoadedModules ( hprocess : super::super::super::Foundation:: HANDLE , enumloadedmodulescallback : PENUMLOADED_MODULES_CALLBACK , usercontext : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    EnumerateLoadedModules(hprocess.into_param().abi(), enumloadedmodulescallback, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumerateLoadedModules64<P0>(hprocess: P0, enumloadedmodulescallback: PENUMLOADED_MODULES_CALLBACK64, usercontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn EnumerateLoadedModules64 ( hprocess : super::super::super::Foundation:: HANDLE , enumloadedmodulescallback : PENUMLOADED_MODULES_CALLBACK64 , usercontext : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    EnumerateLoadedModules64(hprocess.into_param().abi(), enumloadedmodulescallback, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumerateLoadedModulesEx<P0>(hprocess: P0, enumloadedmodulescallback: PENUMLOADED_MODULES_CALLBACK64, usercontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn EnumerateLoadedModulesEx ( hprocess : super::super::super::Foundation:: HANDLE , enumloadedmodulescallback : PENUMLOADED_MODULES_CALLBACK64 , usercontext : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    EnumerateLoadedModulesEx(hprocess.into_param().abi(), enumloadedmodulescallback, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumerateLoadedModulesExW<P0>(hprocess: P0, enumloadedmodulescallback: PENUMLOADED_MODULES_CALLBACKW64, usercontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn EnumerateLoadedModulesExW ( hprocess : super::super::super::Foundation:: HANDLE , enumloadedmodulescallback : PENUMLOADED_MODULES_CALLBACKW64 , usercontext : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    EnumerateLoadedModulesExW(hprocess.into_param().abi(), enumloadedmodulescallback, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumerateLoadedModulesW64<P0>(hprocess: P0, enumloadedmodulescallback: PENUMLOADED_MODULES_CALLBACKW64, usercontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn EnumerateLoadedModulesW64 ( hprocess : super::super::super::Foundation:: HANDLE , enumloadedmodulescallback : PENUMLOADED_MODULES_CALLBACKW64 , usercontext : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    EnumerateLoadedModulesW64(hprocess.into_param().abi(), enumloadedmodulescallback, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn FatalAppExitA<P0>(uaction: u32, lpmessagetext: P0)
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FatalAppExitA ( uaction : u32 , lpmessagetext : ::windows::core::PCSTR ) -> ( ) );
    FatalAppExitA(uaction, lpmessagetext.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn FatalAppExitW<P0>(uaction: u32, lpmessagetext: P0)
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FatalAppExitW ( uaction : u32 , lpmessagetext : ::windows::core::PCWSTR ) -> ( ) );
    FatalAppExitW(uaction, lpmessagetext.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn FatalExit(exitcode: i32) -> ! {
    ::windows_targets::link ! ( "kernel32.dll""system" fn FatalExit ( exitcode : i32 ) -> ! );
    FatalExit(exitcode)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindDebugInfoFile<P0, P1>(filename: P0, symbolpath: P1, debugfilepath: ::windows::core::PSTR) -> ::windows::core::Result<super::super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn FindDebugInfoFile ( filename : ::windows::core::PCSTR , symbolpath : ::windows::core::PCSTR , debugfilepath : ::windows::core::PSTR ) -> super::super::super::Foundation:: HANDLE );
    let result__ = FindDebugInfoFile(filename.into_param().abi(), symbolpath.into_param().abi(), ::core::mem::transmute(debugfilepath));
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindDebugInfoFileEx<P0, P1>(filename: P0, symbolpath: P1, debugfilepath: ::windows::core::PSTR, callback: PFIND_DEBUG_FILE_CALLBACK, callerdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<super::super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn FindDebugInfoFileEx ( filename : ::windows::core::PCSTR , symbolpath : ::windows::core::PCSTR , debugfilepath : ::windows::core::PSTR , callback : PFIND_DEBUG_FILE_CALLBACK , callerdata : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: HANDLE );
    let result__ = FindDebugInfoFileEx(filename.into_param().abi(), symbolpath.into_param().abi(), ::core::mem::transmute(debugfilepath), callback, ::core::mem::transmute(callerdata.unwrap_or(::std::ptr::null())));
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindDebugInfoFileExW<P0, P1>(filename: P0, symbolpath: P1, debugfilepath: ::windows::core::PWSTR, callback: PFIND_DEBUG_FILE_CALLBACKW, callerdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<super::super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn FindDebugInfoFileExW ( filename : ::windows::core::PCWSTR , symbolpath : ::windows::core::PCWSTR , debugfilepath : ::windows::core::PWSTR , callback : PFIND_DEBUG_FILE_CALLBACKW , callerdata : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: HANDLE );
    let result__ = FindDebugInfoFileExW(filename.into_param().abi(), symbolpath.into_param().abi(), ::core::mem::transmute(debugfilepath), callback, ::core::mem::transmute(callerdata.unwrap_or(::std::ptr::null())));
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindExecutableImage<P0, P1>(filename: P0, symbolpath: P1, imagefilepath: ::windows::core::PSTR) -> ::windows::core::Result<super::super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn FindExecutableImage ( filename : ::windows::core::PCSTR , symbolpath : ::windows::core::PCSTR , imagefilepath : ::windows::core::PSTR ) -> super::super::super::Foundation:: HANDLE );
    let result__ = FindExecutableImage(filename.into_param().abi(), symbolpath.into_param().abi(), ::core::mem::transmute(imagefilepath));
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindExecutableImageEx<P0, P1>(filename: P0, symbolpath: P1, imagefilepath: ::windows::core::PSTR, callback: PFIND_EXE_FILE_CALLBACK, callerdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<super::super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn FindExecutableImageEx ( filename : ::windows::core::PCSTR , symbolpath : ::windows::core::PCSTR , imagefilepath : ::windows::core::PSTR , callback : PFIND_EXE_FILE_CALLBACK , callerdata : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: HANDLE );
    let result__ = FindExecutableImageEx(filename.into_param().abi(), symbolpath.into_param().abi(), ::core::mem::transmute(imagefilepath), callback, ::core::mem::transmute(callerdata.unwrap_or(::std::ptr::null())));
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindExecutableImageExW<P0, P1>(filename: P0, symbolpath: P1, imagefilepath: ::windows::core::PWSTR, callback: PFIND_EXE_FILE_CALLBACKW, callerdata: *const ::core::ffi::c_void) -> ::windows::core::Result<super::super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn FindExecutableImageExW ( filename : ::windows::core::PCWSTR , symbolpath : ::windows::core::PCWSTR , imagefilepath : ::windows::core::PWSTR , callback : PFIND_EXE_FILE_CALLBACKW , callerdata : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: HANDLE );
    let result__ = FindExecutableImageExW(filename.into_param().abi(), symbolpath.into_param().abi(), ::core::mem::transmute(imagefilepath), callback, callerdata);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFileInPath<P0, P1, P2>(hprocess: P0, searchpatha: P1, filename: P2, id: *const ::core::ffi::c_void, two: u32, three: u32, flags: u32, filepath: ::windows::core::PSTR) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn FindFileInPath ( hprocess : super::super::super::Foundation:: HANDLE , searchpatha : ::windows::core::PCSTR , filename : ::windows::core::PCSTR , id : *const ::core::ffi::c_void , two : u32 , three : u32 , flags : u32 , filepath : ::windows::core::PSTR ) -> super::super::super::Foundation:: BOOL );
    FindFileInPath(hprocess.into_param().abi(), searchpatha.into_param().abi(), filename.into_param().abi(), id, two, three, flags, ::core::mem::transmute(filepath))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFileInSearchPath<P0, P1, P2>(hprocess: P0, searchpatha: P1, filename: P2, one: u32, two: u32, three: u32, filepath: ::windows::core::PSTR) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn FindFileInSearchPath ( hprocess : super::super::super::Foundation:: HANDLE , searchpatha : ::windows::core::PCSTR , filename : ::windows::core::PCSTR , one : u32 , two : u32 , three : u32 , filepath : ::windows::core::PSTR ) -> super::super::super::Foundation:: BOOL );
    FindFileInSearchPath(hprocess.into_param().abi(), searchpatha.into_param().abi(), filename.into_param().abi(), one, two, three, ::core::mem::transmute(filepath))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FlushInstructionCache<P0>(hprocess: P0, lpbaseaddress: ::core::option::Option<*const ::core::ffi::c_void>, dwsize: usize) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FlushInstructionCache ( hprocess : super::super::super::Foundation:: HANDLE , lpbaseaddress : *const ::core::ffi::c_void , dwsize : usize ) -> super::super::super::Foundation:: BOOL );
    FlushInstructionCache(hprocess.into_param().abi(), ::core::mem::transmute(lpbaseaddress.unwrap_or(::std::ptr::null())), dwsize)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn FormatMessageA(dwflags: FORMAT_MESSAGE_OPTIONS, lpsource: ::core::option::Option<*const ::core::ffi::c_void>, dwmessageid: u32, dwlanguageid: u32, lpbuffer: ::windows::core::PSTR, nsize: u32, arguments: ::core::option::Option<*const *const i8>) -> u32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn FormatMessageA ( dwflags : FORMAT_MESSAGE_OPTIONS , lpsource : *const ::core::ffi::c_void , dwmessageid : u32 , dwlanguageid : u32 , lpbuffer : ::windows::core::PSTR , nsize : u32 , arguments : *const *const i8 ) -> u32 );
    FormatMessageA(dwflags, ::core::mem::transmute(lpsource.unwrap_or(::std::ptr::null())), dwmessageid, dwlanguageid, ::core::mem::transmute(lpbuffer), nsize, ::core::mem::transmute(arguments.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn FormatMessageW(dwflags: FORMAT_MESSAGE_OPTIONS, lpsource: ::core::option::Option<*const ::core::ffi::c_void>, dwmessageid: u32, dwlanguageid: u32, lpbuffer: ::windows::core::PWSTR, nsize: u32, arguments: ::core::option::Option<*const *const i8>) -> u32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn FormatMessageW ( dwflags : FORMAT_MESSAGE_OPTIONS , lpsource : *const ::core::ffi::c_void , dwmessageid : u32 , dwlanguageid : u32 , lpbuffer : ::windows::core::PWSTR , nsize : u32 , arguments : *const *const i8 ) -> u32 );
    FormatMessageW(dwflags, ::core::mem::transmute(lpsource.unwrap_or(::std::ptr::null())), dwmessageid, dwlanguageid, ::core::mem::transmute(lpbuffer), nsize, ::core::mem::transmute(arguments.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[inline]
pub unsafe fn GetEnabledXStateFeatures() -> u64 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetEnabledXStateFeatures ( ) -> u64 );
    GetEnabledXStateFeatures()
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn GetErrorMode() -> u32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetErrorMode ( ) -> u32 );
    GetErrorMode()
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn GetImageConfigInformation(loadedimage: *const LOADED_IMAGE, imageconfiginformation: *mut IMAGE_LOAD_CONFIG_DIRECTORY64) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "imagehlp.dll""system" fn GetImageConfigInformation ( loadedimage : *const LOADED_IMAGE , imageconfiginformation : *mut IMAGE_LOAD_CONFIG_DIRECTORY64 ) -> super::super::super::Foundation:: BOOL );
    GetImageConfigInformation(loadedimage, imageconfiginformation)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn GetImageConfigInformation(loadedimage: *const LOADED_IMAGE, imageconfiginformation: *mut IMAGE_LOAD_CONFIG_DIRECTORY32) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "imagehlp.dll""system" fn GetImageConfigInformation ( loadedimage : *const LOADED_IMAGE , imageconfiginformation : *mut IMAGE_LOAD_CONFIG_DIRECTORY32 ) -> super::super::super::Foundation:: BOOL );
    GetImageConfigInformation(loadedimage, imageconfiginformation)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn GetImageUnusedHeaderBytes(loadedimage: *const LOADED_IMAGE, sizeunusedheaderbytes: *mut u32) -> u32 {
    ::windows_targets::link ! ( "imagehlp.dll""system" fn GetImageUnusedHeaderBytes ( loadedimage : *const LOADED_IMAGE , sizeunusedheaderbytes : *mut u32 ) -> u32 );
    GetImageUnusedHeaderBytes(loadedimage, sizeunusedheaderbytes)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn GetSymLoadError() -> u32 {
    ::windows_targets::link ! ( "dbghelp.dll""system" fn GetSymLoadError ( ) -> u32 );
    GetSymLoadError()
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn GetThreadContext<P0>(hthread: P0, lpcontext: *mut CONTEXT) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetThreadContext ( hthread : super::super::super::Foundation:: HANDLE , lpcontext : *mut CONTEXT ) -> super::super::super::Foundation:: BOOL );
    GetThreadContext(hthread.into_param().abi(), lpcontext)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn GetThreadErrorMode() -> u32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetThreadErrorMode ( ) -> u32 );
    GetThreadErrorMode()
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThreadSelectorEntry<P0>(hthread: P0, dwselector: u32, lpselectorentry: *mut LDT_ENTRY) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetThreadSelectorEntry ( hthread : super::super::super::Foundation:: HANDLE , dwselector : u32 , lpselectorentry : *mut LDT_ENTRY ) -> super::super::super::Foundation:: BOOL );
    GetThreadSelectorEntry(hthread.into_param().abi(), dwselector, lpselectorentry)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThreadWaitChain(wcthandle: *const ::core::ffi::c_void, context: usize, flags: WAIT_CHAIN_THREAD_OPTIONS, threadid: u32, nodecount: *mut u32, nodeinfoarray: *mut WAITCHAIN_NODE_INFO, iscycle: *mut i32) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "advapi32.dll""system" fn GetThreadWaitChain ( wcthandle : *const ::core::ffi::c_void , context : usize , flags : WAIT_CHAIN_THREAD_OPTIONS , threadid : u32 , nodecount : *mut u32 , nodeinfoarray : *mut WAITCHAIN_NODE_INFO , iscycle : *mut i32 ) -> super::super::super::Foundation:: BOOL );
    GetThreadWaitChain(wcthandle, context, flags, threadid, nodecount, nodeinfoarray, iscycle)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTimestampForLoadedLibrary<P0>(module: P0) -> u32
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HMODULE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn GetTimestampForLoadedLibrary ( module : super::super::super::Foundation:: HMODULE ) -> u32 );
    GetTimestampForLoadedLibrary(module.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn GetXStateFeaturesMask(context: *const CONTEXT, featuremask: *mut u64) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetXStateFeaturesMask ( context : *const CONTEXT , featuremask : *mut u64 ) -> super::super::super::Foundation:: BOOL );
    GetXStateFeaturesMask(context, featuremask)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_Security_WinTrust\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_WinTrust"))]
#[inline]
pub unsafe fn ImageAddCertificate<P0>(filehandle: P0, certificate: *const super::super::super::Security::WinTrust::WIN_CERTIFICATE, index: *mut u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "imagehlp.dll""system" fn ImageAddCertificate ( filehandle : super::super::super::Foundation:: HANDLE , certificate : *const super::super::super::Security::WinTrust:: WIN_CERTIFICATE , index : *mut u32 ) -> super::super::super::Foundation:: BOOL );
    ImageAddCertificate(filehandle.into_param().abi(), certificate, index)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageDirectoryEntryToData<P0>(base: *const ::core::ffi::c_void, mappedasimage: P0, directoryentry: IMAGE_DIRECTORY_ENTRY, size: *mut u32) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOLEAN>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn ImageDirectoryEntryToData ( base : *const ::core::ffi::c_void , mappedasimage : super::super::super::Foundation:: BOOLEAN , directoryentry : IMAGE_DIRECTORY_ENTRY , size : *mut u32 ) -> *mut ::core::ffi::c_void );
    ImageDirectoryEntryToData(base, mappedasimage.into_param().abi(), directoryentry, size)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageDirectoryEntryToDataEx<P0>(base: *const ::core::ffi::c_void, mappedasimage: P0, directoryentry: IMAGE_DIRECTORY_ENTRY, size: *mut u32, foundheader: ::core::option::Option<*mut *mut IMAGE_SECTION_HEADER>) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOLEAN>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn ImageDirectoryEntryToDataEx ( base : *const ::core::ffi::c_void , mappedasimage : super::super::super::Foundation:: BOOLEAN , directoryentry : IMAGE_DIRECTORY_ENTRY , size : *mut u32 , foundheader : *mut *mut IMAGE_SECTION_HEADER ) -> *mut ::core::ffi::c_void );
    ImageDirectoryEntryToDataEx(base, mappedasimage.into_param().abi(), directoryentry, size, ::core::mem::transmute(foundheader.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageEnumerateCertificates<P0>(filehandle: P0, typefilter: u16, certificatecount: *mut u32, indices: ::core::option::Option<&mut [u32]>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "imagehlp.dll""system" fn ImageEnumerateCertificates ( filehandle : super::super::super::Foundation:: HANDLE , typefilter : u16 , certificatecount : *mut u32 , indices : *mut u32 , indexcount : u32 ) -> super::super::super::Foundation:: BOOL );
    ImageEnumerateCertificates(filehandle.into_param().abi(), typefilter, certificatecount, ::core::mem::transmute(indices.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), indices.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_Security_WinTrust\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_WinTrust"))]
#[inline]
pub unsafe fn ImageGetCertificateData<P0>(filehandle: P0, certificateindex: u32, certificate: *mut super::super::super::Security::WinTrust::WIN_CERTIFICATE, requiredlength: *mut u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "imagehlp.dll""system" fn ImageGetCertificateData ( filehandle : super::super::super::Foundation:: HANDLE , certificateindex : u32 , certificate : *mut super::super::super::Security::WinTrust:: WIN_CERTIFICATE , requiredlength : *mut u32 ) -> super::super::super::Foundation:: BOOL );
    ImageGetCertificateData(filehandle.into_param().abi(), certificateindex, certificate, requiredlength)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_Security_WinTrust\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_WinTrust"))]
#[inline]
pub unsafe fn ImageGetCertificateHeader<P0>(filehandle: P0, certificateindex: u32, certificateheader: *mut super::super::super::Security::WinTrust::WIN_CERTIFICATE) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "imagehlp.dll""system" fn ImageGetCertificateHeader ( filehandle : super::super::super::Foundation:: HANDLE , certificateindex : u32 , certificateheader : *mut super::super::super::Security::WinTrust:: WIN_CERTIFICATE ) -> super::super::super::Foundation:: BOOL );
    ImageGetCertificateHeader(filehandle.into_param().abi(), certificateindex, certificateheader)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageGetDigestStream<P0>(filehandle: P0, digestlevel: u32, digestfunction: DIGEST_FUNCTION, digesthandle: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "imagehlp.dll""system" fn ImageGetDigestStream ( filehandle : super::super::super::Foundation:: HANDLE , digestlevel : u32 , digestfunction : DIGEST_FUNCTION , digesthandle : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    ImageGetDigestStream(filehandle.into_param().abi(), digestlevel, digestfunction, digesthandle)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn ImageLoad<P0, P1>(dllname: P0, dllpath: P1) -> *mut LOADED_IMAGE
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "imagehlp.dll""system" fn ImageLoad ( dllname : ::windows::core::PCSTR , dllpath : ::windows::core::PCSTR ) -> *mut LOADED_IMAGE );
    ImageLoad(dllname.into_param().abi(), dllpath.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_SystemInformation")]
#[inline]
pub unsafe fn ImageNtHeader(base: *const ::core::ffi::c_void) -> *mut IMAGE_NT_HEADERS64 {
    ::windows_targets::link ! ( "dbghelp.dll""system" fn ImageNtHeader ( base : *const ::core::ffi::c_void ) -> *mut IMAGE_NT_HEADERS64 );
    ImageNtHeader(base)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_SystemInformation")]
#[inline]
pub unsafe fn ImageNtHeader(base: *const ::core::ffi::c_void) -> *mut IMAGE_NT_HEADERS32 {
    ::windows_targets::link ! ( "dbghelp.dll""system" fn ImageNtHeader ( base : *const ::core::ffi::c_void ) -> *mut IMAGE_NT_HEADERS32 );
    ImageNtHeader(base)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageRemoveCertificate<P0>(filehandle: P0, index: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "imagehlp.dll""system" fn ImageRemoveCertificate ( filehandle : super::super::super::Foundation:: HANDLE , index : u32 ) -> super::super::super::Foundation:: BOOL );
    ImageRemoveCertificate(filehandle.into_param().abi(), index)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_SystemInformation")]
#[inline]
pub unsafe fn ImageRvaToSection(ntheaders: *const IMAGE_NT_HEADERS64, base: *const ::core::ffi::c_void, rva: u32) -> *mut IMAGE_SECTION_HEADER {
    ::windows_targets::link ! ( "dbghelp.dll""system" fn ImageRvaToSection ( ntheaders : *const IMAGE_NT_HEADERS64 , base : *const ::core::ffi::c_void , rva : u32 ) -> *mut IMAGE_SECTION_HEADER );
    ImageRvaToSection(ntheaders, base, rva)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_SystemInformation")]
#[inline]
pub unsafe fn ImageRvaToSection(ntheaders: *const IMAGE_NT_HEADERS32, base: *const ::core::ffi::c_void, rva: u32) -> *mut IMAGE_SECTION_HEADER {
    ::windows_targets::link ! ( "dbghelp.dll""system" fn ImageRvaToSection ( ntheaders : *const IMAGE_NT_HEADERS32 , base : *const ::core::ffi::c_void , rva : u32 ) -> *mut IMAGE_SECTION_HEADER );
    ImageRvaToSection(ntheaders, base, rva)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_SystemInformation")]
#[inline]
pub unsafe fn ImageRvaToVa(ntheaders: *const IMAGE_NT_HEADERS64, base: *const ::core::ffi::c_void, rva: u32, lastrvasection: ::core::option::Option<*const *const IMAGE_SECTION_HEADER>) -> *mut ::core::ffi::c_void {
    ::windows_targets::link ! ( "dbghelp.dll""system" fn ImageRvaToVa ( ntheaders : *const IMAGE_NT_HEADERS64 , base : *const ::core::ffi::c_void , rva : u32 , lastrvasection : *const *const IMAGE_SECTION_HEADER ) -> *mut ::core::ffi::c_void );
    ImageRvaToVa(ntheaders, base, rva, ::core::mem::transmute(lastrvasection.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_SystemInformation")]
#[inline]
pub unsafe fn ImageRvaToVa(ntheaders: *const IMAGE_NT_HEADERS32, base: *const ::core::ffi::c_void, rva: u32, lastrvasection: ::core::option::Option<*const *const IMAGE_SECTION_HEADER>) -> *mut ::core::ffi::c_void {
    ::windows_targets::link ! ( "dbghelp.dll""system" fn ImageRvaToVa ( ntheaders : *const IMAGE_NT_HEADERS32 , base : *const ::core::ffi::c_void , rva : u32 , lastrvasection : *const *const IMAGE_SECTION_HEADER ) -> *mut ::core::ffi::c_void );
    ImageRvaToVa(ntheaders, base, rva, ::core::mem::transmute(lastrvasection.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn ImageUnload(loadedimage: *mut LOADED_IMAGE) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "imagehlp.dll""system" fn ImageUnload ( loadedimage : *mut LOADED_IMAGE ) -> super::super::super::Foundation:: BOOL );
    ImageUnload(loadedimage)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn ImagehlpApiVersion() -> *mut API_VERSION {
    ::windows_targets::link ! ( "dbghelp.dll""system" fn ImagehlpApiVersion ( ) -> *mut API_VERSION );
    ImagehlpApiVersion()
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn ImagehlpApiVersionEx(appversion: *const API_VERSION) -> *mut API_VERSION {
    ::windows_targets::link ! ( "dbghelp.dll""system" fn ImagehlpApiVersionEx ( appversion : *const API_VERSION ) -> *mut API_VERSION );
    ImagehlpApiVersionEx(appversion)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn InitializeContext(buffer: ::core::option::Option<*mut ::core::ffi::c_void>, contextflags: u32, context: *mut *mut CONTEXT, contextlength: *mut u32) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn InitializeContext ( buffer : *mut ::core::ffi::c_void , contextflags : u32 , context : *mut *mut CONTEXT , contextlength : *mut u32 ) -> super::super::super::Foundation:: BOOL );
    InitializeContext(::core::mem::transmute(buffer.unwrap_or(::std::ptr::null_mut())), contextflags, context, contextlength)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn InitializeContext2(buffer: ::core::option::Option<*mut ::core::ffi::c_void>, contextflags: u32, context: *mut *mut CONTEXT, contextlength: *mut u32, xstatecompactionmask: u64) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn InitializeContext2 ( buffer : *mut ::core::ffi::c_void , contextflags : u32 , context : *mut *mut CONTEXT , contextlength : *mut u32 , xstatecompactionmask : u64 ) -> super::super::super::Foundation:: BOOL );
    InitializeContext2(::core::mem::transmute(buffer.unwrap_or(::std::ptr::null_mut())), contextflags, context, contextlength, xstatecompactionmask)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsDebuggerPresent() -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn IsDebuggerPresent ( ) -> super::super::super::Foundation:: BOOL );
    IsDebuggerPresent()
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn LocateXStateFeature(context: *const CONTEXT, featureid: u32, length: ::core::option::Option<*mut u32>) -> *mut ::core::ffi::c_void {
    ::windows_targets::link ! ( "kernel32.dll""system" fn LocateXStateFeature ( context : *const CONTEXT , featureid : u32 , length : *mut u32 ) -> *mut ::core::ffi::c_void );
    LocateXStateFeature(context, featureid, ::core::mem::transmute(length.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MakeSureDirectoryPathExists<P0>(dirpath: P0) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn MakeSureDirectoryPathExists ( dirpath : ::windows::core::PCSTR ) -> super::super::super::Foundation:: BOOL );
    MakeSureDirectoryPathExists(dirpath.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn MapAndLoad<P0, P1, P2, P3>(imagename: P0, dllpath: P1, loadedimage: *mut LOADED_IMAGE, dotdll: P2, readonly: P3) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    P3: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "imagehlp.dll""system" fn MapAndLoad ( imagename : ::windows::core::PCSTR , dllpath : ::windows::core::PCSTR , loadedimage : *mut LOADED_IMAGE , dotdll : super::super::super::Foundation:: BOOL , readonly : super::super::super::Foundation:: BOOL ) -> super::super::super::Foundation:: BOOL );
    MapAndLoad(imagename.into_param().abi(), dllpath.into_param().abi(), loadedimage, dotdll.into_param().abi(), readonly.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn MapFileAndCheckSumA<P0>(filename: P0, headersum: *mut u32, checksum: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "imagehlp.dll""system" fn MapFileAndCheckSumA ( filename : ::windows::core::PCSTR , headersum : *mut u32 , checksum : *mut u32 ) -> u32 );
    MapFileAndCheckSumA(filename.into_param().abi(), headersum, checksum)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn MapFileAndCheckSumW<P0>(filename: P0, headersum: *mut u32, checksum: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "imagehlp.dll""system" fn MapFileAndCheckSumW ( filename : ::windows::core::PCWSTR , headersum : *mut u32 , checksum : *mut u32 ) -> u32 );
    MapFileAndCheckSumW(filename.into_param().abi(), headersum, checksum)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn MessageBeep(utype: super::super::super::UI::WindowsAndMessaging::MESSAGEBOX_STYLE) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "user32.dll""system" fn MessageBeep ( utype : super::super::super::UI::WindowsAndMessaging:: MESSAGEBOX_STYLE ) -> super::super::super::Foundation:: BOOL );
    MessageBeep(utype)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MiniDumpReadDumpStream(baseofdump: *const ::core::ffi::c_void, streamnumber: u32, dir: *mut *mut MINIDUMP_DIRECTORY, streampointer: *mut *mut ::core::ffi::c_void, streamsize: ::core::option::Option<*mut u32>) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "dbghelp.dll""system" fn MiniDumpReadDumpStream ( baseofdump : *const ::core::ffi::c_void , streamnumber : u32 , dir : *mut *mut MINIDUMP_DIRECTORY , streampointer : *mut *mut ::core::ffi::c_void , streamsize : *mut u32 ) -> super::super::super::Foundation:: BOOL );
    MiniDumpReadDumpStream(baseofdump, streamnumber, dir, streampointer, ::core::mem::transmute(streamsize.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_FileSystem\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Memory\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_Kernel", feature = "Win32_System_Memory"))]
#[inline]
pub unsafe fn MiniDumpWriteDump<P0, P1>(hprocess: P0, processid: u32, hfile: P1, dumptype: MINIDUMP_TYPE, exceptionparam: ::core::option::Option<*const MINIDUMP_EXCEPTION_INFORMATION>, userstreamparam: ::core::option::Option<*const MINIDUMP_USER_STREAM_INFORMATION>, callbackparam: ::core::option::Option<*const MINIDUMP_CALLBACK_INFORMATION>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn MiniDumpWriteDump ( hprocess : super::super::super::Foundation:: HANDLE , processid : u32 , hfile : super::super::super::Foundation:: HANDLE , dumptype : MINIDUMP_TYPE , exceptionparam : *const MINIDUMP_EXCEPTION_INFORMATION , userstreamparam : *const MINIDUMP_USER_STREAM_INFORMATION , callbackparam : *const MINIDUMP_CALLBACK_INFORMATION ) -> super::super::super::Foundation:: BOOL );
    MiniDumpWriteDump(hprocess.into_param().abi(), processid, hfile.into_param().abi(), dumptype, ::core::mem::transmute(exceptionparam.unwrap_or(::std::ptr::null())), ::core::mem::transmute(userstreamparam.unwrap_or(::std::ptr::null())), ::core::mem::transmute(callbackparam.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenThreadWaitChainSession(flags: OPEN_THREAD_WAIT_CHAIN_SESSION_FLAGS, callback: PWAITCHAINCALLBACK) -> *mut ::core::ffi::c_void {
    ::windows_targets::link ! ( "advapi32.dll""system" fn OpenThreadWaitChainSession ( flags : OPEN_THREAD_WAIT_CHAIN_SESSION_FLAGS , callback : PWAITCHAINCALLBACK ) -> *mut ::core::ffi::c_void );
    OpenThreadWaitChainSession(flags, callback)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn OutputDebugStringA<P0>(lpoutputstring: P0)
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn OutputDebugStringA ( lpoutputstring : ::windows::core::PCSTR ) -> ( ) );
    OutputDebugStringA(lpoutputstring.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn OutputDebugStringW<P0>(lpoutputstring: P0)
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn OutputDebugStringW ( lpoutputstring : ::windows::core::PCWSTR ) -> ( ) );
    OutputDebugStringW(lpoutputstring.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn RaiseException(dwexceptioncode: u32, dwexceptionflags: u32, lparguments: ::core::option::Option<&[usize]>) {
    ::windows_targets::link ! ( "kernel32.dll""system" fn RaiseException ( dwexceptioncode : u32 , dwexceptionflags : u32 , nnumberofarguments : u32 , lparguments : *const usize ) -> ( ) );
    RaiseException(dwexceptioncode, dwexceptionflags, lparguments.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lparguments.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn RaiseFailFastException(pexceptionrecord: ::core::option::Option<*const EXCEPTION_RECORD>, pcontextrecord: ::core::option::Option<*const CONTEXT>, dwflags: u32) {
    ::windows_targets::link ! ( "kernel32.dll""system" fn RaiseFailFastException ( pexceptionrecord : *const EXCEPTION_RECORD , pcontextrecord : *const CONTEXT , dwflags : u32 ) -> ( ) );
    RaiseFailFastException(::core::mem::transmute(pexceptionrecord.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pcontextrecord.unwrap_or(::std::ptr::null())), dwflags)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RangeMapAddPeImageSections<P0>(rmaphandle: *const ::core::ffi::c_void, imagename: P0, mappedimage: *const ::core::ffi::c_void, mappingbytes: u32, imagebase: u64, usertag: u64, mappingflags: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn RangeMapAddPeImageSections ( rmaphandle : *const ::core::ffi::c_void , imagename : ::windows::core::PCWSTR , mappedimage : *const ::core::ffi::c_void , mappingbytes : u32 , imagebase : u64 , usertag : u64 , mappingflags : u32 ) -> super::super::super::Foundation:: BOOL );
    RangeMapAddPeImageSections(rmaphandle, imagename.into_param().abi(), mappedimage, mappingbytes, imagebase, usertag, mappingflags)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn RangeMapCreate() -> *mut ::core::ffi::c_void {
    ::windows_targets::link ! ( "dbghelp.dll""system" fn RangeMapCreate ( ) -> *mut ::core::ffi::c_void );
    RangeMapCreate()
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn RangeMapFree(rmaphandle: ::core::option::Option<*const ::core::ffi::c_void>) {
    ::windows_targets::link ! ( "dbghelp.dll""system" fn RangeMapFree ( rmaphandle : *const ::core::ffi::c_void ) -> ( ) );
    RangeMapFree(::core::mem::transmute(rmaphandle.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RangeMapRead(rmaphandle: *const ::core::ffi::c_void, offset: u64, buffer: *mut ::core::ffi::c_void, requestbytes: u32, flags: u32, donebytes: ::core::option::Option<*mut u32>) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "dbghelp.dll""system" fn RangeMapRead ( rmaphandle : *const ::core::ffi::c_void , offset : u64 , buffer : *mut ::core::ffi::c_void , requestbytes : u32 , flags : u32 , donebytes : *mut u32 ) -> super::super::super::Foundation:: BOOL );
    RangeMapRead(rmaphandle, offset, buffer, requestbytes, flags, ::core::mem::transmute(donebytes.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RangeMapRemove(rmaphandle: *const ::core::ffi::c_void, usertag: u64) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "dbghelp.dll""system" fn RangeMapRemove ( rmaphandle : *const ::core::ffi::c_void , usertag : u64 ) -> super::super::super::Foundation:: BOOL );
    RangeMapRemove(rmaphandle, usertag)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RangeMapWrite(rmaphandle: *const ::core::ffi::c_void, offset: u64, buffer: *const ::core::ffi::c_void, requestbytes: u32, flags: u32, donebytes: ::core::option::Option<*mut u32>) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "dbghelp.dll""system" fn RangeMapWrite ( rmaphandle : *const ::core::ffi::c_void , offset : u64 , buffer : *const ::core::ffi::c_void , requestbytes : u32 , flags : u32 , donebytes : *mut u32 ) -> super::super::super::Foundation:: BOOL );
    RangeMapWrite(rmaphandle, offset, buffer, requestbytes, flags, ::core::mem::transmute(donebytes.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReBaseImage<P0, P1, P2, P3, P4>(currentimagename: P0, symbolpath: P1, frebase: P2, frebasesysfileok: P3, fgoingdown: P4, checkimagesize: u32, oldimagesize: *mut u32, oldimagebase: *mut usize, newimagesize: *mut u32, newimagebase: *mut usize, timestamp: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    P3: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    P4: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "imagehlp.dll""system" fn ReBaseImage ( currentimagename : ::windows::core::PCSTR , symbolpath : ::windows::core::PCSTR , frebase : super::super::super::Foundation:: BOOL , frebasesysfileok : super::super::super::Foundation:: BOOL , fgoingdown : super::super::super::Foundation:: BOOL , checkimagesize : u32 , oldimagesize : *mut u32 , oldimagebase : *mut usize , newimagesize : *mut u32 , newimagebase : *mut usize , timestamp : u32 ) -> super::super::super::Foundation:: BOOL );
    ReBaseImage(currentimagename.into_param().abi(), symbolpath.into_param().abi(), frebase.into_param().abi(), frebasesysfileok.into_param().abi(), fgoingdown.into_param().abi(), checkimagesize, oldimagesize, oldimagebase, newimagesize, newimagebase, timestamp)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReBaseImage64<P0, P1, P2, P3, P4>(currentimagename: P0, symbolpath: P1, frebase: P2, frebasesysfileok: P3, fgoingdown: P4, checkimagesize: u32, oldimagesize: *mut u32, oldimagebase: *mut u64, newimagesize: *mut u32, newimagebase: *mut u64, timestamp: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    P3: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    P4: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "imagehlp.dll""system" fn ReBaseImage64 ( currentimagename : ::windows::core::PCSTR , symbolpath : ::windows::core::PCSTR , frebase : super::super::super::Foundation:: BOOL , frebasesysfileok : super::super::super::Foundation:: BOOL , fgoingdown : super::super::super::Foundation:: BOOL , checkimagesize : u32 , oldimagesize : *mut u32 , oldimagebase : *mut u64 , newimagesize : *mut u32 , newimagebase : *mut u64 , timestamp : u32 ) -> super::super::super::Foundation:: BOOL );
    ReBaseImage64(currentimagename.into_param().abi(), symbolpath.into_param().abi(), frebase.into_param().abi(), frebasesysfileok.into_param().abi(), fgoingdown.into_param().abi(), checkimagesize, oldimagesize, oldimagebase, newimagesize, newimagebase, timestamp)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadProcessMemory<P0>(hprocess: P0, lpbaseaddress: *const ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, nsize: usize, lpnumberofbytesread: ::core::option::Option<*mut usize>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn ReadProcessMemory ( hprocess : super::super::super::Foundation:: HANDLE , lpbaseaddress : *const ::core::ffi::c_void , lpbuffer : *mut ::core::ffi::c_void , nsize : usize , lpnumberofbytesread : *mut usize ) -> super::super::super::Foundation:: BOOL );
    ReadProcessMemory(hprocess.into_param().abi(), lpbaseaddress, lpbuffer, nsize, ::core::mem::transmute(lpnumberofbytesread.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn RegisterWaitChainCOMCallback(callstatecallback: PCOGETCALLSTATE, activationstatecallback: PCOGETACTIVATIONSTATE) {
    ::windows_targets::link ! ( "advapi32.dll""system" fn RegisterWaitChainCOMCallback ( callstatecallback : PCOGETCALLSTATE , activationstatecallback : PCOGETACTIVATIONSTATE ) -> ( ) );
    RegisterWaitChainCOMCallback(callstatecallback, activationstatecallback)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveInvalidModuleList<P0>(hprocess: P0)
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn RemoveInvalidModuleList ( hprocess : super::super::super::Foundation:: HANDLE ) -> ( ) );
    RemoveInvalidModuleList(hprocess.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn RemoveVectoredContinueHandler(handle: *const ::core::ffi::c_void) -> u32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn RemoveVectoredContinueHandler ( handle : *const ::core::ffi::c_void ) -> u32 );
    RemoveVectoredContinueHandler(handle)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn RemoveVectoredExceptionHandler(handle: *const ::core::ffi::c_void) -> u32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn RemoveVectoredExceptionHandler ( handle : *const ::core::ffi::c_void ) -> u32 );
    RemoveVectoredExceptionHandler(handle)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReportSymbolLoadSummary<P0, P1>(hprocess: P0, ploadmodule: P1, psymboldata: *const DBGHELP_DATA_REPORT_STRUCT) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn ReportSymbolLoadSummary ( hprocess : super::super::super::Foundation:: HANDLE , ploadmodule : ::windows::core::PCWSTR , psymboldata : *const DBGHELP_DATA_REPORT_STRUCT ) -> super::super::super::Foundation:: BOOL );
    ReportSymbolLoadSummary(hprocess.into_param().abi(), ploadmodule.into_param().abi(), psymboldata)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlAddFunctionTable(functiontable: &[IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY], baseaddress: usize) -> super::super::super::Foundation::BOOLEAN {
    ::windows_targets::link ! ( "kernel32.dll""system" fn RtlAddFunctionTable ( functiontable : *const IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY , entrycount : u32 , baseaddress : usize ) -> super::super::super::Foundation:: BOOLEAN );
    RtlAddFunctionTable(::core::mem::transmute(functiontable.as_ptr()), functiontable.len() as _, baseaddress)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86_64")]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlAddFunctionTable(functiontable: &[IMAGE_RUNTIME_FUNCTION_ENTRY], baseaddress: u64) -> super::super::super::Foundation::BOOLEAN {
    ::windows_targets::link ! ( "kernel32.dll""system" fn RtlAddFunctionTable ( functiontable : *const IMAGE_RUNTIME_FUNCTION_ENTRY , entrycount : u32 , baseaddress : u64 ) -> super::super::super::Foundation:: BOOLEAN );
    RtlAddFunctionTable(::core::mem::transmute(functiontable.as_ptr()), functiontable.len() as _, baseaddress)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "aarch64")]
#[inline]
pub unsafe fn RtlAddGrowableFunctionTable(dynamictable: *mut *mut ::core::ffi::c_void, functiontable: &[IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY], entrycount: u32, rangebase: usize, rangeend: usize) -> u32 {
    ::windows_targets::link ! ( "ntdll.dll""system" fn RtlAddGrowableFunctionTable ( dynamictable : *mut *mut ::core::ffi::c_void , functiontable : *const IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY , entrycount : u32 , maximumentrycount : u32 , rangebase : usize , rangeend : usize ) -> u32 );
    RtlAddGrowableFunctionTable(dynamictable, ::core::mem::transmute(functiontable.as_ptr()), entrycount, functiontable.len() as _, rangebase, rangeend)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86_64")]
#[inline]
pub unsafe fn RtlAddGrowableFunctionTable(dynamictable: *mut *mut ::core::ffi::c_void, functiontable: &[IMAGE_RUNTIME_FUNCTION_ENTRY], entrycount: u32, rangebase: usize, rangeend: usize) -> u32 {
    ::windows_targets::link ! ( "ntdll.dll""system" fn RtlAddGrowableFunctionTable ( dynamictable : *mut *mut ::core::ffi::c_void , functiontable : *const IMAGE_RUNTIME_FUNCTION_ENTRY , entrycount : u32 , maximumentrycount : u32 , rangebase : usize , rangeend : usize ) -> u32 );
    RtlAddGrowableFunctionTable(dynamictable, ::core::mem::transmute(functiontable.as_ptr()), entrycount, functiontable.len() as _, rangebase, rangeend)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn RtlCaptureContext(contextrecord: *mut CONTEXT) {
    ::windows_targets::link ! ( "kernel32.dll""system" fn RtlCaptureContext ( contextrecord : *mut CONTEXT ) -> ( ) );
    RtlCaptureContext(contextrecord)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "x86_64")]
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn RtlCaptureContext2(contextrecord: *mut CONTEXT) {
    ::windows_targets::link ! ( "kernel32.dll""system" fn RtlCaptureContext2 ( contextrecord : *mut CONTEXT ) -> ( ) );
    RtlCaptureContext2(contextrecord)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn RtlCaptureStackBackTrace(framestoskip: u32, backtrace: &mut [*mut ::core::ffi::c_void], backtracehash: ::core::option::Option<*mut u32>) -> u16 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn RtlCaptureStackBackTrace ( framestoskip : u32 , framestocapture : u32 , backtrace : *mut *mut ::core::ffi::c_void , backtracehash : *mut u32 ) -> u16 );
    RtlCaptureStackBackTrace(framestoskip, backtrace.len() as _, ::core::mem::transmute(backtrace.as_ptr()), ::core::mem::transmute(backtracehash.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlDeleteFunctionTable(functiontable: *const IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY) -> super::super::super::Foundation::BOOLEAN {
    ::windows_targets::link ! ( "kernel32.dll""system" fn RtlDeleteFunctionTable ( functiontable : *const IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY ) -> super::super::super::Foundation:: BOOLEAN );
    RtlDeleteFunctionTable(functiontable)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86_64")]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlDeleteFunctionTable(functiontable: *const IMAGE_RUNTIME_FUNCTION_ENTRY) -> super::super::super::Foundation::BOOLEAN {
    ::windows_targets::link ! ( "kernel32.dll""system" fn RtlDeleteFunctionTable ( functiontable : *const IMAGE_RUNTIME_FUNCTION_ENTRY ) -> super::super::super::Foundation:: BOOLEAN );
    RtlDeleteFunctionTable(functiontable)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlDeleteGrowableFunctionTable(dynamictable: *const ::core::ffi::c_void) {
    ::windows_targets::link ! ( "ntdll.dll""system" fn RtlDeleteGrowableFunctionTable ( dynamictable : *const ::core::ffi::c_void ) -> ( ) );
    RtlDeleteGrowableFunctionTable(dynamictable)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlGrowFunctionTable(dynamictable: *mut ::core::ffi::c_void, newentrycount: u32) {
    ::windows_targets::link ! ( "ntdll.dll""system" fn RtlGrowFunctionTable ( dynamictable : *mut ::core::ffi::c_void , newentrycount : u32 ) -> ( ) );
    RtlGrowFunctionTable(dynamictable, newentrycount)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlInstallFunctionTableCallback<P0>(tableidentifier: u64, baseaddress: u64, length: u32, callback: PGET_RUNTIME_FUNCTION_CALLBACK, context: ::core::option::Option<*const ::core::ffi::c_void>, outofprocesscallbackdll: P0) -> super::super::super::Foundation::BOOLEAN
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn RtlInstallFunctionTableCallback ( tableidentifier : u64 , baseaddress : u64 , length : u32 , callback : PGET_RUNTIME_FUNCTION_CALLBACK , context : *const ::core::ffi::c_void , outofprocesscallbackdll : ::windows::core::PCWSTR ) -> super::super::super::Foundation:: BOOLEAN );
    RtlInstallFunctionTableCallback(tableidentifier, baseaddress, length, callback, ::core::mem::transmute(context.unwrap_or(::std::ptr::null())), outofprocesscallbackdll.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86_64")]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlInstallFunctionTableCallback<P0>(tableidentifier: u64, baseaddress: u64, length: u32, callback: PGET_RUNTIME_FUNCTION_CALLBACK, context: ::core::option::Option<*const ::core::ffi::c_void>, outofprocesscallbackdll: P0) -> super::super::super::Foundation::BOOLEAN
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn RtlInstallFunctionTableCallback ( tableidentifier : u64 , baseaddress : u64 , length : u32 , callback : PGET_RUNTIME_FUNCTION_CALLBACK , context : *const ::core::ffi::c_void , outofprocesscallbackdll : ::windows::core::PCWSTR ) -> super::super::super::Foundation:: BOOLEAN );
    RtlInstallFunctionTableCallback(tableidentifier, baseaddress, length, callback, ::core::mem::transmute(context.unwrap_or(::std::ptr::null())), outofprocesscallbackdll.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "aarch64")]
#[inline]
pub unsafe fn RtlLookupFunctionEntry(controlpc: usize, imagebase: *mut usize, historytable: ::core::option::Option<*mut UNWIND_HISTORY_TABLE>) -> *mut IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY {
    ::windows_targets::link ! ( "kernel32.dll""system" fn RtlLookupFunctionEntry ( controlpc : usize , imagebase : *mut usize , historytable : *mut UNWIND_HISTORY_TABLE ) -> *mut IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY );
    RtlLookupFunctionEntry(controlpc, imagebase, ::core::mem::transmute(historytable.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86_64")]
#[inline]
pub unsafe fn RtlLookupFunctionEntry(controlpc: u64, imagebase: *mut u64, historytable: ::core::option::Option<*mut UNWIND_HISTORY_TABLE>) -> *mut IMAGE_RUNTIME_FUNCTION_ENTRY {
    ::windows_targets::link ! ( "kernel32.dll""system" fn RtlLookupFunctionEntry ( controlpc : u64 , imagebase : *mut u64 , historytable : *mut UNWIND_HISTORY_TABLE ) -> *mut IMAGE_RUNTIME_FUNCTION_ENTRY );
    RtlLookupFunctionEntry(controlpc, imagebase, ::core::mem::transmute(historytable.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn RtlPcToFileHeader(pcvalue: *const ::core::ffi::c_void, baseofimage: *mut *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    ::windows_targets::link ! ( "kernel32.dll""system" fn RtlPcToFileHeader ( pcvalue : *const ::core::ffi::c_void , baseofimage : *mut *mut ::core::ffi::c_void ) -> *mut ::core::ffi::c_void );
    RtlPcToFileHeader(pcvalue, baseofimage)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlRaiseException(exceptionrecord: *const EXCEPTION_RECORD) {
    ::windows_targets::link ! ( "kernel32.dll""system" fn RtlRaiseException ( exceptionrecord : *const EXCEPTION_RECORD ) -> ( ) );
    RtlRaiseException(exceptionrecord)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn RtlRestoreContext(contextrecord: *const CONTEXT, exceptionrecord: ::core::option::Option<*const EXCEPTION_RECORD>) {
    ::windows_targets::link ! ( "kernel32.dll""cdecl" fn RtlRestoreContext ( contextrecord : *const CONTEXT , exceptionrecord : *const EXCEPTION_RECORD ) -> ( ) );
    RtlRestoreContext(contextrecord, ::core::mem::transmute(exceptionrecord.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlUnwind(targetframe: ::core::option::Option<*const ::core::ffi::c_void>, targetip: ::core::option::Option<*const ::core::ffi::c_void>, exceptionrecord: ::core::option::Option<*const EXCEPTION_RECORD>, returnvalue: *const ::core::ffi::c_void) {
    ::windows_targets::link ! ( "kernel32.dll""system" fn RtlUnwind ( targetframe : *const ::core::ffi::c_void , targetip : *const ::core::ffi::c_void , exceptionrecord : *const EXCEPTION_RECORD , returnvalue : *const ::core::ffi::c_void ) -> ( ) );
    RtlUnwind(::core::mem::transmute(targetframe.unwrap_or(::std::ptr::null())), ::core::mem::transmute(targetip.unwrap_or(::std::ptr::null())), ::core::mem::transmute(exceptionrecord.unwrap_or(::std::ptr::null())), returnvalue)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn RtlUnwindEx(targetframe: ::core::option::Option<*const ::core::ffi::c_void>, targetip: ::core::option::Option<*const ::core::ffi::c_void>, exceptionrecord: ::core::option::Option<*const EXCEPTION_RECORD>, returnvalue: *const ::core::ffi::c_void, contextrecord: *const CONTEXT, historytable: ::core::option::Option<*const UNWIND_HISTORY_TABLE>) {
    ::windows_targets::link ! ( "kernel32.dll""system" fn RtlUnwindEx ( targetframe : *const ::core::ffi::c_void , targetip : *const ::core::ffi::c_void , exceptionrecord : *const EXCEPTION_RECORD , returnvalue : *const ::core::ffi::c_void , contextrecord : *const CONTEXT , historytable : *const UNWIND_HISTORY_TABLE ) -> ( ) );
    RtlUnwindEx(::core::mem::transmute(targetframe.unwrap_or(::std::ptr::null())), ::core::mem::transmute(targetip.unwrap_or(::std::ptr::null())), ::core::mem::transmute(exceptionrecord.unwrap_or(::std::ptr::null())), returnvalue, contextrecord, ::core::mem::transmute(historytable.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn RtlVirtualUnwind(handlertype: RTL_VIRTUAL_UNWIND_HANDLER_TYPE, imagebase: usize, controlpc: usize, functionentry: *const IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY, contextrecord: *mut CONTEXT, handlerdata: *mut *mut ::core::ffi::c_void, establisherframe: *mut usize, contextpointers: ::core::option::Option<*mut KNONVOLATILE_CONTEXT_POINTERS_ARM64>) -> super::super::Kernel::EXCEPTION_ROUTINE {
    ::windows_targets::link ! ( "kernel32.dll""system" fn RtlVirtualUnwind ( handlertype : RTL_VIRTUAL_UNWIND_HANDLER_TYPE , imagebase : usize , controlpc : usize , functionentry : *const IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY , contextrecord : *mut CONTEXT , handlerdata : *mut *mut ::core::ffi::c_void , establisherframe : *mut usize , contextpointers : *mut KNONVOLATILE_CONTEXT_POINTERS_ARM64 ) -> super::super::Kernel:: EXCEPTION_ROUTINE );
    RtlVirtualUnwind(handlertype, imagebase, controlpc, functionentry, contextrecord, handlerdata, establisherframe, ::core::mem::transmute(contextpointers.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "x86_64")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn RtlVirtualUnwind(handlertype: RTL_VIRTUAL_UNWIND_HANDLER_TYPE, imagebase: u64, controlpc: u64, functionentry: *const IMAGE_RUNTIME_FUNCTION_ENTRY, contextrecord: *mut CONTEXT, handlerdata: *mut *mut ::core::ffi::c_void, establisherframe: *mut u64, contextpointers: ::core::option::Option<*mut KNONVOLATILE_CONTEXT_POINTERS>) -> super::super::Kernel::EXCEPTION_ROUTINE {
    ::windows_targets::link ! ( "kernel32.dll""system" fn RtlVirtualUnwind ( handlertype : RTL_VIRTUAL_UNWIND_HANDLER_TYPE , imagebase : u64 , controlpc : u64 , functionentry : *const IMAGE_RUNTIME_FUNCTION_ENTRY , contextrecord : *mut CONTEXT , handlerdata : *mut *mut ::core::ffi::c_void , establisherframe : *mut u64 , contextpointers : *mut KNONVOLATILE_CONTEXT_POINTERS ) -> super::super::Kernel:: EXCEPTION_ROUTINE );
    RtlVirtualUnwind(handlertype, imagebase, controlpc, functionentry, contextrecord, handlerdata, establisherframe, ::core::mem::transmute(contextpointers.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SearchTreeForFile<P0, P1>(rootpath: P0, inputpathname: P1, outputpathbuffer: ::windows::core::PSTR) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SearchTreeForFile ( rootpath : ::windows::core::PCSTR , inputpathname : ::windows::core::PCSTR , outputpathbuffer : ::windows::core::PSTR ) -> super::super::super::Foundation:: BOOL );
    SearchTreeForFile(rootpath.into_param().abi(), inputpathname.into_param().abi(), ::core::mem::transmute(outputpathbuffer))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SearchTreeForFileW<P0, P1>(rootpath: P0, inputpathname: P1, outputpathbuffer: ::windows::core::PWSTR) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SearchTreeForFileW ( rootpath : ::windows::core::PCWSTR , inputpathname : ::windows::core::PCWSTR , outputpathbuffer : ::windows::core::PWSTR ) -> super::super::super::Foundation:: BOOL );
    SearchTreeForFileW(rootpath.into_param().abi(), inputpathname.into_param().abi(), ::core::mem::transmute(outputpathbuffer))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn SetCheckUserInterruptShared(lpstartaddress: LPCALL_BACK_USER_INTERRUPT_ROUTINE) {
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SetCheckUserInterruptShared ( lpstartaddress : LPCALL_BACK_USER_INTERRUPT_ROUTINE ) -> ( ) );
    SetCheckUserInterruptShared(lpstartaddress)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn SetErrorMode(umode: THREAD_ERROR_MODE) -> u32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetErrorMode ( umode : THREAD_ERROR_MODE ) -> u32 );
    SetErrorMode(umode)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn SetImageConfigInformation(loadedimage: *mut LOADED_IMAGE, imageconfiginformation: *const IMAGE_LOAD_CONFIG_DIRECTORY64) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "imagehlp.dll""system" fn SetImageConfigInformation ( loadedimage : *mut LOADED_IMAGE , imageconfiginformation : *const IMAGE_LOAD_CONFIG_DIRECTORY64 ) -> super::super::super::Foundation:: BOOL );
    SetImageConfigInformation(loadedimage, imageconfiginformation)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn SetImageConfigInformation(loadedimage: *mut LOADED_IMAGE, imageconfiginformation: *const IMAGE_LOAD_CONFIG_DIRECTORY32) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "imagehlp.dll""system" fn SetImageConfigInformation ( loadedimage : *mut LOADED_IMAGE , imageconfiginformation : *const IMAGE_LOAD_CONFIG_DIRECTORY32 ) -> super::super::super::Foundation:: BOOL );
    SetImageConfigInformation(loadedimage, imageconfiginformation)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn SetSymLoadError(error: u32) {
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SetSymLoadError ( error : u32 ) -> ( ) );
    SetSymLoadError(error)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn SetThreadContext<P0>(hthread: P0, lpcontext: *const CONTEXT) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetThreadContext ( hthread : super::super::super::Foundation:: HANDLE , lpcontext : *const CONTEXT ) -> super::super::super::Foundation:: BOOL );
    SetThreadContext(hthread.into_param().abi(), lpcontext)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadErrorMode(dwnewmode: THREAD_ERROR_MODE, lpoldmode: ::core::option::Option<*mut THREAD_ERROR_MODE>) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetThreadErrorMode ( dwnewmode : THREAD_ERROR_MODE , lpoldmode : *mut THREAD_ERROR_MODE ) -> super::super::super::Foundation:: BOOL );
    SetThreadErrorMode(dwnewmode, ::core::mem::transmute(lpoldmode.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn SetUnhandledExceptionFilter(lptoplevelexceptionfilter: LPTOP_LEVEL_EXCEPTION_FILTER) -> LPTOP_LEVEL_EXCEPTION_FILTER {
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetUnhandledExceptionFilter ( lptoplevelexceptionfilter : LPTOP_LEVEL_EXCEPTION_FILTER ) -> LPTOP_LEVEL_EXCEPTION_FILTER );
    SetUnhandledExceptionFilter(lptoplevelexceptionfilter)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn SetXStateFeaturesMask(context: *mut CONTEXT, featuremask: u64) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetXStateFeaturesMask ( context : *mut CONTEXT , featuremask : u64 ) -> super::super::super::Foundation:: BOOL );
    SetXStateFeaturesMask(context, featuremask)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StackWalk<P0, P1>(machinetype: u32, hprocess: P0, hthread: P1, stackframe: *mut STACKFRAME, contextrecord: *mut ::core::ffi::c_void, readmemoryroutine: PREAD_PROCESS_MEMORY_ROUTINE, functiontableaccessroutine: PFUNCTION_TABLE_ACCESS_ROUTINE, getmodulebaseroutine: PGET_MODULE_BASE_ROUTINE, translateaddress: PTRANSLATE_ADDRESS_ROUTINE) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn StackWalk ( machinetype : u32 , hprocess : super::super::super::Foundation:: HANDLE , hthread : super::super::super::Foundation:: HANDLE , stackframe : *mut STACKFRAME , contextrecord : *mut ::core::ffi::c_void , readmemoryroutine : PREAD_PROCESS_MEMORY_ROUTINE , functiontableaccessroutine : PFUNCTION_TABLE_ACCESS_ROUTINE , getmodulebaseroutine : PGET_MODULE_BASE_ROUTINE , translateaddress : PTRANSLATE_ADDRESS_ROUTINE ) -> super::super::super::Foundation:: BOOL );
    StackWalk(machinetype, hprocess.into_param().abi(), hthread.into_param().abi(), stackframe, contextrecord, readmemoryroutine, functiontableaccessroutine, getmodulebaseroutine, translateaddress)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StackWalk64<P0, P1>(machinetype: u32, hprocess: P0, hthread: P1, stackframe: *mut STACKFRAME64, contextrecord: *mut ::core::ffi::c_void, readmemoryroutine: PREAD_PROCESS_MEMORY_ROUTINE64, functiontableaccessroutine: PFUNCTION_TABLE_ACCESS_ROUTINE64, getmodulebaseroutine: PGET_MODULE_BASE_ROUTINE64, translateaddress: PTRANSLATE_ADDRESS_ROUTINE64) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn StackWalk64 ( machinetype : u32 , hprocess : super::super::super::Foundation:: HANDLE , hthread : super::super::super::Foundation:: HANDLE , stackframe : *mut STACKFRAME64 , contextrecord : *mut ::core::ffi::c_void , readmemoryroutine : PREAD_PROCESS_MEMORY_ROUTINE64 , functiontableaccessroutine : PFUNCTION_TABLE_ACCESS_ROUTINE64 , getmodulebaseroutine : PGET_MODULE_BASE_ROUTINE64 , translateaddress : PTRANSLATE_ADDRESS_ROUTINE64 ) -> super::super::super::Foundation:: BOOL );
    StackWalk64(machinetype, hprocess.into_param().abi(), hthread.into_param().abi(), stackframe, contextrecord, readmemoryroutine, functiontableaccessroutine, getmodulebaseroutine, translateaddress)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StackWalkEx<P0, P1>(machinetype: u32, hprocess: P0, hthread: P1, stackframe: *mut STACKFRAME_EX, contextrecord: *mut ::core::ffi::c_void, readmemoryroutine: PREAD_PROCESS_MEMORY_ROUTINE64, functiontableaccessroutine: PFUNCTION_TABLE_ACCESS_ROUTINE64, getmodulebaseroutine: PGET_MODULE_BASE_ROUTINE64, translateaddress: PTRANSLATE_ADDRESS_ROUTINE64, flags: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn StackWalkEx ( machinetype : u32 , hprocess : super::super::super::Foundation:: HANDLE , hthread : super::super::super::Foundation:: HANDLE , stackframe : *mut STACKFRAME_EX , contextrecord : *mut ::core::ffi::c_void , readmemoryroutine : PREAD_PROCESS_MEMORY_ROUTINE64 , functiontableaccessroutine : PFUNCTION_TABLE_ACCESS_ROUTINE64 , getmodulebaseroutine : PGET_MODULE_BASE_ROUTINE64 , translateaddress : PTRANSLATE_ADDRESS_ROUTINE64 , flags : u32 ) -> super::super::super::Foundation:: BOOL );
    StackWalkEx(machinetype, hprocess.into_param().abi(), hthread.into_param().abi(), stackframe, contextrecord, readmemoryroutine, functiontableaccessroutine, getmodulebaseroutine, translateaddress, flags)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymAddSourceStream<P0, P1>(hprocess: P0, base: u64, streamfile: P1, buffer: ::core::option::Option<&[u8]>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymAddSourceStream ( hprocess : super::super::super::Foundation:: HANDLE , base : u64 , streamfile : ::windows::core::PCSTR , buffer : *const u8 , size : usize ) -> super::super::super::Foundation:: BOOL );
    SymAddSourceStream(hprocess.into_param().abi(), base, streamfile.into_param().abi(), ::core::mem::transmute(buffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), buffer.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymAddSourceStreamA<P0, P1>(hprocess: P0, base: u64, streamfile: P1, buffer: ::core::option::Option<&[u8]>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymAddSourceStreamA ( hprocess : super::super::super::Foundation:: HANDLE , base : u64 , streamfile : ::windows::core::PCSTR , buffer : *const u8 , size : usize ) -> super::super::super::Foundation:: BOOL );
    SymAddSourceStreamA(hprocess.into_param().abi(), base, streamfile.into_param().abi(), ::core::mem::transmute(buffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), buffer.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymAddSourceStreamW<P0, P1>(hprocess: P0, base: u64, filespec: P1, buffer: ::core::option::Option<&[u8]>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymAddSourceStreamW ( hprocess : super::super::super::Foundation:: HANDLE , base : u64 , filespec : ::windows::core::PCWSTR , buffer : *const u8 , size : usize ) -> super::super::super::Foundation:: BOOL );
    SymAddSourceStreamW(hprocess.into_param().abi(), base, filespec.into_param().abi(), ::core::mem::transmute(buffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), buffer.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymAddSymbol<P0, P1>(hprocess: P0, baseofdll: u64, name: P1, address: u64, size: u32, flags: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymAddSymbol ( hprocess : super::super::super::Foundation:: HANDLE , baseofdll : u64 , name : ::windows::core::PCSTR , address : u64 , size : u32 , flags : u32 ) -> super::super::super::Foundation:: BOOL );
    SymAddSymbol(hprocess.into_param().abi(), baseofdll, name.into_param().abi(), address, size, flags)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymAddSymbolW<P0, P1>(hprocess: P0, baseofdll: u64, name: P1, address: u64, size: u32, flags: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymAddSymbolW ( hprocess : super::super::super::Foundation:: HANDLE , baseofdll : u64 , name : ::windows::core::PCWSTR , address : u64 , size : u32 , flags : u32 ) -> super::super::super::Foundation:: BOOL );
    SymAddSymbolW(hprocess.into_param().abi(), baseofdll, name.into_param().abi(), address, size, flags)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymAddrIncludeInlineTrace<P0>(hprocess: P0, address: u64) -> u32
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymAddrIncludeInlineTrace ( hprocess : super::super::super::Foundation:: HANDLE , address : u64 ) -> u32 );
    SymAddrIncludeInlineTrace(hprocess.into_param().abi(), address)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymCleanup<P0>(hprocess: P0) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymCleanup ( hprocess : super::super::super::Foundation:: HANDLE ) -> super::super::super::Foundation:: BOOL );
    SymCleanup(hprocess.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymCompareInlineTrace<P0>(hprocess: P0, address1: u64, inlinecontext1: u32, retaddress1: u64, address2: u64, retaddress2: u64) -> u32
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymCompareInlineTrace ( hprocess : super::super::super::Foundation:: HANDLE , address1 : u64 , inlinecontext1 : u32 , retaddress1 : u64 , address2 : u64 , retaddress2 : u64 ) -> u32 );
    SymCompareInlineTrace(hprocess.into_param().abi(), address1, inlinecontext1, retaddress1, address2, retaddress2)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymDeleteSymbol<P0, P1>(hprocess: P0, baseofdll: u64, name: P1, address: u64, flags: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymDeleteSymbol ( hprocess : super::super::super::Foundation:: HANDLE , baseofdll : u64 , name : ::windows::core::PCSTR , address : u64 , flags : u32 ) -> super::super::super::Foundation:: BOOL );
    SymDeleteSymbol(hprocess.into_param().abi(), baseofdll, name.into_param().abi(), address, flags)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymDeleteSymbolW<P0, P1>(hprocess: P0, baseofdll: u64, name: P1, address: u64, flags: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymDeleteSymbolW ( hprocess : super::super::super::Foundation:: HANDLE , baseofdll : u64 , name : ::windows::core::PCWSTR , address : u64 , flags : u32 ) -> super::super::super::Foundation:: BOOL );
    SymDeleteSymbolW(hprocess.into_param().abi(), baseofdll, name.into_param().abi(), address, flags)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymEnumLines<P0, P1, P2>(hprocess: P0, base: u64, obj: P1, file: P2, enumlinescallback: PSYM_ENUMLINES_CALLBACK, usercontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymEnumLines ( hprocess : super::super::super::Foundation:: HANDLE , base : u64 , obj : ::windows::core::PCSTR , file : ::windows::core::PCSTR , enumlinescallback : PSYM_ENUMLINES_CALLBACK , usercontext : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    SymEnumLines(hprocess.into_param().abi(), base, obj.into_param().abi(), file.into_param().abi(), enumlinescallback, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymEnumLinesW<P0, P1, P2>(hprocess: P0, base: u64, obj: P1, file: P2, enumlinescallback: PSYM_ENUMLINES_CALLBACKW, usercontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymEnumLinesW ( hprocess : super::super::super::Foundation:: HANDLE , base : u64 , obj : ::windows::core::PCWSTR , file : ::windows::core::PCWSTR , enumlinescallback : PSYM_ENUMLINES_CALLBACKW , usercontext : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    SymEnumLinesW(hprocess.into_param().abi(), base, obj.into_param().abi(), file.into_param().abi(), enumlinescallback, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymEnumProcesses(enumprocessescallback: PSYM_ENUMPROCESSES_CALLBACK, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymEnumProcesses ( enumprocessescallback : PSYM_ENUMPROCESSES_CALLBACK , usercontext : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    SymEnumProcesses(enumprocessescallback, usercontext)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymEnumSourceFileTokens<P0>(hprocess: P0, base: u64, callback: PENUMSOURCEFILETOKENSCALLBACK) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymEnumSourceFileTokens ( hprocess : super::super::super::Foundation:: HANDLE , base : u64 , callback : PENUMSOURCEFILETOKENSCALLBACK ) -> super::super::super::Foundation:: BOOL );
    SymEnumSourceFileTokens(hprocess.into_param().abi(), base, callback)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymEnumSourceFiles<P0, P1>(hprocess: P0, modbase: u64, mask: P1, cbsrcfiles: PSYM_ENUMSOURCEFILES_CALLBACK, usercontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymEnumSourceFiles ( hprocess : super::super::super::Foundation:: HANDLE , modbase : u64 , mask : ::windows::core::PCSTR , cbsrcfiles : PSYM_ENUMSOURCEFILES_CALLBACK , usercontext : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    SymEnumSourceFiles(hprocess.into_param().abi(), modbase, mask.into_param().abi(), cbsrcfiles, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymEnumSourceFilesW<P0, P1>(hprocess: P0, modbase: u64, mask: P1, cbsrcfiles: PSYM_ENUMSOURCEFILES_CALLBACKW, usercontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymEnumSourceFilesW ( hprocess : super::super::super::Foundation:: HANDLE , modbase : u64 , mask : ::windows::core::PCWSTR , cbsrcfiles : PSYM_ENUMSOURCEFILES_CALLBACKW , usercontext : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    SymEnumSourceFilesW(hprocess.into_param().abi(), modbase, mask.into_param().abi(), cbsrcfiles, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymEnumSourceLines<P0, P1, P2>(hprocess: P0, base: u64, obj: P1, file: P2, line: u32, flags: u32, enumlinescallback: PSYM_ENUMLINES_CALLBACK, usercontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymEnumSourceLines ( hprocess : super::super::super::Foundation:: HANDLE , base : u64 , obj : ::windows::core::PCSTR , file : ::windows::core::PCSTR , line : u32 , flags : u32 , enumlinescallback : PSYM_ENUMLINES_CALLBACK , usercontext : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    SymEnumSourceLines(hprocess.into_param().abi(), base, obj.into_param().abi(), file.into_param().abi(), line, flags, enumlinescallback, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymEnumSourceLinesW<P0, P1, P2>(hprocess: P0, base: u64, obj: P1, file: P2, line: u32, flags: u32, enumlinescallback: PSYM_ENUMLINES_CALLBACKW, usercontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymEnumSourceLinesW ( hprocess : super::super::super::Foundation:: HANDLE , base : u64 , obj : ::windows::core::PCWSTR , file : ::windows::core::PCWSTR , line : u32 , flags : u32 , enumlinescallback : PSYM_ENUMLINES_CALLBACKW , usercontext : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    SymEnumSourceLinesW(hprocess.into_param().abi(), base, obj.into_param().abi(), file.into_param().abi(), line, flags, enumlinescallback, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymEnumSym<P0>(hprocess: P0, baseofdll: u64, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymEnumSym ( hprocess : super::super::super::Foundation:: HANDLE , baseofdll : u64 , enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK , usercontext : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    SymEnumSym(hprocess.into_param().abi(), baseofdll, enumsymbolscallback, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymEnumSymbols<P0, P1>(hprocess: P0, baseofdll: u64, mask: P1, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymEnumSymbols ( hprocess : super::super::super::Foundation:: HANDLE , baseofdll : u64 , mask : ::windows::core::PCSTR , enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK , usercontext : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    SymEnumSymbols(hprocess.into_param().abi(), baseofdll, mask.into_param().abi(), enumsymbolscallback, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymEnumSymbolsEx<P0, P1>(hprocess: P0, baseofdll: u64, mask: P1, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: ::core::option::Option<*const ::core::ffi::c_void>, options: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymEnumSymbolsEx ( hprocess : super::super::super::Foundation:: HANDLE , baseofdll : u64 , mask : ::windows::core::PCSTR , enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK , usercontext : *const ::core::ffi::c_void , options : u32 ) -> super::super::super::Foundation:: BOOL );
    SymEnumSymbolsEx(hprocess.into_param().abi(), baseofdll, mask.into_param().abi(), enumsymbolscallback, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())), options)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymEnumSymbolsExW<P0, P1>(hprocess: P0, baseofdll: u64, mask: P1, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext: ::core::option::Option<*const ::core::ffi::c_void>, options: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymEnumSymbolsExW ( hprocess : super::super::super::Foundation:: HANDLE , baseofdll : u64 , mask : ::windows::core::PCWSTR , enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACKW , usercontext : *const ::core::ffi::c_void , options : u32 ) -> super::super::super::Foundation:: BOOL );
    SymEnumSymbolsExW(hprocess.into_param().abi(), baseofdll, mask.into_param().abi(), enumsymbolscallback, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())), options)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymEnumSymbolsForAddr<P0>(hprocess: P0, address: u64, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymEnumSymbolsForAddr ( hprocess : super::super::super::Foundation:: HANDLE , address : u64 , enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK , usercontext : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    SymEnumSymbolsForAddr(hprocess.into_param().abi(), address, enumsymbolscallback, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymEnumSymbolsForAddrW<P0>(hprocess: P0, address: u64, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymEnumSymbolsForAddrW ( hprocess : super::super::super::Foundation:: HANDLE , address : u64 , enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACKW , usercontext : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    SymEnumSymbolsForAddrW(hprocess.into_param().abi(), address, enumsymbolscallback, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymEnumSymbolsW<P0, P1>(hprocess: P0, baseofdll: u64, mask: P1, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymEnumSymbolsW ( hprocess : super::super::super::Foundation:: HANDLE , baseofdll : u64 , mask : ::windows::core::PCWSTR , enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACKW , usercontext : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    SymEnumSymbolsW(hprocess.into_param().abi(), baseofdll, mask.into_param().abi(), enumsymbolscallback, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymEnumTypes<P0>(hprocess: P0, baseofdll: u64, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymEnumTypes ( hprocess : super::super::super::Foundation:: HANDLE , baseofdll : u64 , enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK , usercontext : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    SymEnumTypes(hprocess.into_param().abi(), baseofdll, enumsymbolscallback, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymEnumTypesByName<P0, P1>(hprocess: P0, baseofdll: u64, mask: P1, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymEnumTypesByName ( hprocess : super::super::super::Foundation:: HANDLE , baseofdll : u64 , mask : ::windows::core::PCSTR , enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK , usercontext : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    SymEnumTypesByName(hprocess.into_param().abi(), baseofdll, mask.into_param().abi(), enumsymbolscallback, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymEnumTypesByNameW<P0, P1>(hprocess: P0, baseofdll: u64, mask: P1, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymEnumTypesByNameW ( hprocess : super::super::super::Foundation:: HANDLE , baseofdll : u64 , mask : ::windows::core::PCWSTR , enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACKW , usercontext : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    SymEnumTypesByNameW(hprocess.into_param().abi(), baseofdll, mask.into_param().abi(), enumsymbolscallback, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymEnumTypesW<P0>(hprocess: P0, baseofdll: u64, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymEnumTypesW ( hprocess : super::super::super::Foundation:: HANDLE , baseofdll : u64 , enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACKW , usercontext : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    SymEnumTypesW(hprocess.into_param().abi(), baseofdll, enumsymbolscallback, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymEnumerateModules<P0>(hprocess: P0, enummodulescallback: PSYM_ENUMMODULES_CALLBACK, usercontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymEnumerateModules ( hprocess : super::super::super::Foundation:: HANDLE , enummodulescallback : PSYM_ENUMMODULES_CALLBACK , usercontext : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    SymEnumerateModules(hprocess.into_param().abi(), enummodulescallback, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymEnumerateModules64<P0>(hprocess: P0, enummodulescallback: PSYM_ENUMMODULES_CALLBACK64, usercontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymEnumerateModules64 ( hprocess : super::super::super::Foundation:: HANDLE , enummodulescallback : PSYM_ENUMMODULES_CALLBACK64 , usercontext : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    SymEnumerateModules64(hprocess.into_param().abi(), enummodulescallback, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymEnumerateModulesW64<P0>(hprocess: P0, enummodulescallback: PSYM_ENUMMODULES_CALLBACKW64, usercontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymEnumerateModulesW64 ( hprocess : super::super::super::Foundation:: HANDLE , enummodulescallback : PSYM_ENUMMODULES_CALLBACKW64 , usercontext : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    SymEnumerateModulesW64(hprocess.into_param().abi(), enummodulescallback, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymEnumerateSymbols<P0>(hprocess: P0, baseofdll: u32, enumsymbolscallback: PSYM_ENUMSYMBOLS_CALLBACK, usercontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymEnumerateSymbols ( hprocess : super::super::super::Foundation:: HANDLE , baseofdll : u32 , enumsymbolscallback : PSYM_ENUMSYMBOLS_CALLBACK , usercontext : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    SymEnumerateSymbols(hprocess.into_param().abi(), baseofdll, enumsymbolscallback, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymEnumerateSymbols64<P0>(hprocess: P0, baseofdll: u64, enumsymbolscallback: PSYM_ENUMSYMBOLS_CALLBACK64, usercontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymEnumerateSymbols64 ( hprocess : super::super::super::Foundation:: HANDLE , baseofdll : u64 , enumsymbolscallback : PSYM_ENUMSYMBOLS_CALLBACK64 , usercontext : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    SymEnumerateSymbols64(hprocess.into_param().abi(), baseofdll, enumsymbolscallback, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymEnumerateSymbolsW<P0>(hprocess: P0, baseofdll: u32, enumsymbolscallback: PSYM_ENUMSYMBOLS_CALLBACKW, usercontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymEnumerateSymbolsW ( hprocess : super::super::super::Foundation:: HANDLE , baseofdll : u32 , enumsymbolscallback : PSYM_ENUMSYMBOLS_CALLBACKW , usercontext : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    SymEnumerateSymbolsW(hprocess.into_param().abi(), baseofdll, enumsymbolscallback, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymEnumerateSymbolsW64<P0>(hprocess: P0, baseofdll: u64, enumsymbolscallback: PSYM_ENUMSYMBOLS_CALLBACK64W, usercontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymEnumerateSymbolsW64 ( hprocess : super::super::super::Foundation:: HANDLE , baseofdll : u64 , enumsymbolscallback : PSYM_ENUMSYMBOLS_CALLBACK64W , usercontext : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    SymEnumerateSymbolsW64(hprocess.into_param().abi(), baseofdll, enumsymbolscallback, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymFindDebugInfoFile<P0, P1>(hprocess: P0, filename: P1, debugfilepath: ::windows::core::PSTR, callback: PFIND_DEBUG_FILE_CALLBACK, callerdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<super::super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymFindDebugInfoFile ( hprocess : super::super::super::Foundation:: HANDLE , filename : ::windows::core::PCSTR , debugfilepath : ::windows::core::PSTR , callback : PFIND_DEBUG_FILE_CALLBACK , callerdata : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: HANDLE );
    let result__ = SymFindDebugInfoFile(hprocess.into_param().abi(), filename.into_param().abi(), ::core::mem::transmute(debugfilepath), callback, ::core::mem::transmute(callerdata.unwrap_or(::std::ptr::null())));
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymFindDebugInfoFileW<P0, P1>(hprocess: P0, filename: P1, debugfilepath: ::windows::core::PWSTR, callback: PFIND_DEBUG_FILE_CALLBACKW, callerdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<super::super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymFindDebugInfoFileW ( hprocess : super::super::super::Foundation:: HANDLE , filename : ::windows::core::PCWSTR , debugfilepath : ::windows::core::PWSTR , callback : PFIND_DEBUG_FILE_CALLBACKW , callerdata : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: HANDLE );
    let result__ = SymFindDebugInfoFileW(hprocess.into_param().abi(), filename.into_param().abi(), ::core::mem::transmute(debugfilepath), callback, ::core::mem::transmute(callerdata.unwrap_or(::std::ptr::null())));
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymFindExecutableImage<P0, P1>(hprocess: P0, filename: P1, imagefilepath: ::windows::core::PSTR, callback: PFIND_EXE_FILE_CALLBACK, callerdata: *const ::core::ffi::c_void) -> ::windows::core::Result<super::super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymFindExecutableImage ( hprocess : super::super::super::Foundation:: HANDLE , filename : ::windows::core::PCSTR , imagefilepath : ::windows::core::PSTR , callback : PFIND_EXE_FILE_CALLBACK , callerdata : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: HANDLE );
    let result__ = SymFindExecutableImage(hprocess.into_param().abi(), filename.into_param().abi(), ::core::mem::transmute(imagefilepath), callback, callerdata);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymFindExecutableImageW<P0, P1>(hprocess: P0, filename: P1, imagefilepath: ::windows::core::PWSTR, callback: PFIND_EXE_FILE_CALLBACKW, callerdata: *const ::core::ffi::c_void) -> ::windows::core::Result<super::super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymFindExecutableImageW ( hprocess : super::super::super::Foundation:: HANDLE , filename : ::windows::core::PCWSTR , imagefilepath : ::windows::core::PWSTR , callback : PFIND_EXE_FILE_CALLBACKW , callerdata : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: HANDLE );
    let result__ = SymFindExecutableImageW(hprocess.into_param().abi(), filename.into_param().abi(), ::core::mem::transmute(imagefilepath), callback, callerdata);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymFindFileInPath<P0, P1, P2>(hprocess: P0, searchpatha: P1, filename: P2, id: ::core::option::Option<*const ::core::ffi::c_void>, two: u32, three: u32, flags: SYM_FIND_ID_OPTION, foundfile: ::windows::core::PSTR, callback: PFINDFILEINPATHCALLBACK, context: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymFindFileInPath ( hprocess : super::super::super::Foundation:: HANDLE , searchpatha : ::windows::core::PCSTR , filename : ::windows::core::PCSTR , id : *const ::core::ffi::c_void , two : u32 , three : u32 , flags : SYM_FIND_ID_OPTION , foundfile : ::windows::core::PSTR , callback : PFINDFILEINPATHCALLBACK , context : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    SymFindFileInPath(hprocess.into_param().abi(), searchpatha.into_param().abi(), filename.into_param().abi(), ::core::mem::transmute(id.unwrap_or(::std::ptr::null())), two, three, flags, ::core::mem::transmute(foundfile), callback, ::core::mem::transmute(context.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymFindFileInPathW<P0, P1, P2>(hprocess: P0, searchpatha: P1, filename: P2, id: ::core::option::Option<*const ::core::ffi::c_void>, two: u32, three: u32, flags: SYM_FIND_ID_OPTION, foundfile: ::windows::core::PWSTR, callback: PFINDFILEINPATHCALLBACKW, context: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymFindFileInPathW ( hprocess : super::super::super::Foundation:: HANDLE , searchpatha : ::windows::core::PCWSTR , filename : ::windows::core::PCWSTR , id : *const ::core::ffi::c_void , two : u32 , three : u32 , flags : SYM_FIND_ID_OPTION , foundfile : ::windows::core::PWSTR , callback : PFINDFILEINPATHCALLBACKW , context : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    SymFindFileInPathW(hprocess.into_param().abi(), searchpatha.into_param().abi(), filename.into_param().abi(), ::core::mem::transmute(id.unwrap_or(::std::ptr::null())), two, three, flags, ::core::mem::transmute(foundfile), callback, ::core::mem::transmute(context.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymFromAddr<P0>(hprocess: P0, address: u64, displacement: ::core::option::Option<*mut u64>, symbol: *mut SYMBOL_INFO) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymFromAddr ( hprocess : super::super::super::Foundation:: HANDLE , address : u64 , displacement : *mut u64 , symbol : *mut SYMBOL_INFO ) -> super::super::super::Foundation:: BOOL );
    SymFromAddr(hprocess.into_param().abi(), address, ::core::mem::transmute(displacement.unwrap_or(::std::ptr::null_mut())), symbol)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymFromAddrW<P0>(hprocess: P0, address: u64, displacement: ::core::option::Option<*mut u64>, symbol: *mut SYMBOL_INFOW) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymFromAddrW ( hprocess : super::super::super::Foundation:: HANDLE , address : u64 , displacement : *mut u64 , symbol : *mut SYMBOL_INFOW ) -> super::super::super::Foundation:: BOOL );
    SymFromAddrW(hprocess.into_param().abi(), address, ::core::mem::transmute(displacement.unwrap_or(::std::ptr::null_mut())), symbol)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymFromIndex<P0>(hprocess: P0, baseofdll: u64, index: u32, symbol: *mut SYMBOL_INFO) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymFromIndex ( hprocess : super::super::super::Foundation:: HANDLE , baseofdll : u64 , index : u32 , symbol : *mut SYMBOL_INFO ) -> super::super::super::Foundation:: BOOL );
    SymFromIndex(hprocess.into_param().abi(), baseofdll, index, symbol)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymFromIndexW<P0>(hprocess: P0, baseofdll: u64, index: u32, symbol: *mut SYMBOL_INFOW) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymFromIndexW ( hprocess : super::super::super::Foundation:: HANDLE , baseofdll : u64 , index : u32 , symbol : *mut SYMBOL_INFOW ) -> super::super::super::Foundation:: BOOL );
    SymFromIndexW(hprocess.into_param().abi(), baseofdll, index, symbol)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymFromInlineContext<P0>(hprocess: P0, address: u64, inlinecontext: u32, displacement: ::core::option::Option<*mut u64>, symbol: *mut SYMBOL_INFO) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymFromInlineContext ( hprocess : super::super::super::Foundation:: HANDLE , address : u64 , inlinecontext : u32 , displacement : *mut u64 , symbol : *mut SYMBOL_INFO ) -> super::super::super::Foundation:: BOOL );
    SymFromInlineContext(hprocess.into_param().abi(), address, inlinecontext, ::core::mem::transmute(displacement.unwrap_or(::std::ptr::null_mut())), symbol)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymFromInlineContextW<P0>(hprocess: P0, address: u64, inlinecontext: u32, displacement: ::core::option::Option<*mut u64>, symbol: *mut SYMBOL_INFOW) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymFromInlineContextW ( hprocess : super::super::super::Foundation:: HANDLE , address : u64 , inlinecontext : u32 , displacement : *mut u64 , symbol : *mut SYMBOL_INFOW ) -> super::super::super::Foundation:: BOOL );
    SymFromInlineContextW(hprocess.into_param().abi(), address, inlinecontext, ::core::mem::transmute(displacement.unwrap_or(::std::ptr::null_mut())), symbol)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymFromName<P0, P1>(hprocess: P0, name: P1, symbol: *mut SYMBOL_INFO) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymFromName ( hprocess : super::super::super::Foundation:: HANDLE , name : ::windows::core::PCSTR , symbol : *mut SYMBOL_INFO ) -> super::super::super::Foundation:: BOOL );
    SymFromName(hprocess.into_param().abi(), name.into_param().abi(), symbol)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymFromNameW<P0, P1>(hprocess: P0, name: P1, symbol: *mut SYMBOL_INFOW) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymFromNameW ( hprocess : super::super::super::Foundation:: HANDLE , name : ::windows::core::PCWSTR , symbol : *mut SYMBOL_INFOW ) -> super::super::super::Foundation:: BOOL );
    SymFromNameW(hprocess.into_param().abi(), name.into_param().abi(), symbol)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymFromToken<P0>(hprocess: P0, base: u64, token: u32, symbol: *mut SYMBOL_INFO) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymFromToken ( hprocess : super::super::super::Foundation:: HANDLE , base : u64 , token : u32 , symbol : *mut SYMBOL_INFO ) -> super::super::super::Foundation:: BOOL );
    SymFromToken(hprocess.into_param().abi(), base, token, symbol)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymFromTokenW<P0>(hprocess: P0, base: u64, token: u32, symbol: *mut SYMBOL_INFOW) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymFromTokenW ( hprocess : super::super::super::Foundation:: HANDLE , base : u64 , token : u32 , symbol : *mut SYMBOL_INFOW ) -> super::super::super::Foundation:: BOOL );
    SymFromTokenW(hprocess.into_param().abi(), base, token, symbol)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymFunctionTableAccess<P0>(hprocess: P0, addrbase: u32) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymFunctionTableAccess ( hprocess : super::super::super::Foundation:: HANDLE , addrbase : u32 ) -> *mut ::core::ffi::c_void );
    SymFunctionTableAccess(hprocess.into_param().abi(), addrbase)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymFunctionTableAccess64<P0>(hprocess: P0, addrbase: u64) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymFunctionTableAccess64 ( hprocess : super::super::super::Foundation:: HANDLE , addrbase : u64 ) -> *mut ::core::ffi::c_void );
    SymFunctionTableAccess64(hprocess.into_param().abi(), addrbase)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymFunctionTableAccess64AccessRoutines<P0>(hprocess: P0, addrbase: u64, readmemoryroutine: PREAD_PROCESS_MEMORY_ROUTINE64, getmodulebaseroutine: PGET_MODULE_BASE_ROUTINE64) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymFunctionTableAccess64AccessRoutines ( hprocess : super::super::super::Foundation:: HANDLE , addrbase : u64 , readmemoryroutine : PREAD_PROCESS_MEMORY_ROUTINE64 , getmodulebaseroutine : PGET_MODULE_BASE_ROUTINE64 ) -> *mut ::core::ffi::c_void );
    SymFunctionTableAccess64AccessRoutines(hprocess.into_param().abi(), addrbase, readmemoryroutine, getmodulebaseroutine)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetExtendedOption(option: IMAGEHLP_EXTENDED_OPTIONS) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetExtendedOption ( option : IMAGEHLP_EXTENDED_OPTIONS ) -> super::super::super::Foundation:: BOOL );
    SymGetExtendedOption(option)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetFileLineOffsets64<P0, P1, P2>(hprocess: P0, modulename: P1, filename: P2, buffer: &mut [u64]) -> u32
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetFileLineOffsets64 ( hprocess : super::super::super::Foundation:: HANDLE , modulename : ::windows::core::PCSTR , filename : ::windows::core::PCSTR , buffer : *mut u64 , bufferlines : u32 ) -> u32 );
    SymGetFileLineOffsets64(hprocess.into_param().abi(), modulename.into_param().abi(), filename.into_param().abi(), ::core::mem::transmute(buffer.as_ptr()), buffer.len() as _)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn SymGetHomeDirectory(r#type: IMAGEHLP_HD_TYPE, dir: &mut [u8]) -> ::windows::core::PSTR {
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetHomeDirectory ( r#type : IMAGEHLP_HD_TYPE , dir : ::windows::core::PSTR , size : usize ) -> ::windows::core::PSTR );
    SymGetHomeDirectory(r#type, ::core::mem::transmute(dir.as_ptr()), dir.len() as _)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn SymGetHomeDirectoryW(r#type: IMAGEHLP_HD_TYPE, dir: &mut [u16]) -> ::windows::core::PWSTR {
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetHomeDirectoryW ( r#type : IMAGEHLP_HD_TYPE , dir : ::windows::core::PWSTR , size : usize ) -> ::windows::core::PWSTR );
    SymGetHomeDirectoryW(r#type, ::core::mem::transmute(dir.as_ptr()), dir.len() as _)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetLineFromAddr<P0>(hprocess: P0, dwaddr: u32, pdwdisplacement: *mut u32, line: *mut IMAGEHLP_LINE) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetLineFromAddr ( hprocess : super::super::super::Foundation:: HANDLE , dwaddr : u32 , pdwdisplacement : *mut u32 , line : *mut IMAGEHLP_LINE ) -> super::super::super::Foundation:: BOOL );
    SymGetLineFromAddr(hprocess.into_param().abi(), dwaddr, pdwdisplacement, line)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetLineFromAddr64<P0>(hprocess: P0, qwaddr: u64, pdwdisplacement: *mut u32, line64: *mut IMAGEHLP_LINE64) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetLineFromAddr64 ( hprocess : super::super::super::Foundation:: HANDLE , qwaddr : u64 , pdwdisplacement : *mut u32 , line64 : *mut IMAGEHLP_LINE64 ) -> super::super::super::Foundation:: BOOL );
    SymGetLineFromAddr64(hprocess.into_param().abi(), qwaddr, pdwdisplacement, line64)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetLineFromAddrW64<P0>(hprocess: P0, dwaddr: u64, pdwdisplacement: *mut u32, line: *mut IMAGEHLP_LINEW64) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetLineFromAddrW64 ( hprocess : super::super::super::Foundation:: HANDLE , dwaddr : u64 , pdwdisplacement : *mut u32 , line : *mut IMAGEHLP_LINEW64 ) -> super::super::super::Foundation:: BOOL );
    SymGetLineFromAddrW64(hprocess.into_param().abi(), dwaddr, pdwdisplacement, line)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetLineFromInlineContext<P0>(hprocess: P0, qwaddr: u64, inlinecontext: u32, qwmodulebaseaddress: u64, pdwdisplacement: *mut u32, line64: *mut IMAGEHLP_LINE64) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetLineFromInlineContext ( hprocess : super::super::super::Foundation:: HANDLE , qwaddr : u64 , inlinecontext : u32 , qwmodulebaseaddress : u64 , pdwdisplacement : *mut u32 , line64 : *mut IMAGEHLP_LINE64 ) -> super::super::super::Foundation:: BOOL );
    SymGetLineFromInlineContext(hprocess.into_param().abi(), qwaddr, inlinecontext, qwmodulebaseaddress, pdwdisplacement, line64)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetLineFromInlineContextW<P0>(hprocess: P0, dwaddr: u64, inlinecontext: u32, qwmodulebaseaddress: u64, pdwdisplacement: *mut u32, line: *mut IMAGEHLP_LINEW64) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetLineFromInlineContextW ( hprocess : super::super::super::Foundation:: HANDLE , dwaddr : u64 , inlinecontext : u32 , qwmodulebaseaddress : u64 , pdwdisplacement : *mut u32 , line : *mut IMAGEHLP_LINEW64 ) -> super::super::super::Foundation:: BOOL );
    SymGetLineFromInlineContextW(hprocess.into_param().abi(), dwaddr, inlinecontext, qwmodulebaseaddress, pdwdisplacement, line)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetLineFromName<P0, P1, P2>(hprocess: P0, modulename: P1, filename: P2, dwlinenumber: u32, pldisplacement: *mut i32, line: *mut IMAGEHLP_LINE) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetLineFromName ( hprocess : super::super::super::Foundation:: HANDLE , modulename : ::windows::core::PCSTR , filename : ::windows::core::PCSTR , dwlinenumber : u32 , pldisplacement : *mut i32 , line : *mut IMAGEHLP_LINE ) -> super::super::super::Foundation:: BOOL );
    SymGetLineFromName(hprocess.into_param().abi(), modulename.into_param().abi(), filename.into_param().abi(), dwlinenumber, pldisplacement, line)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetLineFromName64<P0, P1, P2>(hprocess: P0, modulename: P1, filename: P2, dwlinenumber: u32, pldisplacement: *mut i32, line: *mut IMAGEHLP_LINE64) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetLineFromName64 ( hprocess : super::super::super::Foundation:: HANDLE , modulename : ::windows::core::PCSTR , filename : ::windows::core::PCSTR , dwlinenumber : u32 , pldisplacement : *mut i32 , line : *mut IMAGEHLP_LINE64 ) -> super::super::super::Foundation:: BOOL );
    SymGetLineFromName64(hprocess.into_param().abi(), modulename.into_param().abi(), filename.into_param().abi(), dwlinenumber, pldisplacement, line)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetLineFromNameW64<P0, P1, P2>(hprocess: P0, modulename: P1, filename: P2, dwlinenumber: u32, pldisplacement: *mut i32, line: *mut IMAGEHLP_LINEW64) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetLineFromNameW64 ( hprocess : super::super::super::Foundation:: HANDLE , modulename : ::windows::core::PCWSTR , filename : ::windows::core::PCWSTR , dwlinenumber : u32 , pldisplacement : *mut i32 , line : *mut IMAGEHLP_LINEW64 ) -> super::super::super::Foundation:: BOOL );
    SymGetLineFromNameW64(hprocess.into_param().abi(), modulename.into_param().abi(), filename.into_param().abi(), dwlinenumber, pldisplacement, line)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetLineNext<P0>(hprocess: P0, line: *mut IMAGEHLP_LINE) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetLineNext ( hprocess : super::super::super::Foundation:: HANDLE , line : *mut IMAGEHLP_LINE ) -> super::super::super::Foundation:: BOOL );
    SymGetLineNext(hprocess.into_param().abi(), line)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetLineNext64<P0>(hprocess: P0, line: *mut IMAGEHLP_LINE64) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetLineNext64 ( hprocess : super::super::super::Foundation:: HANDLE , line : *mut IMAGEHLP_LINE64 ) -> super::super::super::Foundation:: BOOL );
    SymGetLineNext64(hprocess.into_param().abi(), line)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetLineNextW64<P0>(hprocess: P0, line: *mut IMAGEHLP_LINEW64) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetLineNextW64 ( hprocess : super::super::super::Foundation:: HANDLE , line : *mut IMAGEHLP_LINEW64 ) -> super::super::super::Foundation:: BOOL );
    SymGetLineNextW64(hprocess.into_param().abi(), line)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetLinePrev<P0>(hprocess: P0, line: *mut IMAGEHLP_LINE) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetLinePrev ( hprocess : super::super::super::Foundation:: HANDLE , line : *mut IMAGEHLP_LINE ) -> super::super::super::Foundation:: BOOL );
    SymGetLinePrev(hprocess.into_param().abi(), line)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetLinePrev64<P0>(hprocess: P0, line: *mut IMAGEHLP_LINE64) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetLinePrev64 ( hprocess : super::super::super::Foundation:: HANDLE , line : *mut IMAGEHLP_LINE64 ) -> super::super::super::Foundation:: BOOL );
    SymGetLinePrev64(hprocess.into_param().abi(), line)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetLinePrevW64<P0>(hprocess: P0, line: *mut IMAGEHLP_LINEW64) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetLinePrevW64 ( hprocess : super::super::super::Foundation:: HANDLE , line : *mut IMAGEHLP_LINEW64 ) -> super::super::super::Foundation:: BOOL );
    SymGetLinePrevW64(hprocess.into_param().abi(), line)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetModuleBase<P0>(hprocess: P0, dwaddr: u32) -> u32
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetModuleBase ( hprocess : super::super::super::Foundation:: HANDLE , dwaddr : u32 ) -> u32 );
    SymGetModuleBase(hprocess.into_param().abi(), dwaddr)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetModuleBase64<P0>(hprocess: P0, qwaddr: u64) -> u64
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetModuleBase64 ( hprocess : super::super::super::Foundation:: HANDLE , qwaddr : u64 ) -> u64 );
    SymGetModuleBase64(hprocess.into_param().abi(), qwaddr)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetModuleInfo<P0>(hprocess: P0, dwaddr: u32, moduleinfo: *mut IMAGEHLP_MODULE) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetModuleInfo ( hprocess : super::super::super::Foundation:: HANDLE , dwaddr : u32 , moduleinfo : *mut IMAGEHLP_MODULE ) -> super::super::super::Foundation:: BOOL );
    SymGetModuleInfo(hprocess.into_param().abi(), dwaddr, moduleinfo)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetModuleInfo64<P0>(hprocess: P0, qwaddr: u64, moduleinfo: *mut IMAGEHLP_MODULE64) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetModuleInfo64 ( hprocess : super::super::super::Foundation:: HANDLE , qwaddr : u64 , moduleinfo : *mut IMAGEHLP_MODULE64 ) -> super::super::super::Foundation:: BOOL );
    SymGetModuleInfo64(hprocess.into_param().abi(), qwaddr, moduleinfo)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetModuleInfoW<P0>(hprocess: P0, dwaddr: u32, moduleinfo: *mut IMAGEHLP_MODULEW) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetModuleInfoW ( hprocess : super::super::super::Foundation:: HANDLE , dwaddr : u32 , moduleinfo : *mut IMAGEHLP_MODULEW ) -> super::super::super::Foundation:: BOOL );
    SymGetModuleInfoW(hprocess.into_param().abi(), dwaddr, moduleinfo)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetModuleInfoW64<P0>(hprocess: P0, qwaddr: u64, moduleinfo: *mut IMAGEHLP_MODULEW64) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetModuleInfoW64 ( hprocess : super::super::super::Foundation:: HANDLE , qwaddr : u64 , moduleinfo : *mut IMAGEHLP_MODULEW64 ) -> super::super::super::Foundation:: BOOL );
    SymGetModuleInfoW64(hprocess.into_param().abi(), qwaddr, moduleinfo)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetOmaps<P0>(hprocess: P0, baseofdll: u64, omapto: *mut *mut OMAP, comapto: *mut u64, omapfrom: *mut *mut OMAP, comapfrom: *mut u64) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetOmaps ( hprocess : super::super::super::Foundation:: HANDLE , baseofdll : u64 , omapto : *mut *mut OMAP , comapto : *mut u64 , omapfrom : *mut *mut OMAP , comapfrom : *mut u64 ) -> super::super::super::Foundation:: BOOL );
    SymGetOmaps(hprocess.into_param().abi(), baseofdll, omapto, comapto, omapfrom, comapfrom)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn SymGetOptions() -> u32 {
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetOptions ( ) -> u32 );
    SymGetOptions()
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetScope<P0>(hprocess: P0, baseofdll: u64, index: u32, symbol: *mut SYMBOL_INFO) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetScope ( hprocess : super::super::super::Foundation:: HANDLE , baseofdll : u64 , index : u32 , symbol : *mut SYMBOL_INFO ) -> super::super::super::Foundation:: BOOL );
    SymGetScope(hprocess.into_param().abi(), baseofdll, index, symbol)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetScopeW<P0>(hprocess: P0, baseofdll: u64, index: u32, symbol: *mut SYMBOL_INFOW) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetScopeW ( hprocess : super::super::super::Foundation:: HANDLE , baseofdll : u64 , index : u32 , symbol : *mut SYMBOL_INFOW ) -> super::super::super::Foundation:: BOOL );
    SymGetScopeW(hprocess.into_param().abi(), baseofdll, index, symbol)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetSearchPath<P0>(hprocess: P0, searchpatha: &mut [u8]) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetSearchPath ( hprocess : super::super::super::Foundation:: HANDLE , searchpatha : ::windows::core::PSTR , searchpathlength : u32 ) -> super::super::super::Foundation:: BOOL );
    SymGetSearchPath(hprocess.into_param().abi(), ::core::mem::transmute(searchpatha.as_ptr()), searchpatha.len() as _)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetSearchPathW<P0>(hprocess: P0, searchpatha: &mut [u16]) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetSearchPathW ( hprocess : super::super::super::Foundation:: HANDLE , searchpatha : ::windows::core::PWSTR , searchpathlength : u32 ) -> super::super::super::Foundation:: BOOL );
    SymGetSearchPathW(hprocess.into_param().abi(), ::core::mem::transmute(searchpatha.as_ptr()), searchpatha.len() as _)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetSourceFile<P0, P1, P2>(hprocess: P0, base: u64, params: P1, filespec: P2, filepath: &mut [u8]) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetSourceFile ( hprocess : super::super::super::Foundation:: HANDLE , base : u64 , params : ::windows::core::PCSTR , filespec : ::windows::core::PCSTR , filepath : ::windows::core::PSTR , size : u32 ) -> super::super::super::Foundation:: BOOL );
    SymGetSourceFile(hprocess.into_param().abi(), base, params.into_param().abi(), filespec.into_param().abi(), ::core::mem::transmute(filepath.as_ptr()), filepath.len() as _)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetSourceFileChecksum<P0, P1>(hprocess: P0, base: u64, filespec: P1, pchecksumtype: *mut u32, pchecksum: &mut [u8], pactualbyteswritten: *mut u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetSourceFileChecksum ( hprocess : super::super::super::Foundation:: HANDLE , base : u64 , filespec : ::windows::core::PCSTR , pchecksumtype : *mut u32 , pchecksum : *mut u8 , checksumsize : u32 , pactualbyteswritten : *mut u32 ) -> super::super::super::Foundation:: BOOL );
    SymGetSourceFileChecksum(hprocess.into_param().abi(), base, filespec.into_param().abi(), pchecksumtype, ::core::mem::transmute(pchecksum.as_ptr()), pchecksum.len() as _, pactualbyteswritten)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetSourceFileChecksumW<P0, P1>(hprocess: P0, base: u64, filespec: P1, pchecksumtype: *mut u32, pchecksum: &mut [u8], pactualbyteswritten: *mut u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetSourceFileChecksumW ( hprocess : super::super::super::Foundation:: HANDLE , base : u64 , filespec : ::windows::core::PCWSTR , pchecksumtype : *mut u32 , pchecksum : *mut u8 , checksumsize : u32 , pactualbyteswritten : *mut u32 ) -> super::super::super::Foundation:: BOOL );
    SymGetSourceFileChecksumW(hprocess.into_param().abi(), base, filespec.into_param().abi(), pchecksumtype, ::core::mem::transmute(pchecksum.as_ptr()), pchecksum.len() as _, pactualbyteswritten)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetSourceFileFromToken<P0, P1>(hprocess: P0, token: *const ::core::ffi::c_void, params: P1, filepath: &mut [u8]) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetSourceFileFromToken ( hprocess : super::super::super::Foundation:: HANDLE , token : *const ::core::ffi::c_void , params : ::windows::core::PCSTR , filepath : ::windows::core::PSTR , size : u32 ) -> super::super::super::Foundation:: BOOL );
    SymGetSourceFileFromToken(hprocess.into_param().abi(), token, params.into_param().abi(), ::core::mem::transmute(filepath.as_ptr()), filepath.len() as _)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetSourceFileFromTokenByTokenName<P0, P1, P2>(hprocess: P0, token: *const ::core::ffi::c_void, tokenname: P1, params: P2, filepath: &mut [u8]) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetSourceFileFromTokenByTokenName ( hprocess : super::super::super::Foundation:: HANDLE , token : *const ::core::ffi::c_void , tokenname : ::windows::core::PCSTR , params : ::windows::core::PCSTR , filepath : ::windows::core::PSTR , size : u32 ) -> super::super::super::Foundation:: BOOL );
    SymGetSourceFileFromTokenByTokenName(hprocess.into_param().abi(), token, tokenname.into_param().abi(), params.into_param().abi(), ::core::mem::transmute(filepath.as_ptr()), filepath.len() as _)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetSourceFileFromTokenByTokenNameW<P0, P1, P2>(hprocess: P0, token: *const ::core::ffi::c_void, tokenname: P1, params: P2, filepath: &mut [u16]) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetSourceFileFromTokenByTokenNameW ( hprocess : super::super::super::Foundation:: HANDLE , token : *const ::core::ffi::c_void , tokenname : ::windows::core::PCWSTR , params : ::windows::core::PCWSTR , filepath : ::windows::core::PWSTR , size : u32 ) -> super::super::super::Foundation:: BOOL );
    SymGetSourceFileFromTokenByTokenNameW(hprocess.into_param().abi(), token, tokenname.into_param().abi(), params.into_param().abi(), ::core::mem::transmute(filepath.as_ptr()), filepath.len() as _)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetSourceFileFromTokenW<P0, P1>(hprocess: P0, token: *const ::core::ffi::c_void, params: P1, filepath: &mut [u16]) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetSourceFileFromTokenW ( hprocess : super::super::super::Foundation:: HANDLE , token : *const ::core::ffi::c_void , params : ::windows::core::PCWSTR , filepath : ::windows::core::PWSTR , size : u32 ) -> super::super::super::Foundation:: BOOL );
    SymGetSourceFileFromTokenW(hprocess.into_param().abi(), token, params.into_param().abi(), ::core::mem::transmute(filepath.as_ptr()), filepath.len() as _)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetSourceFileToken<P0, P1>(hprocess: P0, base: u64, filespec: P1, token: *mut *mut ::core::ffi::c_void, size: *mut u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetSourceFileToken ( hprocess : super::super::super::Foundation:: HANDLE , base : u64 , filespec : ::windows::core::PCSTR , token : *mut *mut ::core::ffi::c_void , size : *mut u32 ) -> super::super::super::Foundation:: BOOL );
    SymGetSourceFileToken(hprocess.into_param().abi(), base, filespec.into_param().abi(), token, size)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetSourceFileTokenByTokenName<P0, P1, P2, P3>(hprocess: P0, base: u64, filespec: P1, tokenname: P2, tokenparameters: P3, token: *mut *mut ::core::ffi::c_void, size: *mut u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetSourceFileTokenByTokenName ( hprocess : super::super::super::Foundation:: HANDLE , base : u64 , filespec : ::windows::core::PCSTR , tokenname : ::windows::core::PCSTR , tokenparameters : ::windows::core::PCSTR , token : *mut *mut ::core::ffi::c_void , size : *mut u32 ) -> super::super::super::Foundation:: BOOL );
    SymGetSourceFileTokenByTokenName(hprocess.into_param().abi(), base, filespec.into_param().abi(), tokenname.into_param().abi(), tokenparameters.into_param().abi(), token, size)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetSourceFileTokenByTokenNameW<P0, P1, P2, P3>(hprocess: P0, base: u64, filespec: P1, tokenname: P2, tokenparameters: P3, token: *mut *mut ::core::ffi::c_void, size: *mut u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetSourceFileTokenByTokenNameW ( hprocess : super::super::super::Foundation:: HANDLE , base : u64 , filespec : ::windows::core::PCWSTR , tokenname : ::windows::core::PCWSTR , tokenparameters : ::windows::core::PCWSTR , token : *mut *mut ::core::ffi::c_void , size : *mut u32 ) -> super::super::super::Foundation:: BOOL );
    SymGetSourceFileTokenByTokenNameW(hprocess.into_param().abi(), base, filespec.into_param().abi(), tokenname.into_param().abi(), tokenparameters.into_param().abi(), token, size)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetSourceFileTokenW<P0, P1>(hprocess: P0, base: u64, filespec: P1, token: *mut *mut ::core::ffi::c_void, size: *mut u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetSourceFileTokenW ( hprocess : super::super::super::Foundation:: HANDLE , base : u64 , filespec : ::windows::core::PCWSTR , token : *mut *mut ::core::ffi::c_void , size : *mut u32 ) -> super::super::super::Foundation:: BOOL );
    SymGetSourceFileTokenW(hprocess.into_param().abi(), base, filespec.into_param().abi(), token, size)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetSourceFileW<P0, P1, P2>(hprocess: P0, base: u64, params: P1, filespec: P2, filepath: &mut [u16]) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetSourceFileW ( hprocess : super::super::super::Foundation:: HANDLE , base : u64 , params : ::windows::core::PCWSTR , filespec : ::windows::core::PCWSTR , filepath : ::windows::core::PWSTR , size : u32 ) -> super::super::super::Foundation:: BOOL );
    SymGetSourceFileW(hprocess.into_param().abi(), base, params.into_param().abi(), filespec.into_param().abi(), ::core::mem::transmute(filepath.as_ptr()), filepath.len() as _)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetSourceVarFromToken<P0, P1, P2>(hprocess: P0, token: *const ::core::ffi::c_void, params: P1, varname: P2, value: &mut [u8]) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetSourceVarFromToken ( hprocess : super::super::super::Foundation:: HANDLE , token : *const ::core::ffi::c_void , params : ::windows::core::PCSTR , varname : ::windows::core::PCSTR , value : ::windows::core::PSTR , size : u32 ) -> super::super::super::Foundation:: BOOL );
    SymGetSourceVarFromToken(hprocess.into_param().abi(), token, params.into_param().abi(), varname.into_param().abi(), ::core::mem::transmute(value.as_ptr()), value.len() as _)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetSourceVarFromTokenW<P0, P1, P2>(hprocess: P0, token: *const ::core::ffi::c_void, params: P1, varname: P2, value: &mut [u16]) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetSourceVarFromTokenW ( hprocess : super::super::super::Foundation:: HANDLE , token : *const ::core::ffi::c_void , params : ::windows::core::PCWSTR , varname : ::windows::core::PCWSTR , value : ::windows::core::PWSTR , size : u32 ) -> super::super::super::Foundation:: BOOL );
    SymGetSourceVarFromTokenW(hprocess.into_param().abi(), token, params.into_param().abi(), varname.into_param().abi(), ::core::mem::transmute(value.as_ptr()), value.len() as _)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetSymFromAddr<P0>(hprocess: P0, dwaddr: u32, pdwdisplacement: ::core::option::Option<*mut u32>, symbol: *mut IMAGEHLP_SYMBOL) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetSymFromAddr ( hprocess : super::super::super::Foundation:: HANDLE , dwaddr : u32 , pdwdisplacement : *mut u32 , symbol : *mut IMAGEHLP_SYMBOL ) -> super::super::super::Foundation:: BOOL );
    SymGetSymFromAddr(hprocess.into_param().abi(), dwaddr, ::core::mem::transmute(pdwdisplacement.unwrap_or(::std::ptr::null_mut())), symbol)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetSymFromAddr64<P0>(hprocess: P0, qwaddr: u64, pdwdisplacement: ::core::option::Option<*mut u64>, symbol: *mut IMAGEHLP_SYMBOL64) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetSymFromAddr64 ( hprocess : super::super::super::Foundation:: HANDLE , qwaddr : u64 , pdwdisplacement : *mut u64 , symbol : *mut IMAGEHLP_SYMBOL64 ) -> super::super::super::Foundation:: BOOL );
    SymGetSymFromAddr64(hprocess.into_param().abi(), qwaddr, ::core::mem::transmute(pdwdisplacement.unwrap_or(::std::ptr::null_mut())), symbol)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetSymFromName<P0, P1>(hprocess: P0, name: P1, symbol: *mut IMAGEHLP_SYMBOL) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetSymFromName ( hprocess : super::super::super::Foundation:: HANDLE , name : ::windows::core::PCSTR , symbol : *mut IMAGEHLP_SYMBOL ) -> super::super::super::Foundation:: BOOL );
    SymGetSymFromName(hprocess.into_param().abi(), name.into_param().abi(), symbol)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetSymFromName64<P0, P1>(hprocess: P0, name: P1, symbol: *mut IMAGEHLP_SYMBOL64) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetSymFromName64 ( hprocess : super::super::super::Foundation:: HANDLE , name : ::windows::core::PCSTR , symbol : *mut IMAGEHLP_SYMBOL64 ) -> super::super::super::Foundation:: BOOL );
    SymGetSymFromName64(hprocess.into_param().abi(), name.into_param().abi(), symbol)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetSymNext<P0>(hprocess: P0, symbol: *mut IMAGEHLP_SYMBOL) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetSymNext ( hprocess : super::super::super::Foundation:: HANDLE , symbol : *mut IMAGEHLP_SYMBOL ) -> super::super::super::Foundation:: BOOL );
    SymGetSymNext(hprocess.into_param().abi(), symbol)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetSymNext64<P0>(hprocess: P0, symbol: *mut IMAGEHLP_SYMBOL64) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetSymNext64 ( hprocess : super::super::super::Foundation:: HANDLE , symbol : *mut IMAGEHLP_SYMBOL64 ) -> super::super::super::Foundation:: BOOL );
    SymGetSymNext64(hprocess.into_param().abi(), symbol)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetSymPrev<P0>(hprocess: P0, symbol: *mut IMAGEHLP_SYMBOL) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetSymPrev ( hprocess : super::super::super::Foundation:: HANDLE , symbol : *mut IMAGEHLP_SYMBOL ) -> super::super::super::Foundation:: BOOL );
    SymGetSymPrev(hprocess.into_param().abi(), symbol)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetSymPrev64<P0>(hprocess: P0, symbol: *mut IMAGEHLP_SYMBOL64) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetSymPrev64 ( hprocess : super::super::super::Foundation:: HANDLE , symbol : *mut IMAGEHLP_SYMBOL64 ) -> super::super::super::Foundation:: BOOL );
    SymGetSymPrev64(hprocess.into_param().abi(), symbol)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetSymbolFile<P0, P1, P2>(hprocess: P0, sympath: P1, imagefile: P2, r#type: IMAGEHLP_SF_TYPE, symbolfile: &mut [u8], dbgfile: &mut [u8]) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetSymbolFile ( hprocess : super::super::super::Foundation:: HANDLE , sympath : ::windows::core::PCSTR , imagefile : ::windows::core::PCSTR , r#type : IMAGEHLP_SF_TYPE , symbolfile : ::windows::core::PSTR , csymbolfile : usize , dbgfile : ::windows::core::PSTR , cdbgfile : usize ) -> super::super::super::Foundation:: BOOL );
    SymGetSymbolFile(hprocess.into_param().abi(), sympath.into_param().abi(), imagefile.into_param().abi(), r#type, ::core::mem::transmute(symbolfile.as_ptr()), symbolfile.len() as _, ::core::mem::transmute(dbgfile.as_ptr()), dbgfile.len() as _)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetSymbolFileW<P0, P1, P2>(hprocess: P0, sympath: P1, imagefile: P2, r#type: IMAGEHLP_SF_TYPE, symbolfile: &mut [u16], dbgfile: &mut [u16]) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetSymbolFileW ( hprocess : super::super::super::Foundation:: HANDLE , sympath : ::windows::core::PCWSTR , imagefile : ::windows::core::PCWSTR , r#type : IMAGEHLP_SF_TYPE , symbolfile : ::windows::core::PWSTR , csymbolfile : usize , dbgfile : ::windows::core::PWSTR , cdbgfile : usize ) -> super::super::super::Foundation:: BOOL );
    SymGetSymbolFileW(hprocess.into_param().abi(), sympath.into_param().abi(), imagefile.into_param().abi(), r#type, ::core::mem::transmute(symbolfile.as_ptr()), symbolfile.len() as _, ::core::mem::transmute(dbgfile.as_ptr()), dbgfile.len() as _)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetTypeFromName<P0, P1>(hprocess: P0, baseofdll: u64, name: P1, symbol: *mut SYMBOL_INFO) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetTypeFromName ( hprocess : super::super::super::Foundation:: HANDLE , baseofdll : u64 , name : ::windows::core::PCSTR , symbol : *mut SYMBOL_INFO ) -> super::super::super::Foundation:: BOOL );
    SymGetTypeFromName(hprocess.into_param().abi(), baseofdll, name.into_param().abi(), symbol)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetTypeFromNameW<P0, P1>(hprocess: P0, baseofdll: u64, name: P1, symbol: *mut SYMBOL_INFOW) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetTypeFromNameW ( hprocess : super::super::super::Foundation:: HANDLE , baseofdll : u64 , name : ::windows::core::PCWSTR , symbol : *mut SYMBOL_INFOW ) -> super::super::super::Foundation:: BOOL );
    SymGetTypeFromNameW(hprocess.into_param().abi(), baseofdll, name.into_param().abi(), symbol)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetTypeInfo<P0>(hprocess: P0, modbase: u64, typeid: u32, gettype: IMAGEHLP_SYMBOL_TYPE_INFO, pinfo: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetTypeInfo ( hprocess : super::super::super::Foundation:: HANDLE , modbase : u64 , typeid : u32 , gettype : IMAGEHLP_SYMBOL_TYPE_INFO , pinfo : *mut ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    SymGetTypeInfo(hprocess.into_param().abi(), modbase, typeid, gettype, pinfo)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetTypeInfoEx<P0>(hprocess: P0, modbase: u64, params: *mut IMAGEHLP_GET_TYPE_INFO_PARAMS) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetTypeInfoEx ( hprocess : super::super::super::Foundation:: HANDLE , modbase : u64 , params : *mut IMAGEHLP_GET_TYPE_INFO_PARAMS ) -> super::super::super::Foundation:: BOOL );
    SymGetTypeInfoEx(hprocess.into_param().abi(), modbase, params)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymGetUnwindInfo<P0>(hprocess: P0, address: u64, buffer: ::core::option::Option<*mut ::core::ffi::c_void>, size: *mut u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymGetUnwindInfo ( hprocess : super::super::super::Foundation:: HANDLE , address : u64 , buffer : *mut ::core::ffi::c_void , size : *mut u32 ) -> super::super::super::Foundation:: BOOL );
    SymGetUnwindInfo(hprocess.into_param().abi(), address, ::core::mem::transmute(buffer.unwrap_or(::std::ptr::null_mut())), size)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymInitialize<P0, P1, P2>(hprocess: P0, usersearchpath: P1, finvadeprocess: P2) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymInitialize ( hprocess : super::super::super::Foundation:: HANDLE , usersearchpath : ::windows::core::PCSTR , finvadeprocess : super::super::super::Foundation:: BOOL ) -> super::super::super::Foundation:: BOOL );
    SymInitialize(hprocess.into_param().abi(), usersearchpath.into_param().abi(), finvadeprocess.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymInitializeW<P0, P1, P2>(hprocess: P0, usersearchpath: P1, finvadeprocess: P2) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymInitializeW ( hprocess : super::super::super::Foundation:: HANDLE , usersearchpath : ::windows::core::PCWSTR , finvadeprocess : super::super::super::Foundation:: BOOL ) -> super::super::super::Foundation:: BOOL );
    SymInitializeW(hprocess.into_param().abi(), usersearchpath.into_param().abi(), finvadeprocess.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymLoadModule<P0, P1, P2, P3>(hprocess: P0, hfile: P1, imagename: P2, modulename: P3, baseofdll: u32, sizeofdll: u32) -> u32
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymLoadModule ( hprocess : super::super::super::Foundation:: HANDLE , hfile : super::super::super::Foundation:: HANDLE , imagename : ::windows::core::PCSTR , modulename : ::windows::core::PCSTR , baseofdll : u32 , sizeofdll : u32 ) -> u32 );
    SymLoadModule(hprocess.into_param().abi(), hfile.into_param().abi(), imagename.into_param().abi(), modulename.into_param().abi(), baseofdll, sizeofdll)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymLoadModule64<P0, P1, P2, P3>(hprocess: P0, hfile: P1, imagename: P2, modulename: P3, baseofdll: u64, sizeofdll: u32) -> u64
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymLoadModule64 ( hprocess : super::super::super::Foundation:: HANDLE , hfile : super::super::super::Foundation:: HANDLE , imagename : ::windows::core::PCSTR , modulename : ::windows::core::PCSTR , baseofdll : u64 , sizeofdll : u32 ) -> u64 );
    SymLoadModule64(hprocess.into_param().abi(), hfile.into_param().abi(), imagename.into_param().abi(), modulename.into_param().abi(), baseofdll, sizeofdll)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymLoadModuleEx<P0, P1, P2, P3>(hprocess: P0, hfile: P1, imagename: P2, modulename: P3, baseofdll: u64, dllsize: u32, data: ::core::option::Option<*const MODLOAD_DATA>, flags: SYM_LOAD_FLAGS) -> u64
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymLoadModuleEx ( hprocess : super::super::super::Foundation:: HANDLE , hfile : super::super::super::Foundation:: HANDLE , imagename : ::windows::core::PCSTR , modulename : ::windows::core::PCSTR , baseofdll : u64 , dllsize : u32 , data : *const MODLOAD_DATA , flags : SYM_LOAD_FLAGS ) -> u64 );
    SymLoadModuleEx(hprocess.into_param().abi(), hfile.into_param().abi(), imagename.into_param().abi(), modulename.into_param().abi(), baseofdll, dllsize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null())), flags)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymLoadModuleExW<P0, P1, P2, P3>(hprocess: P0, hfile: P1, imagename: P2, modulename: P3, baseofdll: u64, dllsize: u32, data: ::core::option::Option<*const MODLOAD_DATA>, flags: SYM_LOAD_FLAGS) -> u64
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymLoadModuleExW ( hprocess : super::super::super::Foundation:: HANDLE , hfile : super::super::super::Foundation:: HANDLE , imagename : ::windows::core::PCWSTR , modulename : ::windows::core::PCWSTR , baseofdll : u64 , dllsize : u32 , data : *const MODLOAD_DATA , flags : SYM_LOAD_FLAGS ) -> u64 );
    SymLoadModuleExW(hprocess.into_param().abi(), hfile.into_param().abi(), imagename.into_param().abi(), modulename.into_param().abi(), baseofdll, dllsize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null())), flags)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymMatchFileName<P0, P1>(filename: P0, r#match: P1, filenamestop: ::core::option::Option<*mut ::windows::core::PSTR>, matchstop: ::core::option::Option<*mut ::windows::core::PSTR>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymMatchFileName ( filename : ::windows::core::PCSTR , r#match : ::windows::core::PCSTR , filenamestop : *mut ::windows::core::PSTR , matchstop : *mut ::windows::core::PSTR ) -> super::super::super::Foundation:: BOOL );
    SymMatchFileName(filename.into_param().abi(), r#match.into_param().abi(), ::core::mem::transmute(filenamestop.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(matchstop.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymMatchFileNameW<P0, P1>(filename: P0, r#match: P1, filenamestop: ::core::option::Option<*mut ::windows::core::PWSTR>, matchstop: ::core::option::Option<*mut ::windows::core::PWSTR>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymMatchFileNameW ( filename : ::windows::core::PCWSTR , r#match : ::windows::core::PCWSTR , filenamestop : *mut ::windows::core::PWSTR , matchstop : *mut ::windows::core::PWSTR ) -> super::super::super::Foundation:: BOOL );
    SymMatchFileNameW(filename.into_param().abi(), r#match.into_param().abi(), ::core::mem::transmute(filenamestop.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(matchstop.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymMatchString<P0, P1, P2>(string: P0, expression: P1, fcase: P2) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymMatchString ( string : ::windows::core::PCSTR , expression : ::windows::core::PCSTR , fcase : super::super::super::Foundation:: BOOL ) -> super::super::super::Foundation:: BOOL );
    SymMatchString(string.into_param().abi(), expression.into_param().abi(), fcase.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymMatchStringA<P0, P1, P2>(string: P0, expression: P1, fcase: P2) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymMatchStringA ( string : ::windows::core::PCSTR , expression : ::windows::core::PCSTR , fcase : super::super::super::Foundation:: BOOL ) -> super::super::super::Foundation:: BOOL );
    SymMatchStringA(string.into_param().abi(), expression.into_param().abi(), fcase.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymMatchStringW<P0, P1, P2>(string: P0, expression: P1, fcase: P2) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymMatchStringW ( string : ::windows::core::PCWSTR , expression : ::windows::core::PCWSTR , fcase : super::super::super::Foundation:: BOOL ) -> super::super::super::Foundation:: BOOL );
    SymMatchStringW(string.into_param().abi(), expression.into_param().abi(), fcase.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymNext<P0>(hprocess: P0, si: *mut SYMBOL_INFO) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymNext ( hprocess : super::super::super::Foundation:: HANDLE , si : *mut SYMBOL_INFO ) -> super::super::super::Foundation:: BOOL );
    SymNext(hprocess.into_param().abi(), si)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymNextW<P0>(hprocess: P0, siw: *mut SYMBOL_INFOW) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymNextW ( hprocess : super::super::super::Foundation:: HANDLE , siw : *mut SYMBOL_INFOW ) -> super::super::super::Foundation:: BOOL );
    SymNextW(hprocess.into_param().abi(), siw)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymPrev<P0>(hprocess: P0, si: *mut SYMBOL_INFO) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymPrev ( hprocess : super::super::super::Foundation:: HANDLE , si : *mut SYMBOL_INFO ) -> super::super::super::Foundation:: BOOL );
    SymPrev(hprocess.into_param().abi(), si)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymPrevW<P0>(hprocess: P0, siw: *mut SYMBOL_INFOW) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymPrevW ( hprocess : super::super::super::Foundation:: HANDLE , siw : *mut SYMBOL_INFOW ) -> super::super::super::Foundation:: BOOL );
    SymPrevW(hprocess.into_param().abi(), siw)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymQueryInlineTrace<P0>(hprocess: P0, startaddress: u64, startcontext: u32, startretaddress: u64, curaddress: u64, curcontext: *mut u32, curframeindex: *mut u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymQueryInlineTrace ( hprocess : super::super::super::Foundation:: HANDLE , startaddress : u64 , startcontext : u32 , startretaddress : u64 , curaddress : u64 , curcontext : *mut u32 , curframeindex : *mut u32 ) -> super::super::super::Foundation:: BOOL );
    SymQueryInlineTrace(hprocess.into_param().abi(), startaddress, startcontext, startretaddress, curaddress, curcontext, curframeindex)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymRefreshModuleList<P0>(hprocess: P0) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymRefreshModuleList ( hprocess : super::super::super::Foundation:: HANDLE ) -> super::super::super::Foundation:: BOOL );
    SymRefreshModuleList(hprocess.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymRegisterCallback<P0>(hprocess: P0, callbackfunction: PSYMBOL_REGISTERED_CALLBACK, usercontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymRegisterCallback ( hprocess : super::super::super::Foundation:: HANDLE , callbackfunction : PSYMBOL_REGISTERED_CALLBACK , usercontext : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    SymRegisterCallback(hprocess.into_param().abi(), callbackfunction, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymRegisterCallback64<P0>(hprocess: P0, callbackfunction: PSYMBOL_REGISTERED_CALLBACK64, usercontext: u64) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymRegisterCallback64 ( hprocess : super::super::super::Foundation:: HANDLE , callbackfunction : PSYMBOL_REGISTERED_CALLBACK64 , usercontext : u64 ) -> super::super::super::Foundation:: BOOL );
    SymRegisterCallback64(hprocess.into_param().abi(), callbackfunction, usercontext)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymRegisterCallbackW64<P0>(hprocess: P0, callbackfunction: PSYMBOL_REGISTERED_CALLBACK64, usercontext: u64) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymRegisterCallbackW64 ( hprocess : super::super::super::Foundation:: HANDLE , callbackfunction : PSYMBOL_REGISTERED_CALLBACK64 , usercontext : u64 ) -> super::super::super::Foundation:: BOOL );
    SymRegisterCallbackW64(hprocess.into_param().abi(), callbackfunction, usercontext)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymRegisterFunctionEntryCallback<P0>(hprocess: P0, callbackfunction: PSYMBOL_FUNCENTRY_CALLBACK, usercontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymRegisterFunctionEntryCallback ( hprocess : super::super::super::Foundation:: HANDLE , callbackfunction : PSYMBOL_FUNCENTRY_CALLBACK , usercontext : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    SymRegisterFunctionEntryCallback(hprocess.into_param().abi(), callbackfunction, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymRegisterFunctionEntryCallback64<P0>(hprocess: P0, callbackfunction: PSYMBOL_FUNCENTRY_CALLBACK64, usercontext: u64) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymRegisterFunctionEntryCallback64 ( hprocess : super::super::super::Foundation:: HANDLE , callbackfunction : PSYMBOL_FUNCENTRY_CALLBACK64 , usercontext : u64 ) -> super::super::super::Foundation:: BOOL );
    SymRegisterFunctionEntryCallback64(hprocess.into_param().abi(), callbackfunction, usercontext)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymSearch<P0, P1>(hprocess: P0, baseofdll: u64, index: u32, symtag: u32, mask: P1, address: u64, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: ::core::option::Option<*const ::core::ffi::c_void>, options: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymSearch ( hprocess : super::super::super::Foundation:: HANDLE , baseofdll : u64 , index : u32 , symtag : u32 , mask : ::windows::core::PCSTR , address : u64 , enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACK , usercontext : *const ::core::ffi::c_void , options : u32 ) -> super::super::super::Foundation:: BOOL );
    SymSearch(hprocess.into_param().abi(), baseofdll, index, symtag, mask.into_param().abi(), address, enumsymbolscallback, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())), options)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymSearchW<P0, P1>(hprocess: P0, baseofdll: u64, index: u32, symtag: u32, mask: P1, address: u64, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext: ::core::option::Option<*const ::core::ffi::c_void>, options: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymSearchW ( hprocess : super::super::super::Foundation:: HANDLE , baseofdll : u64 , index : u32 , symtag : u32 , mask : ::windows::core::PCWSTR , address : u64 , enumsymbolscallback : PSYM_ENUMERATESYMBOLS_CALLBACKW , usercontext : *const ::core::ffi::c_void , options : u32 ) -> super::super::super::Foundation:: BOOL );
    SymSearchW(hprocess.into_param().abi(), baseofdll, index, symtag, mask.into_param().abi(), address, enumsymbolscallback, ::core::mem::transmute(usercontext.unwrap_or(::std::ptr::null())), options)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymSetContext<P0>(hprocess: P0, stackframe: *const IMAGEHLP_STACK_FRAME, context: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymSetContext ( hprocess : super::super::super::Foundation:: HANDLE , stackframe : *const IMAGEHLP_STACK_FRAME , context : *const ::core::ffi::c_void ) -> super::super::super::Foundation:: BOOL );
    SymSetContext(hprocess.into_param().abi(), stackframe, ::core::mem::transmute(context.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymSetExtendedOption<P0>(option: IMAGEHLP_EXTENDED_OPTIONS, value: P0) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymSetExtendedOption ( option : IMAGEHLP_EXTENDED_OPTIONS , value : super::super::super::Foundation:: BOOL ) -> super::super::super::Foundation:: BOOL );
    SymSetExtendedOption(option, value.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymSetHomeDirectory<P0, P1>(hprocess: P0, dir: P1) -> ::windows::core::PSTR
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymSetHomeDirectory ( hprocess : super::super::super::Foundation:: HANDLE , dir : ::windows::core::PCSTR ) -> ::windows::core::PSTR );
    SymSetHomeDirectory(hprocess.into_param().abi(), dir.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymSetHomeDirectoryW<P0, P1>(hprocess: P0, dir: P1) -> ::windows::core::PWSTR
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymSetHomeDirectoryW ( hprocess : super::super::super::Foundation:: HANDLE , dir : ::windows::core::PCWSTR ) -> ::windows::core::PWSTR );
    SymSetHomeDirectoryW(hprocess.into_param().abi(), dir.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn SymSetOptions(symoptions: u32) -> u32 {
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymSetOptions ( symoptions : u32 ) -> u32 );
    SymSetOptions(symoptions)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymSetParentWindow<P0>(hwnd: P0) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymSetParentWindow ( hwnd : super::super::super::Foundation:: HWND ) -> super::super::super::Foundation:: BOOL );
    SymSetParentWindow(hwnd.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymSetScopeFromAddr<P0>(hprocess: P0, address: u64) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymSetScopeFromAddr ( hprocess : super::super::super::Foundation:: HANDLE , address : u64 ) -> super::super::super::Foundation:: BOOL );
    SymSetScopeFromAddr(hprocess.into_param().abi(), address)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymSetScopeFromIndex<P0>(hprocess: P0, baseofdll: u64, index: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymSetScopeFromIndex ( hprocess : super::super::super::Foundation:: HANDLE , baseofdll : u64 , index : u32 ) -> super::super::super::Foundation:: BOOL );
    SymSetScopeFromIndex(hprocess.into_param().abi(), baseofdll, index)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymSetScopeFromInlineContext<P0>(hprocess: P0, address: u64, inlinecontext: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymSetScopeFromInlineContext ( hprocess : super::super::super::Foundation:: HANDLE , address : u64 , inlinecontext : u32 ) -> super::super::super::Foundation:: BOOL );
    SymSetScopeFromInlineContext(hprocess.into_param().abi(), address, inlinecontext)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymSetSearchPath<P0, P1>(hprocess: P0, searchpatha: P1) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymSetSearchPath ( hprocess : super::super::super::Foundation:: HANDLE , searchpatha : ::windows::core::PCSTR ) -> super::super::super::Foundation:: BOOL );
    SymSetSearchPath(hprocess.into_param().abi(), searchpatha.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymSetSearchPathW<P0, P1>(hprocess: P0, searchpatha: P1) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymSetSearchPathW ( hprocess : super::super::super::Foundation:: HANDLE , searchpatha : ::windows::core::PCWSTR ) -> super::super::super::Foundation:: BOOL );
    SymSetSearchPathW(hprocess.into_param().abi(), searchpatha.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymSrvDeltaName<P0, P1, P2, P3, P4>(hprocess: P0, sympath: P1, r#type: P2, file1: P3, file2: P4) -> ::windows::core::PCSTR
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P4: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymSrvDeltaName ( hprocess : super::super::super::Foundation:: HANDLE , sympath : ::windows::core::PCSTR , r#type : ::windows::core::PCSTR , file1 : ::windows::core::PCSTR , file2 : ::windows::core::PCSTR ) -> ::windows::core::PCSTR );
    SymSrvDeltaName(hprocess.into_param().abi(), sympath.into_param().abi(), r#type.into_param().abi(), file1.into_param().abi(), file2.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymSrvDeltaNameW<P0, P1, P2, P3, P4>(hprocess: P0, sympath: P1, r#type: P2, file1: P3, file2: P4) -> ::windows::core::PCWSTR
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P4: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymSrvDeltaNameW ( hprocess : super::super::super::Foundation:: HANDLE , sympath : ::windows::core::PCWSTR , r#type : ::windows::core::PCWSTR , file1 : ::windows::core::PCWSTR , file2 : ::windows::core::PCWSTR ) -> ::windows::core::PCWSTR );
    SymSrvDeltaNameW(hprocess.into_param().abi(), sympath.into_param().abi(), r#type.into_param().abi(), file1.into_param().abi(), file2.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymSrvGetFileIndexInfo<P0>(file: P0, info: *mut SYMSRV_INDEX_INFO, flags: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymSrvGetFileIndexInfo ( file : ::windows::core::PCSTR , info : *mut SYMSRV_INDEX_INFO , flags : u32 ) -> super::super::super::Foundation:: BOOL );
    SymSrvGetFileIndexInfo(file.into_param().abi(), info, flags)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymSrvGetFileIndexInfoW<P0>(file: P0, info: *mut SYMSRV_INDEX_INFOW, flags: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymSrvGetFileIndexInfoW ( file : ::windows::core::PCWSTR , info : *mut SYMSRV_INDEX_INFOW , flags : u32 ) -> super::super::super::Foundation:: BOOL );
    SymSrvGetFileIndexInfoW(file.into_param().abi(), info, flags)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymSrvGetFileIndexString<P0, P1, P2>(hprocess: P0, srvpath: P1, file: P2, index: &mut [u8], flags: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymSrvGetFileIndexString ( hprocess : super::super::super::Foundation:: HANDLE , srvpath : ::windows::core::PCSTR , file : ::windows::core::PCSTR , index : ::windows::core::PSTR , size : usize , flags : u32 ) -> super::super::super::Foundation:: BOOL );
    SymSrvGetFileIndexString(hprocess.into_param().abi(), srvpath.into_param().abi(), file.into_param().abi(), ::core::mem::transmute(index.as_ptr()), index.len() as _, flags)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymSrvGetFileIndexStringW<P0, P1, P2>(hprocess: P0, srvpath: P1, file: P2, index: &mut [u16], flags: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymSrvGetFileIndexStringW ( hprocess : super::super::super::Foundation:: HANDLE , srvpath : ::windows::core::PCWSTR , file : ::windows::core::PCWSTR , index : ::windows::core::PWSTR , size : usize , flags : u32 ) -> super::super::super::Foundation:: BOOL );
    SymSrvGetFileIndexStringW(hprocess.into_param().abi(), srvpath.into_param().abi(), file.into_param().abi(), ::core::mem::transmute(index.as_ptr()), index.len() as _, flags)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymSrvGetFileIndexes<P0>(file: P0, id: *mut ::windows::core::GUID, val1: *mut u32, val2: ::core::option::Option<*mut u32>, flags: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymSrvGetFileIndexes ( file : ::windows::core::PCSTR , id : *mut ::windows::core::GUID , val1 : *mut u32 , val2 : *mut u32 , flags : u32 ) -> super::super::super::Foundation:: BOOL );
    SymSrvGetFileIndexes(file.into_param().abi(), id, val1, ::core::mem::transmute(val2.unwrap_or(::std::ptr::null_mut())), flags)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymSrvGetFileIndexesW<P0>(file: P0, id: *mut ::windows::core::GUID, val1: *mut u32, val2: ::core::option::Option<*mut u32>, flags: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymSrvGetFileIndexesW ( file : ::windows::core::PCWSTR , id : *mut ::windows::core::GUID , val1 : *mut u32 , val2 : *mut u32 , flags : u32 ) -> super::super::super::Foundation:: BOOL );
    SymSrvGetFileIndexesW(file.into_param().abi(), id, val1, ::core::mem::transmute(val2.unwrap_or(::std::ptr::null_mut())), flags)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymSrvGetSupplement<P0, P1, P2, P3>(hprocess: P0, sympath: P1, node: P2, file: P3) -> ::windows::core::PCSTR
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymSrvGetSupplement ( hprocess : super::super::super::Foundation:: HANDLE , sympath : ::windows::core::PCSTR , node : ::windows::core::PCSTR , file : ::windows::core::PCSTR ) -> ::windows::core::PCSTR );
    SymSrvGetSupplement(hprocess.into_param().abi(), sympath.into_param().abi(), node.into_param().abi(), file.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymSrvGetSupplementW<P0, P1, P2, P3>(hprocess: P0, sympath: P1, node: P2, file: P3) -> ::windows::core::PCWSTR
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymSrvGetSupplementW ( hprocess : super::super::super::Foundation:: HANDLE , sympath : ::windows::core::PCWSTR , node : ::windows::core::PCWSTR , file : ::windows::core::PCWSTR ) -> ::windows::core::PCWSTR );
    SymSrvGetSupplementW(hprocess.into_param().abi(), sympath.into_param().abi(), node.into_param().abi(), file.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymSrvIsStore<P0, P1>(hprocess: P0, path: P1) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymSrvIsStore ( hprocess : super::super::super::Foundation:: HANDLE , path : ::windows::core::PCSTR ) -> super::super::super::Foundation:: BOOL );
    SymSrvIsStore(hprocess.into_param().abi(), path.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymSrvIsStoreW<P0, P1>(hprocess: P0, path: P1) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymSrvIsStoreW ( hprocess : super::super::super::Foundation:: HANDLE , path : ::windows::core::PCWSTR ) -> super::super::super::Foundation:: BOOL );
    SymSrvIsStoreW(hprocess.into_param().abi(), path.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymSrvStoreFile<P0, P1, P2>(hprocess: P0, srvpath: P1, file: P2, flags: SYM_SRV_STORE_FILE_FLAGS) -> ::windows::core::PCSTR
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymSrvStoreFile ( hprocess : super::super::super::Foundation:: HANDLE , srvpath : ::windows::core::PCSTR , file : ::windows::core::PCSTR , flags : SYM_SRV_STORE_FILE_FLAGS ) -> ::windows::core::PCSTR );
    SymSrvStoreFile(hprocess.into_param().abi(), srvpath.into_param().abi(), file.into_param().abi(), flags)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymSrvStoreFileW<P0, P1, P2>(hprocess: P0, srvpath: P1, file: P2, flags: SYM_SRV_STORE_FILE_FLAGS) -> ::windows::core::PCWSTR
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymSrvStoreFileW ( hprocess : super::super::super::Foundation:: HANDLE , srvpath : ::windows::core::PCWSTR , file : ::windows::core::PCWSTR , flags : SYM_SRV_STORE_FILE_FLAGS ) -> ::windows::core::PCWSTR );
    SymSrvStoreFileW(hprocess.into_param().abi(), srvpath.into_param().abi(), file.into_param().abi(), flags)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymSrvStoreSupplement<P0, P1, P2, P3>(hprocess: P0, srvpath: P1, node: P2, file: P3, flags: u32) -> ::windows::core::PCSTR
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymSrvStoreSupplement ( hprocess : super::super::super::Foundation:: HANDLE , srvpath : ::windows::core::PCSTR , node : ::windows::core::PCSTR , file : ::windows::core::PCSTR , flags : u32 ) -> ::windows::core::PCSTR );
    SymSrvStoreSupplement(hprocess.into_param().abi(), srvpath.into_param().abi(), node.into_param().abi(), file.into_param().abi(), flags)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymSrvStoreSupplementW<P0, P1, P2, P3>(hprocess: P0, sympath: P1, node: P2, file: P3, flags: u32) -> ::windows::core::PCWSTR
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymSrvStoreSupplementW ( hprocess : super::super::super::Foundation:: HANDLE , sympath : ::windows::core::PCWSTR , node : ::windows::core::PCWSTR , file : ::windows::core::PCWSTR , flags : u32 ) -> ::windows::core::PCWSTR );
    SymSrvStoreSupplementW(hprocess.into_param().abi(), sympath.into_param().abi(), node.into_param().abi(), file.into_param().abi(), flags)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymUnDName(sym: *const IMAGEHLP_SYMBOL, undecname: &mut [u8]) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymUnDName ( sym : *const IMAGEHLP_SYMBOL , undecname : ::windows::core::PSTR , undecnamelength : u32 ) -> super::super::super::Foundation:: BOOL );
    SymUnDName(sym, ::core::mem::transmute(undecname.as_ptr()), undecname.len() as _)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymUnDName64(sym: *const IMAGEHLP_SYMBOL64, undecname: &mut [u8]) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymUnDName64 ( sym : *const IMAGEHLP_SYMBOL64 , undecname : ::windows::core::PSTR , undecnamelength : u32 ) -> super::super::super::Foundation:: BOOL );
    SymUnDName64(sym, ::core::mem::transmute(undecname.as_ptr()), undecname.len() as _)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymUnloadModule<P0>(hprocess: P0, baseofdll: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymUnloadModule ( hprocess : super::super::super::Foundation:: HANDLE , baseofdll : u32 ) -> super::super::super::Foundation:: BOOL );
    SymUnloadModule(hprocess.into_param().abi(), baseofdll)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SymUnloadModule64<P0>(hprocess: P0, baseofdll: u64) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn SymUnloadModule64 ( hprocess : super::super::super::Foundation:: HANDLE , baseofdll : u64 ) -> super::super::super::Foundation:: BOOL );
    SymUnloadModule64(hprocess.into_param().abi(), baseofdll)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn TerminateProcessOnMemoryExhaustion(failedallocationsize: usize) {
    ::windows_targets::link ! ( "api-ms-win-core-errorhandling-l1-1-3.dll""system" fn TerminateProcessOnMemoryExhaustion ( failedallocationsize : usize ) -> ( ) );
    TerminateProcessOnMemoryExhaustion(failedallocationsize)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TouchFileTimes<P0>(filehandle: P0, psystemtime: ::core::option::Option<*const super::super::super::Foundation::SYSTEMTIME>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "imagehlp.dll""system" fn TouchFileTimes ( filehandle : super::super::super::Foundation:: HANDLE , psystemtime : *const super::super::super::Foundation:: SYSTEMTIME ) -> super::super::super::Foundation:: BOOL );
    TouchFileTimes(filehandle.into_param().abi(), ::core::mem::transmute(psystemtime.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn UnDecorateSymbolName<P0>(name: P0, outputstring: &mut [u8], flags: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn UnDecorateSymbolName ( name : ::windows::core::PCSTR , outputstring : ::windows::core::PSTR , maxstringlength : u32 , flags : u32 ) -> u32 );
    UnDecorateSymbolName(name.into_param().abi(), ::core::mem::transmute(outputstring.as_ptr()), outputstring.len() as _, flags)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[inline]
pub unsafe fn UnDecorateSymbolNameW<P0>(name: P0, outputstring: &mut [u16], flags: u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "dbghelp.dll""system" fn UnDecorateSymbolNameW ( name : ::windows::core::PCWSTR , outputstring : ::windows::core::PWSTR , maxstringlength : u32 , flags : u32 ) -> u32 );
    UnDecorateSymbolNameW(name.into_param().abi(), ::core::mem::transmute(outputstring.as_ptr()), outputstring.len() as _, flags)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn UnMapAndLoad(loadedimage: *mut LOADED_IMAGE) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "imagehlp.dll""system" fn UnMapAndLoad ( loadedimage : *mut LOADED_IMAGE ) -> super::super::super::Foundation:: BOOL );
    UnMapAndLoad(loadedimage)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn UnhandledExceptionFilter(exceptioninfo: *const EXCEPTION_POINTERS) -> i32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn UnhandledExceptionFilter ( exceptioninfo : *const EXCEPTION_POINTERS ) -> i32 );
    UnhandledExceptionFilter(exceptioninfo)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn UpdateDebugInfoFile<P0, P1>(imagefilename: P0, symbolpath: P1, debugfilepath: ::windows::core::PSTR, ntheaders: *const IMAGE_NT_HEADERS32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "imagehlp.dll""system" fn UpdateDebugInfoFile ( imagefilename : ::windows::core::PCSTR , symbolpath : ::windows::core::PCSTR , debugfilepath : ::windows::core::PSTR , ntheaders : *const IMAGE_NT_HEADERS32 ) -> super::super::super::Foundation:: BOOL );
    UpdateDebugInfoFile(imagefilename.into_param().abi(), symbolpath.into_param().abi(), ::core::mem::transmute(debugfilepath), ntheaders)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn UpdateDebugInfoFileEx<P0, P1>(imagefilename: P0, symbolpath: P1, debugfilepath: ::windows::core::PSTR, ntheaders: *const IMAGE_NT_HEADERS32, oldchecksum: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "imagehlp.dll""system" fn UpdateDebugInfoFileEx ( imagefilename : ::windows::core::PCSTR , symbolpath : ::windows::core::PCSTR , debugfilepath : ::windows::core::PSTR , ntheaders : *const IMAGE_NT_HEADERS32 , oldchecksum : u32 ) -> super::super::super::Foundation:: BOOL );
    UpdateDebugInfoFileEx(imagefilename.into_param().abi(), symbolpath.into_param().abi(), ::core::mem::transmute(debugfilepath), ntheaders, oldchecksum)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Threading\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
#[inline]
pub unsafe fn WaitForDebugEvent(lpdebugevent: *mut DEBUG_EVENT, dwmilliseconds: u32) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn WaitForDebugEvent ( lpdebugevent : *mut DEBUG_EVENT , dwmilliseconds : u32 ) -> super::super::super::Foundation:: BOOL );
    WaitForDebugEvent(lpdebugevent, dwmilliseconds)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Threading\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
#[inline]
pub unsafe fn WaitForDebugEventEx(lpdebugevent: *mut DEBUG_EVENT, dwmilliseconds: u32) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn WaitForDebugEventEx ( lpdebugevent : *mut DEBUG_EVENT , dwmilliseconds : u32 ) -> super::super::super::Foundation:: BOOL );
    WaitForDebugEventEx(lpdebugevent, dwmilliseconds)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Wow64GetThreadContext<P0>(hthread: P0, lpcontext: *mut WOW64_CONTEXT) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn Wow64GetThreadContext ( hthread : super::super::super::Foundation:: HANDLE , lpcontext : *mut WOW64_CONTEXT ) -> super::super::super::Foundation:: BOOL );
    Wow64GetThreadContext(hthread.into_param().abi(), lpcontext)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Wow64GetThreadSelectorEntry<P0>(hthread: P0, dwselector: u32, lpselectorentry: *mut WOW64_LDT_ENTRY) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn Wow64GetThreadSelectorEntry ( hthread : super::super::super::Foundation:: HANDLE , dwselector : u32 , lpselectorentry : *mut WOW64_LDT_ENTRY ) -> super::super::super::Foundation:: BOOL );
    Wow64GetThreadSelectorEntry(hthread.into_param().abi(), dwselector, lpselectorentry)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Wow64SetThreadContext<P0>(hthread: P0, lpcontext: *const WOW64_CONTEXT) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn Wow64SetThreadContext ( hthread : super::super::super::Foundation:: HANDLE , lpcontext : *const WOW64_CONTEXT ) -> super::super::super::Foundation:: BOOL );
    Wow64SetThreadContext(hthread.into_param().abi(), lpcontext)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteProcessMemory<P0>(hprocess: P0, lpbaseaddress: *const ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, nsize: usize, lpnumberofbyteswritten: ::core::option::Option<*mut usize>) -> super::super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn WriteProcessMemory ( hprocess : super::super::super::Foundation:: HANDLE , lpbaseaddress : *const ::core::ffi::c_void , lpbuffer : *const ::core::ffi::c_void , nsize : usize , lpnumberofbyteswritten : *mut usize ) -> super::super::super::Foundation:: BOOL );
    WriteProcessMemory(hprocess.into_param().abi(), lpbaseaddress, lpbuffer, nsize, ::core::mem::transmute(lpnumberofbyteswritten.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
pub struct IDebugExtendedProperty(::windows::core::IUnknown);
impl IDebugExtendedProperty {
    pub unsafe fn GetPropertyInfo(&self, dwfieldspec: u32, nradix: u32, ppropertyinfo: *mut DebugPropertyInfo) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPropertyInfo)(::windows::core::Interface::as_raw(self), dwfieldspec, nradix, ppropertyinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetExtendedInfo(&self, cinfos: u32, rgguidextendedinfo: *const ::windows::core::GUID, rgvar: *mut super::super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetExtendedInfo)(::windows::core::Interface::as_raw(self), cinfos, rgguidextendedinfo, rgvar).ok()
    }
    pub unsafe fn SetValueAsString<P0>(&self, pszvalue: P0, nradix: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetValueAsString)(::windows::core::Interface::as_raw(self), pszvalue.into_param().abi(), nradix).ok()
    }
    pub unsafe fn EnumMembers(&self, dwfieldspec: u32, nradix: u32, refiid: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumDebugPropertyInfo> {
        let mut result__ = ::windows::core::zeroed::<IEnumDebugPropertyInfo>();
        (::windows::core::Interface::vtable(self).base__.EnumMembers)(::windows::core::Interface::as_raw(self), dwfieldspec, nradix, refiid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetParent(&self) -> ::windows::core::Result<IDebugProperty> {
        let mut result__ = ::windows::core::zeroed::<IDebugProperty>();
        (::windows::core::Interface::vtable(self).base__.GetParent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub unsafe fn GetExtendedPropertyInfo(&self, dwfieldspec: u32, nradix: u32, pextendedpropertyinfo: *mut ExtendedDebugPropertyInfo) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetExtendedPropertyInfo)(::windows::core::Interface::as_raw(self), dwfieldspec, nradix, pextendedpropertyinfo).ok()
    }
    pub unsafe fn EnumExtendedMembers(&self, dwfieldspec: u32, nradix: u32) -> ::windows::core::Result<IEnumDebugExtendedPropertyInfo> {
        let mut result__ = ::windows::core::zeroed::<IEnumDebugExtendedPropertyInfo>();
        (::windows::core::Interface::vtable(self).EnumExtendedMembers)(::windows::core::Interface::as_raw(self), dwfieldspec, nradix, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDebugExtendedProperty, ::windows::core::IUnknown, IDebugProperty);
impl ::core::cmp::PartialEq for IDebugExtendedProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugExtendedProperty {}
impl ::core::fmt::Debug for IDebugExtendedProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugExtendedProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugExtendedProperty {
    type Vtable = IDebugExtendedProperty_Vtbl;
}
impl ::core::clone::Clone for IDebugExtendedProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugExtendedProperty {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c52_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugExtendedProperty_Vtbl {
    pub base__: IDebugProperty_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub GetExtendedPropertyInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwfieldspec: u32, nradix: u32, pextendedpropertyinfo: *mut ExtendedDebugPropertyInfo) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole")))]
    GetExtendedPropertyInfo: usize,
    pub EnumExtendedMembers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwfieldspec: u32, nradix: u32, ppeepi: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
pub struct IDebugProperty(::windows::core::IUnknown);
impl IDebugProperty {
    pub unsafe fn GetPropertyInfo(&self, dwfieldspec: u32, nradix: u32, ppropertyinfo: *mut DebugPropertyInfo) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPropertyInfo)(::windows::core::Interface::as_raw(self), dwfieldspec, nradix, ppropertyinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetExtendedInfo(&self, cinfos: u32, rgguidextendedinfo: *const ::windows::core::GUID, rgvar: *mut super::super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetExtendedInfo)(::windows::core::Interface::as_raw(self), cinfos, rgguidextendedinfo, rgvar).ok()
    }
    pub unsafe fn SetValueAsString<P0>(&self, pszvalue: P0, nradix: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetValueAsString)(::windows::core::Interface::as_raw(self), pszvalue.into_param().abi(), nradix).ok()
    }
    pub unsafe fn EnumMembers(&self, dwfieldspec: u32, nradix: u32, refiid: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumDebugPropertyInfo> {
        let mut result__ = ::windows::core::zeroed::<IEnumDebugPropertyInfo>();
        (::windows::core::Interface::vtable(self).EnumMembers)(::windows::core::Interface::as_raw(self), dwfieldspec, nradix, refiid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetParent(&self) -> ::windows::core::Result<IDebugProperty> {
        let mut result__ = ::windows::core::zeroed::<IDebugProperty>();
        (::windows::core::Interface::vtable(self).GetParent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDebugProperty, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDebugProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugProperty {}
impl ::core::fmt::Debug for IDebugProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugProperty {
    type Vtable = IDebugProperty_Vtbl;
}
impl ::core::clone::Clone for IDebugProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugProperty {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c50_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugProperty_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetPropertyInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwfieldspec: u32, nradix: u32, ppropertyinfo: *mut DebugPropertyInfo) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetExtendedInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cinfos: u32, rgguidextendedinfo: *const ::windows::core::GUID, rgvar: *mut super::super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetExtendedInfo: usize,
    pub SetValueAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszvalue: ::windows::core::PCWSTR, nradix: u32) -> ::windows::core::HRESULT,
    pub EnumMembers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwfieldspec: u32, nradix: u32, refiid: *const ::windows::core::GUID, ppepi: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetParent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdebugprop: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
pub struct IDebugPropertyEnumType_All(::windows::core::IUnknown);
impl IDebugPropertyEnumType_All {
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDebugPropertyEnumType_All, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDebugPropertyEnumType_All {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugPropertyEnumType_All {}
impl ::core::fmt::Debug for IDebugPropertyEnumType_All {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugPropertyEnumType_All").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugPropertyEnumType_All {
    type Vtable = IDebugPropertyEnumType_All_Vtbl;
}
impl ::core::clone::Clone for IDebugPropertyEnumType_All {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugPropertyEnumType_All {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c55_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugPropertyEnumType_All_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, __midl__idebugpropertyenumtype_all0000: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
pub struct IDebugPropertyEnumType_Arguments(::windows::core::IUnknown);
impl IDebugPropertyEnumType_Arguments {
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDebugPropertyEnumType_Arguments, ::windows::core::IUnknown, IDebugPropertyEnumType_All);
impl ::core::cmp::PartialEq for IDebugPropertyEnumType_Arguments {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugPropertyEnumType_Arguments {}
impl ::core::fmt::Debug for IDebugPropertyEnumType_Arguments {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugPropertyEnumType_Arguments").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugPropertyEnumType_Arguments {
    type Vtable = IDebugPropertyEnumType_Arguments_Vtbl;
}
impl ::core::clone::Clone for IDebugPropertyEnumType_Arguments {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugPropertyEnumType_Arguments {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c57_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugPropertyEnumType_Arguments_Vtbl {
    pub base__: IDebugPropertyEnumType_All_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
pub struct IDebugPropertyEnumType_Locals(::windows::core::IUnknown);
impl IDebugPropertyEnumType_Locals {
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDebugPropertyEnumType_Locals, ::windows::core::IUnknown, IDebugPropertyEnumType_All);
impl ::core::cmp::PartialEq for IDebugPropertyEnumType_Locals {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugPropertyEnumType_Locals {}
impl ::core::fmt::Debug for IDebugPropertyEnumType_Locals {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugPropertyEnumType_Locals").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugPropertyEnumType_Locals {
    type Vtable = IDebugPropertyEnumType_Locals_Vtbl;
}
impl ::core::clone::Clone for IDebugPropertyEnumType_Locals {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugPropertyEnumType_Locals {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c56_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugPropertyEnumType_Locals_Vtbl {
    pub base__: IDebugPropertyEnumType_All_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
pub struct IDebugPropertyEnumType_LocalsPlusArgs(::windows::core::IUnknown);
impl IDebugPropertyEnumType_LocalsPlusArgs {
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDebugPropertyEnumType_LocalsPlusArgs, ::windows::core::IUnknown, IDebugPropertyEnumType_All);
impl ::core::cmp::PartialEq for IDebugPropertyEnumType_LocalsPlusArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugPropertyEnumType_LocalsPlusArgs {}
impl ::core::fmt::Debug for IDebugPropertyEnumType_LocalsPlusArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugPropertyEnumType_LocalsPlusArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugPropertyEnumType_LocalsPlusArgs {
    type Vtable = IDebugPropertyEnumType_LocalsPlusArgs_Vtbl;
}
impl ::core::clone::Clone for IDebugPropertyEnumType_LocalsPlusArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugPropertyEnumType_LocalsPlusArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c58_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugPropertyEnumType_LocalsPlusArgs_Vtbl {
    pub base__: IDebugPropertyEnumType_All_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
pub struct IDebugPropertyEnumType_Registers(::windows::core::IUnknown);
impl IDebugPropertyEnumType_Registers {
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDebugPropertyEnumType_Registers, ::windows::core::IUnknown, IDebugPropertyEnumType_All);
impl ::core::cmp::PartialEq for IDebugPropertyEnumType_Registers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDebugPropertyEnumType_Registers {}
impl ::core::fmt::Debug for IDebugPropertyEnumType_Registers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDebugPropertyEnumType_Registers").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDebugPropertyEnumType_Registers {
    type Vtable = IDebugPropertyEnumType_Registers_Vtbl;
}
impl ::core::clone::Clone for IDebugPropertyEnumType_Registers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDebugPropertyEnumType_Registers {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c59_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugPropertyEnumType_Registers_Vtbl {
    pub base__: IDebugPropertyEnumType_All_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
pub struct IEnumDebugExtendedPropertyInfo(::windows::core::IUnknown);
impl IEnumDebugExtendedPropertyInfo {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub unsafe fn Next(&self, rgextendedpropertyinfo: &mut [ExtendedDebugPropertyInfo], pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), rgextendedpropertyinfo.len() as _, ::core::mem::transmute(rgextendedpropertyinfo.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumDebugExtendedPropertyInfo> {
        let mut result__ = ::windows::core::zeroed::<IEnumDebugExtendedPropertyInfo>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IEnumDebugExtendedPropertyInfo, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumDebugExtendedPropertyInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDebugExtendedPropertyInfo {}
impl ::core::fmt::Debug for IEnumDebugExtendedPropertyInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumDebugExtendedPropertyInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumDebugExtendedPropertyInfo {
    type Vtable = IEnumDebugExtendedPropertyInfo_Vtbl;
}
impl ::core::clone::Clone for IEnumDebugExtendedPropertyInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumDebugExtendedPropertyInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c53_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDebugExtendedPropertyInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgextendedpropertyinfo: *mut ExtendedDebugPropertyInfo, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pedpe: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
pub struct IEnumDebugPropertyInfo(::windows::core::IUnknown);
impl IEnumDebugPropertyInfo {
    pub unsafe fn Next(&self, pi: &mut [DebugPropertyInfo], pceltsfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), pi.len() as _, ::core::mem::transmute(pi.as_ptr()), pceltsfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumDebugPropertyInfo> {
        let mut result__ = ::windows::core::zeroed::<IEnumDebugPropertyInfo>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IEnumDebugPropertyInfo, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumDebugPropertyInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDebugPropertyInfo {}
impl ::core::fmt::Debug for IEnumDebugPropertyInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumDebugPropertyInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumDebugPropertyInfo {
    type Vtable = IEnumDebugPropertyInfo_Vtbl;
}
impl ::core::clone::Clone for IEnumDebugPropertyInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumDebugPropertyInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c51_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDebugPropertyInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, pi: *mut DebugPropertyInfo, pceltsfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppepi: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
pub struct IObjectSafety(::windows::core::IUnknown);
impl IObjectSafety {
    pub unsafe fn GetInterfaceSafetyOptions(&self, riid: *const ::windows::core::GUID, pdwsupportedoptions: *mut u32, pdwenabledoptions: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetInterfaceSafetyOptions)(::windows::core::Interface::as_raw(self), riid, pdwsupportedoptions, pdwenabledoptions).ok()
    }
    pub unsafe fn SetInterfaceSafetyOptions(&self, riid: *const ::windows::core::GUID, dwoptionsetmask: u32, dwenabledoptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInterfaceSafetyOptions)(::windows::core::Interface::as_raw(self), riid, dwoptionsetmask, dwenabledoptions).ok()
    }
}
::windows::imp::interface_hierarchy!(IObjectSafety, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IObjectSafety {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectSafety {}
impl ::core::fmt::Debug for IObjectSafety {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectSafety").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IObjectSafety {
    type Vtable = IObjectSafety_Vtbl;
}
impl ::core::clone::Clone for IObjectSafety {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IObjectSafety {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb5bdc81_93c1_11cf_8f20_00805f2cd064);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectSafety_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetInterfaceSafetyOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pdwsupportedoptions: *mut u32, pdwenabledoptions: *mut u32) -> ::windows::core::HRESULT,
    pub SetInterfaceSafetyOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, dwoptionsetmask: u32, dwenabledoptions: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
pub struct IPerPropertyBrowsing2(::windows::core::IUnknown);
impl IPerPropertyBrowsing2 {
    pub unsafe fn GetDisplayString(&self, dispid: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetDisplayString)(::windows::core::Interface::as_raw(self), dispid, &mut result__).from_abi(result__)
    }
    pub unsafe fn MapPropertyToPage(&self, dispid: i32) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).MapPropertyToPage)(::windows::core::Interface::as_raw(self), dispid, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetPredefinedStrings(&self, dispid: i32, pcastrings: *mut super::super::Ole::CALPOLESTR, pcacookies: *mut super::super::Ole::CADWORD) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPredefinedStrings)(::windows::core::Interface::as_raw(self), dispid, pcastrings, pcacookies).ok()
    }
    pub unsafe fn SetPredefinedValue(&self, dispid: i32, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPredefinedValue)(::windows::core::Interface::as_raw(self), dispid, dwcookie).ok()
    }
}
::windows::imp::interface_hierarchy!(IPerPropertyBrowsing2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPerPropertyBrowsing2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPerPropertyBrowsing2 {}
impl ::core::fmt::Debug for IPerPropertyBrowsing2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPerPropertyBrowsing2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPerPropertyBrowsing2 {
    type Vtable = IPerPropertyBrowsing2_Vtbl;
}
impl ::core::clone::Clone for IPerPropertyBrowsing2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPerPropertyBrowsing2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51973c54_cb0c_11d0_b5c9_00a0244a0e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerPropertyBrowsing2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDisplayString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispid: i32, pbstr: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub MapPropertyToPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispid: i32, pclsidproppage: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetPredefinedStrings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispid: i32, pcastrings: *mut super::super::Ole::CALPOLESTR, pcacookies: *mut super::super::Ole::CADWORD) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetPredefinedStrings: usize,
    pub SetPredefinedValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispid: i32, dwcookie: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const API_VERSION_NUMBER: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BIND_ALL_IMAGES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BIND_CACHE_IMPORT_DLLS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BIND_NO_BOUND_IMPORTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BIND_NO_UPDATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BIND_REPORT_64BIT_VA: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CBA_CHECK_ARM_MACHINE_THUMB_TYPE_OVERRIDE: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CBA_CHECK_ENGOPT_DISALLOW_NETWORK_PATHS: u32 = 1879048192u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CBA_DEBUG_INFO: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CBA_DEFERRED_SYMBOL_LOAD_CANCEL: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CBA_DEFERRED_SYMBOL_LOAD_COMPLETE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CBA_DEFERRED_SYMBOL_LOAD_FAILURE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CBA_DEFERRED_SYMBOL_LOAD_PARTIAL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CBA_DEFERRED_SYMBOL_LOAD_START: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CBA_DUPLICATE_SYMBOL: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CBA_ENGINE_PRESENT: u32 = 1610612736u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CBA_EVENT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CBA_MAP_JIT_SYMBOL: u32 = 2684354560u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CBA_READ_MEMORY: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CBA_SET_OPTIONS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CBA_SRCSRV_EVENT: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CBA_SRCSRV_INFO: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CBA_SYMBOLS_UNLOADED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CBA_UPDATE_STATUS_BAR: u32 = 1342177280u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CBA_XML_LOG: u32 = 2415919104u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CERT_PE_IMAGE_DIGEST_ALL_IMPORT_INFO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CERT_PE_IMAGE_DIGEST_DEBUG_INFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CERT_PE_IMAGE_DIGEST_NON_PE_INFO: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CERT_PE_IMAGE_DIGEST_RESOURCES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CERT_SECTION_TYPE_ANY: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CHECKSUM_MAPVIEW_FAILURE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CHECKSUM_MAP_FAILURE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CHECKSUM_OPEN_FAILURE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CHECKSUM_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CHECKSUM_UNICODE_FAILURE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBHHEADER_PDBGUID: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DMP_CONTEXT_RECORD_SIZE_32: u32 = 1200u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DMP_CONTEXT_RECORD_SIZE_64: u32 = 3000u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DMP_HEADER_COMMENT_SIZE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DMP_PHYSICAL_MEMORY_BLOCK_SIZE_32: u32 = 700u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DMP_PHYSICAL_MEMORY_BLOCK_SIZE_64: u32 = 700u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DMP_RESERVED_0_SIZE_32: u32 = 1760u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DMP_RESERVED_0_SIZE_64: u32 = 4008u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DMP_RESERVED_2_SIZE_32: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DMP_RESERVED_3_SIZE_32: u32 = 56u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DSLFLAG_MISMATCHED_DBG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DSLFLAG_MISMATCHED_PDB: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DUMP_SUMMARY_VALID_CURRENT_USER_VA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DUMP_SUMMARY_VALID_KERNEL_VA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ERROR_IMAGE_NOT_STRIPPED: u32 = 34816u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ERROR_NO_DBG_POINTER: u32 = 34817u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ERROR_NO_PDB_POINTER: u32 = 34818u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ESLFLAG_FULLPATH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ESLFLAG_INLINE_SITE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ESLFLAG_NEAREST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ESLFLAG_NEXT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ESLFLAG_PREV: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const EVENT_SRCSPEW: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const EVENT_SRCSPEW_END: u32 = 199u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const EVENT_SRCSPEW_START: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const EXT_OUTPUT_VER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FLAG_ENGINE_PRESENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FLAG_ENGOPT_DISALLOW_NETWORK_PATHS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FLAG_OVERRIDE_ARM_MACHINE_TYPE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGEHLP_MODULE_REGION_ADDITIONAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGEHLP_MODULE_REGION_ALL: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGEHLP_MODULE_REGION_DLLBASE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGEHLP_MODULE_REGION_DLLRANGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGEHLP_MODULE_REGION_JIT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGEHLP_RMAP_BIG_ENDIAN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGEHLP_RMAP_FIXUP_ARM64X: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGEHLP_RMAP_FIXUP_IMAGEBASE: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGEHLP_RMAP_IGNORE_MISCOMPARE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGEHLP_RMAP_LOAD_RW_DATA_SECTIONS: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGEHLP_RMAP_MAPPED_FLAT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGEHLP_RMAP_OMIT_SHARED_RW_DATA_SECTIONS: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGEHLP_SYMBOL_FUNCTION: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGEHLP_SYMBOL_INFO_CONSTANT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGEHLP_SYMBOL_INFO_FRAMERELATIVE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGEHLP_SYMBOL_INFO_LOCAL: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGEHLP_SYMBOL_INFO_PARAMETER: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGEHLP_SYMBOL_INFO_REGISTER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGEHLP_SYMBOL_INFO_REGRELATIVE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGEHLP_SYMBOL_INFO_TLSRELATIVE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGEHLP_SYMBOL_INFO_VALUEPRESENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGEHLP_SYMBOL_THUNK: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGEHLP_SYMBOL_VIRTUAL: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INLINE_FRAME_CONTEXT_IGNORE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INLINE_FRAME_CONTEXT_INIT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INTERFACESAFE_FOR_UNTRUSTED_CALLER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INTERFACESAFE_FOR_UNTRUSTED_DATA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INTERFACE_USES_DISPEX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INTERFACE_USES_SECURITY_MANAGER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IOCTL_IPMI_INTERNAL_RECORD_SEL_EVENT: u32 = 2232320u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IPMI_IOCTL_INDEX: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IPMI_OS_SEL_RECORD_MASK: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IPMI_OS_SEL_RECORD_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IPMI_OS_SEL_RECORD_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MAX_SYM_NAME: u32 = 2000u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MINIDUMP_MISC1_PROCESSOR_POWER_INFO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MINIDUMP_MISC3_PROCESS_EXECUTE_FLAGS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MINIDUMP_MISC3_PROCESS_INTEGRITY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MINIDUMP_MISC3_PROTECTED_PROCESS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MINIDUMP_MISC3_TIMEZONE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MINIDUMP_MISC4_BUILDSTRING: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MINIDUMP_MISC5_PROCESS_COOKIE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MINIDUMP_PROCESS_VM_COUNTERS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MINIDUMP_PROCESS_VM_COUNTERS_EX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MINIDUMP_PROCESS_VM_COUNTERS_EX2: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MINIDUMP_PROCESS_VM_COUNTERS_JOB: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MINIDUMP_PROCESS_VM_COUNTERS_VIRTUALSIZE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MINIDUMP_SYSMEMINFO1_BASICPERF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MINIDUMP_SYSMEMINFO1_FILECACHE_TRANSITIONREPURPOSECOUNT_FLAGS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MINIDUMP_SYSMEMINFO1_PERF_CCTOTALDIRTYPAGES_CCDIRTYPAGETHRESHOLD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MINIDUMP_SYSMEMINFO1_PERF_RESIDENTAVAILABLEPAGES_SHAREDCOMMITPAGES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MINIDUMP_VERSION: u32 = 42899u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const NUM_SSRVOPTS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const RESTORE_LAST_ERROR_NAME: ::windows::core::PCWSTR = ::windows::core::w!("RestoreLastError");
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const RESTORE_LAST_ERROR_NAME_A: ::windows::core::PCSTR = ::windows::core::s!("RestoreLastError");
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const RESTORE_LAST_ERROR_NAME_W: ::windows::core::PCWSTR = ::windows::core::w!("RestoreLastError");
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SPLITSYM_EXTRACT_ALL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SPLITSYM_REMOVE_PRIVATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SPLITSYM_SYMBOLPATH_IS_SRC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVACTION_CHECKSUMSTATUS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVACTION_EVENT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVACTION_EVENTW: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVACTION_HTTPSTATUS: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVACTION_QUERYCANCEL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVACTION_SIZE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVACTION_TRACE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVACTION_XMLOUTPUT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_CALLBACK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_CALLBACKW: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_DISABLE_PING_HOST: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_DISABLE_TIMEOUT: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_DONT_UNCOMPRESS: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_DOWNSTREAM_STORE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_ENABLE_COMM_MSG: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_FAVOR_COMPRESSED: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_FLAT_DEFAULT_STORE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_GETPATH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_MAX: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_MESSAGE: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_NOCOPY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_OLDGUIDPTR: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_OVERWRITE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_PARAMTYPE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_PARENTWIN: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_PROXY: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_PROXYW: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_RESETTOU: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_RETRY_APP_HANG: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_SECURE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_SERVICE: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_SETCONTEXT: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_STRING: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_TRACE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_UNATTENDED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_URI_FILTER: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_URI_TIERS: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_WINHTTP: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_WININET: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVURI_ALL: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVURI_COMPRESSED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVURI_FILEPTR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVURI_HTTP_COMPRESSED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVURI_HTTP_FILEPTR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVURI_HTTP_MASK: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVURI_HTTP_NORMAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVURI_NORMAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVURI_UNC_COMPRESSED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVURI_UNC_FILEPTR: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVURI_UNC_MASK: u32 = 240u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVURI_UNC_NORMAL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMENUM_OPTIONS_DEFAULT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMENUM_OPTIONS_INLINE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMFLAG_FIXUP_ARM64X: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMFLAG_FUNC_NO_RETURN: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMFLAG_GLOBAL: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMFLAG_NULL: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMFLAG_PUBLIC_CODE: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMFLAG_REGREL_ALIASINDIR: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMFLAG_RESET: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMFLAG_SYNTHETIC_ZEROBASE: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMF_CONSTANT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMF_EXPORT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMF_FORWARDER: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMF_FRAMEREL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMF_FUNCTION: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMF_LOCAL: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMF_OMAP_GENERATED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMF_OMAP_MODIFIED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMF_PARAMETER: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMF_REGISTER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMF_REGREL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMF_THUNK: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMF_TLSREL: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMF_VIRTUAL: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_ALLOW_ABSOLUTE_SYMBOLS: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_ALLOW_ZERO_ADDRESS: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_AUTO_PUBLICS: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_CASE_INSENSITIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_DEBUG: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_DEFERRED_LOADS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_DISABLE_FAST_SYMBOLS: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_DISABLE_SRVSTAR_ON_STARTUP: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_DISABLE_SYMSRV_AUTODETECT: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_DISABLE_SYMSRV_TIMEOUT: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_EXACT_SYMBOLS: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_FAIL_CRITICAL_ERRORS: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_FAVOR_COMPRESSED: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_FLAT_DIRECTORY: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_IGNORE_CVREC: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_IGNORE_IMAGEDIR: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_IGNORE_NT_SYMPATH: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_INCLUDE_32BIT_MODULES: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_LOAD_ANYTHING: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_LOAD_LINES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_NO_CPP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_NO_IMAGE_SEARCH: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_NO_PROMPTS: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_NO_PUBLICS: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_NO_UNQUALIFIED_LOADS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_OMAP_FIND_NEAREST: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_OVERWRITE: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_PUBLICS_ONLY: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_READONLY_CACHE: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_SECURE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_SYMPATH_LAST: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_UNDNAME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMSEARCH_ALLITEMS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMSEARCH_GLOBALSONLY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMSEARCH_MASKOBJS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMSEARCH_RECURSE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMSRV_VERSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMSTOREOPT_ALT_INDEX: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMSTOREOPT_UNICODE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYM_INLINE_COMP_DIFFERENT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYM_INLINE_COMP_ERROR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYM_INLINE_COMP_IDENTICAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYM_INLINE_COMP_STEPIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYM_INLINE_COMP_STEPOUT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYM_INLINE_COMP_STEPOVER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYM_STKWALK_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYM_STKWALK_FORCE_FRAMEPTR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYM_STKWALK_ZEROEXTEND_PTRS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TARGET_ATTRIBUTE_PACMASK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UNDNAME_32_BIT_DECODE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UNDNAME_COMPLETE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UNDNAME_NAME_ONLY: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UNDNAME_NO_ACCESS_SPECIFIERS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UNDNAME_NO_ALLOCATION_LANGUAGE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UNDNAME_NO_ALLOCATION_MODEL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UNDNAME_NO_ARGUMENTS: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UNDNAME_NO_CV_THISTYPE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UNDNAME_NO_FUNCTION_RETURNS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UNDNAME_NO_LEADING_UNDERSCORES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UNDNAME_NO_MEMBER_TYPE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UNDNAME_NO_MS_KEYWORDS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UNDNAME_NO_MS_THISTYPE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UNDNAME_NO_RETURN_UDT_MODEL: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UNDNAME_NO_SPECIAL_SYMS: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UNDNAME_NO_THISTYPE: u32 = 96u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UNDNAME_NO_THROW_SIGNATURES: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WCT_MAX_NODE_COUNT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WCT_NETWORK_IO_FLAG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WCT_OBJNAME_LENGTH: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_BAD_PAGE_LIST_LOCATION: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_BAD_PAGE_LIST_MAX_SIZE: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_CMCI_THRESHOLD_COUNT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_CMCI_THRESHOLD_POLL_COUNT: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_CMCI_THRESHOLD_TIME: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_DEVICE_DRIVER_BUFFER_SET_MAX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_DEVICE_DRIVER_BUFFER_SET_MIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_DEVICE_DRIVER_BUFFER_SET_V1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_DEVICE_DRIVER_CONFIG_MAX: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_DEVICE_DRIVER_CONFIG_MIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_DEVICE_DRIVER_CONFIG_V1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_DEVICE_DRIVER_CONFIG_V2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_DISABLE_DUMMY_WRITE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_DISABLE_OFFLINE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_AERBRIDGE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_AERENDPOINT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_AERROOTPORT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_GENERIC: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_GENERIC_V2: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_IPFCMC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_IPFCPE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_IPFMCA: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_XPFCMC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_XPFMCE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_XPFNMI: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_VERSION_10: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_VERSION_11: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_ERROR_SOURCE_FLAG_DEFAULTSOURCE: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_ERROR_SOURCE_FLAG_FIRMWAREFIRST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_ERROR_SOURCE_FLAG_GHES_ASSIST: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_ERROR_SOURCE_FLAG_GLOBAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_ERROR_SOURCE_INVALID_RELATED_SOURCE: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_MAX_MC_BANKS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_MEM_PERSISTOFFLINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_MEM_PFA_DISABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_MEM_PFA_PAGECOUNT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_MEM_PFA_THRESHOLD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_MEM_PFA_TIMEOUT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_NOTIFICATION_TYPE_ARMV8_SEA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_NOTIFICATION_TYPE_ARMV8_SEI: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_NOTIFICATION_TYPE_CMCI: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_NOTIFICATION_TYPE_EXTERNALINTERRUPT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_NOTIFICATION_TYPE_EXTERNALINTERRUPT_GSIV: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_NOTIFICATION_TYPE_GPIO_SIGNAL: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_NOTIFICATION_TYPE_LOCALINTERRUPT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_NOTIFICATION_TYPE_MCE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_NOTIFICATION_TYPE_NMI: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_NOTIFICATION_TYPE_POLLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_NOTIFICATION_TYPE_SCI: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_NOTIFICATION_TYPE_SDEI: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_NOTIFY_ALL_OFFLINES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_PENDING_PAGE_LIST_SZ: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_RESTORE_CMCI_ATTEMPTS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_RESTORE_CMCI_ENABLED: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_RESTORE_CMCI_ERR_LIMIT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_ROW_FAIL_CHECK_ENABLE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_ROW_FAIL_CHECK_EXTENT: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_ROW_FAIL_CHECK_THRESHOLD: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_XPF_MC_BANK_STATUSFORMAT_AMD64MCA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_XPF_MC_BANK_STATUSFORMAT_IA32MCA: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_XPF_MC_BANK_STATUSFORMAT_Intel64MCA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WOW64_CONTEXT_EXCEPTION_ACTIVE: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WOW64_CONTEXT_EXCEPTION_REPORTING: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WOW64_CONTEXT_EXCEPTION_REQUEST: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WOW64_CONTEXT_SERVICE_ACTIVE: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WOW64_CONTEXT_i386: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WOW64_CONTEXT_i486: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WOW64_MAXIMUM_SUPPORTED_EXTENSION: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WOW64_SIZE_OF_80387_REGISTERS: u32 = 80u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const sevMax: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADDRESS_MODE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const AddrMode1616: ADDRESS_MODE = ADDRESS_MODE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const AddrMode1632: ADDRESS_MODE = ADDRESS_MODE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const AddrModeReal: ADDRESS_MODE = ADDRESS_MODE(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const AddrModeFlat: ADDRESS_MODE = ADDRESS_MODE(3i32);
impl ::core::marker::Copy for ADDRESS_MODE {}
impl ::core::clone::Clone for ADDRESS_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADDRESS_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ADDRESS_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ADDRESS_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADDRESS_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BUGCHECK_ERROR(pub u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const HARDWARE_PROFILE_UNDOCKED_STRING: BUGCHECK_ERROR = BUGCHECK_ERROR(1073807361u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const HARDWARE_PROFILE_DOCKED_STRING: BUGCHECK_ERROR = BUGCHECK_ERROR(1073807362u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const HARDWARE_PROFILE_UNKNOWN_STRING: BUGCHECK_ERROR = BUGCHECK_ERROR(1073807363u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WINDOWS_NT_BANNER: BUGCHECK_ERROR = BUGCHECK_ERROR(1073741950u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WINDOWS_NT_CSD_STRING: BUGCHECK_ERROR = BUGCHECK_ERROR(1073741959u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WINDOWS_NT_INFO_STRING: BUGCHECK_ERROR = BUGCHECK_ERROR(1073741960u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WINDOWS_NT_MP_STRING: BUGCHECK_ERROR = BUGCHECK_ERROR(1073741961u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const THREAD_TERMINATE_HELD_MUTEX: BUGCHECK_ERROR = BUGCHECK_ERROR(1073741962u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WINDOWS_NT_INFO_STRING_PLURAL: BUGCHECK_ERROR = BUGCHECK_ERROR(1073741981u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WINDOWS_NT_RC_STRING: BUGCHECK_ERROR = BUGCHECK_ERROR(1073741982u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const APC_INDEX_MISMATCH: BUGCHECK_ERROR = BUGCHECK_ERROR(1u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DEVICE_QUEUE_NOT_BUSY: BUGCHECK_ERROR = BUGCHECK_ERROR(2u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INVALID_AFFINITY_SET: BUGCHECK_ERROR = BUGCHECK_ERROR(3u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INVALID_DATA_ACCESS_TRAP: BUGCHECK_ERROR = BUGCHECK_ERROR(4u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INVALID_PROCESS_ATTACH_ATTEMPT: BUGCHECK_ERROR = BUGCHECK_ERROR(5u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INVALID_PROCESS_DETACH_ATTEMPT: BUGCHECK_ERROR = BUGCHECK_ERROR(6u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INVALID_SOFTWARE_INTERRUPT: BUGCHECK_ERROR = BUGCHECK_ERROR(7u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IRQL_NOT_DISPATCH_LEVEL: BUGCHECK_ERROR = BUGCHECK_ERROR(8u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IRQL_NOT_GREATER_OR_EQUAL: BUGCHECK_ERROR = BUGCHECK_ERROR(9u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IRQL_NOT_LESS_OR_EQUAL: BUGCHECK_ERROR = BUGCHECK_ERROR(10u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const NO_EXCEPTION_HANDLING_SUPPORT: BUGCHECK_ERROR = BUGCHECK_ERROR(11u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MAXIMUM_WAIT_OBJECTS_EXCEEDED: BUGCHECK_ERROR = BUGCHECK_ERROR(12u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MUTEX_LEVEL_NUMBER_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(13u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const NO_USER_MODE_CONTEXT: BUGCHECK_ERROR = BUGCHECK_ERROR(14u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SPIN_LOCK_ALREADY_OWNED: BUGCHECK_ERROR = BUGCHECK_ERROR(15u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SPIN_LOCK_NOT_OWNED: BUGCHECK_ERROR = BUGCHECK_ERROR(16u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const THREAD_NOT_MUTEX_OWNER: BUGCHECK_ERROR = BUGCHECK_ERROR(17u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TRAP_CAUSE_UNKNOWN: BUGCHECK_ERROR = BUGCHECK_ERROR(18u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const EMPTY_THREAD_REAPER_LIST: BUGCHECK_ERROR = BUGCHECK_ERROR(19u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CREATE_DELETE_LOCK_NOT_LOCKED: BUGCHECK_ERROR = BUGCHECK_ERROR(20u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const LAST_CHANCE_CALLED_FROM_KMODE: BUGCHECK_ERROR = BUGCHECK_ERROR(21u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CID_HANDLE_CREATION: BUGCHECK_ERROR = BUGCHECK_ERROR(22u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CID_HANDLE_DELETION: BUGCHECK_ERROR = BUGCHECK_ERROR(23u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const REFERENCE_BY_POINTER: BUGCHECK_ERROR = BUGCHECK_ERROR(24u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BAD_POOL_HEADER: BUGCHECK_ERROR = BUGCHECK_ERROR(25u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MEMORY_MANAGEMENT: BUGCHECK_ERROR = BUGCHECK_ERROR(26u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PFN_SHARE_COUNT: BUGCHECK_ERROR = BUGCHECK_ERROR(27u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PFN_REFERENCE_COUNT: BUGCHECK_ERROR = BUGCHECK_ERROR(28u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const NO_SPIN_LOCK_AVAILABLE: BUGCHECK_ERROR = BUGCHECK_ERROR(29u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const KMODE_EXCEPTION_NOT_HANDLED: BUGCHECK_ERROR = BUGCHECK_ERROR(30u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SHARED_RESOURCE_CONV_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(31u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const KERNEL_APC_PENDING_DURING_EXIT: BUGCHECK_ERROR = BUGCHECK_ERROR(32u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const QUOTA_UNDERFLOW: BUGCHECK_ERROR = BUGCHECK_ERROR(33u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(34u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FAT_FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(35u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const NTFS_FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(36u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const NPFS_FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(37u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CDFS_FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(38u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const RDR_FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(39u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CORRUPT_ACCESS_TOKEN: BUGCHECK_ERROR = BUGCHECK_ERROR(40u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SECURITY_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(41u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INCONSISTENT_IRP: BUGCHECK_ERROR = BUGCHECK_ERROR(42u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PANIC_STACK_SWITCH: BUGCHECK_ERROR = BUGCHECK_ERROR(43u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PORT_DRIVER_INTERNAL: BUGCHECK_ERROR = BUGCHECK_ERROR(44u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SCSI_DISK_DRIVER_INTERNAL: BUGCHECK_ERROR = BUGCHECK_ERROR(45u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DATA_BUS_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(46u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INSTRUCTION_BUS_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(47u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SET_OF_INVALID_CONTEXT: BUGCHECK_ERROR = BUGCHECK_ERROR(48u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PHASE0_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(49u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PHASE1_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(50u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UNEXPECTED_INITIALIZATION_CALL: BUGCHECK_ERROR = BUGCHECK_ERROR(51u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CACHE_MANAGER: BUGCHECK_ERROR = BUGCHECK_ERROR(52u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const NO_MORE_IRP_STACK_LOCATIONS: BUGCHECK_ERROR = BUGCHECK_ERROR(53u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DEVICE_REFERENCE_COUNT_NOT_ZERO: BUGCHECK_ERROR = BUGCHECK_ERROR(54u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FLOPPY_INTERNAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(55u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SERIAL_DRIVER_INTERNAL: BUGCHECK_ERROR = BUGCHECK_ERROR(56u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYSTEM_EXIT_OWNED_MUTEX: BUGCHECK_ERROR = BUGCHECK_ERROR(57u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYSTEM_UNWIND_PREVIOUS_USER: BUGCHECK_ERROR = BUGCHECK_ERROR(58u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYSTEM_SERVICE_EXCEPTION: BUGCHECK_ERROR = BUGCHECK_ERROR(59u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INTERRUPT_UNWIND_ATTEMPTED: BUGCHECK_ERROR = BUGCHECK_ERROR(60u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INTERRUPT_EXCEPTION_NOT_HANDLED: BUGCHECK_ERROR = BUGCHECK_ERROR(61u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MULTIPROCESSOR_CONFIGURATION_NOT_SUPPORTED: BUGCHECK_ERROR = BUGCHECK_ERROR(62u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const NO_MORE_SYSTEM_PTES: BUGCHECK_ERROR = BUGCHECK_ERROR(63u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TARGET_MDL_TOO_SMALL: BUGCHECK_ERROR = BUGCHECK_ERROR(64u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MUST_SUCCEED_POOL_EMPTY: BUGCHECK_ERROR = BUGCHECK_ERROR(65u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ATDISK_DRIVER_INTERNAL: BUGCHECK_ERROR = BUGCHECK_ERROR(66u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const NO_SUCH_PARTITION: BUGCHECK_ERROR = BUGCHECK_ERROR(67u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MULTIPLE_IRP_COMPLETE_REQUESTS: BUGCHECK_ERROR = BUGCHECK_ERROR(68u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INSUFFICIENT_SYSTEM_MAP_REGS: BUGCHECK_ERROR = BUGCHECK_ERROR(69u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DEREF_UNKNOWN_LOGON_SESSION: BUGCHECK_ERROR = BUGCHECK_ERROR(70u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const REF_UNKNOWN_LOGON_SESSION: BUGCHECK_ERROR = BUGCHECK_ERROR(71u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CANCEL_STATE_IN_COMPLETED_IRP: BUGCHECK_ERROR = BUGCHECK_ERROR(72u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PAGE_FAULT_WITH_INTERRUPTS_OFF: BUGCHECK_ERROR = BUGCHECK_ERROR(73u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IRQL_GT_ZERO_AT_SYSTEM_SERVICE: BUGCHECK_ERROR = BUGCHECK_ERROR(74u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const STREAMS_INTERNAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(75u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FATAL_UNHANDLED_HARD_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(76u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const NO_PAGES_AVAILABLE: BUGCHECK_ERROR = BUGCHECK_ERROR(77u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PFN_LIST_CORRUPT: BUGCHECK_ERROR = BUGCHECK_ERROR(78u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const NDIS_INTERNAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(79u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PAGE_FAULT_IN_NONPAGED_AREA: BUGCHECK_ERROR = BUGCHECK_ERROR(80u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PAGE_FAULT_IN_NONPAGED_AREA_M: BUGCHECK_ERROR = BUGCHECK_ERROR(268435536u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const REGISTRY_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(81u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MAILSLOT_FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(82u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const NO_BOOT_DEVICE: BUGCHECK_ERROR = BUGCHECK_ERROR(83u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const LM_SERVER_INTERNAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(84u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DATA_COHERENCY_EXCEPTION: BUGCHECK_ERROR = BUGCHECK_ERROR(85u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INSTRUCTION_COHERENCY_EXCEPTION: BUGCHECK_ERROR = BUGCHECK_ERROR(86u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const XNS_INTERNAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(87u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const VOLMGRX_INTERNAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(88u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PINBALL_FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(89u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CRITICAL_SERVICE_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(90u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SET_ENV_VAR_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(91u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const HAL_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(92u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UNSUPPORTED_PROCESSOR: BUGCHECK_ERROR = BUGCHECK_ERROR(93u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(94u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SECURITY_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(95u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PROCESS_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(96u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const HAL1_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(97u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT1_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(98u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SECURITY1_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(99u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMBOLIC_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(100u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MEMORY1_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(101u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CACHE_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(102u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CONFIG_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(103u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FILE_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(104u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IO1_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(105u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const LPC_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(106u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PROCESS1_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(107u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const REFMON_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(108u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SESSION1_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(109u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BOOTPROC_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(110u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const VSL_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(111u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SOFT_RESTART_FATAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(112u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ASSIGN_DRIVE_LETTERS_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(114u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CONFIG_LIST_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(115u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BAD_SYSTEM_CONFIG_INFO: BUGCHECK_ERROR = BUGCHECK_ERROR(116u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CANNOT_WRITE_CONFIGURATION: BUGCHECK_ERROR = BUGCHECK_ERROR(117u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PROCESS_HAS_LOCKED_PAGES: BUGCHECK_ERROR = BUGCHECK_ERROR(118u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const KERNEL_STACK_INPAGE_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(119u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PHASE0_EXCEPTION: BUGCHECK_ERROR = BUGCHECK_ERROR(120u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MISMATCHED_HAL: BUGCHECK_ERROR = BUGCHECK_ERROR(121u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const KERNEL_DATA_INPAGE_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(122u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INACCESSIBLE_BOOT_DEVICE: BUGCHECK_ERROR = BUGCHECK_ERROR(123u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BUGCODE_NDIS_DRIVER: BUGCHECK_ERROR = BUGCHECK_ERROR(124u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INSTALL_MORE_MEMORY: BUGCHECK_ERROR = BUGCHECK_ERROR(125u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYSTEM_THREAD_EXCEPTION_NOT_HANDLED: BUGCHECK_ERROR = BUGCHECK_ERROR(126u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYSTEM_THREAD_EXCEPTION_NOT_HANDLED_M: BUGCHECK_ERROR = BUGCHECK_ERROR(268435582u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UNEXPECTED_KERNEL_MODE_TRAP: BUGCHECK_ERROR = BUGCHECK_ERROR(127u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UNEXPECTED_KERNEL_MODE_TRAP_M: BUGCHECK_ERROR = BUGCHECK_ERROR(268435583u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const NMI_HARDWARE_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(128u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SPIN_LOCK_INIT_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(129u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DFS_FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(130u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OFS_FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(131u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const RECOM_DRIVER: BUGCHECK_ERROR = BUGCHECK_ERROR(132u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SETUP_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(133u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const AUDIT_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(134u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MBR_CHECKSUM_MISMATCH: BUGCHECK_ERROR = BUGCHECK_ERROR(139u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const KERNEL_MODE_EXCEPTION_NOT_HANDLED: BUGCHECK_ERROR = BUGCHECK_ERROR(142u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const KERNEL_MODE_EXCEPTION_NOT_HANDLED_M: BUGCHECK_ERROR = BUGCHECK_ERROR(268435598u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PP0_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(143u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PP1_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(144u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WIN32K_INIT_OR_RIT_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(145u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UP_DRIVER_ON_MP_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(146u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INVALID_KERNEL_HANDLE: BUGCHECK_ERROR = BUGCHECK_ERROR(147u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const KERNEL_STACK_LOCKED_AT_EXIT: BUGCHECK_ERROR = BUGCHECK_ERROR(148u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PNP_INTERNAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(149u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INVALID_WORK_QUEUE_ITEM: BUGCHECK_ERROR = BUGCHECK_ERROR(150u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BOUND_IMAGE_UNSUPPORTED: BUGCHECK_ERROR = BUGCHECK_ERROR(151u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const END_OF_NT_EVALUATION_PERIOD: BUGCHECK_ERROR = BUGCHECK_ERROR(152u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INVALID_REGION_OR_SEGMENT: BUGCHECK_ERROR = BUGCHECK_ERROR(153u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYSTEM_LICENSE_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(154u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UDFS_FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(155u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MACHINE_CHECK_EXCEPTION: BUGCHECK_ERROR = BUGCHECK_ERROR(156u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const USER_MODE_HEALTH_MONITOR: BUGCHECK_ERROR = BUGCHECK_ERROR(158u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DRIVER_POWER_STATE_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(159u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INTERNAL_POWER_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(160u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PCI_BUS_DRIVER_INTERNAL: BUGCHECK_ERROR = BUGCHECK_ERROR(161u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MEMORY_IMAGE_CORRUPT: BUGCHECK_ERROR = BUGCHECK_ERROR(162u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ACPI_DRIVER_INTERNAL: BUGCHECK_ERROR = BUGCHECK_ERROR(163u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CNSS_FILE_SYSTEM_FILTER: BUGCHECK_ERROR = BUGCHECK_ERROR(164u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ACPI_BIOS_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(165u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FP_EMULATION_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(166u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BAD_EXHANDLE: BUGCHECK_ERROR = BUGCHECK_ERROR(167u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BOOTING_IN_SAFEMODE_MINIMAL: BUGCHECK_ERROR = BUGCHECK_ERROR(168u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BOOTING_IN_SAFEMODE_NETWORK: BUGCHECK_ERROR = BUGCHECK_ERROR(169u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BOOTING_IN_SAFEMODE_DSREPAIR: BUGCHECK_ERROR = BUGCHECK_ERROR(170u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SESSION_HAS_VALID_POOL_ON_EXIT: BUGCHECK_ERROR = BUGCHECK_ERROR(171u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const HAL_MEMORY_ALLOCATION: BUGCHECK_ERROR = BUGCHECK_ERROR(172u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const VIDEO_DRIVER_DEBUG_REPORT_REQUEST: BUGCHECK_ERROR = BUGCHECK_ERROR(1073741997u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BGI_DETECTED_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(177u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const VIDEO_DRIVER_INIT_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(180u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BOOTLOG_LOADED: BUGCHECK_ERROR = BUGCHECK_ERROR(181u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BOOTLOG_NOT_LOADED: BUGCHECK_ERROR = BUGCHECK_ERROR(182u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BOOTLOG_ENABLED: BUGCHECK_ERROR = BUGCHECK_ERROR(183u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ATTEMPTED_SWITCH_FROM_DPC: BUGCHECK_ERROR = BUGCHECK_ERROR(184u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CHIPSET_DETECTED_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(185u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SESSION_HAS_VALID_VIEWS_ON_EXIT: BUGCHECK_ERROR = BUGCHECK_ERROR(186u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const NETWORK_BOOT_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(187u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const NETWORK_BOOT_DUPLICATE_ADDRESS: BUGCHECK_ERROR = BUGCHECK_ERROR(188u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INVALID_HIBERNATED_STATE: BUGCHECK_ERROR = BUGCHECK_ERROR(189u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ATTEMPTED_WRITE_TO_READONLY_MEMORY: BUGCHECK_ERROR = BUGCHECK_ERROR(190u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MUTEX_ALREADY_OWNED: BUGCHECK_ERROR = BUGCHECK_ERROR(191u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PCI_CONFIG_SPACE_ACCESS_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(192u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SPECIAL_POOL_DETECTED_MEMORY_CORRUPTION: BUGCHECK_ERROR = BUGCHECK_ERROR(193u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BAD_POOL_CALLER: BUGCHECK_ERROR = BUGCHECK_ERROR(194u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYSTEM_IMAGE_BAD_SIGNATURE: BUGCHECK_ERROR = BUGCHECK_ERROR(195u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DRIVER_VERIFIER_DETECTED_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(196u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DRIVER_CORRUPTED_EXPOOL: BUGCHECK_ERROR = BUGCHECK_ERROR(197u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DRIVER_CAUGHT_MODIFYING_FREED_POOL: BUGCHECK_ERROR = BUGCHECK_ERROR(198u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TIMER_OR_DPC_INVALID: BUGCHECK_ERROR = BUGCHECK_ERROR(199u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IRQL_UNEXPECTED_VALUE: BUGCHECK_ERROR = BUGCHECK_ERROR(200u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DRIVER_VERIFIER_IOMANAGER_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(201u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PNP_DETECTED_FATAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(202u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DRIVER_LEFT_LOCKED_PAGES_IN_PROCESS: BUGCHECK_ERROR = BUGCHECK_ERROR(203u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PAGE_FAULT_IN_FREED_SPECIAL_POOL: BUGCHECK_ERROR = BUGCHECK_ERROR(204u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PAGE_FAULT_BEYOND_END_OF_ALLOCATION: BUGCHECK_ERROR = BUGCHECK_ERROR(205u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DRIVER_UNLOADED_WITHOUT_CANCELLING_PENDING_OPERATIONS: BUGCHECK_ERROR = BUGCHECK_ERROR(206u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TERMINAL_SERVER_DRIVER_MADE_INCORRECT_MEMORY_REFERENCE: BUGCHECK_ERROR = BUGCHECK_ERROR(207u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DRIVER_CORRUPTED_MMPOOL: BUGCHECK_ERROR = BUGCHECK_ERROR(208u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DRIVER_IRQL_NOT_LESS_OR_EQUAL: BUGCHECK_ERROR = BUGCHECK_ERROR(209u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BUGCODE_ID_DRIVER: BUGCHECK_ERROR = BUGCHECK_ERROR(210u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DRIVER_PORTION_MUST_BE_NONPAGED: BUGCHECK_ERROR = BUGCHECK_ERROR(211u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYSTEM_SCAN_AT_RAISED_IRQL_CAUGHT_IMPROPER_DRIVER_UNLOAD: BUGCHECK_ERROR = BUGCHECK_ERROR(212u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DRIVER_PAGE_FAULT_IN_FREED_SPECIAL_POOL: BUGCHECK_ERROR = BUGCHECK_ERROR(213u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DRIVER_PAGE_FAULT_BEYOND_END_OF_ALLOCATION: BUGCHECK_ERROR = BUGCHECK_ERROR(214u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DRIVER_PAGE_FAULT_BEYOND_END_OF_ALLOCATION_M: BUGCHECK_ERROR = BUGCHECK_ERROR(268435670u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DRIVER_UNMAPPING_INVALID_VIEW: BUGCHECK_ERROR = BUGCHECK_ERROR(215u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DRIVER_USED_EXCESSIVE_PTES: BUGCHECK_ERROR = BUGCHECK_ERROR(216u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const LOCKED_PAGES_TRACKER_CORRUPTION: BUGCHECK_ERROR = BUGCHECK_ERROR(217u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYSTEM_PTE_MISUSE: BUGCHECK_ERROR = BUGCHECK_ERROR(218u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DRIVER_CORRUPTED_SYSPTES: BUGCHECK_ERROR = BUGCHECK_ERROR(219u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DRIVER_INVALID_STACK_ACCESS: BUGCHECK_ERROR = BUGCHECK_ERROR(220u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const POOL_CORRUPTION_IN_FILE_AREA: BUGCHECK_ERROR = BUGCHECK_ERROR(222u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMPERSONATING_WORKER_THREAD: BUGCHECK_ERROR = BUGCHECK_ERROR(223u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ACPI_BIOS_FATAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(224u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WORKER_THREAD_RETURNED_AT_BAD_IRQL: BUGCHECK_ERROR = BUGCHECK_ERROR(225u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MANUALLY_INITIATED_CRASH: BUGCHECK_ERROR = BUGCHECK_ERROR(226u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const RESOURCE_NOT_OWNED: BUGCHECK_ERROR = BUGCHECK_ERROR(227u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WORKER_INVALID: BUGCHECK_ERROR = BUGCHECK_ERROR(228u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const POWER_FAILURE_SIMULATE: BUGCHECK_ERROR = BUGCHECK_ERROR(229u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DRIVER_VERIFIER_DMA_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(230u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INVALID_FLOATING_POINT_STATE: BUGCHECK_ERROR = BUGCHECK_ERROR(231u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INVALID_CANCEL_OF_FILE_OPEN: BUGCHECK_ERROR = BUGCHECK_ERROR(232u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ACTIVE_EX_WORKER_THREAD_TERMINATION: BUGCHECK_ERROR = BUGCHECK_ERROR(233u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_UNSPECIFIED: BUGCHECK_ERROR = BUGCHECK_ERROR(61440u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_BLANKSCREEN: BUGCHECK_ERROR = BUGCHECK_ERROR(61442u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_INPUT: BUGCHECK_ERROR = BUGCHECK_ERROR(61443u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_WATCHDOG: BUGCHECK_ERROR = BUGCHECK_ERROR(61444u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_STARTNOTVISIBLE: BUGCHECK_ERROR = BUGCHECK_ERROR(61445u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_NAVIGATIONMODEL: BUGCHECK_ERROR = BUGCHECK_ERROR(61446u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_OUTOFMEMORY: BUGCHECK_ERROR = BUGCHECK_ERROR(61447u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_GRAPHICS: BUGCHECK_ERROR = BUGCHECK_ERROR(61448u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_NAVSERVERTIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(61449u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_CHROMEPROCESSCRASH: BUGCHECK_ERROR = BUGCHECK_ERROR(61450u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_NOTIFICATIONDISMISSAL: BUGCHECK_ERROR = BUGCHECK_ERROR(61451u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_SPEECHDISMISSAL: BUGCHECK_ERROR = BUGCHECK_ERROR(61452u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_CALLDISMISSAL: BUGCHECK_ERROR = BUGCHECK_ERROR(61453u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_APPBARDISMISSAL: BUGCHECK_ERROR = BUGCHECK_ERROR(61454u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_RILADAPTATIONCRASH: BUGCHECK_ERROR = BUGCHECK_ERROR(61455u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_APPLISTUNREACHABLE: BUGCHECK_ERROR = BUGCHECK_ERROR(61456u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_REPORTNOTIFICATIONFAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(61457u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_UNEXPECTEDSHUTDOWN: BUGCHECK_ERROR = BUGCHECK_ERROR(61458u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_RPCFAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(61459u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_AUXILIARYFULLDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(61460u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_ACCOUNTPROVSVCINITFAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(61461u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_MTBFCOMMANDTIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(789u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_MTBFCOMMANDHANG: BUGCHECK_ERROR = BUGCHECK_ERROR(61697u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_MTBFPASSBUGCHECK: BUGCHECK_ERROR = BUGCHECK_ERROR(61698u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_MTBFIOERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(61699u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_RENDERTHREADHANG: BUGCHECK_ERROR = BUGCHECK_ERROR(61952u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_RENDERMOBILEUIOOM: BUGCHECK_ERROR = BUGCHECK_ERROR(61953u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_DEVICEUPDATEUNSPECIFIED: BUGCHECK_ERROR = BUGCHECK_ERROR(62208u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_AUDIODRIVERHANG: BUGCHECK_ERROR = BUGCHECK_ERROR(62464u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_BATTERYPULLOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(62720u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_MEDIACORETESTHANG: BUGCHECK_ERROR = BUGCHECK_ERROR(62976u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_RESOURCEMANAGEMENT: BUGCHECK_ERROR = BUGCHECK_ERROR(63232u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_CAPTURESERVICE: BUGCHECK_ERROR = BUGCHECK_ERROR(63488u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_WAITFORSHELLREADY: BUGCHECK_ERROR = BUGCHECK_ERROR(63744u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_NONRESPONSIVEPROCESS: BUGCHECK_ERROR = BUGCHECK_ERROR(404u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SAVER_SICKAPPLICATION: BUGCHECK_ERROR = BUGCHECK_ERROR(34918u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const THREAD_STUCK_IN_DEVICE_DRIVER: BUGCHECK_ERROR = BUGCHECK_ERROR(234u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const THREAD_STUCK_IN_DEVICE_DRIVER_M: BUGCHECK_ERROR = BUGCHECK_ERROR(268435690u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DIRTY_MAPPED_PAGES_CONGESTION: BUGCHECK_ERROR = BUGCHECK_ERROR(235u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SESSION_HAS_VALID_SPECIAL_POOL_ON_EXIT: BUGCHECK_ERROR = BUGCHECK_ERROR(236u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UNMOUNTABLE_BOOT_VOLUME: BUGCHECK_ERROR = BUGCHECK_ERROR(237u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CRITICAL_PROCESS_DIED: BUGCHECK_ERROR = BUGCHECK_ERROR(239u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const STORAGE_MINIPORT_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(240u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SCSI_VERIFIER_DETECTED_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(241u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const HARDWARE_INTERRUPT_STORM: BUGCHECK_ERROR = BUGCHECK_ERROR(242u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DISORDERLY_SHUTDOWN: BUGCHECK_ERROR = BUGCHECK_ERROR(243u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CRITICAL_OBJECT_TERMINATION: BUGCHECK_ERROR = BUGCHECK_ERROR(244u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FLTMGR_FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(245u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PCI_VERIFIER_DETECTED_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(246u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DRIVER_OVERRAN_STACK_BUFFER: BUGCHECK_ERROR = BUGCHECK_ERROR(247u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const RAMDISK_BOOT_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(248u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DRIVER_RETURNED_STATUS_REPARSE_FOR_VOLUME_OPEN: BUGCHECK_ERROR = BUGCHECK_ERROR(249u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const HTTP_DRIVER_CORRUPTED: BUGCHECK_ERROR = BUGCHECK_ERROR(250u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const RECURSIVE_MACHINE_CHECK: BUGCHECK_ERROR = BUGCHECK_ERROR(251u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ATTEMPTED_EXECUTE_OF_NOEXECUTE_MEMORY: BUGCHECK_ERROR = BUGCHECK_ERROR(252u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DIRTY_NOWRITE_PAGES_CONGESTION: BUGCHECK_ERROR = BUGCHECK_ERROR(253u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BUGCODE_USB_DRIVER: BUGCHECK_ERROR = BUGCHECK_ERROR(254u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BC_BLUETOOTH_VERIFIER_FAULT: BUGCHECK_ERROR = BUGCHECK_ERROR(3070u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BC_BTHMINI_VERIFIER_FAULT: BUGCHECK_ERROR = BUGCHECK_ERROR(3071u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const RESERVE_QUEUE_OVERFLOW: BUGCHECK_ERROR = BUGCHECK_ERROR(255u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const LOADER_BLOCK_MISMATCH: BUGCHECK_ERROR = BUGCHECK_ERROR(256u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CLOCK_WATCHDOG_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(257u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DPC_WATCHDOG_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(258u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MUP_FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(259u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const AGP_INVALID_ACCESS: BUGCHECK_ERROR = BUGCHECK_ERROR(260u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const AGP_GART_CORRUPTION: BUGCHECK_ERROR = BUGCHECK_ERROR(261u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const AGP_ILLEGALLY_REPROGRAMMED: BUGCHECK_ERROR = BUGCHECK_ERROR(262u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const KERNEL_EXPAND_STACK_ACTIVE: BUGCHECK_ERROR = BUGCHECK_ERROR(263u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const THIRD_PARTY_FILE_SYSTEM_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(264u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CRITICAL_STRUCTURE_CORRUPTION: BUGCHECK_ERROR = BUGCHECK_ERROR(265u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const APP_TAGGING_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(266u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DFSC_FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(267u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FSRTL_EXTRA_CREATE_PARAMETER_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(268u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WDF_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(269u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const VIDEO_MEMORY_MANAGEMENT_INTERNAL: BUGCHECK_ERROR = BUGCHECK_ERROR(270u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DRIVER_INVALID_CRUNTIME_PARAMETER: BUGCHECK_ERROR = BUGCHECK_ERROR(272u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const RECURSIVE_NMI: BUGCHECK_ERROR = BUGCHECK_ERROR(273u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MSRPC_STATE_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(274u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const VIDEO_DXGKRNL_FATAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(275u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const VIDEO_SHADOW_DRIVER_FATAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(276u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const AGP_INTERNAL: BUGCHECK_ERROR = BUGCHECK_ERROR(277u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const VIDEO_TDR_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(278u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const VIDEO_TDR_TIMEOUT_DETECTED: BUGCHECK_ERROR = BUGCHECK_ERROR(279u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const NTHV_GUEST_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(280u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const VIDEO_SCHEDULER_INTERNAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(281u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const EM_INITIALIZATION_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(282u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DRIVER_RETURNED_HOLDING_CANCEL_LOCK: BUGCHECK_ERROR = BUGCHECK_ERROR(283u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ATTEMPTED_WRITE_TO_CM_PROTECTED_STORAGE: BUGCHECK_ERROR = BUGCHECK_ERROR(284u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const EVENT_TRACING_FATAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(285u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TOO_MANY_RECURSIVE_FAULTS: BUGCHECK_ERROR = BUGCHECK_ERROR(286u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INVALID_DRIVER_HANDLE: BUGCHECK_ERROR = BUGCHECK_ERROR(287u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BITLOCKER_FATAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(288u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DRIVER_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(289u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_INTERNAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(290u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CRYPTO_SELF_TEST_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(291u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WHEA_UNCORRECTABLE_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(292u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const NMR_INVALID_STATE: BUGCHECK_ERROR = BUGCHECK_ERROR(293u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const NETIO_INVALID_POOL_CALLER: BUGCHECK_ERROR = BUGCHECK_ERROR(294u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PAGE_NOT_ZERO: BUGCHECK_ERROR = BUGCHECK_ERROR(295u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WORKER_THREAD_RETURNED_WITH_BAD_IO_PRIORITY: BUGCHECK_ERROR = BUGCHECK_ERROR(296u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WORKER_THREAD_RETURNED_WITH_BAD_PAGING_IO_PRIORITY: BUGCHECK_ERROR = BUGCHECK_ERROR(297u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MUI_NO_VALID_SYSTEM_LANGUAGE: BUGCHECK_ERROR = BUGCHECK_ERROR(298u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FAULTY_HARDWARE_CORRUPTED_PAGE: BUGCHECK_ERROR = BUGCHECK_ERROR(299u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const EXFAT_FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(300u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const VOLSNAP_OVERLAPPED_TABLE_ACCESS: BUGCHECK_ERROR = BUGCHECK_ERROR(301u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INVALID_MDL_RANGE: BUGCHECK_ERROR = BUGCHECK_ERROR(302u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const VHD_BOOT_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(303u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DYNAMIC_ADD_PROCESSOR_MISMATCH: BUGCHECK_ERROR = BUGCHECK_ERROR(304u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INVALID_EXTENDED_PROCESSOR_STATE: BUGCHECK_ERROR = BUGCHECK_ERROR(305u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const RESOURCE_OWNER_POINTER_INVALID: BUGCHECK_ERROR = BUGCHECK_ERROR(306u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DPC_WATCHDOG_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(307u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DRIVE_EXTENDER: BUGCHECK_ERROR = BUGCHECK_ERROR(308u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const REGISTRY_FILTER_DRIVER_EXCEPTION: BUGCHECK_ERROR = BUGCHECK_ERROR(309u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const VHD_BOOT_HOST_VOLUME_NOT_ENOUGH_SPACE: BUGCHECK_ERROR = BUGCHECK_ERROR(310u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WIN32K_HANDLE_MANAGER: BUGCHECK_ERROR = BUGCHECK_ERROR(311u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const GPIO_CONTROLLER_DRIVER_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(312u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const KERNEL_SECURITY_CHECK_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(313u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const KERNEL_MODE_HEAP_CORRUPTION: BUGCHECK_ERROR = BUGCHECK_ERROR(314u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PASSIVE_INTERRUPT_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(315u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INVALID_IO_BOOST_STATE: BUGCHECK_ERROR = BUGCHECK_ERROR(316u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CRITICAL_INITIALIZATION_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(317u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ERRATA_WORKAROUND_UNSUCCESSFUL: BUGCHECK_ERROR = BUGCHECK_ERROR(318u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const REGISTRY_CALLBACK_DRIVER_EXCEPTION: BUGCHECK_ERROR = BUGCHECK_ERROR(319u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const STORAGE_DEVICE_ABNORMALITY_DETECTED: BUGCHECK_ERROR = BUGCHECK_ERROR(320u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const VIDEO_ENGINE_TIMEOUT_DETECTED: BUGCHECK_ERROR = BUGCHECK_ERROR(321u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const VIDEO_TDR_APPLICATION_BLOCKED: BUGCHECK_ERROR = BUGCHECK_ERROR(322u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PROCESSOR_DRIVER_INTERNAL: BUGCHECK_ERROR = BUGCHECK_ERROR(323u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BUGCODE_USB3_DRIVER: BUGCHECK_ERROR = BUGCHECK_ERROR(324u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SECURE_BOOT_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(325u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const NDIS_NET_BUFFER_LIST_INFO_ILLEGALLY_TRANSFERRED: BUGCHECK_ERROR = BUGCHECK_ERROR(326u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ABNORMAL_RESET_DETECTED: BUGCHECK_ERROR = BUGCHECK_ERROR(327u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IO_OBJECT_INVALID: BUGCHECK_ERROR = BUGCHECK_ERROR(328u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const REFS_FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(329u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const KERNEL_WMI_INTERNAL: BUGCHECK_ERROR = BUGCHECK_ERROR(330u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SOC_SUBSYSTEM_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(331u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FATAL_ABNORMAL_RESET_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(332u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const EXCEPTION_SCOPE_INVALID: BUGCHECK_ERROR = BUGCHECK_ERROR(333u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SOC_CRITICAL_DEVICE_REMOVED: BUGCHECK_ERROR = BUGCHECK_ERROR(334u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PDC_WATCHDOG_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(335u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TCPIP_AOAC_NIC_ACTIVE_REFERENCE_LEAK: BUGCHECK_ERROR = BUGCHECK_ERROR(336u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UNSUPPORTED_INSTRUCTION_MODE: BUGCHECK_ERROR = BUGCHECK_ERROR(337u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INVALID_PUSH_LOCK_FLAGS: BUGCHECK_ERROR = BUGCHECK_ERROR(338u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const KERNEL_LOCK_ENTRY_LEAKED_ON_THREAD_TERMINATION: BUGCHECK_ERROR = BUGCHECK_ERROR(339u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UNEXPECTED_STORE_EXCEPTION: BUGCHECK_ERROR = BUGCHECK_ERROR(340u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OS_DATA_TAMPERING: BUGCHECK_ERROR = BUGCHECK_ERROR(341u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WINSOCK_DETECTED_HUNG_CLOSESOCKET_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(342u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const KERNEL_THREAD_PRIORITY_FLOOR_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(343u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ILLEGAL_IOMMU_PAGE_FAULT: BUGCHECK_ERROR = BUGCHECK_ERROR(344u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const HAL_ILLEGAL_IOMMU_PAGE_FAULT: BUGCHECK_ERROR = BUGCHECK_ERROR(345u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SDBUS_INTERNAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(346u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WORKER_THREAD_RETURNED_WITH_SYSTEM_PAGE_PRIORITY_ACTIVE: BUGCHECK_ERROR = BUGCHECK_ERROR(347u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PDC_WATCHDOG_TIMEOUT_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(348u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SOC_SUBSYSTEM_FAILURE_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(349u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BUGCODE_NDIS_DRIVER_LIVE_DUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(350u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CONNECTED_STANDBY_WATCHDOG_TIMEOUT_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(351u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WIN32K_ATOMIC_CHECK_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(352u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const LIVE_SYSTEM_DUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(353u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const KERNEL_AUTO_BOOST_INVALID_LOCK_RELEASE: BUGCHECK_ERROR = BUGCHECK_ERROR(354u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WORKER_THREAD_TEST_CONDITION: BUGCHECK_ERROR = BUGCHECK_ERROR(355u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WIN32K_CRITICAL_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(356u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CLUSTER_CSV_STATUS_IO_TIMEOUT_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(357u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CLUSTER_RESOURCE_CALL_TIMEOUT_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(358u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CLUSTER_CSV_SNAPSHOT_DEVICE_INFO_TIMEOUT_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(359u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CLUSTER_CSV_STATE_TRANSITION_TIMEOUT_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(360u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CLUSTER_CSV_VOLUME_ARRIVAL_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(361u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CLUSTER_CSV_VOLUME_REMOVAL_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(362u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CLUSTER_CSV_CLUSTER_WATCHDOG_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(363u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INVALID_RUNDOWN_PROTECTION_FLAGS: BUGCHECK_ERROR = BUGCHECK_ERROR(364u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INVALID_SLOT_ALLOCATOR_FLAGS: BUGCHECK_ERROR = BUGCHECK_ERROR(365u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ERESOURCE_INVALID_RELEASE: BUGCHECK_ERROR = BUGCHECK_ERROR(366u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CLUSTER_CSV_STATE_TRANSITION_INTERVAL_TIMEOUT_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(367u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CLUSTER_CSV_CLUSSVC_DISCONNECT_WATCHDOG: BUGCHECK_ERROR = BUGCHECK_ERROR(368u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CRYPTO_LIBRARY_INTERNAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(369u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const COREMSGCALL_INTERNAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(371u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const COREMSG_INTERNAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(372u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PREVIOUS_FATAL_ABNORMAL_RESET_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(373u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ELAM_DRIVER_DETECTED_FATAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(376u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CLUSTER_CLUSPORT_STATUS_IO_TIMEOUT_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(377u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PROFILER_CONFIGURATION_ILLEGAL: BUGCHECK_ERROR = BUGCHECK_ERROR(379u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PDC_LOCK_WATCHDOG_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(380u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PDC_UNEXPECTED_REVOCATION_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(381u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MICROCODE_REVISION_MISMATCH: BUGCHECK_ERROR = BUGCHECK_ERROR(382u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const HYPERGUARD_INITIALIZATION_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(383u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WVR_LIVEDUMP_REPLICATION_IOCONTEXT_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(384u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WVR_LIVEDUMP_STATE_TRANSITION_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(385u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WVR_LIVEDUMP_RECOVERY_IOCONTEXT_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(386u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WVR_LIVEDUMP_APP_IO_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(387u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WVR_LIVEDUMP_MANUALLY_INITIATED: BUGCHECK_ERROR = BUGCHECK_ERROR(388u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WVR_LIVEDUMP_STATE_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(389u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WVR_LIVEDUMP_CRITICAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(390u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const VIDEO_DWMINIT_TIMEOUT_FALLBACK_BDD: BUGCHECK_ERROR = BUGCHECK_ERROR(391u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CLUSTER_CSVFS_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(392u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BAD_OBJECT_HEADER: BUGCHECK_ERROR = BUGCHECK_ERROR(393u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SILO_CORRUPT: BUGCHECK_ERROR = BUGCHECK_ERROR(394u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SECURE_KERNEL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(395u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const HYPERGUARD_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(396u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SECURE_FAULT_UNHANDLED: BUGCHECK_ERROR = BUGCHECK_ERROR(397u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const KERNEL_PARTITION_REFERENCE_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(398u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYNTHETIC_EXCEPTION_UNHANDLED: BUGCHECK_ERROR = BUGCHECK_ERROR(399u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WIN32K_CRITICAL_FAILURE_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(400u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PF_DETECTED_CORRUPTION: BUGCHECK_ERROR = BUGCHECK_ERROR(401u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const KERNEL_AUTO_BOOST_LOCK_ACQUISITION_WITH_RAISED_IRQL: BUGCHECK_ERROR = BUGCHECK_ERROR(402u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const VIDEO_DXGKRNL_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(403u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const KERNEL_STORAGE_SLOT_IN_USE: BUGCHECK_ERROR = BUGCHECK_ERROR(409u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SMB_SERVER_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(405u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const LOADER_ROLLBACK_DETECTED: BUGCHECK_ERROR = BUGCHECK_ERROR(406u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WIN32K_SECURITY_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(407u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UFX_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(408u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WORKER_THREAD_RETURNED_WHILE_ATTACHED_TO_SILO: BUGCHECK_ERROR = BUGCHECK_ERROR(410u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TTM_FATAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(411u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WIN32K_POWER_WATCHDOG_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(412u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CLUSTER_SVHDX_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(413u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BUGCODE_NETADAPTER_DRIVER: BUGCHECK_ERROR = BUGCHECK_ERROR(414u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PDC_PRIVILEGE_CHECK_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(415u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TTM_WATCHDOG_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(416u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WIN32K_CALLOUT_WATCHDOG_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(417u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WIN32K_CALLOUT_WATCHDOG_BUGCHECK: BUGCHECK_ERROR = BUGCHECK_ERROR(418u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CALL_HAS_NOT_RETURNED_WATCHDOG_TIMEOUT_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(419u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DRIPS_SW_HW_DIVERGENCE_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(420u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const USB_DRIPS_BLOCKER_SURPRISE_REMOVAL_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(421u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BLUETOOTH_ERROR_RECOVERY_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(422u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SMB_REDIRECTOR_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(423u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const VIDEO_DXGKRNL_BLACK_SCREEN_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(424u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DIRECTED_FX_TRANSITION_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(425u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const EXCEPTION_ON_INVALID_STACK: BUGCHECK_ERROR = BUGCHECK_ERROR(426u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UNWIND_ON_INVALID_STACK: BUGCHECK_ERROR = BUGCHECK_ERROR(427u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const VIDEO_MINIPORT_FAILED_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(432u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const VIDEO_MINIPORT_BLACK_SCREEN_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(440u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DRIVER_VERIFIER_DETECTED_VIOLATION_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(452u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IO_THREADPOOL_DEADLOCK_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(453u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FAST_ERESOURCE_PRECONDITION_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(454u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const STORE_DATA_STRUCTURE_CORRUPTION: BUGCHECK_ERROR = BUGCHECK_ERROR(455u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MANUALLY_INITIATED_POWER_BUTTON_HOLD: BUGCHECK_ERROR = BUGCHECK_ERROR(456u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const USER_MODE_HEALTH_MONITOR_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(457u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYNTHETIC_WATCHDOG_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(458u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INVALID_SILO_DETACH: BUGCHECK_ERROR = BUGCHECK_ERROR(459u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const EXRESOURCE_TIMEOUT_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(460u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INVALID_CALLBACK_STACK_ADDRESS: BUGCHECK_ERROR = BUGCHECK_ERROR(461u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INVALID_KERNEL_STACK_ADDRESS: BUGCHECK_ERROR = BUGCHECK_ERROR(462u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const HARDWARE_WATCHDOG_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(463u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ACPI_FIRMWARE_WATCHDOG_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(464u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TELEMETRY_ASSERTS_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(465u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WORKER_THREAD_INVALID_STATE: BUGCHECK_ERROR = BUGCHECK_ERROR(466u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WFP_INVALID_OPERATION: BUGCHECK_ERROR = BUGCHECK_ERROR(467u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UCMUCSI_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(468u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DRIVER_PNP_WATCHDOG: BUGCHECK_ERROR = BUGCHECK_ERROR(469u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WORKER_THREAD_RETURNED_WITH_NON_DEFAULT_WORKLOAD_CLASS: BUGCHECK_ERROR = BUGCHECK_ERROR(470u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const EFS_FATAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(471u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UCMUCSI_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(472u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const HAL_IOMMU_INTERNAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(473u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const HAL_BLOCKED_PROCESSOR_INTERNAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(474u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IPI_WATCHDOG_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(475u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DMA_COMMON_BUFFER_VECTOR_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(476u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BUGCODE_MBBADAPTER_DRIVER: BUGCHECK_ERROR = BUGCHECK_ERROR(477u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BUGCODE_WIFIADAPTER_DRIVER: BUGCHECK_ERROR = BUGCHECK_ERROR(478u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PROCESSOR_START_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(479u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INVALID_ALTERNATE_SYSTEM_CALL_HANDLER_REGISTRATION: BUGCHECK_ERROR = BUGCHECK_ERROR(480u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DEVICE_DIAGNOSTIC_LOG_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(481u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const AZURE_DEVICE_FW_DUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(482u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BREAKAWAY_CABLE_TRANSITION: BUGCHECK_ERROR = BUGCHECK_ERROR(483u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const VIDEO_DXGKRNL_SYSMM_FATAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(484u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DRIVER_VERIFIER_TRACKING_LIVE_DUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(485u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CRASHDUMP_WATCHDOG_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(486u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const REGISTRY_LIVE_DUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(487u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const INVALID_THREAD_AFFINITY_STATE: BUGCHECK_ERROR = BUGCHECK_ERROR(488u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ILLEGAL_ATS_INITIALIZATION: BUGCHECK_ERROR = BUGCHECK_ERROR(489u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SECURE_PCI_CONFIG_SPACE_ACCESS_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(490u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DAM_WATCHDOG_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(491u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const HANDLE_LIVE_DUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(492u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const HANDLE_ERROR_ON_CRITICAL_THREAD: BUGCHECK_ERROR = BUGCHECK_ERROR(493u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MPSDRV_QUERY_USER: BUGCHECK_ERROR = BUGCHECK_ERROR(1073742318u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const VMBUS_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(1073742319u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const USB4_HARDWARE_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(496u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const KASAN_ENLIGHTENMENT_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(497u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const KASAN_ILLEGAL_ACCESS: BUGCHECK_ERROR = BUGCHECK_ERROR(498u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IORING: BUGCHECK_ERROR = BUGCHECK_ERROR(499u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MDL_CACHE: BUGCHECK_ERROR = BUGCHECK_ERROR(500u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MISALIGNED_POINTER_PARAMETER: BUGCHECK_ERROR = BUGCHECK_ERROR(502u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const XBOX_VMCTRL_CS_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(854u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const XBOX_CORRUPTED_IMAGE: BUGCHECK_ERROR = BUGCHECK_ERROR(855u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const XBOX_INVERTED_FUNCTION_TABLE_OVERFLOW: BUGCHECK_ERROR = BUGCHECK_ERROR(856u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const XBOX_CORRUPTED_IMAGE_BASE: BUGCHECK_ERROR = BUGCHECK_ERROR(857u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const XBOX_XDS_WATCHDOG_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(858u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const XBOX_SHUTDOWN_WATCHDOG_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(859u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const XBOX_360_SYSTEM_CRASH: BUGCHECK_ERROR = BUGCHECK_ERROR(864u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const XBOX_360_SYSTEM_CRASH_RESERVED: BUGCHECK_ERROR = BUGCHECK_ERROR(1056u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const XBOX_SECURITY_FAILUE: BUGCHECK_ERROR = BUGCHECK_ERROR(1057u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const KERNEL_CFG_INIT_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(1058u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MANUALLY_INITIATED_POWER_BUTTON_HOLD_LIVE_DUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(4552u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const HYPERVISOR_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(131073u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const XBOX_MANUALLY_INITIATED_CRASH: BUGCHECK_ERROR = BUGCHECK_ERROR(196614u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MANUALLY_INITIATED_BLACKSCREEN_HOTKEY_LIVE_DUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(8648u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WINLOGON_FATAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(3221226010u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MANUALLY_INITIATED_CRASH1: BUGCHECK_ERROR = BUGCHECK_ERROR(3735936685u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BUGCHECK_CONTEXT_MODIFIER: BUGCHECK_ERROR = BUGCHECK_ERROR(2147483648u32);
impl ::core::marker::Copy for BUGCHECK_ERROR {}
impl ::core::clone::Clone for BUGCHECK_ERROR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BUGCHECK_ERROR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for BUGCHECK_ERROR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for BUGCHECK_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BUGCHECK_ERROR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DBGPROP_ATTRIB_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_ATTRIB_NO_ATTRIB: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_ATTRIB_VALUE_IS_INVALID: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_ATTRIB_VALUE_IS_EXPANDABLE: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_ATTRIB_VALUE_IS_FAKE: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_ATTRIB_VALUE_IS_METHOD: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(256i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_ATTRIB_VALUE_IS_EVENT: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(512i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_ATTRIB_VALUE_IS_RAW_STRING: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(1024i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_ATTRIB_VALUE_READONLY: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(2048i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_ATTRIB_ACCESS_PUBLIC: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(4096i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_ATTRIB_ACCESS_PRIVATE: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(8192i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_ATTRIB_ACCESS_PROTECTED: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(16384i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_ATTRIB_ACCESS_FINAL: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(32768i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_ATTRIB_STORAGE_GLOBAL: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(65536i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_ATTRIB_STORAGE_STATIC: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(131072i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_ATTRIB_STORAGE_FIELD: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(262144i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_ATTRIB_STORAGE_VIRTUAL: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(524288i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_ATTRIB_TYPE_IS_CONSTANT: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(1048576i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_ATTRIB_TYPE_IS_SYNCHRONIZED: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(2097152i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_ATTRIB_TYPE_IS_VOLATILE: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(4194304i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_ATTRIB_HAS_EXTENDED_ATTRIBS: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(8388608i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_ATTRIB_FRAME_INTRYBLOCK: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(16777216i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_ATTRIB_FRAME_INCATCHBLOCK: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(33554432i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_ATTRIB_FRAME_INFINALLYBLOCK: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(67108864i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_ATTRIB_VALUE_IS_RETURN_VALUE: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(134217728i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_ATTRIB_VALUE_PENDING_MUTATION: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(268435456i32);
impl ::core::marker::Copy for DBGPROP_ATTRIB_FLAGS {}
impl ::core::clone::Clone for DBGPROP_ATTRIB_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DBGPROP_ATTRIB_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DBGPROP_ATTRIB_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DBGPROP_ATTRIB_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBGPROP_ATTRIB_FLAGS").field(&self.0).finish()
    }
}
impl DBGPROP_ATTRIB_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for DBGPROP_ATTRIB_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DBGPROP_ATTRIB_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DBGPROP_ATTRIB_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DBGPROP_ATTRIB_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DBGPROP_ATTRIB_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DBGPROP_INFO(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_INFO_NAME: DBGPROP_INFO = DBGPROP_INFO(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_INFO_TYPE: DBGPROP_INFO = DBGPROP_INFO(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_INFO_VALUE: DBGPROP_INFO = DBGPROP_INFO(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_INFO_FULLNAME: DBGPROP_INFO = DBGPROP_INFO(32i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_INFO_ATTRIBUTES: DBGPROP_INFO = DBGPROP_INFO(8i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_INFO_DEBUGPROP: DBGPROP_INFO = DBGPROP_INFO(16i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_INFO_BEAUTIFY: DBGPROP_INFO = DBGPROP_INFO(33554432i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_INFO_CALLTOSTRING: DBGPROP_INFO = DBGPROP_INFO(67108864i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBGPROP_INFO_AUTOEXPAND: DBGPROP_INFO = DBGPROP_INFO(134217728i32);
impl ::core::marker::Copy for DBGPROP_INFO {}
impl ::core::clone::Clone for DBGPROP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DBGPROP_INFO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DBGPROP_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DBGPROP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBGPROP_INFO").field(&self.0).finish()
    }
}
impl DBGPROP_INFO {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for DBGPROP_INFO {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DBGPROP_INFO {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DBGPROP_INFO {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DBGPROP_INFO {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DBGPROP_INFO {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DEBUG_EVENT_CODE(pub u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CREATE_PROCESS_DEBUG_EVENT: DEBUG_EVENT_CODE = DEBUG_EVENT_CODE(3u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CREATE_THREAD_DEBUG_EVENT: DEBUG_EVENT_CODE = DEBUG_EVENT_CODE(2u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const EXCEPTION_DEBUG_EVENT: DEBUG_EVENT_CODE = DEBUG_EVENT_CODE(1u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const EXIT_PROCESS_DEBUG_EVENT: DEBUG_EVENT_CODE = DEBUG_EVENT_CODE(5u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const EXIT_THREAD_DEBUG_EVENT: DEBUG_EVENT_CODE = DEBUG_EVENT_CODE(4u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const LOAD_DLL_DEBUG_EVENT: DEBUG_EVENT_CODE = DEBUG_EVENT_CODE(6u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OUTPUT_DEBUG_STRING_EVENT: DEBUG_EVENT_CODE = DEBUG_EVENT_CODE(8u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const RIP_EVENT: DEBUG_EVENT_CODE = DEBUG_EVENT_CODE(9u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UNLOAD_DLL_DEBUG_EVENT: DEBUG_EVENT_CODE = DEBUG_EVENT_CODE(7u32);
impl ::core::marker::Copy for DEBUG_EVENT_CODE {}
impl ::core::clone::Clone for DEBUG_EVENT_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEBUG_EVENT_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DEBUG_EVENT_CODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DEBUG_EVENT_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEBUG_EVENT_CODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DUMP_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DUMP_TYPE_INVALID: DUMP_TYPE = DUMP_TYPE(-1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DUMP_TYPE_UNKNOWN: DUMP_TYPE = DUMP_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DUMP_TYPE_FULL: DUMP_TYPE = DUMP_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DUMP_TYPE_SUMMARY: DUMP_TYPE = DUMP_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DUMP_TYPE_HEADER: DUMP_TYPE = DUMP_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DUMP_TYPE_TRIAGE: DUMP_TYPE = DUMP_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DUMP_TYPE_BITMAP_FULL: DUMP_TYPE = DUMP_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DUMP_TYPE_BITMAP_KERNEL: DUMP_TYPE = DUMP_TYPE(6i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DUMP_TYPE_AUTOMATIC: DUMP_TYPE = DUMP_TYPE(7i32);
impl ::core::marker::Copy for DUMP_TYPE {}
impl ::core::clone::Clone for DUMP_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DUMP_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DUMP_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DUMP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DUMP_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EX_PROP_INFO_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const EX_PROP_INFO_ID: EX_PROP_INFO_FLAGS = EX_PROP_INFO_FLAGS(256i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const EX_PROP_INFO_NTYPE: EX_PROP_INFO_FLAGS = EX_PROP_INFO_FLAGS(512i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const EX_PROP_INFO_NVALUE: EX_PROP_INFO_FLAGS = EX_PROP_INFO_FLAGS(1024i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const EX_PROP_INFO_LOCKBYTES: EX_PROP_INFO_FLAGS = EX_PROP_INFO_FLAGS(2048i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const EX_PROP_INFO_DEBUGEXTPROP: EX_PROP_INFO_FLAGS = EX_PROP_INFO_FLAGS(4096i32);
impl ::core::marker::Copy for EX_PROP_INFO_FLAGS {}
impl ::core::clone::Clone for EX_PROP_INFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EX_PROP_INFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EX_PROP_INFO_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EX_PROP_INFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EX_PROP_INFO_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FACILITY_CODE(pub u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_NULL: FACILITY_CODE = FACILITY_CODE(0u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_RPC: FACILITY_CODE = FACILITY_CODE(1u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DISPATCH: FACILITY_CODE = FACILITY_CODE(2u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_STORAGE: FACILITY_CODE = FACILITY_CODE(3u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_ITF: FACILITY_CODE = FACILITY_CODE(4u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_WIN32: FACILITY_CODE = FACILITY_CODE(7u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_WINDOWS: FACILITY_CODE = FACILITY_CODE(8u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_SSPI: FACILITY_CODE = FACILITY_CODE(9u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_SECURITY: FACILITY_CODE = FACILITY_CODE(9u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_CONTROL: FACILITY_CODE = FACILITY_CODE(10u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_CERT: FACILITY_CODE = FACILITY_CODE(11u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_INTERNET: FACILITY_CODE = FACILITY_CODE(12u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_MEDIASERVER: FACILITY_CODE = FACILITY_CODE(13u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_MSMQ: FACILITY_CODE = FACILITY_CODE(14u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_SETUPAPI: FACILITY_CODE = FACILITY_CODE(15u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_SCARD: FACILITY_CODE = FACILITY_CODE(16u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_COMPLUS: FACILITY_CODE = FACILITY_CODE(17u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_AAF: FACILITY_CODE = FACILITY_CODE(18u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_URT: FACILITY_CODE = FACILITY_CODE(19u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_ACS: FACILITY_CODE = FACILITY_CODE(20u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DPLAY: FACILITY_CODE = FACILITY_CODE(21u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_UMI: FACILITY_CODE = FACILITY_CODE(22u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_SXS: FACILITY_CODE = FACILITY_CODE(23u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_WINDOWS_CE: FACILITY_CODE = FACILITY_CODE(24u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_HTTP: FACILITY_CODE = FACILITY_CODE(25u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_USERMODE_COMMONLOG: FACILITY_CODE = FACILITY_CODE(26u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_WER: FACILITY_CODE = FACILITY_CODE(27u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_USERMODE_FILTER_MANAGER: FACILITY_CODE = FACILITY_CODE(31u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_BACKGROUNDCOPY: FACILITY_CODE = FACILITY_CODE(32u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_CONFIGURATION: FACILITY_CODE = FACILITY_CODE(33u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_WIA: FACILITY_CODE = FACILITY_CODE(33u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_STATE_MANAGEMENT: FACILITY_CODE = FACILITY_CODE(34u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_METADIRECTORY: FACILITY_CODE = FACILITY_CODE(35u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_WINDOWSUPDATE: FACILITY_CODE = FACILITY_CODE(36u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DIRECTORYSERVICE: FACILITY_CODE = FACILITY_CODE(37u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_GRAPHICS: FACILITY_CODE = FACILITY_CODE(38u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_SHELL: FACILITY_CODE = FACILITY_CODE(39u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_NAP: FACILITY_CODE = FACILITY_CODE(39u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_TPM_SERVICES: FACILITY_CODE = FACILITY_CODE(40u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_TPM_SOFTWARE: FACILITY_CODE = FACILITY_CODE(41u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_UI: FACILITY_CODE = FACILITY_CODE(42u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_XAML: FACILITY_CODE = FACILITY_CODE(43u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_ACTION_QUEUE: FACILITY_CODE = FACILITY_CODE(44u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_PLA: FACILITY_CODE = FACILITY_CODE(48u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_WINDOWS_SETUP: FACILITY_CODE = FACILITY_CODE(48u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_FVE: FACILITY_CODE = FACILITY_CODE(49u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_FWP: FACILITY_CODE = FACILITY_CODE(50u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_WINRM: FACILITY_CODE = FACILITY_CODE(51u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_NDIS: FACILITY_CODE = FACILITY_CODE(52u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_USERMODE_HYPERVISOR: FACILITY_CODE = FACILITY_CODE(53u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_CMI: FACILITY_CODE = FACILITY_CODE(54u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_USERMODE_VIRTUALIZATION: FACILITY_CODE = FACILITY_CODE(55u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_USERMODE_VOLMGR: FACILITY_CODE = FACILITY_CODE(56u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_BCD: FACILITY_CODE = FACILITY_CODE(57u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_USERMODE_VHD: FACILITY_CODE = FACILITY_CODE(58u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_USERMODE_HNS: FACILITY_CODE = FACILITY_CODE(59u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_SDIAG: FACILITY_CODE = FACILITY_CODE(60u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_WEBSERVICES: FACILITY_CODE = FACILITY_CODE(61u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_WINPE: FACILITY_CODE = FACILITY_CODE(61u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_WPN: FACILITY_CODE = FACILITY_CODE(62u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_WINDOWS_STORE: FACILITY_CODE = FACILITY_CODE(63u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_INPUT: FACILITY_CODE = FACILITY_CODE(64u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_QUIC: FACILITY_CODE = FACILITY_CODE(65u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_EAP: FACILITY_CODE = FACILITY_CODE(66u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_IORING: FACILITY_CODE = FACILITY_CODE(70u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_WINDOWS_DEFENDER: FACILITY_CODE = FACILITY_CODE(80u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_OPC: FACILITY_CODE = FACILITY_CODE(81u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_XPS: FACILITY_CODE = FACILITY_CODE(82u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_MBN: FACILITY_CODE = FACILITY_CODE(84u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_POWERSHELL: FACILITY_CODE = FACILITY_CODE(84u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_RAS: FACILITY_CODE = FACILITY_CODE(83u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_P2P_INT: FACILITY_CODE = FACILITY_CODE(98u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_P2P: FACILITY_CODE = FACILITY_CODE(99u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DAF: FACILITY_CODE = FACILITY_CODE(100u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_BLUETOOTH_ATT: FACILITY_CODE = FACILITY_CODE(101u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_AUDIO: FACILITY_CODE = FACILITY_CODE(102u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_STATEREPOSITORY: FACILITY_CODE = FACILITY_CODE(103u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_VISUALCPP: FACILITY_CODE = FACILITY_CODE(109u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_SCRIPT: FACILITY_CODE = FACILITY_CODE(112u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_PARSE: FACILITY_CODE = FACILITY_CODE(113u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_BLB: FACILITY_CODE = FACILITY_CODE(120u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_BLB_CLI: FACILITY_CODE = FACILITY_CODE(121u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_WSBAPP: FACILITY_CODE = FACILITY_CODE(122u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_BLBUI: FACILITY_CODE = FACILITY_CODE(128u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_USN: FACILITY_CODE = FACILITY_CODE(129u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_USERMODE_VOLSNAP: FACILITY_CODE = FACILITY_CODE(130u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_TIERING: FACILITY_CODE = FACILITY_CODE(131u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_WSB_ONLINE: FACILITY_CODE = FACILITY_CODE(133u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_ONLINE_ID: FACILITY_CODE = FACILITY_CODE(134u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DEVICE_UPDATE_AGENT: FACILITY_CODE = FACILITY_CODE(135u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DRVSERVICING: FACILITY_CODE = FACILITY_CODE(136u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DLS: FACILITY_CODE = FACILITY_CODE(153u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DELIVERY_OPTIMIZATION: FACILITY_CODE = FACILITY_CODE(208u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_USERMODE_SPACES: FACILITY_CODE = FACILITY_CODE(231u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_USER_MODE_SECURITY_CORE: FACILITY_CODE = FACILITY_CODE(232u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_USERMODE_LICENSING: FACILITY_CODE = FACILITY_CODE(234u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_SOS: FACILITY_CODE = FACILITY_CODE(160u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_OCP_UPDATE_AGENT: FACILITY_CODE = FACILITY_CODE(173u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DEBUGGERS: FACILITY_CODE = FACILITY_CODE(176u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_SPP: FACILITY_CODE = FACILITY_CODE(256u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_RESTORE: FACILITY_CODE = FACILITY_CODE(256u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DMSERVER: FACILITY_CODE = FACILITY_CODE(256u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DEPLOYMENT_SERVICES_SERVER: FACILITY_CODE = FACILITY_CODE(257u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DEPLOYMENT_SERVICES_IMAGING: FACILITY_CODE = FACILITY_CODE(258u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DEPLOYMENT_SERVICES_MANAGEMENT: FACILITY_CODE = FACILITY_CODE(259u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DEPLOYMENT_SERVICES_UTIL: FACILITY_CODE = FACILITY_CODE(260u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DEPLOYMENT_SERVICES_BINLSVC: FACILITY_CODE = FACILITY_CODE(261u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DEPLOYMENT_SERVICES_PXE: FACILITY_CODE = FACILITY_CODE(263u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DEPLOYMENT_SERVICES_TFTP: FACILITY_CODE = FACILITY_CODE(264u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT: FACILITY_CODE = FACILITY_CODE(272u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DEPLOYMENT_SERVICES_DRIVER_PROVISIONING: FACILITY_CODE = FACILITY_CODE(278u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DEPLOYMENT_SERVICES_MULTICAST_SERVER: FACILITY_CODE = FACILITY_CODE(289u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DEPLOYMENT_SERVICES_MULTICAST_CLIENT: FACILITY_CODE = FACILITY_CODE(290u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DEPLOYMENT_SERVICES_CONTENT_PROVIDER: FACILITY_CODE = FACILITY_CODE(293u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_HSP_SERVICES: FACILITY_CODE = FACILITY_CODE(296u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_HSP_SOFTWARE: FACILITY_CODE = FACILITY_CODE(297u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_LINGUISTIC_SERVICES: FACILITY_CODE = FACILITY_CODE(305u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_AUDIOSTREAMING: FACILITY_CODE = FACILITY_CODE(1094u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_TTD: FACILITY_CODE = FACILITY_CODE(1490u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_ACCELERATOR: FACILITY_CODE = FACILITY_CODE(1536u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_WMAAECMA: FACILITY_CODE = FACILITY_CODE(1996u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DIRECTMUSIC: FACILITY_CODE = FACILITY_CODE(2168u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DIRECT3D10: FACILITY_CODE = FACILITY_CODE(2169u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DXGI: FACILITY_CODE = FACILITY_CODE(2170u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DXGI_DDI: FACILITY_CODE = FACILITY_CODE(2171u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DIRECT3D11: FACILITY_CODE = FACILITY_CODE(2172u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DIRECT3D11_DEBUG: FACILITY_CODE = FACILITY_CODE(2173u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DIRECT3D12: FACILITY_CODE = FACILITY_CODE(2174u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DIRECT3D12_DEBUG: FACILITY_CODE = FACILITY_CODE(2175u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DXCORE: FACILITY_CODE = FACILITY_CODE(2176u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_PRESENTATION: FACILITY_CODE = FACILITY_CODE(2177u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_LEAP: FACILITY_CODE = FACILITY_CODE(2184u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_AUDCLNT: FACILITY_CODE = FACILITY_CODE(2185u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_WINCODEC_DWRITE_DWM: FACILITY_CODE = FACILITY_CODE(2200u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_WINML: FACILITY_CODE = FACILITY_CODE(2192u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DIRECT2D: FACILITY_CODE = FACILITY_CODE(2201u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_DEFRAG: FACILITY_CODE = FACILITY_CODE(2304u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_USERMODE_SDBUS: FACILITY_CODE = FACILITY_CODE(2305u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_JSCRIPT: FACILITY_CODE = FACILITY_CODE(2306u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_PIDGENX: FACILITY_CODE = FACILITY_CODE(2561u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_EAS: FACILITY_CODE = FACILITY_CODE(85u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_WEB: FACILITY_CODE = FACILITY_CODE(885u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_WEB_SOCKET: FACILITY_CODE = FACILITY_CODE(886u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_MOBILE: FACILITY_CODE = FACILITY_CODE(1793u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_SQLITE: FACILITY_CODE = FACILITY_CODE(1967u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_SERVICE_FABRIC: FACILITY_CODE = FACILITY_CODE(1968u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_UTC: FACILITY_CODE = FACILITY_CODE(1989u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_WEP: FACILITY_CODE = FACILITY_CODE(2049u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_SYNCENGINE: FACILITY_CODE = FACILITY_CODE(2050u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_XBOX: FACILITY_CODE = FACILITY_CODE(2339u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_GAME: FACILITY_CODE = FACILITY_CODE(2340u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_PIX: FACILITY_CODE = FACILITY_CODE(2748u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FACILITY_NT_BIT: FACILITY_CODE = FACILITY_CODE(268435456u32);
impl ::core::marker::Copy for FACILITY_CODE {}
impl ::core::clone::Clone for FACILITY_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FACILITY_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FACILITY_CODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FACILITY_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FACILITY_CODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FORMAT_MESSAGE_OPTIONS(pub u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FORMAT_MESSAGE_ALLOCATE_BUFFER: FORMAT_MESSAGE_OPTIONS = FORMAT_MESSAGE_OPTIONS(256u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FORMAT_MESSAGE_ARGUMENT_ARRAY: FORMAT_MESSAGE_OPTIONS = FORMAT_MESSAGE_OPTIONS(8192u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FORMAT_MESSAGE_FROM_HMODULE: FORMAT_MESSAGE_OPTIONS = FORMAT_MESSAGE_OPTIONS(2048u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FORMAT_MESSAGE_FROM_STRING: FORMAT_MESSAGE_OPTIONS = FORMAT_MESSAGE_OPTIONS(1024u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FORMAT_MESSAGE_FROM_SYSTEM: FORMAT_MESSAGE_OPTIONS = FORMAT_MESSAGE_OPTIONS(4096u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FORMAT_MESSAGE_IGNORE_INSERTS: FORMAT_MESSAGE_OPTIONS = FORMAT_MESSAGE_OPTIONS(512u32);
impl ::core::marker::Copy for FORMAT_MESSAGE_OPTIONS {}
impl ::core::clone::Clone for FORMAT_MESSAGE_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FORMAT_MESSAGE_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FORMAT_MESSAGE_OPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FORMAT_MESSAGE_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FORMAT_MESSAGE_OPTIONS").field(&self.0).finish()
    }
}
impl FORMAT_MESSAGE_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for FORMAT_MESSAGE_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FORMAT_MESSAGE_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FORMAT_MESSAGE_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FORMAT_MESSAGE_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FORMAT_MESSAGE_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAGEHLP_CBA_EVENT_SEVERITY(pub u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const sevInfo: IMAGEHLP_CBA_EVENT_SEVERITY = IMAGEHLP_CBA_EVENT_SEVERITY(0u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const sevProblem: IMAGEHLP_CBA_EVENT_SEVERITY = IMAGEHLP_CBA_EVENT_SEVERITY(1u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const sevAttn: IMAGEHLP_CBA_EVENT_SEVERITY = IMAGEHLP_CBA_EVENT_SEVERITY(2u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const sevFatal: IMAGEHLP_CBA_EVENT_SEVERITY = IMAGEHLP_CBA_EVENT_SEVERITY(3u32);
impl ::core::marker::Copy for IMAGEHLP_CBA_EVENT_SEVERITY {}
impl ::core::clone::Clone for IMAGEHLP_CBA_EVENT_SEVERITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAGEHLP_CBA_EVENT_SEVERITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for IMAGEHLP_CBA_EVENT_SEVERITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for IMAGEHLP_CBA_EVENT_SEVERITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGEHLP_CBA_EVENT_SEVERITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAGEHLP_EXTENDED_OPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_EX_DISABLEACCESSTIMEUPDATE: IMAGEHLP_EXTENDED_OPTIONS = IMAGEHLP_EXTENDED_OPTIONS(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_EX_LASTVALIDDEBUGDIRECTORY: IMAGEHLP_EXTENDED_OPTIONS = IMAGEHLP_EXTENDED_OPTIONS(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_EX_NOIMPLICITPATTERNSEARCH: IMAGEHLP_EXTENDED_OPTIONS = IMAGEHLP_EXTENDED_OPTIONS(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_EX_NEVERLOADSYMBOLS: IMAGEHLP_EXTENDED_OPTIONS = IMAGEHLP_EXTENDED_OPTIONS(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMOPT_EX_MAX: IMAGEHLP_EXTENDED_OPTIONS = IMAGEHLP_EXTENDED_OPTIONS(4i32);
impl ::core::marker::Copy for IMAGEHLP_EXTENDED_OPTIONS {}
impl ::core::clone::Clone for IMAGEHLP_EXTENDED_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAGEHLP_EXTENDED_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for IMAGEHLP_EXTENDED_OPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for IMAGEHLP_EXTENDED_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGEHLP_EXTENDED_OPTIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAGEHLP_GET_TYPE_INFO_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGEHLP_GET_TYPE_INFO_CHILDREN: IMAGEHLP_GET_TYPE_INFO_FLAGS = IMAGEHLP_GET_TYPE_INFO_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGEHLP_GET_TYPE_INFO_UNCACHED: IMAGEHLP_GET_TYPE_INFO_FLAGS = IMAGEHLP_GET_TYPE_INFO_FLAGS(1u32);
impl ::core::marker::Copy for IMAGEHLP_GET_TYPE_INFO_FLAGS {}
impl ::core::clone::Clone for IMAGEHLP_GET_TYPE_INFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAGEHLP_GET_TYPE_INFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for IMAGEHLP_GET_TYPE_INFO_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for IMAGEHLP_GET_TYPE_INFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGEHLP_GET_TYPE_INFO_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAGEHLP_HD_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const hdBase: IMAGEHLP_HD_TYPE = IMAGEHLP_HD_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const hdSym: IMAGEHLP_HD_TYPE = IMAGEHLP_HD_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const hdSrc: IMAGEHLP_HD_TYPE = IMAGEHLP_HD_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const hdMax: IMAGEHLP_HD_TYPE = IMAGEHLP_HD_TYPE(3i32);
impl ::core::marker::Copy for IMAGEHLP_HD_TYPE {}
impl ::core::clone::Clone for IMAGEHLP_HD_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAGEHLP_HD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for IMAGEHLP_HD_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for IMAGEHLP_HD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGEHLP_HD_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAGEHLP_SF_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const sfImage: IMAGEHLP_SF_TYPE = IMAGEHLP_SF_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const sfDbg: IMAGEHLP_SF_TYPE = IMAGEHLP_SF_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const sfPdb: IMAGEHLP_SF_TYPE = IMAGEHLP_SF_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const sfMpd: IMAGEHLP_SF_TYPE = IMAGEHLP_SF_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const sfMax: IMAGEHLP_SF_TYPE = IMAGEHLP_SF_TYPE(4i32);
impl ::core::marker::Copy for IMAGEHLP_SF_TYPE {}
impl ::core::clone::Clone for IMAGEHLP_SF_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAGEHLP_SF_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for IMAGEHLP_SF_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for IMAGEHLP_SF_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGEHLP_SF_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAGEHLP_STATUS_REASON(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BindOutOfMemory: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BindRvaToVaFailed: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BindNoRoomInImage: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BindImportModuleFailed: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BindImportProcedureFailed: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BindImportModule: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(5i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BindImportProcedure: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(6i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BindForwarder: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(7i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BindForwarderNOT: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(8i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BindImageModified: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(9i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BindExpandFileHeaders: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(10i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BindImageComplete: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(11i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BindMismatchedSymbols: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(12i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BindSymbolsNotUpdated: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(13i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BindImportProcedure32: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(14i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BindImportProcedure64: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(15i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BindForwarder32: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(16i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BindForwarder64: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(17i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BindForwarderNOT32: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(18i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const BindForwarderNOT64: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(19i32);
impl ::core::marker::Copy for IMAGEHLP_STATUS_REASON {}
impl ::core::clone::Clone for IMAGEHLP_STATUS_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAGEHLP_STATUS_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for IMAGEHLP_STATUS_REASON {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for IMAGEHLP_STATUS_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGEHLP_STATUS_REASON").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAGEHLP_SYMBOL_TYPE_INFO(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GET_SYMTAG: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GET_SYMNAME: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GET_LENGTH: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GET_TYPE: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GET_TYPEID: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GET_BASETYPE: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(5i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GET_ARRAYINDEXTYPEID: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(6i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_FINDCHILDREN: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(7i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GET_DATAKIND: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(8i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GET_ADDRESSOFFSET: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(9i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GET_OFFSET: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(10i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GET_VALUE: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(11i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GET_COUNT: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(12i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GET_CHILDRENCOUNT: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(13i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GET_BITPOSITION: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(14i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GET_VIRTUALBASECLASS: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(15i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GET_VIRTUALTABLESHAPEID: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(16i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GET_VIRTUALBASEPOINTEROFFSET: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(17i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GET_CLASSPARENTID: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(18i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GET_NESTED: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(19i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GET_SYMINDEX: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(20i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GET_LEXICALPARENT: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(21i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GET_ADDRESS: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(22i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GET_THISADJUST: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(23i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GET_UDTKIND: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(24i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_IS_EQUIV_TO: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(25i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GET_CALLING_CONVENTION: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(26i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_IS_CLOSE_EQUIV_TO: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(27i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GTIEX_REQS_VALID: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(28i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GET_VIRTUALBASEOFFSET: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(29i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GET_VIRTUALBASEDISPINDEX: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(30i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GET_IS_REFERENCE: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(31i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GET_INDIRECTVIRTUALBASECLASS: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(32i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GET_VIRTUALBASETABLETYPE: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(33i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TI_GET_OBJECTPOINTERTYPE: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(34i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGEHLP_SYMBOL_TYPE_INFO_MAX: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(35i32);
impl ::core::marker::Copy for IMAGEHLP_SYMBOL_TYPE_INFO {}
impl ::core::clone::Clone for IMAGEHLP_SYMBOL_TYPE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAGEHLP_SYMBOL_TYPE_INFO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for IMAGEHLP_SYMBOL_TYPE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for IMAGEHLP_SYMBOL_TYPE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGEHLP_SYMBOL_TYPE_INFO").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAGE_DEBUG_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DEBUG_TYPE_UNKNOWN: IMAGE_DEBUG_TYPE = IMAGE_DEBUG_TYPE(0u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DEBUG_TYPE_COFF: IMAGE_DEBUG_TYPE = IMAGE_DEBUG_TYPE(1u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DEBUG_TYPE_CODEVIEW: IMAGE_DEBUG_TYPE = IMAGE_DEBUG_TYPE(2u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DEBUG_TYPE_FPO: IMAGE_DEBUG_TYPE = IMAGE_DEBUG_TYPE(3u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DEBUG_TYPE_MISC: IMAGE_DEBUG_TYPE = IMAGE_DEBUG_TYPE(4u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DEBUG_TYPE_EXCEPTION: IMAGE_DEBUG_TYPE = IMAGE_DEBUG_TYPE(5u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DEBUG_TYPE_FIXUP: IMAGE_DEBUG_TYPE = IMAGE_DEBUG_TYPE(6u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DEBUG_TYPE_BORLAND: IMAGE_DEBUG_TYPE = IMAGE_DEBUG_TYPE(9u32);
impl ::core::marker::Copy for IMAGE_DEBUG_TYPE {}
impl ::core::clone::Clone for IMAGE_DEBUG_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAGE_DEBUG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for IMAGE_DEBUG_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for IMAGE_DEBUG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGE_DEBUG_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAGE_DIRECTORY_ENTRY(pub u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DIRECTORY_ENTRY_ARCHITECTURE: IMAGE_DIRECTORY_ENTRY = IMAGE_DIRECTORY_ENTRY(7u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DIRECTORY_ENTRY_BASERELOC: IMAGE_DIRECTORY_ENTRY = IMAGE_DIRECTORY_ENTRY(5u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT: IMAGE_DIRECTORY_ENTRY = IMAGE_DIRECTORY_ENTRY(11u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR: IMAGE_DIRECTORY_ENTRY = IMAGE_DIRECTORY_ENTRY(14u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DIRECTORY_ENTRY_DEBUG: IMAGE_DIRECTORY_ENTRY = IMAGE_DIRECTORY_ENTRY(6u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT: IMAGE_DIRECTORY_ENTRY = IMAGE_DIRECTORY_ENTRY(13u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DIRECTORY_ENTRY_EXCEPTION: IMAGE_DIRECTORY_ENTRY = IMAGE_DIRECTORY_ENTRY(3u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DIRECTORY_ENTRY_EXPORT: IMAGE_DIRECTORY_ENTRY = IMAGE_DIRECTORY_ENTRY(0u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DIRECTORY_ENTRY_GLOBALPTR: IMAGE_DIRECTORY_ENTRY = IMAGE_DIRECTORY_ENTRY(8u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DIRECTORY_ENTRY_IAT: IMAGE_DIRECTORY_ENTRY = IMAGE_DIRECTORY_ENTRY(12u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DIRECTORY_ENTRY_IMPORT: IMAGE_DIRECTORY_ENTRY = IMAGE_DIRECTORY_ENTRY(1u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG: IMAGE_DIRECTORY_ENTRY = IMAGE_DIRECTORY_ENTRY(10u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DIRECTORY_ENTRY_RESOURCE: IMAGE_DIRECTORY_ENTRY = IMAGE_DIRECTORY_ENTRY(2u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DIRECTORY_ENTRY_SECURITY: IMAGE_DIRECTORY_ENTRY = IMAGE_DIRECTORY_ENTRY(4u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DIRECTORY_ENTRY_TLS: IMAGE_DIRECTORY_ENTRY = IMAGE_DIRECTORY_ENTRY(9u16);
impl ::core::marker::Copy for IMAGE_DIRECTORY_ENTRY {}
impl ::core::clone::Clone for IMAGE_DIRECTORY_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAGE_DIRECTORY_ENTRY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for IMAGE_DIRECTORY_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for IMAGE_DIRECTORY_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGE_DIRECTORY_ENTRY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAGE_DLL_CHARACTERISTICS(pub u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(32u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(64u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(128u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DLLCHARACTERISTICS_NX_COMPAT: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(256u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DLLCHARACTERISTICS_NO_ISOLATION: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(512u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DLLCHARACTERISTICS_NO_SEH: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(1024u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DLLCHARACTERISTICS_NO_BIND: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(2048u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DLLCHARACTERISTICS_APPCONTAINER: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(4096u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DLLCHARACTERISTICS_WDM_DRIVER: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(8192u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DLLCHARACTERISTICS_GUARD_CF: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(16384u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(32768u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DLLCHARACTERISTICS_EX_CET_COMPAT: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(1u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DLLCHARACTERISTICS_EX_CET_COMPAT_STRICT_MODE: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(2u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DLLCHARACTERISTICS_EX_CET_SET_CONTEXT_IP_VALIDATION_RELAXED_MODE: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(4u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DLLCHARACTERISTICS_EX_CET_DYNAMIC_APIS_ALLOW_IN_PROC: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(8u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DLLCHARACTERISTICS_EX_CET_RESERVED_1: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(16u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_DLLCHARACTERISTICS_EX_CET_RESERVED_2: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(32u16);
impl ::core::marker::Copy for IMAGE_DLL_CHARACTERISTICS {}
impl ::core::clone::Clone for IMAGE_DLL_CHARACTERISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAGE_DLL_CHARACTERISTICS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for IMAGE_DLL_CHARACTERISTICS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for IMAGE_DLL_CHARACTERISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGE_DLL_CHARACTERISTICS").field(&self.0).finish()
    }
}
impl IMAGE_DLL_CHARACTERISTICS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for IMAGE_DLL_CHARACTERISTICS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IMAGE_DLL_CHARACTERISTICS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IMAGE_DLL_CHARACTERISTICS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IMAGE_DLL_CHARACTERISTICS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IMAGE_DLL_CHARACTERISTICS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAGE_FILE_CHARACTERISTICS(pub u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_FILE_RELOCS_STRIPPED: IMAGE_FILE_CHARACTERISTICS = IMAGE_FILE_CHARACTERISTICS(1u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_FILE_EXECUTABLE_IMAGE: IMAGE_FILE_CHARACTERISTICS = IMAGE_FILE_CHARACTERISTICS(2u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_FILE_LINE_NUMS_STRIPPED: IMAGE_FILE_CHARACTERISTICS = IMAGE_FILE_CHARACTERISTICS(4u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_FILE_LOCAL_SYMS_STRIPPED: IMAGE_FILE_CHARACTERISTICS = IMAGE_FILE_CHARACTERISTICS(8u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_FILE_AGGRESIVE_WS_TRIM: IMAGE_FILE_CHARACTERISTICS = IMAGE_FILE_CHARACTERISTICS(16u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_FILE_LARGE_ADDRESS_AWARE: IMAGE_FILE_CHARACTERISTICS = IMAGE_FILE_CHARACTERISTICS(32u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_FILE_BYTES_REVERSED_LO: IMAGE_FILE_CHARACTERISTICS = IMAGE_FILE_CHARACTERISTICS(128u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_FILE_32BIT_MACHINE: IMAGE_FILE_CHARACTERISTICS = IMAGE_FILE_CHARACTERISTICS(256u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_FILE_DEBUG_STRIPPED: IMAGE_FILE_CHARACTERISTICS = IMAGE_FILE_CHARACTERISTICS(512u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP: IMAGE_FILE_CHARACTERISTICS = IMAGE_FILE_CHARACTERISTICS(1024u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_FILE_NET_RUN_FROM_SWAP: IMAGE_FILE_CHARACTERISTICS = IMAGE_FILE_CHARACTERISTICS(2048u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_FILE_SYSTEM: IMAGE_FILE_CHARACTERISTICS = IMAGE_FILE_CHARACTERISTICS(4096u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_FILE_DLL: IMAGE_FILE_CHARACTERISTICS = IMAGE_FILE_CHARACTERISTICS(8192u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_FILE_UP_SYSTEM_ONLY: IMAGE_FILE_CHARACTERISTICS = IMAGE_FILE_CHARACTERISTICS(16384u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_FILE_BYTES_REVERSED_HI: IMAGE_FILE_CHARACTERISTICS = IMAGE_FILE_CHARACTERISTICS(32768u16);
impl ::core::marker::Copy for IMAGE_FILE_CHARACTERISTICS {}
impl ::core::clone::Clone for IMAGE_FILE_CHARACTERISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAGE_FILE_CHARACTERISTICS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for IMAGE_FILE_CHARACTERISTICS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for IMAGE_FILE_CHARACTERISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGE_FILE_CHARACTERISTICS").field(&self.0).finish()
    }
}
impl IMAGE_FILE_CHARACTERISTICS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for IMAGE_FILE_CHARACTERISTICS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IMAGE_FILE_CHARACTERISTICS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IMAGE_FILE_CHARACTERISTICS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IMAGE_FILE_CHARACTERISTICS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IMAGE_FILE_CHARACTERISTICS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAGE_FILE_CHARACTERISTICS2(pub u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_FILE_RELOCS_STRIPPED2: IMAGE_FILE_CHARACTERISTICS2 = IMAGE_FILE_CHARACTERISTICS2(1u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_FILE_EXECUTABLE_IMAGE2: IMAGE_FILE_CHARACTERISTICS2 = IMAGE_FILE_CHARACTERISTICS2(2u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_FILE_LINE_NUMS_STRIPPED2: IMAGE_FILE_CHARACTERISTICS2 = IMAGE_FILE_CHARACTERISTICS2(4u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_FILE_LOCAL_SYMS_STRIPPED2: IMAGE_FILE_CHARACTERISTICS2 = IMAGE_FILE_CHARACTERISTICS2(8u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_FILE_AGGRESIVE_WS_TRIM2: IMAGE_FILE_CHARACTERISTICS2 = IMAGE_FILE_CHARACTERISTICS2(16u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_FILE_LARGE_ADDRESS_AWARE2: IMAGE_FILE_CHARACTERISTICS2 = IMAGE_FILE_CHARACTERISTICS2(32u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_FILE_BYTES_REVERSED_LO2: IMAGE_FILE_CHARACTERISTICS2 = IMAGE_FILE_CHARACTERISTICS2(128u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_FILE_32BIT_MACHINE2: IMAGE_FILE_CHARACTERISTICS2 = IMAGE_FILE_CHARACTERISTICS2(256u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_FILE_DEBUG_STRIPPED2: IMAGE_FILE_CHARACTERISTICS2 = IMAGE_FILE_CHARACTERISTICS2(512u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP2: IMAGE_FILE_CHARACTERISTICS2 = IMAGE_FILE_CHARACTERISTICS2(1024u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_FILE_NET_RUN_FROM_SWAP2: IMAGE_FILE_CHARACTERISTICS2 = IMAGE_FILE_CHARACTERISTICS2(2048u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_FILE_SYSTEM_2: IMAGE_FILE_CHARACTERISTICS2 = IMAGE_FILE_CHARACTERISTICS2(4096u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_FILE_DLL_2: IMAGE_FILE_CHARACTERISTICS2 = IMAGE_FILE_CHARACTERISTICS2(8192u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_FILE_UP_SYSTEM_ONLY_2: IMAGE_FILE_CHARACTERISTICS2 = IMAGE_FILE_CHARACTERISTICS2(16384u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_FILE_BYTES_REVERSED_HI_2: IMAGE_FILE_CHARACTERISTICS2 = IMAGE_FILE_CHARACTERISTICS2(32768u32);
impl ::core::marker::Copy for IMAGE_FILE_CHARACTERISTICS2 {}
impl ::core::clone::Clone for IMAGE_FILE_CHARACTERISTICS2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAGE_FILE_CHARACTERISTICS2 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for IMAGE_FILE_CHARACTERISTICS2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for IMAGE_FILE_CHARACTERISTICS2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGE_FILE_CHARACTERISTICS2").field(&self.0).finish()
    }
}
impl IMAGE_FILE_CHARACTERISTICS2 {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for IMAGE_FILE_CHARACTERISTICS2 {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IMAGE_FILE_CHARACTERISTICS2 {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IMAGE_FILE_CHARACTERISTICS2 {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IMAGE_FILE_CHARACTERISTICS2 {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IMAGE_FILE_CHARACTERISTICS2 {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAGE_OPTIONAL_HEADER_MAGIC(pub u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_NT_OPTIONAL_HDR_MAGIC: IMAGE_OPTIONAL_HEADER_MAGIC = IMAGE_OPTIONAL_HEADER_MAGIC(523u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_NT_OPTIONAL_HDR32_MAGIC: IMAGE_OPTIONAL_HEADER_MAGIC = IMAGE_OPTIONAL_HEADER_MAGIC(267u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_NT_OPTIONAL_HDR64_MAGIC: IMAGE_OPTIONAL_HEADER_MAGIC = IMAGE_OPTIONAL_HEADER_MAGIC(523u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_ROM_OPTIONAL_HDR_MAGIC: IMAGE_OPTIONAL_HEADER_MAGIC = IMAGE_OPTIONAL_HEADER_MAGIC(263u16);
impl ::core::marker::Copy for IMAGE_OPTIONAL_HEADER_MAGIC {}
impl ::core::clone::Clone for IMAGE_OPTIONAL_HEADER_MAGIC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAGE_OPTIONAL_HEADER_MAGIC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for IMAGE_OPTIONAL_HEADER_MAGIC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for IMAGE_OPTIONAL_HEADER_MAGIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGE_OPTIONAL_HEADER_MAGIC").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAGE_SECTION_CHARACTERISTICS(pub u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_TYPE_NO_PAD: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(8u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_CNT_CODE: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(32u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_CNT_INITIALIZED_DATA: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(64u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_CNT_UNINITIALIZED_DATA: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(128u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_LNK_OTHER: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(256u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_LNK_INFO: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(512u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_LNK_REMOVE: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(2048u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_LNK_COMDAT: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(4096u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_NO_DEFER_SPEC_EXC: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(16384u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_GPREL: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(32768u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_MEM_FARDATA: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(32768u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_MEM_PURGEABLE: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(131072u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_MEM_16BIT: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(131072u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_MEM_LOCKED: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(262144u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_MEM_PRELOAD: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(524288u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_ALIGN_1BYTES: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(1048576u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_ALIGN_2BYTES: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(2097152u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_ALIGN_4BYTES: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(3145728u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_ALIGN_8BYTES: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(4194304u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_ALIGN_16BYTES: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(5242880u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_ALIGN_32BYTES: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(6291456u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_ALIGN_64BYTES: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(7340032u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_ALIGN_128BYTES: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(8388608u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_ALIGN_256BYTES: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(9437184u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_ALIGN_512BYTES: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(10485760u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_ALIGN_1024BYTES: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(11534336u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_ALIGN_2048BYTES: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(12582912u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_ALIGN_4096BYTES: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(13631488u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_ALIGN_8192BYTES: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(14680064u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_ALIGN_MASK: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(15728640u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_LNK_NRELOC_OVFL: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(16777216u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_MEM_DISCARDABLE: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(33554432u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_MEM_NOT_CACHED: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(67108864u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_MEM_NOT_PAGED: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(134217728u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_MEM_SHARED: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(268435456u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_MEM_EXECUTE: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(536870912u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_MEM_READ: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(1073741824u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_MEM_WRITE: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(2147483648u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SCN_SCALE_INDEX: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(1u32);
impl ::core::marker::Copy for IMAGE_SECTION_CHARACTERISTICS {}
impl ::core::clone::Clone for IMAGE_SECTION_CHARACTERISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAGE_SECTION_CHARACTERISTICS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for IMAGE_SECTION_CHARACTERISTICS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for IMAGE_SECTION_CHARACTERISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGE_SECTION_CHARACTERISTICS").field(&self.0).finish()
    }
}
impl IMAGE_SECTION_CHARACTERISTICS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for IMAGE_SECTION_CHARACTERISTICS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IMAGE_SECTION_CHARACTERISTICS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IMAGE_SECTION_CHARACTERISTICS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IMAGE_SECTION_CHARACTERISTICS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IMAGE_SECTION_CHARACTERISTICS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAGE_SUBSYSTEM(pub u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SUBSYSTEM_UNKNOWN: IMAGE_SUBSYSTEM = IMAGE_SUBSYSTEM(0u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SUBSYSTEM_NATIVE: IMAGE_SUBSYSTEM = IMAGE_SUBSYSTEM(1u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SUBSYSTEM_WINDOWS_GUI: IMAGE_SUBSYSTEM = IMAGE_SUBSYSTEM(2u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SUBSYSTEM_WINDOWS_CUI: IMAGE_SUBSYSTEM = IMAGE_SUBSYSTEM(3u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SUBSYSTEM_OS2_CUI: IMAGE_SUBSYSTEM = IMAGE_SUBSYSTEM(5u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SUBSYSTEM_POSIX_CUI: IMAGE_SUBSYSTEM = IMAGE_SUBSYSTEM(7u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SUBSYSTEM_NATIVE_WINDOWS: IMAGE_SUBSYSTEM = IMAGE_SUBSYSTEM(8u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SUBSYSTEM_WINDOWS_CE_GUI: IMAGE_SUBSYSTEM = IMAGE_SUBSYSTEM(9u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SUBSYSTEM_EFI_APPLICATION: IMAGE_SUBSYSTEM = IMAGE_SUBSYSTEM(10u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SUBSYSTEM_EFI_BOOT_SERVICE_DRIVER: IMAGE_SUBSYSTEM = IMAGE_SUBSYSTEM(11u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SUBSYSTEM_EFI_RUNTIME_DRIVER: IMAGE_SUBSYSTEM = IMAGE_SUBSYSTEM(12u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SUBSYSTEM_EFI_ROM: IMAGE_SUBSYSTEM = IMAGE_SUBSYSTEM(13u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SUBSYSTEM_XBOX: IMAGE_SUBSYSTEM = IMAGE_SUBSYSTEM(14u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SUBSYSTEM_WINDOWS_BOOT_APPLICATION: IMAGE_SUBSYSTEM = IMAGE_SUBSYSTEM(16u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IMAGE_SUBSYSTEM_XBOX_CODE_CATALOG: IMAGE_SUBSYSTEM = IMAGE_SUBSYSTEM(17u16);
impl ::core::marker::Copy for IMAGE_SUBSYSTEM {}
impl ::core::clone::Clone for IMAGE_SUBSYSTEM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAGE_SUBSYSTEM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for IMAGE_SUBSYSTEM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for IMAGE_SUBSYSTEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGE_SUBSYSTEM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IPMI_OS_SEL_RECORD_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IpmiOsSelRecordTypeWhea: IPMI_OS_SEL_RECORD_TYPE = IPMI_OS_SEL_RECORD_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IpmiOsSelRecordTypeOther: IPMI_OS_SEL_RECORD_TYPE = IPMI_OS_SEL_RECORD_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IpmiOsSelRecordTypeWheaErrorXpfMca: IPMI_OS_SEL_RECORD_TYPE = IPMI_OS_SEL_RECORD_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IpmiOsSelRecordTypeWheaErrorPci: IPMI_OS_SEL_RECORD_TYPE = IPMI_OS_SEL_RECORD_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IpmiOsSelRecordTypeWheaErrorNmi: IPMI_OS_SEL_RECORD_TYPE = IPMI_OS_SEL_RECORD_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IpmiOsSelRecordTypeWheaErrorOther: IPMI_OS_SEL_RECORD_TYPE = IPMI_OS_SEL_RECORD_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IpmiOsSelRecordTypeRaw: IPMI_OS_SEL_RECORD_TYPE = IPMI_OS_SEL_RECORD_TYPE(6i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IpmiOsSelRecordTypeDriver: IPMI_OS_SEL_RECORD_TYPE = IPMI_OS_SEL_RECORD_TYPE(7i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IpmiOsSelRecordTypeBugcheckRecovery: IPMI_OS_SEL_RECORD_TYPE = IPMI_OS_SEL_RECORD_TYPE(8i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IpmiOsSelRecordTypeBugcheckData: IPMI_OS_SEL_RECORD_TYPE = IPMI_OS_SEL_RECORD_TYPE(9i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IpmiOsSelRecordTypeMax: IPMI_OS_SEL_RECORD_TYPE = IPMI_OS_SEL_RECORD_TYPE(10i32);
impl ::core::marker::Copy for IPMI_OS_SEL_RECORD_TYPE {}
impl ::core::clone::Clone for IPMI_OS_SEL_RECORD_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IPMI_OS_SEL_RECORD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for IPMI_OS_SEL_RECORD_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for IPMI_OS_SEL_RECORD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMI_OS_SEL_RECORD_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MINIDUMP_CALLBACK_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ModuleCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ThreadCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ThreadExCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IncludeThreadCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IncludeModuleCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MemoryCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CancelCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(6i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WriteKernelMinidumpCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(7i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const KernelMinidumpStatusCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(8i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const RemoveMemoryCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(9i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IncludeVmRegionCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(10i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IoStartCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(11i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IoWriteAllCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(12i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IoFinishCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(13i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ReadMemoryFailureCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(14i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SecondaryFlagsCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(15i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IsProcessSnapshotCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(16i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const VmStartCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(17i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const VmQueryCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(18i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const VmPreReadCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(19i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const VmPostReadCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(20i32);
impl ::core::marker::Copy for MINIDUMP_CALLBACK_TYPE {}
impl ::core::clone::Clone for MINIDUMP_CALLBACK_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MINIDUMP_CALLBACK_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MINIDUMP_CALLBACK_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MINIDUMP_CALLBACK_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MINIDUMP_CALLBACK_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniHandleObjectInformationNone: MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniThreadInformation1: MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniMutantInformation1: MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniMutantInformation2: MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniProcessInformation1: MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniProcessInformation2: MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniEventInformation1: MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE(6i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniSectionInformation1: MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE(7i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniSemaphoreInformation1: MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE(8i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniHandleObjectInformationTypeMax: MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE(9i32);
impl ::core::marker::Copy for MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE {}
impl ::core::clone::Clone for MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MINIDUMP_MISC_INFO_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MINIDUMP_MISC1_PROCESS_ID: MINIDUMP_MISC_INFO_FLAGS = MINIDUMP_MISC_INFO_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MINIDUMP_MISC1_PROCESS_TIMES: MINIDUMP_MISC_INFO_FLAGS = MINIDUMP_MISC_INFO_FLAGS(2u32);
impl ::core::marker::Copy for MINIDUMP_MISC_INFO_FLAGS {}
impl ::core::clone::Clone for MINIDUMP_MISC_INFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MINIDUMP_MISC_INFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MINIDUMP_MISC_INFO_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MINIDUMP_MISC_INFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MINIDUMP_MISC_INFO_FLAGS").field(&self.0).finish()
    }
}
impl MINIDUMP_MISC_INFO_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for MINIDUMP_MISC_INFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MINIDUMP_MISC_INFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MINIDUMP_MISC_INFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MINIDUMP_MISC_INFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MINIDUMP_MISC_INFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MINIDUMP_SECONDARY_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniSecondaryWithoutPowerInfo: MINIDUMP_SECONDARY_FLAGS = MINIDUMP_SECONDARY_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniSecondaryValidFlags: MINIDUMP_SECONDARY_FLAGS = MINIDUMP_SECONDARY_FLAGS(1i32);
impl ::core::marker::Copy for MINIDUMP_SECONDARY_FLAGS {}
impl ::core::clone::Clone for MINIDUMP_SECONDARY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MINIDUMP_SECONDARY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MINIDUMP_SECONDARY_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MINIDUMP_SECONDARY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MINIDUMP_SECONDARY_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MINIDUMP_STREAM_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UnusedStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ReservedStream0: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ReservedStream1: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ThreadListStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ModuleListStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MemoryListStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ExceptionStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(6i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SystemInfoStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(7i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ThreadExListStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(8i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const Memory64ListStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(9i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CommentStreamA: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(10i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const CommentStreamW: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(11i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const HandleDataStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(12i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const FunctionTableStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(13i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UnloadedModuleListStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(14i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiscInfoStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(15i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MemoryInfoListStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(16i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ThreadInfoListStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(17i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const HandleOperationListStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(18i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const TokenStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(19i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const JavaScriptDataStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(20i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SystemMemoryInfoStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(21i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ProcessVmCountersStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(22i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const IptTraceStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(23i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ThreadNamesStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(24i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ceStreamNull: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(32768i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ceStreamSystemInfo: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(32769i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ceStreamException: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(32770i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ceStreamModuleList: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(32771i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ceStreamProcessList: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(32772i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ceStreamThreadList: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(32773i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ceStreamThreadContextList: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(32774i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ceStreamThreadCallStackList: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(32775i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ceStreamMemoryVirtualList: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(32776i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ceStreamMemoryPhysicalList: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(32777i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ceStreamBucketParameters: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(32778i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ceStreamProcessModuleMap: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(32779i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ceStreamDiagnosisList: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(32780i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const LastReservedStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(65535i32);
impl ::core::marker::Copy for MINIDUMP_STREAM_TYPE {}
impl ::core::clone::Clone for MINIDUMP_STREAM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MINIDUMP_STREAM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MINIDUMP_STREAM_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MINIDUMP_STREAM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MINIDUMP_STREAM_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MINIDUMP_THREAD_INFO_DUMP_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MINIDUMP_THREAD_INFO_ERROR_THREAD: MINIDUMP_THREAD_INFO_DUMP_FLAGS = MINIDUMP_THREAD_INFO_DUMP_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MINIDUMP_THREAD_INFO_EXITED_THREAD: MINIDUMP_THREAD_INFO_DUMP_FLAGS = MINIDUMP_THREAD_INFO_DUMP_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MINIDUMP_THREAD_INFO_INVALID_CONTEXT: MINIDUMP_THREAD_INFO_DUMP_FLAGS = MINIDUMP_THREAD_INFO_DUMP_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MINIDUMP_THREAD_INFO_INVALID_INFO: MINIDUMP_THREAD_INFO_DUMP_FLAGS = MINIDUMP_THREAD_INFO_DUMP_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MINIDUMP_THREAD_INFO_INVALID_TEB: MINIDUMP_THREAD_INFO_DUMP_FLAGS = MINIDUMP_THREAD_INFO_DUMP_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MINIDUMP_THREAD_INFO_WRITING_THREAD: MINIDUMP_THREAD_INFO_DUMP_FLAGS = MINIDUMP_THREAD_INFO_DUMP_FLAGS(2u32);
impl ::core::marker::Copy for MINIDUMP_THREAD_INFO_DUMP_FLAGS {}
impl ::core::clone::Clone for MINIDUMP_THREAD_INFO_DUMP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MINIDUMP_THREAD_INFO_DUMP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MINIDUMP_THREAD_INFO_DUMP_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MINIDUMP_THREAD_INFO_DUMP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MINIDUMP_THREAD_INFO_DUMP_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MINIDUMP_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniDumpNormal: MINIDUMP_TYPE = MINIDUMP_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniDumpWithDataSegs: MINIDUMP_TYPE = MINIDUMP_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniDumpWithFullMemory: MINIDUMP_TYPE = MINIDUMP_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniDumpWithHandleData: MINIDUMP_TYPE = MINIDUMP_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniDumpFilterMemory: MINIDUMP_TYPE = MINIDUMP_TYPE(8i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniDumpScanMemory: MINIDUMP_TYPE = MINIDUMP_TYPE(16i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniDumpWithUnloadedModules: MINIDUMP_TYPE = MINIDUMP_TYPE(32i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniDumpWithIndirectlyReferencedMemory: MINIDUMP_TYPE = MINIDUMP_TYPE(64i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniDumpFilterModulePaths: MINIDUMP_TYPE = MINIDUMP_TYPE(128i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniDumpWithProcessThreadData: MINIDUMP_TYPE = MINIDUMP_TYPE(256i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniDumpWithPrivateReadWriteMemory: MINIDUMP_TYPE = MINIDUMP_TYPE(512i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniDumpWithoutOptionalData: MINIDUMP_TYPE = MINIDUMP_TYPE(1024i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniDumpWithFullMemoryInfo: MINIDUMP_TYPE = MINIDUMP_TYPE(2048i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniDumpWithThreadInfo: MINIDUMP_TYPE = MINIDUMP_TYPE(4096i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniDumpWithCodeSegs: MINIDUMP_TYPE = MINIDUMP_TYPE(8192i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniDumpWithoutAuxiliaryState: MINIDUMP_TYPE = MINIDUMP_TYPE(16384i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniDumpWithFullAuxiliaryState: MINIDUMP_TYPE = MINIDUMP_TYPE(32768i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniDumpWithPrivateWriteCopyMemory: MINIDUMP_TYPE = MINIDUMP_TYPE(65536i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniDumpIgnoreInaccessibleMemory: MINIDUMP_TYPE = MINIDUMP_TYPE(131072i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniDumpWithTokenInformation: MINIDUMP_TYPE = MINIDUMP_TYPE(262144i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniDumpWithModuleHeaders: MINIDUMP_TYPE = MINIDUMP_TYPE(524288i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniDumpFilterTriage: MINIDUMP_TYPE = MINIDUMP_TYPE(1048576i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniDumpWithAvxXStateContext: MINIDUMP_TYPE = MINIDUMP_TYPE(2097152i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniDumpWithIptTrace: MINIDUMP_TYPE = MINIDUMP_TYPE(4194304i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniDumpScanInaccessiblePartialPages: MINIDUMP_TYPE = MINIDUMP_TYPE(8388608i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniDumpFilterWriteCombinedMemory: MINIDUMP_TYPE = MINIDUMP_TYPE(16777216i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const MiniDumpValidTypeFlags: MINIDUMP_TYPE = MINIDUMP_TYPE(33554431i32);
impl ::core::marker::Copy for MINIDUMP_TYPE {}
impl ::core::clone::Clone for MINIDUMP_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MINIDUMP_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MINIDUMP_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MINIDUMP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MINIDUMP_TYPE").field(&self.0).finish()
    }
}
impl MINIDUMP_TYPE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for MINIDUMP_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MINIDUMP_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MINIDUMP_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MINIDUMP_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MINIDUMP_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MODLOAD_DATA_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBHHEADER_DEBUGDIRS: MODLOAD_DATA_TYPE = MODLOAD_DATA_TYPE(1u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const DBHHEADER_CVMISC: MODLOAD_DATA_TYPE = MODLOAD_DATA_TYPE(2u32);
impl ::core::marker::Copy for MODLOAD_DATA_TYPE {}
impl ::core::clone::Clone for MODLOAD_DATA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MODLOAD_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MODLOAD_DATA_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MODLOAD_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MODLOAD_DATA_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MODULE_WRITE_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ModuleWriteModule: MODULE_WRITE_FLAGS = MODULE_WRITE_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ModuleWriteDataSeg: MODULE_WRITE_FLAGS = MODULE_WRITE_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ModuleWriteMiscRecord: MODULE_WRITE_FLAGS = MODULE_WRITE_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ModuleWriteCvRecord: MODULE_WRITE_FLAGS = MODULE_WRITE_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ModuleReferencedByMemory: MODULE_WRITE_FLAGS = MODULE_WRITE_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ModuleWriteTlsData: MODULE_WRITE_FLAGS = MODULE_WRITE_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ModuleWriteCodeSegs: MODULE_WRITE_FLAGS = MODULE_WRITE_FLAGS(64i32);
impl ::core::marker::Copy for MODULE_WRITE_FLAGS {}
impl ::core::clone::Clone for MODULE_WRITE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MODULE_WRITE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MODULE_WRITE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MODULE_WRITE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MODULE_WRITE_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OBJECT_ATTRIB_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_NO_ATTRIB: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_NO_NAME: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_NO_TYPE: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_NO_VALUE: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_VALUE_IS_INVALID: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_VALUE_IS_OBJECT: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_VALUE_IS_ENUM: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_VALUE_IS_CUSTOM: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(64i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_OBJECT_IS_EXPANDABLE: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(112i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_VALUE_HAS_CODE: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(128i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_TYPE_IS_OBJECT: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(256i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_TYPE_HAS_CODE: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(512i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_TYPE_IS_EXPANDABLE: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(256i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_SLOT_IS_CATEGORY: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(1024i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_VALUE_READONLY: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(2048i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_ACCESS_PUBLIC: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(4096i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_ACCESS_PRIVATE: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(8192i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_ACCESS_PROTECTED: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(16384i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_ACCESS_FINAL: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(32768i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_STORAGE_GLOBAL: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(65536i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_STORAGE_STATIC: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(131072i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_STORAGE_FIELD: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(262144i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_STORAGE_VIRTUAL: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(524288i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_TYPE_IS_CONSTANT: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(1048576i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_TYPE_IS_SYNCHRONIZED: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(2097152i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_TYPE_IS_VOLATILE: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(4194304i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_HAS_EXTENDED_ATTRIBS: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(8388608i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_IS_CLASS: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(16777216i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_IS_FUNCTION: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(33554432i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_IS_VARIABLE: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(67108864i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_IS_PROPERTY: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(134217728i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_IS_MACRO: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(268435456i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_IS_TYPE: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(536870912i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_IS_INHERITED: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(1073741824i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const OBJECT_ATTRIB_IS_INTERFACE: OBJECT_ATTRIB_FLAGS = OBJECT_ATTRIB_FLAGS(-2147483648i32);
impl ::core::marker::Copy for OBJECT_ATTRIB_FLAGS {}
impl ::core::clone::Clone for OBJECT_ATTRIB_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OBJECT_ATTRIB_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for OBJECT_ATTRIB_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for OBJECT_ATTRIB_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OBJECT_ATTRIB_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OPEN_THREAD_WAIT_CHAIN_SESSION_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WCT_ASYNC_OPEN_FLAG: OPEN_THREAD_WAIT_CHAIN_SESSION_FLAGS = OPEN_THREAD_WAIT_CHAIN_SESSION_FLAGS(1u32);
impl ::core::marker::Copy for OPEN_THREAD_WAIT_CHAIN_SESSION_FLAGS {}
impl ::core::clone::Clone for OPEN_THREAD_WAIT_CHAIN_SESSION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OPEN_THREAD_WAIT_CHAIN_SESSION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for OPEN_THREAD_WAIT_CHAIN_SESSION_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for OPEN_THREAD_WAIT_CHAIN_SESSION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPEN_THREAD_WAIT_CHAIN_SESSION_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROCESSOR_ARCHITECTURE(pub u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PROCESSOR_ARCHITECTURE_AMD64: PROCESSOR_ARCHITECTURE = PROCESSOR_ARCHITECTURE(9u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PROCESSOR_ARCHITECTURE_IA64: PROCESSOR_ARCHITECTURE = PROCESSOR_ARCHITECTURE(6u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PROCESSOR_ARCHITECTURE_INTEL: PROCESSOR_ARCHITECTURE = PROCESSOR_ARCHITECTURE(0u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PROCESSOR_ARCHITECTURE_ARM: PROCESSOR_ARCHITECTURE = PROCESSOR_ARCHITECTURE(5u16);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PROCESSOR_ARCHITECTURE_UNKNOWN: PROCESSOR_ARCHITECTURE = PROCESSOR_ARCHITECTURE(65535u16);
impl ::core::marker::Copy for PROCESSOR_ARCHITECTURE {}
impl ::core::clone::Clone for PROCESSOR_ARCHITECTURE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESSOR_ARCHITECTURE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROCESSOR_ARCHITECTURE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROCESSOR_ARCHITECTURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESSOR_ARCHITECTURE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROP_INFO_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PROP_INFO_NAME: PROP_INFO_FLAGS = PROP_INFO_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PROP_INFO_TYPE: PROP_INFO_FLAGS = PROP_INFO_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PROP_INFO_VALUE: PROP_INFO_FLAGS = PROP_INFO_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PROP_INFO_FULLNAME: PROP_INFO_FLAGS = PROP_INFO_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PROP_INFO_ATTRIBUTES: PROP_INFO_FLAGS = PROP_INFO_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PROP_INFO_DEBUGPROP: PROP_INFO_FLAGS = PROP_INFO_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const PROP_INFO_AUTOEXPAND: PROP_INFO_FLAGS = PROP_INFO_FLAGS(134217728i32);
impl ::core::marker::Copy for PROP_INFO_FLAGS {}
impl ::core::clone::Clone for PROP_INFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROP_INFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROP_INFO_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROP_INFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROP_INFO_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RIP_INFO_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SLE_ERROR: RIP_INFO_TYPE = RIP_INFO_TYPE(1u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SLE_MINORERROR: RIP_INFO_TYPE = RIP_INFO_TYPE(2u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SLE_WARNING: RIP_INFO_TYPE = RIP_INFO_TYPE(3u32);
impl ::core::marker::Copy for RIP_INFO_TYPE {}
impl ::core::clone::Clone for RIP_INFO_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RIP_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RIP_INFO_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RIP_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RIP_INFO_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTL_VIRTUAL_UNWIND_HANDLER_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UNW_FLAG_NHANDLER: RTL_VIRTUAL_UNWIND_HANDLER_TYPE = RTL_VIRTUAL_UNWIND_HANDLER_TYPE(0u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UNW_FLAG_EHANDLER: RTL_VIRTUAL_UNWIND_HANDLER_TYPE = RTL_VIRTUAL_UNWIND_HANDLER_TYPE(1u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UNW_FLAG_UHANDLER: RTL_VIRTUAL_UNWIND_HANDLER_TYPE = RTL_VIRTUAL_UNWIND_HANDLER_TYPE(2u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const UNW_FLAG_CHAININFO: RTL_VIRTUAL_UNWIND_HANDLER_TYPE = RTL_VIRTUAL_UNWIND_HANDLER_TYPE(4u32);
impl ::core::marker::Copy for RTL_VIRTUAL_UNWIND_HANDLER_TYPE {}
impl ::core::clone::Clone for RTL_VIRTUAL_UNWIND_HANDLER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTL_VIRTUAL_UNWIND_HANDLER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTL_VIRTUAL_UNWIND_HANDLER_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTL_VIRTUAL_UNWIND_HANDLER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTL_VIRTUAL_UNWIND_HANDLER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYMBOL_INFO_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMFLAG_CLR_TOKEN: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(262144u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMFLAG_CONSTANT: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMFLAG_EXPORT: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMFLAG_FORWARDER: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMFLAG_FRAMEREL: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMFLAG_FUNCTION: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMFLAG_ILREL: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(65536u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMFLAG_LOCAL: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMFLAG_METADATA: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(131072u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMFLAG_PARAMETER: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMFLAG_REGISTER: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMFLAG_REGREL: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMFLAG_SLOT: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(32768u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMFLAG_THUNK: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMFLAG_TLSREL: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(16384u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMFLAG_VALUEPRESENT: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMFLAG_VIRTUAL: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(4096u32);
impl ::core::marker::Copy for SYMBOL_INFO_FLAGS {}
impl ::core::clone::Clone for SYMBOL_INFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYMBOL_INFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SYMBOL_INFO_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SYMBOL_INFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYMBOL_INFO_FLAGS").field(&self.0).finish()
    }
}
impl SYMBOL_INFO_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for SYMBOL_INFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SYMBOL_INFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SYMBOL_INFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SYMBOL_INFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SYMBOL_INFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYM_FIND_ID_OPTION(pub u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_DWORD: SYM_FIND_ID_OPTION = SYM_FIND_ID_OPTION(2u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_DWORDPTR: SYM_FIND_ID_OPTION = SYM_FIND_ID_OPTION(4u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SSRVOPT_GUIDPTR: SYM_FIND_ID_OPTION = SYM_FIND_ID_OPTION(8u32);
impl ::core::marker::Copy for SYM_FIND_ID_OPTION {}
impl ::core::clone::Clone for SYM_FIND_ID_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYM_FIND_ID_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SYM_FIND_ID_OPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SYM_FIND_ID_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYM_FIND_ID_OPTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYM_LOAD_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SLMFLAG_NONE: SYM_LOAD_FLAGS = SYM_LOAD_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SLMFLAG_VIRTUAL: SYM_LOAD_FLAGS = SYM_LOAD_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SLMFLAG_ALT_INDEX: SYM_LOAD_FLAGS = SYM_LOAD_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SLMFLAG_NO_SYMBOLS: SYM_LOAD_FLAGS = SYM_LOAD_FLAGS(4u32);
impl ::core::marker::Copy for SYM_LOAD_FLAGS {}
impl ::core::clone::Clone for SYM_LOAD_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYM_LOAD_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SYM_LOAD_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SYM_LOAD_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYM_LOAD_FLAGS").field(&self.0).finish()
    }
}
impl SYM_LOAD_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for SYM_LOAD_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SYM_LOAD_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SYM_LOAD_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SYM_LOAD_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SYM_LOAD_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYM_SRV_STORE_FILE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMSTOREOPT_COMPRESS: SYM_SRV_STORE_FILE_FLAGS = SYM_SRV_STORE_FILE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMSTOREOPT_OVERWRITE: SYM_SRV_STORE_FILE_FLAGS = SYM_SRV_STORE_FILE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMSTOREOPT_PASS_IF_EXISTS: SYM_SRV_STORE_FILE_FLAGS = SYM_SRV_STORE_FILE_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMSTOREOPT_POINTER: SYM_SRV_STORE_FILE_FLAGS = SYM_SRV_STORE_FILE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SYMSTOREOPT_RETURNINDEX: SYM_SRV_STORE_FILE_FLAGS = SYM_SRV_STORE_FILE_FLAGS(4u32);
impl ::core::marker::Copy for SYM_SRV_STORE_FILE_FLAGS {}
impl ::core::clone::Clone for SYM_SRV_STORE_FILE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYM_SRV_STORE_FILE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SYM_SRV_STORE_FILE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SYM_SRV_STORE_FILE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYM_SRV_STORE_FILE_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYM_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SymNone: SYM_TYPE = SYM_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SymCoff: SYM_TYPE = SYM_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SymCv: SYM_TYPE = SYM_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SymPdb: SYM_TYPE = SYM_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SymExport: SYM_TYPE = SYM_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SymDeferred: SYM_TYPE = SYM_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SymSym: SYM_TYPE = SYM_TYPE(6i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SymDia: SYM_TYPE = SYM_TYPE(7i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SymVirtual: SYM_TYPE = SYM_TYPE(8i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const NumSymTypes: SYM_TYPE = SYM_TYPE(9i32);
impl ::core::marker::Copy for SYM_TYPE {}
impl ::core::clone::Clone for SYM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SYM_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SYM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYM_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct THREAD_ERROR_MODE(pub u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SEM_ALL_ERRORS: THREAD_ERROR_MODE = THREAD_ERROR_MODE(0u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SEM_FAILCRITICALERRORS: THREAD_ERROR_MODE = THREAD_ERROR_MODE(1u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SEM_NOGPFAULTERRORBOX: THREAD_ERROR_MODE = THREAD_ERROR_MODE(2u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SEM_NOOPENFILEERRORBOX: THREAD_ERROR_MODE = THREAD_ERROR_MODE(32768u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const SEM_NOALIGNMENTFAULTEXCEPT: THREAD_ERROR_MODE = THREAD_ERROR_MODE(4u32);
impl ::core::marker::Copy for THREAD_ERROR_MODE {}
impl ::core::clone::Clone for THREAD_ERROR_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for THREAD_ERROR_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for THREAD_ERROR_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for THREAD_ERROR_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THREAD_ERROR_MODE").field(&self.0).finish()
    }
}
impl THREAD_ERROR_MODE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for THREAD_ERROR_MODE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for THREAD_ERROR_MODE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for THREAD_ERROR_MODE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for THREAD_ERROR_MODE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for THREAD_ERROR_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct THREAD_WRITE_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ThreadWriteThread: THREAD_WRITE_FLAGS = THREAD_WRITE_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ThreadWriteStack: THREAD_WRITE_FLAGS = THREAD_WRITE_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ThreadWriteContext: THREAD_WRITE_FLAGS = THREAD_WRITE_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ThreadWriteBackingStore: THREAD_WRITE_FLAGS = THREAD_WRITE_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ThreadWriteInstructionWindow: THREAD_WRITE_FLAGS = THREAD_WRITE_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ThreadWriteThreadData: THREAD_WRITE_FLAGS = THREAD_WRITE_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const ThreadWriteThreadInfo: THREAD_WRITE_FLAGS = THREAD_WRITE_FLAGS(64i32);
impl ::core::marker::Copy for THREAD_WRITE_FLAGS {}
impl ::core::clone::Clone for THREAD_WRITE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for THREAD_WRITE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for THREAD_WRITE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for THREAD_WRITE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THREAD_WRITE_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VER_PLATFORM(pub u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const VER_PLATFORM_WIN32s: VER_PLATFORM = VER_PLATFORM(0u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const VER_PLATFORM_WIN32_WINDOWS: VER_PLATFORM = VER_PLATFORM(1u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const VER_PLATFORM_WIN32_NT: VER_PLATFORM = VER_PLATFORM(2u32);
impl ::core::marker::Copy for VER_PLATFORM {}
impl ::core::clone::Clone for VER_PLATFORM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VER_PLATFORM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VER_PLATFORM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VER_PLATFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VER_PLATFORM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WAIT_CHAIN_THREAD_OPTIONS(pub u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WCT_OUT_OF_PROC_COM_FLAG: WAIT_CHAIN_THREAD_OPTIONS = WAIT_CHAIN_THREAD_OPTIONS(2u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WCT_OUT_OF_PROC_CS_FLAG: WAIT_CHAIN_THREAD_OPTIONS = WAIT_CHAIN_THREAD_OPTIONS(4u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WCT_OUT_OF_PROC_FLAG: WAIT_CHAIN_THREAD_OPTIONS = WAIT_CHAIN_THREAD_OPTIONS(1u32);
impl ::core::marker::Copy for WAIT_CHAIN_THREAD_OPTIONS {}
impl ::core::clone::Clone for WAIT_CHAIN_THREAD_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WAIT_CHAIN_THREAD_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WAIT_CHAIN_THREAD_OPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WAIT_CHAIN_THREAD_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WAIT_CHAIN_THREAD_OPTIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WCT_OBJECT_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WctStatusNoAccess: WCT_OBJECT_STATUS = WCT_OBJECT_STATUS(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WctStatusRunning: WCT_OBJECT_STATUS = WCT_OBJECT_STATUS(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WctStatusBlocked: WCT_OBJECT_STATUS = WCT_OBJECT_STATUS(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WctStatusPidOnly: WCT_OBJECT_STATUS = WCT_OBJECT_STATUS(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WctStatusPidOnlyRpcss: WCT_OBJECT_STATUS = WCT_OBJECT_STATUS(5i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WctStatusOwned: WCT_OBJECT_STATUS = WCT_OBJECT_STATUS(6i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WctStatusNotOwned: WCT_OBJECT_STATUS = WCT_OBJECT_STATUS(7i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WctStatusAbandoned: WCT_OBJECT_STATUS = WCT_OBJECT_STATUS(8i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WctStatusUnknown: WCT_OBJECT_STATUS = WCT_OBJECT_STATUS(9i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WctStatusError: WCT_OBJECT_STATUS = WCT_OBJECT_STATUS(10i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WctStatusMax: WCT_OBJECT_STATUS = WCT_OBJECT_STATUS(11i32);
impl ::core::marker::Copy for WCT_OBJECT_STATUS {}
impl ::core::clone::Clone for WCT_OBJECT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WCT_OBJECT_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WCT_OBJECT_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WCT_OBJECT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCT_OBJECT_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WCT_OBJECT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WctCriticalSectionType: WCT_OBJECT_TYPE = WCT_OBJECT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WctSendMessageType: WCT_OBJECT_TYPE = WCT_OBJECT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WctMutexType: WCT_OBJECT_TYPE = WCT_OBJECT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WctAlpcType: WCT_OBJECT_TYPE = WCT_OBJECT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WctComType: WCT_OBJECT_TYPE = WCT_OBJECT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WctThreadWaitType: WCT_OBJECT_TYPE = WCT_OBJECT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WctProcessWaitType: WCT_OBJECT_TYPE = WCT_OBJECT_TYPE(7i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WctThreadType: WCT_OBJECT_TYPE = WCT_OBJECT_TYPE(8i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WctComActivationType: WCT_OBJECT_TYPE = WCT_OBJECT_TYPE(9i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WctUnknownType: WCT_OBJECT_TYPE = WCT_OBJECT_TYPE(10i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WctSocketIoType: WCT_OBJECT_TYPE = WCT_OBJECT_TYPE(11i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WctSmbIoType: WCT_OBJECT_TYPE = WCT_OBJECT_TYPE(12i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WctMaxType: WCT_OBJECT_TYPE = WCT_OBJECT_TYPE(13i32);
impl ::core::marker::Copy for WCT_OBJECT_TYPE {}
impl ::core::clone::Clone for WCT_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WCT_OBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WCT_OBJECT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WCT_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCT_OBJECT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WHEA_ERROR_SOURCE_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WheaErrSrcStateStopped: WHEA_ERROR_SOURCE_STATE = WHEA_ERROR_SOURCE_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WheaErrSrcStateStarted: WHEA_ERROR_SOURCE_STATE = WHEA_ERROR_SOURCE_STATE(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WheaErrSrcStateRemoved: WHEA_ERROR_SOURCE_STATE = WHEA_ERROR_SOURCE_STATE(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WheaErrSrcStateRemovePending: WHEA_ERROR_SOURCE_STATE = WHEA_ERROR_SOURCE_STATE(4i32);
impl ::core::marker::Copy for WHEA_ERROR_SOURCE_STATE {}
impl ::core::clone::Clone for WHEA_ERROR_SOURCE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WHEA_ERROR_SOURCE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WHEA_ERROR_SOURCE_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WHEA_ERROR_SOURCE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHEA_ERROR_SOURCE_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WHEA_ERROR_SOURCE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WheaErrSrcTypeMCE: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WheaErrSrcTypeCMC: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WheaErrSrcTypeCPE: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WheaErrSrcTypeNMI: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WheaErrSrcTypePCIe: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WheaErrSrcTypeGeneric: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WheaErrSrcTypeINIT: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(6i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WheaErrSrcTypeBOOT: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(7i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WheaErrSrcTypeSCIGeneric: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(8i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WheaErrSrcTypeIPFMCA: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(9i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WheaErrSrcTypeIPFCMC: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(10i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WheaErrSrcTypeIPFCPE: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(11i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WheaErrSrcTypeGenericV2: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(12i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WheaErrSrcTypeSCIGenericV2: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(13i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WheaErrSrcTypeBMC: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(14i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WheaErrSrcTypePMEM: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(15i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WheaErrSrcTypeDeviceDriver: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(16i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WheaErrSrcTypeSea: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(17i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WheaErrSrcTypeSei: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(18i32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub const WheaErrSrcTypeMax: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(19i32);
impl ::core::marker::Copy for WHEA_ERROR_SOURCE_TYPE {}
impl ::core::clone::Clone for WHEA_ERROR_SOURCE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WHEA_ERROR_SOURCE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WHEA_ERROR_SOURCE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WHEA_ERROR_SOURCE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHEA_ERROR_SOURCE_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86")]
pub struct ADDRESS {
    pub Offset: u32,
    pub Segment: u16,
    pub Mode: ADDRESS_MODE,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for ADDRESS {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADDRESS").field("Offset", &self.Offset).field("Segment", &self.Segment).field("Mode", &self.Mode).finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::windows::core::TypeKind for ADDRESS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.Offset == other.Offset && self.Segment == other.Segment && self.Mode == other.Mode
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for ADDRESS {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct ADDRESS64 {
    pub Offset: u64,
    pub Segment: u16,
    pub Mode: ADDRESS_MODE,
}
impl ::core::marker::Copy for ADDRESS64 {}
impl ::core::clone::Clone for ADDRESS64 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ADDRESS64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADDRESS64").field("Offset", &self.Offset).field("Segment", &self.Segment).field("Mode", &self.Mode).finish()
    }
}
impl ::windows::core::TypeKind for ADDRESS64 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ADDRESS64 {
    fn eq(&self, other: &Self) -> bool {
        self.Offset == other.Offset && self.Segment == other.Segment && self.Mode == other.Mode
    }
}
impl ::core::cmp::Eq for ADDRESS64 {}
impl ::core::default::Default for ADDRESS64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub union AER_BRIDGE_DESCRIPTOR_FLAGS {
    pub Anonymous: AER_BRIDGE_DESCRIPTOR_FLAGS_0,
    pub AsUSHORT: u16,
}
impl ::core::marker::Copy for AER_BRIDGE_DESCRIPTOR_FLAGS {}
impl ::core::clone::Clone for AER_BRIDGE_DESCRIPTOR_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for AER_BRIDGE_DESCRIPTOR_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for AER_BRIDGE_DESCRIPTOR_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct AER_BRIDGE_DESCRIPTOR_FLAGS_0 {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for AER_BRIDGE_DESCRIPTOR_FLAGS_0 {}
impl ::core::clone::Clone for AER_BRIDGE_DESCRIPTOR_FLAGS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for AER_BRIDGE_DESCRIPTOR_FLAGS_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for AER_BRIDGE_DESCRIPTOR_FLAGS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub union AER_ENDPOINT_DESCRIPTOR_FLAGS {
    pub Anonymous: AER_ENDPOINT_DESCRIPTOR_FLAGS_0,
    pub AsUSHORT: u16,
}
impl ::core::marker::Copy for AER_ENDPOINT_DESCRIPTOR_FLAGS {}
impl ::core::clone::Clone for AER_ENDPOINT_DESCRIPTOR_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for AER_ENDPOINT_DESCRIPTOR_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for AER_ENDPOINT_DESCRIPTOR_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct AER_ENDPOINT_DESCRIPTOR_FLAGS_0 {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for AER_ENDPOINT_DESCRIPTOR_FLAGS_0 {}
impl ::core::clone::Clone for AER_ENDPOINT_DESCRIPTOR_FLAGS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for AER_ENDPOINT_DESCRIPTOR_FLAGS_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for AER_ENDPOINT_DESCRIPTOR_FLAGS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub union AER_ROOTPORT_DESCRIPTOR_FLAGS {
    pub Anonymous: AER_ROOTPORT_DESCRIPTOR_FLAGS_0,
    pub AsUSHORT: u16,
}
impl ::core::marker::Copy for AER_ROOTPORT_DESCRIPTOR_FLAGS {}
impl ::core::clone::Clone for AER_ROOTPORT_DESCRIPTOR_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for AER_ROOTPORT_DESCRIPTOR_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for AER_ROOTPORT_DESCRIPTOR_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct AER_ROOTPORT_DESCRIPTOR_FLAGS_0 {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for AER_ROOTPORT_DESCRIPTOR_FLAGS_0 {}
impl ::core::clone::Clone for AER_ROOTPORT_DESCRIPTOR_FLAGS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for AER_ROOTPORT_DESCRIPTOR_FLAGS_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for AER_ROOTPORT_DESCRIPTOR_FLAGS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
pub struct APC_CALLBACK_DATA {
    pub Parameter: usize,
    pub ContextRecord: *mut CONTEXT,
    pub Reserved0: usize,
    pub Reserved1: usize,
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::marker::Copy for APC_CALLBACK_DATA {}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::clone::Clone for APC_CALLBACK_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::fmt::Debug for APC_CALLBACK_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APC_CALLBACK_DATA").field("Parameter", &self.Parameter).field("ContextRecord", &self.ContextRecord).field("Reserved0", &self.Reserved0).field("Reserved1", &self.Reserved1).finish()
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::windows::core::TypeKind for APC_CALLBACK_DATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::cmp::PartialEq for APC_CALLBACK_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Parameter == other.Parameter && self.ContextRecord == other.ContextRecord && self.Reserved0 == other.Reserved0 && self.Reserved1 == other.Reserved1
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::cmp::Eq for APC_CALLBACK_DATA {}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::default::Default for APC_CALLBACK_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct API_VERSION {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub Revision: u16,
    pub Reserved: u16,
}
impl ::core::marker::Copy for API_VERSION {}
impl ::core::clone::Clone for API_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for API_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("API_VERSION").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).field("Revision", &self.Revision).field("Reserved", &self.Reserved).finish()
    }
}
impl ::windows::core::TypeKind for API_VERSION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for API_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion && self.Revision == other.Revision && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for API_VERSION {}
impl ::core::default::Default for API_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub struct ARM64_NT_CONTEXT {
    pub ContextFlags: u32,
    pub Cpsr: u32,
    pub Anonymous: ARM64_NT_CONTEXT_0,
    pub Sp: u64,
    pub Pc: u64,
    pub V: [ARM64_NT_NEON128; 32],
    pub Fpcr: u32,
    pub Fpsr: u32,
    pub Bcr: [u32; 8],
    pub Bvr: [u64; 8],
    pub Wcr: [u32; 2],
    pub Wvr: [u64; 2],
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl ::core::marker::Copy for ARM64_NT_CONTEXT {}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl ::core::clone::Clone for ARM64_NT_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl ::windows::core::TypeKind for ARM64_NT_CONTEXT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl ::core::default::Default for ARM64_NT_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub union ARM64_NT_CONTEXT_0 {
    pub Anonymous: ARM64_NT_CONTEXT_0_0,
    pub X: [u64; 31],
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl ::core::marker::Copy for ARM64_NT_CONTEXT_0 {}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl ::core::clone::Clone for ARM64_NT_CONTEXT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl ::windows::core::TypeKind for ARM64_NT_CONTEXT_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl ::core::default::Default for ARM64_NT_CONTEXT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub struct ARM64_NT_CONTEXT_0_0 {
    pub X0: u64,
    pub X1: u64,
    pub X2: u64,
    pub X3: u64,
    pub X4: u64,
    pub X5: u64,
    pub X6: u64,
    pub X7: u64,
    pub X8: u64,
    pub X9: u64,
    pub X10: u64,
    pub X11: u64,
    pub X12: u64,
    pub X13: u64,
    pub X14: u64,
    pub X15: u64,
    pub X16: u64,
    pub X17: u64,
    pub X18: u64,
    pub X19: u64,
    pub X20: u64,
    pub X21: u64,
    pub X22: u64,
    pub X23: u64,
    pub X24: u64,
    pub X25: u64,
    pub X26: u64,
    pub X27: u64,
    pub X28: u64,
    pub Fp: u64,
    pub Lr: u64,
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl ::core::marker::Copy for ARM64_NT_CONTEXT_0_0 {}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl ::core::clone::Clone for ARM64_NT_CONTEXT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl ::core::fmt::Debug for ARM64_NT_CONTEXT_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ARM64_NT_CONTEXT_0_0")
            .field("X0", &self.X0)
            .field("X1", &self.X1)
            .field("X2", &self.X2)
            .field("X3", &self.X3)
            .field("X4", &self.X4)
            .field("X5", &self.X5)
            .field("X6", &self.X6)
            .field("X7", &self.X7)
            .field("X8", &self.X8)
            .field("X9", &self.X9)
            .field("X10", &self.X10)
            .field("X11", &self.X11)
            .field("X12", &self.X12)
            .field("X13", &self.X13)
            .field("X14", &self.X14)
            .field("X15", &self.X15)
            .field("X16", &self.X16)
            .field("X17", &self.X17)
            .field("X18", &self.X18)
            .field("X19", &self.X19)
            .field("X20", &self.X20)
            .field("X21", &self.X21)
            .field("X22", &self.X22)
            .field("X23", &self.X23)
            .field("X24", &self.X24)
            .field("X25", &self.X25)
            .field("X26", &self.X26)
            .field("X27", &self.X27)
            .field("X28", &self.X28)
            .field("Fp", &self.Fp)
            .field("Lr", &self.Lr)
            .finish()
    }
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl ::windows::core::TypeKind for ARM64_NT_CONTEXT_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for ARM64_NT_CONTEXT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.X0 == other.X0 && self.X1 == other.X1 && self.X2 == other.X2 && self.X3 == other.X3 && self.X4 == other.X4 && self.X5 == other.X5 && self.X6 == other.X6 && self.X7 == other.X7 && self.X8 == other.X8 && self.X9 == other.X9 && self.X10 == other.X10 && self.X11 == other.X11 && self.X12 == other.X12 && self.X13 == other.X13 && self.X14 == other.X14 && self.X15 == other.X15 && self.X16 == other.X16 && self.X17 == other.X17 && self.X18 == other.X18 && self.X19 == other.X19 && self.X20 == other.X20 && self.X21 == other.X21 && self.X22 == other.X22 && self.X23 == other.X23 && self.X24 == other.X24 && self.X25 == other.X25 && self.X26 == other.X26 && self.X27 == other.X27 && self.X28 == other.X28 && self.Fp == other.Fp && self.Lr == other.Lr
    }
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl ::core::cmp::Eq for ARM64_NT_CONTEXT_0_0 {}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl ::core::default::Default for ARM64_NT_CONTEXT_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub union ARM64_NT_NEON128 {
    pub Anonymous: ARM64_NT_NEON128_0,
    pub D: [f64; 2],
    pub S: [f32; 4],
    pub H: [u16; 8],
    pub B: [u8; 16],
}
impl ::core::marker::Copy for ARM64_NT_NEON128 {}
impl ::core::clone::Clone for ARM64_NT_NEON128 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for ARM64_NT_NEON128 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for ARM64_NT_NEON128 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct ARM64_NT_NEON128_0 {
    pub Low: u64,
    pub High: i64,
}
impl ::core::marker::Copy for ARM64_NT_NEON128_0 {}
impl ::core::clone::Clone for ARM64_NT_NEON128_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ARM64_NT_NEON128_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ARM64_NT_NEON128_0").field("Low", &self.Low).field("High", &self.High).finish()
    }
}
impl ::windows::core::TypeKind for ARM64_NT_NEON128_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ARM64_NT_NEON128_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Low == other.Low && self.High == other.High
    }
}
impl ::core::cmp::Eq for ARM64_NT_NEON128_0 {}
impl ::core::default::Default for ARM64_NT_NEON128_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "Win32_System_Kernel")]
pub struct CONTEXT {
    pub ContextFlags: u32,
    pub Cpsr: u32,
    pub Anonymous: CONTEXT_0,
    pub Sp: u64,
    pub Pc: u64,
    pub V: [ARM64_NT_NEON128; 32],
    pub Fpcr: u32,
    pub Fpsr: u32,
    pub Bcr: [u32; 8],
    pub Bvr: [u64; 8],
    pub Wcr: [u32; 2],
    pub Wvr: [u64; 2],
}
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::marker::Copy for CONTEXT {}
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::clone::Clone for CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::windows::core::TypeKind for CONTEXT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::default::Default for CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "Win32_System_Kernel")]
pub union CONTEXT_0 {
    pub Anonymous: CONTEXT_0_0,
    pub X: [u64; 31],
}
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::marker::Copy for CONTEXT_0 {}
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::clone::Clone for CONTEXT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::windows::core::TypeKind for CONTEXT_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::default::Default for CONTEXT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "Win32_System_Kernel")]
pub struct CONTEXT_0_0 {
    pub X0: u64,
    pub X1: u64,
    pub X2: u64,
    pub X3: u64,
    pub X4: u64,
    pub X5: u64,
    pub X6: u64,
    pub X7: u64,
    pub X8: u64,
    pub X9: u64,
    pub X10: u64,
    pub X11: u64,
    pub X12: u64,
    pub X13: u64,
    pub X14: u64,
    pub X15: u64,
    pub X16: u64,
    pub X17: u64,
    pub X18: u64,
    pub X19: u64,
    pub X20: u64,
    pub X21: u64,
    pub X22: u64,
    pub X23: u64,
    pub X24: u64,
    pub X25: u64,
    pub X26: u64,
    pub X27: u64,
    pub X28: u64,
    pub Fp: u64,
    pub Lr: u64,
}
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::marker::Copy for CONTEXT_0_0 {}
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::clone::Clone for CONTEXT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::fmt::Debug for CONTEXT_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONTEXT_0_0")
            .field("X0", &self.X0)
            .field("X1", &self.X1)
            .field("X2", &self.X2)
            .field("X3", &self.X3)
            .field("X4", &self.X4)
            .field("X5", &self.X5)
            .field("X6", &self.X6)
            .field("X7", &self.X7)
            .field("X8", &self.X8)
            .field("X9", &self.X9)
            .field("X10", &self.X10)
            .field("X11", &self.X11)
            .field("X12", &self.X12)
            .field("X13", &self.X13)
            .field("X14", &self.X14)
            .field("X15", &self.X15)
            .field("X16", &self.X16)
            .field("X17", &self.X17)
            .field("X18", &self.X18)
            .field("X19", &self.X19)
            .field("X20", &self.X20)
            .field("X21", &self.X21)
            .field("X22", &self.X22)
            .field("X23", &self.X23)
            .field("X24", &self.X24)
            .field("X25", &self.X25)
            .field("X26", &self.X26)
            .field("X27", &self.X27)
            .field("X28", &self.X28)
            .field("Fp", &self.Fp)
            .field("Lr", &self.Lr)
            .finish()
    }
}
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::windows::core::TypeKind for CONTEXT_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::cmp::PartialEq for CONTEXT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.X0 == other.X0 && self.X1 == other.X1 && self.X2 == other.X2 && self.X3 == other.X3 && self.X4 == other.X4 && self.X5 == other.X5 && self.X6 == other.X6 && self.X7 == other.X7 && self.X8 == other.X8 && self.X9 == other.X9 && self.X10 == other.X10 && self.X11 == other.X11 && self.X12 == other.X12 && self.X13 == other.X13 && self.X14 == other.X14 && self.X15 == other.X15 && self.X16 == other.X16 && self.X17 == other.X17 && self.X18 == other.X18 && self.X19 == other.X19 && self.X20 == other.X20 && self.X21 == other.X21 && self.X22 == other.X22 && self.X23 == other.X23 && self.X24 == other.X24 && self.X25 == other.X25 && self.X26 == other.X26 && self.X27 == other.X27 && self.X28 == other.X28 && self.Fp == other.Fp && self.Lr == other.Lr
    }
}
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::cmp::Eq for CONTEXT_0_0 {}
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::default::Default for CONTEXT_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "x86_64")]
#[cfg(feature = "Win32_System_Kernel")]
pub struct CONTEXT {
    pub P1Home: u64,
    pub P2Home: u64,
    pub P3Home: u64,
    pub P4Home: u64,
    pub P5Home: u64,
    pub P6Home: u64,
    pub ContextFlags: u32,
    pub MxCsr: u32,
    pub SegCs: u16,
    pub SegDs: u16,
    pub SegEs: u16,
    pub SegFs: u16,
    pub SegGs: u16,
    pub SegSs: u16,
    pub EFlags: u32,
    pub Dr0: u64,
    pub Dr1: u64,
    pub Dr2: u64,
    pub Dr3: u64,
    pub Dr6: u64,
    pub Dr7: u64,
    pub Rax: u64,
    pub Rcx: u64,
    pub Rdx: u64,
    pub Rbx: u64,
    pub Rsp: u64,
    pub Rbp: u64,
    pub Rsi: u64,
    pub Rdi: u64,
    pub R8: u64,
    pub R9: u64,
    pub R10: u64,
    pub R11: u64,
    pub R12: u64,
    pub R13: u64,
    pub R14: u64,
    pub R15: u64,
    pub Rip: u64,
    pub Anonymous: CONTEXT_0,
    pub VectorRegister: [M128A; 26],
    pub VectorControl: u64,
    pub DebugControl: u64,
    pub LastBranchToRip: u64,
    pub LastBranchFromRip: u64,
    pub LastExceptionToRip: u64,
    pub LastExceptionFromRip: u64,
}
#[cfg(target_arch = "x86_64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::marker::Copy for CONTEXT {}
#[cfg(target_arch = "x86_64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::clone::Clone for CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86_64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::windows::core::TypeKind for CONTEXT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86_64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::default::Default for CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "x86_64")]
#[cfg(feature = "Win32_System_Kernel")]
pub union CONTEXT_0 {
    pub FltSave: XSAVE_FORMAT,
    pub Anonymous: CONTEXT_0_0,
}
#[cfg(target_arch = "x86_64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::marker::Copy for CONTEXT_0 {}
#[cfg(target_arch = "x86_64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::clone::Clone for CONTEXT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86_64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::windows::core::TypeKind for CONTEXT_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86_64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::default::Default for CONTEXT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "x86_64")]
#[cfg(feature = "Win32_System_Kernel")]
pub struct CONTEXT_0_0 {
    pub Header: [M128A; 2],
    pub Legacy: [M128A; 8],
    pub Xmm0: M128A,
    pub Xmm1: M128A,
    pub Xmm2: M128A,
    pub Xmm3: M128A,
    pub Xmm4: M128A,
    pub Xmm5: M128A,
    pub Xmm6: M128A,
    pub Xmm7: M128A,
    pub Xmm8: M128A,
    pub Xmm9: M128A,
    pub Xmm10: M128A,
    pub Xmm11: M128A,
    pub Xmm12: M128A,
    pub Xmm13: M128A,
    pub Xmm14: M128A,
    pub Xmm15: M128A,
}
#[cfg(target_arch = "x86_64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::marker::Copy for CONTEXT_0_0 {}
#[cfg(target_arch = "x86_64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::clone::Clone for CONTEXT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86_64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::fmt::Debug for CONTEXT_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONTEXT_0_0")
            .field("Header", &self.Header)
            .field("Legacy", &self.Legacy)
            .field("Xmm0", &self.Xmm0)
            .field("Xmm1", &self.Xmm1)
            .field("Xmm2", &self.Xmm2)
            .field("Xmm3", &self.Xmm3)
            .field("Xmm4", &self.Xmm4)
            .field("Xmm5", &self.Xmm5)
            .field("Xmm6", &self.Xmm6)
            .field("Xmm7", &self.Xmm7)
            .field("Xmm8", &self.Xmm8)
            .field("Xmm9", &self.Xmm9)
            .field("Xmm10", &self.Xmm10)
            .field("Xmm11", &self.Xmm11)
            .field("Xmm12", &self.Xmm12)
            .field("Xmm13", &self.Xmm13)
            .field("Xmm14", &self.Xmm14)
            .field("Xmm15", &self.Xmm15)
            .finish()
    }
}
#[cfg(target_arch = "x86_64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::windows::core::TypeKind for CONTEXT_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86_64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::cmp::PartialEq for CONTEXT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Legacy == other.Legacy && self.Xmm0 == other.Xmm0 && self.Xmm1 == other.Xmm1 && self.Xmm2 == other.Xmm2 && self.Xmm3 == other.Xmm3 && self.Xmm4 == other.Xmm4 && self.Xmm5 == other.Xmm5 && self.Xmm6 == other.Xmm6 && self.Xmm7 == other.Xmm7 && self.Xmm8 == other.Xmm8 && self.Xmm9 == other.Xmm9 && self.Xmm10 == other.Xmm10 && self.Xmm11 == other.Xmm11 && self.Xmm12 == other.Xmm12 && self.Xmm13 == other.Xmm13 && self.Xmm14 == other.Xmm14 && self.Xmm15 == other.Xmm15
    }
}
#[cfg(target_arch = "x86_64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::cmp::Eq for CONTEXT_0_0 {}
#[cfg(target_arch = "x86_64")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::default::Default for CONTEXT_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Kernel")]
pub struct CONTEXT {
    pub ContextFlags: u32,
    pub Dr0: u32,
    pub Dr1: u32,
    pub Dr2: u32,
    pub Dr3: u32,
    pub Dr6: u32,
    pub Dr7: u32,
    pub FloatSave: super::super::Kernel::FLOATING_SAVE_AREA,
    pub SegGs: u32,
    pub SegFs: u32,
    pub SegEs: u32,
    pub SegDs: u32,
    pub Edi: u32,
    pub Esi: u32,
    pub Ebx: u32,
    pub Edx: u32,
    pub Ecx: u32,
    pub Eax: u32,
    pub Ebp: u32,
    pub Eip: u32,
    pub SegCs: u32,
    pub EFlags: u32,
    pub Esp: u32,
    pub SegSs: u32,
    pub ExtendedRegisters: [u8; 512],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::marker::Copy for CONTEXT {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::clone::Clone for CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::windows::core::TypeKind for CONTEXT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::default::Default for CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub union CPU_INFORMATION {
    pub X86CpuInfo: CPU_INFORMATION_1,
    pub OtherCpuInfo: CPU_INFORMATION_0,
}
impl ::core::marker::Copy for CPU_INFORMATION {}
impl ::core::clone::Clone for CPU_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for CPU_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for CPU_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct CPU_INFORMATION_0 {
    pub ProcessorFeatures: [u64; 2],
}
impl ::core::marker::Copy for CPU_INFORMATION_0 {}
impl ::core::clone::Clone for CPU_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for CPU_INFORMATION_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for CPU_INFORMATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct CPU_INFORMATION_1 {
    pub VendorId: [u32; 3],
    pub VersionInformation: u32,
    pub FeatureInformation: u32,
    pub AMDExtendedCpuFeatures: u32,
}
impl ::core::marker::Copy for CPU_INFORMATION_1 {}
impl ::core::clone::Clone for CPU_INFORMATION_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CPU_INFORMATION_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CPU_INFORMATION_1").field("VendorId", &self.VendorId).field("VersionInformation", &self.VersionInformation).field("FeatureInformation", &self.FeatureInformation).field("AMDExtendedCpuFeatures", &self.AMDExtendedCpuFeatures).finish()
    }
}
impl ::windows::core::TypeKind for CPU_INFORMATION_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CPU_INFORMATION_1 {
    fn eq(&self, other: &Self) -> bool {
        self.VendorId == other.VendorId && self.VersionInformation == other.VersionInformation && self.FeatureInformation == other.FeatureInformation && self.AMDExtendedCpuFeatures == other.AMDExtendedCpuFeatures
    }
}
impl ::core::cmp::Eq for CPU_INFORMATION_1 {}
impl ::core::default::Default for CPU_INFORMATION_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Threading\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
pub struct CREATE_PROCESS_DEBUG_INFO {
    pub hFile: super::super::super::Foundation::HANDLE,
    pub hProcess: super::super::super::Foundation::HANDLE,
    pub hThread: super::super::super::Foundation::HANDLE,
    pub lpBaseOfImage: *mut ::core::ffi::c_void,
    pub dwDebugInfoFileOffset: u32,
    pub nDebugInfoSize: u32,
    pub lpThreadLocalBase: *mut ::core::ffi::c_void,
    pub lpStartAddress: super::super::Threading::LPTHREAD_START_ROUTINE,
    pub lpImageName: *mut ::core::ffi::c_void,
    pub fUnicode: u16,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
impl ::core::marker::Copy for CREATE_PROCESS_DEBUG_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
impl ::core::clone::Clone for CREATE_PROCESS_DEBUG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
impl ::core::fmt::Debug for CREATE_PROCESS_DEBUG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREATE_PROCESS_DEBUG_INFO").field("hFile", &self.hFile).field("hProcess", &self.hProcess).field("hThread", &self.hThread).field("lpBaseOfImage", &self.lpBaseOfImage).field("dwDebugInfoFileOffset", &self.dwDebugInfoFileOffset).field("nDebugInfoSize", &self.nDebugInfoSize).field("lpThreadLocalBase", &self.lpThreadLocalBase).field("lpImageName", &self.lpImageName).field("fUnicode", &self.fUnicode).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
impl ::windows::core::TypeKind for CREATE_PROCESS_DEBUG_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
impl ::core::default::Default for CREATE_PROCESS_DEBUG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Threading\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
pub struct CREATE_THREAD_DEBUG_INFO {
    pub hThread: super::super::super::Foundation::HANDLE,
    pub lpThreadLocalBase: *mut ::core::ffi::c_void,
    pub lpStartAddress: super::super::Threading::LPTHREAD_START_ROUTINE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
impl ::core::marker::Copy for CREATE_THREAD_DEBUG_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
impl ::core::clone::Clone for CREATE_THREAD_DEBUG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
impl ::core::fmt::Debug for CREATE_THREAD_DEBUG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREATE_THREAD_DEBUG_INFO").field("hThread", &self.hThread).field("lpThreadLocalBase", &self.lpThreadLocalBase).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
impl ::windows::core::TypeKind for CREATE_THREAD_DEBUG_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
impl ::core::default::Default for CREATE_THREAD_DEBUG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct DBGHELP_DATA_REPORT_STRUCT {
    pub pBinPathNonExist: ::windows::core::PCWSTR,
    pub pSymbolPathNonExist: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for DBGHELP_DATA_REPORT_STRUCT {}
impl ::core::clone::Clone for DBGHELP_DATA_REPORT_STRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DBGHELP_DATA_REPORT_STRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DBGHELP_DATA_REPORT_STRUCT").field("pBinPathNonExist", &self.pBinPathNonExist).field("pSymbolPathNonExist", &self.pSymbolPathNonExist).finish()
    }
}
impl ::windows::core::TypeKind for DBGHELP_DATA_REPORT_STRUCT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DBGHELP_DATA_REPORT_STRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.pBinPathNonExist == other.pBinPathNonExist && self.pSymbolPathNonExist == other.pSymbolPathNonExist
    }
}
impl ::core::cmp::Eq for DBGHELP_DATA_REPORT_STRUCT {}
impl ::core::default::Default for DBGHELP_DATA_REPORT_STRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Threading\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
pub struct DEBUG_EVENT {
    pub dwDebugEventCode: DEBUG_EVENT_CODE,
    pub dwProcessId: u32,
    pub dwThreadId: u32,
    pub u: DEBUG_EVENT_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
impl ::core::marker::Copy for DEBUG_EVENT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
impl ::core::clone::Clone for DEBUG_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
impl ::windows::core::TypeKind for DEBUG_EVENT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
impl ::core::default::Default for DEBUG_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Threading\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
pub union DEBUG_EVENT_0 {
    pub Exception: EXCEPTION_DEBUG_INFO,
    pub CreateThread: CREATE_THREAD_DEBUG_INFO,
    pub CreateProcessInfo: CREATE_PROCESS_DEBUG_INFO,
    pub ExitThread: EXIT_THREAD_DEBUG_INFO,
    pub ExitProcess: EXIT_PROCESS_DEBUG_INFO,
    pub LoadDll: LOAD_DLL_DEBUG_INFO,
    pub UnloadDll: UNLOAD_DLL_DEBUG_INFO,
    pub DebugString: OUTPUT_DEBUG_STRING_INFO,
    pub RipInfo: RIP_INFO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
impl ::core::marker::Copy for DEBUG_EVENT_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
impl ::core::clone::Clone for DEBUG_EVENT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
impl ::windows::core::TypeKind for DEBUG_EVENT_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
impl ::core::default::Default for DEBUG_EVENT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct DISPATCHER_CONTEXT {
    pub ControlPc: usize,
    pub ImageBase: usize,
    pub FunctionEntry: *mut IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY,
    pub EstablisherFrame: usize,
    pub TargetPc: usize,
    pub ContextRecord: *mut CONTEXT,
    pub LanguageHandler: super::super::Kernel::EXCEPTION_ROUTINE,
    pub HandlerData: *mut ::core::ffi::c_void,
    pub HistoryTable: *mut UNWIND_HISTORY_TABLE,
    pub ScopeIndex: u32,
    pub ControlPcIsUnwound: super::super::super::Foundation::BOOLEAN,
    pub NonVolatileRegisters: *mut u8,
}
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for DISPATCHER_CONTEXT {}
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for DISPATCHER_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::fmt::Debug for DISPATCHER_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISPATCHER_CONTEXT")
            .field("ControlPc", &self.ControlPc)
            .field("ImageBase", &self.ImageBase)
            .field("FunctionEntry", &self.FunctionEntry)
            .field("EstablisherFrame", &self.EstablisherFrame)
            .field("TargetPc", &self.TargetPc)
            .field("ContextRecord", &self.ContextRecord)
            .field("HandlerData", &self.HandlerData)
            .field("HistoryTable", &self.HistoryTable)
            .field("ScopeIndex", &self.ScopeIndex)
            .field("ControlPcIsUnwound", &self.ControlPcIsUnwound)
            .field("NonVolatileRegisters", &self.NonVolatileRegisters)
            .finish()
    }
}
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::windows::core::TypeKind for DISPATCHER_CONTEXT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for DISPATCHER_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "x86_64")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct DISPATCHER_CONTEXT {
    pub ControlPc: u64,
    pub ImageBase: u64,
    pub FunctionEntry: *mut IMAGE_RUNTIME_FUNCTION_ENTRY,
    pub EstablisherFrame: u64,
    pub TargetIp: u64,
    pub ContextRecord: *mut CONTEXT,
    pub LanguageHandler: super::super::Kernel::EXCEPTION_ROUTINE,
    pub HandlerData: *mut ::core::ffi::c_void,
    pub HistoryTable: *mut UNWIND_HISTORY_TABLE,
    pub ScopeIndex: u32,
    pub Fill0: u32,
}
#[cfg(target_arch = "x86_64")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for DISPATCHER_CONTEXT {}
#[cfg(target_arch = "x86_64")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for DISPATCHER_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86_64")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::fmt::Debug for DISPATCHER_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISPATCHER_CONTEXT").field("ControlPc", &self.ControlPc).field("ImageBase", &self.ImageBase).field("FunctionEntry", &self.FunctionEntry).field("EstablisherFrame", &self.EstablisherFrame).field("TargetIp", &self.TargetIp).field("ContextRecord", &self.ContextRecord).field("HandlerData", &self.HandlerData).field("HistoryTable", &self.HistoryTable).field("ScopeIndex", &self.ScopeIndex).field("Fill0", &self.Fill0).finish()
    }
}
#[cfg(target_arch = "x86_64")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::windows::core::TypeKind for DISPATCHER_CONTEXT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86_64")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for DISPATCHER_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub union DUMP_FILE_ATTRIBUTES {
    pub Anonymous: DUMP_FILE_ATTRIBUTES_0,
    pub Attributes: u32,
}
impl ::core::marker::Copy for DUMP_FILE_ATTRIBUTES {}
impl ::core::clone::Clone for DUMP_FILE_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DUMP_FILE_ATTRIBUTES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DUMP_FILE_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct DUMP_FILE_ATTRIBUTES_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for DUMP_FILE_ATTRIBUTES_0 {}
impl ::core::clone::Clone for DUMP_FILE_ATTRIBUTES_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DUMP_FILE_ATTRIBUTES_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DUMP_FILE_ATTRIBUTES_0").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows::core::TypeKind for DUMP_FILE_ATTRIBUTES_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DUMP_FILE_ATTRIBUTES_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for DUMP_FILE_ATTRIBUTES_0 {}
impl ::core::default::Default for DUMP_FILE_ATTRIBUTES_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DUMP_HEADER32 {
    pub Signature: u32,
    pub ValidDump: u32,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub DirectoryTableBase: u32,
    pub PfnDataBase: u32,
    pub PsLoadedModuleList: u32,
    pub PsActiveProcessHead: u32,
    pub MachineImageType: u32,
    pub NumberProcessors: u32,
    pub BugCheckCode: u32,
    pub BugCheckParameter1: u32,
    pub BugCheckParameter2: u32,
    pub BugCheckParameter3: u32,
    pub BugCheckParameter4: u32,
    pub VersionUser: [u8; 32],
    pub PaeEnabled: u8,
    pub KdSecondaryVersion: u8,
    pub Spare3: [u8; 2],
    pub KdDebuggerDataBlock: u32,
    pub Anonymous: DUMP_HEADER32_0,
    pub ContextRecord: [u8; 1200],
    pub Exception: EXCEPTION_RECORD32,
    pub Comment: [u8; 128],
    pub Attributes: DUMP_FILE_ATTRIBUTES,
    pub BootId: u32,
    pub _reserved0: [u8; 1760],
    pub DumpType: u32,
    pub MiniDumpFields: u32,
    pub SecondaryDataState: u32,
    pub ProductType: u32,
    pub SuiteMask: u32,
    pub WriterStatus: u32,
    pub RequiredDumpSpace: i64,
    pub _reserved2: [u8; 16],
    pub SystemUpTime: i64,
    pub SystemTime: i64,
    pub _reserved3: [u8; 56],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DUMP_HEADER32 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DUMP_HEADER32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DUMP_HEADER32 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DUMP_HEADER32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union DUMP_HEADER32_0 {
    pub PhysicalMemoryBlock: PHYSICAL_MEMORY_DESCRIPTOR32,
    pub PhysicalMemoryBlockBuffer: [u8; 700],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DUMP_HEADER32_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DUMP_HEADER32_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DUMP_HEADER32_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DUMP_HEADER32_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DUMP_HEADER64 {
    pub Signature: u32,
    pub ValidDump: u32,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub DirectoryTableBase: u64,
    pub PfnDataBase: u64,
    pub PsLoadedModuleList: u64,
    pub PsActiveProcessHead: u64,
    pub MachineImageType: u32,
    pub NumberProcessors: u32,
    pub BugCheckCode: u32,
    pub BugCheckParameter1: u64,
    pub BugCheckParameter2: u64,
    pub BugCheckParameter3: u64,
    pub BugCheckParameter4: u64,
    pub VersionUser: [u8; 32],
    pub KdDebuggerDataBlock: u64,
    pub Anonymous: DUMP_HEADER64_0,
    pub ContextRecord: [u8; 3000],
    pub Exception: EXCEPTION_RECORD64,
    pub DumpType: u32,
    pub RequiredDumpSpace: i64,
    pub SystemTime: i64,
    pub Comment: [u8; 128],
    pub SystemUpTime: i64,
    pub MiniDumpFields: u32,
    pub SecondaryDataState: u32,
    pub ProductType: u32,
    pub SuiteMask: u32,
    pub WriterStatus: u32,
    pub Unused1: u8,
    pub KdSecondaryVersion: u8,
    pub Unused: [u8; 2],
    pub Attributes: DUMP_FILE_ATTRIBUTES,
    pub BootId: u32,
    pub _reserved0: [u8; 4008],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DUMP_HEADER64 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DUMP_HEADER64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DUMP_HEADER64 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DUMP_HEADER64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union DUMP_HEADER64_0 {
    pub PhysicalMemoryBlock: PHYSICAL_MEMORY_DESCRIPTOR64,
    pub PhysicalMemoryBlockBuffer: [u8; 700],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DUMP_HEADER64_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DUMP_HEADER64_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DUMP_HEADER64_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DUMP_HEADER64_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct DebugPropertyInfo {
    pub m_dwValidFields: u32,
    pub m_bstrName: ::std::mem::ManuallyDrop<::windows::core::BSTR>,
    pub m_bstrType: ::std::mem::ManuallyDrop<::windows::core::BSTR>,
    pub m_bstrValue: ::std::mem::ManuallyDrop<::windows::core::BSTR>,
    pub m_bstrFullName: ::std::mem::ManuallyDrop<::windows::core::BSTR>,
    pub m_dwAttrib: u32,
    pub m_pDebugProp: ::std::mem::ManuallyDrop<::core::option::Option<IDebugProperty>>,
}
impl ::core::clone::Clone for DebugPropertyInfo {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for DebugPropertyInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DebugPropertyInfo").field("m_dwValidFields", &self.m_dwValidFields).field("m_bstrName", &self.m_bstrName).field("m_bstrType", &self.m_bstrType).field("m_bstrValue", &self.m_bstrValue).field("m_bstrFullName", &self.m_bstrFullName).field("m_dwAttrib", &self.m_dwAttrib).field("m_pDebugProp", &self.m_pDebugProp).finish()
    }
}
impl ::windows::core::TypeKind for DebugPropertyInfo {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DebugPropertyInfo {
    fn eq(&self, other: &Self) -> bool {
        self.m_dwValidFields == other.m_dwValidFields && self.m_bstrName == other.m_bstrName && self.m_bstrType == other.m_bstrType && self.m_bstrValue == other.m_bstrValue && self.m_bstrFullName == other.m_bstrFullName && self.m_dwAttrib == other.m_dwAttrib && self.m_pDebugProp == other.m_pDebugProp
    }
}
impl ::core::cmp::Eq for DebugPropertyInfo {}
impl ::core::default::Default for DebugPropertyInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EXCEPTION_DEBUG_INFO {
    pub ExceptionRecord: EXCEPTION_RECORD,
    pub dwFirstChance: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EXCEPTION_DEBUG_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EXCEPTION_DEBUG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EXCEPTION_DEBUG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXCEPTION_DEBUG_INFO").field("ExceptionRecord", &self.ExceptionRecord).field("dwFirstChance", &self.dwFirstChance).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for EXCEPTION_DEBUG_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EXCEPTION_DEBUG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ExceptionRecord == other.ExceptionRecord && self.dwFirstChance == other.dwFirstChance
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EXCEPTION_DEBUG_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EXCEPTION_DEBUG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct EXCEPTION_POINTERS {
    pub ExceptionRecord: *mut EXCEPTION_RECORD,
    pub ContextRecord: *mut CONTEXT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for EXCEPTION_POINTERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for EXCEPTION_POINTERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::fmt::Debug for EXCEPTION_POINTERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXCEPTION_POINTERS").field("ExceptionRecord", &self.ExceptionRecord).field("ContextRecord", &self.ContextRecord).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::windows::core::TypeKind for EXCEPTION_POINTERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::PartialEq for EXCEPTION_POINTERS {
    fn eq(&self, other: &Self) -> bool {
        self.ExceptionRecord == other.ExceptionRecord && self.ContextRecord == other.ContextRecord
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::Eq for EXCEPTION_POINTERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for EXCEPTION_POINTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EXCEPTION_RECORD {
    pub ExceptionCode: super::super::super::Foundation::NTSTATUS,
    pub ExceptionFlags: u32,
    pub ExceptionRecord: *mut EXCEPTION_RECORD,
    pub ExceptionAddress: *mut ::core::ffi::c_void,
    pub NumberParameters: u32,
    pub ExceptionInformation: [usize; 15],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EXCEPTION_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EXCEPTION_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EXCEPTION_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXCEPTION_RECORD").field("ExceptionCode", &self.ExceptionCode).field("ExceptionFlags", &self.ExceptionFlags).field("ExceptionRecord", &self.ExceptionRecord).field("ExceptionAddress", &self.ExceptionAddress).field("NumberParameters", &self.NumberParameters).field("ExceptionInformation", &self.ExceptionInformation).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for EXCEPTION_RECORD {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EXCEPTION_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.ExceptionCode == other.ExceptionCode && self.ExceptionFlags == other.ExceptionFlags && self.ExceptionRecord == other.ExceptionRecord && self.ExceptionAddress == other.ExceptionAddress && self.NumberParameters == other.NumberParameters && self.ExceptionInformation == other.ExceptionInformation
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EXCEPTION_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EXCEPTION_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EXCEPTION_RECORD32 {
    pub ExceptionCode: super::super::super::Foundation::NTSTATUS,
    pub ExceptionFlags: u32,
    pub ExceptionRecord: u32,
    pub ExceptionAddress: u32,
    pub NumberParameters: u32,
    pub ExceptionInformation: [u32; 15],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EXCEPTION_RECORD32 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EXCEPTION_RECORD32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EXCEPTION_RECORD32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXCEPTION_RECORD32").field("ExceptionCode", &self.ExceptionCode).field("ExceptionFlags", &self.ExceptionFlags).field("ExceptionRecord", &self.ExceptionRecord).field("ExceptionAddress", &self.ExceptionAddress).field("NumberParameters", &self.NumberParameters).field("ExceptionInformation", &self.ExceptionInformation).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for EXCEPTION_RECORD32 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EXCEPTION_RECORD32 {
    fn eq(&self, other: &Self) -> bool {
        self.ExceptionCode == other.ExceptionCode && self.ExceptionFlags == other.ExceptionFlags && self.ExceptionRecord == other.ExceptionRecord && self.ExceptionAddress == other.ExceptionAddress && self.NumberParameters == other.NumberParameters && self.ExceptionInformation == other.ExceptionInformation
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EXCEPTION_RECORD32 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EXCEPTION_RECORD32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EXCEPTION_RECORD64 {
    pub ExceptionCode: super::super::super::Foundation::NTSTATUS,
    pub ExceptionFlags: u32,
    pub ExceptionRecord: u64,
    pub ExceptionAddress: u64,
    pub NumberParameters: u32,
    pub __unusedAlignment: u32,
    pub ExceptionInformation: [u64; 15],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EXCEPTION_RECORD64 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EXCEPTION_RECORD64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EXCEPTION_RECORD64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXCEPTION_RECORD64").field("ExceptionCode", &self.ExceptionCode).field("ExceptionFlags", &self.ExceptionFlags).field("ExceptionRecord", &self.ExceptionRecord).field("ExceptionAddress", &self.ExceptionAddress).field("NumberParameters", &self.NumberParameters).field("__unusedAlignment", &self.__unusedAlignment).field("ExceptionInformation", &self.ExceptionInformation).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for EXCEPTION_RECORD64 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EXCEPTION_RECORD64 {
    fn eq(&self, other: &Self) -> bool {
        self.ExceptionCode == other.ExceptionCode && self.ExceptionFlags == other.ExceptionFlags && self.ExceptionRecord == other.ExceptionRecord && self.ExceptionAddress == other.ExceptionAddress && self.NumberParameters == other.NumberParameters && self.__unusedAlignment == other.__unusedAlignment && self.ExceptionInformation == other.ExceptionInformation
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EXCEPTION_RECORD64 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EXCEPTION_RECORD64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct EXIT_PROCESS_DEBUG_INFO {
    pub dwExitCode: u32,
}
impl ::core::marker::Copy for EXIT_PROCESS_DEBUG_INFO {}
impl ::core::clone::Clone for EXIT_PROCESS_DEBUG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EXIT_PROCESS_DEBUG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXIT_PROCESS_DEBUG_INFO").field("dwExitCode", &self.dwExitCode).finish()
    }
}
impl ::windows::core::TypeKind for EXIT_PROCESS_DEBUG_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for EXIT_PROCESS_DEBUG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwExitCode == other.dwExitCode
    }
}
impl ::core::cmp::Eq for EXIT_PROCESS_DEBUG_INFO {}
impl ::core::default::Default for EXIT_PROCESS_DEBUG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct EXIT_THREAD_DEBUG_INFO {
    pub dwExitCode: u32,
}
impl ::core::marker::Copy for EXIT_THREAD_DEBUG_INFO {}
impl ::core::clone::Clone for EXIT_THREAD_DEBUG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EXIT_THREAD_DEBUG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXIT_THREAD_DEBUG_INFO").field("dwExitCode", &self.dwExitCode).finish()
    }
}
impl ::windows::core::TypeKind for EXIT_THREAD_DEBUG_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for EXIT_THREAD_DEBUG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwExitCode == other.dwExitCode
    }
}
impl ::core::cmp::Eq for EXIT_THREAD_DEBUG_INFO {}
impl ::core::default::Default for EXIT_THREAD_DEBUG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub struct ExtendedDebugPropertyInfo {
    pub dwValidFields: u32,
    pub pszName: ::windows::core::PWSTR,
    pub pszType: ::windows::core::PWSTR,
    pub pszValue: ::windows::core::PWSTR,
    pub pszFullName: ::windows::core::PWSTR,
    pub dwAttrib: u32,
    pub pDebugProp: ::std::mem::ManuallyDrop<::core::option::Option<IDebugProperty>>,
    pub nDISPID: u32,
    pub nType: u32,
    pub varValue: super::super::Com::VARIANT,
    pub plbValue: ::std::mem::ManuallyDrop<::core::option::Option<super::super::Com::StructuredStorage::ILockBytes>>,
    pub pDebugExtProp: ::std::mem::ManuallyDrop<::core::option::Option<IDebugExtendedProperty>>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for ExtendedDebugPropertyInfo {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::windows::core::TypeKind for ExtendedDebugPropertyInfo {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::core::default::Default for ExtendedDebugPropertyInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct FPO_DATA {
    pub ulOffStart: u32,
    pub cbProcSize: u32,
    pub cdwLocals: u32,
    pub cdwParams: u16,
    pub _bitfield: u16,
}
impl ::core::marker::Copy for FPO_DATA {}
impl ::core::clone::Clone for FPO_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FPO_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FPO_DATA").field("ulOffStart", &self.ulOffStart).field("cbProcSize", &self.cbProcSize).field("cdwLocals", &self.cdwLocals).field("cdwParams", &self.cdwParams).field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows::core::TypeKind for FPO_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FPO_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.ulOffStart == other.ulOffStart && self.cbProcSize == other.cbProcSize && self.cdwLocals == other.cdwLocals && self.cdwParams == other.cdwParams && self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for FPO_DATA {}
impl ::core::default::Default for FPO_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct IMAGEHLP_CBA_EVENT {
    pub severity: IMAGEHLP_CBA_EVENT_SEVERITY,
    pub code: u32,
    pub desc: ::windows::core::PSTR,
    pub object: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for IMAGEHLP_CBA_EVENT {}
impl ::core::clone::Clone for IMAGEHLP_CBA_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAGEHLP_CBA_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_CBA_EVENT").field("severity", &self.severity).field("code", &self.code).field("desc", &self.desc).field("object", &self.object).finish()
    }
}
impl ::windows::core::TypeKind for IMAGEHLP_CBA_EVENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IMAGEHLP_CBA_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.severity == other.severity && self.code == other.code && self.desc == other.desc && self.object == other.object
    }
}
impl ::core::cmp::Eq for IMAGEHLP_CBA_EVENT {}
impl ::core::default::Default for IMAGEHLP_CBA_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct IMAGEHLP_CBA_EVENTW {
    pub severity: IMAGEHLP_CBA_EVENT_SEVERITY,
    pub code: u32,
    pub desc: ::windows::core::PCWSTR,
    pub object: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for IMAGEHLP_CBA_EVENTW {}
impl ::core::clone::Clone for IMAGEHLP_CBA_EVENTW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAGEHLP_CBA_EVENTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_CBA_EVENTW").field("severity", &self.severity).field("code", &self.code).field("desc", &self.desc).field("object", &self.object).finish()
    }
}
impl ::windows::core::TypeKind for IMAGEHLP_CBA_EVENTW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IMAGEHLP_CBA_EVENTW {
    fn eq(&self, other: &Self) -> bool {
        self.severity == other.severity && self.code == other.code && self.desc == other.desc && self.object == other.object
    }
}
impl ::core::cmp::Eq for IMAGEHLP_CBA_EVENTW {}
impl ::core::default::Default for IMAGEHLP_CBA_EVENTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct IMAGEHLP_CBA_READ_MEMORY {
    pub addr: u64,
    pub buf: *mut ::core::ffi::c_void,
    pub bytes: u32,
    pub bytesread: *mut u32,
}
impl ::core::marker::Copy for IMAGEHLP_CBA_READ_MEMORY {}
impl ::core::clone::Clone for IMAGEHLP_CBA_READ_MEMORY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAGEHLP_CBA_READ_MEMORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_CBA_READ_MEMORY").field("addr", &self.addr).field("buf", &self.buf).field("bytes", &self.bytes).field("bytesread", &self.bytesread).finish()
    }
}
impl ::windows::core::TypeKind for IMAGEHLP_CBA_READ_MEMORY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IMAGEHLP_CBA_READ_MEMORY {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr && self.buf == other.buf && self.bytes == other.bytes && self.bytesread == other.bytesread
    }
}
impl ::core::cmp::Eq for IMAGEHLP_CBA_READ_MEMORY {}
impl ::core::default::Default for IMAGEHLP_CBA_READ_MEMORY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct IMAGEHLP_DEFERRED_SYMBOL_LOAD {
    pub SizeOfStruct: u32,
    pub BaseOfImage: u32,
    pub CheckSum: u32,
    pub TimeDateStamp: u32,
    pub FileName: [u8; 260],
    pub Reparse: super::super::super::Foundation::BOOLEAN,
    pub hFile: super::super::super::Foundation::HANDLE,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMAGEHLP_DEFERRED_SYMBOL_LOAD {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMAGEHLP_DEFERRED_SYMBOL_LOAD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMAGEHLP_DEFERRED_SYMBOL_LOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_DEFERRED_SYMBOL_LOAD").field("SizeOfStruct", &self.SizeOfStruct).field("BaseOfImage", &self.BaseOfImage).field("CheckSum", &self.CheckSum).field("TimeDateStamp", &self.TimeDateStamp).field("FileName", &self.FileName).field("Reparse", &self.Reparse).field("hFile", &self.hFile).finish()
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for IMAGEHLP_DEFERRED_SYMBOL_LOAD {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMAGEHLP_DEFERRED_SYMBOL_LOAD {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.BaseOfImage == other.BaseOfImage && self.CheckSum == other.CheckSum && self.TimeDateStamp == other.TimeDateStamp && self.FileName == other.FileName && self.Reparse == other.Reparse && self.hFile == other.hFile
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMAGEHLP_DEFERRED_SYMBOL_LOAD {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMAGEHLP_DEFERRED_SYMBOL_LOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMAGEHLP_DEFERRED_SYMBOL_LOAD64 {
    pub SizeOfStruct: u32,
    pub BaseOfImage: u64,
    pub CheckSum: u32,
    pub TimeDateStamp: u32,
    pub FileName: [u8; 260],
    pub Reparse: super::super::super::Foundation::BOOLEAN,
    pub hFile: super::super::super::Foundation::HANDLE,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMAGEHLP_DEFERRED_SYMBOL_LOAD64 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMAGEHLP_DEFERRED_SYMBOL_LOAD64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMAGEHLP_DEFERRED_SYMBOL_LOAD64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_DEFERRED_SYMBOL_LOAD64").field("SizeOfStruct", &self.SizeOfStruct).field("BaseOfImage", &self.BaseOfImage).field("CheckSum", &self.CheckSum).field("TimeDateStamp", &self.TimeDateStamp).field("FileName", &self.FileName).field("Reparse", &self.Reparse).field("hFile", &self.hFile).field("Flags", &self.Flags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for IMAGEHLP_DEFERRED_SYMBOL_LOAD64 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMAGEHLP_DEFERRED_SYMBOL_LOAD64 {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.BaseOfImage == other.BaseOfImage && self.CheckSum == other.CheckSum && self.TimeDateStamp == other.TimeDateStamp && self.FileName == other.FileName && self.Reparse == other.Reparse && self.hFile == other.hFile && self.Flags == other.Flags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMAGEHLP_DEFERRED_SYMBOL_LOAD64 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMAGEHLP_DEFERRED_SYMBOL_LOAD64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMAGEHLP_DEFERRED_SYMBOL_LOADW64 {
    pub SizeOfStruct: u32,
    pub BaseOfImage: u64,
    pub CheckSum: u32,
    pub TimeDateStamp: u32,
    pub FileName: [u16; 261],
    pub Reparse: super::super::super::Foundation::BOOLEAN,
    pub hFile: super::super::super::Foundation::HANDLE,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMAGEHLP_DEFERRED_SYMBOL_LOADW64 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMAGEHLP_DEFERRED_SYMBOL_LOADW64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMAGEHLP_DEFERRED_SYMBOL_LOADW64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_DEFERRED_SYMBOL_LOADW64").field("SizeOfStruct", &self.SizeOfStruct).field("BaseOfImage", &self.BaseOfImage).field("CheckSum", &self.CheckSum).field("TimeDateStamp", &self.TimeDateStamp).field("FileName", &self.FileName).field("Reparse", &self.Reparse).field("hFile", &self.hFile).field("Flags", &self.Flags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for IMAGEHLP_DEFERRED_SYMBOL_LOADW64 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMAGEHLP_DEFERRED_SYMBOL_LOADW64 {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.BaseOfImage == other.BaseOfImage && self.CheckSum == other.CheckSum && self.TimeDateStamp == other.TimeDateStamp && self.FileName == other.FileName && self.Reparse == other.Reparse && self.hFile == other.hFile && self.Flags == other.Flags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMAGEHLP_DEFERRED_SYMBOL_LOADW64 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMAGEHLP_DEFERRED_SYMBOL_LOADW64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86")]
pub struct IMAGEHLP_DUPLICATE_SYMBOL {
    pub SizeOfStruct: u32,
    pub NumberOfDups: u32,
    pub Symbol: *mut IMAGEHLP_SYMBOL,
    pub SelectedSymbol: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for IMAGEHLP_DUPLICATE_SYMBOL {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for IMAGEHLP_DUPLICATE_SYMBOL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for IMAGEHLP_DUPLICATE_SYMBOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_DUPLICATE_SYMBOL").field("SizeOfStruct", &self.SizeOfStruct).field("NumberOfDups", &self.NumberOfDups).field("Symbol", &self.Symbol).field("SelectedSymbol", &self.SelectedSymbol).finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::windows::core::TypeKind for IMAGEHLP_DUPLICATE_SYMBOL {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for IMAGEHLP_DUPLICATE_SYMBOL {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.NumberOfDups == other.NumberOfDups && self.Symbol == other.Symbol && self.SelectedSymbol == other.SelectedSymbol
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for IMAGEHLP_DUPLICATE_SYMBOL {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for IMAGEHLP_DUPLICATE_SYMBOL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct IMAGEHLP_DUPLICATE_SYMBOL64 {
    pub SizeOfStruct: u32,
    pub NumberOfDups: u32,
    pub Symbol: *mut IMAGEHLP_SYMBOL64,
    pub SelectedSymbol: u32,
}
impl ::core::marker::Copy for IMAGEHLP_DUPLICATE_SYMBOL64 {}
impl ::core::clone::Clone for IMAGEHLP_DUPLICATE_SYMBOL64 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAGEHLP_DUPLICATE_SYMBOL64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_DUPLICATE_SYMBOL64").field("SizeOfStruct", &self.SizeOfStruct).field("NumberOfDups", &self.NumberOfDups).field("Symbol", &self.Symbol).field("SelectedSymbol", &self.SelectedSymbol).finish()
    }
}
impl ::windows::core::TypeKind for IMAGEHLP_DUPLICATE_SYMBOL64 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IMAGEHLP_DUPLICATE_SYMBOL64 {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.NumberOfDups == other.NumberOfDups && self.Symbol == other.Symbol && self.SelectedSymbol == other.SelectedSymbol
    }
}
impl ::core::cmp::Eq for IMAGEHLP_DUPLICATE_SYMBOL64 {}
impl ::core::default::Default for IMAGEHLP_DUPLICATE_SYMBOL64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct IMAGEHLP_GET_TYPE_INFO_PARAMS {
    pub SizeOfStruct: u32,
    pub Flags: IMAGEHLP_GET_TYPE_INFO_FLAGS,
    pub NumIds: u32,
    pub TypeIds: *mut u32,
    pub TagFilter: u64,
    pub NumReqs: u32,
    pub ReqKinds: *mut IMAGEHLP_SYMBOL_TYPE_INFO,
    pub ReqOffsets: *mut usize,
    pub ReqSizes: *mut u32,
    pub ReqStride: usize,
    pub BufferSize: usize,
    pub Buffer: *mut ::core::ffi::c_void,
    pub EntriesMatched: u32,
    pub EntriesFilled: u32,
    pub TagsFound: u64,
    pub AllReqsValid: u64,
    pub NumReqsValid: u32,
    pub ReqsValid: *mut u64,
}
impl ::core::marker::Copy for IMAGEHLP_GET_TYPE_INFO_PARAMS {}
impl ::core::clone::Clone for IMAGEHLP_GET_TYPE_INFO_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAGEHLP_GET_TYPE_INFO_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_GET_TYPE_INFO_PARAMS")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("Flags", &self.Flags)
            .field("NumIds", &self.NumIds)
            .field("TypeIds", &self.TypeIds)
            .field("TagFilter", &self.TagFilter)
            .field("NumReqs", &self.NumReqs)
            .field("ReqKinds", &self.ReqKinds)
            .field("ReqOffsets", &self.ReqOffsets)
            .field("ReqSizes", &self.ReqSizes)
            .field("ReqStride", &self.ReqStride)
            .field("BufferSize", &self.BufferSize)
            .field("Buffer", &self.Buffer)
            .field("EntriesMatched", &self.EntriesMatched)
            .field("EntriesFilled", &self.EntriesFilled)
            .field("TagsFound", &self.TagsFound)
            .field("AllReqsValid", &self.AllReqsValid)
            .field("NumReqsValid", &self.NumReqsValid)
            .field("ReqsValid", &self.ReqsValid)
            .finish()
    }
}
impl ::windows::core::TypeKind for IMAGEHLP_GET_TYPE_INFO_PARAMS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IMAGEHLP_GET_TYPE_INFO_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.Flags == other.Flags && self.NumIds == other.NumIds && self.TypeIds == other.TypeIds && self.TagFilter == other.TagFilter && self.NumReqs == other.NumReqs && self.ReqKinds == other.ReqKinds && self.ReqOffsets == other.ReqOffsets && self.ReqSizes == other.ReqSizes && self.ReqStride == other.ReqStride && self.BufferSize == other.BufferSize && self.Buffer == other.Buffer && self.EntriesMatched == other.EntriesMatched && self.EntriesFilled == other.EntriesFilled && self.TagsFound == other.TagsFound && self.AllReqsValid == other.AllReqsValid && self.NumReqsValid == other.NumReqsValid && self.ReqsValid == other.ReqsValid
    }
}
impl ::core::cmp::Eq for IMAGEHLP_GET_TYPE_INFO_PARAMS {}
impl ::core::default::Default for IMAGEHLP_GET_TYPE_INFO_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct IMAGEHLP_JIT_SYMBOLMAP {
    pub SizeOfStruct: u32,
    pub Address: u64,
    pub BaseOfImage: u64,
}
impl ::core::marker::Copy for IMAGEHLP_JIT_SYMBOLMAP {}
impl ::core::clone::Clone for IMAGEHLP_JIT_SYMBOLMAP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAGEHLP_JIT_SYMBOLMAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_JIT_SYMBOLMAP").field("SizeOfStruct", &self.SizeOfStruct).field("Address", &self.Address).field("BaseOfImage", &self.BaseOfImage).finish()
    }
}
impl ::windows::core::TypeKind for IMAGEHLP_JIT_SYMBOLMAP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IMAGEHLP_JIT_SYMBOLMAP {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.Address == other.Address && self.BaseOfImage == other.BaseOfImage
    }
}
impl ::core::cmp::Eq for IMAGEHLP_JIT_SYMBOLMAP {}
impl ::core::default::Default for IMAGEHLP_JIT_SYMBOLMAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86")]
pub struct IMAGEHLP_LINE {
    pub SizeOfStruct: u32,
    pub Key: *mut ::core::ffi::c_void,
    pub LineNumber: u32,
    pub FileName: ::windows::core::PSTR,
    pub Address: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for IMAGEHLP_LINE {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for IMAGEHLP_LINE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for IMAGEHLP_LINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_LINE").field("SizeOfStruct", &self.SizeOfStruct).field("Key", &self.Key).field("LineNumber", &self.LineNumber).field("FileName", &self.FileName).field("Address", &self.Address).finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::windows::core::TypeKind for IMAGEHLP_LINE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for IMAGEHLP_LINE {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.Key == other.Key && self.LineNumber == other.LineNumber && self.FileName == other.FileName && self.Address == other.Address
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for IMAGEHLP_LINE {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for IMAGEHLP_LINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct IMAGEHLP_LINE64 {
    pub SizeOfStruct: u32,
    pub Key: *mut ::core::ffi::c_void,
    pub LineNumber: u32,
    pub FileName: ::windows::core::PSTR,
    pub Address: u64,
}
impl ::core::marker::Copy for IMAGEHLP_LINE64 {}
impl ::core::clone::Clone for IMAGEHLP_LINE64 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAGEHLP_LINE64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_LINE64").field("SizeOfStruct", &self.SizeOfStruct).field("Key", &self.Key).field("LineNumber", &self.LineNumber).field("FileName", &self.FileName).field("Address", &self.Address).finish()
    }
}
impl ::windows::core::TypeKind for IMAGEHLP_LINE64 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IMAGEHLP_LINE64 {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.Key == other.Key && self.LineNumber == other.LineNumber && self.FileName == other.FileName && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for IMAGEHLP_LINE64 {}
impl ::core::default::Default for IMAGEHLP_LINE64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86")]
pub struct IMAGEHLP_LINEW {
    pub SizeOfStruct: u32,
    pub Key: *mut ::core::ffi::c_void,
    pub LineNumber: u32,
    pub FileName: ::windows::core::PSTR,
    pub Address: u64,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for IMAGEHLP_LINEW {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for IMAGEHLP_LINEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for IMAGEHLP_LINEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_LINEW").field("SizeOfStruct", &self.SizeOfStruct).field("Key", &self.Key).field("LineNumber", &self.LineNumber).field("FileName", &self.FileName).field("Address", &self.Address).finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::windows::core::TypeKind for IMAGEHLP_LINEW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for IMAGEHLP_LINEW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.Key == other.Key && self.LineNumber == other.LineNumber && self.FileName == other.FileName && self.Address == other.Address
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for IMAGEHLP_LINEW {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for IMAGEHLP_LINEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct IMAGEHLP_LINEW64 {
    pub SizeOfStruct: u32,
    pub Key: *mut ::core::ffi::c_void,
    pub LineNumber: u32,
    pub FileName: ::windows::core::PWSTR,
    pub Address: u64,
}
impl ::core::marker::Copy for IMAGEHLP_LINEW64 {}
impl ::core::clone::Clone for IMAGEHLP_LINEW64 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAGEHLP_LINEW64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_LINEW64").field("SizeOfStruct", &self.SizeOfStruct).field("Key", &self.Key).field("LineNumber", &self.LineNumber).field("FileName", &self.FileName).field("Address", &self.Address).finish()
    }
}
impl ::windows::core::TypeKind for IMAGEHLP_LINEW64 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IMAGEHLP_LINEW64 {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.Key == other.Key && self.LineNumber == other.LineNumber && self.FileName == other.FileName && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for IMAGEHLP_LINEW64 {}
impl ::core::default::Default for IMAGEHLP_LINEW64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86")]
pub struct IMAGEHLP_MODULE {
    pub SizeOfStruct: u32,
    pub BaseOfImage: u32,
    pub ImageSize: u32,
    pub TimeDateStamp: u32,
    pub CheckSum: u32,
    pub NumSyms: u32,
    pub SymType: SYM_TYPE,
    pub ModuleName: [u8; 32],
    pub ImageName: [u8; 256],
    pub LoadedImageName: [u8; 256],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for IMAGEHLP_MODULE {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for IMAGEHLP_MODULE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for IMAGEHLP_MODULE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_MODULE").field("SizeOfStruct", &self.SizeOfStruct).field("BaseOfImage", &self.BaseOfImage).field("ImageSize", &self.ImageSize).field("TimeDateStamp", &self.TimeDateStamp).field("CheckSum", &self.CheckSum).field("NumSyms", &self.NumSyms).field("SymType", &self.SymType).field("ModuleName", &self.ModuleName).field("ImageName", &self.ImageName).field("LoadedImageName", &self.LoadedImageName).finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::windows::core::TypeKind for IMAGEHLP_MODULE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for IMAGEHLP_MODULE {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.BaseOfImage == other.BaseOfImage && self.ImageSize == other.ImageSize && self.TimeDateStamp == other.TimeDateStamp && self.CheckSum == other.CheckSum && self.NumSyms == other.NumSyms && self.SymType == other.SymType && self.ModuleName == other.ModuleName && self.ImageName == other.ImageName && self.LoadedImageName == other.LoadedImageName
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for IMAGEHLP_MODULE {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for IMAGEHLP_MODULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMAGEHLP_MODULE64 {
    pub SizeOfStruct: u32,
    pub BaseOfImage: u64,
    pub ImageSize: u32,
    pub TimeDateStamp: u32,
    pub CheckSum: u32,
    pub NumSyms: u32,
    pub SymType: SYM_TYPE,
    pub ModuleName: [u8; 32],
    pub ImageName: [u8; 256],
    pub LoadedImageName: [u8; 256],
    pub LoadedPdbName: [u8; 256],
    pub CVSig: u32,
    pub CVData: [u8; 780],
    pub PdbSig: u32,
    pub PdbSig70: ::windows::core::GUID,
    pub PdbAge: u32,
    pub PdbUnmatched: super::super::super::Foundation::BOOL,
    pub DbgUnmatched: super::super::super::Foundation::BOOL,
    pub LineNumbers: super::super::super::Foundation::BOOL,
    pub GlobalSymbols: super::super::super::Foundation::BOOL,
    pub TypeInfo: super::super::super::Foundation::BOOL,
    pub SourceIndexed: super::super::super::Foundation::BOOL,
    pub Publics: super::super::super::Foundation::BOOL,
    pub MachineType: u32,
    pub Reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMAGEHLP_MODULE64 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMAGEHLP_MODULE64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMAGEHLP_MODULE64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_MODULE64")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("BaseOfImage", &self.BaseOfImage)
            .field("ImageSize", &self.ImageSize)
            .field("TimeDateStamp", &self.TimeDateStamp)
            .field("CheckSum", &self.CheckSum)
            .field("NumSyms", &self.NumSyms)
            .field("SymType", &self.SymType)
            .field("ModuleName", &self.ModuleName)
            .field("ImageName", &self.ImageName)
            .field("LoadedImageName", &self.LoadedImageName)
            .field("LoadedPdbName", &self.LoadedPdbName)
            .field("CVSig", &self.CVSig)
            .field("CVData", &self.CVData)
            .field("PdbSig", &self.PdbSig)
            .field("PdbSig70", &self.PdbSig70)
            .field("PdbAge", &self.PdbAge)
            .field("PdbUnmatched", &self.PdbUnmatched)
            .field("DbgUnmatched", &self.DbgUnmatched)
            .field("LineNumbers", &self.LineNumbers)
            .field("GlobalSymbols", &self.GlobalSymbols)
            .field("TypeInfo", &self.TypeInfo)
            .field("SourceIndexed", &self.SourceIndexed)
            .field("Publics", &self.Publics)
            .field("MachineType", &self.MachineType)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for IMAGEHLP_MODULE64 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMAGEHLP_MODULE64 {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct
            && self.BaseOfImage == other.BaseOfImage
            && self.ImageSize == other.ImageSize
            && self.TimeDateStamp == other.TimeDateStamp
            && self.CheckSum == other.CheckSum
            && self.NumSyms == other.NumSyms
            && self.SymType == other.SymType
            && self.ModuleName == other.ModuleName
            && self.ImageName == other.ImageName
            && self.LoadedImageName == other.LoadedImageName
            && self.LoadedPdbName == other.LoadedPdbName
            && self.CVSig == other.CVSig
            && self.CVData == other.CVData
            && self.PdbSig == other.PdbSig
            && self.PdbSig70 == other.PdbSig70
            && self.PdbAge == other.PdbAge
            && self.PdbUnmatched == other.PdbUnmatched
            && self.DbgUnmatched == other.DbgUnmatched
            && self.LineNumbers == other.LineNumbers
            && self.GlobalSymbols == other.GlobalSymbols
            && self.TypeInfo == other.TypeInfo
            && self.SourceIndexed == other.SourceIndexed
            && self.Publics == other.Publics
            && self.MachineType == other.MachineType
            && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMAGEHLP_MODULE64 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMAGEHLP_MODULE64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMAGEHLP_MODULE64_EX {
    pub Module: IMAGEHLP_MODULE64,
    pub RegionFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMAGEHLP_MODULE64_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMAGEHLP_MODULE64_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMAGEHLP_MODULE64_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_MODULE64_EX").field("Module", &self.Module).field("RegionFlags", &self.RegionFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for IMAGEHLP_MODULE64_EX {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMAGEHLP_MODULE64_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Module == other.Module && self.RegionFlags == other.RegionFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMAGEHLP_MODULE64_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMAGEHLP_MODULE64_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86")]
pub struct IMAGEHLP_MODULEW {
    pub SizeOfStruct: u32,
    pub BaseOfImage: u32,
    pub ImageSize: u32,
    pub TimeDateStamp: u32,
    pub CheckSum: u32,
    pub NumSyms: u32,
    pub SymType: SYM_TYPE,
    pub ModuleName: [u16; 32],
    pub ImageName: [u16; 256],
    pub LoadedImageName: [u16; 256],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for IMAGEHLP_MODULEW {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for IMAGEHLP_MODULEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for IMAGEHLP_MODULEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_MODULEW").field("SizeOfStruct", &self.SizeOfStruct).field("BaseOfImage", &self.BaseOfImage).field("ImageSize", &self.ImageSize).field("TimeDateStamp", &self.TimeDateStamp).field("CheckSum", &self.CheckSum).field("NumSyms", &self.NumSyms).field("SymType", &self.SymType).field("ModuleName", &self.ModuleName).field("ImageName", &self.ImageName).field("LoadedImageName", &self.LoadedImageName).finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::windows::core::TypeKind for IMAGEHLP_MODULEW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for IMAGEHLP_MODULEW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.BaseOfImage == other.BaseOfImage && self.ImageSize == other.ImageSize && self.TimeDateStamp == other.TimeDateStamp && self.CheckSum == other.CheckSum && self.NumSyms == other.NumSyms && self.SymType == other.SymType && self.ModuleName == other.ModuleName && self.ImageName == other.ImageName && self.LoadedImageName == other.LoadedImageName
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for IMAGEHLP_MODULEW {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for IMAGEHLP_MODULEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMAGEHLP_MODULEW64 {
    pub SizeOfStruct: u32,
    pub BaseOfImage: u64,
    pub ImageSize: u32,
    pub TimeDateStamp: u32,
    pub CheckSum: u32,
    pub NumSyms: u32,
    pub SymType: SYM_TYPE,
    pub ModuleName: [u16; 32],
    pub ImageName: [u16; 256],
    pub LoadedImageName: [u16; 256],
    pub LoadedPdbName: [u16; 256],
    pub CVSig: u32,
    pub CVData: [u16; 780],
    pub PdbSig: u32,
    pub PdbSig70: ::windows::core::GUID,
    pub PdbAge: u32,
    pub PdbUnmatched: super::super::super::Foundation::BOOL,
    pub DbgUnmatched: super::super::super::Foundation::BOOL,
    pub LineNumbers: super::super::super::Foundation::BOOL,
    pub GlobalSymbols: super::super::super::Foundation::BOOL,
    pub TypeInfo: super::super::super::Foundation::BOOL,
    pub SourceIndexed: super::super::super::Foundation::BOOL,
    pub Publics: super::super::super::Foundation::BOOL,
    pub MachineType: u32,
    pub Reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMAGEHLP_MODULEW64 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMAGEHLP_MODULEW64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMAGEHLP_MODULEW64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_MODULEW64")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("BaseOfImage", &self.BaseOfImage)
            .field("ImageSize", &self.ImageSize)
            .field("TimeDateStamp", &self.TimeDateStamp)
            .field("CheckSum", &self.CheckSum)
            .field("NumSyms", &self.NumSyms)
            .field("SymType", &self.SymType)
            .field("ModuleName", &self.ModuleName)
            .field("ImageName", &self.ImageName)
            .field("LoadedImageName", &self.LoadedImageName)
            .field("LoadedPdbName", &self.LoadedPdbName)
            .field("CVSig", &self.CVSig)
            .field("CVData", &self.CVData)
            .field("PdbSig", &self.PdbSig)
            .field("PdbSig70", &self.PdbSig70)
            .field("PdbAge", &self.PdbAge)
            .field("PdbUnmatched", &self.PdbUnmatched)
            .field("DbgUnmatched", &self.DbgUnmatched)
            .field("LineNumbers", &self.LineNumbers)
            .field("GlobalSymbols", &self.GlobalSymbols)
            .field("TypeInfo", &self.TypeInfo)
            .field("SourceIndexed", &self.SourceIndexed)
            .field("Publics", &self.Publics)
            .field("MachineType", &self.MachineType)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for IMAGEHLP_MODULEW64 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMAGEHLP_MODULEW64 {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct
            && self.BaseOfImage == other.BaseOfImage
            && self.ImageSize == other.ImageSize
            && self.TimeDateStamp == other.TimeDateStamp
            && self.CheckSum == other.CheckSum
            && self.NumSyms == other.NumSyms
            && self.SymType == other.SymType
            && self.ModuleName == other.ModuleName
            && self.ImageName == other.ImageName
            && self.LoadedImageName == other.LoadedImageName
            && self.LoadedPdbName == other.LoadedPdbName
            && self.CVSig == other.CVSig
            && self.CVData == other.CVData
            && self.PdbSig == other.PdbSig
            && self.PdbSig70 == other.PdbSig70
            && self.PdbAge == other.PdbAge
            && self.PdbUnmatched == other.PdbUnmatched
            && self.DbgUnmatched == other.DbgUnmatched
            && self.LineNumbers == other.LineNumbers
            && self.GlobalSymbols == other.GlobalSymbols
            && self.TypeInfo == other.TypeInfo
            && self.SourceIndexed == other.SourceIndexed
            && self.Publics == other.Publics
            && self.MachineType == other.MachineType
            && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMAGEHLP_MODULEW64 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMAGEHLP_MODULEW64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMAGEHLP_MODULEW64_EX {
    pub Module: IMAGEHLP_MODULEW64,
    pub RegionFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMAGEHLP_MODULEW64_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMAGEHLP_MODULEW64_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMAGEHLP_MODULEW64_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_MODULEW64_EX").field("Module", &self.Module).field("RegionFlags", &self.RegionFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for IMAGEHLP_MODULEW64_EX {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMAGEHLP_MODULEW64_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Module == other.Module && self.RegionFlags == other.RegionFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMAGEHLP_MODULEW64_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMAGEHLP_MODULEW64_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMAGEHLP_STACK_FRAME {
    pub InstructionOffset: u64,
    pub ReturnOffset: u64,
    pub FrameOffset: u64,
    pub StackOffset: u64,
    pub BackingStoreOffset: u64,
    pub FuncTableEntry: u64,
    pub Params: [u64; 4],
    pub Reserved: [u64; 5],
    pub Virtual: super::super::super::Foundation::BOOL,
    pub Reserved2: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMAGEHLP_STACK_FRAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMAGEHLP_STACK_FRAME {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMAGEHLP_STACK_FRAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_STACK_FRAME").field("InstructionOffset", &self.InstructionOffset).field("ReturnOffset", &self.ReturnOffset).field("FrameOffset", &self.FrameOffset).field("StackOffset", &self.StackOffset).field("BackingStoreOffset", &self.BackingStoreOffset).field("FuncTableEntry", &self.FuncTableEntry).field("Params", &self.Params).field("Reserved", &self.Reserved).field("Virtual", &self.Virtual).field("Reserved2", &self.Reserved2).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for IMAGEHLP_STACK_FRAME {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMAGEHLP_STACK_FRAME {
    fn eq(&self, other: &Self) -> bool {
        self.InstructionOffset == other.InstructionOffset && self.ReturnOffset == other.ReturnOffset && self.FrameOffset == other.FrameOffset && self.StackOffset == other.StackOffset && self.BackingStoreOffset == other.BackingStoreOffset && self.FuncTableEntry == other.FuncTableEntry && self.Params == other.Params && self.Reserved == other.Reserved && self.Virtual == other.Virtual && self.Reserved2 == other.Reserved2
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMAGEHLP_STACK_FRAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMAGEHLP_STACK_FRAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86")]
pub struct IMAGEHLP_SYMBOL {
    pub SizeOfStruct: u32,
    pub Address: u32,
    pub Size: u32,
    pub Flags: u32,
    pub MaxNameLength: u32,
    pub Name: [u8; 1],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for IMAGEHLP_SYMBOL {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for IMAGEHLP_SYMBOL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for IMAGEHLP_SYMBOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_SYMBOL").field("SizeOfStruct", &self.SizeOfStruct).field("Address", &self.Address).field("Size", &self.Size).field("Flags", &self.Flags).field("MaxNameLength", &self.MaxNameLength).field("Name", &self.Name).finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::windows::core::TypeKind for IMAGEHLP_SYMBOL {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for IMAGEHLP_SYMBOL {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.Address == other.Address && self.Size == other.Size && self.Flags == other.Flags && self.MaxNameLength == other.MaxNameLength && self.Name == other.Name
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for IMAGEHLP_SYMBOL {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for IMAGEHLP_SYMBOL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct IMAGEHLP_SYMBOL64 {
    pub SizeOfStruct: u32,
    pub Address: u64,
    pub Size: u32,
    pub Flags: u32,
    pub MaxNameLength: u32,
    pub Name: [u8; 1],
}
impl ::core::marker::Copy for IMAGEHLP_SYMBOL64 {}
impl ::core::clone::Clone for IMAGEHLP_SYMBOL64 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAGEHLP_SYMBOL64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_SYMBOL64").field("SizeOfStruct", &self.SizeOfStruct).field("Address", &self.Address).field("Size", &self.Size).field("Flags", &self.Flags).field("MaxNameLength", &self.MaxNameLength).field("Name", &self.Name).finish()
    }
}
impl ::windows::core::TypeKind for IMAGEHLP_SYMBOL64 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IMAGEHLP_SYMBOL64 {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.Address == other.Address && self.Size == other.Size && self.Flags == other.Flags && self.MaxNameLength == other.MaxNameLength && self.Name == other.Name
    }
}
impl ::core::cmp::Eq for IMAGEHLP_SYMBOL64 {}
impl ::core::default::Default for IMAGEHLP_SYMBOL64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct IMAGEHLP_SYMBOL64_PACKAGE {
    pub sym: IMAGEHLP_SYMBOL64,
    pub name: [u8; 2001],
}
impl ::core::marker::Copy for IMAGEHLP_SYMBOL64_PACKAGE {}
impl ::core::clone::Clone for IMAGEHLP_SYMBOL64_PACKAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAGEHLP_SYMBOL64_PACKAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_SYMBOL64_PACKAGE").field("sym", &self.sym).field("name", &self.name).finish()
    }
}
impl ::windows::core::TypeKind for IMAGEHLP_SYMBOL64_PACKAGE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IMAGEHLP_SYMBOL64_PACKAGE {
    fn eq(&self, other: &Self) -> bool {
        self.sym == other.sym && self.name == other.name
    }
}
impl ::core::cmp::Eq for IMAGEHLP_SYMBOL64_PACKAGE {}
impl ::core::default::Default for IMAGEHLP_SYMBOL64_PACKAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86")]
pub struct IMAGEHLP_SYMBOLW {
    pub SizeOfStruct: u32,
    pub Address: u32,
    pub Size: u32,
    pub Flags: u32,
    pub MaxNameLength: u32,
    pub Name: [u16; 1],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for IMAGEHLP_SYMBOLW {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for IMAGEHLP_SYMBOLW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for IMAGEHLP_SYMBOLW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_SYMBOLW").field("SizeOfStruct", &self.SizeOfStruct).field("Address", &self.Address).field("Size", &self.Size).field("Flags", &self.Flags).field("MaxNameLength", &self.MaxNameLength).field("Name", &self.Name).finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::windows::core::TypeKind for IMAGEHLP_SYMBOLW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for IMAGEHLP_SYMBOLW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.Address == other.Address && self.Size == other.Size && self.Flags == other.Flags && self.MaxNameLength == other.MaxNameLength && self.Name == other.Name
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for IMAGEHLP_SYMBOLW {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for IMAGEHLP_SYMBOLW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct IMAGEHLP_SYMBOLW64 {
    pub SizeOfStruct: u32,
    pub Address: u64,
    pub Size: u32,
    pub Flags: u32,
    pub MaxNameLength: u32,
    pub Name: [u16; 1],
}
impl ::core::marker::Copy for IMAGEHLP_SYMBOLW64 {}
impl ::core::clone::Clone for IMAGEHLP_SYMBOLW64 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAGEHLP_SYMBOLW64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_SYMBOLW64").field("SizeOfStruct", &self.SizeOfStruct).field("Address", &self.Address).field("Size", &self.Size).field("Flags", &self.Flags).field("MaxNameLength", &self.MaxNameLength).field("Name", &self.Name).finish()
    }
}
impl ::windows::core::TypeKind for IMAGEHLP_SYMBOLW64 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IMAGEHLP_SYMBOLW64 {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.Address == other.Address && self.Size == other.Size && self.Flags == other.Flags && self.MaxNameLength == other.MaxNameLength && self.Name == other.Name
    }
}
impl ::core::cmp::Eq for IMAGEHLP_SYMBOLW64 {}
impl ::core::default::Default for IMAGEHLP_SYMBOLW64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct IMAGEHLP_SYMBOLW64_PACKAGE {
    pub sym: IMAGEHLP_SYMBOLW64,
    pub name: [u16; 2001],
}
impl ::core::marker::Copy for IMAGEHLP_SYMBOLW64_PACKAGE {}
impl ::core::clone::Clone for IMAGEHLP_SYMBOLW64_PACKAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAGEHLP_SYMBOLW64_PACKAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_SYMBOLW64_PACKAGE").field("sym", &self.sym).field("name", &self.name).finish()
    }
}
impl ::windows::core::TypeKind for IMAGEHLP_SYMBOLW64_PACKAGE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IMAGEHLP_SYMBOLW64_PACKAGE {
    fn eq(&self, other: &Self) -> bool {
        self.sym == other.sym && self.name == other.name
    }
}
impl ::core::cmp::Eq for IMAGEHLP_SYMBOLW64_PACKAGE {}
impl ::core::default::Default for IMAGEHLP_SYMBOLW64_PACKAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86")]
pub struct IMAGEHLP_SYMBOLW_PACKAGE {
    pub sym: IMAGEHLP_SYMBOLW,
    pub name: [u16; 2001],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for IMAGEHLP_SYMBOLW_PACKAGE {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for IMAGEHLP_SYMBOLW_PACKAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for IMAGEHLP_SYMBOLW_PACKAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_SYMBOLW_PACKAGE").field("sym", &self.sym).field("name", &self.name).finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::windows::core::TypeKind for IMAGEHLP_SYMBOLW_PACKAGE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for IMAGEHLP_SYMBOLW_PACKAGE {
    fn eq(&self, other: &Self) -> bool {
        self.sym == other.sym && self.name == other.name
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for IMAGEHLP_SYMBOLW_PACKAGE {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for IMAGEHLP_SYMBOLW_PACKAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86")]
pub struct IMAGEHLP_SYMBOL_PACKAGE {
    pub sym: IMAGEHLP_SYMBOL,
    pub name: [u8; 2001],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for IMAGEHLP_SYMBOL_PACKAGE {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for IMAGEHLP_SYMBOL_PACKAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for IMAGEHLP_SYMBOL_PACKAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_SYMBOL_PACKAGE").field("sym", &self.sym).field("name", &self.name).finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::windows::core::TypeKind for IMAGEHLP_SYMBOL_PACKAGE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for IMAGEHLP_SYMBOL_PACKAGE {
    fn eq(&self, other: &Self) -> bool {
        self.sym == other.sym && self.name == other.name
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for IMAGEHLP_SYMBOL_PACKAGE {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for IMAGEHLP_SYMBOL_PACKAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct IMAGEHLP_SYMBOL_SRC {
    pub sizeofstruct: u32,
    pub r#type: u32,
    pub file: [u8; 260],
}
impl ::core::marker::Copy for IMAGEHLP_SYMBOL_SRC {}
impl ::core::clone::Clone for IMAGEHLP_SYMBOL_SRC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAGEHLP_SYMBOL_SRC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEHLP_SYMBOL_SRC").field("sizeofstruct", &self.sizeofstruct).field("type", &self.r#type).field("file", &self.file).finish()
    }
}
impl ::windows::core::TypeKind for IMAGEHLP_SYMBOL_SRC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IMAGEHLP_SYMBOL_SRC {
    fn eq(&self, other: &Self) -> bool {
        self.sizeofstruct == other.sizeofstruct && self.r#type == other.r#type && self.file == other.file
    }
}
impl ::core::cmp::Eq for IMAGEHLP_SYMBOL_SRC {}
impl ::core::default::Default for IMAGEHLP_SYMBOL_SRC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY {
    pub BeginAddress: u32,
    pub Anonymous: IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_0,
}
impl ::core::marker::Copy for IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY {}
impl ::core::clone::Clone for IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub union IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_0 {
    pub UnwindData: u32,
    pub Anonymous: IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_0_0,
}
impl ::core::marker::Copy for IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_0 {}
impl ::core::clone::Clone for IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_0_0 {}
impl ::core::clone::Clone for IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows::core::TypeKind for IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_0_0 {}
impl ::core::default::Default for IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct IMAGE_COFF_SYMBOLS_HEADER {
    pub NumberOfSymbols: u32,
    pub LvaToFirstSymbol: u32,
    pub NumberOfLinenumbers: u32,
    pub LvaToFirstLinenumber: u32,
    pub RvaToFirstByteOfCode: u32,
    pub RvaToLastByteOfCode: u32,
    pub RvaToFirstByteOfData: u32,
    pub RvaToLastByteOfData: u32,
}
impl ::core::marker::Copy for IMAGE_COFF_SYMBOLS_HEADER {}
impl ::core::clone::Clone for IMAGE_COFF_SYMBOLS_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAGE_COFF_SYMBOLS_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_COFF_SYMBOLS_HEADER")
            .field("NumberOfSymbols", &self.NumberOfSymbols)
            .field("LvaToFirstSymbol", &self.LvaToFirstSymbol)
            .field("NumberOfLinenumbers", &self.NumberOfLinenumbers)
            .field("LvaToFirstLinenumber", &self.LvaToFirstLinenumber)
            .field("RvaToFirstByteOfCode", &self.RvaToFirstByteOfCode)
            .field("RvaToLastByteOfCode", &self.RvaToLastByteOfCode)
            .field("RvaToFirstByteOfData", &self.RvaToFirstByteOfData)
            .field("RvaToLastByteOfData", &self.RvaToLastByteOfData)
            .finish()
    }
}
impl ::windows::core::TypeKind for IMAGE_COFF_SYMBOLS_HEADER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IMAGE_COFF_SYMBOLS_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfSymbols == other.NumberOfSymbols && self.LvaToFirstSymbol == other.LvaToFirstSymbol && self.NumberOfLinenumbers == other.NumberOfLinenumbers && self.LvaToFirstLinenumber == other.LvaToFirstLinenumber && self.RvaToFirstByteOfCode == other.RvaToFirstByteOfCode && self.RvaToLastByteOfCode == other.RvaToLastByteOfCode && self.RvaToFirstByteOfData == other.RvaToFirstByteOfData && self.RvaToLastByteOfData == other.RvaToLastByteOfData
    }
}
impl ::core::cmp::Eq for IMAGE_COFF_SYMBOLS_HEADER {}
impl ::core::default::Default for IMAGE_COFF_SYMBOLS_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct IMAGE_COR20_HEADER {
    pub cb: u32,
    pub MajorRuntimeVersion: u16,
    pub MinorRuntimeVersion: u16,
    pub MetaData: IMAGE_DATA_DIRECTORY,
    pub Flags: u32,
    pub Anonymous: IMAGE_COR20_HEADER_0,
    pub Resources: IMAGE_DATA_DIRECTORY,
    pub StrongNameSignature: IMAGE_DATA_DIRECTORY,
    pub CodeManagerTable: IMAGE_DATA_DIRECTORY,
    pub VTableFixups: IMAGE_DATA_DIRECTORY,
    pub ExportAddressTableJumps: IMAGE_DATA_DIRECTORY,
    pub ManagedNativeHeader: IMAGE_DATA_DIRECTORY,
}
impl ::core::marker::Copy for IMAGE_COR20_HEADER {}
impl ::core::clone::Clone for IMAGE_COR20_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for IMAGE_COR20_HEADER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for IMAGE_COR20_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub union IMAGE_COR20_HEADER_0 {
    pub EntryPointToken: u32,
    pub EntryPointRVA: u32,
}
impl ::core::marker::Copy for IMAGE_COR20_HEADER_0 {}
impl ::core::clone::Clone for IMAGE_COR20_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for IMAGE_COR20_HEADER_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for IMAGE_COR20_HEADER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct IMAGE_DATA_DIRECTORY {
    pub VirtualAddress: u32,
    pub Size: u32,
}
impl ::core::marker::Copy for IMAGE_DATA_DIRECTORY {}
impl ::core::clone::Clone for IMAGE_DATA_DIRECTORY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAGE_DATA_DIRECTORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_DATA_DIRECTORY").field("VirtualAddress", &self.VirtualAddress).field("Size", &self.Size).finish()
    }
}
impl ::windows::core::TypeKind for IMAGE_DATA_DIRECTORY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IMAGE_DATA_DIRECTORY {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualAddress == other.VirtualAddress && self.Size == other.Size
    }
}
impl ::core::cmp::Eq for IMAGE_DATA_DIRECTORY {}
impl ::core::default::Default for IMAGE_DATA_DIRECTORY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct IMAGE_DEBUG_DIRECTORY {
    pub Characteristics: u32,
    pub TimeDateStamp: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub Type: IMAGE_DEBUG_TYPE,
    pub SizeOfData: u32,
    pub AddressOfRawData: u32,
    pub PointerToRawData: u32,
}
impl ::core::marker::Copy for IMAGE_DEBUG_DIRECTORY {}
impl ::core::clone::Clone for IMAGE_DEBUG_DIRECTORY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAGE_DEBUG_DIRECTORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_DEBUG_DIRECTORY").field("Characteristics", &self.Characteristics).field("TimeDateStamp", &self.TimeDateStamp).field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).field("Type", &self.Type).field("SizeOfData", &self.SizeOfData).field("AddressOfRawData", &self.AddressOfRawData).field("PointerToRawData", &self.PointerToRawData).finish()
    }
}
impl ::windows::core::TypeKind for IMAGE_DEBUG_DIRECTORY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IMAGE_DEBUG_DIRECTORY {
    fn eq(&self, other: &Self) -> bool {
        self.Characteristics == other.Characteristics && self.TimeDateStamp == other.TimeDateStamp && self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion && self.Type == other.Type && self.SizeOfData == other.SizeOfData && self.AddressOfRawData == other.AddressOfRawData && self.PointerToRawData == other.PointerToRawData
    }
}
impl ::core::cmp::Eq for IMAGE_DEBUG_DIRECTORY {}
impl ::core::default::Default for IMAGE_DEBUG_DIRECTORY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct IMAGE_DEBUG_INFORMATION {
    pub List: super::super::Kernel::LIST_ENTRY,
    pub ReservedSize: u32,
    pub ReservedMappedBase: *mut ::core::ffi::c_void,
    pub ReservedMachine: u16,
    pub ReservedCharacteristics: u16,
    pub ReservedCheckSum: u32,
    pub ImageBase: u32,
    pub SizeOfImage: u32,
    pub ReservedNumberOfSections: u32,
    pub ReservedSections: *mut IMAGE_SECTION_HEADER,
    pub ReservedExportedNamesSize: u32,
    pub ReservedExportedNames: ::windows::core::PSTR,
    pub ReservedNumberOfFunctionTableEntries: u32,
    pub ReservedFunctionTableEntries: *mut IMAGE_FUNCTION_ENTRY,
    pub ReservedLowestFunctionStartingAddress: u32,
    pub ReservedHighestFunctionEndingAddress: u32,
    pub ReservedNumberOfFpoTableEntries: u32,
    pub ReservedFpoTableEntries: *mut FPO_DATA,
    pub SizeOfCoffSymbols: u32,
    pub CoffSymbols: *mut IMAGE_COFF_SYMBOLS_HEADER,
    pub ReservedSizeOfCodeViewSymbols: u32,
    pub ReservedCodeViewSymbols: *mut ::core::ffi::c_void,
    pub ImageFilePath: ::windows::core::PSTR,
    pub ImageFileName: ::windows::core::PSTR,
    pub ReservedDebugFilePath: ::windows::core::PSTR,
    pub ReservedTimeDateStamp: u32,
    pub ReservedRomImage: super::super::super::Foundation::BOOL,
    pub ReservedDebugDirectory: *mut IMAGE_DEBUG_DIRECTORY,
    pub ReservedNumberOfDebugDirectories: u32,
    pub ReservedOriginalFunctionTableBaseAddress: u32,
    pub Reserved: [u32; 2],
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for IMAGE_DEBUG_INFORMATION {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for IMAGE_DEBUG_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::fmt::Debug for IMAGE_DEBUG_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_DEBUG_INFORMATION")
            .field("List", &self.List)
            .field("ReservedSize", &self.ReservedSize)
            .field("ReservedMappedBase", &self.ReservedMappedBase)
            .field("ReservedMachine", &self.ReservedMachine)
            .field("ReservedCharacteristics", &self.ReservedCharacteristics)
            .field("ReservedCheckSum", &self.ReservedCheckSum)
            .field("ImageBase", &self.ImageBase)
            .field("SizeOfImage", &self.SizeOfImage)
            .field("ReservedNumberOfSections", &self.ReservedNumberOfSections)
            .field("ReservedSections", &self.ReservedSections)
            .field("ReservedExportedNamesSize", &self.ReservedExportedNamesSize)
            .field("ReservedExportedNames", &self.ReservedExportedNames)
            .field("ReservedNumberOfFunctionTableEntries", &self.ReservedNumberOfFunctionTableEntries)
            .field("ReservedFunctionTableEntries", &self.ReservedFunctionTableEntries)
            .field("ReservedLowestFunctionStartingAddress", &self.ReservedLowestFunctionStartingAddress)
            .field("ReservedHighestFunctionEndingAddress", &self.ReservedHighestFunctionEndingAddress)
            .field("ReservedNumberOfFpoTableEntries", &self.ReservedNumberOfFpoTableEntries)
            .field("ReservedFpoTableEntries", &self.ReservedFpoTableEntries)
            .field("SizeOfCoffSymbols", &self.SizeOfCoffSymbols)
            .field("CoffSymbols", &self.CoffSymbols)
            .field("ReservedSizeOfCodeViewSymbols", &self.ReservedSizeOfCodeViewSymbols)
            .field("ReservedCodeViewSymbols", &self.ReservedCodeViewSymbols)
            .field("ImageFilePath", &self.ImageFilePath)
            .field("ImageFileName", &self.ImageFileName)
            .field("ReservedDebugFilePath", &self.ReservedDebugFilePath)
            .field("ReservedTimeDateStamp", &self.ReservedTimeDateStamp)
            .field("ReservedRomImage", &self.ReservedRomImage)
            .field("ReservedDebugDirectory", &self.ReservedDebugDirectory)
            .field("ReservedNumberOfDebugDirectories", &self.ReservedNumberOfDebugDirectories)
            .field("ReservedOriginalFunctionTableBaseAddress", &self.ReservedOriginalFunctionTableBaseAddress)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::windows::core::TypeKind for IMAGE_DEBUG_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::PartialEq for IMAGE_DEBUG_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.List == other.List
            && self.ReservedSize == other.ReservedSize
            && self.ReservedMappedBase == other.ReservedMappedBase
            && self.ReservedMachine == other.ReservedMachine
            && self.ReservedCharacteristics == other.ReservedCharacteristics
            && self.ReservedCheckSum == other.ReservedCheckSum
            && self.ImageBase == other.ImageBase
            && self.SizeOfImage == other.SizeOfImage
            && self.ReservedNumberOfSections == other.ReservedNumberOfSections
            && self.ReservedSections == other.ReservedSections
            && self.ReservedExportedNamesSize == other.ReservedExportedNamesSize
            && self.ReservedExportedNames == other.ReservedExportedNames
            && self.ReservedNumberOfFunctionTableEntries == other.ReservedNumberOfFunctionTableEntries
            && self.ReservedFunctionTableEntries == other.ReservedFunctionTableEntries
            && self.ReservedLowestFunctionStartingAddress == other.ReservedLowestFunctionStartingAddress
            && self.ReservedHighestFunctionEndingAddress == other.ReservedHighestFunctionEndingAddress
            && self.ReservedNumberOfFpoTableEntries == other.ReservedNumberOfFpoTableEntries
            && self.ReservedFpoTableEntries == other.ReservedFpoTableEntries
            && self.SizeOfCoffSymbols == other.SizeOfCoffSymbols
            && self.CoffSymbols == other.CoffSymbols
            && self.ReservedSizeOfCodeViewSymbols == other.ReservedSizeOfCodeViewSymbols
            && self.ReservedCodeViewSymbols == other.ReservedCodeViewSymbols
            && self.ImageFilePath == other.ImageFilePath
            && self.ImageFileName == other.ImageFileName
            && self.ReservedDebugFilePath == other.ReservedDebugFilePath
            && self.ReservedTimeDateStamp == other.ReservedTimeDateStamp
            && self.ReservedRomImage == other.ReservedRomImage
            && self.ReservedDebugDirectory == other.ReservedDebugDirectory
            && self.ReservedNumberOfDebugDirectories == other.ReservedNumberOfDebugDirectories
            && self.ReservedOriginalFunctionTableBaseAddress == other.ReservedOriginalFunctionTableBaseAddress
            && self.Reserved == other.Reserved
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::Eq for IMAGE_DEBUG_INFORMATION {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for IMAGE_DEBUG_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(feature = "Win32_System_SystemInformation")]
pub struct IMAGE_FILE_HEADER {
    pub Machine: super::super::SystemInformation::IMAGE_FILE_MACHINE,
    pub NumberOfSections: u16,
    pub TimeDateStamp: u32,
    pub PointerToSymbolTable: u32,
    pub NumberOfSymbols: u32,
    pub SizeOfOptionalHeader: u16,
    pub Characteristics: IMAGE_FILE_CHARACTERISTICS,
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::marker::Copy for IMAGE_FILE_HEADER {}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::clone::Clone for IMAGE_FILE_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::fmt::Debug for IMAGE_FILE_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_FILE_HEADER").field("Machine", &self.Machine).field("NumberOfSections", &self.NumberOfSections).field("TimeDateStamp", &self.TimeDateStamp).field("PointerToSymbolTable", &self.PointerToSymbolTable).field("NumberOfSymbols", &self.NumberOfSymbols).field("SizeOfOptionalHeader", &self.SizeOfOptionalHeader).field("Characteristics", &self.Characteristics).finish()
    }
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::windows::core::TypeKind for IMAGE_FILE_HEADER {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::cmp::PartialEq for IMAGE_FILE_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Machine == other.Machine && self.NumberOfSections == other.NumberOfSections && self.TimeDateStamp == other.TimeDateStamp && self.PointerToSymbolTable == other.PointerToSymbolTable && self.NumberOfSymbols == other.NumberOfSymbols && self.SizeOfOptionalHeader == other.SizeOfOptionalHeader && self.Characteristics == other.Characteristics
    }
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::cmp::Eq for IMAGE_FILE_HEADER {}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::default::Default for IMAGE_FILE_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct IMAGE_FUNCTION_ENTRY {
    pub StartingAddress: u32,
    pub EndingAddress: u32,
    pub EndOfPrologue: u32,
}
impl ::core::marker::Copy for IMAGE_FUNCTION_ENTRY {}
impl ::core::clone::Clone for IMAGE_FUNCTION_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAGE_FUNCTION_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_FUNCTION_ENTRY").field("StartingAddress", &self.StartingAddress).field("EndingAddress", &self.EndingAddress).field("EndOfPrologue", &self.EndOfPrologue).finish()
    }
}
impl ::windows::core::TypeKind for IMAGE_FUNCTION_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IMAGE_FUNCTION_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.StartingAddress == other.StartingAddress && self.EndingAddress == other.EndingAddress && self.EndOfPrologue == other.EndOfPrologue
    }
}
impl ::core::cmp::Eq for IMAGE_FUNCTION_ENTRY {}
impl ::core::default::Default for IMAGE_FUNCTION_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct IMAGE_FUNCTION_ENTRY64 {
    pub StartingAddress: u64,
    pub EndingAddress: u64,
    pub Anonymous: IMAGE_FUNCTION_ENTRY64_0,
}
impl ::core::marker::Copy for IMAGE_FUNCTION_ENTRY64 {}
impl ::core::clone::Clone for IMAGE_FUNCTION_ENTRY64 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for IMAGE_FUNCTION_ENTRY64 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for IMAGE_FUNCTION_ENTRY64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub union IMAGE_FUNCTION_ENTRY64_0 {
    pub EndOfPrologue: u64,
    pub UnwindInfoAddress: u64,
}
impl ::core::marker::Copy for IMAGE_FUNCTION_ENTRY64_0 {}
impl ::core::clone::Clone for IMAGE_FUNCTION_ENTRY64_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for IMAGE_FUNCTION_ENTRY64_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for IMAGE_FUNCTION_ENTRY64_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct IMAGE_LOAD_CONFIG_CODE_INTEGRITY {
    pub Flags: u16,
    pub Catalog: u16,
    pub CatalogOffset: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for IMAGE_LOAD_CONFIG_CODE_INTEGRITY {}
impl ::core::clone::Clone for IMAGE_LOAD_CONFIG_CODE_INTEGRITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAGE_LOAD_CONFIG_CODE_INTEGRITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_LOAD_CONFIG_CODE_INTEGRITY").field("Flags", &self.Flags).field("Catalog", &self.Catalog).field("CatalogOffset", &self.CatalogOffset).field("Reserved", &self.Reserved).finish()
    }
}
impl ::windows::core::TypeKind for IMAGE_LOAD_CONFIG_CODE_INTEGRITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IMAGE_LOAD_CONFIG_CODE_INTEGRITY {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Catalog == other.Catalog && self.CatalogOffset == other.CatalogOffset && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for IMAGE_LOAD_CONFIG_CODE_INTEGRITY {}
impl ::core::default::Default for IMAGE_LOAD_CONFIG_CODE_INTEGRITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct IMAGE_LOAD_CONFIG_DIRECTORY32 {
    pub Size: u32,
    pub TimeDateStamp: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub GlobalFlagsClear: u32,
    pub GlobalFlagsSet: u32,
    pub CriticalSectionDefaultTimeout: u32,
    pub DeCommitFreeBlockThreshold: u32,
    pub DeCommitTotalFreeThreshold: u32,
    pub LockPrefixTable: u32,
    pub MaximumAllocationSize: u32,
    pub VirtualMemoryThreshold: u32,
    pub ProcessHeapFlags: u32,
    pub ProcessAffinityMask: u32,
    pub CSDVersion: u16,
    pub DependentLoadFlags: u16,
    pub EditList: u32,
    pub SecurityCookie: u32,
    pub SEHandlerTable: u32,
    pub SEHandlerCount: u32,
    pub GuardCFCheckFunctionPointer: u32,
    pub GuardCFDispatchFunctionPointer: u32,
    pub GuardCFFunctionTable: u32,
    pub GuardCFFunctionCount: u32,
    pub GuardFlags: u32,
    pub CodeIntegrity: IMAGE_LOAD_CONFIG_CODE_INTEGRITY,
    pub GuardAddressTakenIatEntryTable: u32,
    pub GuardAddressTakenIatEntryCount: u32,
    pub GuardLongJumpTargetTable: u32,
    pub GuardLongJumpTargetCount: u32,
    pub DynamicValueRelocTable: u32,
    pub CHPEMetadataPointer: u32,
    pub GuardRFFailureRoutine: u32,
    pub GuardRFFailureRoutineFunctionPointer: u32,
    pub DynamicValueRelocTableOffset: u32,
    pub DynamicValueRelocTableSection: u16,
    pub Reserved2: u16,
    pub GuardRFVerifyStackPointerFunctionPointer: u32,
    pub HotPatchTableOffset: u32,
    pub Reserved3: u32,
    pub EnclaveConfigurationPointer: u32,
    pub VolatileMetadataPointer: u32,
    pub GuardEHContinuationTable: u32,
    pub GuardEHContinuationCount: u32,
    pub GuardXFGCheckFunctionPointer: u32,
    pub GuardXFGDispatchFunctionPointer: u32,
    pub GuardXFGTableDispatchFunctionPointer: u32,
    pub CastGuardOsDeterminedFailureMode: u32,
    pub GuardMemcpyFunctionPointer: u32,
}
impl ::core::marker::Copy for IMAGE_LOAD_CONFIG_DIRECTORY32 {}
impl ::core::clone::Clone for IMAGE_LOAD_CONFIG_DIRECTORY32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAGE_LOAD_CONFIG_DIRECTORY32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_LOAD_CONFIG_DIRECTORY32")
            .field("Size", &self.Size)
            .field("TimeDateStamp", &self.TimeDateStamp)
            .field("MajorVersion", &self.MajorVersion)
            .field("MinorVersion", &self.MinorVersion)
            .field("GlobalFlagsClear", &self.GlobalFlagsClear)
            .field("GlobalFlagsSet", &self.GlobalFlagsSet)
            .field("CriticalSectionDefaultTimeout", &self.CriticalSectionDefaultTimeout)
            .field("DeCommitFreeBlockThreshold", &self.DeCommitFreeBlockThreshold)
            .field("DeCommitTotalFreeThreshold", &self.DeCommitTotalFreeThreshold)
            .field("LockPrefixTable", &self.LockPrefixTable)
            .field("MaximumAllocationSize", &self.MaximumAllocationSize)
            .field("VirtualMemoryThreshold", &self.VirtualMemoryThreshold)
            .field("ProcessHeapFlags", &self.ProcessHeapFlags)
            .field("ProcessAffinityMask", &self.ProcessAffinityMask)
            .field("CSDVersion", &self.CSDVersion)
            .field("DependentLoadFlags", &self.DependentLoadFlags)
            .field("EditList", &self.EditList)
            .field("SecurityCookie", &self.SecurityCookie)
            .field("SEHandlerTable", &self.SEHandlerTable)
            .field("SEHandlerCount", &self.SEHandlerCount)
            .field("GuardCFCheckFunctionPointer", &self.GuardCFCheckFunctionPointer)
            .field("GuardCFDispatchFunctionPointer", &self.GuardCFDispatchFunctionPointer)
            .field("GuardCFFunctionTable", &self.GuardCFFunctionTable)
            .field("GuardCFFunctionCount", &self.GuardCFFunctionCount)
            .field("GuardFlags", &self.GuardFlags)
            .field("CodeIntegrity", &self.CodeIntegrity)
            .field("GuardAddressTakenIatEntryTable", &self.GuardAddressTakenIatEntryTable)
            .field("GuardAddressTakenIatEntryCount", &self.GuardAddressTakenIatEntryCount)
            .field("GuardLongJumpTargetTable", &self.GuardLongJumpTargetTable)
            .field("GuardLongJumpTargetCount", &self.GuardLongJumpTargetCount)
            .field("DynamicValueRelocTable", &self.DynamicValueRelocTable)
            .field("CHPEMetadataPointer", &self.CHPEMetadataPointer)
            .field("GuardRFFailureRoutine", &self.GuardRFFailureRoutine)
            .field("GuardRFFailureRoutineFunctionPointer", &self.GuardRFFailureRoutineFunctionPointer)
            .field("DynamicValueRelocTableOffset", &self.DynamicValueRelocTableOffset)
            .field("DynamicValueRelocTableSection", &self.DynamicValueRelocTableSection)
            .field("Reserved2", &self.Reserved2)
            .field("GuardRFVerifyStackPointerFunctionPointer", &self.GuardRFVerifyStackPointerFunctionPointer)
            .field("HotPatchTableOffset", &self.HotPatchTableOffset)
            .field("Reserved3", &self.Reserved3)
            .field("EnclaveConfigurationPointer", &self.EnclaveConfigurationPointer)
            .field("VolatileMetadataPointer", &self.VolatileMetadataPointer)
            .field("GuardEHContinuationTable", &self.GuardEHContinuationTable)
            .field("GuardEHContinuationCount", &self.GuardEHContinuationCount)
            .field("GuardXFGCheckFunctionPointer", &self.GuardXFGCheckFunctionPointer)
            .field("GuardXFGDispatchFunctionPointer", &self.GuardXFGDispatchFunctionPointer)
            .field("GuardXFGTableDispatchFunctionPointer", &self.GuardXFGTableDispatchFunctionPointer)
            .field("CastGuardOsDeterminedFailureMode", &self.CastGuardOsDeterminedFailureMode)
            .field("GuardMemcpyFunctionPointer", &self.GuardMemcpyFunctionPointer)
            .finish()
    }
}
impl ::windows::core::TypeKind for IMAGE_LOAD_CONFIG_DIRECTORY32 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IMAGE_LOAD_CONFIG_DIRECTORY32 {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.TimeDateStamp == other.TimeDateStamp
            && self.MajorVersion == other.MajorVersion
            && self.MinorVersion == other.MinorVersion
            && self.GlobalFlagsClear == other.GlobalFlagsClear
            && self.GlobalFlagsSet == other.GlobalFlagsSet
            && self.CriticalSectionDefaultTimeout == other.CriticalSectionDefaultTimeout
            && self.DeCommitFreeBlockThreshold == other.DeCommitFreeBlockThreshold
            && self.DeCommitTotalFreeThreshold == other.DeCommitTotalFreeThreshold
            && self.LockPrefixTable == other.LockPrefixTable
            && self.MaximumAllocationSize == other.MaximumAllocationSize
            && self.VirtualMemoryThreshold == other.VirtualMemoryThreshold
            && self.ProcessHeapFlags == other.ProcessHeapFlags
            && self.ProcessAffinityMask == other.ProcessAffinityMask
            && self.CSDVersion == other.CSDVersion
            && self.DependentLoadFlags == other.DependentLoadFlags
            && self.EditList == other.EditList
            && self.SecurityCookie == other.SecurityCookie
            && self.SEHandlerTable == other.SEHandlerTable
            && self.SEHandlerCount == other.SEHandlerCount
            && self.GuardCFCheckFunctionPointer == other.GuardCFCheckFunctionPointer
            && self.GuardCFDispatchFunctionPointer == other.GuardCFDispatchFunctionPointer
            && self.GuardCFFunctionTable == other.GuardCFFunctionTable
            && self.GuardCFFunctionCount == other.GuardCFFunctionCount
            && self.GuardFlags == other.GuardFlags
            && self.CodeIntegrity == other.CodeIntegrity
            && self.GuardAddressTakenIatEntryTable == other.GuardAddressTakenIatEntryTable
            && self.GuardAddressTakenIatEntryCount == other.GuardAddressTakenIatEntryCount
            && self.GuardLongJumpTargetTable == other.GuardLongJumpTargetTable
            && self.GuardLongJumpTargetCount == other.GuardLongJumpTargetCount
            && self.DynamicValueRelocTable == other.DynamicValueRelocTable
            && self.CHPEMetadataPointer == other.CHPEMetadataPointer
            && self.GuardRFFailureRoutine == other.GuardRFFailureRoutine
            && self.GuardRFFailureRoutineFunctionPointer == other.GuardRFFailureRoutineFunctionPointer
            && self.DynamicValueRelocTableOffset == other.DynamicValueRelocTableOffset
            && self.DynamicValueRelocTableSection == other.DynamicValueRelocTableSection
            && self.Reserved2 == other.Reserved2
            && self.GuardRFVerifyStackPointerFunctionPointer == other.GuardRFVerifyStackPointerFunctionPointer
            && self.HotPatchTableOffset == other.HotPatchTableOffset
            && self.Reserved3 == other.Reserved3
            && self.EnclaveConfigurationPointer == other.EnclaveConfigurationPointer
            && self.VolatileMetadataPointer == other.VolatileMetadataPointer
            && self.GuardEHContinuationTable == other.GuardEHContinuationTable
            && self.GuardEHContinuationCount == other.GuardEHContinuationCount
            && self.GuardXFGCheckFunctionPointer == other.GuardXFGCheckFunctionPointer
            && self.GuardXFGDispatchFunctionPointer == other.GuardXFGDispatchFunctionPointer
            && self.GuardXFGTableDispatchFunctionPointer == other.GuardXFGTableDispatchFunctionPointer
            && self.CastGuardOsDeterminedFailureMode == other.CastGuardOsDeterminedFailureMode
            && self.GuardMemcpyFunctionPointer == other.GuardMemcpyFunctionPointer
    }
}
impl ::core::cmp::Eq for IMAGE_LOAD_CONFIG_DIRECTORY32 {}
impl ::core::default::Default for IMAGE_LOAD_CONFIG_DIRECTORY32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct IMAGE_LOAD_CONFIG_DIRECTORY64 {
    pub Size: u32,
    pub TimeDateStamp: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub GlobalFlagsClear: u32,
    pub GlobalFlagsSet: u32,
    pub CriticalSectionDefaultTimeout: u32,
    pub DeCommitFreeBlockThreshold: u64,
    pub DeCommitTotalFreeThreshold: u64,
    pub LockPrefixTable: u64,
    pub MaximumAllocationSize: u64,
    pub VirtualMemoryThreshold: u64,
    pub ProcessAffinityMask: u64,
    pub ProcessHeapFlags: u32,
    pub CSDVersion: u16,
    pub DependentLoadFlags: u16,
    pub EditList: u64,
    pub SecurityCookie: u64,
    pub SEHandlerTable: u64,
    pub SEHandlerCount: u64,
    pub GuardCFCheckFunctionPointer: u64,
    pub GuardCFDispatchFunctionPointer: u64,
    pub GuardCFFunctionTable: u64,
    pub GuardCFFunctionCount: u64,
    pub GuardFlags: u32,
    pub CodeIntegrity: IMAGE_LOAD_CONFIG_CODE_INTEGRITY,
    pub GuardAddressTakenIatEntryTable: u64,
    pub GuardAddressTakenIatEntryCount: u64,
    pub GuardLongJumpTargetTable: u64,
    pub GuardLongJumpTargetCount: u64,
    pub DynamicValueRelocTable: u64,
    pub CHPEMetadataPointer: u64,
    pub GuardRFFailureRoutine: u64,
    pub GuardRFFailureRoutineFunctionPointer: u64,
    pub DynamicValueRelocTableOffset: u32,
    pub DynamicValueRelocTableSection: u16,
    pub Reserved2: u16,
    pub GuardRFVerifyStackPointerFunctionPointer: u64,
    pub HotPatchTableOffset: u32,
    pub Reserved3: u32,
    pub EnclaveConfigurationPointer: u64,
    pub VolatileMetadataPointer: u64,
    pub GuardEHContinuationTable: u64,
    pub GuardEHContinuationCount: u64,
    pub GuardXFGCheckFunctionPointer: u64,
    pub GuardXFGDispatchFunctionPointer: u64,
    pub GuardXFGTableDispatchFunctionPointer: u64,
    pub CastGuardOsDeterminedFailureMode: u64,
    pub GuardMemcpyFunctionPointer: u64,
}
impl ::core::marker::Copy for IMAGE_LOAD_CONFIG_DIRECTORY64 {}
impl ::core::clone::Clone for IMAGE_LOAD_CONFIG_DIRECTORY64 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for IMAGE_LOAD_CONFIG_DIRECTORY64 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for IMAGE_LOAD_CONFIG_DIRECTORY64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(feature = "Win32_System_SystemInformation")]
pub struct IMAGE_NT_HEADERS32 {
    pub Signature: u32,
    pub FileHeader: IMAGE_FILE_HEADER,
    pub OptionalHeader: IMAGE_OPTIONAL_HEADER32,
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::marker::Copy for IMAGE_NT_HEADERS32 {}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::clone::Clone for IMAGE_NT_HEADERS32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::fmt::Debug for IMAGE_NT_HEADERS32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_NT_HEADERS32").field("Signature", &self.Signature).field("FileHeader", &self.FileHeader).field("OptionalHeader", &self.OptionalHeader).finish()
    }
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::windows::core::TypeKind for IMAGE_NT_HEADERS32 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::cmp::PartialEq for IMAGE_NT_HEADERS32 {
    fn eq(&self, other: &Self) -> bool {
        self.Signature == other.Signature && self.FileHeader == other.FileHeader && self.OptionalHeader == other.OptionalHeader
    }
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::cmp::Eq for IMAGE_NT_HEADERS32 {}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::default::Default for IMAGE_NT_HEADERS32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(feature = "Win32_System_SystemInformation")]
pub struct IMAGE_NT_HEADERS64 {
    pub Signature: u32,
    pub FileHeader: IMAGE_FILE_HEADER,
    pub OptionalHeader: IMAGE_OPTIONAL_HEADER64,
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::marker::Copy for IMAGE_NT_HEADERS64 {}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::clone::Clone for IMAGE_NT_HEADERS64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::windows::core::TypeKind for IMAGE_NT_HEADERS64 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::default::Default for IMAGE_NT_HEADERS64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct IMAGE_OPTIONAL_HEADER32 {
    pub Magic: IMAGE_OPTIONAL_HEADER_MAGIC,
    pub MajorLinkerVersion: u8,
    pub MinorLinkerVersion: u8,
    pub SizeOfCode: u32,
    pub SizeOfInitializedData: u32,
    pub SizeOfUninitializedData: u32,
    pub AddressOfEntryPoint: u32,
    pub BaseOfCode: u32,
    pub BaseOfData: u32,
    pub ImageBase: u32,
    pub SectionAlignment: u32,
    pub FileAlignment: u32,
    pub MajorOperatingSystemVersion: u16,
    pub MinorOperatingSystemVersion: u16,
    pub MajorImageVersion: u16,
    pub MinorImageVersion: u16,
    pub MajorSubsystemVersion: u16,
    pub MinorSubsystemVersion: u16,
    pub Win32VersionValue: u32,
    pub SizeOfImage: u32,
    pub SizeOfHeaders: u32,
    pub CheckSum: u32,
    pub Subsystem: IMAGE_SUBSYSTEM,
    pub DllCharacteristics: IMAGE_DLL_CHARACTERISTICS,
    pub SizeOfStackReserve: u32,
    pub SizeOfStackCommit: u32,
    pub SizeOfHeapReserve: u32,
    pub SizeOfHeapCommit: u32,
    pub LoaderFlags: u32,
    pub NumberOfRvaAndSizes: u32,
    pub DataDirectory: [IMAGE_DATA_DIRECTORY; 16],
}
impl ::core::marker::Copy for IMAGE_OPTIONAL_HEADER32 {}
impl ::core::clone::Clone for IMAGE_OPTIONAL_HEADER32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAGE_OPTIONAL_HEADER32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_OPTIONAL_HEADER32")
            .field("Magic", &self.Magic)
            .field("MajorLinkerVersion", &self.MajorLinkerVersion)
            .field("MinorLinkerVersion", &self.MinorLinkerVersion)
            .field("SizeOfCode", &self.SizeOfCode)
            .field("SizeOfInitializedData", &self.SizeOfInitializedData)
            .field("SizeOfUninitializedData", &self.SizeOfUninitializedData)
            .field("AddressOfEntryPoint", &self.AddressOfEntryPoint)
            .field("BaseOfCode", &self.BaseOfCode)
            .field("BaseOfData", &self.BaseOfData)
            .field("ImageBase", &self.ImageBase)
            .field("SectionAlignment", &self.SectionAlignment)
            .field("FileAlignment", &self.FileAlignment)
            .field("MajorOperatingSystemVersion", &self.MajorOperatingSystemVersion)
            .field("MinorOperatingSystemVersion", &self.MinorOperatingSystemVersion)
            .field("MajorImageVersion", &self.MajorImageVersion)
            .field("MinorImageVersion", &self.MinorImageVersion)
            .field("MajorSubsystemVersion", &self.MajorSubsystemVersion)
            .field("MinorSubsystemVersion", &self.MinorSubsystemVersion)
            .field("Win32VersionValue", &self.Win32VersionValue)
            .field("SizeOfImage", &self.SizeOfImage)
            .field("SizeOfHeaders", &self.SizeOfHeaders)
            .field("CheckSum", &self.CheckSum)
            .field("Subsystem", &self.Subsystem)
            .field("DllCharacteristics", &self.DllCharacteristics)
            .field("SizeOfStackReserve", &self.SizeOfStackReserve)
            .field("SizeOfStackCommit", &self.SizeOfStackCommit)
            .field("SizeOfHeapReserve", &self.SizeOfHeapReserve)
            .field("SizeOfHeapCommit", &self.SizeOfHeapCommit)
            .field("LoaderFlags", &self.LoaderFlags)
            .field("NumberOfRvaAndSizes", &self.NumberOfRvaAndSizes)
            .field("DataDirectory", &self.DataDirectory)
            .finish()
    }
}
impl ::windows::core::TypeKind for IMAGE_OPTIONAL_HEADER32 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IMAGE_OPTIONAL_HEADER32 {
    fn eq(&self, other: &Self) -> bool {
        self.Magic == other.Magic
            && self.MajorLinkerVersion == other.MajorLinkerVersion
            && self.MinorLinkerVersion == other.MinorLinkerVersion
            && self.SizeOfCode == other.SizeOfCode
            && self.SizeOfInitializedData == other.SizeOfInitializedData
            && self.SizeOfUninitializedData == other.SizeOfUninitializedData
            && self.AddressOfEntryPoint == other.AddressOfEntryPoint
            && self.BaseOfCode == other.BaseOfCode
            && self.BaseOfData == other.BaseOfData
            && self.ImageBase == other.ImageBase
            && self.SectionAlignment == other.SectionAlignment
            && self.FileAlignment == other.FileAlignment
            && self.MajorOperatingSystemVersion == other.MajorOperatingSystemVersion
            && self.MinorOperatingSystemVersion == other.MinorOperatingSystemVersion
            && self.MajorImageVersion == other.MajorImageVersion
            && self.MinorImageVersion == other.MinorImageVersion
            && self.MajorSubsystemVersion == other.MajorSubsystemVersion
            && self.MinorSubsystemVersion == other.MinorSubsystemVersion
            && self.Win32VersionValue == other.Win32VersionValue
            && self.SizeOfImage == other.SizeOfImage
            && self.SizeOfHeaders == other.SizeOfHeaders
            && self.CheckSum == other.CheckSum
            && self.Subsystem == other.Subsystem
            && self.DllCharacteristics == other.DllCharacteristics
            && self.SizeOfStackReserve == other.SizeOfStackReserve
            && self.SizeOfStackCommit == other.SizeOfStackCommit
            && self.SizeOfHeapReserve == other.SizeOfHeapReserve
            && self.SizeOfHeapCommit == other.SizeOfHeapCommit
            && self.LoaderFlags == other.LoaderFlags
            && self.NumberOfRvaAndSizes == other.NumberOfRvaAndSizes
            && self.DataDirectory == other.DataDirectory
    }
}
impl ::core::cmp::Eq for IMAGE_OPTIONAL_HEADER32 {}
impl ::core::default::Default for IMAGE_OPTIONAL_HEADER32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct IMAGE_OPTIONAL_HEADER64 {
    pub Magic: IMAGE_OPTIONAL_HEADER_MAGIC,
    pub MajorLinkerVersion: u8,
    pub MinorLinkerVersion: u8,
    pub SizeOfCode: u32,
    pub SizeOfInitializedData: u32,
    pub SizeOfUninitializedData: u32,
    pub AddressOfEntryPoint: u32,
    pub BaseOfCode: u32,
    pub ImageBase: u64,
    pub SectionAlignment: u32,
    pub FileAlignment: u32,
    pub MajorOperatingSystemVersion: u16,
    pub MinorOperatingSystemVersion: u16,
    pub MajorImageVersion: u16,
    pub MinorImageVersion: u16,
    pub MajorSubsystemVersion: u16,
    pub MinorSubsystemVersion: u16,
    pub Win32VersionValue: u32,
    pub SizeOfImage: u32,
    pub SizeOfHeaders: u32,
    pub CheckSum: u32,
    pub Subsystem: IMAGE_SUBSYSTEM,
    pub DllCharacteristics: IMAGE_DLL_CHARACTERISTICS,
    pub SizeOfStackReserve: u64,
    pub SizeOfStackCommit: u64,
    pub SizeOfHeapReserve: u64,
    pub SizeOfHeapCommit: u64,
    pub LoaderFlags: u32,
    pub NumberOfRvaAndSizes: u32,
    pub DataDirectory: [IMAGE_DATA_DIRECTORY; 16],
}
impl ::core::marker::Copy for IMAGE_OPTIONAL_HEADER64 {}
impl ::core::clone::Clone for IMAGE_OPTIONAL_HEADER64 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for IMAGE_OPTIONAL_HEADER64 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for IMAGE_OPTIONAL_HEADER64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(feature = "Win32_System_SystemInformation")]
pub struct IMAGE_ROM_HEADERS {
    pub FileHeader: IMAGE_FILE_HEADER,
    pub OptionalHeader: IMAGE_ROM_OPTIONAL_HEADER,
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::marker::Copy for IMAGE_ROM_HEADERS {}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::clone::Clone for IMAGE_ROM_HEADERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::fmt::Debug for IMAGE_ROM_HEADERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_ROM_HEADERS").field("FileHeader", &self.FileHeader).field("OptionalHeader", &self.OptionalHeader).finish()
    }
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::windows::core::TypeKind for IMAGE_ROM_HEADERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::cmp::PartialEq for IMAGE_ROM_HEADERS {
    fn eq(&self, other: &Self) -> bool {
        self.FileHeader == other.FileHeader && self.OptionalHeader == other.OptionalHeader
    }
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::cmp::Eq for IMAGE_ROM_HEADERS {}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::default::Default for IMAGE_ROM_HEADERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct IMAGE_ROM_OPTIONAL_HEADER {
    pub Magic: u16,
    pub MajorLinkerVersion: u8,
    pub MinorLinkerVersion: u8,
    pub SizeOfCode: u32,
    pub SizeOfInitializedData: u32,
    pub SizeOfUninitializedData: u32,
    pub AddressOfEntryPoint: u32,
    pub BaseOfCode: u32,
    pub BaseOfData: u32,
    pub BaseOfBss: u32,
    pub GprMask: u32,
    pub CprMask: [u32; 4],
    pub GpValue: u32,
}
impl ::core::marker::Copy for IMAGE_ROM_OPTIONAL_HEADER {}
impl ::core::clone::Clone for IMAGE_ROM_OPTIONAL_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAGE_ROM_OPTIONAL_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_ROM_OPTIONAL_HEADER")
            .field("Magic", &self.Magic)
            .field("MajorLinkerVersion", &self.MajorLinkerVersion)
            .field("MinorLinkerVersion", &self.MinorLinkerVersion)
            .field("SizeOfCode", &self.SizeOfCode)
            .field("SizeOfInitializedData", &self.SizeOfInitializedData)
            .field("SizeOfUninitializedData", &self.SizeOfUninitializedData)
            .field("AddressOfEntryPoint", &self.AddressOfEntryPoint)
            .field("BaseOfCode", &self.BaseOfCode)
            .field("BaseOfData", &self.BaseOfData)
            .field("BaseOfBss", &self.BaseOfBss)
            .field("GprMask", &self.GprMask)
            .field("CprMask", &self.CprMask)
            .field("GpValue", &self.GpValue)
            .finish()
    }
}
impl ::windows::core::TypeKind for IMAGE_ROM_OPTIONAL_HEADER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IMAGE_ROM_OPTIONAL_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Magic == other.Magic && self.MajorLinkerVersion == other.MajorLinkerVersion && self.MinorLinkerVersion == other.MinorLinkerVersion && self.SizeOfCode == other.SizeOfCode && self.SizeOfInitializedData == other.SizeOfInitializedData && self.SizeOfUninitializedData == other.SizeOfUninitializedData && self.AddressOfEntryPoint == other.AddressOfEntryPoint && self.BaseOfCode == other.BaseOfCode && self.BaseOfData == other.BaseOfData && self.BaseOfBss == other.BaseOfBss && self.GprMask == other.GprMask && self.CprMask == other.CprMask && self.GpValue == other.GpValue
    }
}
impl ::core::cmp::Eq for IMAGE_ROM_OPTIONAL_HEADER {}
impl ::core::default::Default for IMAGE_ROM_OPTIONAL_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct IMAGE_RUNTIME_FUNCTION_ENTRY {
    pub BeginAddress: u32,
    pub EndAddress: u32,
    pub Anonymous: IMAGE_RUNTIME_FUNCTION_ENTRY_0,
}
impl ::core::marker::Copy for IMAGE_RUNTIME_FUNCTION_ENTRY {}
impl ::core::clone::Clone for IMAGE_RUNTIME_FUNCTION_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for IMAGE_RUNTIME_FUNCTION_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for IMAGE_RUNTIME_FUNCTION_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub union IMAGE_RUNTIME_FUNCTION_ENTRY_0 {
    pub UnwindInfoAddress: u32,
    pub UnwindData: u32,
}
impl ::core::marker::Copy for IMAGE_RUNTIME_FUNCTION_ENTRY_0 {}
impl ::core::clone::Clone for IMAGE_RUNTIME_FUNCTION_ENTRY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for IMAGE_RUNTIME_FUNCTION_ENTRY_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for IMAGE_RUNTIME_FUNCTION_ENTRY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct IMAGE_SECTION_HEADER {
    pub Name: [u8; 8],
    pub Misc: IMAGE_SECTION_HEADER_0,
    pub VirtualAddress: u32,
    pub SizeOfRawData: u32,
    pub PointerToRawData: u32,
    pub PointerToRelocations: u32,
    pub PointerToLinenumbers: u32,
    pub NumberOfRelocations: u16,
    pub NumberOfLinenumbers: u16,
    pub Characteristics: IMAGE_SECTION_CHARACTERISTICS,
}
impl ::core::marker::Copy for IMAGE_SECTION_HEADER {}
impl ::core::clone::Clone for IMAGE_SECTION_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for IMAGE_SECTION_HEADER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for IMAGE_SECTION_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub union IMAGE_SECTION_HEADER_0 {
    pub PhysicalAddress: u32,
    pub VirtualSize: u32,
}
impl ::core::marker::Copy for IMAGE_SECTION_HEADER_0 {}
impl ::core::clone::Clone for IMAGE_SECTION_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for IMAGE_SECTION_HEADER_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for IMAGE_SECTION_HEADER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct IPMI_OS_SEL_RECORD {
    pub Signature: u32,
    pub Version: u32,
    pub Length: u32,
    pub RecordType: IPMI_OS_SEL_RECORD_TYPE,
    pub DataLength: u32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for IPMI_OS_SEL_RECORD {}
impl ::core::clone::Clone for IPMI_OS_SEL_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for IPMI_OS_SEL_RECORD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for IPMI_OS_SEL_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86")]
pub struct KDHELP {
    pub Thread: u32,
    pub ThCallbackStack: u32,
    pub NextCallback: u32,
    pub FramePointer: u32,
    pub KiCallUserMode: u32,
    pub KeUserCallbackDispatcher: u32,
    pub SystemRangeStart: u32,
    pub ThCallbackBStore: u32,
    pub KiUserExceptionDispatcher: u32,
    pub StackBase: u32,
    pub StackLimit: u32,
    pub Reserved: [u32; 5],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for KDHELP {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for KDHELP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for KDHELP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KDHELP")
            .field("Thread", &self.Thread)
            .field("ThCallbackStack", &self.ThCallbackStack)
            .field("NextCallback", &self.NextCallback)
            .field("FramePointer", &self.FramePointer)
            .field("KiCallUserMode", &self.KiCallUserMode)
            .field("KeUserCallbackDispatcher", &self.KeUserCallbackDispatcher)
            .field("SystemRangeStart", &self.SystemRangeStart)
            .field("ThCallbackBStore", &self.ThCallbackBStore)
            .field("KiUserExceptionDispatcher", &self.KiUserExceptionDispatcher)
            .field("StackBase", &self.StackBase)
            .field("StackLimit", &self.StackLimit)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::windows::core::TypeKind for KDHELP {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for KDHELP {
    fn eq(&self, other: &Self) -> bool {
        self.Thread == other.Thread && self.ThCallbackStack == other.ThCallbackStack && self.NextCallback == other.NextCallback && self.FramePointer == other.FramePointer && self.KiCallUserMode == other.KiCallUserMode && self.KeUserCallbackDispatcher == other.KeUserCallbackDispatcher && self.SystemRangeStart == other.SystemRangeStart && self.ThCallbackBStore == other.ThCallbackBStore && self.KiUserExceptionDispatcher == other.KiUserExceptionDispatcher && self.StackBase == other.StackBase && self.StackLimit == other.StackLimit && self.Reserved == other.Reserved
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for KDHELP {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for KDHELP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct KDHELP64 {
    pub Thread: u64,
    pub ThCallbackStack: u32,
    pub ThCallbackBStore: u32,
    pub NextCallback: u32,
    pub FramePointer: u32,
    pub KiCallUserMode: u64,
    pub KeUserCallbackDispatcher: u64,
    pub SystemRangeStart: u64,
    pub KiUserExceptionDispatcher: u64,
    pub StackBase: u64,
    pub StackLimit: u64,
    pub BuildVersion: u32,
    pub RetpolineStubFunctionTableSize: u32,
    pub RetpolineStubFunctionTable: u64,
    pub RetpolineStubOffset: u32,
    pub RetpolineStubSize: u32,
    pub Reserved0: [u64; 2],
}
impl ::core::marker::Copy for KDHELP64 {}
impl ::core::clone::Clone for KDHELP64 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KDHELP64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KDHELP64")
            .field("Thread", &self.Thread)
            .field("ThCallbackStack", &self.ThCallbackStack)
            .field("ThCallbackBStore", &self.ThCallbackBStore)
            .field("NextCallback", &self.NextCallback)
            .field("FramePointer", &self.FramePointer)
            .field("KiCallUserMode", &self.KiCallUserMode)
            .field("KeUserCallbackDispatcher", &self.KeUserCallbackDispatcher)
            .field("SystemRangeStart", &self.SystemRangeStart)
            .field("KiUserExceptionDispatcher", &self.KiUserExceptionDispatcher)
            .field("StackBase", &self.StackBase)
            .field("StackLimit", &self.StackLimit)
            .field("BuildVersion", &self.BuildVersion)
            .field("RetpolineStubFunctionTableSize", &self.RetpolineStubFunctionTableSize)
            .field("RetpolineStubFunctionTable", &self.RetpolineStubFunctionTable)
            .field("RetpolineStubOffset", &self.RetpolineStubOffset)
            .field("RetpolineStubSize", &self.RetpolineStubSize)
            .field("Reserved0", &self.Reserved0)
            .finish()
    }
}
impl ::windows::core::TypeKind for KDHELP64 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for KDHELP64 {
    fn eq(&self, other: &Self) -> bool {
        self.Thread == other.Thread
            && self.ThCallbackStack == other.ThCallbackStack
            && self.ThCallbackBStore == other.ThCallbackBStore
            && self.NextCallback == other.NextCallback
            && self.FramePointer == other.FramePointer
            && self.KiCallUserMode == other.KiCallUserMode
            && self.KeUserCallbackDispatcher == other.KeUserCallbackDispatcher
            && self.SystemRangeStart == other.SystemRangeStart
            && self.KiUserExceptionDispatcher == other.KiUserExceptionDispatcher
            && self.StackBase == other.StackBase
            && self.StackLimit == other.StackLimit
            && self.BuildVersion == other.BuildVersion
            && self.RetpolineStubFunctionTableSize == other.RetpolineStubFunctionTableSize
            && self.RetpolineStubFunctionTable == other.RetpolineStubFunctionTable
            && self.RetpolineStubOffset == other.RetpolineStubOffset
            && self.RetpolineStubSize == other.RetpolineStubSize
            && self.Reserved0 == other.Reserved0
    }
}
impl ::core::cmp::Eq for KDHELP64 {}
impl ::core::default::Default for KDHELP64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86_64")]
pub struct KNONVOLATILE_CONTEXT_POINTERS {
    pub Anonymous1: KNONVOLATILE_CONTEXT_POINTERS_0,
    pub Anonymous2: KNONVOLATILE_CONTEXT_POINTERS_1,
}
#[cfg(target_arch = "x86_64")]
impl ::core::marker::Copy for KNONVOLATILE_CONTEXT_POINTERS {}
#[cfg(target_arch = "x86_64")]
impl ::core::clone::Clone for KNONVOLATILE_CONTEXT_POINTERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86_64")]
impl ::windows::core::TypeKind for KNONVOLATILE_CONTEXT_POINTERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86_64")]
impl ::core::default::Default for KNONVOLATILE_CONTEXT_POINTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86_64")]
pub union KNONVOLATILE_CONTEXT_POINTERS_0 {
    pub FloatingContext: [*mut M128A; 16],
    pub Anonymous: KNONVOLATILE_CONTEXT_POINTERS_0_0,
}
#[cfg(target_arch = "x86_64")]
impl ::core::marker::Copy for KNONVOLATILE_CONTEXT_POINTERS_0 {}
#[cfg(target_arch = "x86_64")]
impl ::core::clone::Clone for KNONVOLATILE_CONTEXT_POINTERS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86_64")]
impl ::windows::core::TypeKind for KNONVOLATILE_CONTEXT_POINTERS_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86_64")]
impl ::core::default::Default for KNONVOLATILE_CONTEXT_POINTERS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86_64")]
pub struct KNONVOLATILE_CONTEXT_POINTERS_0_0 {
    pub Xmm0: *mut M128A,
    pub Xmm1: *mut M128A,
    pub Xmm2: *mut M128A,
    pub Xmm3: *mut M128A,
    pub Xmm4: *mut M128A,
    pub Xmm5: *mut M128A,
    pub Xmm6: *mut M128A,
    pub Xmm7: *mut M128A,
    pub Xmm8: *mut M128A,
    pub Xmm9: *mut M128A,
    pub Xmm10: *mut M128A,
    pub Xmm11: *mut M128A,
    pub Xmm12: *mut M128A,
    pub Xmm13: *mut M128A,
    pub Xmm14: *mut M128A,
    pub Xmm15: *mut M128A,
}
#[cfg(target_arch = "x86_64")]
impl ::core::marker::Copy for KNONVOLATILE_CONTEXT_POINTERS_0_0 {}
#[cfg(target_arch = "x86_64")]
impl ::core::clone::Clone for KNONVOLATILE_CONTEXT_POINTERS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86_64")]
impl ::core::fmt::Debug for KNONVOLATILE_CONTEXT_POINTERS_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KNONVOLATILE_CONTEXT_POINTERS_0_0")
            .field("Xmm0", &self.Xmm0)
            .field("Xmm1", &self.Xmm1)
            .field("Xmm2", &self.Xmm2)
            .field("Xmm3", &self.Xmm3)
            .field("Xmm4", &self.Xmm4)
            .field("Xmm5", &self.Xmm5)
            .field("Xmm6", &self.Xmm6)
            .field("Xmm7", &self.Xmm7)
            .field("Xmm8", &self.Xmm8)
            .field("Xmm9", &self.Xmm9)
            .field("Xmm10", &self.Xmm10)
            .field("Xmm11", &self.Xmm11)
            .field("Xmm12", &self.Xmm12)
            .field("Xmm13", &self.Xmm13)
            .field("Xmm14", &self.Xmm14)
            .field("Xmm15", &self.Xmm15)
            .finish()
    }
}
#[cfg(target_arch = "x86_64")]
impl ::windows::core::TypeKind for KNONVOLATILE_CONTEXT_POINTERS_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86_64")]
impl ::core::cmp::PartialEq for KNONVOLATILE_CONTEXT_POINTERS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Xmm0 == other.Xmm0 && self.Xmm1 == other.Xmm1 && self.Xmm2 == other.Xmm2 && self.Xmm3 == other.Xmm3 && self.Xmm4 == other.Xmm4 && self.Xmm5 == other.Xmm5 && self.Xmm6 == other.Xmm6 && self.Xmm7 == other.Xmm7 && self.Xmm8 == other.Xmm8 && self.Xmm9 == other.Xmm9 && self.Xmm10 == other.Xmm10 && self.Xmm11 == other.Xmm11 && self.Xmm12 == other.Xmm12 && self.Xmm13 == other.Xmm13 && self.Xmm14 == other.Xmm14 && self.Xmm15 == other.Xmm15
    }
}
#[cfg(target_arch = "x86_64")]
impl ::core::cmp::Eq for KNONVOLATILE_CONTEXT_POINTERS_0_0 {}
#[cfg(target_arch = "x86_64")]
impl ::core::default::Default for KNONVOLATILE_CONTEXT_POINTERS_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86_64")]
pub union KNONVOLATILE_CONTEXT_POINTERS_1 {
    pub IntegerContext: [*mut u64; 16],
    pub Anonymous: KNONVOLATILE_CONTEXT_POINTERS_1_0,
}
#[cfg(target_arch = "x86_64")]
impl ::core::marker::Copy for KNONVOLATILE_CONTEXT_POINTERS_1 {}
#[cfg(target_arch = "x86_64")]
impl ::core::clone::Clone for KNONVOLATILE_CONTEXT_POINTERS_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86_64")]
impl ::windows::core::TypeKind for KNONVOLATILE_CONTEXT_POINTERS_1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86_64")]
impl ::core::default::Default for KNONVOLATILE_CONTEXT_POINTERS_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86_64")]
pub struct KNONVOLATILE_CONTEXT_POINTERS_1_0 {
    pub Rax: *mut u64,
    pub Rcx: *mut u64,
    pub Rdx: *mut u64,
    pub Rbx: *mut u64,
    pub Rsp: *mut u64,
    pub Rbp: *mut u64,
    pub Rsi: *mut u64,
    pub Rdi: *mut u64,
    pub R8: *mut u64,
    pub R9: *mut u64,
    pub R10: *mut u64,
    pub R11: *mut u64,
    pub R12: *mut u64,
    pub R13: *mut u64,
    pub R14: *mut u64,
    pub R15: *mut u64,
}
#[cfg(target_arch = "x86_64")]
impl ::core::marker::Copy for KNONVOLATILE_CONTEXT_POINTERS_1_0 {}
#[cfg(target_arch = "x86_64")]
impl ::core::clone::Clone for KNONVOLATILE_CONTEXT_POINTERS_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86_64")]
impl ::core::fmt::Debug for KNONVOLATILE_CONTEXT_POINTERS_1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KNONVOLATILE_CONTEXT_POINTERS_1_0").field("Rax", &self.Rax).field("Rcx", &self.Rcx).field("Rdx", &self.Rdx).field("Rbx", &self.Rbx).field("Rsp", &self.Rsp).field("Rbp", &self.Rbp).field("Rsi", &self.Rsi).field("Rdi", &self.Rdi).field("R8", &self.R8).field("R9", &self.R9).field("R10", &self.R10).field("R11", &self.R11).field("R12", &self.R12).field("R13", &self.R13).field("R14", &self.R14).field("R15", &self.R15).finish()
    }
}
#[cfg(target_arch = "x86_64")]
impl ::windows::core::TypeKind for KNONVOLATILE_CONTEXT_POINTERS_1_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86_64")]
impl ::core::cmp::PartialEq for KNONVOLATILE_CONTEXT_POINTERS_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Rax == other.Rax && self.Rcx == other.Rcx && self.Rdx == other.Rdx && self.Rbx == other.Rbx && self.Rsp == other.Rsp && self.Rbp == other.Rbp && self.Rsi == other.Rsi && self.Rdi == other.Rdi && self.R8 == other.R8 && self.R9 == other.R9 && self.R10 == other.R10 && self.R11 == other.R11 && self.R12 == other.R12 && self.R13 == other.R13 && self.R14 == other.R14 && self.R15 == other.R15
    }
}
#[cfg(target_arch = "x86_64")]
impl ::core::cmp::Eq for KNONVOLATILE_CONTEXT_POINTERS_1_0 {}
#[cfg(target_arch = "x86_64")]
impl ::core::default::Default for KNONVOLATILE_CONTEXT_POINTERS_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86")]
pub struct KNONVOLATILE_CONTEXT_POINTERS {
    pub Dummy: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for KNONVOLATILE_CONTEXT_POINTERS {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for KNONVOLATILE_CONTEXT_POINTERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::windows::core::TypeKind for KNONVOLATILE_CONTEXT_POINTERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for KNONVOLATILE_CONTEXT_POINTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "aarch64")]
pub struct KNONVOLATILE_CONTEXT_POINTERS_ARM64 {
    pub X19: *mut u64,
    pub X20: *mut u64,
    pub X21: *mut u64,
    pub X22: *mut u64,
    pub X23: *mut u64,
    pub X24: *mut u64,
    pub X25: *mut u64,
    pub X26: *mut u64,
    pub X27: *mut u64,
    pub X28: *mut u64,
    pub Fp: *mut u64,
    pub Lr: *mut u64,
    pub D8: *mut u64,
    pub D9: *mut u64,
    pub D10: *mut u64,
    pub D11: *mut u64,
    pub D12: *mut u64,
    pub D13: *mut u64,
    pub D14: *mut u64,
    pub D15: *mut u64,
}
#[cfg(target_arch = "aarch64")]
impl ::core::marker::Copy for KNONVOLATILE_CONTEXT_POINTERS_ARM64 {}
#[cfg(target_arch = "aarch64")]
impl ::core::clone::Clone for KNONVOLATILE_CONTEXT_POINTERS_ARM64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "aarch64")]
impl ::core::fmt::Debug for KNONVOLATILE_CONTEXT_POINTERS_ARM64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KNONVOLATILE_CONTEXT_POINTERS_ARM64")
            .field("X19", &self.X19)
            .field("X20", &self.X20)
            .field("X21", &self.X21)
            .field("X22", &self.X22)
            .field("X23", &self.X23)
            .field("X24", &self.X24)
            .field("X25", &self.X25)
            .field("X26", &self.X26)
            .field("X27", &self.X27)
            .field("X28", &self.X28)
            .field("Fp", &self.Fp)
            .field("Lr", &self.Lr)
            .field("D8", &self.D8)
            .field("D9", &self.D9)
            .field("D10", &self.D10)
            .field("D11", &self.D11)
            .field("D12", &self.D12)
            .field("D13", &self.D13)
            .field("D14", &self.D14)
            .field("D15", &self.D15)
            .finish()
    }
}
#[cfg(target_arch = "aarch64")]
impl ::windows::core::TypeKind for KNONVOLATILE_CONTEXT_POINTERS_ARM64 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "aarch64")]
impl ::core::cmp::PartialEq for KNONVOLATILE_CONTEXT_POINTERS_ARM64 {
    fn eq(&self, other: &Self) -> bool {
        self.X19 == other.X19 && self.X20 == other.X20 && self.X21 == other.X21 && self.X22 == other.X22 && self.X23 == other.X23 && self.X24 == other.X24 && self.X25 == other.X25 && self.X26 == other.X26 && self.X27 == other.X27 && self.X28 == other.X28 && self.Fp == other.Fp && self.Lr == other.Lr && self.D8 == other.D8 && self.D9 == other.D9 && self.D10 == other.D10 && self.D11 == other.D11 && self.D12 == other.D12 && self.D13 == other.D13 && self.D14 == other.D14 && self.D15 == other.D15
    }
}
#[cfg(target_arch = "aarch64")]
impl ::core::cmp::Eq for KNONVOLATILE_CONTEXT_POINTERS_ARM64 {}
#[cfg(target_arch = "aarch64")]
impl ::core::default::Default for KNONVOLATILE_CONTEXT_POINTERS_ARM64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct LDT_ENTRY {
    pub LimitLow: u16,
    pub BaseLow: u16,
    pub HighWord: LDT_ENTRY_0,
}
impl ::core::marker::Copy for LDT_ENTRY {}
impl ::core::clone::Clone for LDT_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for LDT_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for LDT_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub union LDT_ENTRY_0 {
    pub Bytes: LDT_ENTRY_0_1,
    pub Bits: LDT_ENTRY_0_0,
}
impl ::core::marker::Copy for LDT_ENTRY_0 {}
impl ::core::clone::Clone for LDT_ENTRY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for LDT_ENTRY_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for LDT_ENTRY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct LDT_ENTRY_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for LDT_ENTRY_0_0 {}
impl ::core::clone::Clone for LDT_ENTRY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LDT_ENTRY_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDT_ENTRY_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows::core::TypeKind for LDT_ENTRY_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for LDT_ENTRY_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for LDT_ENTRY_0_0 {}
impl ::core::default::Default for LDT_ENTRY_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct LDT_ENTRY_0_1 {
    pub BaseMid: u8,
    pub Flags1: u8,
    pub Flags2: u8,
    pub BaseHi: u8,
}
impl ::core::marker::Copy for LDT_ENTRY_0_1 {}
impl ::core::clone::Clone for LDT_ENTRY_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LDT_ENTRY_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDT_ENTRY_0_1").field("BaseMid", &self.BaseMid).field("Flags1", &self.Flags1).field("Flags2", &self.Flags2).field("BaseHi", &self.BaseHi).finish()
    }
}
impl ::windows::core::TypeKind for LDT_ENTRY_0_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for LDT_ENTRY_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.BaseMid == other.BaseMid && self.Flags1 == other.Flags1 && self.Flags2 == other.Flags2 && self.BaseHi == other.BaseHi
    }
}
impl ::core::cmp::Eq for LDT_ENTRY_0_1 {}
impl ::core::default::Default for LDT_ENTRY_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
pub struct LOADED_IMAGE {
    pub ModuleName: ::windows::core::PSTR,
    pub hFile: super::super::super::Foundation::HANDLE,
    pub MappedAddress: *mut u8,
    pub FileHeader: *mut IMAGE_NT_HEADERS64,
    pub LastRvaSection: *mut IMAGE_SECTION_HEADER,
    pub NumberOfSections: u32,
    pub Sections: *mut IMAGE_SECTION_HEADER,
    pub Characteristics: IMAGE_FILE_CHARACTERISTICS2,
    pub fSystemImage: super::super::super::Foundation::BOOLEAN,
    pub fDOSImage: super::super::super::Foundation::BOOLEAN,
    pub fReadOnly: super::super::super::Foundation::BOOLEAN,
    pub Version: u8,
    pub Links: super::super::Kernel::LIST_ENTRY,
    pub SizeOfImage: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
impl ::core::marker::Copy for LOADED_IMAGE {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
impl ::core::clone::Clone for LOADED_IMAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
impl ::core::fmt::Debug for LOADED_IMAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOADED_IMAGE")
            .field("ModuleName", &self.ModuleName)
            .field("hFile", &self.hFile)
            .field("MappedAddress", &self.MappedAddress)
            .field("FileHeader", &self.FileHeader)
            .field("LastRvaSection", &self.LastRvaSection)
            .field("NumberOfSections", &self.NumberOfSections)
            .field("Sections", &self.Sections)
            .field("Characteristics", &self.Characteristics)
            .field("fSystemImage", &self.fSystemImage)
            .field("fDOSImage", &self.fDOSImage)
            .field("fReadOnly", &self.fReadOnly)
            .field("Version", &self.Version)
            .field("Links", &self.Links)
            .field("SizeOfImage", &self.SizeOfImage)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
impl ::windows::core::TypeKind for LOADED_IMAGE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
impl ::core::cmp::PartialEq for LOADED_IMAGE {
    fn eq(&self, other: &Self) -> bool {
        self.ModuleName == other.ModuleName && self.hFile == other.hFile && self.MappedAddress == other.MappedAddress && self.FileHeader == other.FileHeader && self.LastRvaSection == other.LastRvaSection && self.NumberOfSections == other.NumberOfSections && self.Sections == other.Sections && self.Characteristics == other.Characteristics && self.fSystemImage == other.fSystemImage && self.fDOSImage == other.fDOSImage && self.fReadOnly == other.fReadOnly && self.Version == other.Version && self.Links == other.Links && self.SizeOfImage == other.SizeOfImage
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
impl ::core::cmp::Eq for LOADED_IMAGE {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
impl ::core::default::Default for LOADED_IMAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
pub struct LOADED_IMAGE {
    pub ModuleName: ::windows::core::PSTR,
    pub hFile: super::super::super::Foundation::HANDLE,
    pub MappedAddress: *mut u8,
    pub FileHeader: *mut IMAGE_NT_HEADERS32,
    pub LastRvaSection: *mut IMAGE_SECTION_HEADER,
    pub NumberOfSections: u32,
    pub Sections: *mut IMAGE_SECTION_HEADER,
    pub Characteristics: IMAGE_FILE_CHARACTERISTICS2,
    pub fSystemImage: super::super::super::Foundation::BOOLEAN,
    pub fDOSImage: super::super::super::Foundation::BOOLEAN,
    pub fReadOnly: super::super::super::Foundation::BOOLEAN,
    pub Version: u8,
    pub Links: super::super::Kernel::LIST_ENTRY,
    pub SizeOfImage: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
impl ::core::marker::Copy for LOADED_IMAGE {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
impl ::core::clone::Clone for LOADED_IMAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
impl ::core::fmt::Debug for LOADED_IMAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOADED_IMAGE")
            .field("ModuleName", &self.ModuleName)
            .field("hFile", &self.hFile)
            .field("MappedAddress", &self.MappedAddress)
            .field("FileHeader", &self.FileHeader)
            .field("LastRvaSection", &self.LastRvaSection)
            .field("NumberOfSections", &self.NumberOfSections)
            .field("Sections", &self.Sections)
            .field("Characteristics", &self.Characteristics)
            .field("fSystemImage", &self.fSystemImage)
            .field("fDOSImage", &self.fDOSImage)
            .field("fReadOnly", &self.fReadOnly)
            .field("Version", &self.Version)
            .field("Links", &self.Links)
            .field("SizeOfImage", &self.SizeOfImage)
            .finish()
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
impl ::windows::core::TypeKind for LOADED_IMAGE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
impl ::core::cmp::PartialEq for LOADED_IMAGE {
    fn eq(&self, other: &Self) -> bool {
        self.ModuleName == other.ModuleName && self.hFile == other.hFile && self.MappedAddress == other.MappedAddress && self.FileHeader == other.FileHeader && self.LastRvaSection == other.LastRvaSection && self.NumberOfSections == other.NumberOfSections && self.Sections == other.Sections && self.Characteristics == other.Characteristics && self.fSystemImage == other.fSystemImage && self.fDOSImage == other.fDOSImage && self.fReadOnly == other.fReadOnly && self.Version == other.Version && self.Links == other.Links && self.SizeOfImage == other.SizeOfImage
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
impl ::core::cmp::Eq for LOADED_IMAGE {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
impl ::core::default::Default for LOADED_IMAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LOAD_DLL_DEBUG_INFO {
    pub hFile: super::super::super::Foundation::HANDLE,
    pub lpBaseOfDll: *mut ::core::ffi::c_void,
    pub dwDebugInfoFileOffset: u32,
    pub nDebugInfoSize: u32,
    pub lpImageName: *mut ::core::ffi::c_void,
    pub fUnicode: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LOAD_DLL_DEBUG_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LOAD_DLL_DEBUG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LOAD_DLL_DEBUG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOAD_DLL_DEBUG_INFO").field("hFile", &self.hFile).field("lpBaseOfDll", &self.lpBaseOfDll).field("dwDebugInfoFileOffset", &self.dwDebugInfoFileOffset).field("nDebugInfoSize", &self.nDebugInfoSize).field("lpImageName", &self.lpImageName).field("fUnicode", &self.fUnicode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for LOAD_DLL_DEBUG_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LOAD_DLL_DEBUG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.hFile == other.hFile && self.lpBaseOfDll == other.lpBaseOfDll && self.dwDebugInfoFileOffset == other.dwDebugInfoFileOffset && self.nDebugInfoSize == other.nDebugInfoSize && self.lpImageName == other.lpImageName && self.fUnicode == other.fUnicode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LOAD_DLL_DEBUG_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LOAD_DLL_DEBUG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct M128A {
    pub Low: u64,
    pub High: i64,
}
impl ::core::marker::Copy for M128A {}
impl ::core::clone::Clone for M128A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for M128A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("M128A").field("Low", &self.Low).field("High", &self.High).finish()
    }
}
impl ::windows::core::TypeKind for M128A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for M128A {
    fn eq(&self, other: &Self) -> bool {
        self.Low == other.Low && self.High == other.High
    }
}
impl ::core::cmp::Eq for M128A {}
impl ::core::default::Default for M128A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_FileSystem\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Memory\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_Kernel", feature = "Win32_System_Memory"))]
pub struct MINIDUMP_CALLBACK_INFORMATION {
    pub CallbackRoutine: MINIDUMP_CALLBACK_ROUTINE,
    pub CallbackParam: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_Kernel", feature = "Win32_System_Memory"))]
impl ::core::marker::Copy for MINIDUMP_CALLBACK_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_Kernel", feature = "Win32_System_Memory"))]
impl ::core::clone::Clone for MINIDUMP_CALLBACK_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_Kernel", feature = "Win32_System_Memory"))]
impl ::windows::core::TypeKind for MINIDUMP_CALLBACK_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_Kernel", feature = "Win32_System_Memory"))]
impl ::core::default::Default for MINIDUMP_CALLBACK_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_FileSystem\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_Kernel"))]
pub struct MINIDUMP_CALLBACK_INPUT {
    pub ProcessId: u32,
    pub ProcessHandle: super::super::super::Foundation::HANDLE,
    pub CallbackType: u32,
    pub Anonymous: MINIDUMP_CALLBACK_INPUT_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for MINIDUMP_CALLBACK_INPUT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for MINIDUMP_CALLBACK_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_Kernel"))]
impl ::windows::core::TypeKind for MINIDUMP_CALLBACK_INPUT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for MINIDUMP_CALLBACK_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_FileSystem\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_Kernel"))]
pub union MINIDUMP_CALLBACK_INPUT_0 {
    pub Status: ::windows::core::HRESULT,
    pub Thread: MINIDUMP_THREAD_CALLBACK,
    pub ThreadEx: MINIDUMP_THREAD_EX_CALLBACK,
    pub Module: MINIDUMP_MODULE_CALLBACK,
    pub IncludeThread: MINIDUMP_INCLUDE_THREAD_CALLBACK,
    pub IncludeModule: MINIDUMP_INCLUDE_MODULE_CALLBACK,
    pub Io: MINIDUMP_IO_CALLBACK,
    pub ReadMemoryFailure: MINIDUMP_READ_MEMORY_FAILURE_CALLBACK,
    pub SecondaryFlags: u32,
    pub VmQuery: MINIDUMP_VM_QUERY_CALLBACK,
    pub VmPreRead: MINIDUMP_VM_PRE_READ_CALLBACK,
    pub VmPostRead: MINIDUMP_VM_POST_READ_CALLBACK,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for MINIDUMP_CALLBACK_INPUT_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for MINIDUMP_CALLBACK_INPUT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_Kernel"))]
impl ::windows::core::TypeKind for MINIDUMP_CALLBACK_INPUT_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for MINIDUMP_CALLBACK_INPUT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Memory\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
pub struct MINIDUMP_CALLBACK_OUTPUT {
    pub Anonymous: MINIDUMP_CALLBACK_OUTPUT_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::marker::Copy for MINIDUMP_CALLBACK_OUTPUT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::clone::Clone for MINIDUMP_CALLBACK_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::windows::core::TypeKind for MINIDUMP_CALLBACK_OUTPUT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::default::Default for MINIDUMP_CALLBACK_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Memory\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
pub union MINIDUMP_CALLBACK_OUTPUT_0 {
    pub ModuleWriteFlags: u32,
    pub ThreadWriteFlags: u32,
    pub SecondaryFlags: u32,
    pub Anonymous1: MINIDUMP_CALLBACK_OUTPUT_0_0,
    pub Anonymous2: MINIDUMP_CALLBACK_OUTPUT_0_1,
    pub Handle: super::super::super::Foundation::HANDLE,
    pub Anonymous3: MINIDUMP_CALLBACK_OUTPUT_0_2,
    pub Anonymous4: MINIDUMP_CALLBACK_OUTPUT_0_3,
    pub Anonymous5: MINIDUMP_CALLBACK_OUTPUT_0_4,
    pub Status: ::windows::core::HRESULT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::marker::Copy for MINIDUMP_CALLBACK_OUTPUT_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::clone::Clone for MINIDUMP_CALLBACK_OUTPUT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::windows::core::TypeKind for MINIDUMP_CALLBACK_OUTPUT_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::default::Default for MINIDUMP_CALLBACK_OUTPUT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Memory\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
pub struct MINIDUMP_CALLBACK_OUTPUT_0_0 {
    pub MemoryBase: u64,
    pub MemorySize: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::marker::Copy for MINIDUMP_CALLBACK_OUTPUT_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::clone::Clone for MINIDUMP_CALLBACK_OUTPUT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::windows::core::TypeKind for MINIDUMP_CALLBACK_OUTPUT_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::default::Default for MINIDUMP_CALLBACK_OUTPUT_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Memory\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
pub struct MINIDUMP_CALLBACK_OUTPUT_0_1 {
    pub CheckCancel: super::super::super::Foundation::BOOL,
    pub Cancel: super::super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::marker::Copy for MINIDUMP_CALLBACK_OUTPUT_0_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::clone::Clone for MINIDUMP_CALLBACK_OUTPUT_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::fmt::Debug for MINIDUMP_CALLBACK_OUTPUT_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MINIDUMP_CALLBACK_OUTPUT_0_1").field("CheckCancel", &self.CheckCancel).field("Cancel", &self.Cancel).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::windows::core::TypeKind for MINIDUMP_CALLBACK_OUTPUT_0_1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::cmp::PartialEq for MINIDUMP_CALLBACK_OUTPUT_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.CheckCancel == other.CheckCancel && self.Cancel == other.Cancel
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::cmp::Eq for MINIDUMP_CALLBACK_OUTPUT_0_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::default::Default for MINIDUMP_CALLBACK_OUTPUT_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Memory\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
pub struct MINIDUMP_CALLBACK_OUTPUT_0_2 {
    pub VmRegion: MINIDUMP_MEMORY_INFO,
    pub Continue: super::super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::marker::Copy for MINIDUMP_CALLBACK_OUTPUT_0_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::clone::Clone for MINIDUMP_CALLBACK_OUTPUT_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::windows::core::TypeKind for MINIDUMP_CALLBACK_OUTPUT_0_2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::default::Default for MINIDUMP_CALLBACK_OUTPUT_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Memory\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
pub struct MINIDUMP_CALLBACK_OUTPUT_0_3 {
    pub VmQueryStatus: ::windows::core::HRESULT,
    pub VmQueryResult: MINIDUMP_MEMORY_INFO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::marker::Copy for MINIDUMP_CALLBACK_OUTPUT_0_3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::clone::Clone for MINIDUMP_CALLBACK_OUTPUT_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::windows::core::TypeKind for MINIDUMP_CALLBACK_OUTPUT_0_3 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::default::Default for MINIDUMP_CALLBACK_OUTPUT_0_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Memory\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
pub struct MINIDUMP_CALLBACK_OUTPUT_0_4 {
    pub VmReadStatus: ::windows::core::HRESULT,
    pub VmReadBytesCompleted: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::marker::Copy for MINIDUMP_CALLBACK_OUTPUT_0_4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::clone::Clone for MINIDUMP_CALLBACK_OUTPUT_0_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::fmt::Debug for MINIDUMP_CALLBACK_OUTPUT_0_4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MINIDUMP_CALLBACK_OUTPUT_0_4").field("VmReadStatus", &self.VmReadStatus).field("VmReadBytesCompleted", &self.VmReadBytesCompleted).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::windows::core::TypeKind for MINIDUMP_CALLBACK_OUTPUT_0_4 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::cmp::PartialEq for MINIDUMP_CALLBACK_OUTPUT_0_4 {
    fn eq(&self, other: &Self) -> bool {
        self.VmReadStatus == other.VmReadStatus && self.VmReadBytesCompleted == other.VmReadBytesCompleted
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::cmp::Eq for MINIDUMP_CALLBACK_OUTPUT_0_4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::default::Default for MINIDUMP_CALLBACK_OUTPUT_0_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_DIRECTORY {
    pub StreamType: u32,
    pub Location: MINIDUMP_LOCATION_DESCRIPTOR,
}
impl ::core::marker::Copy for MINIDUMP_DIRECTORY {}
impl ::core::clone::Clone for MINIDUMP_DIRECTORY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_DIRECTORY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_DIRECTORY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_EXCEPTION {
    pub ExceptionCode: u32,
    pub ExceptionFlags: u32,
    pub ExceptionRecord: u64,
    pub ExceptionAddress: u64,
    pub NumberParameters: u32,
    pub __unusedAlignment: u32,
    pub ExceptionInformation: [u64; 15],
}
impl ::core::marker::Copy for MINIDUMP_EXCEPTION {}
impl ::core::clone::Clone for MINIDUMP_EXCEPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_EXCEPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_EXCEPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct MINIDUMP_EXCEPTION_INFORMATION {
    pub ThreadId: u32,
    pub ExceptionPointers: *mut EXCEPTION_POINTERS,
    pub ClientPointers: super::super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for MINIDUMP_EXCEPTION_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for MINIDUMP_EXCEPTION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::windows::core::TypeKind for MINIDUMP_EXCEPTION_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for MINIDUMP_EXCEPTION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MINIDUMP_EXCEPTION_INFORMATION64 {
    pub ThreadId: u32,
    pub ExceptionRecord: u64,
    pub ContextRecord: u64,
    pub ClientPointers: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MINIDUMP_EXCEPTION_INFORMATION64 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MINIDUMP_EXCEPTION_INFORMATION64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for MINIDUMP_EXCEPTION_INFORMATION64 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MINIDUMP_EXCEPTION_INFORMATION64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_EXCEPTION_STREAM {
    pub ThreadId: u32,
    pub __alignment: u32,
    pub ExceptionRecord: MINIDUMP_EXCEPTION,
    pub ThreadContext: MINIDUMP_LOCATION_DESCRIPTOR,
}
impl ::core::marker::Copy for MINIDUMP_EXCEPTION_STREAM {}
impl ::core::clone::Clone for MINIDUMP_EXCEPTION_STREAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_EXCEPTION_STREAM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_EXCEPTION_STREAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_FUNCTION_TABLE_DESCRIPTOR {
    pub MinimumAddress: u64,
    pub MaximumAddress: u64,
    pub BaseAddress: u64,
    pub EntryCount: u32,
    pub SizeOfAlignPad: u32,
}
impl ::core::marker::Copy for MINIDUMP_FUNCTION_TABLE_DESCRIPTOR {}
impl ::core::clone::Clone for MINIDUMP_FUNCTION_TABLE_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_FUNCTION_TABLE_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_FUNCTION_TABLE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_FUNCTION_TABLE_STREAM {
    pub SizeOfHeader: u32,
    pub SizeOfDescriptor: u32,
    pub SizeOfNativeDescriptor: u32,
    pub SizeOfFunctionEntry: u32,
    pub NumberOfDescriptors: u32,
    pub SizeOfAlignPad: u32,
}
impl ::core::marker::Copy for MINIDUMP_FUNCTION_TABLE_STREAM {}
impl ::core::clone::Clone for MINIDUMP_FUNCTION_TABLE_STREAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_FUNCTION_TABLE_STREAM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_FUNCTION_TABLE_STREAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_HANDLE_DATA_STREAM {
    pub SizeOfHeader: u32,
    pub SizeOfDescriptor: u32,
    pub NumberOfDescriptors: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for MINIDUMP_HANDLE_DATA_STREAM {}
impl ::core::clone::Clone for MINIDUMP_HANDLE_DATA_STREAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_HANDLE_DATA_STREAM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_HANDLE_DATA_STREAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_HANDLE_DESCRIPTOR {
    pub Handle: u64,
    pub TypeNameRva: u32,
    pub ObjectNameRva: u32,
    pub Attributes: u32,
    pub GrantedAccess: u32,
    pub HandleCount: u32,
    pub PointerCount: u32,
}
impl ::core::marker::Copy for MINIDUMP_HANDLE_DESCRIPTOR {}
impl ::core::clone::Clone for MINIDUMP_HANDLE_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_HANDLE_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_HANDLE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_HANDLE_DESCRIPTOR_2 {
    pub Handle: u64,
    pub TypeNameRva: u32,
    pub ObjectNameRva: u32,
    pub Attributes: u32,
    pub GrantedAccess: u32,
    pub HandleCount: u32,
    pub PointerCount: u32,
    pub ObjectInfoRva: u32,
    pub Reserved0: u32,
}
impl ::core::marker::Copy for MINIDUMP_HANDLE_DESCRIPTOR_2 {}
impl ::core::clone::Clone for MINIDUMP_HANDLE_DESCRIPTOR_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_HANDLE_DESCRIPTOR_2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_HANDLE_DESCRIPTOR_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_HANDLE_OBJECT_INFORMATION {
    pub NextInfoRva: u32,
    pub InfoType: u32,
    pub SizeOfInfo: u32,
}
impl ::core::marker::Copy for MINIDUMP_HANDLE_OBJECT_INFORMATION {}
impl ::core::clone::Clone for MINIDUMP_HANDLE_OBJECT_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_HANDLE_OBJECT_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_HANDLE_OBJECT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_HANDLE_OPERATION_LIST {
    pub SizeOfHeader: u32,
    pub SizeOfEntry: u32,
    pub NumberOfEntries: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for MINIDUMP_HANDLE_OPERATION_LIST {}
impl ::core::clone::Clone for MINIDUMP_HANDLE_OPERATION_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_HANDLE_OPERATION_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_HANDLE_OPERATION_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_HEADER {
    pub Signature: u32,
    pub Version: u32,
    pub NumberOfStreams: u32,
    pub StreamDirectoryRva: u32,
    pub CheckSum: u32,
    pub Anonymous: MINIDUMP_HEADER_0,
    pub Flags: u64,
}
impl ::core::marker::Copy for MINIDUMP_HEADER {}
impl ::core::clone::Clone for MINIDUMP_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_HEADER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub union MINIDUMP_HEADER_0 {
    pub Reserved: u32,
    pub TimeDateStamp: u32,
}
impl ::core::marker::Copy for MINIDUMP_HEADER_0 {}
impl ::core::clone::Clone for MINIDUMP_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_HEADER_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_HEADER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_INCLUDE_MODULE_CALLBACK {
    pub BaseOfImage: u64,
}
impl ::core::marker::Copy for MINIDUMP_INCLUDE_MODULE_CALLBACK {}
impl ::core::clone::Clone for MINIDUMP_INCLUDE_MODULE_CALLBACK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_INCLUDE_MODULE_CALLBACK {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_INCLUDE_MODULE_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_INCLUDE_THREAD_CALLBACK {
    pub ThreadId: u32,
}
impl ::core::marker::Copy for MINIDUMP_INCLUDE_THREAD_CALLBACK {}
impl ::core::clone::Clone for MINIDUMP_INCLUDE_THREAD_CALLBACK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_INCLUDE_THREAD_CALLBACK {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_INCLUDE_THREAD_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MINIDUMP_IO_CALLBACK {
    pub Handle: super::super::super::Foundation::HANDLE,
    pub Offset: u64,
    pub Buffer: *mut ::core::ffi::c_void,
    pub BufferBytes: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MINIDUMP_IO_CALLBACK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MINIDUMP_IO_CALLBACK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for MINIDUMP_IO_CALLBACK {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MINIDUMP_IO_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_LOCATION_DESCRIPTOR {
    pub DataSize: u32,
    pub Rva: u32,
}
impl ::core::marker::Copy for MINIDUMP_LOCATION_DESCRIPTOR {}
impl ::core::clone::Clone for MINIDUMP_LOCATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_LOCATION_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_LOCATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_LOCATION_DESCRIPTOR64 {
    pub DataSize: u64,
    pub Rva: u64,
}
impl ::core::marker::Copy for MINIDUMP_LOCATION_DESCRIPTOR64 {}
impl ::core::clone::Clone for MINIDUMP_LOCATION_DESCRIPTOR64 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_LOCATION_DESCRIPTOR64 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_LOCATION_DESCRIPTOR64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_MEMORY64_LIST {
    pub NumberOfMemoryRanges: u64,
    pub BaseRva: u64,
    pub MemoryRanges: [MINIDUMP_MEMORY_DESCRIPTOR64; 1],
}
impl ::core::marker::Copy for MINIDUMP_MEMORY64_LIST {}
impl ::core::clone::Clone for MINIDUMP_MEMORY64_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_MEMORY64_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_MEMORY64_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_MEMORY_DESCRIPTOR {
    pub StartOfMemoryRange: u64,
    pub Memory: MINIDUMP_LOCATION_DESCRIPTOR,
}
impl ::core::marker::Copy for MINIDUMP_MEMORY_DESCRIPTOR {}
impl ::core::clone::Clone for MINIDUMP_MEMORY_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_MEMORY_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_MEMORY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_MEMORY_DESCRIPTOR64 {
    pub StartOfMemoryRange: u64,
    pub DataSize: u64,
}
impl ::core::marker::Copy for MINIDUMP_MEMORY_DESCRIPTOR64 {}
impl ::core::clone::Clone for MINIDUMP_MEMORY_DESCRIPTOR64 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_MEMORY_DESCRIPTOR64 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_MEMORY_DESCRIPTOR64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_Memory\"`*"]
#[cfg(feature = "Win32_System_Memory")]
pub struct MINIDUMP_MEMORY_INFO {
    pub BaseAddress: u64,
    pub AllocationBase: u64,
    pub AllocationProtect: u32,
    pub __alignment1: u32,
    pub RegionSize: u64,
    pub State: super::super::Memory::VIRTUAL_ALLOCATION_TYPE,
    pub Protect: u32,
    pub Type: u32,
    pub __alignment2: u32,
}
#[cfg(feature = "Win32_System_Memory")]
impl ::core::marker::Copy for MINIDUMP_MEMORY_INFO {}
#[cfg(feature = "Win32_System_Memory")]
impl ::core::clone::Clone for MINIDUMP_MEMORY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Memory")]
impl ::windows::core::TypeKind for MINIDUMP_MEMORY_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Memory")]
impl ::core::default::Default for MINIDUMP_MEMORY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_MEMORY_INFO_LIST {
    pub SizeOfHeader: u32,
    pub SizeOfEntry: u32,
    pub NumberOfEntries: u64,
}
impl ::core::marker::Copy for MINIDUMP_MEMORY_INFO_LIST {}
impl ::core::clone::Clone for MINIDUMP_MEMORY_INFO_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_MEMORY_INFO_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_MEMORY_INFO_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_MEMORY_LIST {
    pub NumberOfMemoryRanges: u32,
    pub MemoryRanges: [MINIDUMP_MEMORY_DESCRIPTOR; 1],
}
impl ::core::marker::Copy for MINIDUMP_MEMORY_LIST {}
impl ::core::clone::Clone for MINIDUMP_MEMORY_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_MEMORY_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_MEMORY_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_MISC_INFO {
    pub SizeOfInfo: u32,
    pub Flags1: MINIDUMP_MISC_INFO_FLAGS,
    pub ProcessId: u32,
    pub ProcessCreateTime: u32,
    pub ProcessUserTime: u32,
    pub ProcessKernelTime: u32,
}
impl ::core::marker::Copy for MINIDUMP_MISC_INFO {}
impl ::core::clone::Clone for MINIDUMP_MISC_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_MISC_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_MISC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_MISC_INFO_2 {
    pub SizeOfInfo: u32,
    pub Flags1: u32,
    pub ProcessId: u32,
    pub ProcessCreateTime: u32,
    pub ProcessUserTime: u32,
    pub ProcessKernelTime: u32,
    pub ProcessorMaxMhz: u32,
    pub ProcessorCurrentMhz: u32,
    pub ProcessorMhzLimit: u32,
    pub ProcessorMaxIdleState: u32,
    pub ProcessorCurrentIdleState: u32,
}
impl ::core::marker::Copy for MINIDUMP_MISC_INFO_2 {}
impl ::core::clone::Clone for MINIDUMP_MISC_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_MISC_INFO_2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_MISC_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub struct MINIDUMP_MISC_INFO_3 {
    pub SizeOfInfo: u32,
    pub Flags1: u32,
    pub ProcessId: u32,
    pub ProcessCreateTime: u32,
    pub ProcessUserTime: u32,
    pub ProcessKernelTime: u32,
    pub ProcessorMaxMhz: u32,
    pub ProcessorCurrentMhz: u32,
    pub ProcessorMhzLimit: u32,
    pub ProcessorMaxIdleState: u32,
    pub ProcessorCurrentIdleState: u32,
    pub ProcessIntegrityLevel: u32,
    pub ProcessExecuteFlags: u32,
    pub ProtectedProcess: u32,
    pub TimeZoneId: u32,
    pub TimeZone: super::super::Time::TIME_ZONE_INFORMATION,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for MINIDUMP_MISC_INFO_3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for MINIDUMP_MISC_INFO_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::windows::core::TypeKind for MINIDUMP_MISC_INFO_3 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for MINIDUMP_MISC_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub struct MINIDUMP_MISC_INFO_4 {
    pub SizeOfInfo: u32,
    pub Flags1: u32,
    pub ProcessId: u32,
    pub ProcessCreateTime: u32,
    pub ProcessUserTime: u32,
    pub ProcessKernelTime: u32,
    pub ProcessorMaxMhz: u32,
    pub ProcessorCurrentMhz: u32,
    pub ProcessorMhzLimit: u32,
    pub ProcessorMaxIdleState: u32,
    pub ProcessorCurrentIdleState: u32,
    pub ProcessIntegrityLevel: u32,
    pub ProcessExecuteFlags: u32,
    pub ProtectedProcess: u32,
    pub TimeZoneId: u32,
    pub TimeZone: super::super::Time::TIME_ZONE_INFORMATION,
    pub BuildString: [u16; 260],
    pub DbgBldStr: [u16; 40],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for MINIDUMP_MISC_INFO_4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for MINIDUMP_MISC_INFO_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::windows::core::TypeKind for MINIDUMP_MISC_INFO_4 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for MINIDUMP_MISC_INFO_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Time\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
pub struct MINIDUMP_MISC_INFO_5 {
    pub SizeOfInfo: u32,
    pub Flags1: u32,
    pub ProcessId: u32,
    pub ProcessCreateTime: u32,
    pub ProcessUserTime: u32,
    pub ProcessKernelTime: u32,
    pub ProcessorMaxMhz: u32,
    pub ProcessorCurrentMhz: u32,
    pub ProcessorMhzLimit: u32,
    pub ProcessorMaxIdleState: u32,
    pub ProcessorCurrentIdleState: u32,
    pub ProcessIntegrityLevel: u32,
    pub ProcessExecuteFlags: u32,
    pub ProtectedProcess: u32,
    pub TimeZoneId: u32,
    pub TimeZone: super::super::Time::TIME_ZONE_INFORMATION,
    pub BuildString: [u16; 260],
    pub DbgBldStr: [u16; 40],
    pub XStateData: XSTATE_CONFIG_FEATURE_MSC_INFO,
    pub ProcessCookie: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::marker::Copy for MINIDUMP_MISC_INFO_5 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::clone::Clone for MINIDUMP_MISC_INFO_5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::windows::core::TypeKind for MINIDUMP_MISC_INFO_5 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
impl ::core::default::Default for MINIDUMP_MISC_INFO_5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Storage_FileSystem\"`*"]
#[cfg(feature = "Win32_Storage_FileSystem")]
pub struct MINIDUMP_MODULE {
    pub BaseOfImage: u64,
    pub SizeOfImage: u32,
    pub CheckSum: u32,
    pub TimeDateStamp: u32,
    pub ModuleNameRva: u32,
    pub VersionInfo: super::super::super::Storage::FileSystem::VS_FIXEDFILEINFO,
    pub CvRecord: MINIDUMP_LOCATION_DESCRIPTOR,
    pub MiscRecord: MINIDUMP_LOCATION_DESCRIPTOR,
    pub Reserved0: u64,
    pub Reserved1: u64,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::marker::Copy for MINIDUMP_MODULE {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::clone::Clone for MINIDUMP_MODULE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::windows::core::TypeKind for MINIDUMP_MODULE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::default::Default for MINIDUMP_MODULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Storage_FileSystem\"`*"]
#[cfg(feature = "Win32_Storage_FileSystem")]
pub struct MINIDUMP_MODULE_CALLBACK {
    pub FullPath: ::windows::core::PWSTR,
    pub BaseOfImage: u64,
    pub SizeOfImage: u32,
    pub CheckSum: u32,
    pub TimeDateStamp: u32,
    pub VersionInfo: super::super::super::Storage::FileSystem::VS_FIXEDFILEINFO,
    pub CvRecord: *mut ::core::ffi::c_void,
    pub SizeOfCvRecord: u32,
    pub MiscRecord: *mut ::core::ffi::c_void,
    pub SizeOfMiscRecord: u32,
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::marker::Copy for MINIDUMP_MODULE_CALLBACK {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::clone::Clone for MINIDUMP_MODULE_CALLBACK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::windows::core::TypeKind for MINIDUMP_MODULE_CALLBACK {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::default::Default for MINIDUMP_MODULE_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Storage_FileSystem\"`*"]
#[cfg(feature = "Win32_Storage_FileSystem")]
pub struct MINIDUMP_MODULE_LIST {
    pub NumberOfModules: u32,
    pub Modules: [MINIDUMP_MODULE; 1],
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::marker::Copy for MINIDUMP_MODULE_LIST {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::clone::Clone for MINIDUMP_MODULE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::windows::core::TypeKind for MINIDUMP_MODULE_LIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::default::Default for MINIDUMP_MODULE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_PROCESS_VM_COUNTERS_1 {
    pub Revision: u16,
    pub PageFaultCount: u32,
    pub PeakWorkingSetSize: u64,
    pub WorkingSetSize: u64,
    pub QuotaPeakPagedPoolUsage: u64,
    pub QuotaPagedPoolUsage: u64,
    pub QuotaPeakNonPagedPoolUsage: u64,
    pub QuotaNonPagedPoolUsage: u64,
    pub PagefileUsage: u64,
    pub PeakPagefileUsage: u64,
    pub PrivateUsage: u64,
}
impl ::core::marker::Copy for MINIDUMP_PROCESS_VM_COUNTERS_1 {}
impl ::core::clone::Clone for MINIDUMP_PROCESS_VM_COUNTERS_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_PROCESS_VM_COUNTERS_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_PROCESS_VM_COUNTERS_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_PROCESS_VM_COUNTERS_2 {
    pub Revision: u16,
    pub Flags: u16,
    pub PageFaultCount: u32,
    pub PeakWorkingSetSize: u64,
    pub WorkingSetSize: u64,
    pub QuotaPeakPagedPoolUsage: u64,
    pub QuotaPagedPoolUsage: u64,
    pub QuotaPeakNonPagedPoolUsage: u64,
    pub QuotaNonPagedPoolUsage: u64,
    pub PagefileUsage: u64,
    pub PeakPagefileUsage: u64,
    pub PeakVirtualSize: u64,
    pub VirtualSize: u64,
    pub PrivateUsage: u64,
    pub PrivateWorkingSetSize: u64,
    pub SharedCommitUsage: u64,
    pub JobSharedCommitUsage: u64,
    pub JobPrivateCommitUsage: u64,
    pub JobPeakPrivateCommitUsage: u64,
    pub JobPrivateCommitLimit: u64,
    pub JobTotalCommitLimit: u64,
}
impl ::core::marker::Copy for MINIDUMP_PROCESS_VM_COUNTERS_2 {}
impl ::core::clone::Clone for MINIDUMP_PROCESS_VM_COUNTERS_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_PROCESS_VM_COUNTERS_2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_PROCESS_VM_COUNTERS_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_READ_MEMORY_FAILURE_CALLBACK {
    pub Offset: u64,
    pub Bytes: u32,
    pub FailureStatus: ::windows::core::HRESULT,
}
impl ::core::marker::Copy for MINIDUMP_READ_MEMORY_FAILURE_CALLBACK {}
impl ::core::clone::Clone for MINIDUMP_READ_MEMORY_FAILURE_CALLBACK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_READ_MEMORY_FAILURE_CALLBACK {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_READ_MEMORY_FAILURE_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_STRING {
    pub Length: u32,
    pub Buffer: [u16; 1],
}
impl ::core::marker::Copy for MINIDUMP_STRING {}
impl ::core::clone::Clone for MINIDUMP_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_STRING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_SYSTEM_BASIC_INFORMATION {
    pub TimerResolution: u32,
    pub PageSize: u32,
    pub NumberOfPhysicalPages: u32,
    pub LowestPhysicalPageNumber: u32,
    pub HighestPhysicalPageNumber: u32,
    pub AllocationGranularity: u32,
    pub MinimumUserModeAddress: u64,
    pub MaximumUserModeAddress: u64,
    pub ActiveProcessorsAffinityMask: u64,
    pub NumberOfProcessors: u32,
}
impl ::core::marker::Copy for MINIDUMP_SYSTEM_BASIC_INFORMATION {}
impl ::core::clone::Clone for MINIDUMP_SYSTEM_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_SYSTEM_BASIC_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_SYSTEM_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_SYSTEM_BASIC_PERFORMANCE_INFORMATION {
    pub AvailablePages: u64,
    pub CommittedPages: u64,
    pub CommitLimit: u64,
    pub PeakCommitment: u64,
}
impl ::core::marker::Copy for MINIDUMP_SYSTEM_BASIC_PERFORMANCE_INFORMATION {}
impl ::core::clone::Clone for MINIDUMP_SYSTEM_BASIC_PERFORMANCE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_SYSTEM_BASIC_PERFORMANCE_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_SYSTEM_BASIC_PERFORMANCE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_SYSTEM_FILECACHE_INFORMATION {
    pub CurrentSize: u64,
    pub PeakSize: u64,
    pub PageFaultCount: u32,
    pub MinimumWorkingSet: u64,
    pub MaximumWorkingSet: u64,
    pub CurrentSizeIncludingTransitionInPages: u64,
    pub PeakSizeIncludingTransitionInPages: u64,
    pub TransitionRePurposeCount: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for MINIDUMP_SYSTEM_FILECACHE_INFORMATION {}
impl ::core::clone::Clone for MINIDUMP_SYSTEM_FILECACHE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_SYSTEM_FILECACHE_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_SYSTEM_FILECACHE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_SYSTEM_INFO {
    pub ProcessorArchitecture: PROCESSOR_ARCHITECTURE,
    pub ProcessorLevel: u16,
    pub ProcessorRevision: u16,
    pub Anonymous1: MINIDUMP_SYSTEM_INFO_0,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub BuildNumber: u32,
    pub PlatformId: VER_PLATFORM,
    pub CSDVersionRva: u32,
    pub Anonymous2: MINIDUMP_SYSTEM_INFO_1,
    pub Cpu: CPU_INFORMATION,
}
impl ::core::marker::Copy for MINIDUMP_SYSTEM_INFO {}
impl ::core::clone::Clone for MINIDUMP_SYSTEM_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_SYSTEM_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_SYSTEM_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub union MINIDUMP_SYSTEM_INFO_0 {
    pub Reserved0: u16,
    pub Anonymous: MINIDUMP_SYSTEM_INFO_0_0,
}
impl ::core::marker::Copy for MINIDUMP_SYSTEM_INFO_0 {}
impl ::core::clone::Clone for MINIDUMP_SYSTEM_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_SYSTEM_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_SYSTEM_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_SYSTEM_INFO_0_0 {
    pub NumberOfProcessors: u8,
    pub ProductType: u8,
}
impl ::core::marker::Copy for MINIDUMP_SYSTEM_INFO_0_0 {}
impl ::core::clone::Clone for MINIDUMP_SYSTEM_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MINIDUMP_SYSTEM_INFO_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MINIDUMP_SYSTEM_INFO_0_0").field("NumberOfProcessors", &self.NumberOfProcessors).field("ProductType", &self.ProductType).finish()
    }
}
impl ::windows::core::TypeKind for MINIDUMP_SYSTEM_INFO_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MINIDUMP_SYSTEM_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfProcessors == other.NumberOfProcessors && self.ProductType == other.ProductType
    }
}
impl ::core::cmp::Eq for MINIDUMP_SYSTEM_INFO_0_0 {}
impl ::core::default::Default for MINIDUMP_SYSTEM_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub union MINIDUMP_SYSTEM_INFO_1 {
    pub Reserved1: u32,
    pub Anonymous: MINIDUMP_SYSTEM_INFO_1_0,
}
impl ::core::marker::Copy for MINIDUMP_SYSTEM_INFO_1 {}
impl ::core::clone::Clone for MINIDUMP_SYSTEM_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_SYSTEM_INFO_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_SYSTEM_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_SYSTEM_INFO_1_0 {
    pub SuiteMask: u16,
    pub Reserved2: u16,
}
impl ::core::marker::Copy for MINIDUMP_SYSTEM_INFO_1_0 {}
impl ::core::clone::Clone for MINIDUMP_SYSTEM_INFO_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MINIDUMP_SYSTEM_INFO_1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MINIDUMP_SYSTEM_INFO_1_0").field("SuiteMask", &self.SuiteMask).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::windows::core::TypeKind for MINIDUMP_SYSTEM_INFO_1_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MINIDUMP_SYSTEM_INFO_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.SuiteMask == other.SuiteMask && self.Reserved2 == other.Reserved2
    }
}
impl ::core::cmp::Eq for MINIDUMP_SYSTEM_INFO_1_0 {}
impl ::core::default::Default for MINIDUMP_SYSTEM_INFO_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_SYSTEM_MEMORY_INFO_1 {
    pub Revision: u16,
    pub Flags: u16,
    pub BasicInfo: MINIDUMP_SYSTEM_BASIC_INFORMATION,
    pub FileCacheInfo: MINIDUMP_SYSTEM_FILECACHE_INFORMATION,
    pub BasicPerfInfo: MINIDUMP_SYSTEM_BASIC_PERFORMANCE_INFORMATION,
    pub PerfInfo: MINIDUMP_SYSTEM_PERFORMANCE_INFORMATION,
}
impl ::core::marker::Copy for MINIDUMP_SYSTEM_MEMORY_INFO_1 {}
impl ::core::clone::Clone for MINIDUMP_SYSTEM_MEMORY_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_SYSTEM_MEMORY_INFO_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_SYSTEM_MEMORY_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_SYSTEM_PERFORMANCE_INFORMATION {
    pub IdleProcessTime: u64,
    pub IoReadTransferCount: u64,
    pub IoWriteTransferCount: u64,
    pub IoOtherTransferCount: u64,
    pub IoReadOperationCount: u32,
    pub IoWriteOperationCount: u32,
    pub IoOtherOperationCount: u32,
    pub AvailablePages: u32,
    pub CommittedPages: u32,
    pub CommitLimit: u32,
    pub PeakCommitment: u32,
    pub PageFaultCount: u32,
    pub CopyOnWriteCount: u32,
    pub TransitionCount: u32,
    pub CacheTransitionCount: u32,
    pub DemandZeroCount: u32,
    pub PageReadCount: u32,
    pub PageReadIoCount: u32,
    pub CacheReadCount: u32,
    pub CacheIoCount: u32,
    pub DirtyPagesWriteCount: u32,
    pub DirtyWriteIoCount: u32,
    pub MappedPagesWriteCount: u32,
    pub MappedWriteIoCount: u32,
    pub PagedPoolPages: u32,
    pub NonPagedPoolPages: u32,
    pub PagedPoolAllocs: u32,
    pub PagedPoolFrees: u32,
    pub NonPagedPoolAllocs: u32,
    pub NonPagedPoolFrees: u32,
    pub FreeSystemPtes: u32,
    pub ResidentSystemCodePage: u32,
    pub TotalSystemDriverPages: u32,
    pub TotalSystemCodePages: u32,
    pub NonPagedPoolLookasideHits: u32,
    pub PagedPoolLookasideHits: u32,
    pub AvailablePagedPoolPages: u32,
    pub ResidentSystemCachePage: u32,
    pub ResidentPagedPoolPage: u32,
    pub ResidentSystemDriverPage: u32,
    pub CcFastReadNoWait: u32,
    pub CcFastReadWait: u32,
    pub CcFastReadResourceMiss: u32,
    pub CcFastReadNotPossible: u32,
    pub CcFastMdlReadNoWait: u32,
    pub CcFastMdlReadWait: u32,
    pub CcFastMdlReadResourceMiss: u32,
    pub CcFastMdlReadNotPossible: u32,
    pub CcMapDataNoWait: u32,
    pub CcMapDataWait: u32,
    pub CcMapDataNoWaitMiss: u32,
    pub CcMapDataWaitMiss: u32,
    pub CcPinMappedDataCount: u32,
    pub CcPinReadNoWait: u32,
    pub CcPinReadWait: u32,
    pub CcPinReadNoWaitMiss: u32,
    pub CcPinReadWaitMiss: u32,
    pub CcCopyReadNoWait: u32,
    pub CcCopyReadWait: u32,
    pub CcCopyReadNoWaitMiss: u32,
    pub CcCopyReadWaitMiss: u32,
    pub CcMdlReadNoWait: u32,
    pub CcMdlReadWait: u32,
    pub CcMdlReadNoWaitMiss: u32,
    pub CcMdlReadWaitMiss: u32,
    pub CcReadAheadIos: u32,
    pub CcLazyWriteIos: u32,
    pub CcLazyWritePages: u32,
    pub CcDataFlushes: u32,
    pub CcDataPages: u32,
    pub ContextSwitches: u32,
    pub FirstLevelTbFills: u32,
    pub SecondLevelTbFills: u32,
    pub SystemCalls: u32,
    pub CcTotalDirtyPages: u64,
    pub CcDirtyPageThreshold: u64,
    pub ResidentAvailablePages: i64,
    pub SharedCommittedPages: u64,
}
impl ::core::marker::Copy for MINIDUMP_SYSTEM_PERFORMANCE_INFORMATION {}
impl ::core::clone::Clone for MINIDUMP_SYSTEM_PERFORMANCE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_SYSTEM_PERFORMANCE_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_SYSTEM_PERFORMANCE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_THREAD {
    pub ThreadId: u32,
    pub SuspendCount: u32,
    pub PriorityClass: u32,
    pub Priority: u32,
    pub Teb: u64,
    pub Stack: MINIDUMP_MEMORY_DESCRIPTOR,
    pub ThreadContext: MINIDUMP_LOCATION_DESCRIPTOR,
}
impl ::core::marker::Copy for MINIDUMP_THREAD {}
impl ::core::clone::Clone for MINIDUMP_THREAD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_THREAD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_THREAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct MINIDUMP_THREAD_CALLBACK {
    pub ThreadId: u32,
    pub ThreadHandle: super::super::super::Foundation::HANDLE,
    pub Pad: u32,
    pub Context: CONTEXT,
    pub SizeOfContext: u32,
    pub StackBase: u64,
    pub StackEnd: u64,
}
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for MINIDUMP_THREAD_CALLBACK {}
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for MINIDUMP_THREAD_CALLBACK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::windows::core::TypeKind for MINIDUMP_THREAD_CALLBACK {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for MINIDUMP_THREAD_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct MINIDUMP_THREAD_CALLBACK {
    pub ThreadId: u32,
    pub ThreadHandle: super::super::super::Foundation::HANDLE,
    pub Context: CONTEXT,
    pub SizeOfContext: u32,
    pub StackBase: u64,
    pub StackEnd: u64,
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for MINIDUMP_THREAD_CALLBACK {}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for MINIDUMP_THREAD_CALLBACK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::windows::core::TypeKind for MINIDUMP_THREAD_CALLBACK {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for MINIDUMP_THREAD_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_THREAD_EX {
    pub ThreadId: u32,
    pub SuspendCount: u32,
    pub PriorityClass: u32,
    pub Priority: u32,
    pub Teb: u64,
    pub Stack: MINIDUMP_MEMORY_DESCRIPTOR,
    pub ThreadContext: MINIDUMP_LOCATION_DESCRIPTOR,
    pub BackingStore: MINIDUMP_MEMORY_DESCRIPTOR,
}
impl ::core::marker::Copy for MINIDUMP_THREAD_EX {}
impl ::core::clone::Clone for MINIDUMP_THREAD_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_THREAD_EX {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_THREAD_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct MINIDUMP_THREAD_EX_CALLBACK {
    pub ThreadId: u32,
    pub ThreadHandle: super::super::super::Foundation::HANDLE,
    pub Pad: u32,
    pub Context: CONTEXT,
    pub SizeOfContext: u32,
    pub StackBase: u64,
    pub StackEnd: u64,
    pub BackingStoreBase: u64,
    pub BackingStoreEnd: u64,
}
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for MINIDUMP_THREAD_EX_CALLBACK {}
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for MINIDUMP_THREAD_EX_CALLBACK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::windows::core::TypeKind for MINIDUMP_THREAD_EX_CALLBACK {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for MINIDUMP_THREAD_EX_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct MINIDUMP_THREAD_EX_CALLBACK {
    pub ThreadId: u32,
    pub ThreadHandle: super::super::super::Foundation::HANDLE,
    pub Context: CONTEXT,
    pub SizeOfContext: u32,
    pub StackBase: u64,
    pub StackEnd: u64,
    pub BackingStoreBase: u64,
    pub BackingStoreEnd: u64,
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for MINIDUMP_THREAD_EX_CALLBACK {}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for MINIDUMP_THREAD_EX_CALLBACK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::windows::core::TypeKind for MINIDUMP_THREAD_EX_CALLBACK {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for MINIDUMP_THREAD_EX_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_THREAD_EX_LIST {
    pub NumberOfThreads: u32,
    pub Threads: [MINIDUMP_THREAD_EX; 1],
}
impl ::core::marker::Copy for MINIDUMP_THREAD_EX_LIST {}
impl ::core::clone::Clone for MINIDUMP_THREAD_EX_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_THREAD_EX_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_THREAD_EX_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_THREAD_INFO {
    pub ThreadId: u32,
    pub DumpFlags: MINIDUMP_THREAD_INFO_DUMP_FLAGS,
    pub DumpError: u32,
    pub ExitStatus: u32,
    pub CreateTime: u64,
    pub ExitTime: u64,
    pub KernelTime: u64,
    pub UserTime: u64,
    pub StartAddress: u64,
    pub Affinity: u64,
}
impl ::core::marker::Copy for MINIDUMP_THREAD_INFO {}
impl ::core::clone::Clone for MINIDUMP_THREAD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_THREAD_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_THREAD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_THREAD_INFO_LIST {
    pub SizeOfHeader: u32,
    pub SizeOfEntry: u32,
    pub NumberOfEntries: u32,
}
impl ::core::marker::Copy for MINIDUMP_THREAD_INFO_LIST {}
impl ::core::clone::Clone for MINIDUMP_THREAD_INFO_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_THREAD_INFO_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_THREAD_INFO_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_THREAD_LIST {
    pub NumberOfThreads: u32,
    pub Threads: [MINIDUMP_THREAD; 1],
}
impl ::core::marker::Copy for MINIDUMP_THREAD_LIST {}
impl ::core::clone::Clone for MINIDUMP_THREAD_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_THREAD_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_THREAD_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_THREAD_NAME {
    pub ThreadId: u32,
    pub RvaOfThreadName: u64,
}
impl ::core::marker::Copy for MINIDUMP_THREAD_NAME {}
impl ::core::clone::Clone for MINIDUMP_THREAD_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_THREAD_NAME {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_THREAD_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_THREAD_NAME_LIST {
    pub NumberOfThreadNames: u32,
    pub ThreadNames: [MINIDUMP_THREAD_NAME; 1],
}
impl ::core::marker::Copy for MINIDUMP_THREAD_NAME_LIST {}
impl ::core::clone::Clone for MINIDUMP_THREAD_NAME_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_THREAD_NAME_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_THREAD_NAME_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_TOKEN_INFO_HEADER {
    pub TokenSize: u32,
    pub TokenId: u32,
    pub TokenHandle: u64,
}
impl ::core::marker::Copy for MINIDUMP_TOKEN_INFO_HEADER {}
impl ::core::clone::Clone for MINIDUMP_TOKEN_INFO_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_TOKEN_INFO_HEADER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_TOKEN_INFO_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_TOKEN_INFO_LIST {
    pub TokenListSize: u32,
    pub TokenListEntries: u32,
    pub ListHeaderSize: u32,
    pub ElementHeaderSize: u32,
}
impl ::core::marker::Copy for MINIDUMP_TOKEN_INFO_LIST {}
impl ::core::clone::Clone for MINIDUMP_TOKEN_INFO_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_TOKEN_INFO_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_TOKEN_INFO_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_UNLOADED_MODULE {
    pub BaseOfImage: u64,
    pub SizeOfImage: u32,
    pub CheckSum: u32,
    pub TimeDateStamp: u32,
    pub ModuleNameRva: u32,
}
impl ::core::marker::Copy for MINIDUMP_UNLOADED_MODULE {}
impl ::core::clone::Clone for MINIDUMP_UNLOADED_MODULE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_UNLOADED_MODULE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_UNLOADED_MODULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_UNLOADED_MODULE_LIST {
    pub SizeOfHeader: u32,
    pub SizeOfEntry: u32,
    pub NumberOfEntries: u32,
}
impl ::core::marker::Copy for MINIDUMP_UNLOADED_MODULE_LIST {}
impl ::core::clone::Clone for MINIDUMP_UNLOADED_MODULE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_UNLOADED_MODULE_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_UNLOADED_MODULE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_USER_RECORD {
    pub Type: u32,
    pub Memory: MINIDUMP_LOCATION_DESCRIPTOR,
}
impl ::core::marker::Copy for MINIDUMP_USER_RECORD {}
impl ::core::clone::Clone for MINIDUMP_USER_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_USER_RECORD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_USER_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_USER_STREAM {
    pub Type: u32,
    pub BufferSize: u32,
    pub Buffer: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for MINIDUMP_USER_STREAM {}
impl ::core::clone::Clone for MINIDUMP_USER_STREAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_USER_STREAM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_USER_STREAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_USER_STREAM_INFORMATION {
    pub UserStreamCount: u32,
    pub UserStreamArray: *mut MINIDUMP_USER_STREAM,
}
impl ::core::marker::Copy for MINIDUMP_USER_STREAM_INFORMATION {}
impl ::core::clone::Clone for MINIDUMP_USER_STREAM_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_USER_STREAM_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_USER_STREAM_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_VM_POST_READ_CALLBACK {
    pub Offset: u64,
    pub Buffer: *mut ::core::ffi::c_void,
    pub Size: u32,
    pub Completed: u32,
    pub Status: ::windows::core::HRESULT,
}
impl ::core::marker::Copy for MINIDUMP_VM_POST_READ_CALLBACK {}
impl ::core::clone::Clone for MINIDUMP_VM_POST_READ_CALLBACK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_VM_POST_READ_CALLBACK {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_VM_POST_READ_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_VM_PRE_READ_CALLBACK {
    pub Offset: u64,
    pub Buffer: *mut ::core::ffi::c_void,
    pub Size: u32,
}
impl ::core::marker::Copy for MINIDUMP_VM_PRE_READ_CALLBACK {}
impl ::core::clone::Clone for MINIDUMP_VM_PRE_READ_CALLBACK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_VM_PRE_READ_CALLBACK {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_VM_PRE_READ_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MINIDUMP_VM_QUERY_CALLBACK {
    pub Offset: u64,
}
impl ::core::marker::Copy for MINIDUMP_VM_QUERY_CALLBACK {}
impl ::core::clone::Clone for MINIDUMP_VM_QUERY_CALLBACK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MINIDUMP_VM_QUERY_CALLBACK {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MINIDUMP_VM_QUERY_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MODLOAD_CVMISC {
    pub oCV: u32,
    pub cCV: usize,
    pub oMisc: u32,
    pub cMisc: usize,
    pub dtImage: u32,
    pub cImage: u32,
}
impl ::core::marker::Copy for MODLOAD_CVMISC {}
impl ::core::clone::Clone for MODLOAD_CVMISC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MODLOAD_CVMISC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MODLOAD_CVMISC").field("oCV", &self.oCV).field("cCV", &self.cCV).field("oMisc", &self.oMisc).field("cMisc", &self.cMisc).field("dtImage", &self.dtImage).field("cImage", &self.cImage).finish()
    }
}
impl ::windows::core::TypeKind for MODLOAD_CVMISC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MODLOAD_CVMISC {
    fn eq(&self, other: &Self) -> bool {
        self.oCV == other.oCV && self.cCV == other.cCV && self.oMisc == other.oMisc && self.cMisc == other.cMisc && self.dtImage == other.dtImage && self.cImage == other.cImage
    }
}
impl ::core::cmp::Eq for MODLOAD_CVMISC {}
impl ::core::default::Default for MODLOAD_CVMISC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MODLOAD_DATA {
    pub ssize: u32,
    pub ssig: MODLOAD_DATA_TYPE,
    pub data: *mut ::core::ffi::c_void,
    pub size: u32,
    pub flags: u32,
}
impl ::core::marker::Copy for MODLOAD_DATA {}
impl ::core::clone::Clone for MODLOAD_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MODLOAD_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MODLOAD_DATA").field("ssize", &self.ssize).field("ssig", &self.ssig).field("data", &self.data).field("size", &self.size).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MODLOAD_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MODLOAD_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.ssize == other.ssize && self.ssig == other.ssig && self.data == other.data && self.size == other.size && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MODLOAD_DATA {}
impl ::core::default::Default for MODLOAD_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MODLOAD_PDBGUID_PDBAGE {
    pub PdbGuid: ::windows::core::GUID,
    pub PdbAge: u32,
}
impl ::core::marker::Copy for MODLOAD_PDBGUID_PDBAGE {}
impl ::core::clone::Clone for MODLOAD_PDBGUID_PDBAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MODLOAD_PDBGUID_PDBAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MODLOAD_PDBGUID_PDBAGE").field("PdbGuid", &self.PdbGuid).field("PdbAge", &self.PdbAge).finish()
    }
}
impl ::windows::core::TypeKind for MODLOAD_PDBGUID_PDBAGE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MODLOAD_PDBGUID_PDBAGE {
    fn eq(&self, other: &Self) -> bool {
        self.PdbGuid == other.PdbGuid && self.PdbAge == other.PdbAge
    }
}
impl ::core::cmp::Eq for MODLOAD_PDBGUID_PDBAGE {}
impl ::core::default::Default for MODLOAD_PDBGUID_PDBAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct MODULE_TYPE_INFO {
    pub dataLength: u16,
    pub leaf: u16,
    pub data: [u8; 1],
}
impl ::core::marker::Copy for MODULE_TYPE_INFO {}
impl ::core::clone::Clone for MODULE_TYPE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MODULE_TYPE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MODULE_TYPE_INFO").field("dataLength", &self.dataLength).field("leaf", &self.leaf).field("data", &self.data).finish()
    }
}
impl ::windows::core::TypeKind for MODULE_TYPE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MODULE_TYPE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dataLength == other.dataLength && self.leaf == other.leaf && self.data == other.data
    }
}
impl ::core::cmp::Eq for MODULE_TYPE_INFO {}
impl ::core::default::Default for MODULE_TYPE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct OMAP {
    pub rva: u32,
    pub rvaTo: u32,
}
impl ::core::marker::Copy for OMAP {}
impl ::core::clone::Clone for OMAP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OMAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OMAP").field("rva", &self.rva).field("rvaTo", &self.rvaTo).finish()
    }
}
impl ::windows::core::TypeKind for OMAP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for OMAP {
    fn eq(&self, other: &Self) -> bool {
        self.rva == other.rva && self.rvaTo == other.rvaTo
    }
}
impl ::core::cmp::Eq for OMAP {}
impl ::core::default::Default for OMAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct OUTPUT_DEBUG_STRING_INFO {
    pub lpDebugStringData: ::windows::core::PSTR,
    pub fUnicode: u16,
    pub nDebugStringLength: u16,
}
impl ::core::marker::Copy for OUTPUT_DEBUG_STRING_INFO {}
impl ::core::clone::Clone for OUTPUT_DEBUG_STRING_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OUTPUT_DEBUG_STRING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OUTPUT_DEBUG_STRING_INFO").field("lpDebugStringData", &self.lpDebugStringData).field("fUnicode", &self.fUnicode).field("nDebugStringLength", &self.nDebugStringLength).finish()
    }
}
impl ::windows::core::TypeKind for OUTPUT_DEBUG_STRING_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for OUTPUT_DEBUG_STRING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpDebugStringData == other.lpDebugStringData && self.fUnicode == other.fUnicode && self.nDebugStringLength == other.nDebugStringLength
    }
}
impl ::core::cmp::Eq for OUTPUT_DEBUG_STRING_INFO {}
impl ::core::default::Default for OUTPUT_DEBUG_STRING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct PHYSICAL_MEMORY_DESCRIPTOR32 {
    pub NumberOfRuns: u32,
    pub NumberOfPages: u32,
    pub Run: [PHYSICAL_MEMORY_RUN32; 1],
}
impl ::core::marker::Copy for PHYSICAL_MEMORY_DESCRIPTOR32 {}
impl ::core::clone::Clone for PHYSICAL_MEMORY_DESCRIPTOR32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PHYSICAL_MEMORY_DESCRIPTOR32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PHYSICAL_MEMORY_DESCRIPTOR32").field("NumberOfRuns", &self.NumberOfRuns).field("NumberOfPages", &self.NumberOfPages).field("Run", &self.Run).finish()
    }
}
impl ::windows::core::TypeKind for PHYSICAL_MEMORY_DESCRIPTOR32 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PHYSICAL_MEMORY_DESCRIPTOR32 {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfRuns == other.NumberOfRuns && self.NumberOfPages == other.NumberOfPages && self.Run == other.Run
    }
}
impl ::core::cmp::Eq for PHYSICAL_MEMORY_DESCRIPTOR32 {}
impl ::core::default::Default for PHYSICAL_MEMORY_DESCRIPTOR32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct PHYSICAL_MEMORY_DESCRIPTOR64 {
    pub NumberOfRuns: u32,
    pub NumberOfPages: u64,
    pub Run: [PHYSICAL_MEMORY_RUN64; 1],
}
impl ::core::marker::Copy for PHYSICAL_MEMORY_DESCRIPTOR64 {}
impl ::core::clone::Clone for PHYSICAL_MEMORY_DESCRIPTOR64 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PHYSICAL_MEMORY_DESCRIPTOR64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PHYSICAL_MEMORY_DESCRIPTOR64").field("NumberOfRuns", &self.NumberOfRuns).field("NumberOfPages", &self.NumberOfPages).field("Run", &self.Run).finish()
    }
}
impl ::windows::core::TypeKind for PHYSICAL_MEMORY_DESCRIPTOR64 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PHYSICAL_MEMORY_DESCRIPTOR64 {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfRuns == other.NumberOfRuns && self.NumberOfPages == other.NumberOfPages && self.Run == other.Run
    }
}
impl ::core::cmp::Eq for PHYSICAL_MEMORY_DESCRIPTOR64 {}
impl ::core::default::Default for PHYSICAL_MEMORY_DESCRIPTOR64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct PHYSICAL_MEMORY_RUN32 {
    pub BasePage: u32,
    pub PageCount: u32,
}
impl ::core::marker::Copy for PHYSICAL_MEMORY_RUN32 {}
impl ::core::clone::Clone for PHYSICAL_MEMORY_RUN32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PHYSICAL_MEMORY_RUN32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PHYSICAL_MEMORY_RUN32").field("BasePage", &self.BasePage).field("PageCount", &self.PageCount).finish()
    }
}
impl ::windows::core::TypeKind for PHYSICAL_MEMORY_RUN32 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PHYSICAL_MEMORY_RUN32 {
    fn eq(&self, other: &Self) -> bool {
        self.BasePage == other.BasePage && self.PageCount == other.PageCount
    }
}
impl ::core::cmp::Eq for PHYSICAL_MEMORY_RUN32 {}
impl ::core::default::Default for PHYSICAL_MEMORY_RUN32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct PHYSICAL_MEMORY_RUN64 {
    pub BasePage: u64,
    pub PageCount: u64,
}
impl ::core::marker::Copy for PHYSICAL_MEMORY_RUN64 {}
impl ::core::clone::Clone for PHYSICAL_MEMORY_RUN64 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PHYSICAL_MEMORY_RUN64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PHYSICAL_MEMORY_RUN64").field("BasePage", &self.BasePage).field("PageCount", &self.PageCount).finish()
    }
}
impl ::windows::core::TypeKind for PHYSICAL_MEMORY_RUN64 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PHYSICAL_MEMORY_RUN64 {
    fn eq(&self, other: &Self) -> bool {
        self.BasePage == other.BasePage && self.PageCount == other.PageCount
    }
}
impl ::core::cmp::Eq for PHYSICAL_MEMORY_RUN64 {}
impl ::core::default::Default for PHYSICAL_MEMORY_RUN64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct RIP_INFO {
    pub dwError: u32,
    pub dwType: RIP_INFO_TYPE,
}
impl ::core::marker::Copy for RIP_INFO {}
impl ::core::clone::Clone for RIP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RIP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RIP_INFO").field("dwError", &self.dwError).field("dwType", &self.dwType).finish()
    }
}
impl ::windows::core::TypeKind for RIP_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RIP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwError == other.dwError && self.dwType == other.dwType
    }
}
impl ::core::cmp::Eq for RIP_INFO {}
impl ::core::default::Default for RIP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct SOURCEFILE {
    pub ModBase: u64,
    pub FileName: ::windows::core::PSTR,
}
impl ::core::marker::Copy for SOURCEFILE {}
impl ::core::clone::Clone for SOURCEFILE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOURCEFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOURCEFILE").field("ModBase", &self.ModBase).field("FileName", &self.FileName).finish()
    }
}
impl ::windows::core::TypeKind for SOURCEFILE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SOURCEFILE {
    fn eq(&self, other: &Self) -> bool {
        self.ModBase == other.ModBase && self.FileName == other.FileName
    }
}
impl ::core::cmp::Eq for SOURCEFILE {}
impl ::core::default::Default for SOURCEFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct SOURCEFILEW {
    pub ModBase: u64,
    pub FileName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for SOURCEFILEW {}
impl ::core::clone::Clone for SOURCEFILEW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOURCEFILEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOURCEFILEW").field("ModBase", &self.ModBase).field("FileName", &self.FileName).finish()
    }
}
impl ::windows::core::TypeKind for SOURCEFILEW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SOURCEFILEW {
    fn eq(&self, other: &Self) -> bool {
        self.ModBase == other.ModBase && self.FileName == other.FileName
    }
}
impl ::core::cmp::Eq for SOURCEFILEW {}
impl ::core::default::Default for SOURCEFILEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct SRCCODEINFO {
    pub SizeOfStruct: u32,
    pub Key: *mut ::core::ffi::c_void,
    pub ModBase: u64,
    pub Obj: [u8; 261],
    pub FileName: [u8; 261],
    pub LineNumber: u32,
    pub Address: u64,
}
impl ::core::marker::Copy for SRCCODEINFO {}
impl ::core::clone::Clone for SRCCODEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SRCCODEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SRCCODEINFO").field("SizeOfStruct", &self.SizeOfStruct).field("Key", &self.Key).field("ModBase", &self.ModBase).field("Obj", &self.Obj).field("FileName", &self.FileName).field("LineNumber", &self.LineNumber).field("Address", &self.Address).finish()
    }
}
impl ::windows::core::TypeKind for SRCCODEINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SRCCODEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.Key == other.Key && self.ModBase == other.ModBase && self.Obj == other.Obj && self.FileName == other.FileName && self.LineNumber == other.LineNumber && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for SRCCODEINFO {}
impl ::core::default::Default for SRCCODEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct SRCCODEINFOW {
    pub SizeOfStruct: u32,
    pub Key: *mut ::core::ffi::c_void,
    pub ModBase: u64,
    pub Obj: [u16; 261],
    pub FileName: [u16; 261],
    pub LineNumber: u32,
    pub Address: u64,
}
impl ::core::marker::Copy for SRCCODEINFOW {}
impl ::core::clone::Clone for SRCCODEINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SRCCODEINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SRCCODEINFOW").field("SizeOfStruct", &self.SizeOfStruct).field("Key", &self.Key).field("ModBase", &self.ModBase).field("Obj", &self.Obj).field("FileName", &self.FileName).field("LineNumber", &self.LineNumber).field("Address", &self.Address).finish()
    }
}
impl ::windows::core::TypeKind for SRCCODEINFOW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SRCCODEINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.Key == other.Key && self.ModBase == other.ModBase && self.Obj == other.Obj && self.FileName == other.FileName && self.LineNumber == other.LineNumber && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for SRCCODEINFOW {}
impl ::core::default::Default for SRCCODEINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct STACKFRAME {
    pub AddrPC: ADDRESS,
    pub AddrReturn: ADDRESS,
    pub AddrFrame: ADDRESS,
    pub AddrStack: ADDRESS,
    pub FuncTableEntry: *mut ::core::ffi::c_void,
    pub Params: [u32; 4],
    pub Far: super::super::super::Foundation::BOOL,
    pub Virtual: super::super::super::Foundation::BOOL,
    pub Reserved: [u32; 3],
    pub KdHelp: KDHELP,
    pub AddrBStore: ADDRESS,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STACKFRAME {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STACKFRAME {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STACKFRAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STACKFRAME").field("AddrPC", &self.AddrPC).field("AddrReturn", &self.AddrReturn).field("AddrFrame", &self.AddrFrame).field("AddrStack", &self.AddrStack).field("FuncTableEntry", &self.FuncTableEntry).field("Params", &self.Params).field("Far", &self.Far).field("Virtual", &self.Virtual).field("Reserved", &self.Reserved).field("KdHelp", &self.KdHelp).field("AddrBStore", &self.AddrBStore).finish()
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for STACKFRAME {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STACKFRAME {
    fn eq(&self, other: &Self) -> bool {
        self.AddrPC == other.AddrPC && self.AddrReturn == other.AddrReturn && self.AddrFrame == other.AddrFrame && self.AddrStack == other.AddrStack && self.FuncTableEntry == other.FuncTableEntry && self.Params == other.Params && self.Far == other.Far && self.Virtual == other.Virtual && self.Reserved == other.Reserved && self.KdHelp == other.KdHelp && self.AddrBStore == other.AddrBStore
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STACKFRAME {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STACKFRAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STACKFRAME64 {
    pub AddrPC: ADDRESS64,
    pub AddrReturn: ADDRESS64,
    pub AddrFrame: ADDRESS64,
    pub AddrStack: ADDRESS64,
    pub AddrBStore: ADDRESS64,
    pub FuncTableEntry: *mut ::core::ffi::c_void,
    pub Params: [u64; 4],
    pub Far: super::super::super::Foundation::BOOL,
    pub Virtual: super::super::super::Foundation::BOOL,
    pub Reserved: [u64; 3],
    pub KdHelp: KDHELP64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STACKFRAME64 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STACKFRAME64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STACKFRAME64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STACKFRAME64").field("AddrPC", &self.AddrPC).field("AddrReturn", &self.AddrReturn).field("AddrFrame", &self.AddrFrame).field("AddrStack", &self.AddrStack).field("AddrBStore", &self.AddrBStore).field("FuncTableEntry", &self.FuncTableEntry).field("Params", &self.Params).field("Far", &self.Far).field("Virtual", &self.Virtual).field("Reserved", &self.Reserved).field("KdHelp", &self.KdHelp).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for STACKFRAME64 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STACKFRAME64 {
    fn eq(&self, other: &Self) -> bool {
        self.AddrPC == other.AddrPC && self.AddrReturn == other.AddrReturn && self.AddrFrame == other.AddrFrame && self.AddrStack == other.AddrStack && self.AddrBStore == other.AddrBStore && self.FuncTableEntry == other.FuncTableEntry && self.Params == other.Params && self.Far == other.Far && self.Virtual == other.Virtual && self.Reserved == other.Reserved && self.KdHelp == other.KdHelp
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STACKFRAME64 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STACKFRAME64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STACKFRAME_EX {
    pub AddrPC: ADDRESS64,
    pub AddrReturn: ADDRESS64,
    pub AddrFrame: ADDRESS64,
    pub AddrStack: ADDRESS64,
    pub AddrBStore: ADDRESS64,
    pub FuncTableEntry: *mut ::core::ffi::c_void,
    pub Params: [u64; 4],
    pub Far: super::super::super::Foundation::BOOL,
    pub Virtual: super::super::super::Foundation::BOOL,
    pub Reserved: [u64; 3],
    pub KdHelp: KDHELP64,
    pub StackFrameSize: u32,
    pub InlineFrameContext: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STACKFRAME_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STACKFRAME_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STACKFRAME_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STACKFRAME_EX")
            .field("AddrPC", &self.AddrPC)
            .field("AddrReturn", &self.AddrReturn)
            .field("AddrFrame", &self.AddrFrame)
            .field("AddrStack", &self.AddrStack)
            .field("AddrBStore", &self.AddrBStore)
            .field("FuncTableEntry", &self.FuncTableEntry)
            .field("Params", &self.Params)
            .field("Far", &self.Far)
            .field("Virtual", &self.Virtual)
            .field("Reserved", &self.Reserved)
            .field("KdHelp", &self.KdHelp)
            .field("StackFrameSize", &self.StackFrameSize)
            .field("InlineFrameContext", &self.InlineFrameContext)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for STACKFRAME_EX {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STACKFRAME_EX {
    fn eq(&self, other: &Self) -> bool {
        self.AddrPC == other.AddrPC && self.AddrReturn == other.AddrReturn && self.AddrFrame == other.AddrFrame && self.AddrStack == other.AddrStack && self.AddrBStore == other.AddrBStore && self.FuncTableEntry == other.FuncTableEntry && self.Params == other.Params && self.Far == other.Far && self.Virtual == other.Virtual && self.Reserved == other.Reserved && self.KdHelp == other.KdHelp && self.StackFrameSize == other.StackFrameSize && self.InlineFrameContext == other.InlineFrameContext
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STACKFRAME_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STACKFRAME_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct SYMBOL_INFO {
    pub SizeOfStruct: u32,
    pub TypeIndex: u32,
    pub Reserved: [u64; 2],
    pub Index: u32,
    pub Size: u32,
    pub ModBase: u64,
    pub Flags: SYMBOL_INFO_FLAGS,
    pub Value: u64,
    pub Address: u64,
    pub Register: u32,
    pub Scope: u32,
    pub Tag: u32,
    pub NameLen: u32,
    pub MaxNameLen: u32,
    pub Name: [u8; 1],
}
impl ::core::marker::Copy for SYMBOL_INFO {}
impl ::core::clone::Clone for SYMBOL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYMBOL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYMBOL_INFO")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("TypeIndex", &self.TypeIndex)
            .field("Reserved", &self.Reserved)
            .field("Index", &self.Index)
            .field("Size", &self.Size)
            .field("ModBase", &self.ModBase)
            .field("Flags", &self.Flags)
            .field("Value", &self.Value)
            .field("Address", &self.Address)
            .field("Register", &self.Register)
            .field("Scope", &self.Scope)
            .field("Tag", &self.Tag)
            .field("NameLen", &self.NameLen)
            .field("MaxNameLen", &self.MaxNameLen)
            .field("Name", &self.Name)
            .finish()
    }
}
impl ::windows::core::TypeKind for SYMBOL_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYMBOL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.TypeIndex == other.TypeIndex && self.Reserved == other.Reserved && self.Index == other.Index && self.Size == other.Size && self.ModBase == other.ModBase && self.Flags == other.Flags && self.Value == other.Value && self.Address == other.Address && self.Register == other.Register && self.Scope == other.Scope && self.Tag == other.Tag && self.NameLen == other.NameLen && self.MaxNameLen == other.MaxNameLen && self.Name == other.Name
    }
}
impl ::core::cmp::Eq for SYMBOL_INFO {}
impl ::core::default::Default for SYMBOL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct SYMBOL_INFOW {
    pub SizeOfStruct: u32,
    pub TypeIndex: u32,
    pub Reserved: [u64; 2],
    pub Index: u32,
    pub Size: u32,
    pub ModBase: u64,
    pub Flags: SYMBOL_INFO_FLAGS,
    pub Value: u64,
    pub Address: u64,
    pub Register: u32,
    pub Scope: u32,
    pub Tag: u32,
    pub NameLen: u32,
    pub MaxNameLen: u32,
    pub Name: [u16; 1],
}
impl ::core::marker::Copy for SYMBOL_INFOW {}
impl ::core::clone::Clone for SYMBOL_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYMBOL_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYMBOL_INFOW")
            .field("SizeOfStruct", &self.SizeOfStruct)
            .field("TypeIndex", &self.TypeIndex)
            .field("Reserved", &self.Reserved)
            .field("Index", &self.Index)
            .field("Size", &self.Size)
            .field("ModBase", &self.ModBase)
            .field("Flags", &self.Flags)
            .field("Value", &self.Value)
            .field("Address", &self.Address)
            .field("Register", &self.Register)
            .field("Scope", &self.Scope)
            .field("Tag", &self.Tag)
            .field("NameLen", &self.NameLen)
            .field("MaxNameLen", &self.MaxNameLen)
            .field("Name", &self.Name)
            .finish()
    }
}
impl ::windows::core::TypeKind for SYMBOL_INFOW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYMBOL_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfStruct == other.SizeOfStruct && self.TypeIndex == other.TypeIndex && self.Reserved == other.Reserved && self.Index == other.Index && self.Size == other.Size && self.ModBase == other.ModBase && self.Flags == other.Flags && self.Value == other.Value && self.Address == other.Address && self.Register == other.Register && self.Scope == other.Scope && self.Tag == other.Tag && self.NameLen == other.NameLen && self.MaxNameLen == other.MaxNameLen && self.Name == other.Name
    }
}
impl ::core::cmp::Eq for SYMBOL_INFOW {}
impl ::core::default::Default for SYMBOL_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct SYMBOL_INFO_PACKAGE {
    pub si: SYMBOL_INFO,
    pub name: [u8; 2001],
}
impl ::core::marker::Copy for SYMBOL_INFO_PACKAGE {}
impl ::core::clone::Clone for SYMBOL_INFO_PACKAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYMBOL_INFO_PACKAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYMBOL_INFO_PACKAGE").field("si", &self.si).field("name", &self.name).finish()
    }
}
impl ::windows::core::TypeKind for SYMBOL_INFO_PACKAGE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYMBOL_INFO_PACKAGE {
    fn eq(&self, other: &Self) -> bool {
        self.si == other.si && self.name == other.name
    }
}
impl ::core::cmp::Eq for SYMBOL_INFO_PACKAGE {}
impl ::core::default::Default for SYMBOL_INFO_PACKAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct SYMBOL_INFO_PACKAGEW {
    pub si: SYMBOL_INFOW,
    pub name: [u16; 2001],
}
impl ::core::marker::Copy for SYMBOL_INFO_PACKAGEW {}
impl ::core::clone::Clone for SYMBOL_INFO_PACKAGEW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYMBOL_INFO_PACKAGEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYMBOL_INFO_PACKAGEW").field("si", &self.si).field("name", &self.name).finish()
    }
}
impl ::windows::core::TypeKind for SYMBOL_INFO_PACKAGEW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYMBOL_INFO_PACKAGEW {
    fn eq(&self, other: &Self) -> bool {
        self.si == other.si && self.name == other.name
    }
}
impl ::core::cmp::Eq for SYMBOL_INFO_PACKAGEW {}
impl ::core::default::Default for SYMBOL_INFO_PACKAGEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct SYMSRV_EXTENDED_OUTPUT_DATA {
    pub sizeOfStruct: u32,
    pub version: u32,
    pub filePtrMsg: [u16; 261],
}
impl ::core::marker::Copy for SYMSRV_EXTENDED_OUTPUT_DATA {}
impl ::core::clone::Clone for SYMSRV_EXTENDED_OUTPUT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYMSRV_EXTENDED_OUTPUT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYMSRV_EXTENDED_OUTPUT_DATA").field("sizeOfStruct", &self.sizeOfStruct).field("version", &self.version).field("filePtrMsg", &self.filePtrMsg).finish()
    }
}
impl ::windows::core::TypeKind for SYMSRV_EXTENDED_OUTPUT_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYMSRV_EXTENDED_OUTPUT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.sizeOfStruct == other.sizeOfStruct && self.version == other.version && self.filePtrMsg == other.filePtrMsg
    }
}
impl ::core::cmp::Eq for SYMSRV_EXTENDED_OUTPUT_DATA {}
impl ::core::default::Default for SYMSRV_EXTENDED_OUTPUT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SYMSRV_INDEX_INFO {
    pub sizeofstruct: u32,
    pub file: [u8; 261],
    pub stripped: super::super::super::Foundation::BOOL,
    pub timestamp: u32,
    pub size: u32,
    pub dbgfile: [u8; 261],
    pub pdbfile: [u8; 261],
    pub guid: ::windows::core::GUID,
    pub sig: u32,
    pub age: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SYMSRV_INDEX_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SYMSRV_INDEX_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SYMSRV_INDEX_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYMSRV_INDEX_INFO").field("sizeofstruct", &self.sizeofstruct).field("file", &self.file).field("stripped", &self.stripped).field("timestamp", &self.timestamp).field("size", &self.size).field("dbgfile", &self.dbgfile).field("pdbfile", &self.pdbfile).field("guid", &self.guid).field("sig", &self.sig).field("age", &self.age).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SYMSRV_INDEX_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SYMSRV_INDEX_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.sizeofstruct == other.sizeofstruct && self.file == other.file && self.stripped == other.stripped && self.timestamp == other.timestamp && self.size == other.size && self.dbgfile == other.dbgfile && self.pdbfile == other.pdbfile && self.guid == other.guid && self.sig == other.sig && self.age == other.age
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SYMSRV_INDEX_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SYMSRV_INDEX_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SYMSRV_INDEX_INFOW {
    pub sizeofstruct: u32,
    pub file: [u16; 261],
    pub stripped: super::super::super::Foundation::BOOL,
    pub timestamp: u32,
    pub size: u32,
    pub dbgfile: [u16; 261],
    pub pdbfile: [u16; 261],
    pub guid: ::windows::core::GUID,
    pub sig: u32,
    pub age: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SYMSRV_INDEX_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SYMSRV_INDEX_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SYMSRV_INDEX_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYMSRV_INDEX_INFOW").field("sizeofstruct", &self.sizeofstruct).field("file", &self.file).field("stripped", &self.stripped).field("timestamp", &self.timestamp).field("size", &self.size).field("dbgfile", &self.dbgfile).field("pdbfile", &self.pdbfile).field("guid", &self.guid).field("sig", &self.sig).field("age", &self.age).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SYMSRV_INDEX_INFOW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SYMSRV_INDEX_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.sizeofstruct == other.sizeofstruct && self.file == other.file && self.stripped == other.stripped && self.timestamp == other.timestamp && self.size == other.size && self.dbgfile == other.dbgfile && self.pdbfile == other.pdbfile && self.guid == other.guid && self.sig == other.sig && self.age == other.age
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SYMSRV_INDEX_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SYMSRV_INDEX_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct TI_FINDCHILDREN_PARAMS {
    pub Count: u32,
    pub Start: u32,
    pub ChildId: [u32; 1],
}
impl ::core::marker::Copy for TI_FINDCHILDREN_PARAMS {}
impl ::core::clone::Clone for TI_FINDCHILDREN_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TI_FINDCHILDREN_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TI_FINDCHILDREN_PARAMS").field("Count", &self.Count).field("Start", &self.Start).field("ChildId", &self.ChildId).finish()
    }
}
impl ::windows::core::TypeKind for TI_FINDCHILDREN_PARAMS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TI_FINDCHILDREN_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Start == other.Start && self.ChildId == other.ChildId
    }
}
impl ::core::cmp::Eq for TI_FINDCHILDREN_PARAMS {}
impl ::core::default::Default for TI_FINDCHILDREN_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct UNLOAD_DLL_DEBUG_INFO {
    pub lpBaseOfDll: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for UNLOAD_DLL_DEBUG_INFO {}
impl ::core::clone::Clone for UNLOAD_DLL_DEBUG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for UNLOAD_DLL_DEBUG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UNLOAD_DLL_DEBUG_INFO").field("lpBaseOfDll", &self.lpBaseOfDll).finish()
    }
}
impl ::windows::core::TypeKind for UNLOAD_DLL_DEBUG_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for UNLOAD_DLL_DEBUG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpBaseOfDll == other.lpBaseOfDll
    }
}
impl ::core::cmp::Eq for UNLOAD_DLL_DEBUG_INFO {}
impl ::core::default::Default for UNLOAD_DLL_DEBUG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct UNWIND_HISTORY_TABLE {
    pub Count: u32,
    pub LocalHint: u8,
    pub GlobalHint: u8,
    pub Search: u8,
    pub Once: u8,
    pub LowAddress: usize,
    pub HighAddress: usize,
    pub Entry: [UNWIND_HISTORY_TABLE_ENTRY; 12],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for UNWIND_HISTORY_TABLE {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for UNWIND_HISTORY_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for UNWIND_HISTORY_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UNWIND_HISTORY_TABLE").field("Count", &self.Count).field("LocalHint", &self.LocalHint).field("GlobalHint", &self.GlobalHint).field("Search", &self.Search).field("Once", &self.Once).field("LowAddress", &self.LowAddress).field("HighAddress", &self.HighAddress).field("Entry", &self.Entry).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows::core::TypeKind for UNWIND_HISTORY_TABLE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for UNWIND_HISTORY_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.LocalHint == other.LocalHint && self.GlobalHint == other.GlobalHint && self.Search == other.Search && self.Once == other.Once && self.LowAddress == other.LowAddress && self.HighAddress == other.HighAddress && self.Entry == other.Entry
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for UNWIND_HISTORY_TABLE {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for UNWIND_HISTORY_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "aarch64")]
pub struct UNWIND_HISTORY_TABLE_ENTRY {
    pub ImageBase: usize,
    pub FunctionEntry: *mut IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY,
}
#[cfg(target_arch = "aarch64")]
impl ::core::marker::Copy for UNWIND_HISTORY_TABLE_ENTRY {}
#[cfg(target_arch = "aarch64")]
impl ::core::clone::Clone for UNWIND_HISTORY_TABLE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "aarch64")]
impl ::core::fmt::Debug for UNWIND_HISTORY_TABLE_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UNWIND_HISTORY_TABLE_ENTRY").field("ImageBase", &self.ImageBase).field("FunctionEntry", &self.FunctionEntry).finish()
    }
}
#[cfg(target_arch = "aarch64")]
impl ::windows::core::TypeKind for UNWIND_HISTORY_TABLE_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "aarch64")]
impl ::core::cmp::PartialEq for UNWIND_HISTORY_TABLE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.ImageBase == other.ImageBase && self.FunctionEntry == other.FunctionEntry
    }
}
#[cfg(target_arch = "aarch64")]
impl ::core::cmp::Eq for UNWIND_HISTORY_TABLE_ENTRY {}
#[cfg(target_arch = "aarch64")]
impl ::core::default::Default for UNWIND_HISTORY_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86_64")]
pub struct UNWIND_HISTORY_TABLE_ENTRY {
    pub ImageBase: usize,
    pub FunctionEntry: *mut IMAGE_RUNTIME_FUNCTION_ENTRY,
}
#[cfg(target_arch = "x86_64")]
impl ::core::marker::Copy for UNWIND_HISTORY_TABLE_ENTRY {}
#[cfg(target_arch = "x86_64")]
impl ::core::clone::Clone for UNWIND_HISTORY_TABLE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86_64")]
impl ::core::fmt::Debug for UNWIND_HISTORY_TABLE_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UNWIND_HISTORY_TABLE_ENTRY").field("ImageBase", &self.ImageBase).field("FunctionEntry", &self.FunctionEntry).finish()
    }
}
#[cfg(target_arch = "x86_64")]
impl ::windows::core::TypeKind for UNWIND_HISTORY_TABLE_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86_64")]
impl ::core::cmp::PartialEq for UNWIND_HISTORY_TABLE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.ImageBase == other.ImageBase && self.FunctionEntry == other.FunctionEntry
    }
}
#[cfg(target_arch = "x86_64")]
impl ::core::cmp::Eq for UNWIND_HISTORY_TABLE_ENTRY {}
#[cfg(target_arch = "x86_64")]
impl ::core::default::Default for UNWIND_HISTORY_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WAITCHAIN_NODE_INFO {
    pub ObjectType: WCT_OBJECT_TYPE,
    pub ObjectStatus: WCT_OBJECT_STATUS,
    pub Anonymous: WAITCHAIN_NODE_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WAITCHAIN_NODE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WAITCHAIN_NODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WAITCHAIN_NODE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WAITCHAIN_NODE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union WAITCHAIN_NODE_INFO_0 {
    pub LockObject: WAITCHAIN_NODE_INFO_0_0,
    pub ThreadObject: WAITCHAIN_NODE_INFO_0_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WAITCHAIN_NODE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WAITCHAIN_NODE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WAITCHAIN_NODE_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WAITCHAIN_NODE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WAITCHAIN_NODE_INFO_0_0 {
    pub ObjectName: [u16; 128],
    pub Timeout: i64,
    pub Alertable: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WAITCHAIN_NODE_INFO_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WAITCHAIN_NODE_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WAITCHAIN_NODE_INFO_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WAITCHAIN_NODE_INFO_0_0").field("ObjectName", &self.ObjectName).field("Timeout", &self.Timeout).field("Alertable", &self.Alertable).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WAITCHAIN_NODE_INFO_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WAITCHAIN_NODE_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectName == other.ObjectName && self.Timeout == other.Timeout && self.Alertable == other.Alertable
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WAITCHAIN_NODE_INFO_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WAITCHAIN_NODE_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WAITCHAIN_NODE_INFO_0_1 {
    pub ProcessId: u32,
    pub ThreadId: u32,
    pub WaitTime: u32,
    pub ContextSwitches: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WAITCHAIN_NODE_INFO_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WAITCHAIN_NODE_INFO_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WAITCHAIN_NODE_INFO_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WAITCHAIN_NODE_INFO_0_1").field("ProcessId", &self.ProcessId).field("ThreadId", &self.ThreadId).field("WaitTime", &self.WaitTime).field("ContextSwitches", &self.ContextSwitches).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WAITCHAIN_NODE_INFO_0_1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WAITCHAIN_NODE_INFO_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.ProcessId == other.ProcessId && self.ThreadId == other.ThreadId && self.WaitTime == other.WaitTime && self.ContextSwitches == other.ContextSwitches
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WAITCHAIN_NODE_INFO_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WAITCHAIN_NODE_INFO_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WHEA_AER_BRIDGE_DESCRIPTOR {
    pub Type: u16,
    pub Enabled: super::super::super::Foundation::BOOLEAN,
    pub Reserved: u8,
    pub BusNumber: u32,
    pub Slot: WHEA_PCI_SLOT_NUMBER,
    pub DeviceControl: u16,
    pub Flags: AER_BRIDGE_DESCRIPTOR_FLAGS,
    pub UncorrectableErrorMask: u32,
    pub UncorrectableErrorSeverity: u32,
    pub CorrectableErrorMask: u32,
    pub AdvancedCapsAndControl: u32,
    pub SecondaryUncorrectableErrorMask: u32,
    pub SecondaryUncorrectableErrorSev: u32,
    pub SecondaryCapsAndControl: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WHEA_AER_BRIDGE_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WHEA_AER_BRIDGE_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WHEA_AER_BRIDGE_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WHEA_AER_BRIDGE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WHEA_AER_ENDPOINT_DESCRIPTOR {
    pub Type: u16,
    pub Enabled: super::super::super::Foundation::BOOLEAN,
    pub Reserved: u8,
    pub BusNumber: u32,
    pub Slot: WHEA_PCI_SLOT_NUMBER,
    pub DeviceControl: u16,
    pub Flags: AER_ENDPOINT_DESCRIPTOR_FLAGS,
    pub UncorrectableErrorMask: u32,
    pub UncorrectableErrorSeverity: u32,
    pub CorrectableErrorMask: u32,
    pub AdvancedCapsAndControl: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WHEA_AER_ENDPOINT_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WHEA_AER_ENDPOINT_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WHEA_AER_ENDPOINT_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WHEA_AER_ENDPOINT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WHEA_AER_ROOTPORT_DESCRIPTOR {
    pub Type: u16,
    pub Enabled: super::super::super::Foundation::BOOLEAN,
    pub Reserved: u8,
    pub BusNumber: u32,
    pub Slot: WHEA_PCI_SLOT_NUMBER,
    pub DeviceControl: u16,
    pub Flags: AER_ROOTPORT_DESCRIPTOR_FLAGS,
    pub UncorrectableErrorMask: u32,
    pub UncorrectableErrorSeverity: u32,
    pub CorrectableErrorMask: u32,
    pub AdvancedCapsAndControl: u32,
    pub RootErrorCommand: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WHEA_AER_ROOTPORT_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WHEA_AER_ROOTPORT_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WHEA_AER_ROOTPORT_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WHEA_AER_ROOTPORT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WHEA_DEVICE_DRIVER_DESCRIPTOR {
    pub Type: u16,
    pub Enabled: super::super::super::Foundation::BOOLEAN,
    pub Reserved: u8,
    pub SourceGuid: ::windows::core::GUID,
    pub LogTag: u16,
    pub Reserved2: u16,
    pub PacketLength: u32,
    pub PacketCount: u32,
    pub PacketBuffer: *mut u8,
    pub Config: WHEA_ERROR_SOURCE_CONFIGURATION_DD,
    pub CreatorId: ::windows::core::GUID,
    pub PartitionId: ::windows::core::GUID,
    pub MaxSectionDataLength: u32,
    pub MaxSectionsPerRecord: u32,
    pub PacketStateBuffer: *mut u8,
    pub OpenHandles: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WHEA_DEVICE_DRIVER_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WHEA_DEVICE_DRIVER_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WHEA_DEVICE_DRIVER_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WHEA_DEVICE_DRIVER_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct WHEA_DRIVER_BUFFER_SET {
    pub Version: u32,
    pub Data: *mut u8,
    pub DataSize: u32,
    pub SectionTypeGuid: *mut ::windows::core::GUID,
    pub SectionFriendlyName: *mut u8,
    pub Flags: *mut u8,
}
impl ::core::marker::Copy for WHEA_DRIVER_BUFFER_SET {}
impl ::core::clone::Clone for WHEA_DRIVER_BUFFER_SET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WHEA_DRIVER_BUFFER_SET {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WHEA_DRIVER_BUFFER_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WHEA_ERROR_SOURCE_CONFIGURATION_DD {
    pub Initialize: WHEA_ERROR_SOURCE_INITIALIZE_DEVICE_DRIVER,
    pub Uninitialize: WHEA_ERROR_SOURCE_UNINITIALIZE_DEVICE_DRIVER,
    pub Correct: WHEA_ERROR_SOURCE_CORRECT_DEVICE_DRIVER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WHEA_ERROR_SOURCE_CONFIGURATION_DD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WHEA_ERROR_SOURCE_CONFIGURATION_DD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WHEA_ERROR_SOURCE_CONFIGURATION_DD {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WHEA_ERROR_SOURCE_CONFIGURATION_DD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER {
    pub Version: u32,
    pub SourceGuid: ::windows::core::GUID,
    pub LogTag: u16,
    pub Reserved: [u8; 6],
    pub Initialize: WHEA_ERROR_SOURCE_INITIALIZE_DEVICE_DRIVER,
    pub Uninitialize: WHEA_ERROR_SOURCE_UNINITIALIZE_DEVICE_DRIVER,
    pub MaxSectionDataLength: u32,
    pub MaxSectionsPerReport: u32,
    pub CreatorId: ::windows::core::GUID,
    pub PartitionId: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER_V1 {
    pub Version: u32,
    pub SourceGuid: ::windows::core::GUID,
    pub LogTag: u16,
    pub Reserved: [u8; 6],
    pub Initialize: WHEA_ERROR_SOURCE_INITIALIZE_DEVICE_DRIVER,
    pub Uninitialize: WHEA_ERROR_SOURCE_UNINITIALIZE_DEVICE_DRIVER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER_V1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER_V1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WHEA_ERROR_SOURCE_DESCRIPTOR {
    pub Length: u32,
    pub Version: u32,
    pub Type: WHEA_ERROR_SOURCE_TYPE,
    pub State: WHEA_ERROR_SOURCE_STATE,
    pub MaxRawDataLength: u32,
    pub NumRecordsToPreallocate: u32,
    pub MaxSectionsPerRecord: u32,
    pub ErrorSourceId: u32,
    pub PlatformErrorSourceId: u32,
    pub Flags: u32,
    pub Info: WHEA_ERROR_SOURCE_DESCRIPTOR_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WHEA_ERROR_SOURCE_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WHEA_ERROR_SOURCE_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WHEA_ERROR_SOURCE_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WHEA_ERROR_SOURCE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union WHEA_ERROR_SOURCE_DESCRIPTOR_0 {
    pub XpfMceDescriptor: WHEA_XPF_MCE_DESCRIPTOR,
    pub XpfCmcDescriptor: WHEA_XPF_CMC_DESCRIPTOR,
    pub XpfNmiDescriptor: WHEA_XPF_NMI_DESCRIPTOR,
    pub IpfMcaDescriptor: WHEA_IPF_MCA_DESCRIPTOR,
    pub IpfCmcDescriptor: WHEA_IPF_CMC_DESCRIPTOR,
    pub IpfCpeDescriptor: WHEA_IPF_CPE_DESCRIPTOR,
    pub AerRootportDescriptor: WHEA_AER_ROOTPORT_DESCRIPTOR,
    pub AerEndpointDescriptor: WHEA_AER_ENDPOINT_DESCRIPTOR,
    pub AerBridgeDescriptor: WHEA_AER_BRIDGE_DESCRIPTOR,
    pub GenErrDescriptor: WHEA_GENERIC_ERROR_DESCRIPTOR,
    pub GenErrDescriptorV2: WHEA_GENERIC_ERROR_DESCRIPTOR_V2,
    pub DeviceDriverDescriptor: WHEA_DEVICE_DRIVER_DESCRIPTOR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WHEA_ERROR_SOURCE_DESCRIPTOR_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WHEA_ERROR_SOURCE_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WHEA_ERROR_SOURCE_DESCRIPTOR_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WHEA_ERROR_SOURCE_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct WHEA_GENERIC_ERROR_DESCRIPTOR {
    pub Type: u16,
    pub Reserved: u8,
    pub Enabled: u8,
    pub ErrStatusBlockLength: u32,
    pub RelatedErrorSourceId: u32,
    pub ErrStatusAddressSpaceID: u8,
    pub ErrStatusAddressBitWidth: u8,
    pub ErrStatusAddressBitOffset: u8,
    pub ErrStatusAddressAccessSize: u8,
    pub ErrStatusAddress: i64,
    pub Notify: WHEA_NOTIFICATION_DESCRIPTOR,
}
impl ::core::marker::Copy for WHEA_GENERIC_ERROR_DESCRIPTOR {}
impl ::core::clone::Clone for WHEA_GENERIC_ERROR_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WHEA_GENERIC_ERROR_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WHEA_GENERIC_ERROR_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct WHEA_GENERIC_ERROR_DESCRIPTOR_V2 {
    pub Type: u16,
    pub Reserved: u8,
    pub Enabled: u8,
    pub ErrStatusBlockLength: u32,
    pub RelatedErrorSourceId: u32,
    pub ErrStatusAddressSpaceID: u8,
    pub ErrStatusAddressBitWidth: u8,
    pub ErrStatusAddressBitOffset: u8,
    pub ErrStatusAddressAccessSize: u8,
    pub ErrStatusAddress: i64,
    pub Notify: WHEA_NOTIFICATION_DESCRIPTOR,
    pub ReadAckAddressSpaceID: u8,
    pub ReadAckAddressBitWidth: u8,
    pub ReadAckAddressBitOffset: u8,
    pub ReadAckAddressAccessSize: u8,
    pub ReadAckAddress: i64,
    pub ReadAckPreserveMask: u64,
    pub ReadAckWriteMask: u64,
}
impl ::core::marker::Copy for WHEA_GENERIC_ERROR_DESCRIPTOR_V2 {}
impl ::core::clone::Clone for WHEA_GENERIC_ERROR_DESCRIPTOR_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WHEA_GENERIC_ERROR_DESCRIPTOR_V2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WHEA_GENERIC_ERROR_DESCRIPTOR_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct WHEA_IPF_CMC_DESCRIPTOR {
    pub Type: u16,
    pub Enabled: u8,
    pub Reserved: u8,
}
impl ::core::marker::Copy for WHEA_IPF_CMC_DESCRIPTOR {}
impl ::core::clone::Clone for WHEA_IPF_CMC_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WHEA_IPF_CMC_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WHEA_IPF_CMC_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct WHEA_IPF_CPE_DESCRIPTOR {
    pub Type: u16,
    pub Enabled: u8,
    pub Reserved: u8,
}
impl ::core::marker::Copy for WHEA_IPF_CPE_DESCRIPTOR {}
impl ::core::clone::Clone for WHEA_IPF_CPE_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WHEA_IPF_CPE_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WHEA_IPF_CPE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct WHEA_IPF_MCA_DESCRIPTOR {
    pub Type: u16,
    pub Enabled: u8,
    pub Reserved: u8,
}
impl ::core::marker::Copy for WHEA_IPF_MCA_DESCRIPTOR {}
impl ::core::clone::Clone for WHEA_IPF_MCA_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WHEA_IPF_MCA_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WHEA_IPF_MCA_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct WHEA_NOTIFICATION_DESCRIPTOR {
    pub Type: u8,
    pub Length: u8,
    pub Flags: WHEA_NOTIFICATION_FLAGS,
    pub u: WHEA_NOTIFICATION_DESCRIPTOR_0,
}
impl ::core::marker::Copy for WHEA_NOTIFICATION_DESCRIPTOR {}
impl ::core::clone::Clone for WHEA_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WHEA_NOTIFICATION_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WHEA_NOTIFICATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub union WHEA_NOTIFICATION_DESCRIPTOR_0 {
    pub Polled: WHEA_NOTIFICATION_DESCRIPTOR_0_4,
    pub Interrupt: WHEA_NOTIFICATION_DESCRIPTOR_0_1,
    pub LocalInterrupt: WHEA_NOTIFICATION_DESCRIPTOR_0_2,
    pub Sci: WHEA_NOTIFICATION_DESCRIPTOR_0_5,
    pub Nmi: WHEA_NOTIFICATION_DESCRIPTOR_0_3,
    pub Sea: WHEA_NOTIFICATION_DESCRIPTOR_0_6,
    pub Sei: WHEA_NOTIFICATION_DESCRIPTOR_0_7,
    pub Gsiv: WHEA_NOTIFICATION_DESCRIPTOR_0_0,
}
impl ::core::marker::Copy for WHEA_NOTIFICATION_DESCRIPTOR_0 {}
impl ::core::clone::Clone for WHEA_NOTIFICATION_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WHEA_NOTIFICATION_DESCRIPTOR_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WHEA_NOTIFICATION_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct WHEA_NOTIFICATION_DESCRIPTOR_0_0 {
    pub PollInterval: u32,
    pub Vector: u32,
    pub SwitchToPollingThreshold: u32,
    pub SwitchToPollingWindow: u32,
    pub ErrorThreshold: u32,
    pub ErrorThresholdWindow: u32,
}
impl ::core::marker::Copy for WHEA_NOTIFICATION_DESCRIPTOR_0_0 {}
impl ::core::clone::Clone for WHEA_NOTIFICATION_DESCRIPTOR_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WHEA_NOTIFICATION_DESCRIPTOR_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WHEA_NOTIFICATION_DESCRIPTOR_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct WHEA_NOTIFICATION_DESCRIPTOR_0_1 {
    pub PollInterval: u32,
    pub Vector: u32,
    pub SwitchToPollingThreshold: u32,
    pub SwitchToPollingWindow: u32,
    pub ErrorThreshold: u32,
    pub ErrorThresholdWindow: u32,
}
impl ::core::marker::Copy for WHEA_NOTIFICATION_DESCRIPTOR_0_1 {}
impl ::core::clone::Clone for WHEA_NOTIFICATION_DESCRIPTOR_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WHEA_NOTIFICATION_DESCRIPTOR_0_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WHEA_NOTIFICATION_DESCRIPTOR_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct WHEA_NOTIFICATION_DESCRIPTOR_0_2 {
    pub PollInterval: u32,
    pub Vector: u32,
    pub SwitchToPollingThreshold: u32,
    pub SwitchToPollingWindow: u32,
    pub ErrorThreshold: u32,
    pub ErrorThresholdWindow: u32,
}
impl ::core::marker::Copy for WHEA_NOTIFICATION_DESCRIPTOR_0_2 {}
impl ::core::clone::Clone for WHEA_NOTIFICATION_DESCRIPTOR_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WHEA_NOTIFICATION_DESCRIPTOR_0_2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WHEA_NOTIFICATION_DESCRIPTOR_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct WHEA_NOTIFICATION_DESCRIPTOR_0_3 {
    pub PollInterval: u32,
    pub Vector: u32,
    pub SwitchToPollingThreshold: u32,
    pub SwitchToPollingWindow: u32,
    pub ErrorThreshold: u32,
    pub ErrorThresholdWindow: u32,
}
impl ::core::marker::Copy for WHEA_NOTIFICATION_DESCRIPTOR_0_3 {}
impl ::core::clone::Clone for WHEA_NOTIFICATION_DESCRIPTOR_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WHEA_NOTIFICATION_DESCRIPTOR_0_3 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WHEA_NOTIFICATION_DESCRIPTOR_0_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct WHEA_NOTIFICATION_DESCRIPTOR_0_4 {
    pub PollInterval: u32,
}
impl ::core::marker::Copy for WHEA_NOTIFICATION_DESCRIPTOR_0_4 {}
impl ::core::clone::Clone for WHEA_NOTIFICATION_DESCRIPTOR_0_4 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WHEA_NOTIFICATION_DESCRIPTOR_0_4 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WHEA_NOTIFICATION_DESCRIPTOR_0_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct WHEA_NOTIFICATION_DESCRIPTOR_0_5 {
    pub PollInterval: u32,
    pub Vector: u32,
    pub SwitchToPollingThreshold: u32,
    pub SwitchToPollingWindow: u32,
    pub ErrorThreshold: u32,
    pub ErrorThresholdWindow: u32,
}
impl ::core::marker::Copy for WHEA_NOTIFICATION_DESCRIPTOR_0_5 {}
impl ::core::clone::Clone for WHEA_NOTIFICATION_DESCRIPTOR_0_5 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WHEA_NOTIFICATION_DESCRIPTOR_0_5 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WHEA_NOTIFICATION_DESCRIPTOR_0_5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct WHEA_NOTIFICATION_DESCRIPTOR_0_6 {
    pub PollInterval: u32,
    pub Vector: u32,
    pub SwitchToPollingThreshold: u32,
    pub SwitchToPollingWindow: u32,
    pub ErrorThreshold: u32,
    pub ErrorThresholdWindow: u32,
}
impl ::core::marker::Copy for WHEA_NOTIFICATION_DESCRIPTOR_0_6 {}
impl ::core::clone::Clone for WHEA_NOTIFICATION_DESCRIPTOR_0_6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WHEA_NOTIFICATION_DESCRIPTOR_0_6 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WHEA_NOTIFICATION_DESCRIPTOR_0_6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct WHEA_NOTIFICATION_DESCRIPTOR_0_7 {
    pub PollInterval: u32,
    pub Vector: u32,
    pub SwitchToPollingThreshold: u32,
    pub SwitchToPollingWindow: u32,
    pub ErrorThreshold: u32,
    pub ErrorThresholdWindow: u32,
}
impl ::core::marker::Copy for WHEA_NOTIFICATION_DESCRIPTOR_0_7 {}
impl ::core::clone::Clone for WHEA_NOTIFICATION_DESCRIPTOR_0_7 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WHEA_NOTIFICATION_DESCRIPTOR_0_7 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WHEA_NOTIFICATION_DESCRIPTOR_0_7 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub union WHEA_NOTIFICATION_FLAGS {
    pub Anonymous: WHEA_NOTIFICATION_FLAGS_0,
    pub AsUSHORT: u16,
}
impl ::core::marker::Copy for WHEA_NOTIFICATION_FLAGS {}
impl ::core::clone::Clone for WHEA_NOTIFICATION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WHEA_NOTIFICATION_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WHEA_NOTIFICATION_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct WHEA_NOTIFICATION_FLAGS_0 {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for WHEA_NOTIFICATION_FLAGS_0 {}
impl ::core::clone::Clone for WHEA_NOTIFICATION_FLAGS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WHEA_NOTIFICATION_FLAGS_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WHEA_NOTIFICATION_FLAGS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct WHEA_PCI_SLOT_NUMBER {
    pub u: WHEA_PCI_SLOT_NUMBER_0,
}
impl ::core::marker::Copy for WHEA_PCI_SLOT_NUMBER {}
impl ::core::clone::Clone for WHEA_PCI_SLOT_NUMBER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WHEA_PCI_SLOT_NUMBER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WHEA_PCI_SLOT_NUMBER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub union WHEA_PCI_SLOT_NUMBER_0 {
    pub bits: WHEA_PCI_SLOT_NUMBER_0_0,
    pub AsULONG: u32,
}
impl ::core::marker::Copy for WHEA_PCI_SLOT_NUMBER_0 {}
impl ::core::clone::Clone for WHEA_PCI_SLOT_NUMBER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WHEA_PCI_SLOT_NUMBER_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WHEA_PCI_SLOT_NUMBER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct WHEA_PCI_SLOT_NUMBER_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for WHEA_PCI_SLOT_NUMBER_0_0 {}
impl ::core::clone::Clone for WHEA_PCI_SLOT_NUMBER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WHEA_PCI_SLOT_NUMBER_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WHEA_PCI_SLOT_NUMBER_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WHEA_XPF_CMC_DESCRIPTOR {
    pub Type: u16,
    pub Enabled: super::super::super::Foundation::BOOLEAN,
    pub NumberOfBanks: u8,
    pub Reserved: u32,
    pub Notify: WHEA_NOTIFICATION_DESCRIPTOR,
    pub Banks: [WHEA_XPF_MC_BANK_DESCRIPTOR; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WHEA_XPF_CMC_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WHEA_XPF_CMC_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WHEA_XPF_CMC_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WHEA_XPF_CMC_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WHEA_XPF_MCE_DESCRIPTOR {
    pub Type: u16,
    pub Enabled: u8,
    pub NumberOfBanks: u8,
    pub Flags: XPF_MCE_FLAGS,
    pub MCG_Capability: u64,
    pub MCG_GlobalControl: u64,
    pub Banks: [WHEA_XPF_MC_BANK_DESCRIPTOR; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WHEA_XPF_MCE_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WHEA_XPF_MCE_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WHEA_XPF_MCE_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WHEA_XPF_MCE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WHEA_XPF_MC_BANK_DESCRIPTOR {
    pub BankNumber: u8,
    pub ClearOnInitialization: super::super::super::Foundation::BOOLEAN,
    pub StatusDataFormat: u8,
    pub Flags: XPF_MC_BANK_FLAGS,
    pub ControlMsr: u32,
    pub StatusMsr: u32,
    pub AddressMsr: u32,
    pub MiscMsr: u32,
    pub ControlData: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WHEA_XPF_MC_BANK_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WHEA_XPF_MC_BANK_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WHEA_XPF_MC_BANK_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WHEA_XPF_MC_BANK_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WHEA_XPF_NMI_DESCRIPTOR {
    pub Type: u16,
    pub Enabled: super::super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WHEA_XPF_NMI_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WHEA_XPF_NMI_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WHEA_XPF_NMI_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WHEA_XPF_NMI_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct WOW64_CONTEXT {
    pub ContextFlags: u32,
    pub Dr0: u32,
    pub Dr1: u32,
    pub Dr2: u32,
    pub Dr3: u32,
    pub Dr6: u32,
    pub Dr7: u32,
    pub FloatSave: WOW64_FLOATING_SAVE_AREA,
    pub SegGs: u32,
    pub SegFs: u32,
    pub SegEs: u32,
    pub SegDs: u32,
    pub Edi: u32,
    pub Esi: u32,
    pub Ebx: u32,
    pub Edx: u32,
    pub Ecx: u32,
    pub Eax: u32,
    pub Ebp: u32,
    pub Eip: u32,
    pub SegCs: u32,
    pub EFlags: u32,
    pub Esp: u32,
    pub SegSs: u32,
    pub ExtendedRegisters: [u8; 512],
}
impl ::core::marker::Copy for WOW64_CONTEXT {}
impl ::core::clone::Clone for WOW64_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WOW64_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WOW64_CONTEXT")
            .field("ContextFlags", &self.ContextFlags)
            .field("Dr0", &self.Dr0)
            .field("Dr1", &self.Dr1)
            .field("Dr2", &self.Dr2)
            .field("Dr3", &self.Dr3)
            .field("Dr6", &self.Dr6)
            .field("Dr7", &self.Dr7)
            .field("FloatSave", &self.FloatSave)
            .field("SegGs", &self.SegGs)
            .field("SegFs", &self.SegFs)
            .field("SegEs", &self.SegEs)
            .field("SegDs", &self.SegDs)
            .field("Edi", &self.Edi)
            .field("Esi", &self.Esi)
            .field("Ebx", &self.Ebx)
            .field("Edx", &self.Edx)
            .field("Ecx", &self.Ecx)
            .field("Eax", &self.Eax)
            .field("Ebp", &self.Ebp)
            .field("Eip", &self.Eip)
            .field("SegCs", &self.SegCs)
            .field("EFlags", &self.EFlags)
            .field("Esp", &self.Esp)
            .field("SegSs", &self.SegSs)
            .field("ExtendedRegisters", &self.ExtendedRegisters)
            .finish()
    }
}
impl ::windows::core::TypeKind for WOW64_CONTEXT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WOW64_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.ContextFlags == other.ContextFlags && self.Dr0 == other.Dr0 && self.Dr1 == other.Dr1 && self.Dr2 == other.Dr2 && self.Dr3 == other.Dr3 && self.Dr6 == other.Dr6 && self.Dr7 == other.Dr7 && self.FloatSave == other.FloatSave && self.SegGs == other.SegGs && self.SegFs == other.SegFs && self.SegEs == other.SegEs && self.SegDs == other.SegDs && self.Edi == other.Edi && self.Esi == other.Esi && self.Ebx == other.Ebx && self.Edx == other.Edx && self.Ecx == other.Ecx && self.Eax == other.Eax && self.Ebp == other.Ebp && self.Eip == other.Eip && self.SegCs == other.SegCs && self.EFlags == other.EFlags && self.Esp == other.Esp && self.SegSs == other.SegSs && self.ExtendedRegisters == other.ExtendedRegisters
    }
}
impl ::core::cmp::Eq for WOW64_CONTEXT {}
impl ::core::default::Default for WOW64_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct WOW64_DESCRIPTOR_TABLE_ENTRY {
    pub Selector: u32,
    pub Descriptor: WOW64_LDT_ENTRY,
}
impl ::core::marker::Copy for WOW64_DESCRIPTOR_TABLE_ENTRY {}
impl ::core::clone::Clone for WOW64_DESCRIPTOR_TABLE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WOW64_DESCRIPTOR_TABLE_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WOW64_DESCRIPTOR_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct WOW64_FLOATING_SAVE_AREA {
    pub ControlWord: u32,
    pub StatusWord: u32,
    pub TagWord: u32,
    pub ErrorOffset: u32,
    pub ErrorSelector: u32,
    pub DataOffset: u32,
    pub DataSelector: u32,
    pub RegisterArea: [u8; 80],
    pub Cr0NpxState: u32,
}
impl ::core::marker::Copy for WOW64_FLOATING_SAVE_AREA {}
impl ::core::clone::Clone for WOW64_FLOATING_SAVE_AREA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WOW64_FLOATING_SAVE_AREA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WOW64_FLOATING_SAVE_AREA").field("ControlWord", &self.ControlWord).field("StatusWord", &self.StatusWord).field("TagWord", &self.TagWord).field("ErrorOffset", &self.ErrorOffset).field("ErrorSelector", &self.ErrorSelector).field("DataOffset", &self.DataOffset).field("DataSelector", &self.DataSelector).field("RegisterArea", &self.RegisterArea).field("Cr0NpxState", &self.Cr0NpxState).finish()
    }
}
impl ::windows::core::TypeKind for WOW64_FLOATING_SAVE_AREA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WOW64_FLOATING_SAVE_AREA {
    fn eq(&self, other: &Self) -> bool {
        self.ControlWord == other.ControlWord && self.StatusWord == other.StatusWord && self.TagWord == other.TagWord && self.ErrorOffset == other.ErrorOffset && self.ErrorSelector == other.ErrorSelector && self.DataOffset == other.DataOffset && self.DataSelector == other.DataSelector && self.RegisterArea == other.RegisterArea && self.Cr0NpxState == other.Cr0NpxState
    }
}
impl ::core::cmp::Eq for WOW64_FLOATING_SAVE_AREA {}
impl ::core::default::Default for WOW64_FLOATING_SAVE_AREA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct WOW64_LDT_ENTRY {
    pub LimitLow: u16,
    pub BaseLow: u16,
    pub HighWord: WOW64_LDT_ENTRY_0,
}
impl ::core::marker::Copy for WOW64_LDT_ENTRY {}
impl ::core::clone::Clone for WOW64_LDT_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WOW64_LDT_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WOW64_LDT_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub union WOW64_LDT_ENTRY_0 {
    pub Bytes: WOW64_LDT_ENTRY_0_1,
    pub Bits: WOW64_LDT_ENTRY_0_0,
}
impl ::core::marker::Copy for WOW64_LDT_ENTRY_0 {}
impl ::core::clone::Clone for WOW64_LDT_ENTRY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for WOW64_LDT_ENTRY_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for WOW64_LDT_ENTRY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct WOW64_LDT_ENTRY_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for WOW64_LDT_ENTRY_0_0 {}
impl ::core::clone::Clone for WOW64_LDT_ENTRY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WOW64_LDT_ENTRY_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WOW64_LDT_ENTRY_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows::core::TypeKind for WOW64_LDT_ENTRY_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WOW64_LDT_ENTRY_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for WOW64_LDT_ENTRY_0_0 {}
impl ::core::default::Default for WOW64_LDT_ENTRY_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct WOW64_LDT_ENTRY_0_1 {
    pub BaseMid: u8,
    pub Flags1: u8,
    pub Flags2: u8,
    pub BaseHi: u8,
}
impl ::core::marker::Copy for WOW64_LDT_ENTRY_0_1 {}
impl ::core::clone::Clone for WOW64_LDT_ENTRY_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WOW64_LDT_ENTRY_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WOW64_LDT_ENTRY_0_1").field("BaseMid", &self.BaseMid).field("Flags1", &self.Flags1).field("Flags2", &self.Flags2).field("BaseHi", &self.BaseHi).finish()
    }
}
impl ::windows::core::TypeKind for WOW64_LDT_ENTRY_0_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WOW64_LDT_ENTRY_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.BaseMid == other.BaseMid && self.Flags1 == other.Flags1 && self.Flags2 == other.Flags2 && self.BaseHi == other.BaseHi
    }
}
impl ::core::cmp::Eq for WOW64_LDT_ENTRY_0_1 {}
impl ::core::default::Default for WOW64_LDT_ENTRY_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub union XPF_MCE_FLAGS {
    pub Anonymous: XPF_MCE_FLAGS_0,
    pub AsULONG: u32,
}
impl ::core::marker::Copy for XPF_MCE_FLAGS {}
impl ::core::clone::Clone for XPF_MCE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for XPF_MCE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for XPF_MCE_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct XPF_MCE_FLAGS_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for XPF_MCE_FLAGS_0 {}
impl ::core::clone::Clone for XPF_MCE_FLAGS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for XPF_MCE_FLAGS_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for XPF_MCE_FLAGS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub union XPF_MC_BANK_FLAGS {
    pub Anonymous: XPF_MC_BANK_FLAGS_0,
    pub AsUCHAR: u8,
}
impl ::core::marker::Copy for XPF_MC_BANK_FLAGS {}
impl ::core::clone::Clone for XPF_MC_BANK_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for XPF_MC_BANK_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for XPF_MC_BANK_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct XPF_MC_BANK_FLAGS_0 {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for XPF_MC_BANK_FLAGS_0 {}
impl ::core::clone::Clone for XPF_MC_BANK_FLAGS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XPF_MC_BANK_FLAGS_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XPF_MC_BANK_FLAGS_0").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows::core::TypeKind for XPF_MC_BANK_FLAGS_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for XPF_MC_BANK_FLAGS_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for XPF_MC_BANK_FLAGS_0 {}
impl ::core::default::Default for XPF_MC_BANK_FLAGS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct XSAVE_AREA {
    pub LegacyState: XSAVE_FORMAT,
    pub Header: XSAVE_AREA_HEADER,
}
impl ::core::marker::Copy for XSAVE_AREA {}
impl ::core::clone::Clone for XSAVE_AREA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XSAVE_AREA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XSAVE_AREA").field("LegacyState", &self.LegacyState).field("Header", &self.Header).finish()
    }
}
impl ::windows::core::TypeKind for XSAVE_AREA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for XSAVE_AREA {
    fn eq(&self, other: &Self) -> bool {
        self.LegacyState == other.LegacyState && self.Header == other.Header
    }
}
impl ::core::cmp::Eq for XSAVE_AREA {}
impl ::core::default::Default for XSAVE_AREA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct XSAVE_AREA_HEADER {
    pub Mask: u64,
    pub CompactionMask: u64,
    pub Reserved2: [u64; 6],
}
impl ::core::marker::Copy for XSAVE_AREA_HEADER {}
impl ::core::clone::Clone for XSAVE_AREA_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XSAVE_AREA_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XSAVE_AREA_HEADER").field("Mask", &self.Mask).field("CompactionMask", &self.CompactionMask).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::windows::core::TypeKind for XSAVE_AREA_HEADER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for XSAVE_AREA_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Mask == other.Mask && self.CompactionMask == other.CompactionMask && self.Reserved2 == other.Reserved2
    }
}
impl ::core::cmp::Eq for XSAVE_AREA_HEADER {}
impl ::core::default::Default for XSAVE_AREA_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct XSAVE_FORMAT {
    pub ControlWord: u16,
    pub StatusWord: u16,
    pub TagWord: u8,
    pub Reserved1: u8,
    pub ErrorOpcode: u16,
    pub ErrorOffset: u32,
    pub ErrorSelector: u16,
    pub Reserved2: u16,
    pub DataOffset: u32,
    pub DataSelector: u16,
    pub Reserved3: u16,
    pub MxCsr: u32,
    pub MxCsr_Mask: u32,
    pub FloatRegisters: [M128A; 8],
    pub XmmRegisters: [M128A; 16],
    pub Reserved4: [u8; 96],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for XSAVE_FORMAT {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for XSAVE_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for XSAVE_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XSAVE_FORMAT")
            .field("ControlWord", &self.ControlWord)
            .field("StatusWord", &self.StatusWord)
            .field("TagWord", &self.TagWord)
            .field("Reserved1", &self.Reserved1)
            .field("ErrorOpcode", &self.ErrorOpcode)
            .field("ErrorOffset", &self.ErrorOffset)
            .field("ErrorSelector", &self.ErrorSelector)
            .field("Reserved2", &self.Reserved2)
            .field("DataOffset", &self.DataOffset)
            .field("DataSelector", &self.DataSelector)
            .field("Reserved3", &self.Reserved3)
            .field("MxCsr", &self.MxCsr)
            .field("MxCsr_Mask", &self.MxCsr_Mask)
            .field("FloatRegisters", &self.FloatRegisters)
            .field("XmmRegisters", &self.XmmRegisters)
            .field("Reserved4", &self.Reserved4)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows::core::TypeKind for XSAVE_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for XSAVE_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.ControlWord == other.ControlWord && self.StatusWord == other.StatusWord && self.TagWord == other.TagWord && self.Reserved1 == other.Reserved1 && self.ErrorOpcode == other.ErrorOpcode && self.ErrorOffset == other.ErrorOffset && self.ErrorSelector == other.ErrorSelector && self.Reserved2 == other.Reserved2 && self.DataOffset == other.DataOffset && self.DataSelector == other.DataSelector && self.Reserved3 == other.Reserved3 && self.MxCsr == other.MxCsr && self.MxCsr_Mask == other.MxCsr_Mask && self.FloatRegisters == other.FloatRegisters && self.XmmRegisters == other.XmmRegisters && self.Reserved4 == other.Reserved4
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for XSAVE_FORMAT {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for XSAVE_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86")]
pub struct XSAVE_FORMAT {
    pub ControlWord: u16,
    pub StatusWord: u16,
    pub TagWord: u8,
    pub Reserved1: u8,
    pub ErrorOpcode: u16,
    pub ErrorOffset: u32,
    pub ErrorSelector: u16,
    pub Reserved2: u16,
    pub DataOffset: u32,
    pub DataSelector: u16,
    pub Reserved3: u16,
    pub MxCsr: u32,
    pub MxCsr_Mask: u32,
    pub FloatRegisters: [M128A; 8],
    pub XmmRegisters: [M128A; 8],
    pub Reserved4: [u8; 224],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for XSAVE_FORMAT {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for XSAVE_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for XSAVE_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XSAVE_FORMAT")
            .field("ControlWord", &self.ControlWord)
            .field("StatusWord", &self.StatusWord)
            .field("TagWord", &self.TagWord)
            .field("Reserved1", &self.Reserved1)
            .field("ErrorOpcode", &self.ErrorOpcode)
            .field("ErrorOffset", &self.ErrorOffset)
            .field("ErrorSelector", &self.ErrorSelector)
            .field("Reserved2", &self.Reserved2)
            .field("DataOffset", &self.DataOffset)
            .field("DataSelector", &self.DataSelector)
            .field("Reserved3", &self.Reserved3)
            .field("MxCsr", &self.MxCsr)
            .field("MxCsr_Mask", &self.MxCsr_Mask)
            .field("FloatRegisters", &self.FloatRegisters)
            .field("XmmRegisters", &self.XmmRegisters)
            .field("Reserved4", &self.Reserved4)
            .finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::windows::core::TypeKind for XSAVE_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for XSAVE_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.ControlWord == other.ControlWord && self.StatusWord == other.StatusWord && self.TagWord == other.TagWord && self.Reserved1 == other.Reserved1 && self.ErrorOpcode == other.ErrorOpcode && self.ErrorOffset == other.ErrorOffset && self.ErrorSelector == other.ErrorSelector && self.Reserved2 == other.Reserved2 && self.DataOffset == other.DataOffset && self.DataSelector == other.DataSelector && self.Reserved3 == other.Reserved3 && self.MxCsr == other.MxCsr && self.MxCsr_Mask == other.MxCsr_Mask && self.FloatRegisters == other.FloatRegisters && self.XmmRegisters == other.XmmRegisters && self.Reserved4 == other.Reserved4
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for XSAVE_FORMAT {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for XSAVE_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct XSTATE_CONFIGURATION {
    pub EnabledFeatures: u64,
    pub EnabledVolatileFeatures: u64,
    pub Size: u32,
    pub Anonymous: XSTATE_CONFIGURATION_0,
    pub Features: [XSTATE_FEATURE; 64],
    pub EnabledSupervisorFeatures: u64,
    pub AlignedFeatures: u64,
    pub AllFeatureSize: u32,
    pub AllFeatures: [u32; 64],
    pub EnabledUserVisibleSupervisorFeatures: u64,
    pub ExtendedFeatureDisableFeatures: u64,
    pub AllNonLargeFeatureSize: u32,
    pub Spare: u32,
}
impl ::core::marker::Copy for XSTATE_CONFIGURATION {}
impl ::core::clone::Clone for XSTATE_CONFIGURATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for XSTATE_CONFIGURATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for XSTATE_CONFIGURATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub union XSTATE_CONFIGURATION_0 {
    pub ControlFlags: u32,
    pub Anonymous: XSTATE_CONFIGURATION_0_0,
}
impl ::core::marker::Copy for XSTATE_CONFIGURATION_0 {}
impl ::core::clone::Clone for XSTATE_CONFIGURATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for XSTATE_CONFIGURATION_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for XSTATE_CONFIGURATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct XSTATE_CONFIGURATION_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for XSTATE_CONFIGURATION_0_0 {}
impl ::core::clone::Clone for XSTATE_CONFIGURATION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XSTATE_CONFIGURATION_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XSTATE_CONFIGURATION_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows::core::TypeKind for XSTATE_CONFIGURATION_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for XSTATE_CONFIGURATION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for XSTATE_CONFIGURATION_0_0 {}
impl ::core::default::Default for XSTATE_CONFIGURATION_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct XSTATE_CONFIG_FEATURE_MSC_INFO {
    pub SizeOfInfo: u32,
    pub ContextSize: u32,
    pub EnabledFeatures: u64,
    pub Features: [XSTATE_FEATURE; 64],
}
impl ::core::marker::Copy for XSTATE_CONFIG_FEATURE_MSC_INFO {}
impl ::core::clone::Clone for XSTATE_CONFIG_FEATURE_MSC_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for XSTATE_CONFIG_FEATURE_MSC_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for XSTATE_CONFIG_FEATURE_MSC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct XSTATE_CONTEXT {
    pub Mask: u64,
    pub Length: u32,
    pub Reserved1: u32,
    pub Area: *mut XSAVE_AREA,
    pub Buffer: *mut ::core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for XSTATE_CONTEXT {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for XSTATE_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for XSTATE_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XSTATE_CONTEXT").field("Mask", &self.Mask).field("Length", &self.Length).field("Reserved1", &self.Reserved1).field("Area", &self.Area).field("Buffer", &self.Buffer).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows::core::TypeKind for XSTATE_CONTEXT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for XSTATE_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.Mask == other.Mask && self.Length == other.Length && self.Reserved1 == other.Reserved1 && self.Area == other.Area && self.Buffer == other.Buffer
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for XSTATE_CONTEXT {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for XSTATE_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86")]
pub struct XSTATE_CONTEXT {
    pub Mask: u64,
    pub Length: u32,
    pub Reserved1: u32,
    pub Area: *mut XSAVE_AREA,
    pub Reserved2: u32,
    pub Buffer: *mut ::core::ffi::c_void,
    pub Reserved3: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for XSTATE_CONTEXT {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for XSTATE_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for XSTATE_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XSTATE_CONTEXT").field("Mask", &self.Mask).field("Length", &self.Length).field("Reserved1", &self.Reserved1).field("Area", &self.Area).field("Reserved2", &self.Reserved2).field("Buffer", &self.Buffer).field("Reserved3", &self.Reserved3).finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::windows::core::TypeKind for XSTATE_CONTEXT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for XSTATE_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.Mask == other.Mask && self.Length == other.Length && self.Reserved1 == other.Reserved1 && self.Area == other.Area && self.Reserved2 == other.Reserved2 && self.Buffer == other.Buffer && self.Reserved3 == other.Reserved3
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for XSTATE_CONTEXT {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for XSTATE_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub struct XSTATE_FEATURE {
    pub Offset: u32,
    pub Size: u32,
}
impl ::core::marker::Copy for XSTATE_FEATURE {}
impl ::core::clone::Clone for XSTATE_FEATURE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XSTATE_FEATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XSTATE_FEATURE").field("Offset", &self.Offset).field("Size", &self.Size).finish()
    }
}
impl ::windows::core::TypeKind for XSTATE_FEATURE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for XSTATE_FEATURE {
    fn eq(&self, other: &Self) -> bool {
        self.Offset == other.Offset && self.Size == other.Size
    }
}
impl ::core::cmp::Eq for XSTATE_FEATURE {}
impl ::core::default::Default for XSTATE_FEATURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DIGEST_FUNCTION = ::core::option::Option<unsafe extern "system" fn(refdata: *mut ::core::ffi::c_void, pdata: *mut u8, dwlength: u32) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub type LPCALL_BACK_USER_INTERRUPT_ROUTINE = ::core::option::Option<unsafe extern "system" fn() -> u32>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type LPTOP_LEVEL_EXCEPTION_FILTER = ::core::option::Option<unsafe extern "system" fn(exceptioninfo: *const EXCEPTION_POINTERS) -> i32>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_FileSystem\"`, `\"Win32_System_Kernel\"`, `\"Win32_System_Memory\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_Kernel", feature = "Win32_System_Memory"))]
pub type MINIDUMP_CALLBACK_ROUTINE = ::core::option::Option<unsafe extern "system" fn(callbackparam: *mut ::core::ffi::c_void, callbackinput: *const MINIDUMP_CALLBACK_INPUT, callbackoutput: *mut MINIDUMP_CALLBACK_OUTPUT) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub type PCOGETACTIVATIONSTATE = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::GUID, param1: u32, param2: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub type PCOGETCALLSTATE = ::core::option::Option<unsafe extern "system" fn(param0: i32, param1: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDBGHELP_CREATE_USER_DUMP_CALLBACK = ::core::option::Option<unsafe extern "system" fn(datatype: u32, data: *const *const ::core::ffi::c_void, datalength: *mut u32, userdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PENUMDIRTREE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(filepath: ::windows::core::PCSTR, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PENUMDIRTREE_CALLBACKW = ::core::option::Option<unsafe extern "system" fn(filepath: ::windows::core::PCWSTR, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub type PENUMLOADED_MODULES_CALLBACK = ::core::option::Option<unsafe extern "system" fn(modulename: ::windows::core::PCSTR, modulebase: u32, modulesize: u32, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PENUMLOADED_MODULES_CALLBACK64 = ::core::option::Option<unsafe extern "system" fn(modulename: ::windows::core::PCSTR, modulebase: u64, modulesize: u32, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PENUMLOADED_MODULES_CALLBACKW64 = ::core::option::Option<unsafe extern "system" fn(modulename: ::windows::core::PCWSTR, modulebase: u64, modulesize: u32, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PENUMSOURCEFILETOKENSCALLBACK = ::core::option::Option<unsafe extern "system" fn(token: *const ::core::ffi::c_void, size: usize) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFINDFILEINPATHCALLBACK = ::core::option::Option<unsafe extern "system" fn(filename: ::windows::core::PCSTR, context: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFINDFILEINPATHCALLBACKW = ::core::option::Option<unsafe extern "system" fn(filename: ::windows::core::PCWSTR, context: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFIND_DEBUG_FILE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(filehandle: super::super::super::Foundation::HANDLE, filename: ::windows::core::PCSTR, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFIND_DEBUG_FILE_CALLBACKW = ::core::option::Option<unsafe extern "system" fn(filehandle: super::super::super::Foundation::HANDLE, filename: ::windows::core::PCWSTR, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFIND_EXE_FILE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(filehandle: super::super::super::Foundation::HANDLE, filename: ::windows::core::PCSTR, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFIND_EXE_FILE_CALLBACKW = ::core::option::Option<unsafe extern "system" fn(filehandle: super::super::super::Foundation::HANDLE, filename: ::windows::core::PCWSTR, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub type PFUNCTION_TABLE_ACCESS_ROUTINE = ::core::option::Option<unsafe extern "system" fn(hprocess: super::super::super::Foundation::HANDLE, addrbase: u32) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFUNCTION_TABLE_ACCESS_ROUTINE64 = ::core::option::Option<unsafe extern "system" fn(ahprocess: super::super::super::Foundation::HANDLE, addrbase: u64) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub type PGET_MODULE_BASE_ROUTINE = ::core::option::Option<unsafe extern "system" fn(hprocess: super::super::super::Foundation::HANDLE, address: u32) -> u32>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PGET_MODULE_BASE_ROUTINE64 = ::core::option::Option<unsafe extern "system" fn(hprocess: super::super::super::Foundation::HANDLE, address: u64) -> u64>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "aarch64")]
pub type PGET_RUNTIME_FUNCTION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(controlpc: u64, context: *const ::core::ffi::c_void) -> *mut IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86_64")]
pub type PGET_RUNTIME_FUNCTION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(controlpc: u64, context: *const ::core::ffi::c_void) -> *mut IMAGE_RUNTIME_FUNCTION_ENTRY>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PGET_TARGET_ATTRIBUTE_VALUE64 = ::core::option::Option<unsafe extern "system" fn(hprocess: super::super::super::Foundation::HANDLE, attribute: u32, attributedata: u64, attributevalue: *mut u64) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PIMAGEHLP_STATUS_ROUTINE = ::core::option::Option<unsafe extern "system" fn(reason: IMAGEHLP_STATUS_REASON, imagename: ::windows::core::PCSTR, dllname: ::windows::core::PCSTR, va: usize, parameter: usize) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PIMAGEHLP_STATUS_ROUTINE32 = ::core::option::Option<unsafe extern "system" fn(reason: IMAGEHLP_STATUS_REASON, imagename: ::windows::core::PCSTR, dllname: ::windows::core::PCSTR, va: u32, parameter: usize) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PIMAGEHLP_STATUS_ROUTINE64 = ::core::option::Option<unsafe extern "system" fn(reason: IMAGEHLP_STATUS_REASON, imagename: ::windows::core::PCSTR, dllname: ::windows::core::PCSTR, va: u64, parameter: usize) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub type PREAD_PROCESS_MEMORY_ROUTINE = ::core::option::Option<unsafe extern "system" fn(hprocess: super::super::super::Foundation::HANDLE, lpbaseaddress: u32, lpbuffer: *mut ::core::ffi::c_void, nsize: u32, lpnumberofbytesread: *mut u32) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PREAD_PROCESS_MEMORY_ROUTINE64 = ::core::option::Option<unsafe extern "system" fn(hprocess: super::super::super::Foundation::HANDLE, qwbaseaddress: u64, lpbuffer: *mut ::core::ffi::c_void, nsize: u32, lpnumberofbytesread: *mut u32) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERBYINDEXPROC = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCSTR, param1: ::windows::core::PCSTR, param2: ::windows::core::PCSTR, param3: ::windows::core::PCSTR) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERBYINDEXPROCA = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCSTR, param1: ::windows::core::PCSTR, param2: ::windows::core::PCSTR, param3: ::windows::core::PCSTR) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERBYINDEXPROCW = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCWSTR, param1: ::windows::core::PCWSTR, param2: ::windows::core::PCWSTR, param3: ::windows::core::PCWSTR) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERCALLBACKPROC = ::core::option::Option<unsafe extern "system" fn(action: usize, data: u64, context: u64) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERCLOSEPROC = ::core::option::Option<unsafe extern "system" fn() -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERDELTANAME = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCSTR, param1: *mut ::core::ffi::c_void, param2: u32, param3: u32, param4: *mut ::core::ffi::c_void, param5: u32, param6: u32, param7: ::windows::core::PCSTR, param8: usize) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERDELTANAMEW = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCWSTR, param1: *mut ::core::ffi::c_void, param2: u32, param3: u32, param4: *mut ::core::ffi::c_void, param5: u32, param6: u32, param7: ::windows::core::PCWSTR, param8: usize) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERGETINDEXSTRING = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: u32, param2: u32, param3: ::windows::core::PCSTR, param4: usize) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERGETINDEXSTRINGW = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: u32, param2: u32, param3: ::windows::core::PCWSTR, param4: usize) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERGETOPTIONDATAPROC = ::core::option::Option<unsafe extern "system" fn(param0: usize, param1: *mut u64) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub type PSYMBOLSERVERGETOPTIONSPROC = ::core::option::Option<unsafe extern "system" fn() -> usize>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERGETSUPPLEMENT = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCSTR, param1: ::windows::core::PCSTR, param2: ::windows::core::PCSTR, param3: ::windows::core::PCSTR, param4: usize) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERGETSUPPLEMENTW = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCWSTR, param1: ::windows::core::PCWSTR, param2: ::windows::core::PCWSTR, param3: ::windows::core::PCWSTR, param4: usize) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERGETVERSION = ::core::option::Option<unsafe extern "system" fn(param0: *mut API_VERSION) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERISSTORE = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCSTR) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERISSTOREW = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCWSTR) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERMESSAGEPROC = ::core::option::Option<unsafe extern "system" fn(action: usize, data: u64, context: u64) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVEROPENPROC = ::core::option::Option<unsafe extern "system" fn() -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERPINGPROC = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCSTR) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERPINGPROCA = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCSTR) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERPINGPROCW = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCWSTR) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERPINGPROCWEX = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCWSTR) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERPROC = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCSTR, param1: ::windows::core::PCSTR, param2: *mut ::core::ffi::c_void, param3: u32, param4: u32, param5: ::windows::core::PCSTR) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERPROCA = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCSTR, param1: ::windows::core::PCSTR, param2: *mut ::core::ffi::c_void, param3: u32, param4: u32, param5: ::windows::core::PCSTR) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERPROCW = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCWSTR, param1: ::windows::core::PCWSTR, param2: *mut ::core::ffi::c_void, param3: u32, param4: u32, param5: ::windows::core::PCWSTR) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERSETHTTPAUTHHEADER = ::core::option::Option<unsafe extern "system" fn(pszauthheader: ::windows::core::PCWSTR) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERSETOPTIONSPROC = ::core::option::Option<unsafe extern "system" fn(param0: usize, param1: u64) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERSETOPTIONSWPROC = ::core::option::Option<unsafe extern "system" fn(param0: usize, param1: u64) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERSTOREFILE = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCSTR, param1: ::windows::core::PCSTR, param2: *mut ::core::ffi::c_void, param3: u32, param4: u32, param5: ::windows::core::PCSTR, param6: usize, param7: u32) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERSTOREFILEW = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCWSTR, param1: ::windows::core::PCWSTR, param2: *mut ::core::ffi::c_void, param3: u32, param4: u32, param5: ::windows::core::PCWSTR, param6: usize, param7: u32) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERSTORESUPPLEMENT = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCSTR, param1: ::windows::core::PCSTR, param2: ::windows::core::PCSTR, param3: ::windows::core::PCSTR, param4: usize, param5: u32) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERSTORESUPPLEMENTW = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCWSTR, param1: ::windows::core::PCWSTR, param2: ::windows::core::PCWSTR, param3: ::windows::core::PCWSTR, param4: usize, param5: u32) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub type PSYMBOLSERVERVERSION = ::core::option::Option<unsafe extern "system" fn() -> u32>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERWEXPROC = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCWSTR, param1: ::windows::core::PCWSTR, param2: *mut ::core::ffi::c_void, param3: u32, param4: u32, param5: ::windows::core::PCWSTR, param6: *mut SYMSRV_EXTENDED_OUTPUT_DATA) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOL_FUNCENTRY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(hprocess: super::super::super::Foundation::HANDLE, addrbase: u32, usercontext: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOL_FUNCENTRY_CALLBACK64 = ::core::option::Option<unsafe extern "system" fn(hprocess: super::super::super::Foundation::HANDLE, addrbase: u64, usercontext: u64) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOL_REGISTERED_CALLBACK = ::core::option::Option<unsafe extern "system" fn(hprocess: super::super::super::Foundation::HANDLE, actioncode: u32, callbackdata: *const ::core::ffi::c_void, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOL_REGISTERED_CALLBACK64 = ::core::option::Option<unsafe extern "system" fn(hprocess: super::super::super::Foundation::HANDLE, actioncode: u32, callbackdata: u64, usercontext: u64) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYM_ENUMERATESYMBOLS_CALLBACK = ::core::option::Option<unsafe extern "system" fn(psyminfo: *const SYMBOL_INFO, symbolsize: u32, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYM_ENUMERATESYMBOLS_CALLBACKW = ::core::option::Option<unsafe extern "system" fn(psyminfo: *const SYMBOL_INFOW, symbolsize: u32, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYM_ENUMLINES_CALLBACK = ::core::option::Option<unsafe extern "system" fn(lineinfo: *const SRCCODEINFO, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYM_ENUMLINES_CALLBACKW = ::core::option::Option<unsafe extern "system" fn(lineinfo: *const SRCCODEINFOW, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub type PSYM_ENUMMODULES_CALLBACK = ::core::option::Option<unsafe extern "system" fn(modulename: ::windows::core::PCSTR, baseofdll: u32, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYM_ENUMMODULES_CALLBACK64 = ::core::option::Option<unsafe extern "system" fn(modulename: ::windows::core::PCSTR, baseofdll: u64, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYM_ENUMMODULES_CALLBACKW64 = ::core::option::Option<unsafe extern "system" fn(modulename: ::windows::core::PCWSTR, baseofdll: u64, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYM_ENUMPROCESSES_CALLBACK = ::core::option::Option<unsafe extern "system" fn(hprocess: super::super::super::Foundation::HANDLE, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYM_ENUMSOURCEFILES_CALLBACK = ::core::option::Option<unsafe extern "system" fn(psourcefile: *const SOURCEFILE, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYM_ENUMSOURCEFILES_CALLBACKW = ::core::option::Option<unsafe extern "system" fn(psourcefile: *const SOURCEFILEW, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub type PSYM_ENUMSYMBOLS_CALLBACK = ::core::option::Option<unsafe extern "system" fn(symbolname: ::windows::core::PCSTR, symboladdress: u32, symbolsize: u32, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYM_ENUMSYMBOLS_CALLBACK64 = ::core::option::Option<unsafe extern "system" fn(symbolname: ::windows::core::PCSTR, symboladdress: u64, symbolsize: u32, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSYM_ENUMSYMBOLS_CALLBACK64W = ::core::option::Option<unsafe extern "system" fn(symbolname: ::windows::core::PCWSTR, symboladdress: u64, symbolsize: u32, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub type PSYM_ENUMSYMBOLS_CALLBACKW = ::core::option::Option<unsafe extern "system" fn(symbolname: ::windows::core::PCWSTR, symboladdress: u32, symbolsize: u32, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub type PTRANSLATE_ADDRESS_ROUTINE = ::core::option::Option<unsafe extern "system" fn(hprocess: super::super::super::Foundation::HANDLE, hthread: super::super::super::Foundation::HANDLE, lpaddr: *mut ADDRESS) -> u32>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PTRANSLATE_ADDRESS_ROUTINE64 = ::core::option::Option<unsafe extern "system" fn(hprocess: super::super::super::Foundation::HANDLE, hthread: super::super::super::Foundation::HANDLE, lpaddr: *const ADDRESS64) -> u64>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type PVECTORED_EXCEPTION_HANDLER = ::core::option::Option<unsafe extern "system" fn(exceptioninfo: *mut EXCEPTION_POINTERS) -> i32>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PWAITCHAINCALLBACK = ::core::option::Option<unsafe extern "system" fn(wcthandle: *mut ::core::ffi::c_void, context: usize, callbackstatus: u32, nodecount: *mut u32, nodeinfoarray: *mut WAITCHAIN_NODE_INFO, iscycle: *mut i32) -> ()>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SYMADDSOURCESTREAM = ::core::option::Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HANDLE, param1: u64, param2: ::windows::core::PCSTR, param3: *mut u8, param4: usize) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SYMADDSOURCESTREAMA = ::core::option::Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HANDLE, param1: u64, param2: ::windows::core::PCSTR, param3: *mut u8, param4: usize) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WHEA_ERROR_SOURCE_CORRECT_DEVICE_DRIVER = ::core::option::Option<unsafe extern "system" fn(errorsourcedesc: *mut ::core::ffi::c_void, maximumsectionlength: *mut u32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WHEA_ERROR_SOURCE_INITIALIZE_DEVICE_DRIVER = ::core::option::Option<unsafe extern "system" fn(context: *mut ::core::ffi::c_void, errorsourceid: u32) -> super::super::super::Foundation::NTSTATUS>;
#[doc = "*Required features: `\"Win32_System_Diagnostics_Debug\"`*"]
pub type WHEA_ERROR_SOURCE_UNINITIALIZE_DEVICE_DRIVER = ::core::option::Option<unsafe extern "system" fn(context: *mut ::core::ffi::c_void) -> ()>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
