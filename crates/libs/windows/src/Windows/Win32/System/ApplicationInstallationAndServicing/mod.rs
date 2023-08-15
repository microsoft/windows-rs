#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ActivateActCtx<P0>(hactctx: P0, lpcookie: *mut usize) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("kernel32.dll" "system" fn ActivateActCtx(hactctx : super::super::Foundation:: HANDLE, lpcookie : *mut usize) -> super::super::Foundation:: BOOL);
    ActivateActCtx(hactctx.into_param().abi(), lpcookie).ok()
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddRefActCtx<P0>(hactctx: P0)
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("kernel32.dll" "system" fn AddRefActCtx(hactctx : super::super::Foundation:: HANDLE) -> ());
    AddRefActCtx(hactctx.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyDeltaA<P0, P1, P2>(applyflags: i64, lpsourcename: P0, lpdeltaname: P1, lptargetname: P2) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msdelta.dll" "system" fn ApplyDeltaA(applyflags : i64, lpsourcename : ::windows_core::PCSTR, lpdeltaname : ::windows_core::PCSTR, lptargetname : ::windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    ApplyDeltaA(applyflags, lpsourcename.into_param().abi(), lpdeltaname.into_param().abi(), lptargetname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyDeltaB(applyflags: i64, source: DELTA_INPUT, delta: DELTA_INPUT, lptarget: *mut DELTA_OUTPUT) -> super::super::Foundation::BOOL {
    ::windows_targets::link!("msdelta.dll" "system" fn ApplyDeltaB(applyflags : i64, source : DELTA_INPUT, delta : DELTA_INPUT, lptarget : *mut DELTA_OUTPUT) -> super::super::Foundation:: BOOL);
    ApplyDeltaB(applyflags, ::core::mem::transmute(source), ::core::mem::transmute(delta), lptarget)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyDeltaGetReverseB(applyflags: i64, source: DELTA_INPUT, delta: DELTA_INPUT, lpreversefiletime: ::core::option::Option<*const super::super::Foundation::FILETIME>, lptarget: *mut DELTA_OUTPUT, lptargetreverse: *mut DELTA_OUTPUT) -> super::super::Foundation::BOOL {
    ::windows_targets::link!("msdelta.dll" "system" fn ApplyDeltaGetReverseB(applyflags : i64, source : DELTA_INPUT, delta : DELTA_INPUT, lpreversefiletime : *const super::super::Foundation:: FILETIME, lptarget : *mut DELTA_OUTPUT, lptargetreverse : *mut DELTA_OUTPUT) -> super::super::Foundation:: BOOL);
    ApplyDeltaGetReverseB(applyflags, ::core::mem::transmute(source), ::core::mem::transmute(delta), ::core::mem::transmute(lpreversefiletime.unwrap_or(::std::ptr::null())), lptarget, lptargetreverse)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyDeltaProvidedB(applyflags: i64, source: DELTA_INPUT, delta: DELTA_INPUT, lptarget: *mut ::core::ffi::c_void, utargetsize: usize) -> super::super::Foundation::BOOL {
    ::windows_targets::link!("msdelta.dll" "system" fn ApplyDeltaProvidedB(applyflags : i64, source : DELTA_INPUT, delta : DELTA_INPUT, lptarget : *mut ::core::ffi::c_void, utargetsize : usize) -> super::super::Foundation:: BOOL);
    ApplyDeltaProvidedB(applyflags, ::core::mem::transmute(source), ::core::mem::transmute(delta), lptarget, utargetsize)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyDeltaW<P0, P1, P2>(applyflags: i64, lpsourcename: P0, lpdeltaname: P1, lptargetname: P2) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msdelta.dll" "system" fn ApplyDeltaW(applyflags : i64, lpsourcename : ::windows_core::PCWSTR, lpdeltaname : ::windows_core::PCWSTR, lptargetname : ::windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    ApplyDeltaW(applyflags, lpsourcename.into_param().abi(), lpdeltaname.into_param().abi(), lptargetname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyPatchToFileA<P0, P1, P2>(patchfilename: P0, oldfilename: P1, newfilename: P2, applyoptionflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("mspatcha.dll" "system" fn ApplyPatchToFileA(patchfilename : ::windows_core::PCSTR, oldfilename : ::windows_core::PCSTR, newfilename : ::windows_core::PCSTR, applyoptionflags : u32) -> super::super::Foundation:: BOOL);
    ApplyPatchToFileA(patchfilename.into_param().abi(), oldfilename.into_param().abi(), newfilename.into_param().abi(), applyoptionflags)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyPatchToFileByBuffers(patchfilemapped: &[u8], oldfilemapped: ::core::option::Option<&[u8]>, newfilebuffer: &mut [u8], newfileactualsize: ::core::option::Option<*mut u32>, newfiletime: ::core::option::Option<*mut super::super::Foundation::FILETIME>, applyoptionflags: u32, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL {
    ::windows_targets::link!("mspatcha.dll" "system" fn ApplyPatchToFileByBuffers(patchfilemapped : *const u8, patchfilesize : u32, oldfilemapped : *const u8, oldfilesize : u32, newfilebuffer : *mut *mut u8, newfilebuffersize : u32, newfileactualsize : *mut u32, newfiletime : *mut super::super::Foundation:: FILETIME, applyoptionflags : u32, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : *const ::core::ffi::c_void) -> super::super::Foundation:: BOOL);
    ApplyPatchToFileByBuffers(
        ::core::mem::transmute(patchfilemapped.as_ptr()),
        patchfilemapped.len() as _,
        ::core::mem::transmute(oldfilemapped.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        oldfilemapped.as_deref().map_or(0, |slice| slice.len() as _),
        ::core::mem::transmute(newfilebuffer.as_ptr()),
        newfilebuffer.len() as _,
        ::core::mem::transmute(newfileactualsize.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(newfiletime.unwrap_or(::std::ptr::null_mut())),
        applyoptionflags,
        progresscallback,
        ::core::mem::transmute(callbackcontext.unwrap_or(::std::ptr::null())),
    )
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyPatchToFileByHandles<P0, P1, P2>(patchfilehandle: P0, oldfilehandle: P1, newfilehandle: P2, applyoptionflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P2: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("mspatcha.dll" "system" fn ApplyPatchToFileByHandles(patchfilehandle : super::super::Foundation:: HANDLE, oldfilehandle : super::super::Foundation:: HANDLE, newfilehandle : super::super::Foundation:: HANDLE, applyoptionflags : u32) -> super::super::Foundation:: BOOL);
    ApplyPatchToFileByHandles(patchfilehandle.into_param().abi(), oldfilehandle.into_param().abi(), newfilehandle.into_param().abi(), applyoptionflags)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyPatchToFileByHandlesEx<P0, P1, P2>(patchfilehandle: P0, oldfilehandle: P1, newfilehandle: P2, applyoptionflags: u32, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P2: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("mspatcha.dll" "system" fn ApplyPatchToFileByHandlesEx(patchfilehandle : super::super::Foundation:: HANDLE, oldfilehandle : super::super::Foundation:: HANDLE, newfilehandle : super::super::Foundation:: HANDLE, applyoptionflags : u32, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : *const ::core::ffi::c_void) -> super::super::Foundation:: BOOL);
    ApplyPatchToFileByHandlesEx(patchfilehandle.into_param().abi(), oldfilehandle.into_param().abi(), newfilehandle.into_param().abi(), applyoptionflags, progresscallback, ::core::mem::transmute(callbackcontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyPatchToFileExA<P0, P1, P2>(patchfilename: P0, oldfilename: P1, newfilename: P2, applyoptionflags: u32, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("mspatcha.dll" "system" fn ApplyPatchToFileExA(patchfilename : ::windows_core::PCSTR, oldfilename : ::windows_core::PCSTR, newfilename : ::windows_core::PCSTR, applyoptionflags : u32, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : *const ::core::ffi::c_void) -> super::super::Foundation:: BOOL);
    ApplyPatchToFileExA(patchfilename.into_param().abi(), oldfilename.into_param().abi(), newfilename.into_param().abi(), applyoptionflags, progresscallback, ::core::mem::transmute(callbackcontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyPatchToFileExW<P0, P1, P2>(patchfilename: P0, oldfilename: P1, newfilename: P2, applyoptionflags: u32, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("mspatcha.dll" "system" fn ApplyPatchToFileExW(patchfilename : ::windows_core::PCWSTR, oldfilename : ::windows_core::PCWSTR, newfilename : ::windows_core::PCWSTR, applyoptionflags : u32, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : *const ::core::ffi::c_void) -> super::super::Foundation:: BOOL);
    ApplyPatchToFileExW(patchfilename.into_param().abi(), oldfilename.into_param().abi(), newfilename.into_param().abi(), applyoptionflags, progresscallback, ::core::mem::transmute(callbackcontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplyPatchToFileW<P0, P1, P2>(patchfilename: P0, oldfilename: P1, newfilename: P2, applyoptionflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("mspatcha.dll" "system" fn ApplyPatchToFileW(patchfilename : ::windows_core::PCWSTR, oldfilename : ::windows_core::PCWSTR, newfilename : ::windows_core::PCWSTR, applyoptionflags : u32) -> super::super::Foundation:: BOOL);
    ApplyPatchToFileW(patchfilename.into_param().abi(), oldfilename.into_param().abi(), newfilename.into_param().abi(), applyoptionflags)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateActCtxA(pactctx: *const ACTCTXA) -> ::windows_core::Result<super::super::Foundation::HANDLE> {
    ::windows_targets::link!("kernel32.dll" "system" fn CreateActCtxA(pactctx : *const ACTCTXA) -> super::super::Foundation:: HANDLE);
    let result__ = CreateActCtxA(pactctx);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows_core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateActCtxW(pactctx: *const ACTCTXW) -> ::windows_core::Result<super::super::Foundation::HANDLE> {
    ::windows_targets::link!("kernel32.dll" "system" fn CreateActCtxW(pactctx : *const ACTCTXW) -> super::super::Foundation:: HANDLE);
    let result__ = CreateActCtxW(pactctx);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows_core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateDeltaA<P0, P1, P2, P3, P4>(filetypeset: i64, setflags: i64, resetflags: i64, lpsourcename: P0, lptargetname: P1, lpsourceoptionsname: P2, lptargetoptionsname: P3, globaloptions: DELTA_INPUT, lptargetfiletime: ::core::option::Option<*const super::super::Foundation::FILETIME>, hashalgid: u32, lpdeltaname: P4) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P4: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msdelta.dll" "system" fn CreateDeltaA(filetypeset : i64, setflags : i64, resetflags : i64, lpsourcename : ::windows_core::PCSTR, lptargetname : ::windows_core::PCSTR, lpsourceoptionsname : ::windows_core::PCSTR, lptargetoptionsname : ::windows_core::PCSTR, globaloptions : DELTA_INPUT, lptargetfiletime : *const super::super::Foundation:: FILETIME, hashalgid : u32, lpdeltaname : ::windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    CreateDeltaA(filetypeset, setflags, resetflags, lpsourcename.into_param().abi(), lptargetname.into_param().abi(), lpsourceoptionsname.into_param().abi(), lptargetoptionsname.into_param().abi(), ::core::mem::transmute(globaloptions), ::core::mem::transmute(lptargetfiletime.unwrap_or(::std::ptr::null())), hashalgid, lpdeltaname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateDeltaB(filetypeset: i64, setflags: i64, resetflags: i64, source: DELTA_INPUT, target: DELTA_INPUT, sourceoptions: DELTA_INPUT, targetoptions: DELTA_INPUT, globaloptions: DELTA_INPUT, lptargetfiletime: ::core::option::Option<*const super::super::Foundation::FILETIME>, hashalgid: u32, lpdelta: *mut DELTA_OUTPUT) -> super::super::Foundation::BOOL {
    ::windows_targets::link!("msdelta.dll" "system" fn CreateDeltaB(filetypeset : i64, setflags : i64, resetflags : i64, source : DELTA_INPUT, target : DELTA_INPUT, sourceoptions : DELTA_INPUT, targetoptions : DELTA_INPUT, globaloptions : DELTA_INPUT, lptargetfiletime : *const super::super::Foundation:: FILETIME, hashalgid : u32, lpdelta : *mut DELTA_OUTPUT) -> super::super::Foundation:: BOOL);
    CreateDeltaB(filetypeset, setflags, resetflags, ::core::mem::transmute(source), ::core::mem::transmute(target), ::core::mem::transmute(sourceoptions), ::core::mem::transmute(targetoptions), ::core::mem::transmute(globaloptions), ::core::mem::transmute(lptargetfiletime.unwrap_or(::std::ptr::null())), hashalgid, lpdelta)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateDeltaW<P0, P1, P2, P3, P4>(filetypeset: i64, setflags: i64, resetflags: i64, lpsourcename: P0, lptargetname: P1, lpsourceoptionsname: P2, lptargetoptionsname: P3, globaloptions: DELTA_INPUT, lptargetfiletime: ::core::option::Option<*const super::super::Foundation::FILETIME>, hashalgid: u32, lpdeltaname: P4) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P4: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msdelta.dll" "system" fn CreateDeltaW(filetypeset : i64, setflags : i64, resetflags : i64, lpsourcename : ::windows_core::PCWSTR, lptargetname : ::windows_core::PCWSTR, lpsourceoptionsname : ::windows_core::PCWSTR, lptargetoptionsname : ::windows_core::PCWSTR, globaloptions : DELTA_INPUT, lptargetfiletime : *const super::super::Foundation:: FILETIME, hashalgid : u32, lpdeltaname : ::windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    CreateDeltaW(filetypeset, setflags, resetflags, lpsourcename.into_param().abi(), lptargetname.into_param().abi(), lpsourceoptionsname.into_param().abi(), lptargetoptionsname.into_param().abi(), ::core::mem::transmute(globaloptions), ::core::mem::transmute(lptargetfiletime.unwrap_or(::std::ptr::null())), hashalgid, lpdeltaname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreatePatchFileA<P0, P1, P2>(oldfilename: P0, newfilename: P1, patchfilename: P2, optionflags: u32, optiondata: ::core::option::Option<*const PATCH_OPTION_DATA>) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("mspatchc.dll" "system" fn CreatePatchFileA(oldfilename : ::windows_core::PCSTR, newfilename : ::windows_core::PCSTR, patchfilename : ::windows_core::PCSTR, optionflags : u32, optiondata : *const PATCH_OPTION_DATA) -> super::super::Foundation:: BOOL);
    CreatePatchFileA(oldfilename.into_param().abi(), newfilename.into_param().abi(), patchfilename.into_param().abi(), optionflags, ::core::mem::transmute(optiondata.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreatePatchFileByHandles<P0, P1, P2>(oldfilehandle: P0, newfilehandle: P1, patchfilehandle: P2, optionflags: u32, optiondata: ::core::option::Option<*const PATCH_OPTION_DATA>) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P2: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("mspatchc.dll" "system" fn CreatePatchFileByHandles(oldfilehandle : super::super::Foundation:: HANDLE, newfilehandle : super::super::Foundation:: HANDLE, patchfilehandle : super::super::Foundation:: HANDLE, optionflags : u32, optiondata : *const PATCH_OPTION_DATA) -> super::super::Foundation:: BOOL);
    CreatePatchFileByHandles(oldfilehandle.into_param().abi(), newfilehandle.into_param().abi(), patchfilehandle.into_param().abi(), optionflags, ::core::mem::transmute(optiondata.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreatePatchFileByHandlesEx<P0, P1>(oldfileinfoarray: &[PATCH_OLD_FILE_INFO_H], newfilehandle: P0, patchfilehandle: P1, optionflags: u32, optiondata: ::core::option::Option<*const PATCH_OPTION_DATA>, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("mspatchc.dll" "system" fn CreatePatchFileByHandlesEx(oldfilecount : u32, oldfileinfoarray : *const PATCH_OLD_FILE_INFO_H, newfilehandle : super::super::Foundation:: HANDLE, patchfilehandle : super::super::Foundation:: HANDLE, optionflags : u32, optiondata : *const PATCH_OPTION_DATA, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : *const ::core::ffi::c_void) -> super::super::Foundation:: BOOL);
    CreatePatchFileByHandlesEx(oldfileinfoarray.len() as _, ::core::mem::transmute(oldfileinfoarray.as_ptr()), newfilehandle.into_param().abi(), patchfilehandle.into_param().abi(), optionflags, ::core::mem::transmute(optiondata.unwrap_or(::std::ptr::null())), progresscallback, ::core::mem::transmute(callbackcontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreatePatchFileExA<P0, P1>(oldfileinfoarray: &[PATCH_OLD_FILE_INFO_A], newfilename: P0, patchfilename: P1, optionflags: u32, optiondata: ::core::option::Option<*const PATCH_OPTION_DATA>, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("mspatchc.dll" "system" fn CreatePatchFileExA(oldfilecount : u32, oldfileinfoarray : *const PATCH_OLD_FILE_INFO_A, newfilename : ::windows_core::PCSTR, patchfilename : ::windows_core::PCSTR, optionflags : u32, optiondata : *const PATCH_OPTION_DATA, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : *const ::core::ffi::c_void) -> super::super::Foundation:: BOOL);
    CreatePatchFileExA(oldfileinfoarray.len() as _, ::core::mem::transmute(oldfileinfoarray.as_ptr()), newfilename.into_param().abi(), patchfilename.into_param().abi(), optionflags, ::core::mem::transmute(optiondata.unwrap_or(::std::ptr::null())), progresscallback, ::core::mem::transmute(callbackcontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreatePatchFileExW<P0, P1>(oldfileinfoarray: &[PATCH_OLD_FILE_INFO_W], newfilename: P0, patchfilename: P1, optionflags: u32, optiondata: ::core::option::Option<*const PATCH_OPTION_DATA>, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("mspatchc.dll" "system" fn CreatePatchFileExW(oldfilecount : u32, oldfileinfoarray : *const PATCH_OLD_FILE_INFO_W, newfilename : ::windows_core::PCWSTR, patchfilename : ::windows_core::PCWSTR, optionflags : u32, optiondata : *const PATCH_OPTION_DATA, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : *const ::core::ffi::c_void) -> super::super::Foundation:: BOOL);
    CreatePatchFileExW(oldfileinfoarray.len() as _, ::core::mem::transmute(oldfileinfoarray.as_ptr()), newfilename.into_param().abi(), patchfilename.into_param().abi(), optionflags, ::core::mem::transmute(optiondata.unwrap_or(::std::ptr::null())), progresscallback, ::core::mem::transmute(callbackcontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreatePatchFileW<P0, P1, P2>(oldfilename: P0, newfilename: P1, patchfilename: P2, optionflags: u32, optiondata: ::core::option::Option<*const PATCH_OPTION_DATA>) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("mspatchc.dll" "system" fn CreatePatchFileW(oldfilename : ::windows_core::PCWSTR, newfilename : ::windows_core::PCWSTR, patchfilename : ::windows_core::PCWSTR, optionflags : u32, optiondata : *const PATCH_OPTION_DATA) -> super::super::Foundation:: BOOL);
    CreatePatchFileW(oldfilename.into_param().abi(), newfilename.into_param().abi(), patchfilename.into_param().abi(), optionflags, ::core::mem::transmute(optiondata.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeactivateActCtx(dwflags: u32, ulcookie: usize) -> ::windows_core::Result<()> {
    ::windows_targets::link!("kernel32.dll" "system" fn DeactivateActCtx(dwflags : u32, ulcookie : usize) -> super::super::Foundation:: BOOL);
    DeactivateActCtx(dwflags, ulcookie).ok()
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeltaFree(lpmemory: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows_targets::link!("msdelta.dll" "system" fn DeltaFree(lpmemory : *const ::core::ffi::c_void) -> super::super::Foundation:: BOOL);
    DeltaFree(lpmemory)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeltaNormalizeProvidedB(filetypeset: i64, normalizeflags: i64, normalizeoptions: DELTA_INPUT, lpsource: *mut ::core::ffi::c_void, usourcesize: usize) -> super::super::Foundation::BOOL {
    ::windows_targets::link!("msdelta.dll" "system" fn DeltaNormalizeProvidedB(filetypeset : i64, normalizeflags : i64, normalizeoptions : DELTA_INPUT, lpsource : *mut ::core::ffi::c_void, usourcesize : usize) -> super::super::Foundation:: BOOL);
    DeltaNormalizeProvidedB(filetypeset, normalizeflags, ::core::mem::transmute(normalizeoptions), lpsource, usourcesize)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExtractPatchHeaderToFileA<P0, P1>(patchfilename: P0, patchheaderfilename: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("mspatchc.dll" "system" fn ExtractPatchHeaderToFileA(patchfilename : ::windows_core::PCSTR, patchheaderfilename : ::windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    ExtractPatchHeaderToFileA(patchfilename.into_param().abi(), patchheaderfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExtractPatchHeaderToFileByHandles<P0, P1>(patchfilehandle: P0, patchheaderfilehandle: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("mspatchc.dll" "system" fn ExtractPatchHeaderToFileByHandles(patchfilehandle : super::super::Foundation:: HANDLE, patchheaderfilehandle : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    ExtractPatchHeaderToFileByHandles(patchfilehandle.into_param().abi(), patchheaderfilehandle.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExtractPatchHeaderToFileW<P0, P1>(patchfilename: P0, patchheaderfilename: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("mspatchc.dll" "system" fn ExtractPatchHeaderToFileW(patchfilename : ::windows_core::PCWSTR, patchheaderfilename : ::windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    ExtractPatchHeaderToFileW(patchfilename.into_param().abi(), patchheaderfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
#[inline]
pub unsafe fn FindActCtxSectionGuid(dwflags: u32, lpextensionguid: ::core::option::Option<*const ::windows_core::GUID>, ulsectionid: u32, lpguidtofind: ::core::option::Option<*const ::windows_core::GUID>, returneddata: *mut ACTCTX_SECTION_KEYED_DATA) -> ::windows_core::Result<()> {
    ::windows_targets::link!("kernel32.dll" "system" fn FindActCtxSectionGuid(dwflags : u32, lpextensionguid : *const ::windows_core::GUID, ulsectionid : u32, lpguidtofind : *const ::windows_core::GUID, returneddata : *mut ACTCTX_SECTION_KEYED_DATA) -> super::super::Foundation:: BOOL);
    FindActCtxSectionGuid(dwflags, ::core::mem::transmute(lpextensionguid.unwrap_or(::std::ptr::null())), ulsectionid, ::core::mem::transmute(lpguidtofind.unwrap_or(::std::ptr::null())), returneddata).ok()
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
#[inline]
pub unsafe fn FindActCtxSectionStringA<P0>(dwflags: u32, lpextensionguid: ::core::option::Option<*const ::windows_core::GUID>, ulsectionid: u32, lpstringtofind: P0, returneddata: *mut ACTCTX_SECTION_KEYED_DATA) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("kernel32.dll" "system" fn FindActCtxSectionStringA(dwflags : u32, lpextensionguid : *const ::windows_core::GUID, ulsectionid : u32, lpstringtofind : ::windows_core::PCSTR, returneddata : *mut ACTCTX_SECTION_KEYED_DATA) -> super::super::Foundation:: BOOL);
    FindActCtxSectionStringA(dwflags, ::core::mem::transmute(lpextensionguid.unwrap_or(::std::ptr::null())), ulsectionid, lpstringtofind.into_param().abi(), returneddata).ok()
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
#[inline]
pub unsafe fn FindActCtxSectionStringW<P0>(dwflags: u32, lpextensionguid: ::core::option::Option<*const ::windows_core::GUID>, ulsectionid: u32, lpstringtofind: P0, returneddata: *mut ACTCTX_SECTION_KEYED_DATA) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("kernel32.dll" "system" fn FindActCtxSectionStringW(dwflags : u32, lpextensionguid : *const ::windows_core::GUID, ulsectionid : u32, lpstringtofind : ::windows_core::PCWSTR, returneddata : *mut ACTCTX_SECTION_KEYED_DATA) -> super::super::Foundation:: BOOL);
    FindActCtxSectionStringW(dwflags, ::core::mem::transmute(lpextensionguid.unwrap_or(::std::ptr::null())), ulsectionid, lpstringtofind.into_param().abi(), returneddata).ok()
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentActCtx(lphactctx: *mut super::super::Foundation::HANDLE) -> ::windows_core::Result<()> {
    ::windows_targets::link!("kernel32.dll" "system" fn GetCurrentActCtx(lphactctx : *mut super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    GetCurrentActCtx(lphactctx).ok()
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDeltaInfoA<P0>(lpdeltaname: P0, lpheaderinfo: *mut DELTA_HEADER_INFO) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msdelta.dll" "system" fn GetDeltaInfoA(lpdeltaname : ::windows_core::PCSTR, lpheaderinfo : *mut DELTA_HEADER_INFO) -> super::super::Foundation:: BOOL);
    GetDeltaInfoA(lpdeltaname.into_param().abi(), lpheaderinfo)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDeltaInfoB(delta: DELTA_INPUT, lpheaderinfo: *mut DELTA_HEADER_INFO) -> super::super::Foundation::BOOL {
    ::windows_targets::link!("msdelta.dll" "system" fn GetDeltaInfoB(delta : DELTA_INPUT, lpheaderinfo : *mut DELTA_HEADER_INFO) -> super::super::Foundation:: BOOL);
    GetDeltaInfoB(::core::mem::transmute(delta), lpheaderinfo)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDeltaInfoW<P0>(lpdeltaname: P0, lpheaderinfo: *mut DELTA_HEADER_INFO) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msdelta.dll" "system" fn GetDeltaInfoW(lpdeltaname : ::windows_core::PCWSTR, lpheaderinfo : *mut DELTA_HEADER_INFO) -> super::super::Foundation:: BOOL);
    GetDeltaInfoW(lpdeltaname.into_param().abi(), lpheaderinfo)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDeltaSignatureA<P0>(filetypeset: i64, hashalgid: u32, lpsourcename: P0, lphash: *mut DELTA_HASH) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msdelta.dll" "system" fn GetDeltaSignatureA(filetypeset : i64, hashalgid : u32, lpsourcename : ::windows_core::PCSTR, lphash : *mut DELTA_HASH) -> super::super::Foundation:: BOOL);
    GetDeltaSignatureA(filetypeset, hashalgid, lpsourcename.into_param().abi(), lphash)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDeltaSignatureB(filetypeset: i64, hashalgid: u32, source: DELTA_INPUT, lphash: *mut DELTA_HASH) -> super::super::Foundation::BOOL {
    ::windows_targets::link!("msdelta.dll" "system" fn GetDeltaSignatureB(filetypeset : i64, hashalgid : u32, source : DELTA_INPUT, lphash : *mut DELTA_HASH) -> super::super::Foundation:: BOOL);
    GetDeltaSignatureB(filetypeset, hashalgid, ::core::mem::transmute(source), lphash)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDeltaSignatureW<P0>(filetypeset: i64, hashalgid: u32, lpsourcename: P0, lphash: *mut DELTA_HASH) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msdelta.dll" "system" fn GetDeltaSignatureW(filetypeset : i64, hashalgid : u32, lpsourcename : ::windows_core::PCWSTR, lphash : *mut DELTA_HASH) -> super::super::Foundation:: BOOL);
    GetDeltaSignatureW(filetypeset, hashalgid, lpsourcename.into_param().abi(), lphash)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFilePatchSignatureA<P0>(filename: P0, optionflags: u32, optiondata: ::core::option::Option<*const ::core::ffi::c_void>, ignorerangearray: ::core::option::Option<&[PATCH_IGNORE_RANGE]>, retainrangearray: ::core::option::Option<&[PATCH_RETAIN_RANGE]>, signaturebuffer: &mut [u8]) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("mspatcha.dll" "system" fn GetFilePatchSignatureA(filename : ::windows_core::PCSTR, optionflags : u32, optiondata : *const ::core::ffi::c_void, ignorerangecount : u32, ignorerangearray : *const PATCH_IGNORE_RANGE, retainrangecount : u32, retainrangearray : *const PATCH_RETAIN_RANGE, signaturebuffersize : u32, signaturebuffer : ::windows_core::PSTR) -> super::super::Foundation:: BOOL);
    GetFilePatchSignatureA(
        filename.into_param().abi(),
        optionflags,
        ::core::mem::transmute(optiondata.unwrap_or(::std::ptr::null())),
        ignorerangearray.as_deref().map_or(0, |slice| slice.len() as _),
        ::core::mem::transmute(ignorerangearray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        retainrangearray.as_deref().map_or(0, |slice| slice.len() as _),
        ::core::mem::transmute(retainrangearray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        signaturebuffer.len() as _,
        ::core::mem::transmute(signaturebuffer.as_ptr()),
    )
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFilePatchSignatureByBuffer(filebufferwritable: &mut [u8], optionflags: u32, optiondata: ::core::option::Option<*const ::core::ffi::c_void>, ignorerangearray: ::core::option::Option<&[PATCH_IGNORE_RANGE]>, retainrangearray: ::core::option::Option<&[PATCH_RETAIN_RANGE]>, signaturebuffer: &mut [u8]) -> super::super::Foundation::BOOL {
    ::windows_targets::link!("mspatcha.dll" "system" fn GetFilePatchSignatureByBuffer(filebufferwritable : *mut u8, filesize : u32, optionflags : u32, optiondata : *const ::core::ffi::c_void, ignorerangecount : u32, ignorerangearray : *const PATCH_IGNORE_RANGE, retainrangecount : u32, retainrangearray : *const PATCH_RETAIN_RANGE, signaturebuffersize : u32, signaturebuffer : ::windows_core::PSTR) -> super::super::Foundation:: BOOL);
    GetFilePatchSignatureByBuffer(
        ::core::mem::transmute(filebufferwritable.as_ptr()),
        filebufferwritable.len() as _,
        optionflags,
        ::core::mem::transmute(optiondata.unwrap_or(::std::ptr::null())),
        ignorerangearray.as_deref().map_or(0, |slice| slice.len() as _),
        ::core::mem::transmute(ignorerangearray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        retainrangearray.as_deref().map_or(0, |slice| slice.len() as _),
        ::core::mem::transmute(retainrangearray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        signaturebuffer.len() as _,
        ::core::mem::transmute(signaturebuffer.as_ptr()),
    )
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFilePatchSignatureByHandle<P0>(filehandle: P0, optionflags: u32, optiondata: ::core::option::Option<*const ::core::ffi::c_void>, ignorerangearray: ::core::option::Option<&[PATCH_IGNORE_RANGE]>, retainrangearray: ::core::option::Option<&[PATCH_RETAIN_RANGE]>, signaturebuffer: &mut [u8]) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("mspatcha.dll" "system" fn GetFilePatchSignatureByHandle(filehandle : super::super::Foundation:: HANDLE, optionflags : u32, optiondata : *const ::core::ffi::c_void, ignorerangecount : u32, ignorerangearray : *const PATCH_IGNORE_RANGE, retainrangecount : u32, retainrangearray : *const PATCH_RETAIN_RANGE, signaturebuffersize : u32, signaturebuffer : ::windows_core::PSTR) -> super::super::Foundation:: BOOL);
    GetFilePatchSignatureByHandle(
        filehandle.into_param().abi(),
        optionflags,
        ::core::mem::transmute(optiondata.unwrap_or(::std::ptr::null())),
        ignorerangearray.as_deref().map_or(0, |slice| slice.len() as _),
        ::core::mem::transmute(ignorerangearray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        retainrangearray.as_deref().map_or(0, |slice| slice.len() as _),
        ::core::mem::transmute(retainrangearray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        signaturebuffer.len() as _,
        ::core::mem::transmute(signaturebuffer.as_ptr()),
    )
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFilePatchSignatureW<P0>(filename: P0, optionflags: u32, optiondata: ::core::option::Option<*const ::core::ffi::c_void>, ignorerangearray: ::core::option::Option<&[PATCH_IGNORE_RANGE]>, retainrangearray: ::core::option::Option<&[PATCH_RETAIN_RANGE]>, signaturebuffersize: u32, signaturebuffer: ::windows_core::PWSTR) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("mspatcha.dll" "system" fn GetFilePatchSignatureW(filename : ::windows_core::PCWSTR, optionflags : u32, optiondata : *const ::core::ffi::c_void, ignorerangecount : u32, ignorerangearray : *const PATCH_IGNORE_RANGE, retainrangecount : u32, retainrangearray : *const PATCH_RETAIN_RANGE, signaturebuffersize : u32, signaturebuffer : ::windows_core::PWSTR) -> super::super::Foundation:: BOOL);
    GetFilePatchSignatureW(
        filename.into_param().abi(),
        optionflags,
        ::core::mem::transmute(optiondata.unwrap_or(::std::ptr::null())),
        ignorerangearray.as_deref().map_or(0, |slice| slice.len() as _),
        ::core::mem::transmute(ignorerangearray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        retainrangearray.as_deref().map_or(0, |slice| slice.len() as _),
        ::core::mem::transmute(retainrangearray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        signaturebuffersize,
        ::core::mem::transmute(signaturebuffer),
    )
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiAdvertiseProductA<P0, P1, P2>(szpackagepath: P0, szscriptfilepath: P1, sztransforms: P2, lgidlanguage: u16) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiAdvertiseProductA(szpackagepath : ::windows_core::PCSTR, szscriptfilepath : ::windows_core::PCSTR, sztransforms : ::windows_core::PCSTR, lgidlanguage : u16) -> u32);
    MsiAdvertiseProductA(szpackagepath.into_param().abi(), szscriptfilepath.into_param().abi(), sztransforms.into_param().abi(), lgidlanguage)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiAdvertiseProductExA<P0, P1, P2>(szpackagepath: P0, szscriptfilepath: P1, sztransforms: P2, lgidlanguage: u16, dwplatform: u32, dwoptions: u32) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiAdvertiseProductExA(szpackagepath : ::windows_core::PCSTR, szscriptfilepath : ::windows_core::PCSTR, sztransforms : ::windows_core::PCSTR, lgidlanguage : u16, dwplatform : u32, dwoptions : u32) -> u32);
    MsiAdvertiseProductExA(szpackagepath.into_param().abi(), szscriptfilepath.into_param().abi(), sztransforms.into_param().abi(), lgidlanguage, dwplatform, dwoptions)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiAdvertiseProductExW<P0, P1, P2>(szpackagepath: P0, szscriptfilepath: P1, sztransforms: P2, lgidlanguage: u16, dwplatform: u32, dwoptions: u32) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiAdvertiseProductExW(szpackagepath : ::windows_core::PCWSTR, szscriptfilepath : ::windows_core::PCWSTR, sztransforms : ::windows_core::PCWSTR, lgidlanguage : u16, dwplatform : u32, dwoptions : u32) -> u32);
    MsiAdvertiseProductExW(szpackagepath.into_param().abi(), szscriptfilepath.into_param().abi(), sztransforms.into_param().abi(), lgidlanguage, dwplatform, dwoptions)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiAdvertiseProductW<P0, P1, P2>(szpackagepath: P0, szscriptfilepath: P1, sztransforms: P2, lgidlanguage: u16) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiAdvertiseProductW(szpackagepath : ::windows_core::PCWSTR, szscriptfilepath : ::windows_core::PCWSTR, sztransforms : ::windows_core::PCWSTR, lgidlanguage : u16) -> u32);
    MsiAdvertiseProductW(szpackagepath.into_param().abi(), szscriptfilepath.into_param().abi(), sztransforms.into_param().abi(), lgidlanguage)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn MsiAdvertiseScriptA<P0, P1>(szscriptfile: P0, dwflags: u32, phregdata: ::core::option::Option<*const super::Registry::HKEY>, fremoveitems: P1) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiAdvertiseScriptA(szscriptfile : ::windows_core::PCSTR, dwflags : u32, phregdata : *const super::Registry:: HKEY, fremoveitems : super::super::Foundation:: BOOL) -> u32);
    MsiAdvertiseScriptA(szscriptfile.into_param().abi(), dwflags, ::core::mem::transmute(phregdata.unwrap_or(::std::ptr::null())), fremoveitems.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn MsiAdvertiseScriptW<P0, P1>(szscriptfile: P0, dwflags: u32, phregdata: ::core::option::Option<*const super::Registry::HKEY>, fremoveitems: P1) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiAdvertiseScriptW(szscriptfile : ::windows_core::PCWSTR, dwflags : u32, phregdata : *const super::Registry:: HKEY, fremoveitems : super::super::Foundation:: BOOL) -> u32);
    MsiAdvertiseScriptW(szscriptfile.into_param().abi(), dwflags, ::core::mem::transmute(phregdata.unwrap_or(::std::ptr::null())), fremoveitems.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiApplyMultiplePatchesA<P0, P1, P2>(szpatchpackages: P0, szproductcode: P1, szpropertieslist: P2) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiApplyMultiplePatchesA(szpatchpackages : ::windows_core::PCSTR, szproductcode : ::windows_core::PCSTR, szpropertieslist : ::windows_core::PCSTR) -> u32);
    MsiApplyMultiplePatchesA(szpatchpackages.into_param().abi(), szproductcode.into_param().abi(), szpropertieslist.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiApplyMultiplePatchesW<P0, P1, P2>(szpatchpackages: P0, szproductcode: P1, szpropertieslist: P2) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiApplyMultiplePatchesW(szpatchpackages : ::windows_core::PCWSTR, szproductcode : ::windows_core::PCWSTR, szpropertieslist : ::windows_core::PCWSTR) -> u32);
    MsiApplyMultiplePatchesW(szpatchpackages.into_param().abi(), szproductcode.into_param().abi(), szpropertieslist.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiApplyPatchA<P0, P1, P2>(szpatchpackage: P0, szinstallpackage: P1, einstalltype: INSTALLTYPE, szcommandline: P2) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiApplyPatchA(szpatchpackage : ::windows_core::PCSTR, szinstallpackage : ::windows_core::PCSTR, einstalltype : INSTALLTYPE, szcommandline : ::windows_core::PCSTR) -> u32);
    MsiApplyPatchA(szpatchpackage.into_param().abi(), szinstallpackage.into_param().abi(), einstalltype, szcommandline.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiApplyPatchW<P0, P1, P2>(szpatchpackage: P0, szinstallpackage: P1, einstalltype: INSTALLTYPE, szcommandline: P2) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiApplyPatchW(szpatchpackage : ::windows_core::PCWSTR, szinstallpackage : ::windows_core::PCWSTR, einstalltype : INSTALLTYPE, szcommandline : ::windows_core::PCWSTR) -> u32);
    MsiApplyPatchW(szpatchpackage.into_param().abi(), szinstallpackage.into_param().abi(), einstalltype, szcommandline.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiBeginTransactionA<P0>(szname: P0, dwtransactionattributes: u32, phtransactionhandle: *mut MSIHANDLE, phchangeofownerevent: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiBeginTransactionA(szname : ::windows_core::PCSTR, dwtransactionattributes : u32, phtransactionhandle : *mut MSIHANDLE, phchangeofownerevent : *mut super::super::Foundation:: HANDLE) -> u32);
    MsiBeginTransactionA(szname.into_param().abi(), dwtransactionattributes, phtransactionhandle, phchangeofownerevent)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiBeginTransactionW<P0>(szname: P0, dwtransactionattributes: u32, phtransactionhandle: *mut MSIHANDLE, phchangeofownerevent: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiBeginTransactionW(szname : ::windows_core::PCWSTR, dwtransactionattributes : u32, phtransactionhandle : *mut MSIHANDLE, phchangeofownerevent : *mut super::super::Foundation:: HANDLE) -> u32);
    MsiBeginTransactionW(szname.into_param().abi(), dwtransactionattributes, phtransactionhandle, phchangeofownerevent)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiCloseAllHandles() -> u32 {
    ::windows_targets::link!("msi.dll" "system" fn MsiCloseAllHandles() -> u32);
    MsiCloseAllHandles()
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiCloseHandle<P0>(hany: P0) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiCloseHandle(hany : MSIHANDLE) -> u32);
    MsiCloseHandle(hany.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiCollectUserInfoA<P0>(szproduct: P0) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiCollectUserInfoA(szproduct : ::windows_core::PCSTR) -> u32);
    MsiCollectUserInfoA(szproduct.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiCollectUserInfoW<P0>(szproduct: P0) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiCollectUserInfoW(szproduct : ::windows_core::PCWSTR) -> u32);
    MsiCollectUserInfoW(szproduct.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiConfigureFeatureA<P0, P1>(szproduct: P0, szfeature: P1, einstallstate: INSTALLSTATE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiConfigureFeatureA(szproduct : ::windows_core::PCSTR, szfeature : ::windows_core::PCSTR, einstallstate : INSTALLSTATE) -> u32);
    MsiConfigureFeatureA(szproduct.into_param().abi(), szfeature.into_param().abi(), einstallstate)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiConfigureFeatureW<P0, P1>(szproduct: P0, szfeature: P1, einstallstate: INSTALLSTATE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiConfigureFeatureW(szproduct : ::windows_core::PCWSTR, szfeature : ::windows_core::PCWSTR, einstallstate : INSTALLSTATE) -> u32);
    MsiConfigureFeatureW(szproduct.into_param().abi(), szfeature.into_param().abi(), einstallstate)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiConfigureProductA<P0>(szproduct: P0, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiConfigureProductA(szproduct : ::windows_core::PCSTR, iinstalllevel : INSTALLLEVEL, einstallstate : INSTALLSTATE) -> u32);
    MsiConfigureProductA(szproduct.into_param().abi(), iinstalllevel, einstallstate)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiConfigureProductExA<P0, P1>(szproduct: P0, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE, szcommandline: P1) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiConfigureProductExA(szproduct : ::windows_core::PCSTR, iinstalllevel : INSTALLLEVEL, einstallstate : INSTALLSTATE, szcommandline : ::windows_core::PCSTR) -> u32);
    MsiConfigureProductExA(szproduct.into_param().abi(), iinstalllevel, einstallstate, szcommandline.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiConfigureProductExW<P0, P1>(szproduct: P0, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE, szcommandline: P1) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiConfigureProductExW(szproduct : ::windows_core::PCWSTR, iinstalllevel : INSTALLLEVEL, einstallstate : INSTALLSTATE, szcommandline : ::windows_core::PCWSTR) -> u32);
    MsiConfigureProductExW(szproduct.into_param().abi(), iinstalllevel, einstallstate, szcommandline.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiConfigureProductW<P0>(szproduct: P0, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiConfigureProductW(szproduct : ::windows_core::PCWSTR, iinstalllevel : INSTALLLEVEL, einstallstate : INSTALLSTATE) -> u32);
    MsiConfigureProductW(szproduct.into_param().abi(), iinstalllevel, einstallstate)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiCreateRecord(cparams: u32) -> MSIHANDLE {
    ::windows_targets::link!("msi.dll" "system" fn MsiCreateRecord(cparams : u32) -> MSIHANDLE);
    MsiCreateRecord(cparams)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiCreateTransformSummaryInfoA<P0, P1, P2>(hdatabase: P0, hdatabasereference: P1, sztransformfile: P2, ierrorconditions: MSITRANSFORM_ERROR, ivalidation: MSITRANSFORM_VALIDATE) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<MSIHANDLE>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiCreateTransformSummaryInfoA(hdatabase : MSIHANDLE, hdatabasereference : MSIHANDLE, sztransformfile : ::windows_core::PCSTR, ierrorconditions : MSITRANSFORM_ERROR, ivalidation : MSITRANSFORM_VALIDATE) -> u32);
    MsiCreateTransformSummaryInfoA(hdatabase.into_param().abi(), hdatabasereference.into_param().abi(), sztransformfile.into_param().abi(), ierrorconditions, ivalidation)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiCreateTransformSummaryInfoW<P0, P1, P2>(hdatabase: P0, hdatabasereference: P1, sztransformfile: P2, ierrorconditions: MSITRANSFORM_ERROR, ivalidation: MSITRANSFORM_VALIDATE) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<MSIHANDLE>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiCreateTransformSummaryInfoW(hdatabase : MSIHANDLE, hdatabasereference : MSIHANDLE, sztransformfile : ::windows_core::PCWSTR, ierrorconditions : MSITRANSFORM_ERROR, ivalidation : MSITRANSFORM_VALIDATE) -> u32);
    MsiCreateTransformSummaryInfoW(hdatabase.into_param().abi(), hdatabasereference.into_param().abi(), sztransformfile.into_param().abi(), ierrorconditions, ivalidation)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseApplyTransformA<P0, P1>(hdatabase: P0, sztransformfile: P1, ierrorconditions: MSITRANSFORM_ERROR) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiDatabaseApplyTransformA(hdatabase : MSIHANDLE, sztransformfile : ::windows_core::PCSTR, ierrorconditions : MSITRANSFORM_ERROR) -> u32);
    MsiDatabaseApplyTransformA(hdatabase.into_param().abi(), sztransformfile.into_param().abi(), ierrorconditions)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseApplyTransformW<P0, P1>(hdatabase: P0, sztransformfile: P1, ierrorconditions: MSITRANSFORM_ERROR) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiDatabaseApplyTransformW(hdatabase : MSIHANDLE, sztransformfile : ::windows_core::PCWSTR, ierrorconditions : MSITRANSFORM_ERROR) -> u32);
    MsiDatabaseApplyTransformW(hdatabase.into_param().abi(), sztransformfile.into_param().abi(), ierrorconditions)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseCommit<P0>(hdatabase: P0) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiDatabaseCommit(hdatabase : MSIHANDLE) -> u32);
    MsiDatabaseCommit(hdatabase.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseExportA<P0, P1, P2, P3>(hdatabase: P0, sztablename: P1, szfolderpath: P2, szfilename: P3) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiDatabaseExportA(hdatabase : MSIHANDLE, sztablename : ::windows_core::PCSTR, szfolderpath : ::windows_core::PCSTR, szfilename : ::windows_core::PCSTR) -> u32);
    MsiDatabaseExportA(hdatabase.into_param().abi(), sztablename.into_param().abi(), szfolderpath.into_param().abi(), szfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseExportW<P0, P1, P2, P3>(hdatabase: P0, sztablename: P1, szfolderpath: P2, szfilename: P3) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiDatabaseExportW(hdatabase : MSIHANDLE, sztablename : ::windows_core::PCWSTR, szfolderpath : ::windows_core::PCWSTR, szfilename : ::windows_core::PCWSTR) -> u32);
    MsiDatabaseExportW(hdatabase.into_param().abi(), sztablename.into_param().abi(), szfolderpath.into_param().abi(), szfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseGenerateTransformA<P0, P1, P2>(hdatabase: P0, hdatabasereference: P1, sztransformfile: P2, ireserved1: i32, ireserved2: i32) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<MSIHANDLE>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiDatabaseGenerateTransformA(hdatabase : MSIHANDLE, hdatabasereference : MSIHANDLE, sztransformfile : ::windows_core::PCSTR, ireserved1 : i32, ireserved2 : i32) -> u32);
    MsiDatabaseGenerateTransformA(hdatabase.into_param().abi(), hdatabasereference.into_param().abi(), sztransformfile.into_param().abi(), ireserved1, ireserved2)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseGenerateTransformW<P0, P1, P2>(hdatabase: P0, hdatabasereference: P1, sztransformfile: P2, ireserved1: i32, ireserved2: i32) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<MSIHANDLE>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiDatabaseGenerateTransformW(hdatabase : MSIHANDLE, hdatabasereference : MSIHANDLE, sztransformfile : ::windows_core::PCWSTR, ireserved1 : i32, ireserved2 : i32) -> u32);
    MsiDatabaseGenerateTransformW(hdatabase.into_param().abi(), hdatabasereference.into_param().abi(), sztransformfile.into_param().abi(), ireserved1, ireserved2)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseGetPrimaryKeysA<P0, P1>(hdatabase: P0, sztablename: P1, phrecord: *mut MSIHANDLE) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiDatabaseGetPrimaryKeysA(hdatabase : MSIHANDLE, sztablename : ::windows_core::PCSTR, phrecord : *mut MSIHANDLE) -> u32);
    MsiDatabaseGetPrimaryKeysA(hdatabase.into_param().abi(), sztablename.into_param().abi(), phrecord)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseGetPrimaryKeysW<P0, P1>(hdatabase: P0, sztablename: P1, phrecord: *mut MSIHANDLE) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiDatabaseGetPrimaryKeysW(hdatabase : MSIHANDLE, sztablename : ::windows_core::PCWSTR, phrecord : *mut MSIHANDLE) -> u32);
    MsiDatabaseGetPrimaryKeysW(hdatabase.into_param().abi(), sztablename.into_param().abi(), phrecord)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseImportA<P0, P1, P2>(hdatabase: P0, szfolderpath: P1, szfilename: P2) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiDatabaseImportA(hdatabase : MSIHANDLE, szfolderpath : ::windows_core::PCSTR, szfilename : ::windows_core::PCSTR) -> u32);
    MsiDatabaseImportA(hdatabase.into_param().abi(), szfolderpath.into_param().abi(), szfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseImportW<P0, P1, P2>(hdatabase: P0, szfolderpath: P1, szfilename: P2) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiDatabaseImportW(hdatabase : MSIHANDLE, szfolderpath : ::windows_core::PCWSTR, szfilename : ::windows_core::PCWSTR) -> u32);
    MsiDatabaseImportW(hdatabase.into_param().abi(), szfolderpath.into_param().abi(), szfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseIsTablePersistentA<P0, P1>(hdatabase: P0, sztablename: P1) -> MSICONDITION
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiDatabaseIsTablePersistentA(hdatabase : MSIHANDLE, sztablename : ::windows_core::PCSTR) -> MSICONDITION);
    MsiDatabaseIsTablePersistentA(hdatabase.into_param().abi(), sztablename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseIsTablePersistentW<P0, P1>(hdatabase: P0, sztablename: P1) -> MSICONDITION
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiDatabaseIsTablePersistentW(hdatabase : MSIHANDLE, sztablename : ::windows_core::PCWSTR) -> MSICONDITION);
    MsiDatabaseIsTablePersistentW(hdatabase.into_param().abi(), sztablename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseMergeA<P0, P1, P2>(hdatabase: P0, hdatabasemerge: P1, sztablename: P2) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<MSIHANDLE>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiDatabaseMergeA(hdatabase : MSIHANDLE, hdatabasemerge : MSIHANDLE, sztablename : ::windows_core::PCSTR) -> u32);
    MsiDatabaseMergeA(hdatabase.into_param().abi(), hdatabasemerge.into_param().abi(), sztablename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseMergeW<P0, P1, P2>(hdatabase: P0, hdatabasemerge: P1, sztablename: P2) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<MSIHANDLE>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiDatabaseMergeW(hdatabase : MSIHANDLE, hdatabasemerge : MSIHANDLE, sztablename : ::windows_core::PCWSTR) -> u32);
    MsiDatabaseMergeW(hdatabase.into_param().abi(), hdatabasemerge.into_param().abi(), sztablename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseOpenViewA<P0, P1>(hdatabase: P0, szquery: P1, phview: *mut MSIHANDLE) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiDatabaseOpenViewA(hdatabase : MSIHANDLE, szquery : ::windows_core::PCSTR, phview : *mut MSIHANDLE) -> u32);
    MsiDatabaseOpenViewA(hdatabase.into_param().abi(), szquery.into_param().abi(), phview)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDatabaseOpenViewW<P0, P1>(hdatabase: P0, szquery: P1, phview: *mut MSIHANDLE) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiDatabaseOpenViewW(hdatabase : MSIHANDLE, szquery : ::windows_core::PCWSTR, phview : *mut MSIHANDLE) -> u32);
    MsiDatabaseOpenViewW(hdatabase.into_param().abi(), szquery.into_param().abi(), phview)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDetermineApplicablePatchesA<P0>(szproductpackagepath: P0, ppatchinfo: &mut [MSIPATCHSEQUENCEINFOA]) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiDetermineApplicablePatchesA(szproductpackagepath : ::windows_core::PCSTR, cpatchinfo : u32, ppatchinfo : *mut MSIPATCHSEQUENCEINFOA) -> u32);
    MsiDetermineApplicablePatchesA(szproductpackagepath.into_param().abi(), ppatchinfo.len() as _, ::core::mem::transmute(ppatchinfo.as_ptr()))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDetermineApplicablePatchesW<P0>(szproductpackagepath: P0, ppatchinfo: &mut [MSIPATCHSEQUENCEINFOW]) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiDetermineApplicablePatchesW(szproductpackagepath : ::windows_core::PCWSTR, cpatchinfo : u32, ppatchinfo : *mut MSIPATCHSEQUENCEINFOW) -> u32);
    MsiDetermineApplicablePatchesW(szproductpackagepath.into_param().abi(), ppatchinfo.len() as _, ::core::mem::transmute(ppatchinfo.as_ptr()))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDeterminePatchSequenceA<P0, P1>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, ppatchinfo: &mut [MSIPATCHSEQUENCEINFOA]) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiDeterminePatchSequenceA(szproductcode : ::windows_core::PCSTR, szusersid : ::windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, cpatchinfo : u32, ppatchinfo : *mut MSIPATCHSEQUENCEINFOA) -> u32);
    MsiDeterminePatchSequenceA(szproductcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, ppatchinfo.len() as _, ::core::mem::transmute(ppatchinfo.as_ptr()))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDeterminePatchSequenceW<P0, P1>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, ppatchinfo: &mut [MSIPATCHSEQUENCEINFOW]) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiDeterminePatchSequenceW(szproductcode : ::windows_core::PCWSTR, szusersid : ::windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, cpatchinfo : u32, ppatchinfo : *mut MSIPATCHSEQUENCEINFOW) -> u32);
    MsiDeterminePatchSequenceW(szproductcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, ppatchinfo.len() as _, ::core::mem::transmute(ppatchinfo.as_ptr()))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDoActionA<P0, P1>(hinstall: P0, szaction: P1) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiDoActionA(hinstall : MSIHANDLE, szaction : ::windows_core::PCSTR) -> u32);
    MsiDoActionA(hinstall.into_param().abi(), szaction.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiDoActionW<P0, P1>(hinstall: P0, szaction: P1) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiDoActionW(hinstall : MSIHANDLE, szaction : ::windows_core::PCWSTR) -> u32);
    MsiDoActionW(hinstall.into_param().abi(), szaction.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnableLogA<P0>(dwlogmode: INSTALLLOGMODE, szlogfile: P0, dwlogattributes: u32) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiEnableLogA(dwlogmode : u32, szlogfile : ::windows_core::PCSTR, dwlogattributes : u32) -> u32);
    MsiEnableLogA(dwlogmode.0 as _, szlogfile.into_param().abi(), dwlogattributes)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnableLogW<P0>(dwlogmode: INSTALLLOGMODE, szlogfile: P0, dwlogattributes: u32) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiEnableLogW(dwlogmode : u32, szlogfile : ::windows_core::PCWSTR, dwlogattributes : u32) -> u32);
    MsiEnableLogW(dwlogmode.0 as _, szlogfile.into_param().abi(), dwlogattributes)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnableUIPreview<P0>(hdatabase: P0, phpreview: *mut MSIHANDLE) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiEnableUIPreview(hdatabase : MSIHANDLE, phpreview : *mut MSIHANDLE) -> u32);
    MsiEnableUIPreview(hdatabase.into_param().abi(), phpreview)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEndTransaction(dwtransactionstate: MSITRANSACTIONSTATE) -> u32 {
    ::windows_targets::link!("msi.dll" "system" fn MsiEndTransaction(dwtransactionstate : MSITRANSACTIONSTATE) -> u32);
    MsiEndTransaction(dwtransactionstate)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumClientsA<P0>(szcomponent: P0, iproductindex: u32, lpproductbuf: ::windows_core::PSTR) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiEnumClientsA(szcomponent : ::windows_core::PCSTR, iproductindex : u32, lpproductbuf : ::windows_core::PSTR) -> u32);
    MsiEnumClientsA(szcomponent.into_param().abi(), iproductindex, ::core::mem::transmute(lpproductbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumClientsExA<P0, P1>(szcomponent: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwproductindex: u32, szproductbuf: ::core::option::Option<&mut [u8; 39]>, pdwinstalledcontext: ::core::option::Option<*mut MSIINSTALLCONTEXT>, szsid: ::windows_core::PSTR, pcchsid: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiEnumClientsExA(szcomponent : ::windows_core::PCSTR, szusersid : ::windows_core::PCSTR, dwcontext : u32, dwproductindex : u32, szproductbuf : ::windows_core::PSTR, pdwinstalledcontext : *mut MSIINSTALLCONTEXT, szsid : ::windows_core::PSTR, pcchsid : *mut u32) -> u32);
    MsiEnumClientsExA(szcomponent.into_param().abi(), szusersid.into_param().abi(), dwcontext.0 as _, dwproductindex, ::core::mem::transmute(szproductbuf.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(pdwinstalledcontext.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(szsid), ::core::mem::transmute(pcchsid.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumClientsExW<P0, P1>(szcomponent: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwproductindex: u32, szproductbuf: ::core::option::Option<&mut [u16; 39]>, pdwinstalledcontext: ::core::option::Option<*mut MSIINSTALLCONTEXT>, szsid: ::windows_core::PWSTR, pcchsid: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiEnumClientsExW(szcomponent : ::windows_core::PCWSTR, szusersid : ::windows_core::PCWSTR, dwcontext : u32, dwproductindex : u32, szproductbuf : ::windows_core::PWSTR, pdwinstalledcontext : *mut MSIINSTALLCONTEXT, szsid : ::windows_core::PWSTR, pcchsid : *mut u32) -> u32);
    MsiEnumClientsExW(szcomponent.into_param().abi(), szusersid.into_param().abi(), dwcontext.0 as _, dwproductindex, ::core::mem::transmute(szproductbuf.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(pdwinstalledcontext.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(szsid), ::core::mem::transmute(pcchsid.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumClientsW<P0>(szcomponent: P0, iproductindex: u32, lpproductbuf: ::windows_core::PWSTR) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiEnumClientsW(szcomponent : ::windows_core::PCWSTR, iproductindex : u32, lpproductbuf : ::windows_core::PWSTR) -> u32);
    MsiEnumClientsW(szcomponent.into_param().abi(), iproductindex, ::core::mem::transmute(lpproductbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumComponentCostsA<P0, P1>(hinstall: P0, szcomponent: P1, dwindex: u32, istate: INSTALLSTATE, szdrivebuf: ::windows_core::PSTR, pcchdrivebuf: *mut u32, picost: *mut i32, pitempcost: *mut i32) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiEnumComponentCostsA(hinstall : MSIHANDLE, szcomponent : ::windows_core::PCSTR, dwindex : u32, istate : INSTALLSTATE, szdrivebuf : ::windows_core::PSTR, pcchdrivebuf : *mut u32, picost : *mut i32, pitempcost : *mut i32) -> u32);
    MsiEnumComponentCostsA(hinstall.into_param().abi(), szcomponent.into_param().abi(), dwindex, istate, ::core::mem::transmute(szdrivebuf), pcchdrivebuf, picost, pitempcost)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumComponentCostsW<P0, P1>(hinstall: P0, szcomponent: P1, dwindex: u32, istate: INSTALLSTATE, szdrivebuf: ::windows_core::PWSTR, pcchdrivebuf: *mut u32, picost: *mut i32, pitempcost: *mut i32) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiEnumComponentCostsW(hinstall : MSIHANDLE, szcomponent : ::windows_core::PCWSTR, dwindex : u32, istate : INSTALLSTATE, szdrivebuf : ::windows_core::PWSTR, pcchdrivebuf : *mut u32, picost : *mut i32, pitempcost : *mut i32) -> u32);
    MsiEnumComponentCostsW(hinstall.into_param().abi(), szcomponent.into_param().abi(), dwindex, istate, ::core::mem::transmute(szdrivebuf), pcchdrivebuf, picost, pitempcost)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumComponentQualifiersA<P0>(szcomponent: P0, iindex: u32, lpqualifierbuf: ::windows_core::PSTR, pcchqualifierbuf: *mut u32, lpapplicationdatabuf: ::windows_core::PSTR, pcchapplicationdatabuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiEnumComponentQualifiersA(szcomponent : ::windows_core::PCSTR, iindex : u32, lpqualifierbuf : ::windows_core::PSTR, pcchqualifierbuf : *mut u32, lpapplicationdatabuf : ::windows_core::PSTR, pcchapplicationdatabuf : *mut u32) -> u32);
    MsiEnumComponentQualifiersA(szcomponent.into_param().abi(), iindex, ::core::mem::transmute(lpqualifierbuf), pcchqualifierbuf, ::core::mem::transmute(lpapplicationdatabuf), ::core::mem::transmute(pcchapplicationdatabuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumComponentQualifiersW<P0>(szcomponent: P0, iindex: u32, lpqualifierbuf: ::windows_core::PWSTR, pcchqualifierbuf: *mut u32, lpapplicationdatabuf: ::windows_core::PWSTR, pcchapplicationdatabuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiEnumComponentQualifiersW(szcomponent : ::windows_core::PCWSTR, iindex : u32, lpqualifierbuf : ::windows_core::PWSTR, pcchqualifierbuf : *mut u32, lpapplicationdatabuf : ::windows_core::PWSTR, pcchapplicationdatabuf : *mut u32) -> u32);
    MsiEnumComponentQualifiersW(szcomponent.into_param().abi(), iindex, ::core::mem::transmute(lpqualifierbuf), pcchqualifierbuf, ::core::mem::transmute(lpapplicationdatabuf), ::core::mem::transmute(pcchapplicationdatabuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumComponentsA(icomponentindex: u32, lpcomponentbuf: ::windows_core::PSTR) -> u32 {
    ::windows_targets::link!("msi.dll" "system" fn MsiEnumComponentsA(icomponentindex : u32, lpcomponentbuf : ::windows_core::PSTR) -> u32);
    MsiEnumComponentsA(icomponentindex, ::core::mem::transmute(lpcomponentbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumComponentsExA<P0>(szusersid: P0, dwcontext: u32, dwindex: u32, szinstalledcomponentcode: ::core::option::Option<&mut [u8; 39]>, pdwinstalledcontext: ::core::option::Option<*mut MSIINSTALLCONTEXT>, szsid: ::windows_core::PSTR, pcchsid: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiEnumComponentsExA(szusersid : ::windows_core::PCSTR, dwcontext : u32, dwindex : u32, szinstalledcomponentcode : ::windows_core::PSTR, pdwinstalledcontext : *mut MSIINSTALLCONTEXT, szsid : ::windows_core::PSTR, pcchsid : *mut u32) -> u32);
    MsiEnumComponentsExA(szusersid.into_param().abi(), dwcontext, dwindex, ::core::mem::transmute(szinstalledcomponentcode.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(pdwinstalledcontext.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(szsid), ::core::mem::transmute(pcchsid.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumComponentsExW<P0>(szusersid: P0, dwcontext: u32, dwindex: u32, szinstalledcomponentcode: ::core::option::Option<&mut [u16; 39]>, pdwinstalledcontext: ::core::option::Option<*mut MSIINSTALLCONTEXT>, szsid: ::windows_core::PWSTR, pcchsid: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiEnumComponentsExW(szusersid : ::windows_core::PCWSTR, dwcontext : u32, dwindex : u32, szinstalledcomponentcode : ::windows_core::PWSTR, pdwinstalledcontext : *mut MSIINSTALLCONTEXT, szsid : ::windows_core::PWSTR, pcchsid : *mut u32) -> u32);
    MsiEnumComponentsExW(szusersid.into_param().abi(), dwcontext, dwindex, ::core::mem::transmute(szinstalledcomponentcode.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(pdwinstalledcontext.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(szsid), ::core::mem::transmute(pcchsid.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumComponentsW(icomponentindex: u32, lpcomponentbuf: ::windows_core::PWSTR) -> u32 {
    ::windows_targets::link!("msi.dll" "system" fn MsiEnumComponentsW(icomponentindex : u32, lpcomponentbuf : ::windows_core::PWSTR) -> u32);
    MsiEnumComponentsW(icomponentindex, ::core::mem::transmute(lpcomponentbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumFeaturesA<P0>(szproduct: P0, ifeatureindex: u32, lpfeaturebuf: ::windows_core::PSTR, lpparentbuf: ::windows_core::PSTR) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiEnumFeaturesA(szproduct : ::windows_core::PCSTR, ifeatureindex : u32, lpfeaturebuf : ::windows_core::PSTR, lpparentbuf : ::windows_core::PSTR) -> u32);
    MsiEnumFeaturesA(szproduct.into_param().abi(), ifeatureindex, ::core::mem::transmute(lpfeaturebuf), ::core::mem::transmute(lpparentbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumFeaturesW<P0>(szproduct: P0, ifeatureindex: u32, lpfeaturebuf: ::windows_core::PWSTR, lpparentbuf: ::windows_core::PWSTR) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiEnumFeaturesW(szproduct : ::windows_core::PCWSTR, ifeatureindex : u32, lpfeaturebuf : ::windows_core::PWSTR, lpparentbuf : ::windows_core::PWSTR) -> u32);
    MsiEnumFeaturesW(szproduct.into_param().abi(), ifeatureindex, ::core::mem::transmute(lpfeaturebuf), ::core::mem::transmute(lpparentbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumPatchesA<P0>(szproduct: P0, ipatchindex: u32, lppatchbuf: ::windows_core::PSTR, lptransformsbuf: ::windows_core::PSTR, pcchtransformsbuf: *mut u32) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiEnumPatchesA(szproduct : ::windows_core::PCSTR, ipatchindex : u32, lppatchbuf : ::windows_core::PSTR, lptransformsbuf : ::windows_core::PSTR, pcchtransformsbuf : *mut u32) -> u32);
    MsiEnumPatchesA(szproduct.into_param().abi(), ipatchindex, ::core::mem::transmute(lppatchbuf), ::core::mem::transmute(lptransformsbuf), pcchtransformsbuf)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumPatchesExA<P0, P1>(szproductcode: P0, szusersid: P1, dwcontext: u32, dwfilter: u32, dwindex: u32, szpatchcode: ::core::option::Option<&mut [u8; 39]>, sztargetproductcode: ::core::option::Option<&mut [u8; 39]>, pdwtargetproductcontext: ::core::option::Option<*mut MSIINSTALLCONTEXT>, sztargetusersid: ::windows_core::PSTR, pcchtargetusersid: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiEnumPatchesExA(szproductcode : ::windows_core::PCSTR, szusersid : ::windows_core::PCSTR, dwcontext : u32, dwfilter : u32, dwindex : u32, szpatchcode : ::windows_core::PSTR, sztargetproductcode : ::windows_core::PSTR, pdwtargetproductcontext : *mut MSIINSTALLCONTEXT, sztargetusersid : ::windows_core::PSTR, pcchtargetusersid : *mut u32) -> u32);
    MsiEnumPatchesExA(
        szproductcode.into_param().abi(),
        szusersid.into_param().abi(),
        dwcontext,
        dwfilter,
        dwindex,
        ::core::mem::transmute(szpatchcode.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        ::core::mem::transmute(sztargetproductcode.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        ::core::mem::transmute(pdwtargetproductcontext.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(sztargetusersid),
        ::core::mem::transmute(pcchtargetusersid.unwrap_or(::std::ptr::null_mut())),
    )
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumPatchesExW<P0, P1>(szproductcode: P0, szusersid: P1, dwcontext: u32, dwfilter: u32, dwindex: u32, szpatchcode: ::core::option::Option<&mut [u16; 39]>, sztargetproductcode: ::core::option::Option<&mut [u16; 39]>, pdwtargetproductcontext: ::core::option::Option<*mut MSIINSTALLCONTEXT>, sztargetusersid: ::windows_core::PWSTR, pcchtargetusersid: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiEnumPatchesExW(szproductcode : ::windows_core::PCWSTR, szusersid : ::windows_core::PCWSTR, dwcontext : u32, dwfilter : u32, dwindex : u32, szpatchcode : ::windows_core::PWSTR, sztargetproductcode : ::windows_core::PWSTR, pdwtargetproductcontext : *mut MSIINSTALLCONTEXT, sztargetusersid : ::windows_core::PWSTR, pcchtargetusersid : *mut u32) -> u32);
    MsiEnumPatchesExW(
        szproductcode.into_param().abi(),
        szusersid.into_param().abi(),
        dwcontext,
        dwfilter,
        dwindex,
        ::core::mem::transmute(szpatchcode.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        ::core::mem::transmute(sztargetproductcode.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        ::core::mem::transmute(pdwtargetproductcontext.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(sztargetusersid),
        ::core::mem::transmute(pcchtargetusersid.unwrap_or(::std::ptr::null_mut())),
    )
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumPatchesW<P0>(szproduct: P0, ipatchindex: u32, lppatchbuf: ::windows_core::PWSTR, lptransformsbuf: ::windows_core::PWSTR, pcchtransformsbuf: *mut u32) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiEnumPatchesW(szproduct : ::windows_core::PCWSTR, ipatchindex : u32, lppatchbuf : ::windows_core::PWSTR, lptransformsbuf : ::windows_core::PWSTR, pcchtransformsbuf : *mut u32) -> u32);
    MsiEnumPatchesW(szproduct.into_param().abi(), ipatchindex, ::core::mem::transmute(lppatchbuf), ::core::mem::transmute(lptransformsbuf), pcchtransformsbuf)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumProductsA(iproductindex: u32, lpproductbuf: ::windows_core::PSTR) -> u32 {
    ::windows_targets::link!("msi.dll" "system" fn MsiEnumProductsA(iproductindex : u32, lpproductbuf : ::windows_core::PSTR) -> u32);
    MsiEnumProductsA(iproductindex, ::core::mem::transmute(lpproductbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumProductsExA<P0, P1>(szproductcode: P0, szusersid: P1, dwcontext: u32, dwindex: u32, szinstalledproductcode: ::core::option::Option<&mut [u8; 39]>, pdwinstalledcontext: ::core::option::Option<*mut MSIINSTALLCONTEXT>, szsid: ::windows_core::PSTR, pcchsid: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiEnumProductsExA(szproductcode : ::windows_core::PCSTR, szusersid : ::windows_core::PCSTR, dwcontext : u32, dwindex : u32, szinstalledproductcode : ::windows_core::PSTR, pdwinstalledcontext : *mut MSIINSTALLCONTEXT, szsid : ::windows_core::PSTR, pcchsid : *mut u32) -> u32);
    MsiEnumProductsExA(szproductcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, dwindex, ::core::mem::transmute(szinstalledproductcode.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(pdwinstalledcontext.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(szsid), ::core::mem::transmute(pcchsid.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumProductsExW<P0, P1>(szproductcode: P0, szusersid: P1, dwcontext: u32, dwindex: u32, szinstalledproductcode: ::core::option::Option<&mut [u16; 39]>, pdwinstalledcontext: ::core::option::Option<*mut MSIINSTALLCONTEXT>, szsid: ::windows_core::PWSTR, pcchsid: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiEnumProductsExW(szproductcode : ::windows_core::PCWSTR, szusersid : ::windows_core::PCWSTR, dwcontext : u32, dwindex : u32, szinstalledproductcode : ::windows_core::PWSTR, pdwinstalledcontext : *mut MSIINSTALLCONTEXT, szsid : ::windows_core::PWSTR, pcchsid : *mut u32) -> u32);
    MsiEnumProductsExW(szproductcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, dwindex, ::core::mem::transmute(szinstalledproductcode.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(pdwinstalledcontext.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(szsid), ::core::mem::transmute(pcchsid.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumProductsW(iproductindex: u32, lpproductbuf: ::windows_core::PWSTR) -> u32 {
    ::windows_targets::link!("msi.dll" "system" fn MsiEnumProductsW(iproductindex : u32, lpproductbuf : ::windows_core::PWSTR) -> u32);
    MsiEnumProductsW(iproductindex, ::core::mem::transmute(lpproductbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumRelatedProductsA<P0>(lpupgradecode: P0, dwreserved: u32, iproductindex: u32, lpproductbuf: ::windows_core::PSTR) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiEnumRelatedProductsA(lpupgradecode : ::windows_core::PCSTR, dwreserved : u32, iproductindex : u32, lpproductbuf : ::windows_core::PSTR) -> u32);
    MsiEnumRelatedProductsA(lpupgradecode.into_param().abi(), dwreserved, iproductindex, ::core::mem::transmute(lpproductbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEnumRelatedProductsW<P0>(lpupgradecode: P0, dwreserved: u32, iproductindex: u32, lpproductbuf: ::windows_core::PWSTR) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiEnumRelatedProductsW(lpupgradecode : ::windows_core::PCWSTR, dwreserved : u32, iproductindex : u32, lpproductbuf : ::windows_core::PWSTR) -> u32);
    MsiEnumRelatedProductsW(lpupgradecode.into_param().abi(), dwreserved, iproductindex, ::core::mem::transmute(lpproductbuf))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEvaluateConditionA<P0, P1>(hinstall: P0, szcondition: P1) -> MSICONDITION
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiEvaluateConditionA(hinstall : MSIHANDLE, szcondition : ::windows_core::PCSTR) -> MSICONDITION);
    MsiEvaluateConditionA(hinstall.into_param().abi(), szcondition.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiEvaluateConditionW<P0, P1>(hinstall: P0, szcondition: P1) -> MSICONDITION
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiEvaluateConditionW(hinstall : MSIHANDLE, szcondition : ::windows_core::PCWSTR) -> MSICONDITION);
    MsiEvaluateConditionW(hinstall.into_param().abi(), szcondition.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiExtractPatchXMLDataA<P0>(szpatchpath: P0, dwreserved: u32, szxmldata: ::windows_core::PSTR, pcchxmldata: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiExtractPatchXMLDataA(szpatchpath : ::windows_core::PCSTR, dwreserved : u32, szxmldata : ::windows_core::PSTR, pcchxmldata : *mut u32) -> u32);
    MsiExtractPatchXMLDataA(szpatchpath.into_param().abi(), dwreserved, ::core::mem::transmute(szxmldata), ::core::mem::transmute(pcchxmldata.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiExtractPatchXMLDataW<P0>(szpatchpath: P0, dwreserved: u32, szxmldata: ::windows_core::PWSTR, pcchxmldata: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiExtractPatchXMLDataW(szpatchpath : ::windows_core::PCWSTR, dwreserved : u32, szxmldata : ::windows_core::PWSTR, pcchxmldata : *mut u32) -> u32);
    MsiExtractPatchXMLDataW(szpatchpath.into_param().abi(), dwreserved, ::core::mem::transmute(szxmldata), ::core::mem::transmute(pcchxmldata.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiFormatRecordA<P0, P1>(hinstall: P0, hrecord: P1, szresultbuf: ::windows_core::PSTR, pcchresultbuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiFormatRecordA(hinstall : MSIHANDLE, hrecord : MSIHANDLE, szresultbuf : ::windows_core::PSTR, pcchresultbuf : *mut u32) -> u32);
    MsiFormatRecordA(hinstall.into_param().abi(), hrecord.into_param().abi(), ::core::mem::transmute(szresultbuf), ::core::mem::transmute(pcchresultbuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiFormatRecordW<P0, P1>(hinstall: P0, hrecord: P1, szresultbuf: ::windows_core::PWSTR, pcchresultbuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiFormatRecordW(hinstall : MSIHANDLE, hrecord : MSIHANDLE, szresultbuf : ::windows_core::PWSTR, pcchresultbuf : *mut u32) -> u32);
    MsiFormatRecordW(hinstall.into_param().abi(), hrecord.into_param().abi(), ::core::mem::transmute(szresultbuf), ::core::mem::transmute(pcchresultbuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetActiveDatabase<P0>(hinstall: P0) -> MSIHANDLE
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetActiveDatabase(hinstall : MSIHANDLE) -> MSIHANDLE);
    MsiGetActiveDatabase(hinstall.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetComponentPathA<P0, P1>(szproduct: P0, szcomponent: P1, lppathbuf: ::windows_core::PSTR, pcchbuf: ::core::option::Option<*mut u32>) -> INSTALLSTATE
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetComponentPathA(szproduct : ::windows_core::PCSTR, szcomponent : ::windows_core::PCSTR, lppathbuf : ::windows_core::PSTR, pcchbuf : *mut u32) -> INSTALLSTATE);
    MsiGetComponentPathA(szproduct.into_param().abi(), szcomponent.into_param().abi(), ::core::mem::transmute(lppathbuf), ::core::mem::transmute(pcchbuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetComponentPathExA<P0, P1, P2>(szproductcode: P0, szcomponentcode: P1, szusersid: P2, dwcontext: MSIINSTALLCONTEXT, lpoutpathbuffer: ::windows_core::PSTR, pcchoutpathbuffer: ::core::option::Option<*mut u32>) -> INSTALLSTATE
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetComponentPathExA(szproductcode : ::windows_core::PCSTR, szcomponentcode : ::windows_core::PCSTR, szusersid : ::windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, lpoutpathbuffer : ::windows_core::PSTR, pcchoutpathbuffer : *mut u32) -> INSTALLSTATE);
    MsiGetComponentPathExA(szproductcode.into_param().abi(), szcomponentcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, ::core::mem::transmute(lpoutpathbuffer), ::core::mem::transmute(pcchoutpathbuffer.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetComponentPathExW<P0, P1, P2>(szproductcode: P0, szcomponentcode: P1, szusersid: P2, dwcontext: MSIINSTALLCONTEXT, lpoutpathbuffer: ::windows_core::PWSTR, pcchoutpathbuffer: ::core::option::Option<*mut u32>) -> INSTALLSTATE
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetComponentPathExW(szproductcode : ::windows_core::PCWSTR, szcomponentcode : ::windows_core::PCWSTR, szusersid : ::windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, lpoutpathbuffer : ::windows_core::PWSTR, pcchoutpathbuffer : *mut u32) -> INSTALLSTATE);
    MsiGetComponentPathExW(szproductcode.into_param().abi(), szcomponentcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, ::core::mem::transmute(lpoutpathbuffer), ::core::mem::transmute(pcchoutpathbuffer.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetComponentPathW<P0, P1>(szproduct: P0, szcomponent: P1, lppathbuf: ::windows_core::PWSTR, pcchbuf: ::core::option::Option<*mut u32>) -> INSTALLSTATE
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetComponentPathW(szproduct : ::windows_core::PCWSTR, szcomponent : ::windows_core::PCWSTR, lppathbuf : ::windows_core::PWSTR, pcchbuf : *mut u32) -> INSTALLSTATE);
    MsiGetComponentPathW(szproduct.into_param().abi(), szcomponent.into_param().abi(), ::core::mem::transmute(lppathbuf), ::core::mem::transmute(pcchbuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetComponentStateA<P0, P1>(hinstall: P0, szcomponent: P1, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetComponentStateA(hinstall : MSIHANDLE, szcomponent : ::windows_core::PCSTR, piinstalled : *mut INSTALLSTATE, piaction : *mut INSTALLSTATE) -> u32);
    MsiGetComponentStateA(hinstall.into_param().abi(), szcomponent.into_param().abi(), piinstalled, piaction)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetComponentStateW<P0, P1>(hinstall: P0, szcomponent: P1, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetComponentStateW(hinstall : MSIHANDLE, szcomponent : ::windows_core::PCWSTR, piinstalled : *mut INSTALLSTATE, piaction : *mut INSTALLSTATE) -> u32);
    MsiGetComponentStateW(hinstall.into_param().abi(), szcomponent.into_param().abi(), piinstalled, piaction)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetDatabaseState<P0>(hdatabase: P0) -> MSIDBSTATE
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetDatabaseState(hdatabase : MSIHANDLE) -> MSIDBSTATE);
    MsiGetDatabaseState(hdatabase.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetFeatureCostA<P0, P1>(hinstall: P0, szfeature: P1, icosttree: MSICOSTTREE, istate: INSTALLSTATE, picost: *mut i32) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetFeatureCostA(hinstall : MSIHANDLE, szfeature : ::windows_core::PCSTR, icosttree : MSICOSTTREE, istate : INSTALLSTATE, picost : *mut i32) -> u32);
    MsiGetFeatureCostA(hinstall.into_param().abi(), szfeature.into_param().abi(), icosttree, istate, picost)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetFeatureCostW<P0, P1>(hinstall: P0, szfeature: P1, icosttree: MSICOSTTREE, istate: INSTALLSTATE, picost: *mut i32) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetFeatureCostW(hinstall : MSIHANDLE, szfeature : ::windows_core::PCWSTR, icosttree : MSICOSTTREE, istate : INSTALLSTATE, picost : *mut i32) -> u32);
    MsiGetFeatureCostW(hinstall.into_param().abi(), szfeature.into_param().abi(), icosttree, istate, picost)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetFeatureInfoA<P0, P1>(hproduct: P0, szfeature: P1, lpattributes: ::core::option::Option<*mut u32>, lptitlebuf: ::windows_core::PSTR, pcchtitlebuf: ::core::option::Option<*mut u32>, lphelpbuf: ::windows_core::PSTR, pcchhelpbuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetFeatureInfoA(hproduct : MSIHANDLE, szfeature : ::windows_core::PCSTR, lpattributes : *mut u32, lptitlebuf : ::windows_core::PSTR, pcchtitlebuf : *mut u32, lphelpbuf : ::windows_core::PSTR, pcchhelpbuf : *mut u32) -> u32);
    MsiGetFeatureInfoA(hproduct.into_param().abi(), szfeature.into_param().abi(), ::core::mem::transmute(lpattributes.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lptitlebuf), ::core::mem::transmute(pcchtitlebuf.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lphelpbuf), ::core::mem::transmute(pcchhelpbuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetFeatureInfoW<P0, P1>(hproduct: P0, szfeature: P1, lpattributes: ::core::option::Option<*mut u32>, lptitlebuf: ::windows_core::PWSTR, pcchtitlebuf: ::core::option::Option<*mut u32>, lphelpbuf: ::windows_core::PWSTR, pcchhelpbuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetFeatureInfoW(hproduct : MSIHANDLE, szfeature : ::windows_core::PCWSTR, lpattributes : *mut u32, lptitlebuf : ::windows_core::PWSTR, pcchtitlebuf : *mut u32, lphelpbuf : ::windows_core::PWSTR, pcchhelpbuf : *mut u32) -> u32);
    MsiGetFeatureInfoW(hproduct.into_param().abi(), szfeature.into_param().abi(), ::core::mem::transmute(lpattributes.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lptitlebuf), ::core::mem::transmute(pcchtitlebuf.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lphelpbuf), ::core::mem::transmute(pcchhelpbuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetFeatureStateA<P0, P1>(hinstall: P0, szfeature: P1, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetFeatureStateA(hinstall : MSIHANDLE, szfeature : ::windows_core::PCSTR, piinstalled : *mut INSTALLSTATE, piaction : *mut INSTALLSTATE) -> u32);
    MsiGetFeatureStateA(hinstall.into_param().abi(), szfeature.into_param().abi(), piinstalled, piaction)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetFeatureStateW<P0, P1>(hinstall: P0, szfeature: P1, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetFeatureStateW(hinstall : MSIHANDLE, szfeature : ::windows_core::PCWSTR, piinstalled : *mut INSTALLSTATE, piaction : *mut INSTALLSTATE) -> u32);
    MsiGetFeatureStateW(hinstall.into_param().abi(), szfeature.into_param().abi(), piinstalled, piaction)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetFeatureUsageA<P0, P1>(szproduct: P0, szfeature: P1, pdwusecount: ::core::option::Option<*mut u32>, pwdateused: ::core::option::Option<*mut u16>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetFeatureUsageA(szproduct : ::windows_core::PCSTR, szfeature : ::windows_core::PCSTR, pdwusecount : *mut u32, pwdateused : *mut u16) -> u32);
    MsiGetFeatureUsageA(szproduct.into_param().abi(), szfeature.into_param().abi(), ::core::mem::transmute(pdwusecount.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pwdateused.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetFeatureUsageW<P0, P1>(szproduct: P0, szfeature: P1, pdwusecount: ::core::option::Option<*mut u32>, pwdateused: ::core::option::Option<*mut u16>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetFeatureUsageW(szproduct : ::windows_core::PCWSTR, szfeature : ::windows_core::PCWSTR, pdwusecount : *mut u32, pwdateused : *mut u16) -> u32);
    MsiGetFeatureUsageW(szproduct.into_param().abi(), szfeature.into_param().abi(), ::core::mem::transmute(pdwusecount.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pwdateused.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetFeatureValidStatesA<P0, P1>(hinstall: P0, szfeature: P1, lpinstallstates: *mut u32) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetFeatureValidStatesA(hinstall : MSIHANDLE, szfeature : ::windows_core::PCSTR, lpinstallstates : *mut u32) -> u32);
    MsiGetFeatureValidStatesA(hinstall.into_param().abi(), szfeature.into_param().abi(), lpinstallstates)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetFeatureValidStatesW<P0, P1>(hinstall: P0, szfeature: P1, lpinstallstates: *mut u32) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetFeatureValidStatesW(hinstall : MSIHANDLE, szfeature : ::windows_core::PCWSTR, lpinstallstates : *mut u32) -> u32);
    MsiGetFeatureValidStatesW(hinstall.into_param().abi(), szfeature.into_param().abi(), lpinstallstates)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetFileHashA<P0>(szfilepath: P0, dwoptions: u32, phash: *mut MSIFILEHASHINFO) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetFileHashA(szfilepath : ::windows_core::PCSTR, dwoptions : u32, phash : *mut MSIFILEHASHINFO) -> u32);
    MsiGetFileHashA(szfilepath.into_param().abi(), dwoptions, phash)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetFileHashW<P0>(szfilepath: P0, dwoptions: u32, phash: *mut MSIFILEHASHINFO) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetFileHashW(szfilepath : ::windows_core::PCWSTR, dwoptions : u32, phash : *mut MSIFILEHASHINFO) -> u32);
    MsiGetFileHashW(szfilepath.into_param().abi(), dwoptions, phash)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn MsiGetFileSignatureInformationA<P0>(szsignedobjectpath: P0, dwflags: u32, ppccertcontext: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT, pbhashdata: ::core::option::Option<*mut u8>, pcbhashdata: ::core::option::Option<*mut u32>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetFileSignatureInformationA(szsignedobjectpath : ::windows_core::PCSTR, dwflags : u32, ppccertcontext : *mut *mut super::super::Security::Cryptography:: CERT_CONTEXT, pbhashdata : *mut u8, pcbhashdata : *mut u32) -> ::windows_core::HRESULT);
    MsiGetFileSignatureInformationA(szsignedobjectpath.into_param().abi(), dwflags, ppccertcontext, ::core::mem::transmute(pbhashdata.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbhashdata.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn MsiGetFileSignatureInformationW<P0>(szsignedobjectpath: P0, dwflags: u32, ppccertcontext: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT, pbhashdata: ::core::option::Option<*mut u8>, pcbhashdata: ::core::option::Option<*mut u32>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetFileSignatureInformationW(szsignedobjectpath : ::windows_core::PCWSTR, dwflags : u32, ppccertcontext : *mut *mut super::super::Security::Cryptography:: CERT_CONTEXT, pbhashdata : *mut u8, pcbhashdata : *mut u32) -> ::windows_core::HRESULT);
    MsiGetFileSignatureInformationW(szsignedobjectpath.into_param().abi(), dwflags, ppccertcontext, ::core::mem::transmute(pbhashdata.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbhashdata.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetFileVersionA<P0>(szfilepath: P0, lpversionbuf: ::windows_core::PSTR, pcchversionbuf: ::core::option::Option<*mut u32>, lplangbuf: ::windows_core::PSTR, pcchlangbuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetFileVersionA(szfilepath : ::windows_core::PCSTR, lpversionbuf : ::windows_core::PSTR, pcchversionbuf : *mut u32, lplangbuf : ::windows_core::PSTR, pcchlangbuf : *mut u32) -> u32);
    MsiGetFileVersionA(szfilepath.into_param().abi(), ::core::mem::transmute(lpversionbuf), ::core::mem::transmute(pcchversionbuf.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lplangbuf), ::core::mem::transmute(pcchlangbuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetFileVersionW<P0>(szfilepath: P0, lpversionbuf: ::windows_core::PWSTR, pcchversionbuf: ::core::option::Option<*mut u32>, lplangbuf: ::windows_core::PWSTR, pcchlangbuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetFileVersionW(szfilepath : ::windows_core::PCWSTR, lpversionbuf : ::windows_core::PWSTR, pcchversionbuf : *mut u32, lplangbuf : ::windows_core::PWSTR, pcchlangbuf : *mut u32) -> u32);
    MsiGetFileVersionW(szfilepath.into_param().abi(), ::core::mem::transmute(lpversionbuf), ::core::mem::transmute(pcchversionbuf.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lplangbuf), ::core::mem::transmute(pcchlangbuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetLanguage<P0>(hinstall: P0) -> u16
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetLanguage(hinstall : MSIHANDLE) -> u16);
    MsiGetLanguage(hinstall.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetLastErrorRecord() -> MSIHANDLE {
    ::windows_targets::link!("msi.dll" "system" fn MsiGetLastErrorRecord() -> MSIHANDLE);
    MsiGetLastErrorRecord()
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiGetMode<P0>(hinstall: P0, erunmode: MSIRUNMODE) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetMode(hinstall : MSIHANDLE, erunmode : MSIRUNMODE) -> super::super::Foundation:: BOOL);
    MsiGetMode(hinstall.into_param().abi(), erunmode)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetPatchFileListA<P0, P1>(szproductcode: P0, szpatchpackages: P1, pcfiles: *mut u32, pphfilerecords: *mut *mut MSIHANDLE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetPatchFileListA(szproductcode : ::windows_core::PCSTR, szpatchpackages : ::windows_core::PCSTR, pcfiles : *mut u32, pphfilerecords : *mut *mut MSIHANDLE) -> u32);
    MsiGetPatchFileListA(szproductcode.into_param().abi(), szpatchpackages.into_param().abi(), pcfiles, pphfilerecords)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetPatchFileListW<P0, P1>(szproductcode: P0, szpatchpackages: P1, pcfiles: *mut u32, pphfilerecords: *mut *mut MSIHANDLE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetPatchFileListW(szproductcode : ::windows_core::PCWSTR, szpatchpackages : ::windows_core::PCWSTR, pcfiles : *mut u32, pphfilerecords : *mut *mut MSIHANDLE) -> u32);
    MsiGetPatchFileListW(szproductcode.into_param().abi(), szpatchpackages.into_param().abi(), pcfiles, pphfilerecords)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetPatchInfoA<P0, P1>(szpatch: P0, szattribute: P1, lpvaluebuf: ::windows_core::PSTR, pcchvaluebuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetPatchInfoA(szpatch : ::windows_core::PCSTR, szattribute : ::windows_core::PCSTR, lpvaluebuf : ::windows_core::PSTR, pcchvaluebuf : *mut u32) -> u32);
    MsiGetPatchInfoA(szpatch.into_param().abi(), szattribute.into_param().abi(), ::core::mem::transmute(lpvaluebuf), ::core::mem::transmute(pcchvaluebuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetPatchInfoExA<P0, P1, P2, P3>(szpatchcode: P0, szproductcode: P1, szusersid: P2, dwcontext: MSIINSTALLCONTEXT, szproperty: P3, lpvalue: ::windows_core::PSTR, pcchvalue: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetPatchInfoExA(szpatchcode : ::windows_core::PCSTR, szproductcode : ::windows_core::PCSTR, szusersid : ::windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, szproperty : ::windows_core::PCSTR, lpvalue : ::windows_core::PSTR, pcchvalue : *mut u32) -> u32);
    MsiGetPatchInfoExA(szpatchcode.into_param().abi(), szproductcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, szproperty.into_param().abi(), ::core::mem::transmute(lpvalue), ::core::mem::transmute(pcchvalue.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetPatchInfoExW<P0, P1, P2, P3>(szpatchcode: P0, szproductcode: P1, szusersid: P2, dwcontext: MSIINSTALLCONTEXT, szproperty: P3, lpvalue: ::windows_core::PWSTR, pcchvalue: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetPatchInfoExW(szpatchcode : ::windows_core::PCWSTR, szproductcode : ::windows_core::PCWSTR, szusersid : ::windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, szproperty : ::windows_core::PCWSTR, lpvalue : ::windows_core::PWSTR, pcchvalue : *mut u32) -> u32);
    MsiGetPatchInfoExW(szpatchcode.into_param().abi(), szproductcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, szproperty.into_param().abi(), ::core::mem::transmute(lpvalue), ::core::mem::transmute(pcchvalue.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetPatchInfoW<P0, P1>(szpatch: P0, szattribute: P1, lpvaluebuf: ::windows_core::PWSTR, pcchvaluebuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetPatchInfoW(szpatch : ::windows_core::PCWSTR, szattribute : ::windows_core::PCWSTR, lpvaluebuf : ::windows_core::PWSTR, pcchvaluebuf : *mut u32) -> u32);
    MsiGetPatchInfoW(szpatch.into_param().abi(), szattribute.into_param().abi(), ::core::mem::transmute(lpvaluebuf), ::core::mem::transmute(pcchvaluebuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetProductCodeA<P0>(szcomponent: P0, lpbuf39: ::windows_core::PSTR) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetProductCodeA(szcomponent : ::windows_core::PCSTR, lpbuf39 : ::windows_core::PSTR) -> u32);
    MsiGetProductCodeA(szcomponent.into_param().abi(), ::core::mem::transmute(lpbuf39))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetProductCodeW<P0>(szcomponent: P0, lpbuf39: ::windows_core::PWSTR) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetProductCodeW(szcomponent : ::windows_core::PCWSTR, lpbuf39 : ::windows_core::PWSTR) -> u32);
    MsiGetProductCodeW(szcomponent.into_param().abi(), ::core::mem::transmute(lpbuf39))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetProductInfoA<P0, P1>(szproduct: P0, szattribute: P1, lpvaluebuf: ::windows_core::PSTR, pcchvaluebuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetProductInfoA(szproduct : ::windows_core::PCSTR, szattribute : ::windows_core::PCSTR, lpvaluebuf : ::windows_core::PSTR, pcchvaluebuf : *mut u32) -> u32);
    MsiGetProductInfoA(szproduct.into_param().abi(), szattribute.into_param().abi(), ::core::mem::transmute(lpvaluebuf), ::core::mem::transmute(pcchvaluebuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetProductInfoExA<P0, P1, P2>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, szproperty: P2, szvalue: ::windows_core::PSTR, pcchvalue: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetProductInfoExA(szproductcode : ::windows_core::PCSTR, szusersid : ::windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, szproperty : ::windows_core::PCSTR, szvalue : ::windows_core::PSTR, pcchvalue : *mut u32) -> u32);
    MsiGetProductInfoExA(szproductcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, szproperty.into_param().abi(), ::core::mem::transmute(szvalue), ::core::mem::transmute(pcchvalue.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetProductInfoExW<P0, P1, P2>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, szproperty: P2, szvalue: ::windows_core::PWSTR, pcchvalue: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetProductInfoExW(szproductcode : ::windows_core::PCWSTR, szusersid : ::windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, szproperty : ::windows_core::PCWSTR, szvalue : ::windows_core::PWSTR, pcchvalue : *mut u32) -> u32);
    MsiGetProductInfoExW(szproductcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, szproperty.into_param().abi(), ::core::mem::transmute(szvalue), ::core::mem::transmute(pcchvalue.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetProductInfoFromScriptA<P0>(szscriptfile: P0, lpproductbuf39: ::windows_core::PSTR, plgidlanguage: ::core::option::Option<*mut u16>, pdwversion: ::core::option::Option<*mut u32>, lpnamebuf: ::windows_core::PSTR, pcchnamebuf: ::core::option::Option<*mut u32>, lppackagebuf: ::windows_core::PSTR, pcchpackagebuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetProductInfoFromScriptA(szscriptfile : ::windows_core::PCSTR, lpproductbuf39 : ::windows_core::PSTR, plgidlanguage : *mut u16, pdwversion : *mut u32, lpnamebuf : ::windows_core::PSTR, pcchnamebuf : *mut u32, lppackagebuf : ::windows_core::PSTR, pcchpackagebuf : *mut u32) -> u32);
    MsiGetProductInfoFromScriptA(szscriptfile.into_param().abi(), ::core::mem::transmute(lpproductbuf39), ::core::mem::transmute(plgidlanguage.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdwversion.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpnamebuf), ::core::mem::transmute(pcchnamebuf.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lppackagebuf), ::core::mem::transmute(pcchpackagebuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetProductInfoFromScriptW<P0>(szscriptfile: P0, lpproductbuf39: ::windows_core::PWSTR, plgidlanguage: ::core::option::Option<*mut u16>, pdwversion: ::core::option::Option<*mut u32>, lpnamebuf: ::windows_core::PWSTR, pcchnamebuf: ::core::option::Option<*mut u32>, lppackagebuf: ::windows_core::PWSTR, pcchpackagebuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetProductInfoFromScriptW(szscriptfile : ::windows_core::PCWSTR, lpproductbuf39 : ::windows_core::PWSTR, plgidlanguage : *mut u16, pdwversion : *mut u32, lpnamebuf : ::windows_core::PWSTR, pcchnamebuf : *mut u32, lppackagebuf : ::windows_core::PWSTR, pcchpackagebuf : *mut u32) -> u32);
    MsiGetProductInfoFromScriptW(szscriptfile.into_param().abi(), ::core::mem::transmute(lpproductbuf39), ::core::mem::transmute(plgidlanguage.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdwversion.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpnamebuf), ::core::mem::transmute(pcchnamebuf.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lppackagebuf), ::core::mem::transmute(pcchpackagebuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetProductInfoW<P0, P1>(szproduct: P0, szattribute: P1, lpvaluebuf: ::windows_core::PWSTR, pcchvaluebuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetProductInfoW(szproduct : ::windows_core::PCWSTR, szattribute : ::windows_core::PCWSTR, lpvaluebuf : ::windows_core::PWSTR, pcchvaluebuf : *mut u32) -> u32);
    MsiGetProductInfoW(szproduct.into_param().abi(), szattribute.into_param().abi(), ::core::mem::transmute(lpvaluebuf), ::core::mem::transmute(pcchvaluebuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetProductPropertyA<P0, P1>(hproduct: P0, szproperty: P1, lpvaluebuf: ::windows_core::PSTR, pcchvaluebuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetProductPropertyA(hproduct : MSIHANDLE, szproperty : ::windows_core::PCSTR, lpvaluebuf : ::windows_core::PSTR, pcchvaluebuf : *mut u32) -> u32);
    MsiGetProductPropertyA(hproduct.into_param().abi(), szproperty.into_param().abi(), ::core::mem::transmute(lpvaluebuf), ::core::mem::transmute(pcchvaluebuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetProductPropertyW<P0, P1>(hproduct: P0, szproperty: P1, lpvaluebuf: ::windows_core::PWSTR, pcchvaluebuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetProductPropertyW(hproduct : MSIHANDLE, szproperty : ::windows_core::PCWSTR, lpvaluebuf : ::windows_core::PWSTR, pcchvaluebuf : *mut u32) -> u32);
    MsiGetProductPropertyW(hproduct.into_param().abi(), szproperty.into_param().abi(), ::core::mem::transmute(lpvaluebuf), ::core::mem::transmute(pcchvaluebuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetPropertyA<P0, P1>(hinstall: P0, szname: P1, szvaluebuf: ::windows_core::PSTR, pcchvaluebuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetPropertyA(hinstall : MSIHANDLE, szname : ::windows_core::PCSTR, szvaluebuf : ::windows_core::PSTR, pcchvaluebuf : *mut u32) -> u32);
    MsiGetPropertyA(hinstall.into_param().abi(), szname.into_param().abi(), ::core::mem::transmute(szvaluebuf), ::core::mem::transmute(pcchvaluebuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetPropertyW<P0, P1>(hinstall: P0, szname: P1, szvaluebuf: ::windows_core::PWSTR, pcchvaluebuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetPropertyW(hinstall : MSIHANDLE, szname : ::windows_core::PCWSTR, szvaluebuf : ::windows_core::PWSTR, pcchvaluebuf : *mut u32) -> u32);
    MsiGetPropertyW(hinstall.into_param().abi(), szname.into_param().abi(), ::core::mem::transmute(szvaluebuf), ::core::mem::transmute(pcchvaluebuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetShortcutTargetA<P0>(szshortcutpath: P0, szproductcode: ::windows_core::PSTR, szfeatureid: ::windows_core::PSTR, szcomponentcode: ::windows_core::PSTR) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetShortcutTargetA(szshortcutpath : ::windows_core::PCSTR, szproductcode : ::windows_core::PSTR, szfeatureid : ::windows_core::PSTR, szcomponentcode : ::windows_core::PSTR) -> u32);
    MsiGetShortcutTargetA(szshortcutpath.into_param().abi(), ::core::mem::transmute(szproductcode), ::core::mem::transmute(szfeatureid), ::core::mem::transmute(szcomponentcode))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetShortcutTargetW<P0>(szshortcutpath: P0, szproductcode: ::windows_core::PWSTR, szfeatureid: ::windows_core::PWSTR, szcomponentcode: ::windows_core::PWSTR) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetShortcutTargetW(szshortcutpath : ::windows_core::PCWSTR, szproductcode : ::windows_core::PWSTR, szfeatureid : ::windows_core::PWSTR, szcomponentcode : ::windows_core::PWSTR) -> u32);
    MsiGetShortcutTargetW(szshortcutpath.into_param().abi(), ::core::mem::transmute(szproductcode), ::core::mem::transmute(szfeatureid), ::core::mem::transmute(szcomponentcode))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetSourcePathA<P0, P1>(hinstall: P0, szfolder: P1, szpathbuf: ::windows_core::PSTR, pcchpathbuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetSourcePathA(hinstall : MSIHANDLE, szfolder : ::windows_core::PCSTR, szpathbuf : ::windows_core::PSTR, pcchpathbuf : *mut u32) -> u32);
    MsiGetSourcePathA(hinstall.into_param().abi(), szfolder.into_param().abi(), ::core::mem::transmute(szpathbuf), ::core::mem::transmute(pcchpathbuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetSourcePathW<P0, P1>(hinstall: P0, szfolder: P1, szpathbuf: ::windows_core::PWSTR, pcchpathbuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetSourcePathW(hinstall : MSIHANDLE, szfolder : ::windows_core::PCWSTR, szpathbuf : ::windows_core::PWSTR, pcchpathbuf : *mut u32) -> u32);
    MsiGetSourcePathW(hinstall.into_param().abi(), szfolder.into_param().abi(), ::core::mem::transmute(szpathbuf), ::core::mem::transmute(pcchpathbuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetSummaryInformationA<P0, P1>(hdatabase: P0, szdatabasepath: P1, uiupdatecount: u32, phsummaryinfo: *mut MSIHANDLE) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetSummaryInformationA(hdatabase : MSIHANDLE, szdatabasepath : ::windows_core::PCSTR, uiupdatecount : u32, phsummaryinfo : *mut MSIHANDLE) -> u32);
    MsiGetSummaryInformationA(hdatabase.into_param().abi(), szdatabasepath.into_param().abi(), uiupdatecount, phsummaryinfo)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetSummaryInformationW<P0, P1>(hdatabase: P0, szdatabasepath: P1, uiupdatecount: u32, phsummaryinfo: *mut MSIHANDLE) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetSummaryInformationW(hdatabase : MSIHANDLE, szdatabasepath : ::windows_core::PCWSTR, uiupdatecount : u32, phsummaryinfo : *mut MSIHANDLE) -> u32);
    MsiGetSummaryInformationW(hdatabase.into_param().abi(), szdatabasepath.into_param().abi(), uiupdatecount, phsummaryinfo)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetTargetPathA<P0, P1>(hinstall: P0, szfolder: P1, szpathbuf: ::windows_core::PSTR, pcchpathbuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetTargetPathA(hinstall : MSIHANDLE, szfolder : ::windows_core::PCSTR, szpathbuf : ::windows_core::PSTR, pcchpathbuf : *mut u32) -> u32);
    MsiGetTargetPathA(hinstall.into_param().abi(), szfolder.into_param().abi(), ::core::mem::transmute(szpathbuf), ::core::mem::transmute(pcchpathbuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetTargetPathW<P0, P1>(hinstall: P0, szfolder: P1, szpathbuf: ::windows_core::PWSTR, pcchpathbuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetTargetPathW(hinstall : MSIHANDLE, szfolder : ::windows_core::PCWSTR, szpathbuf : ::windows_core::PWSTR, pcchpathbuf : *mut u32) -> u32);
    MsiGetTargetPathW(hinstall.into_param().abi(), szfolder.into_param().abi(), ::core::mem::transmute(szpathbuf), ::core::mem::transmute(pcchpathbuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetUserInfoA<P0>(szproduct: P0, lpusernamebuf: ::windows_core::PSTR, pcchusernamebuf: ::core::option::Option<*mut u32>, lporgnamebuf: ::windows_core::PSTR, pcchorgnamebuf: ::core::option::Option<*mut u32>, lpserialbuf: ::windows_core::PSTR, pcchserialbuf: ::core::option::Option<*mut u32>) -> USERINFOSTATE
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetUserInfoA(szproduct : ::windows_core::PCSTR, lpusernamebuf : ::windows_core::PSTR, pcchusernamebuf : *mut u32, lporgnamebuf : ::windows_core::PSTR, pcchorgnamebuf : *mut u32, lpserialbuf : ::windows_core::PSTR, pcchserialbuf : *mut u32) -> USERINFOSTATE);
    MsiGetUserInfoA(szproduct.into_param().abi(), ::core::mem::transmute(lpusernamebuf), ::core::mem::transmute(pcchusernamebuf.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lporgnamebuf), ::core::mem::transmute(pcchorgnamebuf.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpserialbuf), ::core::mem::transmute(pcchserialbuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiGetUserInfoW<P0>(szproduct: P0, lpusernamebuf: ::windows_core::PWSTR, pcchusernamebuf: ::core::option::Option<*mut u32>, lporgnamebuf: ::windows_core::PWSTR, pcchorgnamebuf: ::core::option::Option<*mut u32>, lpserialbuf: ::windows_core::PWSTR, pcchserialbuf: ::core::option::Option<*mut u32>) -> USERINFOSTATE
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiGetUserInfoW(szproduct : ::windows_core::PCWSTR, lpusernamebuf : ::windows_core::PWSTR, pcchusernamebuf : *mut u32, lporgnamebuf : ::windows_core::PWSTR, pcchorgnamebuf : *mut u32, lpserialbuf : ::windows_core::PWSTR, pcchserialbuf : *mut u32) -> USERINFOSTATE);
    MsiGetUserInfoW(szproduct.into_param().abi(), ::core::mem::transmute(lpusernamebuf), ::core::mem::transmute(pcchusernamebuf.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lporgnamebuf), ::core::mem::transmute(pcchorgnamebuf.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpserialbuf), ::core::mem::transmute(pcchserialbuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiInstallMissingComponentA<P0, P1>(szproduct: P0, szcomponent: P1, einstallstate: INSTALLSTATE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiInstallMissingComponentA(szproduct : ::windows_core::PCSTR, szcomponent : ::windows_core::PCSTR, einstallstate : INSTALLSTATE) -> u32);
    MsiInstallMissingComponentA(szproduct.into_param().abi(), szcomponent.into_param().abi(), einstallstate)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiInstallMissingComponentW<P0, P1>(szproduct: P0, szcomponent: P1, einstallstate: INSTALLSTATE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiInstallMissingComponentW(szproduct : ::windows_core::PCWSTR, szcomponent : ::windows_core::PCWSTR, einstallstate : INSTALLSTATE) -> u32);
    MsiInstallMissingComponentW(szproduct.into_param().abi(), szcomponent.into_param().abi(), einstallstate)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiInstallMissingFileA<P0, P1>(szproduct: P0, szfile: P1) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiInstallMissingFileA(szproduct : ::windows_core::PCSTR, szfile : ::windows_core::PCSTR) -> u32);
    MsiInstallMissingFileA(szproduct.into_param().abi(), szfile.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiInstallMissingFileW<P0, P1>(szproduct: P0, szfile: P1) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiInstallMissingFileW(szproduct : ::windows_core::PCWSTR, szfile : ::windows_core::PCWSTR) -> u32);
    MsiInstallMissingFileW(szproduct.into_param().abi(), szfile.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiInstallProductA<P0, P1>(szpackagepath: P0, szcommandline: P1) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiInstallProductA(szpackagepath : ::windows_core::PCSTR, szcommandline : ::windows_core::PCSTR) -> u32);
    MsiInstallProductA(szpackagepath.into_param().abi(), szcommandline.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiInstallProductW<P0, P1>(szpackagepath: P0, szcommandline: P1) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiInstallProductW(szpackagepath : ::windows_core::PCWSTR, szcommandline : ::windows_core::PCWSTR) -> u32);
    MsiInstallProductW(szpackagepath.into_param().abi(), szcommandline.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiIsProductElevatedA<P0>(szproduct: P0, pfelevated: *mut super::super::Foundation::BOOL) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiIsProductElevatedA(szproduct : ::windows_core::PCSTR, pfelevated : *mut super::super::Foundation:: BOOL) -> u32);
    MsiIsProductElevatedA(szproduct.into_param().abi(), pfelevated)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiIsProductElevatedW<P0>(szproduct: P0, pfelevated: *mut super::super::Foundation::BOOL) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiIsProductElevatedW(szproduct : ::windows_core::PCWSTR, pfelevated : *mut super::super::Foundation:: BOOL) -> u32);
    MsiIsProductElevatedW(szproduct.into_param().abi(), pfelevated)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiJoinTransaction<P0>(htransactionhandle: P0, dwtransactionattributes: u32, phchangeofownerevent: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiJoinTransaction(htransactionhandle : MSIHANDLE, dwtransactionattributes : u32, phchangeofownerevent : *mut super::super::Foundation:: HANDLE) -> u32);
    MsiJoinTransaction(htransactionhandle.into_param().abi(), dwtransactionattributes, phchangeofownerevent)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiLocateComponentA<P0>(szcomponent: P0, lppathbuf: ::windows_core::PSTR, pcchbuf: ::core::option::Option<*mut u32>) -> INSTALLSTATE
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiLocateComponentA(szcomponent : ::windows_core::PCSTR, lppathbuf : ::windows_core::PSTR, pcchbuf : *mut u32) -> INSTALLSTATE);
    MsiLocateComponentA(szcomponent.into_param().abi(), ::core::mem::transmute(lppathbuf), ::core::mem::transmute(pcchbuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiLocateComponentW<P0>(szcomponent: P0, lppathbuf: ::windows_core::PWSTR, pcchbuf: ::core::option::Option<*mut u32>) -> INSTALLSTATE
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiLocateComponentW(szcomponent : ::windows_core::PCWSTR, lppathbuf : ::windows_core::PWSTR, pcchbuf : *mut u32) -> INSTALLSTATE);
    MsiLocateComponentW(szcomponent.into_param().abi(), ::core::mem::transmute(lppathbuf), ::core::mem::transmute(pcchbuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiNotifySidChangeA<P0, P1>(poldsid: P0, pnewsid: P1) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiNotifySidChangeA(poldsid : ::windows_core::PCSTR, pnewsid : ::windows_core::PCSTR) -> u32);
    MsiNotifySidChangeA(poldsid.into_param().abi(), pnewsid.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiNotifySidChangeW<P0, P1>(poldsid: P0, pnewsid: P1) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiNotifySidChangeW(poldsid : ::windows_core::PCWSTR, pnewsid : ::windows_core::PCWSTR) -> u32);
    MsiNotifySidChangeW(poldsid.into_param().abi(), pnewsid.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiOpenDatabaseA<P0, P1>(szdatabasepath: P0, szpersist: P1, phdatabase: *mut MSIHANDLE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiOpenDatabaseA(szdatabasepath : ::windows_core::PCSTR, szpersist : ::windows_core::PCSTR, phdatabase : *mut MSIHANDLE) -> u32);
    MsiOpenDatabaseA(szdatabasepath.into_param().abi(), szpersist.into_param().abi(), phdatabase)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiOpenDatabaseW<P0, P1>(szdatabasepath: P0, szpersist: P1, phdatabase: *mut MSIHANDLE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiOpenDatabaseW(szdatabasepath : ::windows_core::PCWSTR, szpersist : ::windows_core::PCWSTR, phdatabase : *mut MSIHANDLE) -> u32);
    MsiOpenDatabaseW(szdatabasepath.into_param().abi(), szpersist.into_param().abi(), phdatabase)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiOpenPackageA<P0>(szpackagepath: P0, hproduct: *mut MSIHANDLE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiOpenPackageA(szpackagepath : ::windows_core::PCSTR, hproduct : *mut MSIHANDLE) -> u32);
    MsiOpenPackageA(szpackagepath.into_param().abi(), hproduct)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiOpenPackageExA<P0>(szpackagepath: P0, dwoptions: u32, hproduct: *mut MSIHANDLE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiOpenPackageExA(szpackagepath : ::windows_core::PCSTR, dwoptions : u32, hproduct : *mut MSIHANDLE) -> u32);
    MsiOpenPackageExA(szpackagepath.into_param().abi(), dwoptions, hproduct)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiOpenPackageExW<P0>(szpackagepath: P0, dwoptions: u32, hproduct: *mut MSIHANDLE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiOpenPackageExW(szpackagepath : ::windows_core::PCWSTR, dwoptions : u32, hproduct : *mut MSIHANDLE) -> u32);
    MsiOpenPackageExW(szpackagepath.into_param().abi(), dwoptions, hproduct)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiOpenPackageW<P0>(szpackagepath: P0, hproduct: *mut MSIHANDLE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiOpenPackageW(szpackagepath : ::windows_core::PCWSTR, hproduct : *mut MSIHANDLE) -> u32);
    MsiOpenPackageW(szpackagepath.into_param().abi(), hproduct)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiOpenProductA<P0>(szproduct: P0, hproduct: *mut MSIHANDLE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiOpenProductA(szproduct : ::windows_core::PCSTR, hproduct : *mut MSIHANDLE) -> u32);
    MsiOpenProductA(szproduct.into_param().abi(), hproduct)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiOpenProductW<P0>(szproduct: P0, hproduct: *mut MSIHANDLE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiOpenProductW(szproduct : ::windows_core::PCWSTR, hproduct : *mut MSIHANDLE) -> u32);
    MsiOpenProductW(szproduct.into_param().abi(), hproduct)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiPreviewBillboardA<P0, P1, P2>(hpreview: P0, szcontrolname: P1, szbillboard: P2) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiPreviewBillboardA(hpreview : MSIHANDLE, szcontrolname : ::windows_core::PCSTR, szbillboard : ::windows_core::PCSTR) -> u32);
    MsiPreviewBillboardA(hpreview.into_param().abi(), szcontrolname.into_param().abi(), szbillboard.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiPreviewBillboardW<P0, P1, P2>(hpreview: P0, szcontrolname: P1, szbillboard: P2) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiPreviewBillboardW(hpreview : MSIHANDLE, szcontrolname : ::windows_core::PCWSTR, szbillboard : ::windows_core::PCWSTR) -> u32);
    MsiPreviewBillboardW(hpreview.into_param().abi(), szcontrolname.into_param().abi(), szbillboard.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiPreviewDialogA<P0, P1>(hpreview: P0, szdialogname: P1) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiPreviewDialogA(hpreview : MSIHANDLE, szdialogname : ::windows_core::PCSTR) -> u32);
    MsiPreviewDialogA(hpreview.into_param().abi(), szdialogname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiPreviewDialogW<P0, P1>(hpreview: P0, szdialogname: P1) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiPreviewDialogW(hpreview : MSIHANDLE, szdialogname : ::windows_core::PCWSTR) -> u32);
    MsiPreviewDialogW(hpreview.into_param().abi(), szdialogname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn MsiProcessAdvertiseScriptA<P0, P1, P2, P3, P4>(szscriptfile: P0, sziconfolder: P1, hregdata: P2, fshortcuts: P3, fremoveitems: P4) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<super::Registry::HKEY>,
    P3: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    P4: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiProcessAdvertiseScriptA(szscriptfile : ::windows_core::PCSTR, sziconfolder : ::windows_core::PCSTR, hregdata : super::Registry:: HKEY, fshortcuts : super::super::Foundation:: BOOL, fremoveitems : super::super::Foundation:: BOOL) -> u32);
    MsiProcessAdvertiseScriptA(szscriptfile.into_param().abi(), sziconfolder.into_param().abi(), hregdata.into_param().abi(), fshortcuts.into_param().abi(), fremoveitems.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn MsiProcessAdvertiseScriptW<P0, P1, P2, P3, P4>(szscriptfile: P0, sziconfolder: P1, hregdata: P2, fshortcuts: P3, fremoveitems: P4) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<super::Registry::HKEY>,
    P3: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    P4: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiProcessAdvertiseScriptW(szscriptfile : ::windows_core::PCWSTR, sziconfolder : ::windows_core::PCWSTR, hregdata : super::Registry:: HKEY, fshortcuts : super::super::Foundation:: BOOL, fremoveitems : super::super::Foundation:: BOOL) -> u32);
    MsiProcessAdvertiseScriptW(szscriptfile.into_param().abi(), sziconfolder.into_param().abi(), hregdata.into_param().abi(), fshortcuts.into_param().abi(), fremoveitems.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiProcessMessage<P0, P1>(hinstall: P0, emessagetype: INSTALLMESSAGE, hrecord: P1) -> i32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiProcessMessage(hinstall : MSIHANDLE, emessagetype : INSTALLMESSAGE, hrecord : MSIHANDLE) -> i32);
    MsiProcessMessage(hinstall.into_param().abi(), emessagetype, hrecord.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiProvideAssemblyA<P0, P1>(szassemblyname: P0, szappcontext: P1, dwinstallmode: INSTALLMODE, dwassemblyinfo: MSIASSEMBLYINFO, lppathbuf: ::windows_core::PSTR, pcchpathbuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiProvideAssemblyA(szassemblyname : ::windows_core::PCSTR, szappcontext : ::windows_core::PCSTR, dwinstallmode : u32, dwassemblyinfo : MSIASSEMBLYINFO, lppathbuf : ::windows_core::PSTR, pcchpathbuf : *mut u32) -> u32);
    MsiProvideAssemblyA(szassemblyname.into_param().abi(), szappcontext.into_param().abi(), dwinstallmode.0 as _, dwassemblyinfo, ::core::mem::transmute(lppathbuf), ::core::mem::transmute(pcchpathbuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiProvideAssemblyW<P0, P1>(szassemblyname: P0, szappcontext: P1, dwinstallmode: INSTALLMODE, dwassemblyinfo: MSIASSEMBLYINFO, lppathbuf: ::windows_core::PWSTR, pcchpathbuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiProvideAssemblyW(szassemblyname : ::windows_core::PCWSTR, szappcontext : ::windows_core::PCWSTR, dwinstallmode : u32, dwassemblyinfo : MSIASSEMBLYINFO, lppathbuf : ::windows_core::PWSTR, pcchpathbuf : *mut u32) -> u32);
    MsiProvideAssemblyW(szassemblyname.into_param().abi(), szappcontext.into_param().abi(), dwinstallmode.0 as _, dwassemblyinfo, ::core::mem::transmute(lppathbuf), ::core::mem::transmute(pcchpathbuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiProvideComponentA<P0, P1, P2>(szproduct: P0, szfeature: P1, szcomponent: P2, dwinstallmode: INSTALLMODE, lppathbuf: ::windows_core::PSTR, pcchpathbuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiProvideComponentA(szproduct : ::windows_core::PCSTR, szfeature : ::windows_core::PCSTR, szcomponent : ::windows_core::PCSTR, dwinstallmode : u32, lppathbuf : ::windows_core::PSTR, pcchpathbuf : *mut u32) -> u32);
    MsiProvideComponentA(szproduct.into_param().abi(), szfeature.into_param().abi(), szcomponent.into_param().abi(), dwinstallmode.0 as _, ::core::mem::transmute(lppathbuf), ::core::mem::transmute(pcchpathbuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiProvideComponentW<P0, P1, P2>(szproduct: P0, szfeature: P1, szcomponent: P2, dwinstallmode: INSTALLMODE, lppathbuf: ::windows_core::PWSTR, pcchpathbuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiProvideComponentW(szproduct : ::windows_core::PCWSTR, szfeature : ::windows_core::PCWSTR, szcomponent : ::windows_core::PCWSTR, dwinstallmode : u32, lppathbuf : ::windows_core::PWSTR, pcchpathbuf : *mut u32) -> u32);
    MsiProvideComponentW(szproduct.into_param().abi(), szfeature.into_param().abi(), szcomponent.into_param().abi(), dwinstallmode.0 as _, ::core::mem::transmute(lppathbuf), ::core::mem::transmute(pcchpathbuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiProvideQualifiedComponentA<P0, P1>(szcategory: P0, szqualifier: P1, dwinstallmode: INSTALLMODE, lppathbuf: ::windows_core::PSTR, pcchpathbuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiProvideQualifiedComponentA(szcategory : ::windows_core::PCSTR, szqualifier : ::windows_core::PCSTR, dwinstallmode : u32, lppathbuf : ::windows_core::PSTR, pcchpathbuf : *mut u32) -> u32);
    MsiProvideQualifiedComponentA(szcategory.into_param().abi(), szqualifier.into_param().abi(), dwinstallmode.0 as _, ::core::mem::transmute(lppathbuf), ::core::mem::transmute(pcchpathbuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiProvideQualifiedComponentExA<P0, P1, P2>(szcategory: P0, szqualifier: P1, dwinstallmode: INSTALLMODE, szproduct: P2, dwunused1: u32, dwunused2: u32, lppathbuf: ::windows_core::PSTR, pcchpathbuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiProvideQualifiedComponentExA(szcategory : ::windows_core::PCSTR, szqualifier : ::windows_core::PCSTR, dwinstallmode : u32, szproduct : ::windows_core::PCSTR, dwunused1 : u32, dwunused2 : u32, lppathbuf : ::windows_core::PSTR, pcchpathbuf : *mut u32) -> u32);
    MsiProvideQualifiedComponentExA(szcategory.into_param().abi(), szqualifier.into_param().abi(), dwinstallmode.0 as _, szproduct.into_param().abi(), dwunused1, dwunused2, ::core::mem::transmute(lppathbuf), ::core::mem::transmute(pcchpathbuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiProvideQualifiedComponentExW<P0, P1, P2>(szcategory: P0, szqualifier: P1, dwinstallmode: INSTALLMODE, szproduct: P2, dwunused1: u32, dwunused2: u32, lppathbuf: ::windows_core::PWSTR, pcchpathbuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiProvideQualifiedComponentExW(szcategory : ::windows_core::PCWSTR, szqualifier : ::windows_core::PCWSTR, dwinstallmode : u32, szproduct : ::windows_core::PCWSTR, dwunused1 : u32, dwunused2 : u32, lppathbuf : ::windows_core::PWSTR, pcchpathbuf : *mut u32) -> u32);
    MsiProvideQualifiedComponentExW(szcategory.into_param().abi(), szqualifier.into_param().abi(), dwinstallmode.0 as _, szproduct.into_param().abi(), dwunused1, dwunused2, ::core::mem::transmute(lppathbuf), ::core::mem::transmute(pcchpathbuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiProvideQualifiedComponentW<P0, P1>(szcategory: P0, szqualifier: P1, dwinstallmode: INSTALLMODE, lppathbuf: ::windows_core::PWSTR, pcchpathbuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiProvideQualifiedComponentW(szcategory : ::windows_core::PCWSTR, szqualifier : ::windows_core::PCWSTR, dwinstallmode : u32, lppathbuf : ::windows_core::PWSTR, pcchpathbuf : *mut u32) -> u32);
    MsiProvideQualifiedComponentW(szcategory.into_param().abi(), szqualifier.into_param().abi(), dwinstallmode.0 as _, ::core::mem::transmute(lppathbuf), ::core::mem::transmute(pcchpathbuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiQueryComponentStateA<P0, P1, P2>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, szcomponentcode: P2, pdwstate: ::core::option::Option<*mut INSTALLSTATE>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiQueryComponentStateA(szproductcode : ::windows_core::PCSTR, szusersid : ::windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, szcomponentcode : ::windows_core::PCSTR, pdwstate : *mut INSTALLSTATE) -> u32);
    MsiQueryComponentStateA(szproductcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, szcomponentcode.into_param().abi(), ::core::mem::transmute(pdwstate.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiQueryComponentStateW<P0, P1, P2>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, szcomponentcode: P2, pdwstate: ::core::option::Option<*mut INSTALLSTATE>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiQueryComponentStateW(szproductcode : ::windows_core::PCWSTR, szusersid : ::windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, szcomponentcode : ::windows_core::PCWSTR, pdwstate : *mut INSTALLSTATE) -> u32);
    MsiQueryComponentStateW(szproductcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, szcomponentcode.into_param().abi(), ::core::mem::transmute(pdwstate.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiQueryFeatureStateA<P0, P1>(szproduct: P0, szfeature: P1) -> INSTALLSTATE
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiQueryFeatureStateA(szproduct : ::windows_core::PCSTR, szfeature : ::windows_core::PCSTR) -> INSTALLSTATE);
    MsiQueryFeatureStateA(szproduct.into_param().abi(), szfeature.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiQueryFeatureStateExA<P0, P1, P2>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, szfeature: P2, pdwstate: ::core::option::Option<*mut INSTALLSTATE>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiQueryFeatureStateExA(szproductcode : ::windows_core::PCSTR, szusersid : ::windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, szfeature : ::windows_core::PCSTR, pdwstate : *mut INSTALLSTATE) -> u32);
    MsiQueryFeatureStateExA(szproductcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, szfeature.into_param().abi(), ::core::mem::transmute(pdwstate.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiQueryFeatureStateExW<P0, P1, P2>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, szfeature: P2, pdwstate: ::core::option::Option<*mut INSTALLSTATE>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiQueryFeatureStateExW(szproductcode : ::windows_core::PCWSTR, szusersid : ::windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, szfeature : ::windows_core::PCWSTR, pdwstate : *mut INSTALLSTATE) -> u32);
    MsiQueryFeatureStateExW(szproductcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, szfeature.into_param().abi(), ::core::mem::transmute(pdwstate.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiQueryFeatureStateW<P0, P1>(szproduct: P0, szfeature: P1) -> INSTALLSTATE
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiQueryFeatureStateW(szproduct : ::windows_core::PCWSTR, szfeature : ::windows_core::PCWSTR) -> INSTALLSTATE);
    MsiQueryFeatureStateW(szproduct.into_param().abi(), szfeature.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiQueryProductStateA<P0>(szproduct: P0) -> INSTALLSTATE
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiQueryProductStateA(szproduct : ::windows_core::PCSTR) -> INSTALLSTATE);
    MsiQueryProductStateA(szproduct.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiQueryProductStateW<P0>(szproduct: P0) -> INSTALLSTATE
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiQueryProductStateW(szproduct : ::windows_core::PCWSTR) -> INSTALLSTATE);
    MsiQueryProductStateW(szproduct.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiRecordClearData<P0>(hrecord: P0) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiRecordClearData(hrecord : MSIHANDLE) -> u32);
    MsiRecordClearData(hrecord.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiRecordDataSize<P0>(hrecord: P0, ifield: u32) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiRecordDataSize(hrecord : MSIHANDLE, ifield : u32) -> u32);
    MsiRecordDataSize(hrecord.into_param().abi(), ifield)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiRecordGetFieldCount<P0>(hrecord: P0) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiRecordGetFieldCount(hrecord : MSIHANDLE) -> u32);
    MsiRecordGetFieldCount(hrecord.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiRecordGetInteger<P0>(hrecord: P0, ifield: u32) -> i32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiRecordGetInteger(hrecord : MSIHANDLE, ifield : u32) -> i32);
    MsiRecordGetInteger(hrecord.into_param().abi(), ifield)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiRecordGetStringA<P0>(hrecord: P0, ifield: u32, szvaluebuf: ::windows_core::PSTR, pcchvaluebuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiRecordGetStringA(hrecord : MSIHANDLE, ifield : u32, szvaluebuf : ::windows_core::PSTR, pcchvaluebuf : *mut u32) -> u32);
    MsiRecordGetStringA(hrecord.into_param().abi(), ifield, ::core::mem::transmute(szvaluebuf), ::core::mem::transmute(pcchvaluebuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiRecordGetStringW<P0>(hrecord: P0, ifield: u32, szvaluebuf: ::windows_core::PWSTR, pcchvaluebuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiRecordGetStringW(hrecord : MSIHANDLE, ifield : u32, szvaluebuf : ::windows_core::PWSTR, pcchvaluebuf : *mut u32) -> u32);
    MsiRecordGetStringW(hrecord.into_param().abi(), ifield, ::core::mem::transmute(szvaluebuf), ::core::mem::transmute(pcchvaluebuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiRecordIsNull<P0>(hrecord: P0, ifield: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiRecordIsNull(hrecord : MSIHANDLE, ifield : u32) -> super::super::Foundation:: BOOL);
    MsiRecordIsNull(hrecord.into_param().abi(), ifield)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiRecordReadStream<P0>(hrecord: P0, ifield: u32, szdatabuf: ::windows_core::PSTR, pcbdatabuf: *mut u32) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiRecordReadStream(hrecord : MSIHANDLE, ifield : u32, szdatabuf : ::windows_core::PSTR, pcbdatabuf : *mut u32) -> u32);
    MsiRecordReadStream(hrecord.into_param().abi(), ifield, ::core::mem::transmute(szdatabuf), pcbdatabuf)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiRecordSetInteger<P0>(hrecord: P0, ifield: u32, ivalue: i32) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiRecordSetInteger(hrecord : MSIHANDLE, ifield : u32, ivalue : i32) -> u32);
    MsiRecordSetInteger(hrecord.into_param().abi(), ifield, ivalue)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiRecordSetStreamA<P0, P1>(hrecord: P0, ifield: u32, szfilepath: P1) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiRecordSetStreamA(hrecord : MSIHANDLE, ifield : u32, szfilepath : ::windows_core::PCSTR) -> u32);
    MsiRecordSetStreamA(hrecord.into_param().abi(), ifield, szfilepath.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiRecordSetStreamW<P0, P1>(hrecord: P0, ifield: u32, szfilepath: P1) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiRecordSetStreamW(hrecord : MSIHANDLE, ifield : u32, szfilepath : ::windows_core::PCWSTR) -> u32);
    MsiRecordSetStreamW(hrecord.into_param().abi(), ifield, szfilepath.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiRecordSetStringA<P0, P1>(hrecord: P0, ifield: u32, szvalue: P1) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiRecordSetStringA(hrecord : MSIHANDLE, ifield : u32, szvalue : ::windows_core::PCSTR) -> u32);
    MsiRecordSetStringA(hrecord.into_param().abi(), ifield, szvalue.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiRecordSetStringW<P0, P1>(hrecord: P0, ifield: u32, szvalue: P1) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiRecordSetStringW(hrecord : MSIHANDLE, ifield : u32, szvalue : ::windows_core::PCWSTR) -> u32);
    MsiRecordSetStringW(hrecord.into_param().abi(), ifield, szvalue.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiReinstallFeatureA<P0, P1>(szproduct: P0, szfeature: P1, dwreinstallmode: REINSTALLMODE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiReinstallFeatureA(szproduct : ::windows_core::PCSTR, szfeature : ::windows_core::PCSTR, dwreinstallmode : u32) -> u32);
    MsiReinstallFeatureA(szproduct.into_param().abi(), szfeature.into_param().abi(), dwreinstallmode.0 as _)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiReinstallFeatureW<P0, P1>(szproduct: P0, szfeature: P1, dwreinstallmode: REINSTALLMODE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiReinstallFeatureW(szproduct : ::windows_core::PCWSTR, szfeature : ::windows_core::PCWSTR, dwreinstallmode : u32) -> u32);
    MsiReinstallFeatureW(szproduct.into_param().abi(), szfeature.into_param().abi(), dwreinstallmode.0 as _)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiReinstallProductA<P0>(szproduct: P0, szreinstallmode: REINSTALLMODE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiReinstallProductA(szproduct : ::windows_core::PCSTR, szreinstallmode : u32) -> u32);
    MsiReinstallProductA(szproduct.into_param().abi(), szreinstallmode.0 as _)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiReinstallProductW<P0>(szproduct: P0, szreinstallmode: REINSTALLMODE) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiReinstallProductW(szproduct : ::windows_core::PCWSTR, szreinstallmode : u32) -> u32);
    MsiReinstallProductW(szproduct.into_param().abi(), szreinstallmode.0 as _)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiRemovePatchesA<P0, P1, P2>(szpatchlist: P0, szproductcode: P1, euninstalltype: INSTALLTYPE, szpropertylist: P2) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiRemovePatchesA(szpatchlist : ::windows_core::PCSTR, szproductcode : ::windows_core::PCSTR, euninstalltype : INSTALLTYPE, szpropertylist : ::windows_core::PCSTR) -> u32);
    MsiRemovePatchesA(szpatchlist.into_param().abi(), szproductcode.into_param().abi(), euninstalltype, szpropertylist.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiRemovePatchesW<P0, P1, P2>(szpatchlist: P0, szproductcode: P1, euninstalltype: INSTALLTYPE, szpropertylist: P2) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiRemovePatchesW(szpatchlist : ::windows_core::PCWSTR, szproductcode : ::windows_core::PCWSTR, euninstalltype : INSTALLTYPE, szpropertylist : ::windows_core::PCWSTR) -> u32);
    MsiRemovePatchesW(szpatchlist.into_param().abi(), szproductcode.into_param().abi(), euninstalltype, szpropertylist.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSequenceA<P0, P1>(hinstall: P0, sztable: P1, isequencemode: i32) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSequenceA(hinstall : MSIHANDLE, sztable : ::windows_core::PCSTR, isequencemode : i32) -> u32);
    MsiSequenceA(hinstall.into_param().abi(), sztable.into_param().abi(), isequencemode)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSequenceW<P0, P1>(hinstall: P0, sztable: P1, isequencemode: i32) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSequenceW(hinstall : MSIHANDLE, sztable : ::windows_core::PCWSTR, isequencemode : i32) -> u32);
    MsiSequenceW(hinstall.into_param().abi(), sztable.into_param().abi(), isequencemode)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSetComponentStateA<P0, P1>(hinstall: P0, szcomponent: P1, istate: INSTALLSTATE) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSetComponentStateA(hinstall : MSIHANDLE, szcomponent : ::windows_core::PCSTR, istate : INSTALLSTATE) -> u32);
    MsiSetComponentStateA(hinstall.into_param().abi(), szcomponent.into_param().abi(), istate)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSetComponentStateW<P0, P1>(hinstall: P0, szcomponent: P1, istate: INSTALLSTATE) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSetComponentStateW(hinstall : MSIHANDLE, szcomponent : ::windows_core::PCWSTR, istate : INSTALLSTATE) -> u32);
    MsiSetComponentStateW(hinstall.into_param().abi(), szcomponent.into_param().abi(), istate)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSetExternalUIA(puihandler: INSTALLUI_HANDLERA, dwmessagefilter: u32, pvcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> INSTALLUI_HANDLERA {
    ::windows_targets::link!("msi.dll" "system" fn MsiSetExternalUIA(puihandler : INSTALLUI_HANDLERA, dwmessagefilter : u32, pvcontext : *const ::core::ffi::c_void) -> INSTALLUI_HANDLERA);
    MsiSetExternalUIA(puihandler, dwmessagefilter, ::core::mem::transmute(pvcontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSetExternalUIRecord(puihandler: PINSTALLUI_HANDLER_RECORD, dwmessagefilter: u32, pvcontext: ::core::option::Option<*const ::core::ffi::c_void>, ppuiprevhandler: PINSTALLUI_HANDLER_RECORD) -> u32 {
    ::windows_targets::link!("msi.dll" "system" fn MsiSetExternalUIRecord(puihandler : PINSTALLUI_HANDLER_RECORD, dwmessagefilter : u32, pvcontext : *const ::core::ffi::c_void, ppuiprevhandler : PINSTALLUI_HANDLER_RECORD) -> u32);
    MsiSetExternalUIRecord(puihandler, dwmessagefilter, ::core::mem::transmute(pvcontext.unwrap_or(::std::ptr::null())), ppuiprevhandler)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSetExternalUIW(puihandler: INSTALLUI_HANDLERW, dwmessagefilter: u32, pvcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> INSTALLUI_HANDLERW {
    ::windows_targets::link!("msi.dll" "system" fn MsiSetExternalUIW(puihandler : INSTALLUI_HANDLERW, dwmessagefilter : u32, pvcontext : *const ::core::ffi::c_void) -> INSTALLUI_HANDLERW);
    MsiSetExternalUIW(puihandler, dwmessagefilter, ::core::mem::transmute(pvcontext.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSetFeatureAttributesA<P0, P1>(hinstall: P0, szfeature: P1, dwattributes: u32) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSetFeatureAttributesA(hinstall : MSIHANDLE, szfeature : ::windows_core::PCSTR, dwattributes : u32) -> u32);
    MsiSetFeatureAttributesA(hinstall.into_param().abi(), szfeature.into_param().abi(), dwattributes)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSetFeatureAttributesW<P0, P1>(hinstall: P0, szfeature: P1, dwattributes: u32) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSetFeatureAttributesW(hinstall : MSIHANDLE, szfeature : ::windows_core::PCWSTR, dwattributes : u32) -> u32);
    MsiSetFeatureAttributesW(hinstall.into_param().abi(), szfeature.into_param().abi(), dwattributes)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSetFeatureStateA<P0, P1>(hinstall: P0, szfeature: P1, istate: INSTALLSTATE) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSetFeatureStateA(hinstall : MSIHANDLE, szfeature : ::windows_core::PCSTR, istate : INSTALLSTATE) -> u32);
    MsiSetFeatureStateA(hinstall.into_param().abi(), szfeature.into_param().abi(), istate)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSetFeatureStateW<P0, P1>(hinstall: P0, szfeature: P1, istate: INSTALLSTATE) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSetFeatureStateW(hinstall : MSIHANDLE, szfeature : ::windows_core::PCWSTR, istate : INSTALLSTATE) -> u32);
    MsiSetFeatureStateW(hinstall.into_param().abi(), szfeature.into_param().abi(), istate)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSetInstallLevel<P0>(hinstall: P0, iinstalllevel: i32) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSetInstallLevel(hinstall : MSIHANDLE, iinstalllevel : i32) -> u32);
    MsiSetInstallLevel(hinstall.into_param().abi(), iinstalllevel)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSetInternalUI(dwuilevel: INSTALLUILEVEL, phwnd: ::core::option::Option<*mut super::super::Foundation::HWND>) -> INSTALLUILEVEL {
    ::windows_targets::link!("msi.dll" "system" fn MsiSetInternalUI(dwuilevel : INSTALLUILEVEL, phwnd : *mut super::super::Foundation:: HWND) -> INSTALLUILEVEL);
    MsiSetInternalUI(dwuilevel, ::core::mem::transmute(phwnd.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSetMode<P0, P1>(hinstall: P0, erunmode: MSIRUNMODE, fstate: P1) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSetMode(hinstall : MSIHANDLE, erunmode : MSIRUNMODE, fstate : super::super::Foundation:: BOOL) -> u32);
    MsiSetMode(hinstall.into_param().abi(), erunmode, fstate.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSetPropertyA<P0, P1, P2>(hinstall: P0, szname: P1, szvalue: P2) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSetPropertyA(hinstall : MSIHANDLE, szname : ::windows_core::PCSTR, szvalue : ::windows_core::PCSTR) -> u32);
    MsiSetPropertyA(hinstall.into_param().abi(), szname.into_param().abi(), szvalue.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSetPropertyW<P0, P1, P2>(hinstall: P0, szname: P1, szvalue: P2) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSetPropertyW(hinstall : MSIHANDLE, szname : ::windows_core::PCWSTR, szvalue : ::windows_core::PCWSTR) -> u32);
    MsiSetPropertyW(hinstall.into_param().abi(), szname.into_param().abi(), szvalue.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSetTargetPathA<P0, P1, P2>(hinstall: P0, szfolder: P1, szfolderpath: P2) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSetTargetPathA(hinstall : MSIHANDLE, szfolder : ::windows_core::PCSTR, szfolderpath : ::windows_core::PCSTR) -> u32);
    MsiSetTargetPathA(hinstall.into_param().abi(), szfolder.into_param().abi(), szfolderpath.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSetTargetPathW<P0, P1, P2>(hinstall: P0, szfolder: P1, szfolderpath: P2) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSetTargetPathW(hinstall : MSIHANDLE, szfolder : ::windows_core::PCWSTR, szfolderpath : ::windows_core::PCWSTR) -> u32);
    MsiSetTargetPathW(hinstall.into_param().abi(), szfolder.into_param().abi(), szfolderpath.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListAddMediaDiskA<P0, P1, P2, P3>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32, szvolumelabel: P2, szdiskprompt: P3) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSourceListAddMediaDiskA(szproductcodeorpatchcode : ::windows_core::PCSTR, szusersid : ::windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwdiskid : u32, szvolumelabel : ::windows_core::PCSTR, szdiskprompt : ::windows_core::PCSTR) -> u32);
    MsiSourceListAddMediaDiskA(szproductcodeorpatchcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, dwoptions, dwdiskid, szvolumelabel.into_param().abi(), szdiskprompt.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListAddMediaDiskW<P0, P1, P2, P3>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32, szvolumelabel: P2, szdiskprompt: P3) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSourceListAddMediaDiskW(szproductcodeorpatchcode : ::windows_core::PCWSTR, szusersid : ::windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwdiskid : u32, szvolumelabel : ::windows_core::PCWSTR, szdiskprompt : ::windows_core::PCWSTR) -> u32);
    MsiSourceListAddMediaDiskW(szproductcodeorpatchcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, dwoptions, dwdiskid, szvolumelabel.into_param().abi(), szdiskprompt.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListAddSourceA<P0, P1, P2>(szproduct: P0, szusername: P1, dwreserved: u32, szsource: P2) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSourceListAddSourceA(szproduct : ::windows_core::PCSTR, szusername : ::windows_core::PCSTR, dwreserved : u32, szsource : ::windows_core::PCSTR) -> u32);
    MsiSourceListAddSourceA(szproduct.into_param().abi(), szusername.into_param().abi(), dwreserved, szsource.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListAddSourceExA<P0, P1, P2>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: P2, dwindex: u32) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSourceListAddSourceExA(szproductcodeorpatchcode : ::windows_core::PCSTR, szusersid : ::windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szsource : ::windows_core::PCSTR, dwindex : u32) -> u32);
    MsiSourceListAddSourceExA(szproductcodeorpatchcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, dwoptions, szsource.into_param().abi(), dwindex)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListAddSourceExW<P0, P1, P2>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: P2, dwindex: u32) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSourceListAddSourceExW(szproductcodeorpatchcode : ::windows_core::PCWSTR, szusersid : ::windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szsource : ::windows_core::PCWSTR, dwindex : u32) -> u32);
    MsiSourceListAddSourceExW(szproductcodeorpatchcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, dwoptions, szsource.into_param().abi(), dwindex)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListAddSourceW<P0, P1, P2>(szproduct: P0, szusername: P1, dwreserved: u32, szsource: P2) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSourceListAddSourceW(szproduct : ::windows_core::PCWSTR, szusername : ::windows_core::PCWSTR, dwreserved : u32, szsource : ::windows_core::PCWSTR) -> u32);
    MsiSourceListAddSourceW(szproduct.into_param().abi(), szusername.into_param().abi(), dwreserved, szsource.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListClearAllA<P0, P1>(szproduct: P0, szusername: P1, dwreserved: u32) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSourceListClearAllA(szproduct : ::windows_core::PCSTR, szusername : ::windows_core::PCSTR, dwreserved : u32) -> u32);
    MsiSourceListClearAllA(szproduct.into_param().abi(), szusername.into_param().abi(), dwreserved)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListClearAllExA<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSourceListClearAllExA(szproductcodeorpatchcode : ::windows_core::PCSTR, szusersid : ::windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32) -> u32);
    MsiSourceListClearAllExA(szproductcodeorpatchcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, dwoptions)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListClearAllExW<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSourceListClearAllExW(szproductcodeorpatchcode : ::windows_core::PCWSTR, szusersid : ::windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32) -> u32);
    MsiSourceListClearAllExW(szproductcodeorpatchcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, dwoptions)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListClearAllW<P0, P1>(szproduct: P0, szusername: P1, dwreserved: u32) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSourceListClearAllW(szproduct : ::windows_core::PCWSTR, szusername : ::windows_core::PCWSTR, dwreserved : u32) -> u32);
    MsiSourceListClearAllW(szproduct.into_param().abi(), szusername.into_param().abi(), dwreserved)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListClearMediaDiskA<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSourceListClearMediaDiskA(szproductcodeorpatchcode : ::windows_core::PCSTR, szusersid : ::windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwdiskid : u32) -> u32);
    MsiSourceListClearMediaDiskA(szproductcodeorpatchcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, dwoptions, dwdiskid)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListClearMediaDiskW<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSourceListClearMediaDiskW(szproductcodeorpatchcode : ::windows_core::PCWSTR, szusersid : ::windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwdiskid : u32) -> u32);
    MsiSourceListClearMediaDiskW(szproductcodeorpatchcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, dwoptions, dwdiskid)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListClearSourceA<P0, P1, P2>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: P2) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSourceListClearSourceA(szproductcodeorpatchcode : ::windows_core::PCSTR, szusersid : ::windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szsource : ::windows_core::PCSTR) -> u32);
    MsiSourceListClearSourceA(szproductcodeorpatchcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, dwoptions, szsource.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListClearSourceW<P0, P1, P2>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: P2) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSourceListClearSourceW(szproductcodeorpatchcode : ::windows_core::PCWSTR, szusersid : ::windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szsource : ::windows_core::PCWSTR) -> u32);
    MsiSourceListClearSourceW(szproductcodeorpatchcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, dwoptions, szsource.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListEnumMediaDisksA<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, pdwdiskid: ::core::option::Option<*mut u32>, szvolumelabel: ::windows_core::PSTR, pcchvolumelabel: ::core::option::Option<*mut u32>, szdiskprompt: ::windows_core::PSTR, pcchdiskprompt: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSourceListEnumMediaDisksA(szproductcodeorpatchcode : ::windows_core::PCSTR, szusersid : ::windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwindex : u32, pdwdiskid : *mut u32, szvolumelabel : ::windows_core::PSTR, pcchvolumelabel : *mut u32, szdiskprompt : ::windows_core::PSTR, pcchdiskprompt : *mut u32) -> u32);
    MsiSourceListEnumMediaDisksA(szproductcodeorpatchcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, dwoptions, dwindex, ::core::mem::transmute(pdwdiskid.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(szvolumelabel), ::core::mem::transmute(pcchvolumelabel.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(szdiskprompt), ::core::mem::transmute(pcchdiskprompt.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListEnumMediaDisksW<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, pdwdiskid: ::core::option::Option<*mut u32>, szvolumelabel: ::windows_core::PWSTR, pcchvolumelabel: ::core::option::Option<*mut u32>, szdiskprompt: ::windows_core::PWSTR, pcchdiskprompt: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSourceListEnumMediaDisksW(szproductcodeorpatchcode : ::windows_core::PCWSTR, szusersid : ::windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwindex : u32, pdwdiskid : *mut u32, szvolumelabel : ::windows_core::PWSTR, pcchvolumelabel : *mut u32, szdiskprompt : ::windows_core::PWSTR, pcchdiskprompt : *mut u32) -> u32);
    MsiSourceListEnumMediaDisksW(szproductcodeorpatchcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, dwoptions, dwindex, ::core::mem::transmute(pdwdiskid.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(szvolumelabel), ::core::mem::transmute(pcchvolumelabel.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(szdiskprompt), ::core::mem::transmute(pcchdiskprompt.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListEnumSourcesA<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, szsource: ::windows_core::PSTR, pcchsource: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSourceListEnumSourcesA(szproductcodeorpatchcode : ::windows_core::PCSTR, szusersid : ::windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwindex : u32, szsource : ::windows_core::PSTR, pcchsource : *mut u32) -> u32);
    MsiSourceListEnumSourcesA(szproductcodeorpatchcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, dwoptions, dwindex, ::core::mem::transmute(szsource), ::core::mem::transmute(pcchsource.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListEnumSourcesW<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, szsource: ::windows_core::PWSTR, pcchsource: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSourceListEnumSourcesW(szproductcodeorpatchcode : ::windows_core::PCWSTR, szusersid : ::windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwindex : u32, szsource : ::windows_core::PWSTR, pcchsource : *mut u32) -> u32);
    MsiSourceListEnumSourcesW(szproductcodeorpatchcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, dwoptions, dwindex, ::core::mem::transmute(szsource), ::core::mem::transmute(pcchsource.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListForceResolutionA<P0, P1>(szproduct: P0, szusername: P1, dwreserved: u32) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSourceListForceResolutionA(szproduct : ::windows_core::PCSTR, szusername : ::windows_core::PCSTR, dwreserved : u32) -> u32);
    MsiSourceListForceResolutionA(szproduct.into_param().abi(), szusername.into_param().abi(), dwreserved)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListForceResolutionExA<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSourceListForceResolutionExA(szproductcodeorpatchcode : ::windows_core::PCSTR, szusersid : ::windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32) -> u32);
    MsiSourceListForceResolutionExA(szproductcodeorpatchcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, dwoptions)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListForceResolutionExW<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSourceListForceResolutionExW(szproductcodeorpatchcode : ::windows_core::PCWSTR, szusersid : ::windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32) -> u32);
    MsiSourceListForceResolutionExW(szproductcodeorpatchcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, dwoptions)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListForceResolutionW<P0, P1>(szproduct: P0, szusername: P1, dwreserved: u32) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSourceListForceResolutionW(szproduct : ::windows_core::PCWSTR, szusername : ::windows_core::PCWSTR, dwreserved : u32) -> u32);
    MsiSourceListForceResolutionW(szproduct.into_param().abi(), szusername.into_param().abi(), dwreserved)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListGetInfoA<P0, P1, P2>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: P2, szvalue: ::windows_core::PSTR, pcchvalue: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSourceListGetInfoA(szproductcodeorpatchcode : ::windows_core::PCSTR, szusersid : ::windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szproperty : ::windows_core::PCSTR, szvalue : ::windows_core::PSTR, pcchvalue : *mut u32) -> u32);
    MsiSourceListGetInfoA(szproductcodeorpatchcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, dwoptions, szproperty.into_param().abi(), ::core::mem::transmute(szvalue), ::core::mem::transmute(pcchvalue.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListGetInfoW<P0, P1, P2>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: P2, szvalue: ::windows_core::PWSTR, pcchvalue: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSourceListGetInfoW(szproductcodeorpatchcode : ::windows_core::PCWSTR, szusersid : ::windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szproperty : ::windows_core::PCWSTR, szvalue : ::windows_core::PWSTR, pcchvalue : *mut u32) -> u32);
    MsiSourceListGetInfoW(szproductcodeorpatchcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, dwoptions, szproperty.into_param().abi(), ::core::mem::transmute(szvalue), ::core::mem::transmute(pcchvalue.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListSetInfoA<P0, P1, P2, P3>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: P2, szvalue: P3) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSourceListSetInfoA(szproductcodeorpatchcode : ::windows_core::PCSTR, szusersid : ::windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szproperty : ::windows_core::PCSTR, szvalue : ::windows_core::PCSTR) -> u32);
    MsiSourceListSetInfoA(szproductcodeorpatchcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, dwoptions, szproperty.into_param().abi(), szvalue.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSourceListSetInfoW<P0, P1, P2, P3>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: P2, szvalue: P3) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSourceListSetInfoW(szproductcodeorpatchcode : ::windows_core::PCWSTR, szusersid : ::windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szproperty : ::windows_core::PCWSTR, szvalue : ::windows_core::PCWSTR) -> u32);
    MsiSourceListSetInfoW(szproductcodeorpatchcode.into_param().abi(), szusersid.into_param().abi(), dwcontext, dwoptions, szproperty.into_param().abi(), szvalue.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSummaryInfoGetPropertyA<P0>(hsummaryinfo: P0, uiproperty: u32, puidatatype: *mut u32, pivalue: *mut i32, pftvalue: ::core::option::Option<*mut super::super::Foundation::FILETIME>, szvaluebuf: ::windows_core::PSTR, pcchvaluebuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSummaryInfoGetPropertyA(hsummaryinfo : MSIHANDLE, uiproperty : u32, puidatatype : *mut u32, pivalue : *mut i32, pftvalue : *mut super::super::Foundation:: FILETIME, szvaluebuf : ::windows_core::PSTR, pcchvaluebuf : *mut u32) -> u32);
    MsiSummaryInfoGetPropertyA(hsummaryinfo.into_param().abi(), uiproperty, puidatatype, pivalue, ::core::mem::transmute(pftvalue.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(szvaluebuf), ::core::mem::transmute(pcchvaluebuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSummaryInfoGetPropertyCount<P0>(hsummaryinfo: P0, puipropertycount: *mut u32) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSummaryInfoGetPropertyCount(hsummaryinfo : MSIHANDLE, puipropertycount : *mut u32) -> u32);
    MsiSummaryInfoGetPropertyCount(hsummaryinfo.into_param().abi(), puipropertycount)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSummaryInfoGetPropertyW<P0>(hsummaryinfo: P0, uiproperty: u32, puidatatype: *mut u32, pivalue: *mut i32, pftvalue: ::core::option::Option<*mut super::super::Foundation::FILETIME>, szvaluebuf: ::windows_core::PWSTR, pcchvaluebuf: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSummaryInfoGetPropertyW(hsummaryinfo : MSIHANDLE, uiproperty : u32, puidatatype : *mut u32, pivalue : *mut i32, pftvalue : *mut super::super::Foundation:: FILETIME, szvaluebuf : ::windows_core::PWSTR, pcchvaluebuf : *mut u32) -> u32);
    MsiSummaryInfoGetPropertyW(hsummaryinfo.into_param().abi(), uiproperty, puidatatype, pivalue, ::core::mem::transmute(pftvalue.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(szvaluebuf), ::core::mem::transmute(pcchvaluebuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiSummaryInfoPersist<P0>(hsummaryinfo: P0) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSummaryInfoPersist(hsummaryinfo : MSIHANDLE) -> u32);
    MsiSummaryInfoPersist(hsummaryinfo.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSummaryInfoSetPropertyA<P0, P1>(hsummaryinfo: P0, uiproperty: u32, uidatatype: u32, ivalue: i32, pftvalue: *mut super::super::Foundation::FILETIME, szvalue: P1) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSummaryInfoSetPropertyA(hsummaryinfo : MSIHANDLE, uiproperty : u32, uidatatype : u32, ivalue : i32, pftvalue : *mut super::super::Foundation:: FILETIME, szvalue : ::windows_core::PCSTR) -> u32);
    MsiSummaryInfoSetPropertyA(hsummaryinfo.into_param().abi(), uiproperty, uidatatype, ivalue, pftvalue, szvalue.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MsiSummaryInfoSetPropertyW<P0, P1>(hsummaryinfo: P0, uiproperty: u32, uidatatype: u32, ivalue: i32, pftvalue: *mut super::super::Foundation::FILETIME, szvalue: P1) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiSummaryInfoSetPropertyW(hsummaryinfo : MSIHANDLE, uiproperty : u32, uidatatype : u32, ivalue : i32, pftvalue : *mut super::super::Foundation:: FILETIME, szvalue : ::windows_core::PCWSTR) -> u32);
    MsiSummaryInfoSetPropertyW(hsummaryinfo.into_param().abi(), uiproperty, uidatatype, ivalue, pftvalue, szvalue.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiUseFeatureA<P0, P1>(szproduct: P0, szfeature: P1) -> INSTALLSTATE
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiUseFeatureA(szproduct : ::windows_core::PCSTR, szfeature : ::windows_core::PCSTR) -> INSTALLSTATE);
    MsiUseFeatureA(szproduct.into_param().abi(), szfeature.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiUseFeatureExA<P0, P1>(szproduct: P0, szfeature: P1, dwinstallmode: u32, dwreserved: u32) -> INSTALLSTATE
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiUseFeatureExA(szproduct : ::windows_core::PCSTR, szfeature : ::windows_core::PCSTR, dwinstallmode : u32, dwreserved : u32) -> INSTALLSTATE);
    MsiUseFeatureExA(szproduct.into_param().abi(), szfeature.into_param().abi(), dwinstallmode, dwreserved)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiUseFeatureExW<P0, P1>(szproduct: P0, szfeature: P1, dwinstallmode: u32, dwreserved: u32) -> INSTALLSTATE
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiUseFeatureExW(szproduct : ::windows_core::PCWSTR, szfeature : ::windows_core::PCWSTR, dwinstallmode : u32, dwreserved : u32) -> INSTALLSTATE);
    MsiUseFeatureExW(szproduct.into_param().abi(), szfeature.into_param().abi(), dwinstallmode, dwreserved)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiUseFeatureW<P0, P1>(szproduct: P0, szfeature: P1) -> INSTALLSTATE
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiUseFeatureW(szproduct : ::windows_core::PCWSTR, szfeature : ::windows_core::PCWSTR) -> INSTALLSTATE);
    MsiUseFeatureW(szproduct.into_param().abi(), szfeature.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiVerifyDiskSpace<P0>(hinstall: P0) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiVerifyDiskSpace(hinstall : MSIHANDLE) -> u32);
    MsiVerifyDiskSpace(hinstall.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiVerifyPackageA<P0>(szpackagepath: P0) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiVerifyPackageA(szpackagepath : ::windows_core::PCSTR) -> u32);
    MsiVerifyPackageA(szpackagepath.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiVerifyPackageW<P0>(szpackagepath: P0) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiVerifyPackageW(szpackagepath : ::windows_core::PCWSTR) -> u32);
    MsiVerifyPackageW(szpackagepath.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiViewClose<P0>(hview: P0) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiViewClose(hview : MSIHANDLE) -> u32);
    MsiViewClose(hview.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiViewExecute<P0, P1>(hview: P0, hrecord: P1) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiViewExecute(hview : MSIHANDLE, hrecord : MSIHANDLE) -> u32);
    MsiViewExecute(hview.into_param().abi(), hrecord.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiViewFetch<P0>(hview: P0, phrecord: *mut MSIHANDLE) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiViewFetch(hview : MSIHANDLE, phrecord : *mut MSIHANDLE) -> u32);
    MsiViewFetch(hview.into_param().abi(), phrecord)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiViewGetColumnInfo<P0>(hview: P0, ecolumninfo: MSICOLINFO, phrecord: *mut MSIHANDLE) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiViewGetColumnInfo(hview : MSIHANDLE, ecolumninfo : MSICOLINFO, phrecord : *mut MSIHANDLE) -> u32);
    MsiViewGetColumnInfo(hview.into_param().abi(), ecolumninfo, phrecord)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiViewGetErrorA<P0>(hview: P0, szcolumnnamebuffer: ::windows_core::PSTR, pcchbuf: ::core::option::Option<*mut u32>) -> MSIDBERROR
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiViewGetErrorA(hview : MSIHANDLE, szcolumnnamebuffer : ::windows_core::PSTR, pcchbuf : *mut u32) -> MSIDBERROR);
    MsiViewGetErrorA(hview.into_param().abi(), ::core::mem::transmute(szcolumnnamebuffer), ::core::mem::transmute(pcchbuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiViewGetErrorW<P0>(hview: P0, szcolumnnamebuffer: ::windows_core::PWSTR, pcchbuf: ::core::option::Option<*mut u32>) -> MSIDBERROR
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiViewGetErrorW(hview : MSIHANDLE, szcolumnnamebuffer : ::windows_core::PWSTR, pcchbuf : *mut u32) -> MSIDBERROR);
    MsiViewGetErrorW(hview.into_param().abi(), ::core::mem::transmute(szcolumnnamebuffer), ::core::mem::transmute(pcchbuf.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[inline]
pub unsafe fn MsiViewModify<P0, P1>(hview: P0, emodifymode: MSIMODIFY, hrecord: P1) -> u32
where
    P0: ::windows_core::IntoParam<MSIHANDLE>,
    P1: ::windows_core::IntoParam<MSIHANDLE>,
{
    ::windows_targets::link!("msi.dll" "system" fn MsiViewModify(hview : MSIHANDLE, emodifymode : MSIMODIFY, hrecord : MSIHANDLE) -> u32);
    MsiViewModify(hview.into_param().abi(), emodifymode, hrecord.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NormalizeFileForPatchSignature(filebuffer: *mut ::core::ffi::c_void, filesize: u32, optionflags: u32, optiondata: ::core::option::Option<*const PATCH_OPTION_DATA>, newfilecoffbase: u32, newfilecofftime: u32, ignorerangearray: ::core::option::Option<&[PATCH_IGNORE_RANGE]>, retainrangearray: ::core::option::Option<&[PATCH_RETAIN_RANGE]>) -> i32 {
    ::windows_targets::link!("mspatcha.dll" "system" fn NormalizeFileForPatchSignature(filebuffer : *mut ::core::ffi::c_void, filesize : u32, optionflags : u32, optiondata : *const PATCH_OPTION_DATA, newfilecoffbase : u32, newfilecofftime : u32, ignorerangecount : u32, ignorerangearray : *const PATCH_IGNORE_RANGE, retainrangecount : u32, retainrangearray : *const PATCH_RETAIN_RANGE) -> i32);
    NormalizeFileForPatchSignature(filebuffer, filesize, optionflags, ::core::mem::transmute(optiondata.unwrap_or(::std::ptr::null())), newfilecoffbase, newfilecofftime, ignorerangearray.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ignorerangearray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), retainrangearray.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(retainrangearray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryActCtxSettingsW<P0, P1, P2>(dwflags: u32, hactctx: P0, settingsnamespace: P1, settingname: P2, pvbuffer: ::windows_core::PWSTR, dwbuffer: usize, pdwwrittenorrequired: ::core::option::Option<*mut usize>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("kernel32.dll" "system" fn QueryActCtxSettingsW(dwflags : u32, hactctx : super::super::Foundation:: HANDLE, settingsnamespace : ::windows_core::PCWSTR, settingname : ::windows_core::PCWSTR, pvbuffer : ::windows_core::PWSTR, dwbuffer : usize, pdwwrittenorrequired : *mut usize) -> super::super::Foundation:: BOOL);
    QueryActCtxSettingsW(dwflags, hactctx.into_param().abi(), settingsnamespace.into_param().abi(), settingname.into_param().abi(), ::core::mem::transmute(pvbuffer), dwbuffer, ::core::mem::transmute(pdwwrittenorrequired.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryActCtxW<P0>(dwflags: u32, hactctx: P0, pvsubinstance: ::core::option::Option<*const ::core::ffi::c_void>, ulinfoclass: u32, pvbuffer: ::core::option::Option<*mut ::core::ffi::c_void>, cbbuffer: usize, pcbwrittenorrequired: ::core::option::Option<*mut usize>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("kernel32.dll" "system" fn QueryActCtxW(dwflags : u32, hactctx : super::super::Foundation:: HANDLE, pvsubinstance : *const ::core::ffi::c_void, ulinfoclass : u32, pvbuffer : *mut ::core::ffi::c_void, cbbuffer : usize, pcbwrittenorrequired : *mut usize) -> super::super::Foundation:: BOOL);
    QueryActCtxW(dwflags, hactctx.into_param().abi(), ::core::mem::transmute(pvsubinstance.unwrap_or(::std::ptr::null())), ulinfoclass, ::core::mem::transmute(pvbuffer.unwrap_or(::std::ptr::null_mut())), cbbuffer, ::core::mem::transmute(pcbwrittenorrequired.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReleaseActCtx<P0>(hactctx: P0)
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("kernel32.dll" "system" fn ReleaseActCtx(hactctx : super::super::Foundation:: HANDLE) -> ());
    ReleaseActCtx(hactctx.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SfcGetNextProtectedFile<P0>(rpchandle: P0, protfiledata: *mut PROTECTED_FILE_DATA) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("sfc.dll" "system" fn SfcGetNextProtectedFile(rpchandle : super::super::Foundation:: HANDLE, protfiledata : *mut PROTECTED_FILE_DATA) -> super::super::Foundation:: BOOL);
    SfcGetNextProtectedFile(rpchandle.into_param().abi(), protfiledata).ok()
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SfcIsFileProtected<P0, P1>(rpchandle: P0, protfilename: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("sfc.dll" "system" fn SfcIsFileProtected(rpchandle : super::super::Foundation:: HANDLE, protfilename : ::windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    SfcIsFileProtected(rpchandle.into_param().abi(), protfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn SfcIsKeyProtected<P0, P1>(keyhandle: P0, subkeyname: P1, keysam: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::Registry::HKEY>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("sfc.dll" "system" fn SfcIsKeyProtected(keyhandle : super::Registry:: HKEY, subkeyname : ::windows_core::PCWSTR, keysam : u32) -> super::super::Foundation:: BOOL);
    SfcIsKeyProtected(keyhandle.into_param().abi(), subkeyname.into_param().abi(), keysam)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SfpVerifyFile<P0>(pszfilename: P0, pszerror: &[u8]) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("sfc.dll" "system" fn SfpVerifyFile(pszfilename : ::windows_core::PCSTR, pszerror : ::windows_core::PCSTR, dwerrsize : u32) -> super::super::Foundation:: BOOL);
    SfpVerifyFile(pszfilename.into_param().abi(), ::core::mem::transmute(pszerror.as_ptr()), pszerror.len() as _)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TestApplyPatchToFileA<P0, P1>(patchfilename: P0, oldfilename: P1, applyoptionflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("mspatcha.dll" "system" fn TestApplyPatchToFileA(patchfilename : ::windows_core::PCSTR, oldfilename : ::windows_core::PCSTR, applyoptionflags : u32) -> super::super::Foundation:: BOOL);
    TestApplyPatchToFileA(patchfilename.into_param().abi(), oldfilename.into_param().abi(), applyoptionflags)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TestApplyPatchToFileByBuffers(patchfilebuffer: &[u8], oldfilebuffer: ::core::option::Option<&[u8]>, newfilesize: ::core::option::Option<*mut u32>, applyoptionflags: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link!("mspatcha.dll" "system" fn TestApplyPatchToFileByBuffers(patchfilebuffer : *const u8, patchfilesize : u32, oldfilebuffer : *const u8, oldfilesize : u32, newfilesize : *mut u32, applyoptionflags : u32) -> super::super::Foundation:: BOOL);
    TestApplyPatchToFileByBuffers(::core::mem::transmute(patchfilebuffer.as_ptr()), patchfilebuffer.len() as _, ::core::mem::transmute(oldfilebuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), oldfilebuffer.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(newfilesize.unwrap_or(::std::ptr::null_mut())), applyoptionflags)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TestApplyPatchToFileByHandles<P0, P1>(patchfilehandle: P0, oldfilehandle: P1, applyoptionflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("mspatcha.dll" "system" fn TestApplyPatchToFileByHandles(patchfilehandle : super::super::Foundation:: HANDLE, oldfilehandle : super::super::Foundation:: HANDLE, applyoptionflags : u32) -> super::super::Foundation:: BOOL);
    TestApplyPatchToFileByHandles(patchfilehandle.into_param().abi(), oldfilehandle.into_param().abi(), applyoptionflags)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TestApplyPatchToFileW<P0, P1>(patchfilename: P0, oldfilename: P1, applyoptionflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("mspatcha.dll" "system" fn TestApplyPatchToFileW(patchfilename : ::windows_core::PCWSTR, oldfilename : ::windows_core::PCWSTR, applyoptionflags : u32) -> super::super::Foundation:: BOOL);
    TestApplyPatchToFileW(patchfilename.into_param().abi(), oldfilename.into_param().abi(), applyoptionflags)
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ZombifyActCtx<P0>(hactctx: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("kernel32.dll" "system" fn ZombifyActCtx(hactctx : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    ZombifyActCtx(hactctx.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IAssemblyCache(::windows_core::IUnknown);
impl IAssemblyCache {
    pub unsafe fn UninstallAssembly<P0>(&self, dwflags: u32, pszassemblyname: P0, prefdata: *mut FUSION_INSTALL_REFERENCE, puldisposition: *mut IASSEMBLYCACHE_UNINSTALL_DISPOSITION) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).UninstallAssembly)(::windows_core::Interface::as_raw(self), dwflags, pszassemblyname.into_param().abi(), prefdata, puldisposition).ok()
    }
    pub unsafe fn QueryAssemblyInfo<P0>(&self, dwflags: QUERYASMINFO_FLAGS, pszassemblyname: P0, pasminfo: *mut ASSEMBLY_INFO) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).QueryAssemblyInfo)(::windows_core::Interface::as_raw(self), dwflags, pszassemblyname.into_param().abi(), pasminfo).ok()
    }
    pub unsafe fn CreateAssemblyCacheItem<P0>(&self, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, ppasmitem: *mut ::core::option::Option<IAssemblyCacheItem>, pszassemblyname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).CreateAssemblyCacheItem)(::windows_core::Interface::as_raw(self), dwflags, pvreserved, ::core::mem::transmute(ppasmitem), pszassemblyname.into_param().abi()).ok()
    }
    pub unsafe fn Reserved(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Reserved)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InstallAssembly<P0>(&self, dwflags: u32, pszmanifestfilepath: P0, prefdata: *mut FUSION_INSTALL_REFERENCE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).InstallAssembly)(::windows_core::Interface::as_raw(self), dwflags, pszmanifestfilepath.into_param().abi(), prefdata).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IAssemblyCache, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IAssemblyCache {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAssemblyCache {}
impl ::core::fmt::Debug for IAssemblyCache {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAssemblyCache").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAssemblyCache {
    type Vtable = IAssemblyCache_Vtbl;
}
impl ::core::clone::Clone for IAssemblyCache {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAssemblyCache {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe707dcde_d1cd_11d2_bab9_00c04f8eceae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAssemblyCache_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub UninstallAssembly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pszassemblyname: ::windows_core::PCWSTR, prefdata: *mut FUSION_INSTALL_REFERENCE, puldisposition: *mut IASSEMBLYCACHE_UNINSTALL_DISPOSITION) -> ::windows_core::HRESULT,
    pub QueryAssemblyInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: QUERYASMINFO_FLAGS, pszassemblyname: ::windows_core::PCWSTR, pasminfo: *mut ASSEMBLY_INFO) -> ::windows_core::HRESULT,
    pub CreateAssemblyCacheItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, ppasmitem: *mut *mut ::core::ffi::c_void, pszassemblyname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub Reserved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InstallAssembly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pszmanifestfilepath: ::windows_core::PCWSTR, prefdata: *mut FUSION_INSTALL_REFERENCE) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IAssemblyCacheItem(::windows_core::IUnknown);
impl IAssemblyCacheItem {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateStream<P0>(&self, dwflags: u32, pszstreamname: P0, dwformat: u32, dwformatflags: u32, ppistream: *mut ::core::option::Option<super::Com::IStream>, pulimaxsize: *mut u64) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).CreateStream)(::windows_core::Interface::as_raw(self), dwflags, pszstreamname.into_param().abi(), dwformat, dwformatflags, ::core::mem::transmute(ppistream), pulimaxsize).ok()
    }
    pub unsafe fn Commit(&self, dwflags: u32, puldisposition: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Commit)(::windows_core::Interface::as_raw(self), dwflags, puldisposition).ok()
    }
    pub unsafe fn AbortItem(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AbortItem)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IAssemblyCacheItem, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IAssemblyCacheItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAssemblyCacheItem {}
impl ::core::fmt::Debug for IAssemblyCacheItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAssemblyCacheItem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAssemblyCacheItem {
    type Vtable = IAssemblyCacheItem_Vtbl;
}
impl ::core::clone::Clone for IAssemblyCacheItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAssemblyCacheItem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e3aaeb4_d1cd_11d2_bab9_00c04f8eceae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAssemblyCacheItem_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pszstreamname: ::windows_core::PCWSTR, dwformat: u32, dwformatflags: u32, ppistream: *mut *mut ::core::ffi::c_void, pulimaxsize: *mut u64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateStream: usize,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, puldisposition: *mut u32) -> ::windows_core::HRESULT,
    pub AbortItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IAssemblyName(::windows_core::IUnknown);
impl IAssemblyName {
    pub unsafe fn SetProperty(&self, propertyid: u32, pvproperty: *mut ::core::ffi::c_void, cbproperty: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProperty)(::windows_core::Interface::as_raw(self), propertyid, pvproperty, cbproperty).ok()
    }
    pub unsafe fn GetProperty(&self, propertyid: u32, pvproperty: *mut ::core::ffi::c_void, pcbproperty: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), propertyid, pvproperty, pcbproperty).ok()
    }
    pub unsafe fn Finalize(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Finalize)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetDisplayName(&self, szdisplayname: ::windows_core::PWSTR, pccdisplayname: *mut u32, dwdisplayflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDisplayName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(szdisplayname), pccdisplayname, dwdisplayflags).ok()
    }
    pub unsafe fn Reserved<P0, P1, P2>(&self, refiid: *const ::windows_core::GUID, punkreserved1: P0, punkreserved2: P1, szreserved: P2, llreserved: i64, pvreserved: *mut ::core::ffi::c_void, cbreserved: u32, ppreserved: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Reserved)(::windows_core::Interface::as_raw(self), refiid, punkreserved1.into_param().abi(), punkreserved2.into_param().abi(), szreserved.into_param().abi(), llreserved, pvreserved, cbreserved, ppreserved).ok()
    }
    pub unsafe fn GetName(&self, lpcwbuffer: *mut u32, pwzname: ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetName)(::windows_core::Interface::as_raw(self), lpcwbuffer, ::core::mem::transmute(pwzname)).ok()
    }
    pub unsafe fn GetVersion(&self, pdwversionhi: *mut u32, pdwversionlow: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetVersion)(::windows_core::Interface::as_raw(self), pdwversionhi, pdwversionlow).ok()
    }
    pub unsafe fn IsEqual<P0>(&self, pname: P0, dwcmpflags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IAssemblyName>,
    {
        (::windows_core::Interface::vtable(self).IsEqual)(::windows_core::Interface::as_raw(self), pname.into_param().abi(), dwcmpflags).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IAssemblyName> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IAssemblyName, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IAssemblyName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAssemblyName {}
impl ::core::fmt::Debug for IAssemblyName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAssemblyName").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IAssemblyName {
    type Vtable = IAssemblyName_Vtbl;
}
impl ::core::clone::Clone for IAssemblyName {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAssemblyName {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd193bc0_b4bc_11d2_9833_00c04fc31d2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAssemblyName_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: u32, pvproperty: *mut ::core::ffi::c_void, cbproperty: u32) -> ::windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: u32, pvproperty: *mut ::core::ffi::c_void, pcbproperty: *mut u32) -> ::windows_core::HRESULT,
    pub Finalize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szdisplayname: ::windows_core::PWSTR, pccdisplayname: *mut u32, dwdisplayflags: u32) -> ::windows_core::HRESULT,
    pub Reserved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, refiid: *const ::windows_core::GUID, punkreserved1: *mut ::core::ffi::c_void, punkreserved2: *mut ::core::ffi::c_void, szreserved: ::windows_core::PCWSTR, llreserved: i64, pvreserved: *mut ::core::ffi::c_void, cbreserved: u32, ppreserved: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpcwbuffer: *mut u32, pwzname: ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversionhi: *mut u32, pdwversionlow: *mut u32) -> ::windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut ::core::ffi::c_void, dwcmpflags: u32) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IEnumMsmDependency(::windows_core::IUnknown);
impl IEnumMsmDependency {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, cfetch: u32, rgmsmdependencies: *mut ::core::option::Option<IMsmDependency>, pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), cfetch, ::core::mem::transmute(rgmsmdependencies), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, cskip: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), cskip).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumMsmDependency> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IEnumMsmDependency, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IEnumMsmDependency {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumMsmDependency {}
impl ::core::fmt::Debug for IEnumMsmDependency {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumMsmDependency").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumMsmDependency {
    type Vtable = IEnumMsmDependency_Vtbl;
}
impl ::core::clone::Clone for IEnumMsmDependency {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IEnumMsmDependency {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0adda82c_2c26_11d2_ad65_00a0c9af11a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumMsmDependency_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cfetch: u32, rgmsmdependencies: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cskip: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pemsmdependencies: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IEnumMsmError(::windows_core::IUnknown);
impl IEnumMsmError {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, cfetch: u32, rgmsmerrors: *mut ::core::option::Option<IMsmError>, pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), cfetch, ::core::mem::transmute(rgmsmerrors), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, cskip: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), cskip).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumMsmError> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IEnumMsmError, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IEnumMsmError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumMsmError {}
impl ::core::fmt::Debug for IEnumMsmError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumMsmError").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumMsmError {
    type Vtable = IEnumMsmError_Vtbl;
}
impl ::core::clone::Clone for IEnumMsmError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IEnumMsmError {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0adda829_2c26_11d2_ad65_00a0c9af11a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumMsmError_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cfetch: u32, rgmsmerrors: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cskip: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pemsmerrors: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IEnumMsmString(::windows_core::IUnknown);
impl IEnumMsmString {
    pub unsafe fn Next(&self, cfetch: u32, rgbstrstrings: *mut ::windows_core::BSTR, pcfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), cfetch, ::core::mem::transmute(rgbstrstrings), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, cskip: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), cskip).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IEnumMsmString> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IEnumMsmString, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IEnumMsmString {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumMsmString {}
impl ::core::fmt::Debug for IEnumMsmString {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumMsmString").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IEnumMsmString {
    type Vtable = IEnumMsmString_Vtbl;
}
impl ::core::clone::Clone for IEnumMsmString {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IEnumMsmString {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0adda826_2c26_11d2_ad65_00a0c9af11a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumMsmString_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cfetch: u32, rgbstrstrings: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pcfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cskip: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pemsmstrings: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMsmDependencies(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMsmDependencies {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, item: i32) -> ::windows_core::Result<IMsmDependency> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), item, &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self, count: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), count).ok()
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IMsmDependencies, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMsmDependencies {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMsmDependencies {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMsmDependencies {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMsmDependencies").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IMsmDependencies {
    type Vtable = IMsmDependencies_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMsmDependencies {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IMsmDependencies {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0adda82d_2c26_11d2_ad65_00a0c9af11a6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMsmDependencies_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: i32, r#return: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMsmDependency(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMsmDependency {
    pub unsafe fn Module(&self, module: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Module)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(module)).ok()
    }
    pub unsafe fn Language(&self, language: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Language)(::windows_core::Interface::as_raw(self), language).ok()
    }
    pub unsafe fn Version(&self, version: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Version)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(version)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IMsmDependency, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMsmDependency {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMsmDependency {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMsmDependency {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMsmDependency").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IMsmDependency {
    type Vtable = IMsmDependency_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMsmDependency {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IMsmDependency {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0adda82b_2c26_11d2_ad65_00a0c9af11a6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMsmDependency_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Module: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, module: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: *mut i16) -> ::windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, version: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMsmError(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMsmError {
    pub unsafe fn Type(&self, errortype: *mut msmErrorType) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Type)(::windows_core::Interface::as_raw(self), errortype).ok()
    }
    pub unsafe fn Path(&self, errorpath: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Path)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(errorpath)).ok()
    }
    pub unsafe fn Language(&self, errorlanguage: *mut i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Language)(::windows_core::Interface::as_raw(self), errorlanguage).ok()
    }
    pub unsafe fn DatabaseTable(&self, errortable: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DatabaseTable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(errortable)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DatabaseKeys(&self) -> ::windows_core::Result<IMsmStrings> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DatabaseKeys)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ModuleTable(&self, errortable: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ModuleTable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(errortable)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ModuleKeys(&self) -> ::windows_core::Result<IMsmStrings> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ModuleKeys)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IMsmError, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMsmError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMsmError {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMsmError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMsmError").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IMsmError {
    type Vtable = IMsmError_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMsmError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IMsmError {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0adda828_2c26_11d2_ad65_00a0c9af11a6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMsmError_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errortype: *mut msmErrorType) -> ::windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorlanguage: *mut i16) -> ::windows_core::HRESULT,
    pub DatabaseTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errortable: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DatabaseKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorkeys: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DatabaseKeys: usize,
    pub ModuleTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errortable: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ModuleKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorkeys: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ModuleKeys: usize,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMsmErrors(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMsmErrors {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, item: i32) -> ::windows_core::Result<IMsmError> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), item, &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self, count: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), count).ok()
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IMsmErrors, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMsmErrors {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMsmErrors {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMsmErrors {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMsmErrors").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IMsmErrors {
    type Vtable = IMsmErrors_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMsmErrors {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IMsmErrors {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0adda82a_2c26_11d2_ad65_00a0c9af11a6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMsmErrors_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: i32, r#return: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMsmGetFiles(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMsmGetFiles {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ModuleFiles(&self) -> ::windows_core::Result<IMsmStrings> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ModuleFiles)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IMsmGetFiles, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMsmGetFiles {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMsmGetFiles {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMsmGetFiles {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMsmGetFiles").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IMsmGetFiles {
    type Vtable = IMsmGetFiles_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMsmGetFiles {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IMsmGetFiles {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7041ae26_2d78_11d2_888a_00a0c981b015);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMsmGetFiles_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ModuleFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, files: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ModuleFiles: usize,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMsmMerge(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMsmMerge {
    pub unsafe fn OpenDatabase<P0>(&self, path: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).OpenDatabase)(::windows_core::Interface::as_raw(self), path.into_param().abi()).ok()
    }
    pub unsafe fn OpenModule<P0>(&self, path: P0, language: i16) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).OpenModule)(::windows_core::Interface::as_raw(self), path.into_param().abi(), language).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CloseDatabase<P0>(&self, commit: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).CloseDatabase)(::windows_core::Interface::as_raw(self), commit.into_param().abi()).ok()
    }
    pub unsafe fn CloseModule(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CloseModule)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OpenLog<P0>(&self, path: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).OpenLog)(::windows_core::Interface::as_raw(self), path.into_param().abi()).ok()
    }
    pub unsafe fn CloseLog(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CloseLog)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Log<P0>(&self, message: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Log)(::windows_core::Interface::as_raw(self), message.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Errors(&self) -> ::windows_core::Result<IMsmErrors> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Errors)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Dependencies(&self) -> ::windows_core::Result<IMsmDependencies> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Dependencies)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Merge<P0, P1>(&self, feature: P0, redirectdir: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Merge)(::windows_core::Interface::as_raw(self), feature.into_param().abi(), redirectdir.into_param().abi()).ok()
    }
    pub unsafe fn Connect<P0>(&self, feature: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Connect)(::windows_core::Interface::as_raw(self), feature.into_param().abi()).ok()
    }
    pub unsafe fn ExtractCAB<P0>(&self, filename: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).ExtractCAB)(::windows_core::Interface::as_raw(self), filename.into_param().abi()).ok()
    }
    pub unsafe fn ExtractFiles<P0>(&self, path: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).ExtractFiles)(::windows_core::Interface::as_raw(self), path.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IMsmMerge, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMsmMerge {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMsmMerge {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMsmMerge {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMsmMerge").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IMsmMerge {
    type Vtable = IMsmMerge_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMsmMerge {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IMsmMerge {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0adda82e_2c26_11d2_ad65_00a0c9af11a6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMsmMerge_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub OpenDatabase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub OpenModule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, language: i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CloseDatabase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commit: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CloseDatabase: usize,
    pub CloseModule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OpenLog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub CloseLog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Log: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Errors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errors: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Errors: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Dependencies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dependencies: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Dependencies: usize,
    pub Merge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feature: ::std::mem::MaybeUninit<::windows_core::BSTR>, redirectdir: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feature: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ExtractCAB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ExtractFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMsmStrings(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMsmStrings {
    pub unsafe fn get_Item(&self, item: i32, r#return: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), item, ::core::mem::transmute(r#return)).ok()
    }
    pub unsafe fn Count(&self, count: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), count).ok()
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IMsmStrings, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMsmStrings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMsmStrings {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMsmStrings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMsmStrings").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IMsmStrings {
    type Vtable = IMsmStrings_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMsmStrings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IMsmStrings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0adda827_2c26_11d2_ad65_00a0c9af11a6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMsmStrings_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: i32, r#return: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMApplicationInfo(::windows_core::IUnknown);
impl IPMApplicationInfo {
    pub unsafe fn ProductID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ProductID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InstanceID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InstanceID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn OfferID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).OfferID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DefaultTask(&self, pdefaulttask: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DefaultTask)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdefaulttask)).ok()
    }
    pub unsafe fn AppTitle(&self, papptitle: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AppTitle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(papptitle)).ok()
    }
    pub unsafe fn IconPath(&self, pappiconpath: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IconPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pappiconpath)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NotificationState(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).NotificationState)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AppInstallType(&self) -> ::windows_core::Result<PM_APPLICATION_INSTALL_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AppInstallType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn State(&self) -> ::windows_core::Result<PM_APPLICATION_STATE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).State)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRevoked(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsRevoked)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateAvailable(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UpdateAvailable)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InstallDate(&self) -> ::windows_core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InstallDate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUninstallable(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsUninstallable)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsThemable(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsThemable)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsTrial(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsTrial)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InstallPath(&self, pinstallpath: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InstallPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinstallpath)).ok()
    }
    pub unsafe fn DataRoot(&self, pdataroot: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DataRoot)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdataroot)).ok()
    }
    pub unsafe fn Genre(&self) -> ::windows_core::Result<PM_APP_GENRE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Genre)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Publisher(&self, ppublisher: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Publisher)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppublisher)).ok()
    }
    pub unsafe fn Author(&self, pauthor: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Author)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pauthor)).ok()
    }
    pub unsafe fn Description(&self, pdescription: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdescription)).ok()
    }
    pub unsafe fn Version(&self, pversion: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Version)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pversion)).ok()
    }
    pub unsafe fn get_InvocationInfo(&self, pimageurn: *mut ::windows_core::BSTR, pparameters: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).get_InvocationInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pimageurn), ::core::mem::transmute(pparameters)).ok()
    }
    pub unsafe fn AppPlatMajorVersion(&self) -> ::windows_core::Result<u8> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AppPlatMajorVersion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AppPlatMinorVersion(&self) -> ::windows_core::Result<u8> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AppPlatMinorVersion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PublisherID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PublisherID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMultiCore(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsMultiCore)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SID(&self, psid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(psid)).ok()
    }
    pub unsafe fn AppPlatMajorVersionLightUp(&self) -> ::windows_core::Result<u8> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AppPlatMajorVersionLightUp)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AppPlatMinorVersionLightUp(&self) -> ::windows_core::Result<u8> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AppPlatMinorVersionLightUp)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_UpdateAvailable<P0>(&self, isupdateavailable: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).set_UpdateAvailable)(::windows_core::Interface::as_raw(self), isupdateavailable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_NotificationState<P0>(&self, isnotified: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).set_NotificationState)(::windows_core::Interface::as_raw(self), isnotified.into_param().abi()).ok()
    }
    pub unsafe fn set_IconPath<P0>(&self, appiconpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).set_IconPath)(::windows_core::Interface::as_raw(self), appiconpath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_UninstallableState<P0>(&self, isuninstallable: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).set_UninstallableState)(::windows_core::Interface::as_raw(self), isuninstallable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPinableOnKidZone(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsPinableOnKidZone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsOriginallyPreInstalled(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsOriginallyPreInstalled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsInstallOnSD(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsInstallOnSD)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsOptoutOnSD(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsOptoutOnSD)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsOptoutBackupRestore(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsOptoutBackupRestore)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_EnterpriseDisabled<P0>(&self, isdisabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).set_EnterpriseDisabled)(::windows_core::Interface::as_raw(self), isdisabled.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_EnterpriseUninstallable<P0>(&self, isuninstallable: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).set_EnterpriseUninstallable)(::windows_core::Interface::as_raw(self), isuninstallable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnterpriseDisabled(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnterpriseDisabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnterpriseUninstallable(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnterpriseUninstallable)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsVisibleOnAppList(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsVisibleOnAppList)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsInboxApp(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsInboxApp)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StorageID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StorageID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartAppBlob(&self, pblob: *mut PM_STARTAPPBLOB) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StartAppBlob)(::windows_core::Interface::as_raw(self), pblob).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMovable(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsMovable)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeploymentAppEnumerationHubFilter(&self) -> ::windows_core::Result<PM_TILE_HUBTYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DeploymentAppEnumerationHubFilter)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModifiedDate(&self) -> ::windows_core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ModifiedDate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsOriginallyRestored(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsOriginallyRestored)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShouldDeferMdilBind(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ShouldDeferMdilBind)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsFullyPreInstall(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsFullyPreInstall)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_IsMdilMaintenanceNeeded<P0>(&self, fismdilmaintenanceneeded: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).set_IsMdilMaintenanceNeeded)(::windows_core::Interface::as_raw(self), fismdilmaintenanceneeded.into_param().abi()).ok()
    }
    pub unsafe fn set_Title<P0>(&self, apptitle: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).set_Title)(::windows_core::Interface::as_raw(self), apptitle.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IPMApplicationInfo, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPMApplicationInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMApplicationInfo {}
impl ::core::fmt::Debug for IPMApplicationInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMApplicationInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPMApplicationInfo {
    type Vtable = IPMApplicationInfo_Vtbl;
}
impl ::core::clone::Clone for IPMApplicationInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPMApplicationInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50afb58a_438c_4088_9789_f8c4899829c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMApplicationInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ProductID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub InstanceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinstanceid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub OfferID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pofferid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub DefaultTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdefaulttask: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub AppTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, papptitle: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub IconPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pappiconpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub NotificationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisnotified: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NotificationState: usize,
    pub AppInstallType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pappinstalltype: *mut PM_APPLICATION_INSTALL_TYPE) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut PM_APPLICATION_STATE) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRevoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisrevoked: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRevoked: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UpdateAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisupdateavailable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UpdateAvailable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InstallDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinstalldate: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InstallDate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUninstallable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisuninstallable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUninstallable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsThemable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisthemable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsThemable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsTrial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistrial: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsTrial: usize,
    pub InstallPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinstallpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub DataRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataroot: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Genre: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgenre: *mut PM_APP_GENRE) -> ::windows_core::HRESULT,
    pub Publisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppublisher: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Author: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pauthor: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pversion: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub get_InvocationInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimageurn: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pparameters: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub AppPlatMajorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmajorver: *mut u8) -> ::windows_core::HRESULT,
    pub AppPlatMinorVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pminorver: *mut u8) -> ::windows_core::HRESULT,
    pub PublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppublisherid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsMultiCore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pismulticore: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsMultiCore: usize,
    pub SID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub AppPlatMajorVersionLightUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmajorver: *mut u8) -> ::windows_core::HRESULT,
    pub AppPlatMinorVersionLightUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pminorver: *mut u8) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub set_UpdateAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isupdateavailable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_UpdateAvailable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub set_NotificationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isnotified: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_NotificationState: usize,
    pub set_IconPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appiconpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub set_UninstallableState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isuninstallable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_UninstallableState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPinableOnKidZone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pispinable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPinableOnKidZone: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsOriginallyPreInstalled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pispreinstalled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsOriginallyPreInstalled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsInstallOnSD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisinstallonsd: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsInstallOnSD: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsOptoutOnSD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisoptoutonsd: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsOptoutOnSD: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsOptoutBackupRestore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisoptoutbackuprestore: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsOptoutBackupRestore: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub set_EnterpriseDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isdisabled: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_EnterpriseDisabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub set_EnterpriseUninstallable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isuninstallable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_EnterpriseUninstallable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnterpriseDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isdisabled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnterpriseDisabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnterpriseUninstallable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isuninstallable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnterpriseUninstallable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsVisibleOnAppList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisvisible: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsVisibleOnAppList: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsInboxApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisinboxapp: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsInboxApp: usize,
    pub StorageID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstorageid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub StartAppBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: *mut PM_STARTAPPBLOB) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StartAppBlob: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsMovable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pismovable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsMovable: usize,
    pub DeploymentAppEnumerationHubFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hubtype: *mut PM_TILE_HUBTYPE) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ModifiedDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmodifieddate: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ModifiedDate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsOriginallyRestored: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisrestored: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsOriginallyRestored: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShouldDeferMdilBind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfdefermdilbind: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShouldDeferMdilBind: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsFullyPreInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisfullypreinstall: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsFullyPreInstall: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub set_IsMdilMaintenanceNeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fismdilmaintenanceneeded: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_IsMdilMaintenanceNeeded: usize,
    pub set_Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, apptitle: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMApplicationInfoEnumerator(::windows_core::IUnknown);
impl IPMApplicationInfoEnumerator {
    pub unsafe fn Next(&self) -> ::windows_core::Result<IPMApplicationInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPMApplicationInfoEnumerator, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPMApplicationInfoEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMApplicationInfoEnumerator {}
impl ::core::fmt::Debug for IPMApplicationInfoEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMApplicationInfoEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPMApplicationInfoEnumerator {
    type Vtable = IPMApplicationInfoEnumerator_Vtbl;
}
impl ::core::clone::Clone for IPMApplicationInfoEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPMApplicationInfoEnumerator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0ec42a96_4d46_4dc6_a3d9_a7acaac0f5fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMApplicationInfoEnumerator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppappinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMBackgroundServiceAgentInfo(::windows_core::IUnknown);
impl IPMBackgroundServiceAgentInfo {
    pub unsafe fn ProductID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ProductID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TaskID(&self, ptaskid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TaskID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptaskid)).ok()
    }
    pub unsafe fn BSAID(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BSAID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn BGSpecifier(&self, pbgspecifier: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BGSpecifier)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbgspecifier)).ok()
    }
    pub unsafe fn BGName(&self, pbgname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BGName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbgname)).ok()
    }
    pub unsafe fn BGSource(&self, pbgsource: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BGSource)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbgsource)).ok()
    }
    pub unsafe fn BGType(&self, pbgtype: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BGType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbgtype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPeriodic(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsPeriodic)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsScheduled(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsScheduled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsScheduleAllowed(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsScheduleAllowed)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Description(&self, pdescription: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdescription)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsLaunchOnBoot(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsLaunchOnBoot)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_IsScheduled<P0>(&self, isscheduled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).set_IsScheduled)(::windows_core::Interface::as_raw(self), isscheduled.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_IsScheduleAllowed<P0>(&self, isscheduleallowed: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).set_IsScheduleAllowed)(::windows_core::Interface::as_raw(self), isscheduleallowed.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IPMBackgroundServiceAgentInfo, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPMBackgroundServiceAgentInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMBackgroundServiceAgentInfo {}
impl ::core::fmt::Debug for IPMBackgroundServiceAgentInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMBackgroundServiceAgentInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPMBackgroundServiceAgentInfo {
    type Vtable = IPMBackgroundServiceAgentInfo_Vtbl;
}
impl ::core::clone::Clone for IPMBackgroundServiceAgentInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPMBackgroundServiceAgentInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3a8b46da_928c_4879_998c_09dc96f3d490);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMBackgroundServiceAgentInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ProductID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub TaskID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptaskid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub BSAID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsaid: *mut u32) -> ::windows_core::HRESULT,
    pub BGSpecifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbgspecifier: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub BGName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbgname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub BGSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbgsource: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub BGType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbgtype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPeriodic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisperiodic: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPeriodic: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsScheduled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisscheduled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsScheduled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsScheduleAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisscheduleallowed: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsScheduleAllowed: usize,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsLaunchOnBoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plaunchonboot: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsLaunchOnBoot: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub set_IsScheduled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isscheduled: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_IsScheduled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub set_IsScheduleAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isscheduleallowed: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_IsScheduleAllowed: usize,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMBackgroundServiceAgentInfoEnumerator(::windows_core::IUnknown);
impl IPMBackgroundServiceAgentInfoEnumerator {
    pub unsafe fn Next(&self) -> ::windows_core::Result<IPMBackgroundServiceAgentInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPMBackgroundServiceAgentInfoEnumerator, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPMBackgroundServiceAgentInfoEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMBackgroundServiceAgentInfoEnumerator {}
impl ::core::fmt::Debug for IPMBackgroundServiceAgentInfoEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMBackgroundServiceAgentInfoEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPMBackgroundServiceAgentInfoEnumerator {
    type Vtable = IPMBackgroundServiceAgentInfoEnumerator_Vtbl;
}
impl ::core::clone::Clone for IPMBackgroundServiceAgentInfoEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPMBackgroundServiceAgentInfoEnumerator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x18eb2072_ab56_43b3_872c_beafb7a6b391);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMBackgroundServiceAgentInfoEnumerator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbsainfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMBackgroundWorkerInfo(::windows_core::IUnknown);
impl IPMBackgroundWorkerInfo {
    pub unsafe fn ProductID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ProductID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TaskID(&self, ptaskid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TaskID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptaskid)).ok()
    }
    pub unsafe fn BGName(&self, pbgname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BGName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbgname)).ok()
    }
    pub unsafe fn MaxStartupLatency(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MaxStartupLatency)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ExpectedRuntime(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ExpectedRuntime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsBootWorker(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsBootWorker)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPMBackgroundWorkerInfo, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPMBackgroundWorkerInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMBackgroundWorkerInfo {}
impl ::core::fmt::Debug for IPMBackgroundWorkerInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMBackgroundWorkerInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPMBackgroundWorkerInfo {
    type Vtable = IPMBackgroundWorkerInfo_Vtbl;
}
impl ::core::clone::Clone for IPMBackgroundWorkerInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPMBackgroundWorkerInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7dd4531b_d3bf_4b6b_94f3_69c098b1497d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMBackgroundWorkerInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ProductID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub TaskID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptaskid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub BGName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbgname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub MaxStartupLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmaxstartuplatency: *mut u32) -> ::windows_core::HRESULT,
    pub ExpectedRuntime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pexpectedruntime: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsBootWorker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisbootworker: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsBootWorker: usize,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMBackgroundWorkerInfoEnumerator(::windows_core::IUnknown);
impl IPMBackgroundWorkerInfoEnumerator {
    pub unsafe fn Next(&self) -> ::windows_core::Result<IPMBackgroundWorkerInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPMBackgroundWorkerInfoEnumerator, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPMBackgroundWorkerInfoEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMBackgroundWorkerInfoEnumerator {}
impl ::core::fmt::Debug for IPMBackgroundWorkerInfoEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMBackgroundWorkerInfoEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPMBackgroundWorkerInfoEnumerator {
    type Vtable = IPMBackgroundWorkerInfoEnumerator_Vtbl;
}
impl ::core::clone::Clone for IPMBackgroundWorkerInfoEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPMBackgroundWorkerInfoEnumerator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x87f479f8_90d8_4ec7_92b9_72787e2f636b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMBackgroundWorkerInfoEnumerator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbwinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMDeploymentManager(::windows_core::IUnknown);
impl IPMDeploymentManager {
    pub unsafe fn ReportDownloadBegin(&self, productid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReportDownloadBegin)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(productid)).ok()
    }
    pub unsafe fn ReportDownloadProgress(&self, productid: ::windows_core::GUID, usprogress: u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReportDownloadProgress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(productid), usprogress).ok()
    }
    pub unsafe fn ReportDownloadComplete(&self, productid: ::windows_core::GUID, hrresult: ::windows_core::HRESULT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReportDownloadComplete)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(productid), hrresult).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginInstall(&self, pinstallinfo: *const PM_INSTALLINFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginInstall)(::windows_core::Interface::as_raw(self), pinstallinfo).ok()
    }
    pub unsafe fn BeginUpdate(&self, pupdateinfo: *const PM_UPDATEINFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginUpdate)(::windows_core::Interface::as_raw(self), pupdateinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginDeployPackage(&self, pinstallinfo: *const PM_INSTALLINFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginDeployPackage)(::windows_core::Interface::as_raw(self), pinstallinfo).ok()
    }
    pub unsafe fn BeginUpdateDeployedPackageLegacy(&self, pupdateinfo: *const PM_UPDATEINFO_LEGACY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginUpdateDeployedPackageLegacy)(::windows_core::Interface::as_raw(self), pupdateinfo).ok()
    }
    pub unsafe fn BeginUninstall(&self, productid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginUninstall)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(productid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginEnterpriseAppInstall(&self, pinstallinfo: *const PM_INSTALLINFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginEnterpriseAppInstall)(::windows_core::Interface::as_raw(self), pinstallinfo).ok()
    }
    pub unsafe fn BeginEnterpriseAppUpdate(&self, pupdateinfo: *const PM_UPDATEINFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginEnterpriseAppUpdate)(::windows_core::Interface::as_raw(self), pupdateinfo).ok()
    }
    pub unsafe fn BeginUpdateLicense(&self, productid: ::windows_core::GUID, offerid: ::windows_core::GUID, pblicense: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginUpdateLicense)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(productid), ::core::mem::transmute(offerid), ::core::mem::transmute(pblicense.as_ptr()), pblicense.len() as _).ok()
    }
    pub unsafe fn GetLicenseChallenge<P0>(&self, packagepath: P0, ppbchallenge: *mut *mut u8, pcbchallenge: *mut u32, ppbkid: ::core::option::Option<*mut *mut u8>, pcbkid: ::core::option::Option<*mut u32>, ppbdeviceid: ::core::option::Option<*mut *mut u8>, pcbdeviceid: ::core::option::Option<*mut u32>, ppbsaltvalue: ::core::option::Option<*mut *mut u8>, pcbsaltvalue: ::core::option::Option<*mut u32>, ppbkgvvalue: ::core::option::Option<*mut *mut u8>, pcbkgvvalue: ::core::option::Option<*mut u32>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).GetLicenseChallenge)(
            ::windows_core::Interface::as_raw(self),
            packagepath.into_param().abi(),
            ppbchallenge,
            pcbchallenge,
            ::core::mem::transmute(ppbkid.unwrap_or(::std::ptr::null_mut())),
            ::core::mem::transmute(pcbkid.unwrap_or(::std::ptr::null_mut())),
            ::core::mem::transmute(ppbdeviceid.unwrap_or(::std::ptr::null_mut())),
            ::core::mem::transmute(pcbdeviceid.unwrap_or(::std::ptr::null_mut())),
            ::core::mem::transmute(ppbsaltvalue.unwrap_or(::std::ptr::null_mut())),
            ::core::mem::transmute(pcbsaltvalue.unwrap_or(::std::ptr::null_mut())),
            ::core::mem::transmute(ppbkgvvalue.unwrap_or(::std::ptr::null_mut())),
            ::core::mem::transmute(pcbkgvvalue.unwrap_or(::std::ptr::null_mut())),
        )
        .ok()
    }
    pub unsafe fn GetLicenseChallengeByProductID(&self, productid: ::windows_core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetLicenseChallengeByProductID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(productid), ppbchallenge, pcblicense).ok()
    }
    pub unsafe fn GetLicenseChallengeByProductID2(&self, productid: ::windows_core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32, ppbkid: ::core::option::Option<*mut *mut u8>, pcbkid: ::core::option::Option<*mut u32>, ppbdeviceid: ::core::option::Option<*mut *mut u8>, pcbdeviceid: ::core::option::Option<*mut u32>, ppbsaltvalue: ::core::option::Option<*mut *mut u8>, pcbsaltvalue: ::core::option::Option<*mut u32>, ppbkgvvalue: ::core::option::Option<*mut *mut u8>, pcbkgvvalue: ::core::option::Option<*mut u32>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetLicenseChallengeByProductID2)(
            ::windows_core::Interface::as_raw(self),
            ::core::mem::transmute(productid),
            ppbchallenge,
            pcblicense,
            ::core::mem::transmute(ppbkid.unwrap_or(::std::ptr::null_mut())),
            ::core::mem::transmute(pcbkid.unwrap_or(::std::ptr::null_mut())),
            ::core::mem::transmute(ppbdeviceid.unwrap_or(::std::ptr::null_mut())),
            ::core::mem::transmute(pcbdeviceid.unwrap_or(::std::ptr::null_mut())),
            ::core::mem::transmute(ppbsaltvalue.unwrap_or(::std::ptr::null_mut())),
            ::core::mem::transmute(pcbsaltvalue.unwrap_or(::std::ptr::null_mut())),
            ::core::mem::transmute(ppbkgvvalue.unwrap_or(::std::ptr::null_mut())),
            ::core::mem::transmute(pcbkgvvalue.unwrap_or(::std::ptr::null_mut())),
        )
        .ok()
    }
    pub unsafe fn RevokeLicense(&self, productid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RevokeLicense)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(productid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RebindMdilBinaries(&self, productid: ::windows_core::GUID, filenames: *const super::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RebindMdilBinaries)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(productid), filenames).ok()
    }
    pub unsafe fn RebindAllMdilBinaries(&self, productid: ::windows_core::GUID, instanceid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RebindAllMdilBinaries)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(productid), ::core::mem::transmute(instanceid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RegenerateXbf(&self, productid: ::windows_core::GUID, assemblypaths: *const super::Com::SAFEARRAY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegenerateXbf)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(productid), assemblypaths).ok()
    }
    pub unsafe fn GenerateXbfForCurrentLocale(&self, productid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GenerateXbfForCurrentLocale)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(productid)).ok()
    }
    pub unsafe fn BeginProvision<P0>(&self, productid: ::windows_core::GUID, xmlpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).BeginProvision)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(productid), xmlpath.into_param().abi()).ok()
    }
    pub unsafe fn BeginDeprovision(&self, productid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginDeprovision)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(productid)).ok()
    }
    pub unsafe fn ReindexSQLCEDatabases(&self, productid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReindexSQLCEDatabases)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(productid)).ok()
    }
    pub unsafe fn SetApplicationsNeedMaintenance(&self, requiredmaintenanceoperations: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SetApplicationsNeedMaintenance)(::windows_core::Interface::as_raw(self), requiredmaintenanceoperations, &mut result__).from_abi(result__)
    }
    pub unsafe fn UpdateChamberProfile(&self, productid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UpdateChamberProfile)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(productid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnterprisePolicyIsApplicationAllowed<P0>(&self, productid: ::windows_core::GUID, publishername: P0) -> ::windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EnterprisePolicyIsApplicationAllowed)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(productid), publishername.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn BeginUpdateDeployedPackage(&self, pupdateinfo: *const PM_UPDATEINFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginUpdateDeployedPackage)(::windows_core::Interface::as_raw(self), pupdateinfo).ok()
    }
    pub unsafe fn ReportRestoreCancelled(&self, productid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReportRestoreCancelled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(productid)).ok()
    }
    pub unsafe fn ResolveResourceString<P0>(&self, resourcestring: P0, presolvedresourcestring: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).ResolveResourceString)(::windows_core::Interface::as_raw(self), resourcestring.into_param().abi(), ::core::mem::transmute(presolvedresourcestring)).ok()
    }
    pub unsafe fn UpdateCapabilitiesForModernApps(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UpdateCapabilitiesForModernApps)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ReportDownloadStatusUpdate(&self, productid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReportDownloadStatusUpdate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(productid)).ok()
    }
    pub unsafe fn BeginUninstallWithOptions(&self, productid: ::windows_core::GUID, removaloptions: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginUninstallWithOptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(productid), removaloptions).ok()
    }
    pub unsafe fn BindDeferredMdilBinaries(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BindDeferredMdilBinaries)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GenerateXamlLightupXbfForCurrentLocale<P0>(&self, packagefamilyname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).GenerateXamlLightupXbfForCurrentLocale)(::windows_core::Interface::as_raw(self), packagefamilyname.into_param().abi()).ok()
    }
    pub unsafe fn AddLicenseForAppx(&self, productid: ::windows_core::GUID, pblicense: &[u8], pbplayreadyheader: ::core::option::Option<&[u8]>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddLicenseForAppx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(productid), ::core::mem::transmute(pblicense.as_ptr()), pblicense.len() as _, ::core::mem::transmute(pbplayreadyheader.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pbplayreadyheader.as_deref().map_or(0, |slice| slice.len() as _)).ok()
    }
    pub unsafe fn FixJunctionsForAppsOnSDCard(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FixJunctionsForAppsOnSDCard)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IPMDeploymentManager, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPMDeploymentManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMDeploymentManager {}
impl ::core::fmt::Debug for IPMDeploymentManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMDeploymentManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPMDeploymentManager {
    type Vtable = IPMDeploymentManager_Vtbl;
}
impl ::core::clone::Clone for IPMDeploymentManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPMDeploymentManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x35f785fa_1979_4a8b_bc8f_fd70eb0d1544);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMDeploymentManager_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ReportDownloadBegin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub ReportDownloadProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, usprogress: u16) -> ::windows_core::HRESULT,
    pub ReportDownloadComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, hrresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinstallinfo: *const PM_INSTALLINFO) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginInstall: usize,
    pub BeginUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginDeployPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinstallinfo: *const PM_INSTALLINFO) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginDeployPackage: usize,
    pub BeginUpdateDeployedPackageLegacy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO_LEGACY) -> ::windows_core::HRESULT,
    pub BeginUninstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginEnterpriseAppInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinstallinfo: *const PM_INSTALLINFO) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginEnterpriseAppInstall: usize,
    pub BeginEnterpriseAppUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO) -> ::windows_core::HRESULT,
    pub BeginUpdateLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, offerid: ::windows_core::GUID, pblicense: *const u8, cblicense: u32) -> ::windows_core::HRESULT,
    pub GetLicenseChallenge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppbchallenge: *mut *mut u8, pcbchallenge: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> ::windows_core::HRESULT,
    pub GetLicenseChallengeByProductID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32) -> ::windows_core::HRESULT,
    pub GetLicenseChallengeByProductID2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> ::windows_core::HRESULT,
    pub RevokeLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RebindMdilBinaries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, filenames: *const super::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RebindMdilBinaries: usize,
    pub RebindAllMdilBinaries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, instanceid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RegenerateXbf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, assemblypaths: *const super::Com::SAFEARRAY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RegenerateXbf: usize,
    pub GenerateXbfForCurrentLocale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub BeginProvision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, xmlpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub BeginDeprovision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub ReindexSQLCEDatabases: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetApplicationsNeedMaintenance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requiredmaintenanceoperations: u32, pcapplications: *mut u32) -> ::windows_core::HRESULT,
    pub UpdateChamberProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnterprisePolicyIsApplicationAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, publishername: ::windows_core::PCWSTR, pisallowed: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnterprisePolicyIsApplicationAllowed: usize,
    pub BeginUpdateDeployedPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO) -> ::windows_core::HRESULT,
    pub ReportRestoreCancelled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub ResolveResourceString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcestring: ::windows_core::PCWSTR, presolvedresourcestring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub UpdateCapabilitiesForModernApps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportDownloadStatusUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub BeginUninstallWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, removaloptions: u32) -> ::windows_core::HRESULT,
    pub BindDeferredMdilBinaries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GenerateXamlLightupXbfForCurrentLocale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub AddLicenseForAppx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, pblicense: *const u8, cblicense: u32, pbplayreadyheader: *const u8, cbplayreadyheader: u32) -> ::windows_core::HRESULT,
    pub FixJunctionsForAppsOnSDCard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMEnumerationManager(::windows_core::IUnknown);
impl IPMEnumerationManager {
    pub unsafe fn get_AllApplications(&self, ppappenum: *mut ::core::option::Option<IPMApplicationInfoEnumerator>, filter: PM_ENUM_FILTER) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).get_AllApplications)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppappenum), ::core::mem::transmute(filter)).ok()
    }
    pub unsafe fn get_AllTiles(&self, pptileenum: *mut ::core::option::Option<IPMTileInfoEnumerator>, filter: PM_ENUM_FILTER) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).get_AllTiles)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pptileenum), ::core::mem::transmute(filter)).ok()
    }
    pub unsafe fn get_AllTasks(&self, pptaskenum: *mut ::core::option::Option<IPMTaskInfoEnumerator>, filter: PM_ENUM_FILTER) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).get_AllTasks)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pptaskenum), ::core::mem::transmute(filter)).ok()
    }
    pub unsafe fn get_AllExtensions(&self, ppextensionenum: *mut ::core::option::Option<IPMExtensionInfoEnumerator>, filter: PM_ENUM_FILTER) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).get_AllExtensions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppextensionenum), ::core::mem::transmute(filter)).ok()
    }
    pub unsafe fn get_AllBackgroundServiceAgents(&self, ppbsaenum: *mut ::core::option::Option<IPMBackgroundServiceAgentInfoEnumerator>, filter: PM_ENUM_FILTER) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).get_AllBackgroundServiceAgents)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppbsaenum), ::core::mem::transmute(filter)).ok()
    }
    pub unsafe fn get_AllBackgroundWorkers(&self, ppbswenum: *mut ::core::option::Option<IPMBackgroundWorkerInfoEnumerator>, filter: PM_ENUM_FILTER) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).get_AllBackgroundWorkers)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppbswenum), ::core::mem::transmute(filter)).ok()
    }
    pub unsafe fn get_ApplicationInfo(&self, productid: ::windows_core::GUID) -> ::windows_core::Result<IPMApplicationInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_ApplicationInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(productid), &mut result__).from_abi(result__)
    }
    pub unsafe fn get_TileInfo<P0>(&self, productid: ::windows_core::GUID, tileid: P0) -> ::windows_core::Result<IPMTileInfo>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_TileInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(productid), tileid.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn get_TaskInfo<P0>(&self, productid: ::windows_core::GUID, taskid: P0) -> ::windows_core::Result<IPMTaskInfo>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_TaskInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(productid), taskid.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn get_TaskInfoEx<P0>(&self, productid: ::windows_core::GUID, taskid: P0) -> ::windows_core::Result<IPMTaskInfo>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_TaskInfoEx)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(productid), taskid.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn get_BackgroundServiceAgentInfo(&self, bsaid: u32) -> ::windows_core::Result<IPMBackgroundServiceAgentInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_BackgroundServiceAgentInfo)(::windows_core::Interface::as_raw(self), bsaid, &mut result__).from_abi(result__)
    }
    pub unsafe fn AllLiveTileJobs(&self) -> ::windows_core::Result<IPMLiveTileJobInfoEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AllLiveTileJobs)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn get_LiveTileJob<P0>(&self, productid: ::windows_core::GUID, tileid: P0, recurrencetype: PM_LIVETILE_RECURRENCE_TYPE) -> ::windows_core::Result<IPMLiveTileJobInfo>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_LiveTileJob)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(productid), tileid.into_param().abi(), recurrencetype, &mut result__).from_abi(result__)
    }
    pub unsafe fn get_ApplicationInfoExternal(&self, productid: ::windows_core::GUID) -> ::windows_core::Result<IPMApplicationInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_ApplicationInfoExternal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(productid), &mut result__).from_abi(result__)
    }
    pub unsafe fn get_FileHandlerGenericLogo<P0>(&self, filetype: P0, logosize: PM_LOGO_SIZE, plogo: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).get_FileHandlerGenericLogo)(::windows_core::Interface::as_raw(self), filetype.into_param().abi(), logosize, ::core::mem::transmute(plogo)).ok()
    }
    pub unsafe fn get_ApplicationInfoFromAccessClaims<P0, P1>(&self, sysappid0: P0, sysappid1: P1) -> ::windows_core::Result<IPMApplicationInfo>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_ApplicationInfoFromAccessClaims)(::windows_core::Interface::as_raw(self), sysappid0.into_param().abi(), sysappid1.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_StartTileEnumeratorBlob(&self, filter: PM_ENUM_FILTER, pctiles: *mut u32, pptileblobs: *mut *mut PM_STARTTILEBLOB) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).get_StartTileEnumeratorBlob)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(filter), pctiles, pptileblobs).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_StartAppEnumeratorBlob(&self, filter: PM_ENUM_FILTER, pcapps: *mut u32, ppappblobs: *mut *mut PM_STARTAPPBLOB) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).get_StartAppEnumeratorBlob)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(filter), pcapps, ppappblobs).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IPMEnumerationManager, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPMEnumerationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMEnumerationManager {}
impl ::core::fmt::Debug for IPMEnumerationManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMEnumerationManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPMEnumerationManager {
    type Vtable = IPMEnumerationManager_Vtbl;
}
impl ::core::clone::Clone for IPMEnumerationManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPMEnumerationManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x698d57c2_292d_4cf3_b73c_d95a6922ed9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMEnumerationManager_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub get_AllApplications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppappenum: *mut *mut ::core::ffi::c_void, filter: PM_ENUM_FILTER) -> ::windows_core::HRESULT,
    pub get_AllTiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptileenum: *mut *mut ::core::ffi::c_void, filter: PM_ENUM_FILTER) -> ::windows_core::HRESULT,
    pub get_AllTasks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptaskenum: *mut *mut ::core::ffi::c_void, filter: PM_ENUM_FILTER) -> ::windows_core::HRESULT,
    pub get_AllExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppextensionenum: *mut *mut ::core::ffi::c_void, filter: PM_ENUM_FILTER) -> ::windows_core::HRESULT,
    pub get_AllBackgroundServiceAgents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbsaenum: *mut *mut ::core::ffi::c_void, filter: PM_ENUM_FILTER) -> ::windows_core::HRESULT,
    pub get_AllBackgroundWorkers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbswenum: *mut *mut ::core::ffi::c_void, filter: PM_ENUM_FILTER) -> ::windows_core::HRESULT,
    pub get_ApplicationInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, ppappinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub get_TileInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, tileid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pptileinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub get_TaskInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, taskid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pptaskinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub get_TaskInfoEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, taskid: ::windows_core::PCWSTR, pptaskinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub get_BackgroundServiceAgentInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bsaid: u32, pptaskinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AllLiveTileJobs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplivetilejobenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub get_LiveTileJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, tileid: ::std::mem::MaybeUninit<::windows_core::BSTR>, recurrencetype: PM_LIVETILE_RECURRENCE_TYPE, pplivetilejobinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub get_ApplicationInfoExternal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, ppappinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub get_FileHandlerGenericLogo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filetype: ::std::mem::MaybeUninit<::windows_core::BSTR>, logosize: PM_LOGO_SIZE, plogo: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub get_ApplicationInfoFromAccessClaims: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sysappid0: ::std::mem::MaybeUninit<::windows_core::BSTR>, sysappid1: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppappinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_StartTileEnumeratorBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filter: PM_ENUM_FILTER, pctiles: *mut u32, pptileblobs: *mut *mut PM_STARTTILEBLOB) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_StartTileEnumeratorBlob: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_StartAppEnumeratorBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filter: PM_ENUM_FILTER, pcapps: *mut u32, ppappblobs: *mut *mut PM_STARTAPPBLOB) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_StartAppEnumeratorBlob: usize,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMExtensionCachedFileUpdaterInfo(::windows_core::IUnknown);
impl IPMExtensionCachedFileUpdaterInfo {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsUpdates(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SupportsUpdates)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPMExtensionCachedFileUpdaterInfo, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPMExtensionCachedFileUpdaterInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMExtensionCachedFileUpdaterInfo {}
impl ::core::fmt::Debug for IPMExtensionCachedFileUpdaterInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMExtensionCachedFileUpdaterInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPMExtensionCachedFileUpdaterInfo {
    type Vtable = IPMExtensionCachedFileUpdaterInfo_Vtbl;
}
impl ::core::clone::Clone for IPMExtensionCachedFileUpdaterInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPMExtensionCachedFileUpdaterInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe2d77509_4e58_4ba9_af7e_b642e370e1b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMExtensionCachedFileUpdaterInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SupportsUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psupportsupdates: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SupportsUpdates: usize,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMExtensionContractInfo(::windows_core::IUnknown);
impl IPMExtensionContractInfo {
    pub unsafe fn get_InvocationInfo(&self, paumid: *mut ::windows_core::BSTR, pargs: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).get_InvocationInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(paumid), ::core::mem::transmute(pargs)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IPMExtensionContractInfo, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPMExtensionContractInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMExtensionContractInfo {}
impl ::core::fmt::Debug for IPMExtensionContractInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMExtensionContractInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPMExtensionContractInfo {
    type Vtable = IPMExtensionContractInfo_Vtbl;
}
impl ::core::clone::Clone for IPMExtensionContractInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPMExtensionContractInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe5666373_7ba1_467c_b819_b175db1c295b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMExtensionContractInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub get_InvocationInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paumid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pargs: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMExtensionFileExtensionInfo(::windows_core::IUnknown);
impl IPMExtensionFileExtensionInfo {
    pub unsafe fn Name(&self, pname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pname)).ok()
    }
    pub unsafe fn DisplayName(&self, pdisplayname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisplayName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdisplayname)).ok()
    }
    pub unsafe fn get_Logo(&self, logosize: PM_LOGO_SIZE, plogo: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).get_Logo)(::windows_core::Interface::as_raw(self), logosize, ::core::mem::transmute(plogo)).ok()
    }
    pub unsafe fn get_ContentType<P0>(&self, filetype: P0, pcontenttype: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).get_ContentType)(::windows_core::Interface::as_raw(self), filetype.into_param().abi(), ::core::mem::transmute(pcontenttype)).ok()
    }
    pub unsafe fn get_FileType<P0>(&self, contenttype: P0, pfiletype: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).get_FileType)(::windows_core::Interface::as_raw(self), contenttype.into_param().abi(), ::core::mem::transmute(pfiletype)).ok()
    }
    pub unsafe fn get_InvocationInfo(&self, pimageurn: *mut ::windows_core::BSTR, pparameters: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).get_InvocationInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pimageurn), ::core::mem::transmute(pparameters)).ok()
    }
    pub unsafe fn get_AllFileTypes(&self, pcbtypes: *mut u32, pptypes: *mut *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).get_AllFileTypes)(::windows_core::Interface::as_raw(self), pcbtypes, pptypes).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IPMExtensionFileExtensionInfo, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPMExtensionFileExtensionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMExtensionFileExtensionInfo {}
impl ::core::fmt::Debug for IPMExtensionFileExtensionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMExtensionFileExtensionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPMExtensionFileExtensionInfo {
    type Vtable = IPMExtensionFileExtensionInfo_Vtbl;
}
impl ::core::clone::Clone for IPMExtensionFileExtensionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPMExtensionFileExtensionInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6b87cb6c_0b88_4989_a4ec_033714f710d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMExtensionFileExtensionInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdisplayname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub get_Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logosize: PM_LOGO_SIZE, plogo: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub get_ContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filetype: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcontenttype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub get_FileType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenttype: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfiletype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub get_InvocationInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimageurn: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pparameters: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub get_AllFileTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbtypes: *mut u32, pptypes: *mut *mut ::windows_core::BSTR) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMExtensionFileOpenPickerInfo(::windows_core::IUnknown);
impl IPMExtensionFileOpenPickerInfo {
    pub unsafe fn get_AllFileTypes(&self, pctypes: *mut u32, pptypes: *mut *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).get_AllFileTypes)(::windows_core::Interface::as_raw(self), pctypes, pptypes).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsAllFileTypes(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SupportsAllFileTypes)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPMExtensionFileOpenPickerInfo, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPMExtensionFileOpenPickerInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMExtensionFileOpenPickerInfo {}
impl ::core::fmt::Debug for IPMExtensionFileOpenPickerInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMExtensionFileOpenPickerInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPMExtensionFileOpenPickerInfo {
    type Vtable = IPMExtensionFileOpenPickerInfo_Vtbl;
}
impl ::core::clone::Clone for IPMExtensionFileOpenPickerInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPMExtensionFileOpenPickerInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6dc91d25_9606_420c_9a78_e034a3418345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMExtensionFileOpenPickerInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub get_AllFileTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctypes: *mut u32, pptypes: *mut *mut ::windows_core::BSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SupportsAllFileTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psupportsalltypes: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SupportsAllFileTypes: usize,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMExtensionFileSavePickerInfo(::windows_core::IUnknown);
impl IPMExtensionFileSavePickerInfo {
    pub unsafe fn get_AllFileTypes(&self, pctypes: *mut u32, pptypes: *mut *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).get_AllFileTypes)(::windows_core::Interface::as_raw(self), pctypes, pptypes).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsAllFileTypes(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SupportsAllFileTypes)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPMExtensionFileSavePickerInfo, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPMExtensionFileSavePickerInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMExtensionFileSavePickerInfo {}
impl ::core::fmt::Debug for IPMExtensionFileSavePickerInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMExtensionFileSavePickerInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPMExtensionFileSavePickerInfo {
    type Vtable = IPMExtensionFileSavePickerInfo_Vtbl;
}
impl ::core::clone::Clone for IPMExtensionFileSavePickerInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPMExtensionFileSavePickerInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38005cba_f81a_493e_a0f8_922c8680da43);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMExtensionFileSavePickerInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub get_AllFileTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctypes: *mut u32, pptypes: *mut *mut ::windows_core::BSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SupportsAllFileTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psupportsalltypes: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SupportsAllFileTypes: usize,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMExtensionInfo(::windows_core::IUnknown);
impl IPMExtensionInfo {
    pub unsafe fn SupplierPID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SupplierPID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SupplierTaskID(&self, psuppliertid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SupplierTaskID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(psuppliertid)).ok()
    }
    pub unsafe fn Title(&self, ptitle: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Title)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptitle)).ok()
    }
    pub unsafe fn IconPath(&self, piconpath: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IconPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(piconpath)).ok()
    }
    pub unsafe fn ExtraFile(&self, pfilepath: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ExtraFile)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pfilepath)).ok()
    }
    pub unsafe fn get_InvocationInfo(&self, pimageurn: *mut ::windows_core::BSTR, pparameters: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).get_InvocationInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pimageurn), ::core::mem::transmute(pparameters)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IPMExtensionInfo, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPMExtensionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMExtensionInfo {}
impl ::core::fmt::Debug for IPMExtensionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMExtensionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPMExtensionInfo {
    type Vtable = IPMExtensionInfo_Vtbl;
}
impl ::core::clone::Clone for IPMExtensionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPMExtensionInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x49acde79_9788_4d0a_8aa0_1746afdb9e9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMExtensionInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SupplierPID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psupplierpid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SupplierTaskID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psuppliertid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptitle: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub IconPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piconpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ExtraFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilepath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub get_InvocationInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimageurn: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pparameters: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMExtensionInfoEnumerator(::windows_core::IUnknown);
impl IPMExtensionInfoEnumerator {
    pub unsafe fn Next(&self) -> ::windows_core::Result<IPMExtensionInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPMExtensionInfoEnumerator, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPMExtensionInfoEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMExtensionInfoEnumerator {}
impl ::core::fmt::Debug for IPMExtensionInfoEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMExtensionInfoEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPMExtensionInfoEnumerator {
    type Vtable = IPMExtensionInfoEnumerator_Vtbl;
}
impl ::core::clone::Clone for IPMExtensionInfoEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPMExtensionInfoEnumerator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x403b9e82_1171_4573_8e6f_6f33f39b83dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMExtensionInfoEnumerator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppextensioninfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMExtensionProtocolInfo(::windows_core::IUnknown);
impl IPMExtensionProtocolInfo {
    pub unsafe fn Protocol(&self, pprotocol: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Protocol)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pprotocol)).ok()
    }
    pub unsafe fn get_InvocationInfo(&self, pimageurn: *mut ::windows_core::BSTR, pparameters: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).get_InvocationInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pimageurn), ::core::mem::transmute(pparameters)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IPMExtensionProtocolInfo, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPMExtensionProtocolInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMExtensionProtocolInfo {}
impl ::core::fmt::Debug for IPMExtensionProtocolInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMExtensionProtocolInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPMExtensionProtocolInfo {
    type Vtable = IPMExtensionProtocolInfo_Vtbl;
}
impl ::core::clone::Clone for IPMExtensionProtocolInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPMExtensionProtocolInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1e3fa036_51eb_4453_baff_b8d8e4b46c8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMExtensionProtocolInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Protocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprotocol: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub get_InvocationInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimageurn: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pparameters: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMExtensionShareTargetInfo(::windows_core::IUnknown);
impl IPMExtensionShareTargetInfo {
    pub unsafe fn get_AllFileTypes(&self, pctypes: *mut u32, pptypes: *mut *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).get_AllFileTypes)(::windows_core::Interface::as_raw(self), pctypes, pptypes).ok()
    }
    pub unsafe fn get_AllDataFormats(&self, pcdataformats: *mut u32, ppdataformats: *mut *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).get_AllDataFormats)(::windows_core::Interface::as_raw(self), pcdataformats, ppdataformats).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsAllFileTypes(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SupportsAllFileTypes)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPMExtensionShareTargetInfo, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPMExtensionShareTargetInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMExtensionShareTargetInfo {}
impl ::core::fmt::Debug for IPMExtensionShareTargetInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMExtensionShareTargetInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPMExtensionShareTargetInfo {
    type Vtable = IPMExtensionShareTargetInfo_Vtbl;
}
impl ::core::clone::Clone for IPMExtensionShareTargetInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPMExtensionShareTargetInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5471f48b_c65c_4656_8c70_242e31195fea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMExtensionShareTargetInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub get_AllFileTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctypes: *mut u32, pptypes: *mut *mut ::windows_core::BSTR) -> ::windows_core::HRESULT,
    pub get_AllDataFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdataformats: *mut u32, ppdataformats: *mut *mut ::windows_core::BSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SupportsAllFileTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psupportsalltypes: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SupportsAllFileTypes: usize,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMLiveTileJobInfo(::windows_core::IUnknown);
impl IPMLiveTileJobInfo {
    pub unsafe fn ProductID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ProductID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TileID(&self, ptileid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TileID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptileid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NextSchedule(&self) -> ::windows_core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).NextSchedule)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_NextSchedule(&self, ftnextschedule: super::super::Foundation::FILETIME) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).set_NextSchedule)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ftnextschedule)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartSchedule(&self) -> ::windows_core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StartSchedule)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_StartSchedule(&self, ftstartschedule: super::super::Foundation::FILETIME) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).set_StartSchedule)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ftstartschedule)).ok()
    }
    pub unsafe fn IntervalDuration(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IntervalDuration)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn set_IntervalDuration(&self, ulintervalduration: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).set_IntervalDuration)(::windows_core::Interface::as_raw(self), ulintervalduration).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunForever(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RunForever)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_RunForever<P0>(&self, frunforever: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).set_RunForever)(::windows_core::Interface::as_raw(self), frunforever.into_param().abi()).ok()
    }
    pub unsafe fn MaxRunCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MaxRunCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn set_MaxRunCount(&self, ulmaxruncount: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).set_MaxRunCount)(::windows_core::Interface::as_raw(self), ulmaxruncount).ok()
    }
    pub unsafe fn RunCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RunCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn set_RunCount(&self, ulruncount: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).set_RunCount)(::windows_core::Interface::as_raw(self), ulruncount).ok()
    }
    pub unsafe fn RecurrenceType(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RecurrenceType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn set_RecurrenceType(&self, ulrecurrencetype: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).set_RecurrenceType)(::windows_core::Interface::as_raw(self), ulrecurrencetype).ok()
    }
    pub unsafe fn get_TileXML(&self, ptilexml: *mut *mut u8, pcbtilexml: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).get_TileXML)(::windows_core::Interface::as_raw(self), ptilexml, pcbtilexml).ok()
    }
    pub unsafe fn set_TileXML(&self, ptilexml: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).set_TileXML)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptilexml.as_ptr()), ptilexml.len() as _).ok()
    }
    pub unsafe fn get_UrlXML(&self, purlxml: *mut *mut u8, pcburlxml: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).get_UrlXML)(::windows_core::Interface::as_raw(self), purlxml, pcburlxml).ok()
    }
    pub unsafe fn set_UrlXML(&self, purlxml: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).set_UrlXML)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(purlxml.as_ptr()), purlxml.len() as _).ok()
    }
    pub unsafe fn AttemptCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AttemptCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn set_AttemptCount(&self, ulattemptcount: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).set_AttemptCount)(::windows_core::Interface::as_raw(self), ulattemptcount).ok()
    }
    pub unsafe fn DownloadState(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DownloadState)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn set_DownloadState(&self, uldownloadstate: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).set_DownloadState)(::windows_core::Interface::as_raw(self), uldownloadstate).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IPMLiveTileJobInfo, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPMLiveTileJobInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMLiveTileJobInfo {}
impl ::core::fmt::Debug for IPMLiveTileJobInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMLiveTileJobInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPMLiveTileJobInfo {
    type Vtable = IPMLiveTileJobInfo_Vtbl;
}
impl ::core::clone::Clone for IPMLiveTileJobInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPMLiveTileJobInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6009a81f_4710_4697_b5f6_2208f6057b8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMLiveTileJobInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ProductID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub TileID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptileid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub NextSchedule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnextschedule: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NextSchedule: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub set_NextSchedule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ftnextschedule: super::super::Foundation::FILETIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_NextSchedule: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub StartSchedule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstartschedule: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StartSchedule: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub set_StartSchedule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ftstartschedule: super::super::Foundation::FILETIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_StartSchedule: usize,
    pub IntervalDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pintervalduration: *mut u32) -> ::windows_core::HRESULT,
    pub set_IntervalDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulintervalduration: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RunForever: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isrunforever: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RunForever: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub set_RunForever: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frunforever: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_RunForever: usize,
    pub MaxRunCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmaxruncount: *mut u32) -> ::windows_core::HRESULT,
    pub set_MaxRunCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulmaxruncount: u32) -> ::windows_core::HRESULT,
    pub RunCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pruncount: *mut u32) -> ::windows_core::HRESULT,
    pub set_RunCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulruncount: u32) -> ::windows_core::HRESULT,
    pub RecurrenceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, precurrencetype: *mut u32) -> ::windows_core::HRESULT,
    pub set_RecurrenceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulrecurrencetype: u32) -> ::windows_core::HRESULT,
    pub get_TileXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptilexml: *mut *mut u8, pcbtilexml: *mut u32) -> ::windows_core::HRESULT,
    pub set_TileXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptilexml: *const u8, cbtilexml: u32) -> ::windows_core::HRESULT,
    pub get_UrlXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, purlxml: *mut *mut u8, pcburlxml: *mut u32) -> ::windows_core::HRESULT,
    pub set_UrlXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, purlxml: *const u8, cburlxml: u32) -> ::windows_core::HRESULT,
    pub AttemptCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattemptcount: *mut u32) -> ::windows_core::HRESULT,
    pub set_AttemptCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulattemptcount: u32) -> ::windows_core::HRESULT,
    pub DownloadState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdownloadstate: *mut u32) -> ::windows_core::HRESULT,
    pub set_DownloadState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uldownloadstate: u32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMLiveTileJobInfoEnumerator(::windows_core::IUnknown);
impl IPMLiveTileJobInfoEnumerator {
    pub unsafe fn Next(&self) -> ::windows_core::Result<IPMLiveTileJobInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPMLiveTileJobInfoEnumerator, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPMLiveTileJobInfoEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMLiveTileJobInfoEnumerator {}
impl ::core::fmt::Debug for IPMLiveTileJobInfoEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMLiveTileJobInfoEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPMLiveTileJobInfoEnumerator {
    type Vtable = IPMLiveTileJobInfoEnumerator_Vtbl;
}
impl ::core::clone::Clone for IPMLiveTileJobInfoEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPMLiveTileJobInfoEnumerator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc042582_9415_4f36_9f99_06f104c07c03);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMLiveTileJobInfoEnumerator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplivetilejobinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMTaskInfo(::windows_core::IUnknown);
impl IPMTaskInfo {
    pub unsafe fn ProductID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ProductID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TaskID(&self, ptaskid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TaskID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptaskid)).ok()
    }
    pub unsafe fn NavigationPage(&self, pnavigationpage: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NavigationPage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pnavigationpage)).ok()
    }
    pub unsafe fn TaskTransition(&self) -> ::windows_core::Result<PM_TASK_TRANSITION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TaskTransition)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RuntimeType(&self) -> ::windows_core::Result<PACKMAN_RUNTIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RuntimeType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ActivationPolicy(&self) -> ::windows_core::Result<PM_ACTIVATION_POLICY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ActivationPolicy)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TaskType(&self) -> ::windows_core::Result<PM_TASK_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TaskType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn get_InvocationInfo(&self, pimageurn: *mut ::windows_core::BSTR, pparameters: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).get_InvocationInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pimageurn), ::core::mem::transmute(pparameters)).ok()
    }
    pub unsafe fn ImagePath(&self, pimagepath: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ImagePath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pimagepath)).ok()
    }
    pub unsafe fn ImageParams(&self, pimageparams: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ImageParams)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pimageparams)).ok()
    }
    pub unsafe fn InstallRootFolder(&self, pinstallrootfolder: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InstallRootFolder)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pinstallrootfolder)).ok()
    }
    pub unsafe fn DataRootFolder(&self, pdatarootfolder: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DataRootFolder)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdatarootfolder)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSingleInstanceHost(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsSingleInstanceHost)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsInteropEnabled(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsInteropEnabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ApplicationState(&self) -> ::windows_core::Result<PM_APPLICATION_STATE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ApplicationState)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InstallType(&self) -> ::windows_core::Result<PM_APPLICATION_INSTALL_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InstallType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn get_Version(&self, ptargetmajorversion: *mut u8, ptargetminorversion: *mut u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).get_Version)(::windows_core::Interface::as_raw(self), ptargetmajorversion, ptargetminorversion).ok()
    }
    pub unsafe fn BitsPerPixel(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BitsPerPixel)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SuppressesDehydration(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SuppressesDehydration)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn BackgroundExecutionAbilities(&self, pbackgroundexecutionabilities: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BackgroundExecutionAbilities)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbackgroundexecutionabilities)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsOptedForExtendedMem(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsOptedForExtendedMem)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPMTaskInfo, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPMTaskInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMTaskInfo {}
impl ::core::fmt::Debug for IPMTaskInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMTaskInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPMTaskInfo {
    type Vtable = IPMTaskInfo_Vtbl;
}
impl ::core::clone::Clone for IPMTaskInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPMTaskInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbf1d8c33_1bf5_4ee0_b549_6b9dd3834942);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMTaskInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ProductID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub TaskID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptaskid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub NavigationPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnavigationpage: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub TaskTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptasktransition: *mut PM_TASK_TRANSITION) -> ::windows_core::HRESULT,
    pub RuntimeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pruntimetype: *mut PACKMAN_RUNTIME) -> ::windows_core::HRESULT,
    pub ActivationPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pactivationpolicy: *mut PM_ACTIVATION_POLICY) -> ::windows_core::HRESULT,
    pub TaskType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptasktype: *mut PM_TASK_TYPE) -> ::windows_core::HRESULT,
    pub get_InvocationInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimageurn: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pparameters: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ImagePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimagepath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ImageParams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimageparams: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub InstallRootFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinstallrootfolder: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub DataRootFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatarootfolder: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSingleInstanceHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pissingleinstancehost: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSingleInstanceHost: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsInteropEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisinteropenabled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsInteropEnabled: usize,
    pub ApplicationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, papplicationstate: *mut PM_APPLICATION_STATE) -> ::windows_core::HRESULT,
    pub InstallType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinstalltype: *mut PM_APPLICATION_INSTALL_TYPE) -> ::windows_core::HRESULT,
    pub get_Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptargetmajorversion: *mut u8, ptargetminorversion: *mut u8) -> ::windows_core::HRESULT,
    pub BitsPerPixel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitsperpixel: *mut u16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SuppressesDehydration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psuppressesdehydration: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SuppressesDehydration: usize,
    pub BackgroundExecutionAbilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbackgroundexecutionabilities: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsOptedForExtendedMem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisoptedin: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsOptedForExtendedMem: usize,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMTaskInfoEnumerator(::windows_core::IUnknown);
impl IPMTaskInfoEnumerator {
    pub unsafe fn Next(&self) -> ::windows_core::Result<IPMTaskInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPMTaskInfoEnumerator, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPMTaskInfoEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMTaskInfoEnumerator {}
impl ::core::fmt::Debug for IPMTaskInfoEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMTaskInfoEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPMTaskInfoEnumerator {
    type Vtable = IPMTaskInfoEnumerator_Vtbl;
}
impl ::core::clone::Clone for IPMTaskInfoEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPMTaskInfoEnumerator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0630b0f8_0bbc_4821_be74_c7995166ed2a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMTaskInfoEnumerator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptaskinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMTileInfo(::windows_core::IUnknown);
impl IPMTileInfo {
    pub unsafe fn ProductID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ProductID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TileID(&self, ptileid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TileID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptileid)).ok()
    }
    pub unsafe fn TemplateType(&self) -> ::windows_core::Result<TILE_TEMPLATE_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TemplateType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_HubPinnedState(&self, hubtype: PM_TILE_HUBTYPE) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_HubPinnedState)(::windows_core::Interface::as_raw(self), hubtype, &mut result__).from_abi(result__)
    }
    pub unsafe fn get_HubPosition(&self, hubtype: PM_TILE_HUBTYPE) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_HubPosition)(::windows_core::Interface::as_raw(self), hubtype, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsNotified(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsNotified)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDefault(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsDefault)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TaskID(&self, ptaskid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TaskID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ptaskid)).ok()
    }
    pub unsafe fn TileType(&self) -> ::windows_core::Result<PM_STARTTILE_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TileType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsThemable(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsThemable)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn get_PropertyById(&self, propid: u32) -> ::windows_core::Result<IPMTilePropertyInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_PropertyById)(::windows_core::Interface::as_raw(self), propid, &mut result__).from_abi(result__)
    }
    pub unsafe fn get_InvocationInfo(&self, pimageurn: *mut ::windows_core::BSTR, pparameters: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).get_InvocationInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pimageurn), ::core::mem::transmute(pparameters)).ok()
    }
    pub unsafe fn PropertyEnum(&self) -> ::windows_core::Result<IPMTilePropertyEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PropertyEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn get_HubTileSize(&self, hubtype: PM_TILE_HUBTYPE) -> ::windows_core::Result<PM_TILE_SIZE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_HubTileSize)(::windows_core::Interface::as_raw(self), hubtype, &mut result__).from_abi(result__)
    }
    pub unsafe fn set_HubPosition(&self, hubtype: PM_TILE_HUBTYPE, position: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).set_HubPosition)(::windows_core::Interface::as_raw(self), hubtype, position).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_NotifiedState<P0>(&self, notified: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).set_NotifiedState)(::windows_core::Interface::as_raw(self), notified.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_HubPinnedState<P0>(&self, hubtype: PM_TILE_HUBTYPE, pinned: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).set_HubPinnedState)(::windows_core::Interface::as_raw(self), hubtype, pinned.into_param().abi()).ok()
    }
    pub unsafe fn set_HubTileSize(&self, hubtype: PM_TILE_HUBTYPE, size: PM_TILE_SIZE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).set_HubTileSize)(::windows_core::Interface::as_raw(self), hubtype, size).ok()
    }
    pub unsafe fn set_InvocationInfo<P0, P1>(&self, taskname: P0, taskparameters: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).set_InvocationInfo)(::windows_core::Interface::as_raw(self), taskname.into_param().abi(), taskparameters.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartTileBlob(&self, pblob: *mut PM_STARTTILEBLOB) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StartTileBlob)(::windows_core::Interface::as_raw(self), pblob).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRestoring(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsRestoring)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsAutoRestoreDisabled(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsAutoRestoreDisabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_IsRestoring<P0>(&self, restoring: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).set_IsRestoring)(::windows_core::Interface::as_raw(self), restoring.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn set_IsAutoRestoreDisabled<P0>(&self, autorestoredisabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).set_IsAutoRestoreDisabled)(::windows_core::Interface::as_raw(self), autorestoredisabled.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IPMTileInfo, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPMTileInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMTileInfo {}
impl ::core::fmt::Debug for IPMTileInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMTileInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPMTileInfo {
    type Vtable = IPMTileInfo_Vtbl;
}
impl ::core::clone::Clone for IPMTileInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPMTileInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd1604833_2b08_4001_82cd_183ad734f752);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMTileInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ProductID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub TileID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptileid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub TemplateType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptemplatetype: *mut TILE_TEMPLATE_TYPE) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_HubPinnedState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, ppinned: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_HubPinnedState: usize,
    pub get_HubPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, pposition: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsNotified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisnotified: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsNotified: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisdefault: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsDefault: usize,
    pub TaskID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptaskid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub TileType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstarttiletype: *mut PM_STARTTILE_TYPE) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsThemable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisthemable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsThemable: usize,
    pub get_PropertyById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propid: u32, pppropinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub get_InvocationInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimageurn: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pparameters: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub PropertyEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptilepropenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub get_HubTileSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, psize: *mut PM_TILE_SIZE) -> ::windows_core::HRESULT,
    pub set_HubPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, position: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub set_NotifiedState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notified: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_NotifiedState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub set_HubPinnedState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, pinned: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_HubPinnedState: usize,
    pub set_HubTileSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, size: PM_TILE_SIZE) -> ::windows_core::HRESULT,
    pub set_InvocationInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: ::std::mem::MaybeUninit<::windows_core::BSTR>, taskparameters: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub StartTileBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblob: *mut PM_STARTTILEBLOB) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StartTileBlob: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRestoring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisrestoring: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRestoring: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsAutoRestoreDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisautorestoredisabled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsAutoRestoreDisabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub set_IsRestoring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restoring: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_IsRestoring: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub set_IsAutoRestoreDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, autorestoredisabled: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    set_IsAutoRestoreDisabled: usize,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMTileInfoEnumerator(::windows_core::IUnknown);
impl IPMTileInfoEnumerator {
    pub unsafe fn Next(&self) -> ::windows_core::Result<IPMTileInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPMTileInfoEnumerator, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPMTileInfoEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMTileInfoEnumerator {}
impl ::core::fmt::Debug for IPMTileInfoEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMTileInfoEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPMTileInfoEnumerator {
    type Vtable = IPMTileInfoEnumerator_Vtbl;
}
impl ::core::clone::Clone for IPMTileInfoEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPMTileInfoEnumerator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xded83065_e462_4b2c_acb5_e39cea61c874);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMTileInfoEnumerator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptileinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMTilePropertyEnumerator(::windows_core::IUnknown);
impl IPMTilePropertyEnumerator {
    pub unsafe fn Next(&self) -> ::windows_core::Result<IPMTilePropertyInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPMTilePropertyEnumerator, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPMTilePropertyEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMTilePropertyEnumerator {}
impl ::core::fmt::Debug for IPMTilePropertyEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMTilePropertyEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPMTilePropertyEnumerator {
    type Vtable = IPMTilePropertyEnumerator_Vtbl;
}
impl ::core::clone::Clone for IPMTilePropertyEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPMTilePropertyEnumerator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcc4cd629_9047_4250_aac8_930e47812421);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMTilePropertyEnumerator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IPMTilePropertyInfo(::windows_core::IUnknown);
impl IPMTilePropertyInfo {
    pub unsafe fn PropertyID(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PropertyID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PropertyValue(&self, ppropvalue: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PropertyValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppropvalue)).ok()
    }
    pub unsafe fn set_Property<P0>(&self, propvalue: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).set_Property)(::windows_core::Interface::as_raw(self), propvalue.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IPMTilePropertyInfo, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IPMTilePropertyInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPMTilePropertyInfo {}
impl ::core::fmt::Debug for IPMTilePropertyInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPMTilePropertyInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IPMTilePropertyInfo {
    type Vtable = IPMTilePropertyInfo_Vtbl;
}
impl ::core::clone::Clone for IPMTilePropertyInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPMTilePropertyInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6c2b8017_1efa_42a7_86c0_6d4b640bf528);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPMTilePropertyInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub PropertyID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropid: *mut u32) -> ::windows_core::HRESULT,
    pub PropertyValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub set_Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propvalue: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
pub struct IValidate(::windows_core::IUnknown);
impl IValidate {
    pub unsafe fn OpenDatabase<P0>(&self, szdatabase: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).OpenDatabase)(::windows_core::Interface::as_raw(self), szdatabase.into_param().abi()).ok()
    }
    pub unsafe fn OpenCUB<P0>(&self, szcubfile: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).OpenCUB)(::windows_core::Interface::as_raw(self), szcubfile.into_param().abi()).ok()
    }
    pub unsafe fn CloseDatabase(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CloseDatabase)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CloseCUB(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CloseCUB)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisplay(&self, pdisplayfunction: LPDISPLAYVAL, pcontext: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDisplay)(::windows_core::Interface::as_raw(self), pdisplayfunction, pcontext).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStatus(&self, pstatusfunction: LPEVALCOMCALLBACK, pcontext: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStatus)(::windows_core::Interface::as_raw(self), pstatusfunction, pcontext).ok()
    }
    pub unsafe fn Validate<P0>(&self, wzices: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Validate)(::windows_core::Interface::as_raw(self), wzices.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IValidate, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IValidate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IValidate {}
impl ::core::fmt::Debug for IValidate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IValidate").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IValidate {
    type Vtable = IValidate_Vtbl;
}
impl ::core::clone::Clone for IValidate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IValidate {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe482e5c6_e31e_4143_a2e6_dbc3d8e4b8d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IValidate_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OpenDatabase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szdatabase: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub OpenCUB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szcubfile: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub CloseDatabase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CloseCUB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdisplayfunction: LPDISPLAYVAL, pcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDisplay: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatusfunction: LPEVALCOMCALLBACK, pcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetStatus: usize,
    pub Validate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wzices: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_MAXVERSIONTESTED: ACTCTX_COMPATIBILITY_ELEMENT_TYPE = ACTCTX_COMPATIBILITY_ELEMENT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_MITIGATION: ACTCTX_COMPATIBILITY_ELEMENT_TYPE = ACTCTX_COMPATIBILITY_ELEMENT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_OS: ACTCTX_COMPATIBILITY_ELEMENT_TYPE = ACTCTX_COMPATIBILITY_ELEMENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_UNKNOWN: ACTCTX_COMPATIBILITY_ELEMENT_TYPE = ACTCTX_COMPATIBILITY_ELEMENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ACTCTX_RUN_LEVEL_AS_INVOKER: ACTCTX_REQUESTED_RUN_LEVEL = ACTCTX_REQUESTED_RUN_LEVEL(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ACTCTX_RUN_LEVEL_HIGHEST_AVAILABLE: ACTCTX_REQUESTED_RUN_LEVEL = ACTCTX_REQUESTED_RUN_LEVEL(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ACTCTX_RUN_LEVEL_NUMBERS: ACTCTX_REQUESTED_RUN_LEVEL = ACTCTX_REQUESTED_RUN_LEVEL(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ACTCTX_RUN_LEVEL_REQUIRE_ADMIN: ACTCTX_REQUESTED_RUN_LEVEL = ACTCTX_REQUESTED_RUN_LEVEL(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ACTCTX_RUN_LEVEL_UNSPECIFIED: ACTCTX_REQUESTED_RUN_LEVEL = ACTCTX_REQUESTED_RUN_LEVEL(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ADVERTISEFLAGS_MACHINEASSIGN: ADVERTISEFLAGS = ADVERTISEFLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ADVERTISEFLAGS_USERASSIGN: ADVERTISEFLAGS = ADVERTISEFLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const APPLY_OPTION_FAIL_IF_CLOSE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const APPLY_OPTION_FAIL_IF_EXACT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const APPLY_OPTION_TEST_ONLY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const APPLY_OPTION_VALID_FLAGS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_BINDF_BINPATH_PROBE_ONLY: ASM_BIND_FLAGS = ASM_BIND_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_BINDF_FORCE_CACHE_INSTALL: ASM_BIND_FLAGS = ASM_BIND_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_BINDF_PARENT_ASM_HINT: ASM_BIND_FLAGS = ASM_BIND_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_BINDF_RFS_INTEGRITY_CHECK: ASM_BIND_FLAGS = ASM_BIND_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_BINDF_RFS_MODULE_CHECK: ASM_BIND_FLAGS = ASM_BIND_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_BINDF_SHARED_BINPATH_HINT: ASM_BIND_FLAGS = ASM_BIND_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_CMPF_ALL: ASM_CMP_FLAGS = ASM_CMP_FLAGS(255i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_CMPF_BUILD_NUMBER: ASM_CMP_FLAGS = ASM_CMP_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_CMPF_CULTURE: ASM_CMP_FLAGS = ASM_CMP_FLAGS(64i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_CMPF_CUSTOM: ASM_CMP_FLAGS = ASM_CMP_FLAGS(128i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_CMPF_DEFAULT: ASM_CMP_FLAGS = ASM_CMP_FLAGS(256i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_CMPF_MAJOR_VERSION: ASM_CMP_FLAGS = ASM_CMP_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_CMPF_MINOR_VERSION: ASM_CMP_FLAGS = ASM_CMP_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_CMPF_NAME: ASM_CMP_FLAGS = ASM_CMP_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_CMPF_PUBLIC_KEY_TOKEN: ASM_CMP_FLAGS = ASM_CMP_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_CMPF_REVISION_NUMBER: ASM_CMP_FLAGS = ASM_CMP_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_DISPLAYF_CULTURE: ASM_DISPLAY_FLAGS = ASM_DISPLAY_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_DISPLAYF_CUSTOM: ASM_DISPLAY_FLAGS = ASM_DISPLAY_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_DISPLAYF_LANGUAGEID: ASM_DISPLAY_FLAGS = ASM_DISPLAY_FLAGS(64i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_DISPLAYF_PROCESSORARCHITECTURE: ASM_DISPLAY_FLAGS = ASM_DISPLAY_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_DISPLAYF_PUBLIC_KEY: ASM_DISPLAY_FLAGS = ASM_DISPLAY_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_DISPLAYF_PUBLIC_KEY_TOKEN: ASM_DISPLAY_FLAGS = ASM_DISPLAY_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_DISPLAYF_VERSION: ASM_DISPLAY_FLAGS = ASM_DISPLAY_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_ALIAS: ASM_NAME = ASM_NAME(12i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_BUILD_NUMBER: ASM_NAME = ASM_NAME(6i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_CODEBASE_LASTMOD: ASM_NAME = ASM_NAME(14i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_CODEBASE_URL: ASM_NAME = ASM_NAME(13i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_CULTURE: ASM_NAME = ASM_NAME(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_CUSTOM: ASM_NAME = ASM_NAME(17i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_HASH_ALGID: ASM_NAME = ASM_NAME(11i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_HASH_VALUE: ASM_NAME = ASM_NAME(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_MAJOR_VERSION: ASM_NAME = ASM_NAME(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_MAX_PARAMS: ASM_NAME = ASM_NAME(20i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_MINOR_VERSION: ASM_NAME = ASM_NAME(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_MVID: ASM_NAME = ASM_NAME(19i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_NAME: ASM_NAME = ASM_NAME(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_NULL_CUSTOM: ASM_NAME = ASM_NAME(18i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_NULL_PUBLIC_KEY: ASM_NAME = ASM_NAME(15i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_NULL_PUBLIC_KEY_TOKEN: ASM_NAME = ASM_NAME(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_OSINFO_ARRAY: ASM_NAME = ASM_NAME(10i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_PROCESSOR_ID_ARRAY: ASM_NAME = ASM_NAME(9i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_PUBLIC_KEY: ASM_NAME = ASM_NAME(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_PUBLIC_KEY_TOKEN: ASM_NAME = ASM_NAME(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASM_NAME_REVISION_NUMBER: ASM_NAME = ASM_NAME(7i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASSEMBLYINFO_FLAG_INSTALLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ASSEMBLYINFO_FLAG_PAYLOADRESIDENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const CANOF_PARSE_DISPLAY_NAME: CREATE_ASM_NAME_OBJ_FLAGS = CREATE_ASM_NAME_OBJ_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const CANOF_SET_DEFAULT_VALUES: CREATE_ASM_NAME_OBJ_FLAGS = CREATE_ASM_NAME_OBJ_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const CLSID_EvalCom2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6e5e1910_8053_4660_b795_6b612e29bc58);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const CLSID_MsmMerge2: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf94985d5_29f9_4743_9805_99bc3f35b678);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const DEFAULT_DISK_ID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const DEFAULT_FILE_SEQUENCE_START: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const DEFAULT_MINIMUM_REQUIRED_MSI_VERSION: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const DELTA_MAX_HASH_SIZE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_BIGGER_THAN_COMPRESSED: u32 = 3222155525u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_CORRUPT: u32 = 3222159618u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_DECODE_FAILURE: u32 = 3222159617u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_ENCODE_FAILURE: u32 = 3222155521u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_IMAGEHLP_FAILURE: u32 = 3222155526u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_INVALID_OPTIONS: u32 = 3222155522u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_NEWER_FORMAT: u32 = 3222159619u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_NOT_AVAILABLE: u32 = 3222159622u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_NOT_NECESSARY: u32 = 3222159621u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_RETAIN_RANGES_DIFFER: u32 = 3222155524u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_SAME_FILE: u32 = 3222155523u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PATCH_WRONG_FILE: u32 = 3222159620u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_API_PATCHING_SYMBOL_FLAGS: u32 = 3222163725u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_FAMILY_RANGE_NAME: u32 = 3222163801u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_FILE_SEQUENCE_START: u32 = 3222163770u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_GUIDS_TO_REPLACE: u32 = 3222163721u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_IMAGE_FAMILY_DISKID: u32 = 3222163773u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_IMAGE_FAMILY_FILESEQSTART: u32 = 3222163774u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_IMAGE_FAMILY_NAME: u32 = 3222163748u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_IMAGE_FAMILY_SRC_PROP: u32 = 3222163750u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_MAJOR_VERSION: u32 = 3222163853u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_PATCH_GUID: u32 = 3222163720u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_PRODUCTVERSION_VALIDATION: u32 = 3222163844u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_SEQUENCE: u32 = 3222163848u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_SUPERCEDENCE: u32 = 3222163847u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_TARGET: u32 = 3222163849u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_TARGET_IMAGE_NAME: u32 = 3222163736u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_TARGET_IMAGE_PRODUCT_CODE: u32 = 3222163834u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_TARGET_IMAGE_PRODUCT_VERSION: u32 = 3222163835u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_TARGET_IMAGE_UPGRADED: u32 = 3222163776u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_TARGET_IMAGE_UPGRADE_CODE: u32 = 3222163836u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_TARGET_PRODUCT_CODE_LIST: u32 = 3222163722u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_TGT_UPD_IMAGES: u32 = 3222163846u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_TRANSFORMSET: u32 = 3222163845u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_FAMILY: u32 = 3222163775u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_NAME: u32 = 3222163728u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_PRODUCT_CODE: u32 = 3222163831u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_PRODUCT_VERSION: u32 = 3222163832u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_UPGRADE_CODE: u32 = 3222163833u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BAD_VERSION_STRING: u32 = 3222163852u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_BASE: u32 = 3222163713u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANNOT_CREATE_TABLE: u32 = 3222163841u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANNOT_RUN_MAKECAB: u32 = 3222163782u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANNOT_WRITE_DDF: u32 = 3222163781u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_COPY_FILE_TO_TEMP_FOLDER: u32 = 3222163771u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_CREATE_ONE_PATCH_FILE: u32 = 3222163772u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_CREATE_PATCH_FILE: u32 = 3222163718u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_CREATE_SUMMARY_INFO: u32 = 3222163828u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_CREATE_SUMMARY_INFO_POUND: u32 = 3222163830u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_CREATE_TEMP_FOLDER: u32 = 3222163715u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_DELETE_TEMP_FOLDER: u32 = 3222163974u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_GENERATE_SEQUENCEINFO_MAJORUPGD: u32 = 3222163842u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_GENERATE_TRANSFORM: u32 = 3222163827u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_GENERATE_TRANSFORM_POUND: u32 = 3222163829u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_OVERWRITE_PATCH: u32 = 3222163717u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CANT_READ_FILE: u32 = 3222163978u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_CREATEFILE_LOG_FAILED: u32 = 3222163861u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_DUPLICATE_SEQUENCE_RECORD: u32 = 3222163858u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_DUP_IMAGE_FAMILY_NAME: u32 = 3222163749u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_DUP_TARGET_IMAGE_NAME: u32 = 3222163737u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_DUP_TARGET_IMAGE_PACKCODE: u32 = 3222163777u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_DUP_UPGRADED_IMAGE_NAME: u32 = 3222163729u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_DUP_UPGRADED_IMAGE_PACKCODE: u32 = 3222163795u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_ERROR_WRITING_TO_LOG: u32 = 3222163864u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXECUTE_VIEW: u32 = 3222163870u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_BAD_FAMILY_FIELD: u32 = 3222163756u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_BAD_IGNORE_LENGTHS: u32 = 3222163814u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_BAD_IGNORE_OFFSETS: u32 = 3222163812u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_BAD_RETAIN_OFFSETS: u32 = 3222163817u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_BLANK_FILE_TABLE_KEY: u32 = 3222163755u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_BLANK_PATH_TO_FILE: u32 = 3222163758u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_IGNORE_COUNT_MISMATCH: u32 = 3222163815u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_LONG_FILE_TABLE_KEY: u32 = 3222163754u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_LONG_IGNORE_LENGTHS: u32 = 3222163813u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_LONG_IGNORE_OFFSETS: u32 = 3222163811u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_LONG_PATH_TO_FILE: u32 = 3222163757u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_LONG_RETAIN_OFFSETS: u32 = 3222163816u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_EXTFILE_MISSING_FILE: u32 = 3222163759u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAILED_CREATE_TRANSFORM: u32 = 3222163973u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAILED_EXPAND_PATH: u32 = 3222163872u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAMILY_RANGE_BAD_RETAIN_LENGTHS: u32 = 3222163809u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAMILY_RANGE_BAD_RETAIN_OFFSETS: u32 = 3222163806u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAMILY_RANGE_BLANK_FILE_TABLE_KEY: u32 = 3222163803u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAMILY_RANGE_BLANK_RETAIN_LENGTHS: u32 = 3222163808u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAMILY_RANGE_BLANK_RETAIN_OFFSETS: u32 = 3222163805u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAMILY_RANGE_COUNT_MISMATCH: u32 = 3222163810u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAMILY_RANGE_LONG_FILE_TABLE_KEY: u32 = 3222163802u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAMILY_RANGE_LONG_RETAIN_LENGTHS: u32 = 3222163807u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAMILY_RANGE_LONG_RETAIN_OFFSETS: u32 = 3222163804u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_FAMILY_RANGE_NAME_TOO_LONG: u32 = 3222163800u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_IMAGE_FAMILY_NAME_TOO_LONG: u32 = 3222163747u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_IMAGE_PATH_NOT_EXIST: u32 = 3222163988u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INTERNAL_ERROR: u32 = 3222163969u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_LOG_LEVEL: u32 = 3222163862u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_MAJOR_VERSION: u32 = 3222163990u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PARAMETER: u32 = 3222163860u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PATCHMETADATA_PROP: u32 = 3222163856u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PATCH_TYPE_SEQUENCING: u32 = 3222163977u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_EXTERNALFILES: u32 = 3222163982u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_FAMILYFILERANGES: u32 = 3222163992u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_IMAGEFAMILIES: u32 = 3222163983u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_PATCHSEQUENCE: u32 = 3222163984u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_PROPERTIES: u32 = 3222163991u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_PROPERTY: u32 = 3222163970u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_TARGETFILES_OPTIONALDATA: u32 = 3222163985u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_TARGETIMAGES: u32 = 3222163971u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_UPGRADEDFILESTOIGNORE: u32 = 3222163980u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_UPGRADEDFILES_OPTIONALDATA: u32 = 3222163986u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_PCP_UPGRADEDIMAGES: u32 = 3222163981u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_RANGE_ELEMENT: u32 = 3222163989u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_SUPERCEDENCE: u32 = 3222163857u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_SUPERSEDENCE_VALUE: u32 = 3222163976u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_INVALID_UI_LEVEL: u32 = 3222163863u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_LAX_VALIDATION_FLAGS: u32 = 3222163972u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_MAJOR_UPGD_WITHOUT_SEQUENCING: u32 = 3222163843u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_MATCHED_PRODUCT_VERSIONS: u32 = 3222163837u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_MISMATCHED_PRODUCT_CODES: u32 = 3222163779u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_MISMATCHED_PRODUCT_VERSIONS: u32 = 3222163780u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_MISSING_DIRECTORY_TABLE: u32 = 3222163975u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_MISSING_PATCHMETADATA: u32 = 3222163987u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_MISSING_PATCH_GUID: u32 = 3222163719u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_MISSING_PATCH_PATH: u32 = 3222163716u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_NO_UPGRADED_IMAGES_TO_PATCH: u32 = 3222163723u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_NULL_PATCHFAMILY: u32 = 3222163850u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_NULL_SEQUENCE_NUMBER: u32 = 3222163851u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_OBSOLETION_WITH_MSI30: u32 = 3222163839u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_OBSOLETION_WITH_PATCHSEQUENCE: u32 = 3222163840u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_OBSOLETION_WITH_SEQUENCE_DATA: u32 = 3222163838u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_OODS_COPYING_MSI: u32 = 3222163726u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_OPEN_VIEW: u32 = 3222163869u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_OUT_OF_MEMORY: u32 = 3222163865u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_PATCHMETADATA_PROP_NOT_SET: u32 = 3222163855u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_PCP_BAD_FORMAT: u32 = 3222163714u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_PCP_DOESNT_EXIST: u32 = 3222163713u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_SEQUENCING_BAD_TARGET: u32 = 3222163854u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TARGET_BAD_PROD_CODE_VAL: u32 = 3222163744u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TARGET_BAD_PROD_VALIDATE: u32 = 3222163743u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TARGET_IMAGE_COMPRESSED: u32 = 3222163742u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TARGET_IMAGE_NAME_TOO_LONG: u32 = 3222163735u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TARGET_IMAGE_PATH_EMPTY: u32 = 3222163739u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TARGET_IMAGE_PATH_NOT_EXIST: u32 = 3222163740u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TARGET_IMAGE_PATH_NOT_MSI: u32 = 3222163741u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TARGET_IMAGE_PATH_TOO_LONG: u32 = 3222163738u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TARGET_MISSING_SRC_FILES: u32 = 3222163746u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TARGET_WRONG_PRODUCT_VERSION_COMP: u32 = 3222163979u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_BAD_IGNORE_LENGTHS: u32 = 3222163822u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_BAD_IGNORE_OFFSETS: u32 = 3222163820u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_BAD_RETAIN_OFFSETS: u32 = 3222163825u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_BAD_TARGET_FIELD: u32 = 3222163791u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_BLANK_FILE_TABLE_KEY: u32 = 3222163789u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_IGNORE_COUNT_MISMATCH: u32 = 3222163823u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_LONG_FILE_TABLE_KEY: u32 = 3222163788u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_LONG_IGNORE_LENGTHS: u32 = 3222163821u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_LONG_IGNORE_OFFSETS: u32 = 3222163819u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_LONG_RETAIN_OFFSETS: u32 = 3222163824u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_TFILEDATA_MISSING_FILE_TABLE_KEY: u32 = 3222163790u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UFILEDATA_BAD_UPGRADED_FIELD: u32 = 3222163778u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UFILEDATA_BLANK_FILE_TABLE_KEY: u32 = 3222163752u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UFILEDATA_LONG_FILE_TABLE_KEY: u32 = 3222163751u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UFILEDATA_MISSING_FILE_TABLE_KEY: u32 = 3222163753u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UFILEIGNORE_BAD_FILE_TABLE_KEY: u32 = 3222163799u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UFILEIGNORE_BAD_UPGRADED_FIELD: u32 = 3222163796u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UFILEIGNORE_BLANK_FILE_TABLE_KEY: u32 = 3222163798u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UFILEIGNORE_LONG_FILE_TABLE_KEY: u32 = 3222163797u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UNKNOWN_ERROR: u32 = 3222163866u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UNKNOWN_INFO: u32 = 3222163867u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UNKNOWN_WARN: u32 = 3222163868u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UPGRADED_IMAGE_COMPRESSED: u32 = 3222163734u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UPGRADED_IMAGE_NAME_TOO_LONG: u32 = 3222163727u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UPGRADED_IMAGE_PATCH_PATH_NOT_EXIST: u32 = 3222163793u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UPGRADED_IMAGE_PATCH_PATH_NOT_MSI: u32 = 3222163794u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UPGRADED_IMAGE_PATCH_PATH_TOO_LONG: u32 = 3222163792u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_EMPTY: u32 = 3222163731u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_NOT_EXIST: u32 = 3222163732u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_NOT_MSI: u32 = 3222163733u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_TOO_LONG: u32 = 3222163730u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_UPGRADED_MISSING_SRC_FILES: u32 = 3222163745u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_VIEW_FETCH: u32 = 3222163871u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_WRITE_SUMMARY_PROPERTIES: u32 = 3222163787u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_PCW_WRONG_PATCHMETADATA_STRD_PROP: u32 = 3222163859u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ERROR_ROLLBACK_DISABLED: u32 = 1653u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const FUSION_REFCOUNT_FILEPATH_GUID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb02f9d65_fb77_4f7a_afa5_b391309f11c9);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const FUSION_REFCOUNT_OPAQUE_STRING_GUID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2ec93463_b0c3_45e1_8364_327e96aea856);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const FUSION_REFCOUNT_UNINSTALL_SUBKEY_GUID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8cedc215_ac4b_488b_93c0_a50a49cb2fb8);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IACTIONNAME_ADMIN: ::windows_core::PCWSTR = ::windows_core::w!("ADMIN");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IACTIONNAME_ADVERTISE: ::windows_core::PCWSTR = ::windows_core::w!("ADVERTISE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IACTIONNAME_COLLECTUSERINFO: ::windows_core::PCWSTR = ::windows_core::w!("CollectUserInfo");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IACTIONNAME_FIRSTRUN: ::windows_core::PCWSTR = ::windows_core::w!("FirstRun");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IACTIONNAME_INSTALL: ::windows_core::PCWSTR = ::windows_core::w!("INSTALL");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IACTIONNAME_SEQUENCE: ::windows_core::PCWSTR = ::windows_core::w!("SEQUENCE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IASSEMBLYCACHEITEM_COMMIT_DISPOSITION_ALREADY_INSTALLED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IASSEMBLYCACHEITEM_COMMIT_DISPOSITION_INSTALLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IASSEMBLYCACHEITEM_COMMIT_DISPOSITION_REFRESHED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IASSEMBLYCACHEITEM_COMMIT_FLAG_REFRESH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_ALREADY_UNINSTALLED: IASSEMBLYCACHE_UNINSTALL_DISPOSITION = IASSEMBLYCACHE_UNINSTALL_DISPOSITION(3u32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_DELETE_PENDING: IASSEMBLYCACHE_UNINSTALL_DISPOSITION = IASSEMBLYCACHE_UNINSTALL_DISPOSITION(4u32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_STILL_IN_USE: IASSEMBLYCACHE_UNINSTALL_DISPOSITION = IASSEMBLYCACHE_UNINSTALL_DISPOSITION(2u32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_UNINSTALLED: IASSEMBLYCACHE_UNINSTALL_DISPOSITION = IASSEMBLYCACHE_UNINSTALL_DISPOSITION(1u32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_BASE: u32 = 3222229249u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_ENTERING_PHASE_I: u32 = 3222229251u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_ENTERING_PHASE_II: u32 = 3222229256u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_ENTERING_PHASE_III: u32 = 3222229257u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_ENTERING_PHASE_IV: u32 = 3222229258u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_ENTERING_PHASE_I_VALIDATION: u32 = 3222229250u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_ENTERING_PHASE_V: u32 = 3222229259u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_GENERATING_METADATA: u32 = 3222229265u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_PASSED_MAIN_CONTROL: u32 = 3222229249u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_PATCHCACHE_FILEINFO_FAILURE: u32 = 3222229267u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_PATCHCACHE_PCI_READFAILURE: u32 = 3222229268u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_PATCHCACHE_PCI_WRITEFAILURE: u32 = 3222229269u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_PCP_PATH: u32 = 3222229252u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_PROPERTY: u32 = 3222229255u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_SET_OPTIONS: u32 = 3222229254u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_SUCCESSFUL_PATCH_CREATION: u32 = 3222229271u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_TEMP_DIR: u32 = 3222229253u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_TEMP_DIR_CLEANUP: u32 = 3222229266u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INFO_USING_USER_MSI_FOR_PATCH_TABLES: u32 = 3222229270u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLFEATUREATTRIBUTE_DISALLOWADVERTISE: INSTALLFEATUREATTRIBUTE = INSTALLFEATUREATTRIBUTE(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLFEATUREATTRIBUTE_FAVORADVERTISE: INSTALLFEATUREATTRIBUTE = INSTALLFEATUREATTRIBUTE(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLFEATUREATTRIBUTE_FAVORLOCAL: INSTALLFEATUREATTRIBUTE = INSTALLFEATUREATTRIBUTE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLFEATUREATTRIBUTE_FAVORSOURCE: INSTALLFEATUREATTRIBUTE = INSTALLFEATUREATTRIBUTE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLFEATUREATTRIBUTE_FOLLOWPARENT: INSTALLFEATUREATTRIBUTE = INSTALLFEATUREATTRIBUTE(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLFEATUREATTRIBUTE_NOUNSUPPORTEDADVERTISE: INSTALLFEATUREATTRIBUTE = INSTALLFEATUREATTRIBUTE(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLEVEL_DEFAULT: INSTALLLEVEL = INSTALLLEVEL(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLEVEL_MAXIMUM: INSTALLLEVEL = INSTALLLEVEL(65535i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLEVEL_MINIMUM: INSTALLLEVEL = INSTALLLEVEL(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGATTRIBUTES_APPEND: INSTALLLOGATTRIBUTES = INSTALLLOGATTRIBUTES(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGATTRIBUTES_FLUSHEACHLINE: INSTALLLOGATTRIBUTES = INSTALLLOGATTRIBUTES(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_ACTIONDATA: INSTALLLOGMODE = INSTALLLOGMODE(512i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_ACTIONSTART: INSTALLLOGMODE = INSTALLLOGMODE(256i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_COMMONDATA: INSTALLLOGMODE = INSTALLLOGMODE(2048i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_ERROR: INSTALLLOGMODE = INSTALLLOGMODE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_EXTRADEBUG: INSTALLLOGMODE = INSTALLLOGMODE(8192i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_FATALEXIT: INSTALLLOGMODE = INSTALLLOGMODE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_FILESINUSE: INSTALLLOGMODE = INSTALLLOGMODE(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_INFO: INSTALLLOGMODE = INSTALLLOGMODE(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_INITIALIZE: INSTALLLOGMODE = INSTALLLOGMODE(4096i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_INSTALLEND: INSTALLLOGMODE = INSTALLLOGMODE(134217728i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_INSTALLSTART: INSTALLLOGMODE = INSTALLLOGMODE(67108864i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_LOGONLYONERROR: INSTALLLOGMODE = INSTALLLOGMODE(16384i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_LOGPERFORMANCE: INSTALLLOGMODE = INSTALLLOGMODE(32768i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_OUTOFDISKSPACE: INSTALLLOGMODE = INSTALLLOGMODE(128i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_PROGRESS: INSTALLLOGMODE = INSTALLLOGMODE(1024i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_PROPERTYDUMP: INSTALLLOGMODE = INSTALLLOGMODE(1024i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_RESOLVESOURCE: INSTALLLOGMODE = INSTALLLOGMODE(64i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_RMFILESINUSE: INSTALLLOGMODE = INSTALLLOGMODE(33554432i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_SHOWDIALOG: INSTALLLOGMODE = INSTALLLOGMODE(16384i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_TERMINATE: INSTALLLOGMODE = INSTALLLOGMODE(8192i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_USER: INSTALLLOGMODE = INSTALLLOGMODE(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_VERBOSE: INSTALLLOGMODE = INSTALLLOGMODE(4096i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLLOGMODE_WARNING: INSTALLLOGMODE = INSTALLLOGMODE(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_ACTIONDATA: INSTALLMESSAGE = INSTALLMESSAGE(150994944i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_ACTIONSTART: INSTALLMESSAGE = INSTALLMESSAGE(134217728i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_COMMONDATA: INSTALLMESSAGE = INSTALLMESSAGE(184549376i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_ERROR: INSTALLMESSAGE = INSTALLMESSAGE(16777216i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_FATALEXIT: INSTALLMESSAGE = INSTALLMESSAGE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_FILESINUSE: INSTALLMESSAGE = INSTALLMESSAGE(83886080i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_INFO: INSTALLMESSAGE = INSTALLMESSAGE(67108864i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_INITIALIZE: INSTALLMESSAGE = INSTALLMESSAGE(201326592i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_INSTALLEND: INSTALLMESSAGE = INSTALLMESSAGE(452984832i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_INSTALLSTART: INSTALLMESSAGE = INSTALLMESSAGE(436207616i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_OUTOFDISKSPACE: INSTALLMESSAGE = INSTALLMESSAGE(117440512i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_PERFORMANCE: INSTALLMESSAGE = INSTALLMESSAGE(251658240i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_PROGRESS: INSTALLMESSAGE = INSTALLMESSAGE(167772160i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_RESOLVESOURCE: INSTALLMESSAGE = INSTALLMESSAGE(100663296i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_RMFILESINUSE: INSTALLMESSAGE = INSTALLMESSAGE(419430400i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_SHOWDIALOG: INSTALLMESSAGE = INSTALLMESSAGE(234881024i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_TERMINATE: INSTALLMESSAGE = INSTALLMESSAGE(218103808i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_TYPEMASK: i32 = -16777216i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_USER: INSTALLMESSAGE = INSTALLMESSAGE(50331648i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMESSAGE_WARNING: INSTALLMESSAGE = INSTALLMESSAGE(33554432i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMODE_DEFAULT: INSTALLMODE = INSTALLMODE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMODE_EXISTING: INSTALLMODE = INSTALLMODE(-1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMODE_NODETECTION: INSTALLMODE = INSTALLMODE(-2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMODE_NODETECTION_ANY: INSTALLMODE = INSTALLMODE(-4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLMODE_NOSOURCERESOLUTION: INSTALLMODE = INSTALLMODE(-3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_ASSIGNMENTTYPE: ::windows_core::PCWSTR = ::windows_core::w!("AssignmentType");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_AUTHORIZED_LUA_APP: ::windows_core::PCWSTR = ::windows_core::w!("AuthorizedLUAApp");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_DISKPROMPT: ::windows_core::PCWSTR = ::windows_core::w!("DiskPrompt");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_DISPLAYNAME: ::windows_core::PCWSTR = ::windows_core::w!("DisplayName");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_HELPLINK: ::windows_core::PCWSTR = ::windows_core::w!("HelpLink");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_HELPTELEPHONE: ::windows_core::PCWSTR = ::windows_core::w!("HelpTelephone");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_INSTALLDATE: ::windows_core::PCWSTR = ::windows_core::w!("InstallDate");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_INSTALLEDLANGUAGE: ::windows_core::PCWSTR = ::windows_core::w!("InstalledLanguage");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_INSTALLEDPRODUCTNAME: ::windows_core::PCWSTR = ::windows_core::w!("InstalledProductName");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_INSTALLLOCATION: ::windows_core::PCWSTR = ::windows_core::w!("InstallLocation");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_INSTALLSOURCE: ::windows_core::PCWSTR = ::windows_core::w!("InstallSource");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_INSTANCETYPE: ::windows_core::PCWSTR = ::windows_core::w!("InstanceType");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_LANGUAGE: ::windows_core::PCWSTR = ::windows_core::w!("Language");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_LASTUSEDSOURCE: ::windows_core::PCWSTR = ::windows_core::w!("LastUsedSource");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_LASTUSEDTYPE: ::windows_core::PCWSTR = ::windows_core::w!("LastUsedType");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_LOCALPACKAGE: ::windows_core::PCWSTR = ::windows_core::w!("LocalPackage");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_LUAENABLED: ::windows_core::PCWSTR = ::windows_core::w!("LUAEnabled");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_MEDIAPACKAGEPATH: ::windows_core::PCWSTR = ::windows_core::w!("MediaPackagePath");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_MOREINFOURL: ::windows_core::PCWSTR = ::windows_core::w!("MoreInfoURL");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_PACKAGECODE: ::windows_core::PCWSTR = ::windows_core::w!("PackageCode");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_PACKAGENAME: ::windows_core::PCWSTR = ::windows_core::w!("PackageName");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_PATCHSTATE: ::windows_core::PCWSTR = ::windows_core::w!("State");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_PATCHTYPE: ::windows_core::PCWSTR = ::windows_core::w!("PatchType");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_PRODUCTICON: ::windows_core::PCWSTR = ::windows_core::w!("ProductIcon");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_PRODUCTID: ::windows_core::PCWSTR = ::windows_core::w!("ProductID");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_PRODUCTNAME: ::windows_core::PCWSTR = ::windows_core::w!("ProductName");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_PRODUCTSTATE: ::windows_core::PCWSTR = ::windows_core::w!("State");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_PUBLISHER: ::windows_core::PCWSTR = ::windows_core::w!("Publisher");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_REGCOMPANY: ::windows_core::PCWSTR = ::windows_core::w!("RegCompany");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_REGOWNER: ::windows_core::PCWSTR = ::windows_core::w!("RegOwner");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_TRANSFORMS: ::windows_core::PCWSTR = ::windows_core::w!("Transforms");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_UNINSTALLABLE: ::windows_core::PCWSTR = ::windows_core::w!("Uninstallable");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_URLINFOABOUT: ::windows_core::PCWSTR = ::windows_core::w!("URLInfoAbout");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_URLUPDATEINFO: ::windows_core::PCWSTR = ::windows_core::w!("URLUpdateInfo");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_VERSION: ::windows_core::PCWSTR = ::windows_core::w!("Version");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_VERSIONMAJOR: ::windows_core::PCWSTR = ::windows_core::w!("VersionMajor");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_VERSIONMINOR: ::windows_core::PCWSTR = ::windows_core::w!("VersionMinor");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLPROPERTY_VERSIONSTRING: ::windows_core::PCWSTR = ::windows_core::w!("VersionString");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_ABSENT: INSTALLSTATE = INSTALLSTATE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_ADVERTISED: INSTALLSTATE = INSTALLSTATE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_BADCONFIG: INSTALLSTATE = INSTALLSTATE(-6i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_BROKEN: INSTALLSTATE = INSTALLSTATE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_DEFAULT: INSTALLSTATE = INSTALLSTATE(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_INCOMPLETE: INSTALLSTATE = INSTALLSTATE(-5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_INVALIDARG: INSTALLSTATE = INSTALLSTATE(-2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_LOCAL: INSTALLSTATE = INSTALLSTATE(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_MOREDATA: INSTALLSTATE = INSTALLSTATE(-3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_NOTUSED: INSTALLSTATE = INSTALLSTATE(-7i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_REMOVED: INSTALLSTATE = INSTALLSTATE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_SOURCE: INSTALLSTATE = INSTALLSTATE(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_SOURCEABSENT: INSTALLSTATE = INSTALLSTATE(-4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLSTATE_UNKNOWN: INSTALLSTATE = INSTALLSTATE(-1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLTYPE_DEFAULT: INSTALLTYPE = INSTALLTYPE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLTYPE_NETWORK_IMAGE: INSTALLTYPE = INSTALLTYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLTYPE_SINGLE_INSTANCE: INSTALLTYPE = INSTALLTYPE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_BASIC: INSTALLUILEVEL = INSTALLUILEVEL(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_DEFAULT: INSTALLUILEVEL = INSTALLUILEVEL(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_ENDDIALOG: INSTALLUILEVEL = INSTALLUILEVEL(128i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_FULL: INSTALLUILEVEL = INSTALLUILEVEL(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_HIDECANCEL: INSTALLUILEVEL = INSTALLUILEVEL(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_NOCHANGE: INSTALLUILEVEL = INSTALLUILEVEL(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_NONE: INSTALLUILEVEL = INSTALLUILEVEL(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_PROGRESSONLY: INSTALLUILEVEL = INSTALLUILEVEL(64i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_REDUCED: INSTALLUILEVEL = INSTALLUILEVEL(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_SOURCERESONLY: INSTALLUILEVEL = INSTALLUILEVEL(256i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const INSTALLUILEVEL_UACONLY: INSTALLUILEVEL = INSTALLUILEVEL(512i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ACTION: ::windows_core::PCWSTR = ::windows_core::w!("ACTION");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ADMINTOOLS_FOLDER: ::windows_core::PCWSTR = ::windows_core::w!("AdminToolsFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ADMINUSER: ::windows_core::PCWSTR = ::windows_core::w!("AdminUser");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ADMIN_PROPERTIES: ::windows_core::PCWSTR = ::windows_core::w!("AdminProperties");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_AFTERREBOOT: ::windows_core::PCWSTR = ::windows_core::w!("AFTERREBOOT");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ALLOWEDPROPERTIES: ::windows_core::PCWSTR = ::windows_core::w!("SecureCustomProperties");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ALLUSERS: ::windows_core::PCWSTR = ::windows_core::w!("ALLUSERS");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_APPDATA_FOLDER: ::windows_core::PCWSTR = ::windows_core::w!("AppDataFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARM: ::windows_core::PCWSTR = ::windows_core::w!("Arm");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARM64: ::windows_core::PCWSTR = ::windows_core::w!("Arm64");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPAUTHORIZEDCDFPREFIX: ::windows_core::PCWSTR = ::windows_core::w!("ARPAUTHORIZEDCDFPREFIX");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPCOMMENTS: ::windows_core::PCWSTR = ::windows_core::w!("ARPCOMMENTS");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPCONTACT: ::windows_core::PCWSTR = ::windows_core::w!("ARPCONTACT");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPHELPLINK: ::windows_core::PCWSTR = ::windows_core::w!("ARPHELPLINK");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPHELPTELEPHONE: ::windows_core::PCWSTR = ::windows_core::w!("ARPHELPTELEPHONE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPINSTALLLOCATION: ::windows_core::PCWSTR = ::windows_core::w!("ARPINSTALLLOCATION");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPNOMODIFY: ::windows_core::PCWSTR = ::windows_core::w!("ARPNOMODIFY");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPNOREMOVE: ::windows_core::PCWSTR = ::windows_core::w!("ARPNOREMOVE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPNOREPAIR: ::windows_core::PCWSTR = ::windows_core::w!("ARPNOREPAIR");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPPRODUCTICON: ::windows_core::PCWSTR = ::windows_core::w!("ARPPRODUCTICON");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPREADME: ::windows_core::PCWSTR = ::windows_core::w!("ARPREADME");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPSETTINGSIDENTIFIER: ::windows_core::PCWSTR = ::windows_core::w!("MSIARPSETTINGSIDENTIFIER");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPSHIMFLAGS: ::windows_core::PCWSTR = ::windows_core::w!("SHIMFLAGS");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPSHIMSERVICEPACKLEVEL: ::windows_core::PCWSTR = ::windows_core::w!("SHIMSERVICEPACKLEVEL");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPSHIMVERSIONNT: ::windows_core::PCWSTR = ::windows_core::w!("SHIMVERSIONNT");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPSIZE: ::windows_core::PCWSTR = ::windows_core::w!("ARPSIZE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPSYSTEMCOMPONENT: ::windows_core::PCWSTR = ::windows_core::w!("ARPSYSTEMCOMPONENT");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPURLINFOABOUT: ::windows_core::PCWSTR = ::windows_core::w!("ARPURLINFOABOUT");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ARPURLUPDATEINFO: ::windows_core::PCWSTR = ::windows_core::w!("ARPURLUPDATEINFO");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_AVAILABLEFREEREG: ::windows_core::PCWSTR = ::windows_core::w!("AVAILABLEFREEREG");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_BORDERSIDE: ::windows_core::PCWSTR = ::windows_core::w!("BorderSide");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_BORDERTOP: ::windows_core::PCWSTR = ::windows_core::w!("BorderTop");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_CAPTIONHEIGHT: ::windows_core::PCWSTR = ::windows_core::w!("CaptionHeight");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_CARRYINGNDP: ::windows_core::PCWSTR = ::windows_core::w!("CARRYINGNDP");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_CHECKCRCS: ::windows_core::PCWSTR = ::windows_core::w!("MSICHECKCRCS");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_COLORBITS: ::windows_core::PCWSTR = ::windows_core::w!("ColorBits");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_COMMONAPPDATA_FOLDER: ::windows_core::PCWSTR = ::windows_core::w!("CommonAppDataFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_COMMONFILES64_FOLDER: ::windows_core::PCWSTR = ::windows_core::w!("CommonFiles64Folder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_COMMONFILES_FOLDER: ::windows_core::PCWSTR = ::windows_core::w!("CommonFilesFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_COMPANYNAME: ::windows_core::PCWSTR = ::windows_core::w!("COMPANYNAME");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_COMPONENTADDDEFAULT: ::windows_core::PCWSTR = ::windows_core::w!("COMPADDDEFAULT");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_COMPONENTADDLOCAL: ::windows_core::PCWSTR = ::windows_core::w!("COMPADDLOCAL");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_COMPONENTADDSOURCE: ::windows_core::PCWSTR = ::windows_core::w!("COMPADDSOURCE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_COMPUTERNAME: ::windows_core::PCWSTR = ::windows_core::w!("ComputerName");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_COSTINGCOMPLETE: ::windows_core::PCWSTR = ::windows_core::w!("CostingComplete");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_CUSTOMACTIONDATA: ::windows_core::PCWSTR = ::windows_core::w!("CustomActionData");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_DATE: ::windows_core::PCWSTR = ::windows_core::w!("Date");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_DATETIME: ::windows_core::PCWSTR = ::windows_core::w!("DateTime");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_DEFAULTUIFONT: ::windows_core::PCWSTR = ::windows_core::w!("DefaultUIFont");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_DESKTOP_FOLDER: ::windows_core::PCWSTR = ::windows_core::w!("DesktopFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_DISABLEADVTSHORTCUTS: ::windows_core::PCWSTR = ::windows_core::w!("DISABLEADVTSHORTCUTS");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_DISABLEROLLBACK: ::windows_core::PCWSTR = ::windows_core::w!("DISABLEROLLBACK");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_DISKPROMPT: ::windows_core::PCWSTR = ::windows_core::w!("DiskPrompt");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ENABLEUSERCONTROL: ::windows_core::PCWSTR = ::windows_core::w!("EnableUserControl");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ENFORCE_UPGRADE_COMPONENT_RULES: ::windows_core::PCWSTR = ::windows_core::w!("MSIENFORCEUPGRADECOMPONENTRULES");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_EXECUTEACTION: ::windows_core::PCWSTR = ::windows_core::w!("EXECUTEACTION");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_EXECUTEMODE: ::windows_core::PCWSTR = ::windows_core::w!("EXECUTEMODE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_FAVORITES_FOLDER: ::windows_core::PCWSTR = ::windows_core::w!("FavoritesFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_FEATUREADDDEFAULT: ::windows_core::PCWSTR = ::windows_core::w!("ADDDEFAULT");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_FEATUREADDLOCAL: ::windows_core::PCWSTR = ::windows_core::w!("ADDLOCAL");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_FEATUREADDSOURCE: ::windows_core::PCWSTR = ::windows_core::w!("ADDSOURCE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_FEATUREADVERTISE: ::windows_core::PCWSTR = ::windows_core::w!("ADVERTISE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_FEATUREREMOVE: ::windows_core::PCWSTR = ::windows_core::w!("REMOVE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_FILEADDDEFAULT: ::windows_core::PCWSTR = ::windows_core::w!("FILEADDDEFAULT");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_FILEADDLOCAL: ::windows_core::PCWSTR = ::windows_core::w!("FILEADDLOCAL");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_FILEADDSOURCE: ::windows_core::PCWSTR = ::windows_core::w!("FILEADDSOURCE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_FONTS_FOLDER: ::windows_core::PCWSTR = ::windows_core::w!("FontsFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_HIDDEN_PROPERTIES: ::windows_core::PCWSTR = ::windows_core::w!("MsiHiddenProperties");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_HIDECANCEL: ::windows_core::PCWSTR = ::windows_core::w!("MsiUIHideCancel");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_IA64: ::windows_core::PCWSTR = ::windows_core::w!("IA64");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_INSTALLED: ::windows_core::PCWSTR = ::windows_core::w!("Installed");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_INSTALLLANGUAGE: ::windows_core::PCWSTR = ::windows_core::w!("ProductLanguage");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_INSTALLLEVEL: ::windows_core::PCWSTR = ::windows_core::w!("INSTALLLEVEL");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_INSTALLPERUSER: ::windows_core::PCWSTR = ::windows_core::w!("MSIINSTALLPERUSER");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_INTEL: ::windows_core::PCWSTR = ::windows_core::w!("Intel");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_INTEL64: ::windows_core::PCWSTR = ::windows_core::w!("Intel64");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_INTERNALINSTALLEDPERUSER: ::windows_core::PCWSTR = ::windows_core::w!("MSIINTERNALINSTALLEDPERUSER");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ISADMINPACKAGE: ::windows_core::PCWSTR = ::windows_core::w!("IsAdminPackage");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_LEFTUNIT: ::windows_core::PCWSTR = ::windows_core::w!("LeftUnit");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_LIMITUI: ::windows_core::PCWSTR = ::windows_core::w!("LIMITUI");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_LOCALAPPDATA_FOLDER: ::windows_core::PCWSTR = ::windows_core::w!("LocalAppDataFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_LOGACTION: ::windows_core::PCWSTR = ::windows_core::w!("LOGACTION");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_LOGONUSER: ::windows_core::PCWSTR = ::windows_core::w!("LogonUser");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MANUFACTURER: ::windows_core::PCWSTR = ::windows_core::w!("Manufacturer");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSIAMD64: ::windows_core::PCWSTR = ::windows_core::w!("MsiAMD64");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSIDISABLEEEUI: ::windows_core::PCWSTR = ::windows_core::w!("MSIDISABLEEEUI");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSIDISABLELUAPATCHING: ::windows_core::PCWSTR = ::windows_core::w!("MSIDISABLELUAPATCHING");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSIINSTANCEGUID: ::windows_core::PCWSTR = ::windows_core::w!("MSIINSTANCEGUID");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSILOGFILELOCATION: ::windows_core::PCWSTR = ::windows_core::w!("MsiLogFileLocation");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSILOGGINGMODE: ::windows_core::PCWSTR = ::windows_core::w!("MsiLogging");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSINEWINSTANCE: ::windows_core::PCWSTR = ::windows_core::w!("MSINEWINSTANCE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSINODISABLEMEDIA: ::windows_core::PCWSTR = ::windows_core::w!("MSINODISABLEMEDIA");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSIPACKAGEDOWNLOADLOCALCOPY: ::windows_core::PCWSTR = ::windows_core::w!("MSIPACKAGEDOWNLOADLOCALCOPY");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSIPATCHDOWNLOADLOCALCOPY: ::windows_core::PCWSTR = ::windows_core::w!("MSIPATCHDOWNLOADLOCALCOPY");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSIPATCHREMOVE: ::windows_core::PCWSTR = ::windows_core::w!("MSIPATCHREMOVE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSITABLETPC: ::windows_core::PCWSTR = ::windows_core::w!("MsiTabletPC");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSIX64: ::windows_core::PCWSTR = ::windows_core::w!("Msix64");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSI_FASTINSTALL: ::windows_core::PCWSTR = ::windows_core::w!("MSIFASTINSTALL");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSI_REBOOT_PENDING: ::windows_core::PCWSTR = ::windows_core::w!("MsiSystemRebootPending");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSI_RM_CONTROL: ::windows_core::PCWSTR = ::windows_core::w!("MSIRESTARTMANAGERCONTROL");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSI_RM_DISABLE_RESTART: ::windows_core::PCWSTR = ::windows_core::w!("MSIDISABLERMRESTART");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSI_RM_SESSION_KEY: ::windows_core::PCWSTR = ::windows_core::w!("MsiRestartManagerSessionKey");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSI_RM_SHUTDOWN: ::windows_core::PCWSTR = ::windows_core::w!("MSIRMSHUTDOWN");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSI_UAC_DEPLOYMENT_COMPLIANT: ::windows_core::PCWSTR = ::windows_core::w!("MSIDEPLOYMENTCOMPLIANT");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSI_UNINSTALL_SUPERSEDED_COMPONENTS: ::windows_core::PCWSTR = ::windows_core::w!("MSIUNINSTALLSUPERSEDEDCOMPONENTS");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MSI_USE_REAL_ADMIN_DETECTION: ::windows_core::PCWSTR = ::windows_core::w!("MSIUSEREALADMINDETECTION");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_MYPICTURES_FOLDER: ::windows_core::PCWSTR = ::windows_core::w!("MyPicturesFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NETASSEMBLYSUPPORT: ::windows_core::PCWSTR = ::windows_core::w!("MsiNetAssemblySupport");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NETHOOD_FOLDER: ::windows_core::PCWSTR = ::windows_core::w!("NetHoodFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NOCOMPANYNAME: ::windows_core::PCWSTR = ::windows_core::w!("NOCOMPANYNAME");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NOUSERNAME: ::windows_core::PCWSTR = ::windows_core::w!("NOUSERNAME");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NTPRODUCTTYPE: ::windows_core::PCWSTR = ::windows_core::w!("MsiNTProductType");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NTSUITEBACKOFFICE: ::windows_core::PCWSTR = ::windows_core::w!("MsiNTSuiteBackOffice");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NTSUITEDATACENTER: ::windows_core::PCWSTR = ::windows_core::w!("MsiNTSuiteDataCenter");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NTSUITEENTERPRISE: ::windows_core::PCWSTR = ::windows_core::w!("MsiNTSuiteEnterprise");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NTSUITEPERSONAL: ::windows_core::PCWSTR = ::windows_core::w!("MsiNTSuitePersonal");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NTSUITESMALLBUSINESS: ::windows_core::PCWSTR = ::windows_core::w!("MsiNTSuiteSmallBusiness");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NTSUITESMALLBUSINESSRESTRICTED: ::windows_core::PCWSTR = ::windows_core::w!("MsiNTSuiteSmallBusinessRestricted");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_NTSUITEWEBSERVER: ::windows_core::PCWSTR = ::windows_core::w!("MsiNTSuiteWebServer");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_OLEADVTSUPPORT: ::windows_core::PCWSTR = ::windows_core::w!("OLEAdvtSupport");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_OUTOFDISKSPACE: ::windows_core::PCWSTR = ::windows_core::w!("OutOfDiskSpace");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_OUTOFNORBDISKSPACE: ::windows_core::PCWSTR = ::windows_core::w!("OutOfNoRbDiskSpace");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PATCH: ::windows_core::PCWSTR = ::windows_core::w!("PATCH");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PATCHNEWPACKAGECODE: ::windows_core::PCWSTR = ::windows_core::w!("PATCHNEWPACKAGECODE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PATCHNEWSUMMARYCOMMENTS: ::windows_core::PCWSTR = ::windows_core::w!("PATCHNEWSUMMARYCOMMENTS");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PATCHNEWSUMMARYSUBJECT: ::windows_core::PCWSTR = ::windows_core::w!("PATCHNEWSUMMARYSUBJECT");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PERSONAL_FOLDER: ::windows_core::PCWSTR = ::windows_core::w!("PersonalFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PHYSICALMEMORY: ::windows_core::PCWSTR = ::windows_core::w!("PhysicalMemory");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PIDKEY: ::windows_core::PCWSTR = ::windows_core::w!("PIDKEY");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PIDTEMPLATE: ::windows_core::PCWSTR = ::windows_core::w!("PIDTemplate");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRESELECTED: ::windows_core::PCWSTR = ::windows_core::w!("Preselected");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRIMARYFOLDER: ::windows_core::PCWSTR = ::windows_core::w!("PRIMARYFOLDER");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRIMARYFOLDER_PATH: ::windows_core::PCWSTR = ::windows_core::w!("PrimaryVolumePath");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRIMARYFOLDER_SPACEAVAILABLE: ::windows_core::PCWSTR = ::windows_core::w!("PrimaryVolumeSpaceAvailable");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRIMARYFOLDER_SPACEREMAINING: ::windows_core::PCWSTR = ::windows_core::w!("PrimaryVolumeSpaceRemaining");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRIMARYFOLDER_SPACEREQUIRED: ::windows_core::PCWSTR = ::windows_core::w!("PrimaryVolumeSpaceRequired");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRINTHOOD_FOLDER: ::windows_core::PCWSTR = ::windows_core::w!("PrintHoodFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRIVILEGED: ::windows_core::PCWSTR = ::windows_core::w!("Privileged");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRODUCTCODE: ::windows_core::PCWSTR = ::windows_core::w!("ProductCode");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRODUCTID: ::windows_core::PCWSTR = ::windows_core::w!("ProductID");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRODUCTLANGUAGE: ::windows_core::PCWSTR = ::windows_core::w!("PRODUCTLANGUAGE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRODUCTNAME: ::windows_core::PCWSTR = ::windows_core::w!("ProductName");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRODUCTSTATE: ::windows_core::PCWSTR = ::windows_core::w!("ProductState");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PRODUCTVERSION: ::windows_core::PCWSTR = ::windows_core::w!("ProductVersion");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PROGRAMFILES64_FOLDER: ::windows_core::PCWSTR = ::windows_core::w!("ProgramFiles64Folder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PROGRAMFILES_FOLDER: ::windows_core::PCWSTR = ::windows_core::w!("ProgramFilesFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PROGRAMMENU_FOLDER: ::windows_core::PCWSTR = ::windows_core::w!("ProgramMenuFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PROGRESSONLY: ::windows_core::PCWSTR = ::windows_core::w!("MsiUIProgressOnly");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_PROMPTROLLBACKCOST: ::windows_core::PCWSTR = ::windows_core::w!("PROMPTROLLBACKCOST");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_REBOOT: ::windows_core::PCWSTR = ::windows_core::w!("REBOOT");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_REBOOTPROMPT: ::windows_core::PCWSTR = ::windows_core::w!("REBOOTPROMPT");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_RECENT_FOLDER: ::windows_core::PCWSTR = ::windows_core::w!("RecentFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_REDIRECTEDDLLSUPPORT: ::windows_core::PCWSTR = ::windows_core::w!("RedirectedDllSupport");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_REINSTALL: ::windows_core::PCWSTR = ::windows_core::w!("REINSTALL");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_REINSTALLMODE: ::windows_core::PCWSTR = ::windows_core::w!("REINSTALLMODE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_REMOTEADMINTS: ::windows_core::PCWSTR = ::windows_core::w!("RemoteAdminTS");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_REPLACEDINUSEFILES: ::windows_core::PCWSTR = ::windows_core::w!("ReplacedInUseFiles");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_RESTRICTEDUSERCONTROL: ::windows_core::PCWSTR = ::windows_core::w!("RestrictedUserControl");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_RESUME: ::windows_core::PCWSTR = ::windows_core::w!("RESUME");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ROLLBACKDISABLED: ::windows_core::PCWSTR = ::windows_core::w!("RollbackDisabled");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_ROOTDRIVE: ::windows_core::PCWSTR = ::windows_core::w!("ROOTDRIVE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_RUNNINGELEVATED: ::windows_core::PCWSTR = ::windows_core::w!("MsiRunningElevated");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SCREENX: ::windows_core::PCWSTR = ::windows_core::w!("ScreenX");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SCREENY: ::windows_core::PCWSTR = ::windows_core::w!("ScreenY");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SENDTO_FOLDER: ::windows_core::PCWSTR = ::windows_core::w!("SendToFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SEQUENCE: ::windows_core::PCWSTR = ::windows_core::w!("SEQUENCE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SERVICEPACKLEVEL: ::windows_core::PCWSTR = ::windows_core::w!("ServicePackLevel");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SERVICEPACKLEVELMINOR: ::windows_core::PCWSTR = ::windows_core::w!("ServicePackLevelMinor");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SHAREDWINDOWS: ::windows_core::PCWSTR = ::windows_core::w!("SharedWindows");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SHELLADVTSUPPORT: ::windows_core::PCWSTR = ::windows_core::w!("ShellAdvtSupport");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SHORTFILENAMES: ::windows_core::PCWSTR = ::windows_core::w!("SHORTFILENAMES");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SOURCEDIR: ::windows_core::PCWSTR = ::windows_core::w!("SourceDir");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SOURCELIST: ::windows_core::PCWSTR = ::windows_core::w!("SOURCELIST");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SOURCERESONLY: ::windows_core::PCWSTR = ::windows_core::w!("MsiUISourceResOnly");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_STARTMENU_FOLDER: ::windows_core::PCWSTR = ::windows_core::w!("StartMenuFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_STARTUP_FOLDER: ::windows_core::PCWSTR = ::windows_core::w!("StartupFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SYSTEM16_FOLDER: ::windows_core::PCWSTR = ::windows_core::w!("System16Folder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SYSTEM64_FOLDER: ::windows_core::PCWSTR = ::windows_core::w!("System64Folder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SYSTEMLANGUAGEID: ::windows_core::PCWSTR = ::windows_core::w!("SystemLanguageID");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_SYSTEM_FOLDER: ::windows_core::PCWSTR = ::windows_core::w!("SystemFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TARGETDIR: ::windows_core::PCWSTR = ::windows_core::w!("TARGETDIR");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TEMPLATE_AMD64: ::windows_core::PCWSTR = ::windows_core::w!("AMD64");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TEMPLATE_FOLDER: ::windows_core::PCWSTR = ::windows_core::w!("TemplateFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TEMPLATE_X64: ::windows_core::PCWSTR = ::windows_core::w!("x64");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TEMP_FOLDER: ::windows_core::PCWSTR = ::windows_core::w!("TempFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TERMSERVER: ::windows_core::PCWSTR = ::windows_core::w!("TerminalServer");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TEXTHEIGHT: ::windows_core::PCWSTR = ::windows_core::w!("TextHeight");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TEXTHEIGHT_CORRECTION: ::windows_core::PCWSTR = ::windows_core::w!("TextHeightCorrection");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TEXTINTERNALLEADING: ::windows_core::PCWSTR = ::windows_core::w!("TextInternalLeading");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TIME: ::windows_core::PCWSTR = ::windows_core::w!("Time");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TRANSFORMS: ::windows_core::PCWSTR = ::windows_core::w!("TRANSFORMS");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TRANSFORMSATSOURCE: ::windows_core::PCWSTR = ::windows_core::w!("TRANSFORMSATSOURCE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TRANSFORMSSECURE: ::windows_core::PCWSTR = ::windows_core::w!("TRANSFORMSSECURE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TRUEADMINUSER: ::windows_core::PCWSTR = ::windows_core::w!("MsiTrueAdminUser");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_TTCSUPPORT: ::windows_core::PCWSTR = ::windows_core::w!("TTCSupport");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_UACONLY: ::windows_core::PCWSTR = ::windows_core::w!("MsiUIUACOnly");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_UPDATESTARTED: ::windows_core::PCWSTR = ::windows_core::w!("UpdateStarted");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_UPGRADECODE: ::windows_core::PCWSTR = ::windows_core::w!("UpgradeCode");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_USERLANGUAGEID: ::windows_core::PCWSTR = ::windows_core::w!("UserLanguageID");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_USERNAME: ::windows_core::PCWSTR = ::windows_core::w!("USERNAME");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_USERSID: ::windows_core::PCWSTR = ::windows_core::w!("UserSID");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_VERSION9X: ::windows_core::PCWSTR = ::windows_core::w!("Version9X");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_VERSIONNT: ::windows_core::PCWSTR = ::windows_core::w!("VersionNT");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_VERSIONNT64: ::windows_core::PCWSTR = ::windows_core::w!("VersionNT64");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_VIRTUALMEMORY: ::windows_core::PCWSTR = ::windows_core::w!("VirtualMemory");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_WIN32ASSEMBLYSUPPORT: ::windows_core::PCWSTR = ::windows_core::w!("MsiWin32AssemblySupport");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_WINDOWSBUILD: ::windows_core::PCWSTR = ::windows_core::w!("WindowsBuild");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_WINDOWS_FOLDER: ::windows_core::PCWSTR = ::windows_core::w!("WindowsFolder");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPNAME_WINDOWS_VOLUME: ::windows_core::PCWSTR = ::windows_core::w!("WindowsVolume");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPVALUE_EXECUTEMODE_NONE: ::windows_core::PCWSTR = ::windows_core::w!("NONE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPVALUE_EXECUTEMODE_SCRIPT: ::windows_core::PCWSTR = ::windows_core::w!("SCRIPT");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPVALUE_FEATURE_ALL: ::windows_core::PCWSTR = ::windows_core::w!("ALL");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPVALUE_MSI_RM_CONTROL_DISABLE: ::windows_core::PCWSTR = ::windows_core::w!("Disable");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPVALUE_MSI_RM_CONTROL_DISABLESHUTDOWN: ::windows_core::PCWSTR = ::windows_core::w!("DisableShutdown");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPVALUE_RBCOST_FAIL: ::windows_core::PCWSTR = ::windows_core::w!("F");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPVALUE_RBCOST_PROMPT: ::windows_core::PCWSTR = ::windows_core::w!("P");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPVALUE_RBCOST_SILENT: ::windows_core::PCWSTR = ::windows_core::w!("D");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPVALUE__CARRYINGNDP_URTREINSTALL: ::windows_core::PCWSTR = ::windows_core::w!("URTREINSTALL");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const IPROPVALUE__CARRYINGNDP_URTUPGRADE: ::windows_core::PCWSTR = ::windows_core::w!("URTUPGRADE");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LIBID_MsmMergeTypeLib: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0adda82f_2c26_11d2_ad65_00a0c9af11a6);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGALL: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGERR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGINFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGNONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGPERFMESSAGES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGTOKEN_NO_LOG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGTOKEN_SETUPAPI_APPLOG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGTOKEN_SETUPAPI_DEVLOG: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGTOKEN_TYPE_MASK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGTOKEN_UNSPECIFIED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const LOGWARN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MAX_FEATURE_CHARS: u32 = 38u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MAX_GUID_CHARS: u32 = 38u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIADVERTISEOPTIONFLAGS_INSTANCE: MSIADVERTISEOPTIONFLAGS = MSIADVERTISEOPTIONFLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIARCHITECTUREFLAGS_AMD64: MSIARCHITECTUREFLAGS = MSIARCHITECTUREFLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIARCHITECTUREFLAGS_ARM: MSIARCHITECTUREFLAGS = MSIARCHITECTUREFLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIARCHITECTUREFLAGS_IA64: MSIARCHITECTUREFLAGS = MSIARCHITECTUREFLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIARCHITECTUREFLAGS_X86: MSIARCHITECTUREFLAGS = MSIARCHITECTUREFLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIASSEMBLYINFO_NETASSEMBLY: MSIASSEMBLYINFO = MSIASSEMBLYINFO(0u32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIASSEMBLYINFO_WIN32ASSEMBLY: MSIASSEMBLYINFO = MSIASSEMBLYINFO(1u32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICODE_PATCH: MSICODE = MSICODE(1073741824i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICODE_PRODUCT: MSICODE = MSICODE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICOLINFO_NAMES: MSICOLINFO = MSICOLINFO(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICOLINFO_TYPES: MSICOLINFO = MSICOLINFO(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICONDITION_ERROR: MSICONDITION = MSICONDITION(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICONDITION_FALSE: MSICONDITION = MSICONDITION(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICONDITION_NONE: MSICONDITION = MSICONDITION(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICONDITION_TRUE: MSICONDITION = MSICONDITION(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICOSTTREE_CHILDREN: MSICOSTTREE = MSICOSTTREE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICOSTTREE_PARENTS: MSICOSTTREE = MSICOSTTREE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICOSTTREE_RESERVED: MSICOSTTREE = MSICOSTTREE(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSICOSTTREE_SELFONLY: MSICOSTTREE = MSICOSTTREE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADCABINET: MSIDBERROR = MSIDBERROR(26i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADCASE: MSIDBERROR = MSIDBERROR(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADCATEGORY: MSIDBERROR = MSIDBERROR(23i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADCONDITION: MSIDBERROR = MSIDBERROR(15i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADCUSTOMSOURCE: MSIDBERROR = MSIDBERROR(20i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADDEFAULTDIR: MSIDBERROR = MSIDBERROR(18i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADFILENAME: MSIDBERROR = MSIDBERROR(13i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADFORMATTED: MSIDBERROR = MSIDBERROR(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADGUID: MSIDBERROR = MSIDBERROR(9i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADIDENTIFIER: MSIDBERROR = MSIDBERROR(11i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADKEYTABLE: MSIDBERROR = MSIDBERROR(24i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADLANGUAGE: MSIDBERROR = MSIDBERROR(12i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADLINK: MSIDBERROR = MSIDBERROR(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADLOCALIZEATTRIB: MSIDBERROR = MSIDBERROR(29i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADMAXMINVALUES: MSIDBERROR = MSIDBERROR(25i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADPATH: MSIDBERROR = MSIDBERROR(14i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADPROPERTY: MSIDBERROR = MSIDBERROR(21i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADREGPATH: MSIDBERROR = MSIDBERROR(19i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADSHORTCUT: MSIDBERROR = MSIDBERROR(27i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADTEMPLATE: MSIDBERROR = MSIDBERROR(17i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADVERSION: MSIDBERROR = MSIDBERROR(7i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_BADWILDCARD: MSIDBERROR = MSIDBERROR(10i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_DUPLICATEKEY: MSIDBERROR = MSIDBERROR(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_FUNCTIONERROR: MSIDBERROR = MSIDBERROR(-1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_INVALIDARG: MSIDBERROR = MSIDBERROR(-3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_MISSINGDATA: MSIDBERROR = MSIDBERROR(22i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_MOREDATA: MSIDBERROR = MSIDBERROR(-2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_NOERROR: MSIDBERROR = MSIDBERROR(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_NOTINSET: MSIDBERROR = MSIDBERROR(6i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_OVERFLOW: MSIDBERROR = MSIDBERROR(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_REQUIRED: MSIDBERROR = MSIDBERROR(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_STRINGOVERFLOW: MSIDBERROR = MSIDBERROR(28i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBERROR_UNDERFLOW: MSIDBERROR = MSIDBERROR(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBOPEN_CREATE: ::windows_core::PCWSTR = ::windows_core::PCWSTR(3i32 as _);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBOPEN_CREATEDIRECT: ::windows_core::PCWSTR = ::windows_core::PCWSTR(4i32 as _);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBOPEN_DIRECT: ::windows_core::PCWSTR = ::windows_core::PCWSTR(2i32 as _);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBOPEN_PATCHFILE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBOPEN_READONLY: ::windows_core::PCWSTR = ::windows_core::PCWSTR(0i32 as _);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBOPEN_TRANSACT: ::windows_core::PCWSTR = ::windows_core::PCWSTR(1i32 as _);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBSTATE_ERROR: MSIDBSTATE = MSIDBSTATE(-1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBSTATE_READ: MSIDBSTATE = MSIDBSTATE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIDBSTATE_WRITE: MSIDBSTATE = MSIDBSTATE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIINSTALLCONTEXT_ALL: MSIINSTALLCONTEXT = MSIINSTALLCONTEXT(7i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIINSTALLCONTEXT_ALLUSERMANAGED: MSIINSTALLCONTEXT = MSIINSTALLCONTEXT(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIINSTALLCONTEXT_FIRSTVISIBLE: MSIINSTALLCONTEXT = MSIINSTALLCONTEXT(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIINSTALLCONTEXT_MACHINE: MSIINSTALLCONTEXT = MSIINSTALLCONTEXT(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIINSTALLCONTEXT_NONE: MSIINSTALLCONTEXT = MSIINSTALLCONTEXT(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIINSTALLCONTEXT_USERMANAGED: MSIINSTALLCONTEXT = MSIINSTALLCONTEXT(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIINSTALLCONTEXT_USERUNMANAGED: MSIINSTALLCONTEXT = MSIINSTALLCONTEXT(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_ASSIGN: MSIMODIFY = MSIMODIFY(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_DELETE: MSIMODIFY = MSIMODIFY(6i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_INSERT: MSIMODIFY = MSIMODIFY(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_INSERT_TEMPORARY: MSIMODIFY = MSIMODIFY(7i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_MERGE: MSIMODIFY = MSIMODIFY(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_REFRESH: MSIMODIFY = MSIMODIFY(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_REPLACE: MSIMODIFY = MSIMODIFY(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_SEEK: MSIMODIFY = MSIMODIFY(-1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_UPDATE: MSIMODIFY = MSIMODIFY(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_VALIDATE: MSIMODIFY = MSIMODIFY(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_VALIDATE_DELETE: MSIMODIFY = MSIMODIFY(11i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_VALIDATE_FIELD: MSIMODIFY = MSIMODIFY(10i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIMODIFY_VALIDATE_NEW: MSIMODIFY = MSIMODIFY(9i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIOPENPACKAGEFLAGS_IGNOREMACHINESTATE: MSIOPENPACKAGEFLAGS = MSIOPENPACKAGEFLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIPATCHSTATE_ALL: MSIPATCHSTATE = MSIPATCHSTATE(15i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIPATCHSTATE_APPLIED: MSIPATCHSTATE = MSIPATCHSTATE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIPATCHSTATE_INVALID: MSIPATCHSTATE = MSIPATCHSTATE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIPATCHSTATE_OBSOLETED: MSIPATCHSTATE = MSIPATCHSTATE(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIPATCHSTATE_REGISTERED: MSIPATCHSTATE = MSIPATCHSTATE(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIPATCHSTATE_SUPERSEDED: MSIPATCHSTATE = MSIPATCHSTATE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIPATCH_DATATYPE_PATCHFILE: MSIPATCHDATATYPE = MSIPATCHDATATYPE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIPATCH_DATATYPE_XMLBLOB: MSIPATCHDATATYPE = MSIPATCHDATATYPE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIPATCH_DATATYPE_XMLPATH: MSIPATCHDATATYPE = MSIPATCHDATATYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_ADMIN: MSIRUNMODE = MSIRUNMODE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_ADVERTISE: MSIRUNMODE = MSIRUNMODE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_CABINET: MSIRUNMODE = MSIRUNMODE(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_COMMIT: MSIRUNMODE = MSIRUNMODE(18i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_LOGENABLED: MSIRUNMODE = MSIRUNMODE(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_MAINTENANCE: MSIRUNMODE = MSIRUNMODE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_OPERATIONS: MSIRUNMODE = MSIRUNMODE(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_REBOOTATEND: MSIRUNMODE = MSIRUNMODE(6i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_REBOOTNOW: MSIRUNMODE = MSIRUNMODE(7i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_RESERVED11: MSIRUNMODE = MSIRUNMODE(11i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_RESERVED14: MSIRUNMODE = MSIRUNMODE(14i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_RESERVED15: MSIRUNMODE = MSIRUNMODE(15i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_ROLLBACK: MSIRUNMODE = MSIRUNMODE(17i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_ROLLBACKENABLED: MSIRUNMODE = MSIRUNMODE(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_SCHEDULED: MSIRUNMODE = MSIRUNMODE(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_SOURCESHORTNAMES: MSIRUNMODE = MSIRUNMODE(9i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_TARGETSHORTNAMES: MSIRUNMODE = MSIRUNMODE(10i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_WINDOWS9X: MSIRUNMODE = MSIRUNMODE(12i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSIRUNMODE_ZAWENABLED: MSIRUNMODE = MSIRUNMODE(13i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSISOURCETYPE_MEDIA: MSISOURCETYPE = MSISOURCETYPE(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSISOURCETYPE_NETWORK: MSISOURCETYPE = MSISOURCETYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSISOURCETYPE_UNKNOWN: MSISOURCETYPE = MSISOURCETYPE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSISOURCETYPE_URL: MSISOURCETYPE = MSISOURCETYPE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSACTIONSTATE_COMMIT: MSITRANSACTIONSTATE = MSITRANSACTIONSTATE(1u32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSACTIONSTATE_ROLLBACK: MSITRANSACTIONSTATE = MSITRANSACTIONSTATE(0u32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSACTION_CHAIN_EMBEDDEDUI: MSITRANSACTION = MSITRANSACTION(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSACTION_JOIN_EXISTING_EMBEDDEDUI: MSITRANSACTION = MSITRANSACTION(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_ERROR_ADDEXISTINGROW: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_ERROR_ADDEXISTINGTABLE: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_ERROR_CHANGECODEPAGE: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_ERROR_DELMISSINGROW: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_ERROR_DELMISSINGTABLE: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_ERROR_NONE: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_ERROR_UPDATEMISSINGROW: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_ERROR_VIEWTRANSFORM: MSITRANSFORM_ERROR = MSITRANSFORM_ERROR(256i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_LANGUAGE: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_MAJORVERSION: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_MINORVERSION: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_NEWEQUALBASEVERSION: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(256i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_NEWGREATERBASEVERSION: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(1024i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_NEWGREATEREQUALBASEVERSION: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(512i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_NEWLESSBASEVERSION: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(64i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_NEWLESSEQUALBASEVERSION: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(128i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_PLATFORM: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_PRODUCT: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_UPDATEVERSION: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSITRANSFORM_VALIDATE_UPGRADECODE: MSITRANSFORM_VALIDATE = MSITRANSFORM_VALIDATE(2048i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSI_INVALID_HASH_IS_FATAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MSI_NULL_INTEGER: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const MsmMerge: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0adda830_2c26_11d2_ad65_00a0c9af11a6);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PACKMAN_RUNTIME_INVALID: PACKMAN_RUNTIME = PACKMAN_RUNTIME(6i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PACKMAN_RUNTIME_JUPITER: PACKMAN_RUNTIME = PACKMAN_RUNTIME(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PACKMAN_RUNTIME_MODERN_NATIVE: PACKMAN_RUNTIME = PACKMAN_RUNTIME(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PACKMAN_RUNTIME_NATIVE: PACKMAN_RUNTIME = PACKMAN_RUNTIME(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PACKMAN_RUNTIME_SILVERLIGHTMOBILE: PACKMAN_RUNTIME = PACKMAN_RUNTIME(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PACKMAN_RUNTIME_XNA: PACKMAN_RUNTIME = PACKMAN_RUNTIME(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_FAIL_IF_BIGGER: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_FAIL_IF_SAME_FILE: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_INTERLEAVE_FILES: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_NO_BINDFIX: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_NO_CHECKSUM: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_NO_LOCKFIX: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_NO_REBASE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_NO_RESTIMEFIX: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_NO_TIMESTAMP: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_RESERVED1: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_SIGNATURE_MD5: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_USE_BEST: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_USE_LZX_A: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_USE_LZX_B: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_USE_LZX_BEST: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_USE_LZX_LARGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_OPTION_VALID_FLAGS: u32 = 3237937159u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_SYMBOL_NO_FAILURES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_SYMBOL_NO_IMAGEHLP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_SYMBOL_RESERVED1: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_SYMBOL_UNDECORATED_TOO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_TRANSFORM_PE_IRELOC_2: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PATCH_TRANSFORM_PE_RESOURCE_2: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_APPNAME: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_AUTHOR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_CHARCOUNT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_COMMENTS: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_CREATE_DTM: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_EDITTIME: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_KEYWORDS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_LASTAUTHOR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_LASTPRINTED: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_LASTSAVE_DTM: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_MSIRESTRICT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_MSISOURCE: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_MSIVERSION: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_PAGECOUNT: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_REVNUMBER: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_SUBJECT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_TEMPLATE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_THUMBNAIL: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_TITLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PID_WORDCOUNT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PMSvc: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9e511fc_e364_497a_a121_b7b3612cedce);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ACTIVATION_POLICY_INVALID: PM_ACTIVATION_POLICY = PM_ACTIVATION_POLICY(7i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ACTIVATION_POLICY_MULTISESSION: PM_ACTIVATION_POLICY = PM_ACTIVATION_POLICY(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ACTIVATION_POLICY_REPLACE: PM_ACTIVATION_POLICY = PM_ACTIVATION_POLICY(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ACTIVATION_POLICY_REPLACESAMEPARAMS: PM_ACTIVATION_POLICY = PM_ACTIVATION_POLICY(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ACTIVATION_POLICY_REPLACE_IGNOREFOREGROUND: PM_ACTIVATION_POLICY = PM_ACTIVATION_POLICY(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ACTIVATION_POLICY_RESUME: PM_ACTIVATION_POLICY = PM_ACTIVATION_POLICY(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ACTIVATION_POLICY_RESUMESAMEPARAMS: PM_ACTIVATION_POLICY = PM_ACTIVATION_POLICY(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ACTIVATION_POLICY_UNKNOWN: PM_ACTIVATION_POLICY = PM_ACTIVATION_POLICY(6i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_HUBTYPE_INVALID: PM_APPLICATION_HUBTYPE = PM_APPLICATION_HUBTYPE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_HUBTYPE_MUSIC: PM_APPLICATION_HUBTYPE = PM_APPLICATION_HUBTYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_HUBTYPE_NONMUSIC: PM_APPLICATION_HUBTYPE = PM_APPLICATION_HUBTYPE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_INSTALL_DEBUG: PM_APPLICATION_INSTALL_TYPE = PM_APPLICATION_INSTALL_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_INSTALL_ENTERPRISE: PM_APPLICATION_INSTALL_TYPE = PM_APPLICATION_INSTALL_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_INSTALL_INVALID: PM_APPLICATION_INSTALL_TYPE = PM_APPLICATION_INSTALL_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_INSTALL_IN_ROM: PM_APPLICATION_INSTALL_TYPE = PM_APPLICATION_INSTALL_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_INSTALL_NORMAL: PM_APPLICATION_INSTALL_TYPE = PM_APPLICATION_INSTALL_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_INSTALL_PA: PM_APPLICATION_INSTALL_TYPE = PM_APPLICATION_INSTALL_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_DISABLED_BACKING_UP: PM_APPLICATION_STATE = PM_APPLICATION_STATE(9i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_DISABLED_ENTERPRISE: PM_APPLICATION_STATE = PM_APPLICATION_STATE(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_DISABLED_MDIL_BINDING: PM_APPLICATION_STATE = PM_APPLICATION_STATE(10i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_DISABLED_SD_CARD: PM_APPLICATION_STATE = PM_APPLICATION_STATE(7i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_INSTALLED: PM_APPLICATION_STATE = PM_APPLICATION_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_INSTALLING: PM_APPLICATION_STATE = PM_APPLICATION_STATE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_INVALID: PM_APPLICATION_STATE = PM_APPLICATION_STATE(11i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_LICENSE_UPDATING: PM_APPLICATION_STATE = PM_APPLICATION_STATE(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_MAX: PM_APPLICATION_STATE = PM_APPLICATION_STATE(10i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_MIN: PM_APPLICATION_STATE = PM_APPLICATION_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_MOVING: PM_APPLICATION_STATE = PM_APPLICATION_STATE(6i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_UNINSTALLING: PM_APPLICATION_STATE = PM_APPLICATION_STATE(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APPLICATION_STATE_UPDATING: PM_APPLICATION_STATE = PM_APPLICATION_STATE(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_FILTER_ALL: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_FILTER_ALL_INCLUDE_MODERN: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(6i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_FILTER_FRAMEWORK: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(7i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_FILTER_GENRE: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_FILTER_HUBTYPE: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_FILTER_MAX: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_FILTER_NONGAMES: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_FILTER_PINABLEONKIDZONE: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_FILTER_VISIBLE: PM_ENUM_APP_FILTER = PM_ENUM_APP_FILTER(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_GENRE_GAMES: PM_APP_GENRE = PM_APP_GENRE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_GENRE_INVALID: PM_APP_GENRE = PM_APP_GENRE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_APP_GENRE_OTHER: PM_APP_GENRE = PM_APP_GENRE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_BSA_FILTER_ALL: PM_ENUM_BSA_FILTER = PM_ENUM_BSA_FILTER(26i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_BSA_FILTER_BY_ALL_LAUNCHONBOOT: PM_ENUM_BSA_FILTER = PM_ENUM_BSA_FILTER(30i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_BSA_FILTER_BY_PERIODIC: PM_ENUM_BSA_FILTER = PM_ENUM_BSA_FILTER(29i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_BSA_FILTER_BY_PRODUCTID: PM_ENUM_BSA_FILTER = PM_ENUM_BSA_FILTER(28i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_BSA_FILTER_BY_TASKID: PM_ENUM_BSA_FILTER = PM_ENUM_BSA_FILTER(27i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_BSA_FILTER_MAX: PM_ENUM_BSA_FILTER = PM_ENUM_BSA_FILTER(31i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_BW_FILTER_BOOTWORKER_ALL: PM_ENUM_BW_FILTER = PM_ENUM_BW_FILTER(31i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_BW_FILTER_BY_TASKID: PM_ENUM_BW_FILTER = PM_ENUM_BW_FILTER(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_BW_FILTER_MAX: PM_ENUM_BW_FILTER = PM_ENUM_BW_FILTER(33i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_APPCONNECT: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(17i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_BY_CONSUMER: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(17i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_CACHEDFILEUPDATER_ALL: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(25i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_FILEOPENPICKER_ALL: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(23i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_FILESAVEPICKER_ALL: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(24i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_FTASSOC_APPLICATION_ALL: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(21i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_FTASSOC_CONTENTTYPE_ALL: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(20i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_FTASSOC_FILETYPE_ALL: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(19i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_MAX: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(26i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_PROTOCOL_ALL: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(18i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_ENUM_EXTENSION_FILTER_SHARETARGET_ALL: PM_ENUM_EXTENSION_FILTER = PM_ENUM_EXTENSION_FILTER(22i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_LIVETILE_RECURRENCE_TYPE_INSTANT: PM_LIVETILE_RECURRENCE_TYPE = PM_LIVETILE_RECURRENCE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_LIVETILE_RECURRENCE_TYPE_INTERVAL: PM_LIVETILE_RECURRENCE_TYPE = PM_LIVETILE_RECURRENCE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_LIVETILE_RECURRENCE_TYPE_MAX: PM_LIVETILE_RECURRENCE_TYPE = PM_LIVETILE_RECURRENCE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_LIVETILE_RECURRENCE_TYPE_ONETIME: PM_LIVETILE_RECURRENCE_TYPE = PM_LIVETILE_RECURRENCE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_LOGO_SIZE_INVALID: PM_LOGO_SIZE = PM_LOGO_SIZE(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_LOGO_SIZE_LARGE: PM_LOGO_SIZE = PM_LOGO_SIZE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_LOGO_SIZE_MEDIUM: PM_LOGO_SIZE = PM_LOGO_SIZE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_LOGO_SIZE_SMALL: PM_LOGO_SIZE = PM_LOGO_SIZE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_STARTTILE_TYPE_APPLIST: PM_STARTTILE_TYPE = PM_STARTTILE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_STARTTILE_TYPE_APPLISTPRIMARY: PM_STARTTILE_TYPE = PM_STARTTILE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_STARTTILE_TYPE_INVALID: PM_STARTTILE_TYPE = PM_STARTTILE_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_STARTTILE_TYPE_PRIMARY: PM_STARTTILE_TYPE = PM_STARTTILE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_STARTTILE_TYPE_SECONDARY: PM_STARTTILE_TYPE = PM_STARTTILE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_FILTER_APP_ALL: PM_ENUM_TASK_FILTER = PM_ENUM_TASK_FILTER(12i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_FILTER_APP_TASK_TYPE: PM_ENUM_TASK_FILTER = PM_ENUM_TASK_FILTER(15i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_FILTER_BGEXECUTION: PM_ENUM_TASK_FILTER = PM_ENUM_TASK_FILTER(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_FILTER_DEHYD_SUPRESSING: PM_ENUM_TASK_FILTER = PM_ENUM_TASK_FILTER(14i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_FILTER_MAX: PM_ENUM_TASK_FILTER = PM_ENUM_TASK_FILTER(17i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_FILTER_TASK_TYPE: PM_ENUM_TASK_FILTER = PM_ENUM_TASK_FILTER(13i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TRANSITION_CUSTOM: PM_TASK_TRANSITION = PM_TASK_TRANSITION(6i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TRANSITION_DEFAULT: PM_TASK_TRANSITION = PM_TASK_TRANSITION(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TRANSITION_INVALID: PM_TASK_TRANSITION = PM_TASK_TRANSITION(7i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TRANSITION_NONE: PM_TASK_TRANSITION = PM_TASK_TRANSITION(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TRANSITION_READERBOARD: PM_TASK_TRANSITION = PM_TASK_TRANSITION(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TRANSITION_SLIDE: PM_TASK_TRANSITION = PM_TASK_TRANSITION(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TRANSITION_SWIVEL: PM_TASK_TRANSITION = PM_TASK_TRANSITION(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TRANSITION_TURNSTILE: PM_TASK_TRANSITION = PM_TASK_TRANSITION(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TYPE_BACKGROUNDSERVICEAGENT: PM_TASK_TYPE = PM_TASK_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TYPE_BACKGROUNDWORKER: PM_TASK_TYPE = PM_TASK_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TYPE_DEFAULT: PM_TASK_TYPE = PM_TASK_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TYPE_INVALID: PM_TASK_TYPE = PM_TASK_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TYPE_NORMAL: PM_TASK_TYPE = PM_TASK_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TASK_TYPE_SETTINGS: PM_TASK_TYPE = PM_TASK_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_FILTER_APPLIST: PM_ENUM_TILE_FILTER = PM_ENUM_TILE_FILTER(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_FILTER_APP_ALL: PM_ENUM_TILE_FILTER = PM_ENUM_TILE_FILTER(11i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_FILTER_HUBTYPE: PM_ENUM_TILE_FILTER = PM_ENUM_TILE_FILTER(10i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_FILTER_MAX: PM_ENUM_TILE_FILTER = PM_ENUM_TILE_FILTER(12i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_FILTER_PINNED: PM_ENUM_TILE_FILTER = PM_ENUM_TILE_FILTER(9i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_HUBTYPE_APPLIST: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(1073741824i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_HUBTYPE_CACHED: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(67108864i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_HUBTYPE_GAMES: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(536870912i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_HUBTYPE_INVALID: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(67108865i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_HUBTYPE_KIDZONE: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(33554432i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_HUBTYPE_LOCKSCREEN: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(16777216i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_HUBTYPE_MOSETTINGS: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(268435456i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_HUBTYPE_MUSIC: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_HUBTYPE_STARTMENU: PM_TILE_HUBTYPE = PM_TILE_HUBTYPE(-2147483648i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_SIZE_INVALID: PM_TILE_SIZE = PM_TILE_SIZE(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_SIZE_LARGE: PM_TILE_SIZE = PM_TILE_SIZE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_SIZE_MEDIUM: PM_TILE_SIZE = PM_TILE_SIZE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_SIZE_SMALL: PM_TILE_SIZE = PM_TILE_SIZE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_SIZE_SQUARE310X310: PM_TILE_SIZE = PM_TILE_SIZE(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const PM_TILE_SIZE_TALL150X310: PM_TILE_SIZE = PM_TILE_SIZE(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const QUERYASMINFO_FLAG_VALIDATE: QUERYASMINFO_FLAGS = QUERYASMINFO_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_FILEEQUALVERSION: REINSTALLMODE = REINSTALLMODE(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_FILEEXACT: REINSTALLMODE = REINSTALLMODE(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_FILEMISSING: REINSTALLMODE = REINSTALLMODE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_FILEOLDERVERSION: REINSTALLMODE = REINSTALLMODE(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_FILEREPLACE: REINSTALLMODE = REINSTALLMODE(64i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_FILEVERIFY: REINSTALLMODE = REINSTALLMODE(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_MACHINEDATA: REINSTALLMODE = REINSTALLMODE(128i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_PACKAGE: REINSTALLMODE = REINSTALLMODE(1024i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_REPAIR: REINSTALLMODE = REINSTALLMODE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_SHORTCUT: REINSTALLMODE = REINSTALLMODE(512i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const REINSTALLMODE_USERDATA: REINSTALLMODE = REINSTALLMODE(256i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SCRIPTFLAGS_CACHEINFO: SCRIPTFLAGS = SCRIPTFLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SCRIPTFLAGS_MACHINEASSIGN: SCRIPTFLAGS = SCRIPTFLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SCRIPTFLAGS_REGDATA: SCRIPTFLAGS = SCRIPTFLAGS(416i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SCRIPTFLAGS_REGDATA_APPINFO: SCRIPTFLAGS = SCRIPTFLAGS(384i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SCRIPTFLAGS_REGDATA_CLASSINFO: SCRIPTFLAGS = SCRIPTFLAGS(128i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SCRIPTFLAGS_REGDATA_CNFGINFO: SCRIPTFLAGS = SCRIPTFLAGS(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SCRIPTFLAGS_REGDATA_EXTENSIONINFO: SCRIPTFLAGS = SCRIPTFLAGS(256i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SCRIPTFLAGS_SHORTCUTS: SCRIPTFLAGS = SCRIPTFLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SCRIPTFLAGS_VALIDATE_TRANSFORMS_LIST: SCRIPTFLAGS = SCRIPTFLAGS(64i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_DISABLE_ASK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_DISABLE_NOPOPUPS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_DISABLE_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_DISABLE_ONCE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_DISABLE_SETUP: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_IDLE_TRIGGER: ::windows_core::PCWSTR = ::windows_core::w!("WFP_IDLE_TRIGGER");
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_QUOTA_DEFAULT: u32 = 50u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_SCAN_ALWAYS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_SCAN_IMMEDIATE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_SCAN_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const SFC_SCAN_ONCE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const STREAM_FORMAT_COMPLIB_MANIFEST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const STREAM_FORMAT_COMPLIB_MODULE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const STREAM_FORMAT_WIN32_MANIFEST: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const STREAM_FORMAT_WIN32_MODULE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_AGILESTORE: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_ALL: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(100i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_BADGE: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_BLOCK: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(17i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_BLOCKANDTEXT01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(33i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_BLOCKANDTEXT02: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(34i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_CALENDAR: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_CONTACT: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(11i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_CYCLE: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(14i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_DEEPLINK: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(13i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_DEFAULT: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(15i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_FLIP: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_FOLDER: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(59i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_GAMES: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_GROUP: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(12i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_IMAGE: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(29i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_IMAGEANDTEXT01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(31i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_IMAGEANDTEXT02: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_IMAGECOLLECTION: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(30i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_INVALID: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_METROCOUNT: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_METROCOUNTQUEUE: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(56i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_MUSICVIDEO: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(7i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGE01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(39i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGE02: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(40i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGE03: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(41i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGE04: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(42i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGE05: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(43i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGE06: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(44i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(35i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT02: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(36i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT03: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(37i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT04: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(38i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(45i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION02: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(46i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION03: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(47i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION04: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(48i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION05: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(49i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION06: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(50i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_PEOPLE: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(10i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_SEARCH: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(57i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(51i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT02: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(52i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT03: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(53i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT04: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(54i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT05: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(55i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(18i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT02: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(19i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT03: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(20i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT04: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(21i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT05: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(22i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT06: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(23i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT07: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(24i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT08: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(25i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT09: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(26i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT10: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(27i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TEXT11: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(28i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TILE_TEMPLATE_TILEFLYOUT01: TILE_TEMPLATE_TYPE = TILE_TEMPLATE_TYPE(58i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_BACKUP: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_CMI: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_COPYFILES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_DEPTH_DECR: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_DEPTH_INCR: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_DETAILS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_DEVINST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_DEVMGR: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_DRIVER_STORE: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_DRVSETUP: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_ERROR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_FILEQ: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_FLUSH_FILE: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_INF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_INFDB: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_INSTALLER: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_NEWDEV: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_POLICY: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_RESERVED_FLAGS: u32 = 65520u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_SETUP: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_SETUPAPI_BITS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_SETUPAPI_CMDLINE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_SETUPAPI_DEVLOG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_SIGVERIF: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_SUMMARY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_SYSTEM_STATE_CHANGE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_TAB_1: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_TIMESTAMP: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_UI: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_UMPNPMGR: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_UTIL: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_VENDOR: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_VERBOSE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_VERY_VERBOSE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const TXTLOG_WARNING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const UIALL: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const UILOGBITS: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const UINONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const USERINFOSTATE_ABSENT: USERINFOSTATE = USERINFOSTATE(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const USERINFOSTATE_INVALIDARG: USERINFOSTATE = USERINFOSTATE(-2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const USERINFOSTATE_MOREDATA: USERINFOSTATE = USERINFOSTATE(-3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const USERINFOSTATE_PRESENT: USERINFOSTATE = USERINFOSTATE(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const USERINFOSTATE_UNKNOWN: USERINFOSTATE = USERINFOSTATE(-1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_BAD_MAJOR_VERSION: u32 = 3222294792u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_BASE: u32 = 3222294785u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_EQUAL_FILE_VERSION: u32 = 3222294794u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_FILE_VERSION_DOWNREV: u32 = 3222294793u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_IMPROPER_TRANSFORM_VALIDATION: u32 = 3222294788u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_INVALID_TRANSFORM_VALIDATION: u32 = 3222294791u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_MAJOR_UPGRADE_PATCH: u32 = 3222294785u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_OBSOLETION_WITH_MSI30: u32 = 3222294801u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_OBSOLETION_WITH_PATCHSEQUENCE: u32 = 3222294803u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_OBSOLETION_WITH_SEQUENCE_DATA: u32 = 3222294802u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_PATCHPROPERTYNOTSET: u32 = 3222294795u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_PCW_MISMATCHED_PRODUCT_CODES: u32 = 3222294789u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_PCW_MISMATCHED_PRODUCT_VERSIONS: u32 = 3222294790u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_SEQUENCE_DATA_GENERATION_DISABLED: u32 = 3222294786u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const WARN_SEQUENCE_DATA_SUPERSEDENCE_IGNORED: u32 = 3222294787u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const _WIN32_MSI: u32 = 500u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const _WIN32_MSM: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const cchMaxInteger: i32 = 12i32;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieError: RESULTTYPES = RESULTTYPES(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieInfo: RESULTTYPES = RESULTTYPES(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusCancel: STATUSTYPES = STATUSTYPES(10i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusCreateEngine: STATUSTYPES = STATUSTYPES(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusFail: STATUSTYPES = STATUSTYPES(9i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusGetCUB: STATUSTYPES = STATUSTYPES(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusICECount: STATUSTYPES = STATUSTYPES(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusMerge: STATUSTYPES = STATUSTYPES(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusRunICE: STATUSTYPES = STATUSTYPES(6i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusShutdown: STATUSTYPES = STATUSTYPES(7i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusStarting: STATUSTYPES = STATUSTYPES(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusSuccess: STATUSTYPES = STATUSTYPES(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieStatusSummaryInfo: STATUSTYPES = STATUSTYPES(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieUnknown: RESULTTYPES = RESULTTYPES(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const ieWarning: RESULTTYPES = RESULTTYPES(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbAssemblyAttributesURT: msidbAssemblyAttributes = msidbAssemblyAttributes(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbAssemblyAttributesWin32: msidbAssemblyAttributes = msidbAssemblyAttributes(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbClassAttributesRelativePath: msidbClassAttributes = msidbClassAttributes(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributes64bit: msidbComponentAttributes = msidbComponentAttributes(256i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesDisableRegistryReflection: msidbComponentAttributes = msidbComponentAttributes(512i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesLocalOnly: msidbComponentAttributes = msidbComponentAttributes(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesNeverOverwrite: msidbComponentAttributes = msidbComponentAttributes(128i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesODBCDataSource: msidbComponentAttributes = msidbComponentAttributes(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesOptional: msidbComponentAttributes = msidbComponentAttributes(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesPermanent: msidbComponentAttributes = msidbComponentAttributes(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesRegistryKeyPath: msidbComponentAttributes = msidbComponentAttributes(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesShared: msidbComponentAttributes = msidbComponentAttributes(2048i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesSharedDllRefCount: msidbComponentAttributes = msidbComponentAttributes(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesSourceOnly: msidbComponentAttributes = msidbComponentAttributes(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesTransitive: msidbComponentAttributes = msidbComponentAttributes(64i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbComponentAttributesUninstallOnSupersedence: msidbComponentAttributes = msidbComponentAttributes(1024i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesBiDi: msidbControlAttributes = msidbControlAttributes(224i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesBitmap: msidbControlAttributes = msidbControlAttributes(262144i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesCDROMVolume: msidbControlAttributes = msidbControlAttributes(524288i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesComboList: msidbControlAttributes = msidbControlAttributes(131072i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesElevationShield: msidbControlAttributes = msidbControlAttributes(8388608i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesEnabled: msidbControlAttributes = msidbControlAttributes(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesFixedSize: msidbControlAttributes = msidbControlAttributes(1048576i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesFixedVolume: msidbControlAttributes = msidbControlAttributes(131072i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesFloppyVolume: msidbControlAttributes = msidbControlAttributes(2097152i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesFormatSize: msidbControlAttributes = msidbControlAttributes(524288i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesHasBorder: msidbControlAttributes = msidbControlAttributes(16777216i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesIcon: msidbControlAttributes = msidbControlAttributes(524288i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesIconSize16: msidbControlAttributes = msidbControlAttributes(2097152i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesIconSize32: msidbControlAttributes = msidbControlAttributes(4194304i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesIconSize48: msidbControlAttributes = msidbControlAttributes(6291456i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesImageHandle: msidbControlAttributes = msidbControlAttributes(65536i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesIndirect: msidbControlAttributes = msidbControlAttributes(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesInteger: msidbControlAttributes = msidbControlAttributes(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesLeftScroll: msidbControlAttributes = msidbControlAttributes(128i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesMultiline: msidbControlAttributes = msidbControlAttributes(65536i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesNoPrefix: msidbControlAttributes = msidbControlAttributes(131072i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesNoWrap: msidbControlAttributes = msidbControlAttributes(262144i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesPasswordInput: msidbControlAttributes = msidbControlAttributes(2097152i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesProgress95: msidbControlAttributes = msidbControlAttributes(65536i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesPushLike: msidbControlAttributes = msidbControlAttributes(131072i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesRAMDiskVolume: msidbControlAttributes = msidbControlAttributes(1048576i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesRTLRO: msidbControlAttributes = msidbControlAttributes(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesRemoteVolume: msidbControlAttributes = msidbControlAttributes(262144i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesRemovableVolume: msidbControlAttributes = msidbControlAttributes(65536i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesRightAligned: msidbControlAttributes = msidbControlAttributes(64i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesSorted: msidbControlAttributes = msidbControlAttributes(65536i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesSunken: msidbControlAttributes = msidbControlAttributes(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesTransparent: msidbControlAttributes = msidbControlAttributes(65536i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesUsersLanguage: msidbControlAttributes = msidbControlAttributes(1048576i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlAttributesVisible: msidbControlAttributes = msidbControlAttributes(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbControlShowRollbackCost: msidbControlAttributes = msidbControlAttributes(4194304i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionType64BitScript: msidbCustomActionType = msidbCustomActionType(4096i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeAsync: msidbCustomActionType = msidbCustomActionType(128i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeBinaryData: msidbCustomActionType = msidbCustomActionType(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeClientRepeat: msidbCustomActionType = msidbCustomActionType(768i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeCommit: msidbCustomActionType = msidbCustomActionType(512i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeContinue: msidbCustomActionType = msidbCustomActionType(64i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeDirectory: msidbCustomActionType = msidbCustomActionType(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeDll: msidbCustomActionType = msidbCustomActionType(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeExe: msidbCustomActionType = msidbCustomActionType(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeFirstSequence: msidbCustomActionType = msidbCustomActionType(256i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeHideTarget: msidbCustomActionType = msidbCustomActionType(8192i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeInScript: msidbCustomActionType = msidbCustomActionType(1024i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeInstall: msidbCustomActionType = msidbCustomActionType(7i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeJScript: msidbCustomActionType = msidbCustomActionType(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeNoImpersonate: msidbCustomActionType = msidbCustomActionType(2048i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeOncePerProcess: msidbCustomActionType = msidbCustomActionType(512i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypePatchUninstall: msidbCustomActionType = msidbCustomActionType(32768i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeProperty: msidbCustomActionType = msidbCustomActionType(48i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeRollback: msidbCustomActionType = msidbCustomActionType(256i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeSourceFile: msidbCustomActionType = msidbCustomActionType(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeTSAware: msidbCustomActionType = msidbCustomActionType(16384i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeTextData: msidbCustomActionType = msidbCustomActionType(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbCustomActionTypeVBScript: msidbCustomActionType = msidbCustomActionType(6i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesBiDi: msidbDialogAttributes = msidbDialogAttributes(896i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesError: msidbDialogAttributes = msidbDialogAttributes(65536i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesKeepModeless: msidbDialogAttributes = msidbDialogAttributes(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesLeftScroll: msidbDialogAttributes = msidbDialogAttributes(512i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesMinimize: msidbDialogAttributes = msidbDialogAttributes(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesModal: msidbDialogAttributes = msidbDialogAttributes(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesRTLRO: msidbDialogAttributes = msidbDialogAttributes(128i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesRightAligned: msidbDialogAttributes = msidbDialogAttributes(256i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesSysModal: msidbDialogAttributes = msidbDialogAttributes(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesTrackDiskSpace: msidbDialogAttributes = msidbDialogAttributes(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesUseCustomPalette: msidbDialogAttributes = msidbDialogAttributes(64i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbDialogAttributesVisible: msidbDialogAttributes = msidbDialogAttributes(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbEmbeddedHandlesBasic: msidbEmbeddedUIAttributes = msidbEmbeddedUIAttributes(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbEmbeddedUI: msidbEmbeddedUIAttributes = msidbEmbeddedUIAttributes(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFeatureAttributesDisallowAdvertise: msidbFeatureAttributes = msidbFeatureAttributes(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFeatureAttributesFavorAdvertise: msidbFeatureAttributes = msidbFeatureAttributes(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFeatureAttributesFavorLocal: msidbFeatureAttributes = msidbFeatureAttributes(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFeatureAttributesFavorSource: msidbFeatureAttributes = msidbFeatureAttributes(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFeatureAttributesFollowParent: msidbFeatureAttributes = msidbFeatureAttributes(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFeatureAttributesNoUnsupportedAdvertise: msidbFeatureAttributes = msidbFeatureAttributes(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFeatureAttributesUIDisallowAbsent: msidbFeatureAttributes = msidbFeatureAttributes(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesChecksum: msidbFileAttributes = msidbFileAttributes(1024i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesCompressed: msidbFileAttributes = msidbFileAttributes(16384i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesHidden: msidbFileAttributes = msidbFileAttributes(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesIsolatedComp: msidbFileAttributes = msidbFileAttributes(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesNoncompressed: msidbFileAttributes = msidbFileAttributes(8192i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesPatchAdded: msidbFileAttributes = msidbFileAttributes(4096i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesReadOnly: msidbFileAttributes = msidbFileAttributes(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesReserved0: msidbFileAttributes = msidbFileAttributes(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesReserved1: msidbFileAttributes = msidbFileAttributes(64i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesReserved2: msidbFileAttributes = msidbFileAttributes(128i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesReserved3: msidbFileAttributes = msidbFileAttributes(256i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesReserved4: msidbFileAttributes = msidbFileAttributes(32768i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesSystem: msidbFileAttributes = msidbFileAttributes(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbFileAttributesVital: msidbFileAttributes = msidbFileAttributes(512i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbIniFileActionAddLine: msidbIniFileAction = msidbIniFileAction(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbIniFileActionAddTag: msidbIniFileAction = msidbIniFileAction(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbIniFileActionCreateLine: msidbIniFileAction = msidbIniFileAction(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbIniFileActionRemoveLine: msidbIniFileAction = msidbIniFileAction(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbIniFileActionRemoveTag: msidbIniFileAction = msidbIniFileAction(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbLocatorType64bit: msidbLocatorType = msidbLocatorType(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbLocatorTypeDirectory: msidbLocatorType = msidbLocatorType(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbLocatorTypeFileName: msidbLocatorType = msidbLocatorType(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbLocatorTypeRawValue: msidbLocatorType = msidbLocatorType(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbMoveFileOptionsMove: msidbMoveFileOptions = msidbMoveFileOptions(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbODBCDataSourceRegistrationPerMachine: msidbODBCDataSourceRegistration = msidbODBCDataSourceRegistration(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbODBCDataSourceRegistrationPerUser: msidbODBCDataSourceRegistration = msidbODBCDataSourceRegistration(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbPatchAttributesNonVital: msidbPatchAttributes = msidbPatchAttributes(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbRegistryRootClassesRoot: msidbRegistryRoot = msidbRegistryRoot(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbRegistryRootCurrentUser: msidbRegistryRoot = msidbRegistryRoot(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbRegistryRootLocalMachine: msidbRegistryRoot = msidbRegistryRoot(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbRegistryRootUsers: msidbRegistryRoot = msidbRegistryRoot(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbRemoveFileInstallModeOnBoth: msidbRemoveFileInstallMode = msidbRemoveFileInstallMode(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbRemoveFileInstallModeOnInstall: msidbRemoveFileInstallMode = msidbRemoveFileInstallMode(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbRemoveFileInstallModeOnRemove: msidbRemoveFileInstallMode = msidbRemoveFileInstallMode(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbServiceConfigEventInstall: msidbServiceConfigEvent = msidbServiceConfigEvent(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbServiceConfigEventReinstall: msidbServiceConfigEvent = msidbServiceConfigEvent(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbServiceConfigEventUninstall: msidbServiceConfigEvent = msidbServiceConfigEvent(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbServiceControlEventDelete: msidbServiceControlEvent = msidbServiceControlEvent(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbServiceControlEventStart: msidbServiceControlEvent = msidbServiceControlEvent(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbServiceControlEventStop: msidbServiceControlEvent = msidbServiceControlEvent(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbServiceControlEventUninstallDelete: msidbServiceControlEvent = msidbServiceControlEvent(128i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbServiceControlEventUninstallStart: msidbServiceControlEvent = msidbServiceControlEvent(16i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbServiceControlEventUninstallStop: msidbServiceControlEvent = msidbServiceControlEvent(32i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbServiceInstallErrorControlVital: msidbServiceInstallErrorControl = msidbServiceInstallErrorControl(32768i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbSumInfoSourceTypeAdminImage: msidbSumInfoSourceType = msidbSumInfoSourceType(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbSumInfoSourceTypeCompressed: msidbSumInfoSourceType = msidbSumInfoSourceType(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbSumInfoSourceTypeLUAPackage: msidbSumInfoSourceType = msidbSumInfoSourceType(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbSumInfoSourceTypeSFN: msidbSumInfoSourceType = msidbSumInfoSourceType(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbTextStyleStyleBitsBold: msidbTextStyleStyleBits = msidbTextStyleStyleBits(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbTextStyleStyleBitsItalic: msidbTextStyleStyleBits = msidbTextStyleStyleBits(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbTextStyleStyleBitsStrike: msidbTextStyleStyleBits = msidbTextStyleStyleBits(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbTextStyleStyleBitsUnderline: msidbTextStyleStyleBits = msidbTextStyleStyleBits(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbUpgradeAttributesIgnoreRemoveFailure: msidbUpgradeAttributes = msidbUpgradeAttributes(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbUpgradeAttributesLanguagesExclusive: msidbUpgradeAttributes = msidbUpgradeAttributes(1024i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbUpgradeAttributesMigrateFeatures: msidbUpgradeAttributes = msidbUpgradeAttributes(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbUpgradeAttributesOnlyDetect: msidbUpgradeAttributes = msidbUpgradeAttributes(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbUpgradeAttributesVersionMaxInclusive: msidbUpgradeAttributes = msidbUpgradeAttributes(512i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msidbUpgradeAttributesVersionMinInclusive: msidbUpgradeAttributes = msidbUpgradeAttributes(256i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msifiFastInstallLessPrgMsg: msifiFastInstallBits = msifiFastInstallBits(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msifiFastInstallNoSR: msifiFastInstallBits = msifiFastInstallBits(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msifiFastInstallQuickCosting: msifiFastInstallBits = msifiFastInstallBits(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msirbRebootCustomActionReason: msirbRebootReason = msirbRebootReason(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msirbRebootDeferred: msirbRebootType = msirbRebootType(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msirbRebootForceRebootReason: msirbRebootReason = msirbRebootReason(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msirbRebootImmediate: msirbRebootType = msirbRebootType(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msirbRebootInUseFilesReason: msirbRebootReason = msirbRebootReason(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msirbRebootScheduleRebootReason: msirbRebootReason = msirbRebootReason(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msirbRebootUndeterminedReason: msirbRebootReason = msirbRebootReason(0i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msmErrorDirCreate: msmErrorType = msmErrorType(7i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msmErrorExclusion: msmErrorType = msmErrorType(3i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msmErrorFeatureRequired: msmErrorType = msmErrorType(8i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msmErrorFileCreate: msmErrorType = msmErrorType(6i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msmErrorLanguageFailed: msmErrorType = msmErrorType(2i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msmErrorLanguageUnsupported: msmErrorType = msmErrorType(1i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msmErrorResequenceMerge: msmErrorType = msmErrorType(5i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub const msmErrorTableMerge: msmErrorType = msmErrorType(4i32);
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ACTCTX_COMPATIBILITY_ELEMENT_TYPE(pub i32);
impl ::core::marker::Copy for ACTCTX_COMPATIBILITY_ELEMENT_TYPE {}
impl ::core::clone::Clone for ACTCTX_COMPATIBILITY_ELEMENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ACTCTX_COMPATIBILITY_ELEMENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ACTCTX_COMPATIBILITY_ELEMENT_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ACTCTX_COMPATIBILITY_ELEMENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACTCTX_COMPATIBILITY_ELEMENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ACTCTX_REQUESTED_RUN_LEVEL(pub i32);
impl ::core::marker::Copy for ACTCTX_REQUESTED_RUN_LEVEL {}
impl ::core::clone::Clone for ACTCTX_REQUESTED_RUN_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ACTCTX_REQUESTED_RUN_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ACTCTX_REQUESTED_RUN_LEVEL {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ACTCTX_REQUESTED_RUN_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACTCTX_REQUESTED_RUN_LEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ADVERTISEFLAGS(pub i32);
impl ::core::marker::Copy for ADVERTISEFLAGS {}
impl ::core::clone::Clone for ADVERTISEFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADVERTISEFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ADVERTISEFLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ADVERTISEFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADVERTISEFLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ASM_BIND_FLAGS(pub i32);
impl ::core::marker::Copy for ASM_BIND_FLAGS {}
impl ::core::clone::Clone for ASM_BIND_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ASM_BIND_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ASM_BIND_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ASM_BIND_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ASM_BIND_FLAGS").field(&self.0).finish()
    }
}
impl ASM_BIND_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for ASM_BIND_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ASM_BIND_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ASM_BIND_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ASM_BIND_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ASM_BIND_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ASM_CMP_FLAGS(pub i32);
impl ::core::marker::Copy for ASM_CMP_FLAGS {}
impl ::core::clone::Clone for ASM_CMP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ASM_CMP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ASM_CMP_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ASM_CMP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ASM_CMP_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ASM_DISPLAY_FLAGS(pub i32);
impl ::core::marker::Copy for ASM_DISPLAY_FLAGS {}
impl ::core::clone::Clone for ASM_DISPLAY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ASM_DISPLAY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ASM_DISPLAY_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ASM_DISPLAY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ASM_DISPLAY_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ASM_NAME(pub i32);
impl ::core::marker::Copy for ASM_NAME {}
impl ::core::clone::Clone for ASM_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ASM_NAME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ASM_NAME {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ASM_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ASM_NAME").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CREATE_ASM_NAME_OBJ_FLAGS(pub i32);
impl ::core::marker::Copy for CREATE_ASM_NAME_OBJ_FLAGS {}
impl ::core::clone::Clone for CREATE_ASM_NAME_OBJ_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CREATE_ASM_NAME_OBJ_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CREATE_ASM_NAME_OBJ_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CREATE_ASM_NAME_OBJ_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREATE_ASM_NAME_OBJ_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IASSEMBLYCACHE_UNINSTALL_DISPOSITION(pub u32);
impl ::core::marker::Copy for IASSEMBLYCACHE_UNINSTALL_DISPOSITION {}
impl ::core::clone::Clone for IASSEMBLYCACHE_UNINSTALL_DISPOSITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IASSEMBLYCACHE_UNINSTALL_DISPOSITION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IASSEMBLYCACHE_UNINSTALL_DISPOSITION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IASSEMBLYCACHE_UNINSTALL_DISPOSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IASSEMBLYCACHE_UNINSTALL_DISPOSITION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INSTALLFEATUREATTRIBUTE(pub i32);
impl ::core::marker::Copy for INSTALLFEATUREATTRIBUTE {}
impl ::core::clone::Clone for INSTALLFEATUREATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INSTALLFEATUREATTRIBUTE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for INSTALLFEATUREATTRIBUTE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for INSTALLFEATUREATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSTALLFEATUREATTRIBUTE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INSTALLLEVEL(pub i32);
impl ::core::marker::Copy for INSTALLLEVEL {}
impl ::core::clone::Clone for INSTALLLEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INSTALLLEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for INSTALLLEVEL {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for INSTALLLEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSTALLLEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INSTALLLOGATTRIBUTES(pub i32);
impl ::core::marker::Copy for INSTALLLOGATTRIBUTES {}
impl ::core::clone::Clone for INSTALLLOGATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INSTALLLOGATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for INSTALLLOGATTRIBUTES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for INSTALLLOGATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSTALLLOGATTRIBUTES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INSTALLLOGMODE(pub i32);
impl ::core::marker::Copy for INSTALLLOGMODE {}
impl ::core::clone::Clone for INSTALLLOGMODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INSTALLLOGMODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for INSTALLLOGMODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for INSTALLLOGMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSTALLLOGMODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INSTALLMESSAGE(pub i32);
impl ::core::marker::Copy for INSTALLMESSAGE {}
impl ::core::clone::Clone for INSTALLMESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INSTALLMESSAGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for INSTALLMESSAGE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for INSTALLMESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSTALLMESSAGE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INSTALLMODE(pub i32);
impl ::core::marker::Copy for INSTALLMODE {}
impl ::core::clone::Clone for INSTALLMODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INSTALLMODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for INSTALLMODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for INSTALLMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSTALLMODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INSTALLSTATE(pub i32);
impl ::core::marker::Copy for INSTALLSTATE {}
impl ::core::clone::Clone for INSTALLSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INSTALLSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for INSTALLSTATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for INSTALLSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSTALLSTATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INSTALLTYPE(pub i32);
impl ::core::marker::Copy for INSTALLTYPE {}
impl ::core::clone::Clone for INSTALLTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INSTALLTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for INSTALLTYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for INSTALLTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSTALLTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INSTALLUILEVEL(pub i32);
impl ::core::marker::Copy for INSTALLUILEVEL {}
impl ::core::clone::Clone for INSTALLUILEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INSTALLUILEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for INSTALLUILEVEL {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for INSTALLUILEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSTALLUILEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSIADVERTISEOPTIONFLAGS(pub i32);
impl ::core::marker::Copy for MSIADVERTISEOPTIONFLAGS {}
impl ::core::clone::Clone for MSIADVERTISEOPTIONFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSIADVERTISEOPTIONFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MSIADVERTISEOPTIONFLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MSIADVERTISEOPTIONFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSIADVERTISEOPTIONFLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSIARCHITECTUREFLAGS(pub i32);
impl ::core::marker::Copy for MSIARCHITECTUREFLAGS {}
impl ::core::clone::Clone for MSIARCHITECTUREFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSIARCHITECTUREFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MSIARCHITECTUREFLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MSIARCHITECTUREFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSIARCHITECTUREFLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSIASSEMBLYINFO(pub u32);
impl ::core::marker::Copy for MSIASSEMBLYINFO {}
impl ::core::clone::Clone for MSIASSEMBLYINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSIASSEMBLYINFO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MSIASSEMBLYINFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MSIASSEMBLYINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSIASSEMBLYINFO").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSICODE(pub i32);
impl ::core::marker::Copy for MSICODE {}
impl ::core::clone::Clone for MSICODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSICODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MSICODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MSICODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSICODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSICOLINFO(pub i32);
impl ::core::marker::Copy for MSICOLINFO {}
impl ::core::clone::Clone for MSICOLINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSICOLINFO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MSICOLINFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MSICOLINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSICOLINFO").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSICONDITION(pub i32);
impl ::core::marker::Copy for MSICONDITION {}
impl ::core::clone::Clone for MSICONDITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSICONDITION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MSICONDITION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MSICONDITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSICONDITION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSICOSTTREE(pub i32);
impl ::core::marker::Copy for MSICOSTTREE {}
impl ::core::clone::Clone for MSICOSTTREE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSICOSTTREE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MSICOSTTREE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MSICOSTTREE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSICOSTTREE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSIDBERROR(pub i32);
impl ::core::marker::Copy for MSIDBERROR {}
impl ::core::clone::Clone for MSIDBERROR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSIDBERROR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MSIDBERROR {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MSIDBERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSIDBERROR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSIDBSTATE(pub i32);
impl ::core::marker::Copy for MSIDBSTATE {}
impl ::core::clone::Clone for MSIDBSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSIDBSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MSIDBSTATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MSIDBSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSIDBSTATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSIINSTALLCONTEXT(pub i32);
impl ::core::marker::Copy for MSIINSTALLCONTEXT {}
impl ::core::clone::Clone for MSIINSTALLCONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSIINSTALLCONTEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MSIINSTALLCONTEXT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MSIINSTALLCONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSIINSTALLCONTEXT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSIMODIFY(pub i32);
impl ::core::marker::Copy for MSIMODIFY {}
impl ::core::clone::Clone for MSIMODIFY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSIMODIFY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MSIMODIFY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MSIMODIFY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSIMODIFY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSIOPENPACKAGEFLAGS(pub i32);
impl ::core::marker::Copy for MSIOPENPACKAGEFLAGS {}
impl ::core::clone::Clone for MSIOPENPACKAGEFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSIOPENPACKAGEFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MSIOPENPACKAGEFLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MSIOPENPACKAGEFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSIOPENPACKAGEFLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSIPATCHDATATYPE(pub i32);
impl ::core::marker::Copy for MSIPATCHDATATYPE {}
impl ::core::clone::Clone for MSIPATCHDATATYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSIPATCHDATATYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MSIPATCHDATATYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MSIPATCHDATATYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSIPATCHDATATYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSIPATCHSTATE(pub i32);
impl ::core::marker::Copy for MSIPATCHSTATE {}
impl ::core::clone::Clone for MSIPATCHSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSIPATCHSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MSIPATCHSTATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MSIPATCHSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSIPATCHSTATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSIRUNMODE(pub i32);
impl ::core::marker::Copy for MSIRUNMODE {}
impl ::core::clone::Clone for MSIRUNMODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSIRUNMODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MSIRUNMODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MSIRUNMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSIRUNMODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSISOURCETYPE(pub i32);
impl ::core::marker::Copy for MSISOURCETYPE {}
impl ::core::clone::Clone for MSISOURCETYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSISOURCETYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MSISOURCETYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MSISOURCETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSISOURCETYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSITRANSACTION(pub i32);
impl ::core::marker::Copy for MSITRANSACTION {}
impl ::core::clone::Clone for MSITRANSACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSITRANSACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MSITRANSACTION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MSITRANSACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSITRANSACTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSITRANSACTIONSTATE(pub u32);
impl ::core::marker::Copy for MSITRANSACTIONSTATE {}
impl ::core::clone::Clone for MSITRANSACTIONSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSITRANSACTIONSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MSITRANSACTIONSTATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MSITRANSACTIONSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSITRANSACTIONSTATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSITRANSFORM_ERROR(pub i32);
impl ::core::marker::Copy for MSITRANSFORM_ERROR {}
impl ::core::clone::Clone for MSITRANSFORM_ERROR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSITRANSFORM_ERROR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MSITRANSFORM_ERROR {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MSITRANSFORM_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSITRANSFORM_ERROR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSITRANSFORM_VALIDATE(pub i32);
impl ::core::marker::Copy for MSITRANSFORM_VALIDATE {}
impl ::core::clone::Clone for MSITRANSFORM_VALIDATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSITRANSFORM_VALIDATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MSITRANSFORM_VALIDATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MSITRANSFORM_VALIDATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSITRANSFORM_VALIDATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PACKMAN_RUNTIME(pub i32);
impl ::core::marker::Copy for PACKMAN_RUNTIME {}
impl ::core::clone::Clone for PACKMAN_RUNTIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PACKMAN_RUNTIME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PACKMAN_RUNTIME {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PACKMAN_RUNTIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PACKMAN_RUNTIME").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_ACTIVATION_POLICY(pub i32);
impl ::core::marker::Copy for PM_ACTIVATION_POLICY {}
impl ::core::clone::Clone for PM_ACTIVATION_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_ACTIVATION_POLICY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PM_ACTIVATION_POLICY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PM_ACTIVATION_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_ACTIVATION_POLICY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_APPLICATION_HUBTYPE(pub i32);
impl ::core::marker::Copy for PM_APPLICATION_HUBTYPE {}
impl ::core::clone::Clone for PM_APPLICATION_HUBTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_APPLICATION_HUBTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PM_APPLICATION_HUBTYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PM_APPLICATION_HUBTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_APPLICATION_HUBTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_APPLICATION_INSTALL_TYPE(pub i32);
impl ::core::marker::Copy for PM_APPLICATION_INSTALL_TYPE {}
impl ::core::clone::Clone for PM_APPLICATION_INSTALL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_APPLICATION_INSTALL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PM_APPLICATION_INSTALL_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PM_APPLICATION_INSTALL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_APPLICATION_INSTALL_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_APPLICATION_STATE(pub i32);
impl ::core::marker::Copy for PM_APPLICATION_STATE {}
impl ::core::clone::Clone for PM_APPLICATION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_APPLICATION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PM_APPLICATION_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PM_APPLICATION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_APPLICATION_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_APP_GENRE(pub i32);
impl ::core::marker::Copy for PM_APP_GENRE {}
impl ::core::clone::Clone for PM_APP_GENRE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_APP_GENRE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PM_APP_GENRE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PM_APP_GENRE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_APP_GENRE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_ENUM_APP_FILTER(pub i32);
impl ::core::marker::Copy for PM_ENUM_APP_FILTER {}
impl ::core::clone::Clone for PM_ENUM_APP_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_ENUM_APP_FILTER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PM_ENUM_APP_FILTER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PM_ENUM_APP_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_ENUM_APP_FILTER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_ENUM_BSA_FILTER(pub i32);
impl ::core::marker::Copy for PM_ENUM_BSA_FILTER {}
impl ::core::clone::Clone for PM_ENUM_BSA_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_ENUM_BSA_FILTER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PM_ENUM_BSA_FILTER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PM_ENUM_BSA_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_ENUM_BSA_FILTER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_ENUM_BW_FILTER(pub i32);
impl ::core::marker::Copy for PM_ENUM_BW_FILTER {}
impl ::core::clone::Clone for PM_ENUM_BW_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_ENUM_BW_FILTER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PM_ENUM_BW_FILTER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PM_ENUM_BW_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_ENUM_BW_FILTER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_ENUM_EXTENSION_FILTER(pub i32);
impl ::core::marker::Copy for PM_ENUM_EXTENSION_FILTER {}
impl ::core::clone::Clone for PM_ENUM_EXTENSION_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_ENUM_EXTENSION_FILTER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PM_ENUM_EXTENSION_FILTER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PM_ENUM_EXTENSION_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_ENUM_EXTENSION_FILTER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_ENUM_TASK_FILTER(pub i32);
impl ::core::marker::Copy for PM_ENUM_TASK_FILTER {}
impl ::core::clone::Clone for PM_ENUM_TASK_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_ENUM_TASK_FILTER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PM_ENUM_TASK_FILTER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PM_ENUM_TASK_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_ENUM_TASK_FILTER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_ENUM_TILE_FILTER(pub i32);
impl ::core::marker::Copy for PM_ENUM_TILE_FILTER {}
impl ::core::clone::Clone for PM_ENUM_TILE_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_ENUM_TILE_FILTER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PM_ENUM_TILE_FILTER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PM_ENUM_TILE_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_ENUM_TILE_FILTER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_LIVETILE_RECURRENCE_TYPE(pub i32);
impl ::core::marker::Copy for PM_LIVETILE_RECURRENCE_TYPE {}
impl ::core::clone::Clone for PM_LIVETILE_RECURRENCE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_LIVETILE_RECURRENCE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PM_LIVETILE_RECURRENCE_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PM_LIVETILE_RECURRENCE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_LIVETILE_RECURRENCE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_LOGO_SIZE(pub i32);
impl ::core::marker::Copy for PM_LOGO_SIZE {}
impl ::core::clone::Clone for PM_LOGO_SIZE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_LOGO_SIZE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PM_LOGO_SIZE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PM_LOGO_SIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_LOGO_SIZE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_STARTTILE_TYPE(pub i32);
impl ::core::marker::Copy for PM_STARTTILE_TYPE {}
impl ::core::clone::Clone for PM_STARTTILE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_STARTTILE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PM_STARTTILE_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PM_STARTTILE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_STARTTILE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_TASK_TRANSITION(pub i32);
impl ::core::marker::Copy for PM_TASK_TRANSITION {}
impl ::core::clone::Clone for PM_TASK_TRANSITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_TASK_TRANSITION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PM_TASK_TRANSITION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PM_TASK_TRANSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_TASK_TRANSITION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_TASK_TYPE(pub i32);
impl ::core::marker::Copy for PM_TASK_TYPE {}
impl ::core::clone::Clone for PM_TASK_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_TASK_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PM_TASK_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PM_TASK_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_TASK_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_TILE_HUBTYPE(pub i32);
impl ::core::marker::Copy for PM_TILE_HUBTYPE {}
impl ::core::clone::Clone for PM_TILE_HUBTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_TILE_HUBTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PM_TILE_HUBTYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PM_TILE_HUBTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_TILE_HUBTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PM_TILE_SIZE(pub i32);
impl ::core::marker::Copy for PM_TILE_SIZE {}
impl ::core::clone::Clone for PM_TILE_SIZE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PM_TILE_SIZE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PM_TILE_SIZE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PM_TILE_SIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PM_TILE_SIZE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct QUERYASMINFO_FLAGS(pub u32);
impl ::core::marker::Copy for QUERYASMINFO_FLAGS {}
impl ::core::clone::Clone for QUERYASMINFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for QUERYASMINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for QUERYASMINFO_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for QUERYASMINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUERYASMINFO_FLAGS").field(&self.0).finish()
    }
}
impl QUERYASMINFO_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for QUERYASMINFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for QUERYASMINFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for QUERYASMINFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for QUERYASMINFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for QUERYASMINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REINSTALLMODE(pub i32);
impl ::core::marker::Copy for REINSTALLMODE {}
impl ::core::clone::Clone for REINSTALLMODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REINSTALLMODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for REINSTALLMODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for REINSTALLMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REINSTALLMODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RESULTTYPES(pub i32);
impl ::core::marker::Copy for RESULTTYPES {}
impl ::core::clone::Clone for RESULTTYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RESULTTYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for RESULTTYPES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RESULTTYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RESULTTYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCRIPTFLAGS(pub i32);
impl ::core::marker::Copy for SCRIPTFLAGS {}
impl ::core::clone::Clone for SCRIPTFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCRIPTFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SCRIPTFLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SCRIPTFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCRIPTFLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct STATUSTYPES(pub i32);
impl ::core::marker::Copy for STATUSTYPES {}
impl ::core::clone::Clone for STATUSTYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STATUSTYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for STATUSTYPES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for STATUSTYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STATUSTYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TILE_TEMPLATE_TYPE(pub i32);
impl ::core::marker::Copy for TILE_TEMPLATE_TYPE {}
impl ::core::clone::Clone for TILE_TEMPLATE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TILE_TEMPLATE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TILE_TEMPLATE_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TILE_TEMPLATE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TILE_TEMPLATE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct USERINFOSTATE(pub i32);
impl ::core::marker::Copy for USERINFOSTATE {}
impl ::core::clone::Clone for USERINFOSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for USERINFOSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for USERINFOSTATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for USERINFOSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USERINFOSTATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbAssemblyAttributes(pub i32);
impl ::core::marker::Copy for msidbAssemblyAttributes {}
impl ::core::clone::Clone for msidbAssemblyAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbAssemblyAttributes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for msidbAssemblyAttributes {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for msidbAssemblyAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbAssemblyAttributes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbClassAttributes(pub i32);
impl ::core::marker::Copy for msidbClassAttributes {}
impl ::core::clone::Clone for msidbClassAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbClassAttributes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for msidbClassAttributes {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for msidbClassAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbClassAttributes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbComponentAttributes(pub i32);
impl ::core::marker::Copy for msidbComponentAttributes {}
impl ::core::clone::Clone for msidbComponentAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbComponentAttributes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for msidbComponentAttributes {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for msidbComponentAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbComponentAttributes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbControlAttributes(pub i32);
impl ::core::marker::Copy for msidbControlAttributes {}
impl ::core::clone::Clone for msidbControlAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbControlAttributes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for msidbControlAttributes {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for msidbControlAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbControlAttributes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbCustomActionType(pub i32);
impl ::core::marker::Copy for msidbCustomActionType {}
impl ::core::clone::Clone for msidbCustomActionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbCustomActionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for msidbCustomActionType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for msidbCustomActionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbCustomActionType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbDialogAttributes(pub i32);
impl ::core::marker::Copy for msidbDialogAttributes {}
impl ::core::clone::Clone for msidbDialogAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbDialogAttributes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for msidbDialogAttributes {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for msidbDialogAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbDialogAttributes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbEmbeddedUIAttributes(pub i32);
impl ::core::marker::Copy for msidbEmbeddedUIAttributes {}
impl ::core::clone::Clone for msidbEmbeddedUIAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbEmbeddedUIAttributes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for msidbEmbeddedUIAttributes {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for msidbEmbeddedUIAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbEmbeddedUIAttributes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbFeatureAttributes(pub i32);
impl ::core::marker::Copy for msidbFeatureAttributes {}
impl ::core::clone::Clone for msidbFeatureAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbFeatureAttributes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for msidbFeatureAttributes {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for msidbFeatureAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbFeatureAttributes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbFileAttributes(pub i32);
impl ::core::marker::Copy for msidbFileAttributes {}
impl ::core::clone::Clone for msidbFileAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbFileAttributes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for msidbFileAttributes {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for msidbFileAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbFileAttributes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbIniFileAction(pub i32);
impl ::core::marker::Copy for msidbIniFileAction {}
impl ::core::clone::Clone for msidbIniFileAction {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbIniFileAction {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for msidbIniFileAction {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for msidbIniFileAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbIniFileAction").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbLocatorType(pub i32);
impl ::core::marker::Copy for msidbLocatorType {}
impl ::core::clone::Clone for msidbLocatorType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbLocatorType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for msidbLocatorType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for msidbLocatorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbLocatorType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbMoveFileOptions(pub i32);
impl ::core::marker::Copy for msidbMoveFileOptions {}
impl ::core::clone::Clone for msidbMoveFileOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbMoveFileOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for msidbMoveFileOptions {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for msidbMoveFileOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbMoveFileOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbODBCDataSourceRegistration(pub i32);
impl ::core::marker::Copy for msidbODBCDataSourceRegistration {}
impl ::core::clone::Clone for msidbODBCDataSourceRegistration {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbODBCDataSourceRegistration {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for msidbODBCDataSourceRegistration {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for msidbODBCDataSourceRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbODBCDataSourceRegistration").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbPatchAttributes(pub i32);
impl ::core::marker::Copy for msidbPatchAttributes {}
impl ::core::clone::Clone for msidbPatchAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbPatchAttributes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for msidbPatchAttributes {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for msidbPatchAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbPatchAttributes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbRegistryRoot(pub i32);
impl ::core::marker::Copy for msidbRegistryRoot {}
impl ::core::clone::Clone for msidbRegistryRoot {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbRegistryRoot {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for msidbRegistryRoot {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for msidbRegistryRoot {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbRegistryRoot").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbRemoveFileInstallMode(pub i32);
impl ::core::marker::Copy for msidbRemoveFileInstallMode {}
impl ::core::clone::Clone for msidbRemoveFileInstallMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbRemoveFileInstallMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for msidbRemoveFileInstallMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for msidbRemoveFileInstallMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbRemoveFileInstallMode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbServiceConfigEvent(pub i32);
impl ::core::marker::Copy for msidbServiceConfigEvent {}
impl ::core::clone::Clone for msidbServiceConfigEvent {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbServiceConfigEvent {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for msidbServiceConfigEvent {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for msidbServiceConfigEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbServiceConfigEvent").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbServiceControlEvent(pub i32);
impl ::core::marker::Copy for msidbServiceControlEvent {}
impl ::core::clone::Clone for msidbServiceControlEvent {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbServiceControlEvent {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for msidbServiceControlEvent {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for msidbServiceControlEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbServiceControlEvent").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbServiceInstallErrorControl(pub i32);
impl ::core::marker::Copy for msidbServiceInstallErrorControl {}
impl ::core::clone::Clone for msidbServiceInstallErrorControl {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbServiceInstallErrorControl {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for msidbServiceInstallErrorControl {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for msidbServiceInstallErrorControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbServiceInstallErrorControl").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbSumInfoSourceType(pub i32);
impl ::core::marker::Copy for msidbSumInfoSourceType {}
impl ::core::clone::Clone for msidbSumInfoSourceType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbSumInfoSourceType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for msidbSumInfoSourceType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for msidbSumInfoSourceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbSumInfoSourceType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbTextStyleStyleBits(pub i32);
impl ::core::marker::Copy for msidbTextStyleStyleBits {}
impl ::core::clone::Clone for msidbTextStyleStyleBits {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbTextStyleStyleBits {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for msidbTextStyleStyleBits {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for msidbTextStyleStyleBits {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbTextStyleStyleBits").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msidbUpgradeAttributes(pub i32);
impl ::core::marker::Copy for msidbUpgradeAttributes {}
impl ::core::clone::Clone for msidbUpgradeAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msidbUpgradeAttributes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for msidbUpgradeAttributes {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for msidbUpgradeAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msidbUpgradeAttributes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msifiFastInstallBits(pub i32);
impl ::core::marker::Copy for msifiFastInstallBits {}
impl ::core::clone::Clone for msifiFastInstallBits {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msifiFastInstallBits {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for msifiFastInstallBits {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for msifiFastInstallBits {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msifiFastInstallBits").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msirbRebootReason(pub i32);
impl ::core::marker::Copy for msirbRebootReason {}
impl ::core::clone::Clone for msirbRebootReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msirbRebootReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for msirbRebootReason {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for msirbRebootReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msirbRebootReason").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msirbRebootType(pub i32);
impl ::core::marker::Copy for msirbRebootType {}
impl ::core::clone::Clone for msirbRebootType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msirbRebootType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for msirbRebootType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for msirbRebootType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msirbRebootType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct msmErrorType(pub i32);
impl ::core::marker::Copy for msmErrorType {}
impl ::core::clone::Clone for msmErrorType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for msmErrorType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for msmErrorType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for msmErrorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("msmErrorType").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACTCTXA {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub lpSource: ::windows_core::PCSTR,
    pub wProcessorArchitecture: u16,
    pub wLangId: u16,
    pub lpAssemblyDirectory: ::windows_core::PCSTR,
    pub lpResourceName: ::windows_core::PCSTR,
    pub lpApplicationName: ::windows_core::PCSTR,
    pub hModule: super::super::Foundation::HMODULE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACTCTXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACTCTXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ACTCTXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTCTXA").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("lpSource", &self.lpSource).field("wProcessorArchitecture", &self.wProcessorArchitecture).field("wLangId", &self.wLangId).field("lpAssemblyDirectory", &self.lpAssemblyDirectory).field("lpResourceName", &self.lpResourceName).field("lpApplicationName", &self.lpApplicationName).field("hModule", &self.hModule).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for ACTCTXA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ACTCTXA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.lpSource == other.lpSource && self.wProcessorArchitecture == other.wProcessorArchitecture && self.wLangId == other.wLangId && self.lpAssemblyDirectory == other.lpAssemblyDirectory && self.lpResourceName == other.lpResourceName && self.lpApplicationName == other.lpApplicationName && self.hModule == other.hModule
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ACTCTXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACTCTXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACTCTXW {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub lpSource: ::windows_core::PCWSTR,
    pub wProcessorArchitecture: u16,
    pub wLangId: u16,
    pub lpAssemblyDirectory: ::windows_core::PCWSTR,
    pub lpResourceName: ::windows_core::PCWSTR,
    pub lpApplicationName: ::windows_core::PCWSTR,
    pub hModule: super::super::Foundation::HMODULE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACTCTXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACTCTXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ACTCTXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTCTXW").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("lpSource", &self.lpSource).field("wProcessorArchitecture", &self.wProcessorArchitecture).field("wLangId", &self.wLangId).field("lpAssemblyDirectory", &self.lpAssemblyDirectory).field("lpResourceName", &self.lpResourceName).field("lpApplicationName", &self.lpApplicationName).field("hModule", &self.hModule).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for ACTCTXW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ACTCTXW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.lpSource == other.lpSource && self.wProcessorArchitecture == other.wProcessorArchitecture && self.wLangId == other.wLangId && self.lpAssemblyDirectory == other.lpAssemblyDirectory && self.lpResourceName == other.lpResourceName && self.lpApplicationName == other.lpApplicationName && self.hModule == other.hModule
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ACTCTXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACTCTXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
pub struct ACTCTX_SECTION_KEYED_DATA {
    pub cbSize: u32,
    pub ulDataFormatVersion: u32,
    pub lpData: *mut ::core::ffi::c_void,
    pub ulLength: u32,
    pub lpSectionGlobalData: *mut ::core::ffi::c_void,
    pub ulSectionGlobalDataLength: u32,
    pub lpSectionBase: *mut ::core::ffi::c_void,
    pub ulSectionTotalLength: u32,
    pub hActCtx: super::super::Foundation::HANDLE,
    pub ulAssemblyRosterIndex: u32,
    pub ulFlags: u32,
    pub AssemblyMetadata: super::WindowsProgramming::ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
impl ::core::marker::Copy for ACTCTX_SECTION_KEYED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
impl ::core::clone::Clone for ACTCTX_SECTION_KEYED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
impl ::core::fmt::Debug for ACTCTX_SECTION_KEYED_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTCTX_SECTION_KEYED_DATA")
            .field("cbSize", &self.cbSize)
            .field("ulDataFormatVersion", &self.ulDataFormatVersion)
            .field("lpData", &self.lpData)
            .field("ulLength", &self.ulLength)
            .field("lpSectionGlobalData", &self.lpSectionGlobalData)
            .field("ulSectionGlobalDataLength", &self.ulSectionGlobalDataLength)
            .field("lpSectionBase", &self.lpSectionBase)
            .field("ulSectionTotalLength", &self.ulSectionTotalLength)
            .field("hActCtx", &self.hActCtx)
            .field("ulAssemblyRosterIndex", &self.ulAssemblyRosterIndex)
            .field("ulFlags", &self.ulFlags)
            .field("AssemblyMetadata", &self.AssemblyMetadata)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
impl ::windows_core::TypeKind for ACTCTX_SECTION_KEYED_DATA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
impl ::core::cmp::PartialEq for ACTCTX_SECTION_KEYED_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.ulDataFormatVersion == other.ulDataFormatVersion && self.lpData == other.lpData && self.ulLength == other.ulLength && self.lpSectionGlobalData == other.lpSectionGlobalData && self.ulSectionGlobalDataLength == other.ulSectionGlobalDataLength && self.lpSectionBase == other.lpSectionBase && self.ulSectionTotalLength == other.ulSectionTotalLength && self.hActCtx == other.hActCtx && self.ulAssemblyRosterIndex == other.ulAssemblyRosterIndex && self.ulFlags == other.ulFlags && self.AssemblyMetadata == other.AssemblyMetadata
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
impl ::core::cmp::Eq for ACTCTX_SECTION_KEYED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
impl ::core::default::Default for ACTCTX_SECTION_KEYED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {
    pub ulFlags: u32,
    pub ulEncodedAssemblyIdentityLength: u32,
    pub ulManifestPathType: u32,
    pub ulManifestPathLength: u32,
    pub liManifestLastWriteTime: i64,
    pub ulPolicyPathType: u32,
    pub ulPolicyPathLength: u32,
    pub liPolicyLastWriteTime: i64,
    pub ulMetadataSatelliteRosterIndex: u32,
    pub ulManifestVersionMajor: u32,
    pub ulManifestVersionMinor: u32,
    pub ulPolicyVersionMajor: u32,
    pub ulPolicyVersionMinor: u32,
    pub ulAssemblyDirectoryNameLength: u32,
    pub lpAssemblyEncodedAssemblyIdentity: ::windows_core::PCWSTR,
    pub lpAssemblyManifestPath: ::windows_core::PCWSTR,
    pub lpAssemblyPolicyPath: ::windows_core::PCWSTR,
    pub lpAssemblyDirectoryName: ::windows_core::PCWSTR,
    pub ulFileCount: u32,
}
impl ::core::marker::Copy for ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {}
impl ::core::clone::Clone for ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION")
            .field("ulFlags", &self.ulFlags)
            .field("ulEncodedAssemblyIdentityLength", &self.ulEncodedAssemblyIdentityLength)
            .field("ulManifestPathType", &self.ulManifestPathType)
            .field("ulManifestPathLength", &self.ulManifestPathLength)
            .field("liManifestLastWriteTime", &self.liManifestLastWriteTime)
            .field("ulPolicyPathType", &self.ulPolicyPathType)
            .field("ulPolicyPathLength", &self.ulPolicyPathLength)
            .field("liPolicyLastWriteTime", &self.liPolicyLastWriteTime)
            .field("ulMetadataSatelliteRosterIndex", &self.ulMetadataSatelliteRosterIndex)
            .field("ulManifestVersionMajor", &self.ulManifestVersionMajor)
            .field("ulManifestVersionMinor", &self.ulManifestVersionMinor)
            .field("ulPolicyVersionMajor", &self.ulPolicyVersionMajor)
            .field("ulPolicyVersionMinor", &self.ulPolicyVersionMinor)
            .field("ulAssemblyDirectoryNameLength", &self.ulAssemblyDirectoryNameLength)
            .field("lpAssemblyEncodedAssemblyIdentity", &self.lpAssemblyEncodedAssemblyIdentity)
            .field("lpAssemblyManifestPath", &self.lpAssemblyManifestPath)
            .field("lpAssemblyPolicyPath", &self.lpAssemblyPolicyPath)
            .field("lpAssemblyDirectoryName", &self.lpAssemblyDirectoryName)
            .field("ulFileCount", &self.ulFileCount)
            .finish()
    }
}
impl ::windows_core::TypeKind for ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulFlags == other.ulFlags
            && self.ulEncodedAssemblyIdentityLength == other.ulEncodedAssemblyIdentityLength
            && self.ulManifestPathType == other.ulManifestPathType
            && self.ulManifestPathLength == other.ulManifestPathLength
            && self.liManifestLastWriteTime == other.liManifestLastWriteTime
            && self.ulPolicyPathType == other.ulPolicyPathType
            && self.ulPolicyPathLength == other.ulPolicyPathLength
            && self.liPolicyLastWriteTime == other.liPolicyLastWriteTime
            && self.ulMetadataSatelliteRosterIndex == other.ulMetadataSatelliteRosterIndex
            && self.ulManifestVersionMajor == other.ulManifestVersionMajor
            && self.ulManifestVersionMinor == other.ulManifestVersionMinor
            && self.ulPolicyVersionMajor == other.ulPolicyVersionMajor
            && self.ulPolicyVersionMinor == other.ulPolicyVersionMinor
            && self.ulAssemblyDirectoryNameLength == other.ulAssemblyDirectoryNameLength
            && self.lpAssemblyEncodedAssemblyIdentity == other.lpAssemblyEncodedAssemblyIdentity
            && self.lpAssemblyManifestPath == other.lpAssemblyManifestPath
            && self.lpAssemblyPolicyPath == other.lpAssemblyPolicyPath
            && self.lpAssemblyDirectoryName == other.lpAssemblyDirectoryName
            && self.ulFileCount == other.ulFileCount
    }
}
impl ::core::cmp::Eq for ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {}
impl ::core::default::Default for ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    pub ElementCount: u32,
    pub Elements: [COMPATIBILITY_CONTEXT_ELEMENT; 1],
}
impl ::core::marker::Copy for ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {}
impl ::core::clone::Clone for ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION").field("ElementCount", &self.ElementCount).field("Elements", &self.Elements).finish()
    }
}
impl ::windows_core::TypeKind for ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ElementCount == other.ElementCount && self.Elements == other.Elements
    }
}
impl ::core::cmp::Eq for ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {}
impl ::core::default::Default for ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct ACTIVATION_CONTEXT_DETAILED_INFORMATION {
    pub dwFlags: u32,
    pub ulFormatVersion: u32,
    pub ulAssemblyCount: u32,
    pub ulRootManifestPathType: u32,
    pub ulRootManifestPathChars: u32,
    pub ulRootConfigurationPathType: u32,
    pub ulRootConfigurationPathChars: u32,
    pub ulAppDirPathType: u32,
    pub ulAppDirPathChars: u32,
    pub lpRootManifestPath: ::windows_core::PCWSTR,
    pub lpRootConfigurationPath: ::windows_core::PCWSTR,
    pub lpAppDirPath: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for ACTIVATION_CONTEXT_DETAILED_INFORMATION {}
impl ::core::clone::Clone for ACTIVATION_CONTEXT_DETAILED_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTIVATION_CONTEXT_DETAILED_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTIVATION_CONTEXT_DETAILED_INFORMATION")
            .field("dwFlags", &self.dwFlags)
            .field("ulFormatVersion", &self.ulFormatVersion)
            .field("ulAssemblyCount", &self.ulAssemblyCount)
            .field("ulRootManifestPathType", &self.ulRootManifestPathType)
            .field("ulRootManifestPathChars", &self.ulRootManifestPathChars)
            .field("ulRootConfigurationPathType", &self.ulRootConfigurationPathType)
            .field("ulRootConfigurationPathChars", &self.ulRootConfigurationPathChars)
            .field("ulAppDirPathType", &self.ulAppDirPathType)
            .field("ulAppDirPathChars", &self.ulAppDirPathChars)
            .field("lpRootManifestPath", &self.lpRootManifestPath)
            .field("lpRootConfigurationPath", &self.lpRootConfigurationPath)
            .field("lpAppDirPath", &self.lpAppDirPath)
            .finish()
    }
}
impl ::windows_core::TypeKind for ACTIVATION_CONTEXT_DETAILED_INFORMATION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for ACTIVATION_CONTEXT_DETAILED_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.ulFormatVersion == other.ulFormatVersion && self.ulAssemblyCount == other.ulAssemblyCount && self.ulRootManifestPathType == other.ulRootManifestPathType && self.ulRootManifestPathChars == other.ulRootManifestPathChars && self.ulRootConfigurationPathType == other.ulRootConfigurationPathType && self.ulRootConfigurationPathChars == other.ulRootConfigurationPathChars && self.ulAppDirPathType == other.ulAppDirPathType && self.ulAppDirPathChars == other.ulAppDirPathChars && self.lpRootManifestPath == other.lpRootManifestPath && self.lpRootConfigurationPath == other.lpRootConfigurationPath && self.lpAppDirPath == other.lpAppDirPath
    }
}
impl ::core::cmp::Eq for ACTIVATION_CONTEXT_DETAILED_INFORMATION {}
impl ::core::default::Default for ACTIVATION_CONTEXT_DETAILED_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct ACTIVATION_CONTEXT_QUERY_INDEX {
    pub ulAssemblyIndex: u32,
    pub ulFileIndexInAssembly: u32,
}
impl ::core::marker::Copy for ACTIVATION_CONTEXT_QUERY_INDEX {}
impl ::core::clone::Clone for ACTIVATION_CONTEXT_QUERY_INDEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTIVATION_CONTEXT_QUERY_INDEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTIVATION_CONTEXT_QUERY_INDEX").field("ulAssemblyIndex", &self.ulAssemblyIndex).field("ulFileIndexInAssembly", &self.ulFileIndexInAssembly).finish()
    }
}
impl ::windows_core::TypeKind for ACTIVATION_CONTEXT_QUERY_INDEX {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for ACTIVATION_CONTEXT_QUERY_INDEX {
    fn eq(&self, other: &Self) -> bool {
        self.ulAssemblyIndex == other.ulAssemblyIndex && self.ulFileIndexInAssembly == other.ulFileIndexInAssembly
    }
}
impl ::core::cmp::Eq for ACTIVATION_CONTEXT_QUERY_INDEX {}
impl ::core::default::Default for ACTIVATION_CONTEXT_QUERY_INDEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {
    pub ulFlags: u32,
    pub RunLevel: ACTCTX_REQUESTED_RUN_LEVEL,
    pub UiAccess: u32,
}
impl ::core::marker::Copy for ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {}
impl ::core::clone::Clone for ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION").field("ulFlags", &self.ulFlags).field("RunLevel", &self.RunLevel).field("UiAccess", &self.UiAccess).finish()
    }
}
impl ::windows_core::TypeKind for ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulFlags == other.ulFlags && self.RunLevel == other.RunLevel && self.UiAccess == other.UiAccess
    }
}
impl ::core::cmp::Eq for ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {}
impl ::core::default::Default for ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct ASSEMBLY_FILE_DETAILED_INFORMATION {
    pub ulFlags: u32,
    pub ulFilenameLength: u32,
    pub ulPathLength: u32,
    pub lpFileName: ::windows_core::PCWSTR,
    pub lpFilePath: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for ASSEMBLY_FILE_DETAILED_INFORMATION {}
impl ::core::clone::Clone for ASSEMBLY_FILE_DETAILED_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ASSEMBLY_FILE_DETAILED_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ASSEMBLY_FILE_DETAILED_INFORMATION").field("ulFlags", &self.ulFlags).field("ulFilenameLength", &self.ulFilenameLength).field("ulPathLength", &self.ulPathLength).field("lpFileName", &self.lpFileName).field("lpFilePath", &self.lpFilePath).finish()
    }
}
impl ::windows_core::TypeKind for ASSEMBLY_FILE_DETAILED_INFORMATION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for ASSEMBLY_FILE_DETAILED_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulFlags == other.ulFlags && self.ulFilenameLength == other.ulFilenameLength && self.ulPathLength == other.ulPathLength && self.lpFileName == other.lpFileName && self.lpFilePath == other.lpFilePath
    }
}
impl ::core::cmp::Eq for ASSEMBLY_FILE_DETAILED_INFORMATION {}
impl ::core::default::Default for ASSEMBLY_FILE_DETAILED_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct ASSEMBLY_INFO {
    pub cbAssemblyInfo: u32,
    pub dwAssemblyFlags: u32,
    pub uliAssemblySizeInKB: u64,
    pub pszCurrentAssemblyPathBuf: ::windows_core::PWSTR,
    pub cchBuf: u32,
}
impl ::core::marker::Copy for ASSEMBLY_INFO {}
impl ::core::clone::Clone for ASSEMBLY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ASSEMBLY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ASSEMBLY_INFO").field("cbAssemblyInfo", &self.cbAssemblyInfo).field("dwAssemblyFlags", &self.dwAssemblyFlags).field("uliAssemblySizeInKB", &self.uliAssemblySizeInKB).field("pszCurrentAssemblyPathBuf", &self.pszCurrentAssemblyPathBuf).field("cchBuf", &self.cchBuf).finish()
    }
}
impl ::windows_core::TypeKind for ASSEMBLY_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for ASSEMBLY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbAssemblyInfo == other.cbAssemblyInfo && self.dwAssemblyFlags == other.dwAssemblyFlags && self.uliAssemblySizeInKB == other.uliAssemblySizeInKB && self.pszCurrentAssemblyPathBuf == other.pszCurrentAssemblyPathBuf && self.cchBuf == other.cchBuf
    }
}
impl ::core::cmp::Eq for ASSEMBLY_INFO {}
impl ::core::default::Default for ASSEMBLY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct COMPATIBILITY_CONTEXT_ELEMENT {
    pub Id: ::windows_core::GUID,
    pub Type: ACTCTX_COMPATIBILITY_ELEMENT_TYPE,
    pub MaxVersionTested: u64,
}
impl ::core::marker::Copy for COMPATIBILITY_CONTEXT_ELEMENT {}
impl ::core::clone::Clone for COMPATIBILITY_CONTEXT_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMPATIBILITY_CONTEXT_ELEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPATIBILITY_CONTEXT_ELEMENT").field("Id", &self.Id).field("Type", &self.Type).field("MaxVersionTested", &self.MaxVersionTested).finish()
    }
}
impl ::windows_core::TypeKind for COMPATIBILITY_CONTEXT_ELEMENT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for COMPATIBILITY_CONTEXT_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.Id == other.Id && self.Type == other.Type && self.MaxVersionTested == other.MaxVersionTested
    }
}
impl ::core::cmp::Eq for COMPATIBILITY_CONTEXT_ELEMENT {}
impl ::core::default::Default for COMPATIBILITY_CONTEXT_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct DELTA_HASH {
    pub HashSize: u32,
    pub HashValue: [u8; 32],
}
impl ::core::marker::Copy for DELTA_HASH {}
impl ::core::clone::Clone for DELTA_HASH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DELTA_HASH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DELTA_HASH").field("HashSize", &self.HashSize).field("HashValue", &self.HashValue).finish()
    }
}
impl ::windows_core::TypeKind for DELTA_HASH {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DELTA_HASH {
    fn eq(&self, other: &Self) -> bool {
        self.HashSize == other.HashSize && self.HashValue == other.HashValue
    }
}
impl ::core::cmp::Eq for DELTA_HASH {}
impl ::core::default::Default for DELTA_HASH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DELTA_HEADER_INFO {
    pub FileTypeSet: i64,
    pub FileType: i64,
    pub Flags: i64,
    pub TargetSize: usize,
    pub TargetFileTime: super::super::Foundation::FILETIME,
    pub TargetHashAlgId: u32,
    pub TargetHash: DELTA_HASH,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DELTA_HEADER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DELTA_HEADER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DELTA_HEADER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DELTA_HEADER_INFO").field("FileTypeSet", &self.FileTypeSet).field("FileType", &self.FileType).field("Flags", &self.Flags).field("TargetSize", &self.TargetSize).field("TargetFileTime", &self.TargetFileTime).field("TargetHashAlgId", &self.TargetHashAlgId).field("TargetHash", &self.TargetHash).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DELTA_HEADER_INFO {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DELTA_HEADER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.FileTypeSet == other.FileTypeSet && self.FileType == other.FileType && self.Flags == other.Flags && self.TargetSize == other.TargetSize && self.TargetFileTime == other.TargetFileTime && self.TargetHashAlgId == other.TargetHashAlgId && self.TargetHash == other.TargetHash
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DELTA_HEADER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DELTA_HEADER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DELTA_INPUT {
    pub Anonymous: DELTA_INPUT_0,
    pub uSize: usize,
    pub Editable: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DELTA_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DELTA_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DELTA_INPUT {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DELTA_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union DELTA_INPUT_0 {
    pub lpcStart: *const ::core::ffi::c_void,
    pub lpStart: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DELTA_INPUT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DELTA_INPUT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DELTA_INPUT_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DELTA_INPUT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct DELTA_OUTPUT {
    pub lpStart: *mut ::core::ffi::c_void,
    pub uSize: usize,
}
impl ::core::marker::Copy for DELTA_OUTPUT {}
impl ::core::clone::Clone for DELTA_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DELTA_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DELTA_OUTPUT").field("lpStart", &self.lpStart).field("uSize", &self.uSize).finish()
    }
}
impl ::windows_core::TypeKind for DELTA_OUTPUT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DELTA_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.lpStart == other.lpStart && self.uSize == other.uSize
    }
}
impl ::core::cmp::Eq for DELTA_OUTPUT {}
impl ::core::default::Default for DELTA_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct FUSION_INSTALL_REFERENCE {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub guidScheme: ::windows_core::GUID,
    pub szIdentifier: ::windows_core::PCWSTR,
    pub szNonCannonicalData: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for FUSION_INSTALL_REFERENCE {}
impl ::core::clone::Clone for FUSION_INSTALL_REFERENCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FUSION_INSTALL_REFERENCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FUSION_INSTALL_REFERENCE").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("guidScheme", &self.guidScheme).field("szIdentifier", &self.szIdentifier).field("szNonCannonicalData", &self.szNonCannonicalData).finish()
    }
}
impl ::windows_core::TypeKind for FUSION_INSTALL_REFERENCE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for FUSION_INSTALL_REFERENCE {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.guidScheme == other.guidScheme && self.szIdentifier == other.szIdentifier && self.szNonCannonicalData == other.szNonCannonicalData
    }
}
impl ::core::cmp::Eq for FUSION_INSTALL_REFERENCE {}
impl ::core::default::Default for FUSION_INSTALL_REFERENCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct MSIFILEHASHINFO {
    pub dwFileHashInfoSize: u32,
    pub dwData: [u32; 4],
}
impl ::core::marker::Copy for MSIFILEHASHINFO {}
impl ::core::clone::Clone for MSIFILEHASHINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MSIFILEHASHINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSIFILEHASHINFO").field("dwFileHashInfoSize", &self.dwFileHashInfoSize).field("dwData", &self.dwData).finish()
    }
}
impl ::windows_core::TypeKind for MSIFILEHASHINFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for MSIFILEHASHINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFileHashInfoSize == other.dwFileHashInfoSize && self.dwData == other.dwData
    }
}
impl ::core::cmp::Eq for MSIFILEHASHINFO {}
impl ::core::default::Default for MSIFILEHASHINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MSIHANDLE(pub u32);
impl MSIHANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0
    }
}
impl ::core::default::Default for MSIHANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for MSIHANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for MSIHANDLE {}
impl ::core::fmt::Debug for MSIHANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSIHANDLE").field(&self.0).finish()
    }
}
impl ::windows_core::TypeKind for MSIHANDLE {
    type TypeKind = ::windows_core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct MSIPATCHSEQUENCEINFOA {
    pub szPatchData: ::windows_core::PCSTR,
    pub ePatchDataType: MSIPATCHDATATYPE,
    pub dwOrder: u32,
    pub uStatus: u32,
}
impl ::core::marker::Copy for MSIPATCHSEQUENCEINFOA {}
impl ::core::clone::Clone for MSIPATCHSEQUENCEINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MSIPATCHSEQUENCEINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSIPATCHSEQUENCEINFOA").field("szPatchData", &self.szPatchData).field("ePatchDataType", &self.ePatchDataType).field("dwOrder", &self.dwOrder).field("uStatus", &self.uStatus).finish()
    }
}
impl ::windows_core::TypeKind for MSIPATCHSEQUENCEINFOA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for MSIPATCHSEQUENCEINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.szPatchData == other.szPatchData && self.ePatchDataType == other.ePatchDataType && self.dwOrder == other.dwOrder && self.uStatus == other.uStatus
    }
}
impl ::core::cmp::Eq for MSIPATCHSEQUENCEINFOA {}
impl ::core::default::Default for MSIPATCHSEQUENCEINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct MSIPATCHSEQUENCEINFOW {
    pub szPatchData: ::windows_core::PCWSTR,
    pub ePatchDataType: MSIPATCHDATATYPE,
    pub dwOrder: u32,
    pub uStatus: u32,
}
impl ::core::marker::Copy for MSIPATCHSEQUENCEINFOW {}
impl ::core::clone::Clone for MSIPATCHSEQUENCEINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MSIPATCHSEQUENCEINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSIPATCHSEQUENCEINFOW").field("szPatchData", &self.szPatchData).field("ePatchDataType", &self.ePatchDataType).field("dwOrder", &self.dwOrder).field("uStatus", &self.uStatus).finish()
    }
}
impl ::windows_core::TypeKind for MSIPATCHSEQUENCEINFOW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for MSIPATCHSEQUENCEINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.szPatchData == other.szPatchData && self.ePatchDataType == other.ePatchDataType && self.dwOrder == other.dwOrder && self.uStatus == other.uStatus
    }
}
impl ::core::cmp::Eq for MSIPATCHSEQUENCEINFOW {}
impl ::core::default::Default for MSIPATCHSEQUENCEINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PATCH_IGNORE_RANGE {
    pub OffsetInOldFile: u32,
    pub LengthInBytes: u32,
}
impl ::core::marker::Copy for PATCH_IGNORE_RANGE {}
impl ::core::clone::Clone for PATCH_IGNORE_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PATCH_IGNORE_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PATCH_IGNORE_RANGE").field("OffsetInOldFile", &self.OffsetInOldFile).field("LengthInBytes", &self.LengthInBytes).finish()
    }
}
impl ::windows_core::TypeKind for PATCH_IGNORE_RANGE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for PATCH_IGNORE_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.OffsetInOldFile == other.OffsetInOldFile && self.LengthInBytes == other.LengthInBytes
    }
}
impl ::core::cmp::Eq for PATCH_IGNORE_RANGE {}
impl ::core::default::Default for PATCH_IGNORE_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PATCH_INTERLEAVE_MAP {
    pub CountRanges: u32,
    pub Range: [PATCH_INTERLEAVE_MAP_0; 1],
}
impl ::core::marker::Copy for PATCH_INTERLEAVE_MAP {}
impl ::core::clone::Clone for PATCH_INTERLEAVE_MAP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PATCH_INTERLEAVE_MAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PATCH_INTERLEAVE_MAP").field("CountRanges", &self.CountRanges).field("Range", &self.Range).finish()
    }
}
impl ::windows_core::TypeKind for PATCH_INTERLEAVE_MAP {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for PATCH_INTERLEAVE_MAP {
    fn eq(&self, other: &Self) -> bool {
        self.CountRanges == other.CountRanges && self.Range == other.Range
    }
}
impl ::core::cmp::Eq for PATCH_INTERLEAVE_MAP {}
impl ::core::default::Default for PATCH_INTERLEAVE_MAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PATCH_INTERLEAVE_MAP_0 {
    pub OldOffset: u32,
    pub OldLength: u32,
    pub NewLength: u32,
}
impl ::core::marker::Copy for PATCH_INTERLEAVE_MAP_0 {}
impl ::core::clone::Clone for PATCH_INTERLEAVE_MAP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PATCH_INTERLEAVE_MAP_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PATCH_INTERLEAVE_MAP_0").field("OldOffset", &self.OldOffset).field("OldLength", &self.OldLength).field("NewLength", &self.NewLength).finish()
    }
}
impl ::windows_core::TypeKind for PATCH_INTERLEAVE_MAP_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for PATCH_INTERLEAVE_MAP_0 {
    fn eq(&self, other: &Self) -> bool {
        self.OldOffset == other.OldOffset && self.OldLength == other.OldLength && self.NewLength == other.NewLength
    }
}
impl ::core::cmp::Eq for PATCH_INTERLEAVE_MAP_0 {}
impl ::core::default::Default for PATCH_INTERLEAVE_MAP_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PATCH_OLD_FILE_INFO {
    pub SizeOfThisStruct: u32,
    pub Anonymous: PATCH_OLD_FILE_INFO_0,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: *mut PATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: *mut PATCH_RETAIN_RANGE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PATCH_OLD_FILE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PATCH_OLD_FILE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for PATCH_OLD_FILE_INFO {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PATCH_OLD_FILE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union PATCH_OLD_FILE_INFO_0 {
    pub OldFileNameA: ::windows_core::PCSTR,
    pub OldFileNameW: ::windows_core::PCWSTR,
    pub OldFileHandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PATCH_OLD_FILE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PATCH_OLD_FILE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for PATCH_OLD_FILE_INFO_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PATCH_OLD_FILE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PATCH_OLD_FILE_INFO_A {
    pub SizeOfThisStruct: u32,
    pub OldFileName: ::windows_core::PCSTR,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: *mut PATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: *mut PATCH_RETAIN_RANGE,
}
impl ::core::marker::Copy for PATCH_OLD_FILE_INFO_A {}
impl ::core::clone::Clone for PATCH_OLD_FILE_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PATCH_OLD_FILE_INFO_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PATCH_OLD_FILE_INFO_A").field("SizeOfThisStruct", &self.SizeOfThisStruct).field("OldFileName", &self.OldFileName).field("IgnoreRangeCount", &self.IgnoreRangeCount).field("IgnoreRangeArray", &self.IgnoreRangeArray).field("RetainRangeCount", &self.RetainRangeCount).field("RetainRangeArray", &self.RetainRangeArray).finish()
    }
}
impl ::windows_core::TypeKind for PATCH_OLD_FILE_INFO_A {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for PATCH_OLD_FILE_INFO_A {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfThisStruct == other.SizeOfThisStruct && self.OldFileName == other.OldFileName && self.IgnoreRangeCount == other.IgnoreRangeCount && self.IgnoreRangeArray == other.IgnoreRangeArray && self.RetainRangeCount == other.RetainRangeCount && self.RetainRangeArray == other.RetainRangeArray
    }
}
impl ::core::cmp::Eq for PATCH_OLD_FILE_INFO_A {}
impl ::core::default::Default for PATCH_OLD_FILE_INFO_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PATCH_OLD_FILE_INFO_H {
    pub SizeOfThisStruct: u32,
    pub OldFileHandle: super::super::Foundation::HANDLE,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: *mut PATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: *mut PATCH_RETAIN_RANGE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PATCH_OLD_FILE_INFO_H {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PATCH_OLD_FILE_INFO_H {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PATCH_OLD_FILE_INFO_H {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PATCH_OLD_FILE_INFO_H").field("SizeOfThisStruct", &self.SizeOfThisStruct).field("OldFileHandle", &self.OldFileHandle).field("IgnoreRangeCount", &self.IgnoreRangeCount).field("IgnoreRangeArray", &self.IgnoreRangeArray).field("RetainRangeCount", &self.RetainRangeCount).field("RetainRangeArray", &self.RetainRangeArray).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for PATCH_OLD_FILE_INFO_H {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PATCH_OLD_FILE_INFO_H {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfThisStruct == other.SizeOfThisStruct && self.OldFileHandle == other.OldFileHandle && self.IgnoreRangeCount == other.IgnoreRangeCount && self.IgnoreRangeArray == other.IgnoreRangeArray && self.RetainRangeCount == other.RetainRangeCount && self.RetainRangeArray == other.RetainRangeArray
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PATCH_OLD_FILE_INFO_H {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PATCH_OLD_FILE_INFO_H {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PATCH_OLD_FILE_INFO_W {
    pub SizeOfThisStruct: u32,
    pub OldFileName: ::windows_core::PCWSTR,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: *mut PATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: *mut PATCH_RETAIN_RANGE,
}
impl ::core::marker::Copy for PATCH_OLD_FILE_INFO_W {}
impl ::core::clone::Clone for PATCH_OLD_FILE_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PATCH_OLD_FILE_INFO_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PATCH_OLD_FILE_INFO_W").field("SizeOfThisStruct", &self.SizeOfThisStruct).field("OldFileName", &self.OldFileName).field("IgnoreRangeCount", &self.IgnoreRangeCount).field("IgnoreRangeArray", &self.IgnoreRangeArray).field("RetainRangeCount", &self.RetainRangeCount).field("RetainRangeArray", &self.RetainRangeArray).finish()
    }
}
impl ::windows_core::TypeKind for PATCH_OLD_FILE_INFO_W {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for PATCH_OLD_FILE_INFO_W {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfThisStruct == other.SizeOfThisStruct && self.OldFileName == other.OldFileName && self.IgnoreRangeCount == other.IgnoreRangeCount && self.IgnoreRangeArray == other.IgnoreRangeArray && self.RetainRangeCount == other.RetainRangeCount && self.RetainRangeArray == other.RetainRangeArray
    }
}
impl ::core::cmp::Eq for PATCH_OLD_FILE_INFO_W {}
impl ::core::default::Default for PATCH_OLD_FILE_INFO_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PATCH_OPTION_DATA {
    pub SizeOfThisStruct: u32,
    pub SymbolOptionFlags: u32,
    pub NewFileSymbolPath: ::windows_core::PCSTR,
    pub OldFileSymbolPathArray: *const ::windows_core::PCSTR,
    pub ExtendedOptionFlags: u32,
    pub SymLoadCallback: PPATCH_SYMLOAD_CALLBACK,
    pub SymLoadContext: *mut ::core::ffi::c_void,
    pub InterleaveMapArray: *mut *mut PATCH_INTERLEAVE_MAP,
    pub MaxLzxWindowSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PATCH_OPTION_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PATCH_OPTION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PATCH_OPTION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PATCH_OPTION_DATA").field("SizeOfThisStruct", &self.SizeOfThisStruct).field("SymbolOptionFlags", &self.SymbolOptionFlags).field("NewFileSymbolPath", &self.NewFileSymbolPath).field("OldFileSymbolPathArray", &self.OldFileSymbolPathArray).field("ExtendedOptionFlags", &self.ExtendedOptionFlags).field("SymLoadContext", &self.SymLoadContext).field("InterleaveMapArray", &self.InterleaveMapArray).field("MaxLzxWindowSize", &self.MaxLzxWindowSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for PATCH_OPTION_DATA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PATCH_OPTION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PATCH_RETAIN_RANGE {
    pub OffsetInOldFile: u32,
    pub LengthInBytes: u32,
    pub OffsetInNewFile: u32,
}
impl ::core::marker::Copy for PATCH_RETAIN_RANGE {}
impl ::core::clone::Clone for PATCH_RETAIN_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PATCH_RETAIN_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PATCH_RETAIN_RANGE").field("OffsetInOldFile", &self.OffsetInOldFile).field("LengthInBytes", &self.LengthInBytes).field("OffsetInNewFile", &self.OffsetInNewFile).finish()
    }
}
impl ::windows_core::TypeKind for PATCH_RETAIN_RANGE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for PATCH_RETAIN_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.OffsetInOldFile == other.OffsetInOldFile && self.LengthInBytes == other.LengthInBytes && self.OffsetInNewFile == other.OffsetInNewFile
    }
}
impl ::core::cmp::Eq for PATCH_RETAIN_RANGE {}
impl ::core::default::Default for PATCH_RETAIN_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PMSIHANDLE {
    pub m_h: MSIHANDLE,
}
impl ::core::marker::Copy for PMSIHANDLE {}
impl ::core::clone::Clone for PMSIHANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PMSIHANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PMSIHANDLE").field("m_h", &self.m_h).finish()
    }
}
impl ::windows_core::TypeKind for PMSIHANDLE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for PMSIHANDLE {
    fn eq(&self, other: &Self) -> bool {
        self.m_h == other.m_h
    }
}
impl ::core::cmp::Eq for PMSIHANDLE {}
impl ::core::default::Default for PMSIHANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PM_APPTASKTYPE {
    pub ProductID: ::windows_core::GUID,
    pub TaskType: PM_TASK_TYPE,
}
impl ::core::marker::Copy for PM_APPTASKTYPE {}
impl ::core::clone::Clone for PM_APPTASKTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PM_APPTASKTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PM_APPTASKTYPE").field("ProductID", &self.ProductID).field("TaskType", &self.TaskType).finish()
    }
}
impl ::windows_core::TypeKind for PM_APPTASKTYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for PM_APPTASKTYPE {
    fn eq(&self, other: &Self) -> bool {
        self.ProductID == other.ProductID && self.TaskType == other.TaskType
    }
}
impl ::core::cmp::Eq for PM_APPTASKTYPE {}
impl ::core::default::Default for PM_APPTASKTYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PM_BSATASKID {
    pub ProductID: ::windows_core::GUID,
    pub TaskID: ::std::mem::ManuallyDrop<::windows_core::BSTR>,
}
impl ::core::clone::Clone for PM_BSATASKID {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for PM_BSATASKID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PM_BSATASKID").field("ProductID", &self.ProductID).field("TaskID", &self.TaskID).finish()
    }
}
impl ::windows_core::TypeKind for PM_BSATASKID {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for PM_BSATASKID {
    fn eq(&self, other: &Self) -> bool {
        self.ProductID == other.ProductID && self.TaskID == other.TaskID
    }
}
impl ::core::cmp::Eq for PM_BSATASKID {}
impl ::core::default::Default for PM_BSATASKID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PM_BWTASKID {
    pub ProductID: ::windows_core::GUID,
    pub TaskID: ::std::mem::ManuallyDrop<::windows_core::BSTR>,
}
impl ::core::clone::Clone for PM_BWTASKID {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for PM_BWTASKID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PM_BWTASKID").field("ProductID", &self.ProductID).field("TaskID", &self.TaskID).finish()
    }
}
impl ::windows_core::TypeKind for PM_BWTASKID {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for PM_BWTASKID {
    fn eq(&self, other: &Self) -> bool {
        self.ProductID == other.ProductID && self.TaskID == other.TaskID
    }
}
impl ::core::cmp::Eq for PM_BWTASKID {}
impl ::core::default::Default for PM_BWTASKID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PM_ENUM_FILTER {
    pub FilterType: i32,
    pub FilterParameter: PM_ENUM_FILTER_0,
}
impl ::core::clone::Clone for PM_ENUM_FILTER {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::windows_core::TypeKind for PM_ENUM_FILTER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for PM_ENUM_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub union PM_ENUM_FILTER_0 {
    pub Dummy: i32,
    pub Genre: PM_APP_GENRE,
    pub AppHubType: PM_APPLICATION_HUBTYPE,
    pub HubType: PM_TILE_HUBTYPE,
    pub Tasktype: PM_TASK_TYPE,
    pub TaskProductID: ::windows_core::GUID,
    pub TileProductID: ::windows_core::GUID,
    pub AppTaskType: PM_APPTASKTYPE,
    pub Consumer: ::std::mem::ManuallyDrop<PM_EXTENSIONCONSUMER>,
    pub BSATask: ::std::mem::ManuallyDrop<PM_BSATASKID>,
    pub BSAProductID: ::windows_core::GUID,
    pub BWTask: ::std::mem::ManuallyDrop<PM_BWTASKID>,
    pub ProtocolName: ::std::mem::ManuallyDrop<::windows_core::BSTR>,
    pub FileType: ::std::mem::ManuallyDrop<::windows_core::BSTR>,
    pub ContentType: ::std::mem::ManuallyDrop<::windows_core::BSTR>,
    pub AppSupportedFileExtPID: ::windows_core::GUID,
    pub ShareTargetFileType: ::std::mem::ManuallyDrop<::windows_core::BSTR>,
}
impl ::core::clone::Clone for PM_ENUM_FILTER_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::windows_core::TypeKind for PM_ENUM_FILTER_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for PM_ENUM_FILTER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PM_EXTENSIONCONSUMER {
    pub ConsumerPID: ::windows_core::GUID,
    pub ExtensionID: ::std::mem::ManuallyDrop<::windows_core::BSTR>,
}
impl ::core::clone::Clone for PM_EXTENSIONCONSUMER {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for PM_EXTENSIONCONSUMER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PM_EXTENSIONCONSUMER").field("ConsumerPID", &self.ConsumerPID).field("ExtensionID", &self.ExtensionID).finish()
    }
}
impl ::windows_core::TypeKind for PM_EXTENSIONCONSUMER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for PM_EXTENSIONCONSUMER {
    fn eq(&self, other: &Self) -> bool {
        self.ConsumerPID == other.ConsumerPID && self.ExtensionID == other.ExtensionID
    }
}
impl ::core::cmp::Eq for PM_EXTENSIONCONSUMER {}
impl ::core::default::Default for PM_EXTENSIONCONSUMER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_INSTALLINFO {
    pub ProductID: ::windows_core::GUID,
    pub PackagePath: ::std::mem::ManuallyDrop<::windows_core::BSTR>,
    pub InstanceID: ::windows_core::GUID,
    pub pbLicense: *mut u8,
    pub cbLicense: u32,
    pub IsUninstallDisabled: super::super::Foundation::BOOL,
    pub DeploymentOptions: u32,
    pub OfferID: ::windows_core::GUID,
    pub MarketplaceAppVersion: ::std::mem::ManuallyDrop<::windows_core::BSTR>,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PM_INSTALLINFO {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PM_INSTALLINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PM_INSTALLINFO").field("ProductID", &self.ProductID).field("PackagePath", &self.PackagePath).field("InstanceID", &self.InstanceID).field("pbLicense", &self.pbLicense).field("cbLicense", &self.cbLicense).field("IsUninstallDisabled", &self.IsUninstallDisabled).field("DeploymentOptions", &self.DeploymentOptions).field("OfferID", &self.OfferID).field("MarketplaceAppVersion", &self.MarketplaceAppVersion).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for PM_INSTALLINFO {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PM_INSTALLINFO {
    fn eq(&self, other: &Self) -> bool {
        self.ProductID == other.ProductID && self.PackagePath == other.PackagePath && self.InstanceID == other.InstanceID && self.pbLicense == other.pbLicense && self.cbLicense == other.cbLicense && self.IsUninstallDisabled == other.IsUninstallDisabled && self.DeploymentOptions == other.DeploymentOptions && self.OfferID == other.OfferID && self.MarketplaceAppVersion == other.MarketplaceAppVersion
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PM_INSTALLINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PM_INSTALLINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PM_INVOCATIONINFO {
    pub URIBaseOrAUMID: ::std::mem::ManuallyDrop<::windows_core::BSTR>,
    pub URIFragmentOrArgs: ::std::mem::ManuallyDrop<::windows_core::BSTR>,
}
impl ::core::clone::Clone for PM_INVOCATIONINFO {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for PM_INVOCATIONINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PM_INVOCATIONINFO").field("URIBaseOrAUMID", &self.URIBaseOrAUMID).field("URIFragmentOrArgs", &self.URIFragmentOrArgs).finish()
    }
}
impl ::windows_core::TypeKind for PM_INVOCATIONINFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for PM_INVOCATIONINFO {
    fn eq(&self, other: &Self) -> bool {
        self.URIBaseOrAUMID == other.URIBaseOrAUMID && self.URIFragmentOrArgs == other.URIFragmentOrArgs
    }
}
impl ::core::cmp::Eq for PM_INVOCATIONINFO {}
impl ::core::default::Default for PM_INVOCATIONINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_STARTAPPBLOB {
    pub cbSize: u32,
    pub ProductID: ::windows_core::GUID,
    pub AppTitle: ::std::mem::ManuallyDrop<::windows_core::BSTR>,
    pub IconPath: ::std::mem::ManuallyDrop<::windows_core::BSTR>,
    pub IsUninstallable: super::super::Foundation::BOOL,
    pub AppInstallType: PM_APPLICATION_INSTALL_TYPE,
    pub InstanceID: ::windows_core::GUID,
    pub State: PM_APPLICATION_STATE,
    pub IsModern: super::super::Foundation::BOOL,
    pub IsModernLightUp: super::super::Foundation::BOOL,
    pub LightUpSupportMask: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PM_STARTAPPBLOB {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PM_STARTAPPBLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PM_STARTAPPBLOB").field("cbSize", &self.cbSize).field("ProductID", &self.ProductID).field("AppTitle", &self.AppTitle).field("IconPath", &self.IconPath).field("IsUninstallable", &self.IsUninstallable).field("AppInstallType", &self.AppInstallType).field("InstanceID", &self.InstanceID).field("State", &self.State).field("IsModern", &self.IsModern).field("IsModernLightUp", &self.IsModernLightUp).field("LightUpSupportMask", &self.LightUpSupportMask).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for PM_STARTAPPBLOB {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PM_STARTAPPBLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.ProductID == other.ProductID && self.AppTitle == other.AppTitle && self.IconPath == other.IconPath && self.IsUninstallable == other.IsUninstallable && self.AppInstallType == other.AppInstallType && self.InstanceID == other.InstanceID && self.State == other.State && self.IsModern == other.IsModern && self.IsModernLightUp == other.IsModernLightUp && self.LightUpSupportMask == other.LightUpSupportMask
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PM_STARTAPPBLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PM_STARTAPPBLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_STARTTILEBLOB {
    pub cbSize: u32,
    pub ProductID: ::windows_core::GUID,
    pub TileID: ::std::mem::ManuallyDrop<::windows_core::BSTR>,
    pub TemplateType: TILE_TEMPLATE_TYPE,
    pub HubPosition: [u32; 32],
    pub HubVisibilityBitmask: u32,
    pub IsDefault: super::super::Foundation::BOOL,
    pub TileType: PM_STARTTILE_TYPE,
    pub pbPropBlob: *mut u8,
    pub cbPropBlob: u32,
    pub IsRestoring: super::super::Foundation::BOOL,
    pub IsModern: super::super::Foundation::BOOL,
    pub InvocationInfo: PM_INVOCATIONINFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PM_STARTTILEBLOB {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PM_STARTTILEBLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PM_STARTTILEBLOB")
            .field("cbSize", &self.cbSize)
            .field("ProductID", &self.ProductID)
            .field("TileID", &self.TileID)
            .field("TemplateType", &self.TemplateType)
            .field("HubPosition", &self.HubPosition)
            .field("HubVisibilityBitmask", &self.HubVisibilityBitmask)
            .field("IsDefault", &self.IsDefault)
            .field("TileType", &self.TileType)
            .field("pbPropBlob", &self.pbPropBlob)
            .field("cbPropBlob", &self.cbPropBlob)
            .field("IsRestoring", &self.IsRestoring)
            .field("IsModern", &self.IsModern)
            .field("InvocationInfo", &self.InvocationInfo)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for PM_STARTTILEBLOB {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PM_STARTTILEBLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.ProductID == other.ProductID && self.TileID == other.TileID && self.TemplateType == other.TemplateType && self.HubPosition == other.HubPosition && self.HubVisibilityBitmask == other.HubVisibilityBitmask && self.IsDefault == other.IsDefault && self.TileType == other.TileType && self.pbPropBlob == other.pbPropBlob && self.cbPropBlob == other.cbPropBlob && self.IsRestoring == other.IsRestoring && self.IsModern == other.IsModern && self.InvocationInfo == other.InvocationInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PM_STARTTILEBLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PM_STARTTILEBLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PM_UPDATEINFO {
    pub ProductID: ::windows_core::GUID,
    pub PackagePath: ::std::mem::ManuallyDrop<::windows_core::BSTR>,
    pub InstanceID: ::windows_core::GUID,
    pub pbLicense: *mut u8,
    pub cbLicense: u32,
    pub MarketplaceAppVersion: ::std::mem::ManuallyDrop<::windows_core::BSTR>,
    pub DeploymentOptions: u32,
}
impl ::core::clone::Clone for PM_UPDATEINFO {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for PM_UPDATEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PM_UPDATEINFO").field("ProductID", &self.ProductID).field("PackagePath", &self.PackagePath).field("InstanceID", &self.InstanceID).field("pbLicense", &self.pbLicense).field("cbLicense", &self.cbLicense).field("MarketplaceAppVersion", &self.MarketplaceAppVersion).field("DeploymentOptions", &self.DeploymentOptions).finish()
    }
}
impl ::windows_core::TypeKind for PM_UPDATEINFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for PM_UPDATEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.ProductID == other.ProductID && self.PackagePath == other.PackagePath && self.InstanceID == other.InstanceID && self.pbLicense == other.pbLicense && self.cbLicense == other.cbLicense && self.MarketplaceAppVersion == other.MarketplaceAppVersion && self.DeploymentOptions == other.DeploymentOptions
    }
}
impl ::core::cmp::Eq for PM_UPDATEINFO {}
impl ::core::default::Default for PM_UPDATEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PM_UPDATEINFO_LEGACY {
    pub ProductID: ::windows_core::GUID,
    pub PackagePath: ::std::mem::ManuallyDrop<::windows_core::BSTR>,
    pub InstanceID: ::windows_core::GUID,
    pub pbLicense: *mut u8,
    pub cbLicense: u32,
    pub MarketplaceAppVersion: ::std::mem::ManuallyDrop<::windows_core::BSTR>,
}
impl ::core::clone::Clone for PM_UPDATEINFO_LEGACY {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for PM_UPDATEINFO_LEGACY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PM_UPDATEINFO_LEGACY").field("ProductID", &self.ProductID).field("PackagePath", &self.PackagePath).field("InstanceID", &self.InstanceID).field("pbLicense", &self.pbLicense).field("cbLicense", &self.cbLicense).field("MarketplaceAppVersion", &self.MarketplaceAppVersion).finish()
    }
}
impl ::windows_core::TypeKind for PM_UPDATEINFO_LEGACY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for PM_UPDATEINFO_LEGACY {
    fn eq(&self, other: &Self) -> bool {
        self.ProductID == other.ProductID && self.PackagePath == other.PackagePath && self.InstanceID == other.InstanceID && self.pbLicense == other.pbLicense && self.cbLicense == other.cbLicense && self.MarketplaceAppVersion == other.MarketplaceAppVersion
    }
}
impl ::core::cmp::Eq for PM_UPDATEINFO_LEGACY {}
impl ::core::default::Default for PM_UPDATEINFO_LEGACY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub struct PROTECTED_FILE_DATA {
    pub FileName: [u16; 260],
    pub FileNumber: u32,
}
impl ::core::marker::Copy for PROTECTED_FILE_DATA {}
impl ::core::clone::Clone for PROTECTED_FILE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROTECTED_FILE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROTECTED_FILE_DATA").field("FileName", &self.FileName).field("FileNumber", &self.FileNumber).finish()
    }
}
impl ::windows_core::TypeKind for PROTECTED_FILE_DATA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for PROTECTED_FILE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.FileName == other.FileName && self.FileNumber == other.FileNumber
    }
}
impl ::core::cmp::Eq for PROTECTED_FILE_DATA {}
impl ::core::default::Default for PROTECTED_FILE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type INSTALLUI_HANDLERA = ::core::option::Option<unsafe extern "system" fn(pvcontext: *mut ::core::ffi::c_void, imessagetype: u32, szmessage: ::windows_core::PCSTR) -> i32>;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type INSTALLUI_HANDLERW = ::core::option::Option<unsafe extern "system" fn(pvcontext: *mut ::core::ffi::c_void, imessagetype: u32, szmessage: ::windows_core::PCWSTR) -> i32>;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPDISPLAYVAL = ::core::option::Option<unsafe extern "system" fn(pcontext: *mut ::core::ffi::c_void, uitype: RESULTTYPES, szwval: ::windows_core::PCWSTR, szwdescription: ::windows_core::PCWSTR, szwlocation: ::windows_core::PCWSTR) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPEVALCOMCALLBACK = ::core::option::Option<unsafe extern "system" fn(istatus: STATUSTYPES, szdata: ::windows_core::PCWSTR, pcontext: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`*"]
pub type PINSTALLUI_HANDLER_RECORD = ::core::option::Option<unsafe extern "system" fn(pvcontext: *mut ::core::ffi::c_void, imessagetype: u32, hrecord: MSIHANDLE) -> i32>;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PPATCH_PROGRESS_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callbackcontext: *mut ::core::ffi::c_void, currentposition: u32, maximumposition: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_ApplicationInstallationAndServicing\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PPATCH_SYMLOAD_CALLBACK = ::core::option::Option<unsafe extern "system" fn(whichfile: u32, symbolfilename: ::windows_core::PCSTR, symtype: u32, symbolfilechecksum: u32, symbolfiletimedate: u32, imagefilechecksum: u32, imagefiletimedate: u32, callbackcontext: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
