#[inline]
pub unsafe fn ApplyPatchToFileA<P0, P1, P2>(patchfilename: P0, oldfilename: P1, newfilename: P2, applyoptionflags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mspatcha.dll" "system" fn ApplyPatchToFileA(patchfilename : windows_core::PCSTR, oldfilename : windows_core::PCSTR, newfilename : windows_core::PCSTR, applyoptionflags : u32) -> windows_core::BOOL);
    unsafe { ApplyPatchToFileA(patchfilename.param().abi(), oldfilename.param().abi(), newfilename.param().abi(), applyoptionflags) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ApplyPatchToFileByBuffers(patchfilemapped: &[u8], oldfilemapped: Option<&[u8]>, newfilebuffer: *mut super::minwindef::PBYTE, newfilebuffersize: u32, newfileactualsize: Option<*mut u32>, newfiletime: Option<*mut super::minwindef::FILETIME>, applyoptionflags: u32, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("mspatcha.dll" "system" fn ApplyPatchToFileByBuffers(patchfilemapped : *const u8, patchfilesize : u32, oldfilemapped : *const u8, oldfilesize : u32, newfilebuffer : *mut super::minwindef::PBYTE, newfilebuffersize : u32, newfileactualsize : *mut u32, newfiletime : *mut super::minwindef::FILETIME, applyoptionflags : u32, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { ApplyPatchToFileByBuffers(core::mem::transmute(patchfilemapped.as_ptr()), patchfilemapped.len().try_into().unwrap(), core::mem::transmute(oldfilemapped.map_or(core::ptr::null(), |slice| slice.as_ptr())), oldfilemapped.map_or(0, |slice| slice.len().try_into().unwrap()), newfilebuffer as _, newfilebuffersize, newfileactualsize.unwrap_or(core::mem::zeroed()) as _, newfiletime.unwrap_or(core::mem::zeroed()) as _, applyoptionflags, progresscallback, callbackcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ApplyPatchToFileByHandles(patchfilehandle: super::winnt::HANDLE, oldfilehandle: Option<super::winnt::HANDLE>, newfilehandle: super::winnt::HANDLE, applyoptionflags: u32) -> windows_core::BOOL {
    windows_core::link!("mspatcha.dll" "system" fn ApplyPatchToFileByHandles(patchfilehandle : super::winnt::HANDLE, oldfilehandle : super::winnt::HANDLE, newfilehandle : super::winnt::HANDLE, applyoptionflags : u32) -> windows_core::BOOL);
    unsafe { ApplyPatchToFileByHandles(patchfilehandle, oldfilehandle.unwrap_or(core::mem::zeroed()) as _, newfilehandle, applyoptionflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ApplyPatchToFileByHandlesEx(patchfilehandle: super::winnt::HANDLE, oldfilehandle: Option<super::winnt::HANDLE>, newfilehandle: super::winnt::HANDLE, applyoptionflags: u32, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("mspatcha.dll" "system" fn ApplyPatchToFileByHandlesEx(patchfilehandle : super::winnt::HANDLE, oldfilehandle : super::winnt::HANDLE, newfilehandle : super::winnt::HANDLE, applyoptionflags : u32, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { ApplyPatchToFileByHandlesEx(patchfilehandle, oldfilehandle.unwrap_or(core::mem::zeroed()) as _, newfilehandle, applyoptionflags, progresscallback, callbackcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ApplyPatchToFileExA<P0, P1, P2>(patchfilename: P0, oldfilename: P1, newfilename: P2, applyoptionflags: u32, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mspatcha.dll" "system" fn ApplyPatchToFileExA(patchfilename : windows_core::PCSTR, oldfilename : windows_core::PCSTR, newfilename : windows_core::PCSTR, applyoptionflags : u32, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { ApplyPatchToFileExA(patchfilename.param().abi(), oldfilename.param().abi(), newfilename.param().abi(), applyoptionflags, progresscallback, callbackcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ApplyPatchToFileExW<P0, P1, P2>(patchfilename: P0, oldfilename: P1, newfilename: P2, applyoptionflags: u32, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mspatcha.dll" "system" fn ApplyPatchToFileExW(patchfilename : windows_core::PCWSTR, oldfilename : windows_core::PCWSTR, newfilename : windows_core::PCWSTR, applyoptionflags : u32, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { ApplyPatchToFileExW(patchfilename.param().abi(), oldfilename.param().abi(), newfilename.param().abi(), applyoptionflags, progresscallback, callbackcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ApplyPatchToFileW<P0, P1, P2>(patchfilename: P0, oldfilename: P1, newfilename: P2, applyoptionflags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mspatcha.dll" "system" fn ApplyPatchToFileW(patchfilename : windows_core::PCWSTR, oldfilename : windows_core::PCWSTR, newfilename : windows_core::PCWSTR, applyoptionflags : u32) -> windows_core::BOOL);
    unsafe { ApplyPatchToFileW(patchfilename.param().abi(), oldfilename.param().abi(), newfilename.param().abi(), applyoptionflags) }
}
#[inline]
pub unsafe fn CreatePatchFileA<P0, P1, P2>(oldfilename: P0, newfilename: P1, patchfilename: P2, optionflags: u32, optiondata: Option<*const PATCH_OPTION_DATA>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mspatchc.dll" "system" fn CreatePatchFileA(oldfilename : windows_core::PCSTR, newfilename : windows_core::PCSTR, patchfilename : windows_core::PCSTR, optionflags : u32, optiondata : *const PATCH_OPTION_DATA) -> windows_core::BOOL);
    unsafe { CreatePatchFileA(oldfilename.param().abi(), newfilename.param().abi(), patchfilename.param().abi(), optionflags, optiondata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CreatePatchFileByHandles(oldfilehandle: Option<super::winnt::HANDLE>, newfilehandle: super::winnt::HANDLE, patchfilehandle: super::winnt::HANDLE, optionflags: u32, optiondata: Option<*const PATCH_OPTION_DATA>) -> windows_core::BOOL {
    windows_core::link!("mspatchc.dll" "system" fn CreatePatchFileByHandles(oldfilehandle : super::winnt::HANDLE, newfilehandle : super::winnt::HANDLE, patchfilehandle : super::winnt::HANDLE, optionflags : u32, optiondata : *const PATCH_OPTION_DATA) -> windows_core::BOOL);
    unsafe { CreatePatchFileByHandles(oldfilehandle.unwrap_or(core::mem::zeroed()) as _, newfilehandle, patchfilehandle, optionflags, optiondata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CreatePatchFileByHandlesEx(oldfileinfoarray: &[PATCH_OLD_FILE_INFO_H], newfilehandle: super::winnt::HANDLE, patchfilehandle: super::winnt::HANDLE, optionflags: u32, optiondata: Option<*const PATCH_OPTION_DATA>, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("mspatchc.dll" "system" fn CreatePatchFileByHandlesEx(oldfilecount : u32, oldfileinfoarray : *const PATCH_OLD_FILE_INFO_H, newfilehandle : super::winnt::HANDLE, patchfilehandle : super::winnt::HANDLE, optionflags : u32, optiondata : *const PATCH_OPTION_DATA, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CreatePatchFileByHandlesEx(oldfileinfoarray.len().try_into().unwrap(), core::mem::transmute(oldfileinfoarray.as_ptr()), newfilehandle, patchfilehandle, optionflags, optiondata.unwrap_or(core::mem::zeroed()) as _, progresscallback, callbackcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CreatePatchFileExA<P2, P3>(oldfileinfoarray: &[PATCH_OLD_FILE_INFO_A], newfilename: P2, patchfilename: P3, optionflags: u32, optiondata: Option<*const PATCH_OPTION_DATA>, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mspatchc.dll" "system" fn CreatePatchFileExA(oldfilecount : u32, oldfileinfoarray : *const PATCH_OLD_FILE_INFO_A, newfilename : windows_core::PCSTR, patchfilename : windows_core::PCSTR, optionflags : u32, optiondata : *const PATCH_OPTION_DATA, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CreatePatchFileExA(oldfileinfoarray.len().try_into().unwrap(), core::mem::transmute(oldfileinfoarray.as_ptr()), newfilename.param().abi(), patchfilename.param().abi(), optionflags, optiondata.unwrap_or(core::mem::zeroed()) as _, progresscallback, callbackcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CreatePatchFileExW<P2, P3>(oldfileinfoarray: &[PATCH_OLD_FILE_INFO_W], newfilename: P2, patchfilename: P3, optionflags: u32, optiondata: Option<*const PATCH_OPTION_DATA>, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mspatchc.dll" "system" fn CreatePatchFileExW(oldfilecount : u32, oldfileinfoarray : *const PATCH_OLD_FILE_INFO_W, newfilename : windows_core::PCWSTR, patchfilename : windows_core::PCWSTR, optionflags : u32, optiondata : *const PATCH_OPTION_DATA, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CreatePatchFileExW(oldfileinfoarray.len().try_into().unwrap(), core::mem::transmute(oldfileinfoarray.as_ptr()), newfilename.param().abi(), patchfilename.param().abi(), optionflags, optiondata.unwrap_or(core::mem::zeroed()) as _, progresscallback, callbackcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CreatePatchFileW<P0, P1, P2>(oldfilename: P0, newfilename: P1, patchfilename: P2, optionflags: u32, optiondata: Option<*const PATCH_OPTION_DATA>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mspatchc.dll" "system" fn CreatePatchFileW(oldfilename : windows_core::PCWSTR, newfilename : windows_core::PCWSTR, patchfilename : windows_core::PCWSTR, optionflags : u32, optiondata : *const PATCH_OPTION_DATA) -> windows_core::BOOL);
    unsafe { CreatePatchFileW(oldfilename.param().abi(), newfilename.param().abi(), patchfilename.param().abi(), optionflags, optiondata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ExtractPatchHeaderToFileA<P0, P1>(patchfilename: P0, patchheaderfilename: P1) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mspatchc.dll" "system" fn ExtractPatchHeaderToFileA(patchfilename : windows_core::PCSTR, patchheaderfilename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { ExtractPatchHeaderToFileA(patchfilename.param().abi(), patchheaderfilename.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ExtractPatchHeaderToFileByHandles(patchfilehandle: super::winnt::HANDLE, patchheaderfilehandle: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("mspatchc.dll" "system" fn ExtractPatchHeaderToFileByHandles(patchfilehandle : super::winnt::HANDLE, patchheaderfilehandle : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { ExtractPatchHeaderToFileByHandles(patchfilehandle, patchheaderfilehandle) }
}
#[inline]
pub unsafe fn ExtractPatchHeaderToFileW<P0, P1>(patchfilename: P0, patchheaderfilename: P1) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mspatchc.dll" "system" fn ExtractPatchHeaderToFileW(patchfilename : windows_core::PCWSTR, patchheaderfilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { ExtractPatchHeaderToFileW(patchfilename.param().abi(), patchheaderfilename.param().abi()) }
}
#[inline]
pub unsafe fn GetFilePatchSignatureA<P0>(filename: P0, optionflags: u32, optiondata: Option<*const core::ffi::c_void>, ignorerangearray: Option<&[PATCH_IGNORE_RANGE]>, retainrangearray: Option<&[PATCH_RETAIN_RANGE]>, signaturebuffer: &mut [u8]) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mspatchc.dll" "system" fn GetFilePatchSignatureA(filename : windows_core::PCSTR, optionflags : u32, optiondata : *const core::ffi::c_void, ignorerangecount : u32, ignorerangearray : *const PATCH_IGNORE_RANGE, retainrangecount : u32, retainrangearray : *const PATCH_RETAIN_RANGE, signaturebuffersize : u32, signaturebuffer : windows_core::PSTR) -> windows_core::BOOL);
    unsafe {
        GetFilePatchSignatureA(
            filename.param().abi(),
            optionflags,
            optiondata.unwrap_or(core::mem::zeroed()) as _,
            ignorerangearray.map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(ignorerangearray.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            retainrangearray.map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(retainrangearray.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            signaturebuffer.len().try_into().unwrap(),
            core::mem::transmute(signaturebuffer.as_ptr()),
        )
    }
}
#[inline]
pub unsafe fn GetFilePatchSignatureByBuffer(filebufferwritable: &mut [u8], optionflags: u32, optiondata: Option<*const core::ffi::c_void>, ignorerangearray: Option<&[PATCH_IGNORE_RANGE]>, retainrangearray: Option<&[PATCH_RETAIN_RANGE]>, signaturebuffer: &mut [u8]) -> windows_core::BOOL {
    windows_core::link!("mspatchc.dll" "system" fn GetFilePatchSignatureByBuffer(filebufferwritable : *mut u8, filesize : u32, optionflags : u32, optiondata : *const core::ffi::c_void, ignorerangecount : u32, ignorerangearray : *const PATCH_IGNORE_RANGE, retainrangecount : u32, retainrangearray : *const PATCH_RETAIN_RANGE, signaturebuffersize : u32, signaturebuffer : windows_core::PSTR) -> windows_core::BOOL);
    unsafe {
        GetFilePatchSignatureByBuffer(
            core::mem::transmute(filebufferwritable.as_ptr()),
            filebufferwritable.len().try_into().unwrap(),
            optionflags,
            optiondata.unwrap_or(core::mem::zeroed()) as _,
            ignorerangearray.map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(ignorerangearray.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            retainrangearray.map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(retainrangearray.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            signaturebuffer.len().try_into().unwrap(),
            core::mem::transmute(signaturebuffer.as_ptr()),
        )
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetFilePatchSignatureByHandle(filehandle: super::winnt::HANDLE, optionflags: u32, optiondata: Option<*const core::ffi::c_void>, ignorerangearray: Option<&[PATCH_IGNORE_RANGE]>, retainrangearray: Option<&[PATCH_RETAIN_RANGE]>, signaturebuffer: &mut [u8]) -> windows_core::BOOL {
    windows_core::link!("mspatchc.dll" "system" fn GetFilePatchSignatureByHandle(filehandle : super::winnt::HANDLE, optionflags : u32, optiondata : *const core::ffi::c_void, ignorerangecount : u32, ignorerangearray : *const PATCH_IGNORE_RANGE, retainrangecount : u32, retainrangearray : *const PATCH_RETAIN_RANGE, signaturebuffersize : u32, signaturebuffer : windows_core::PSTR) -> windows_core::BOOL);
    unsafe { GetFilePatchSignatureByHandle(filehandle, optionflags, optiondata.unwrap_or(core::mem::zeroed()) as _, ignorerangearray.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ignorerangearray.map_or(core::ptr::null(), |slice| slice.as_ptr())), retainrangearray.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(retainrangearray.map_or(core::ptr::null(), |slice| slice.as_ptr())), signaturebuffer.len().try_into().unwrap(), core::mem::transmute(signaturebuffer.as_ptr())) }
}
#[inline]
pub unsafe fn GetFilePatchSignatureW<P0>(filename: P0, optionflags: u32, optiondata: Option<*const core::ffi::c_void>, ignorerangearray: Option<&[PATCH_IGNORE_RANGE]>, retainrangearray: Option<&[PATCH_RETAIN_RANGE]>, signaturebuffersize: u32, signaturebuffer: windows_core::PWSTR) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mspatchc.dll" "system" fn GetFilePatchSignatureW(filename : windows_core::PCWSTR, optionflags : u32, optiondata : *const core::ffi::c_void, ignorerangecount : u32, ignorerangearray : *const PATCH_IGNORE_RANGE, retainrangecount : u32, retainrangearray : *const PATCH_RETAIN_RANGE, signaturebuffersize : u32, signaturebuffer : windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { GetFilePatchSignatureW(filename.param().abi(), optionflags, optiondata.unwrap_or(core::mem::zeroed()) as _, ignorerangearray.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ignorerangearray.map_or(core::ptr::null(), |slice| slice.as_ptr())), retainrangearray.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(retainrangearray.map_or(core::ptr::null(), |slice| slice.as_ptr())), signaturebuffersize, core::mem::transmute(signaturebuffer)) }
}
#[inline]
pub unsafe fn NormalizeFileForPatchSignature(filebuffer: *mut core::ffi::c_void, filesize: u32, optionflags: u32, optiondata: Option<*const PATCH_OPTION_DATA>, newfilecoffbase: u32, newfilecofftime: u32, ignorerangearray: Option<&[PATCH_IGNORE_RANGE]>, retainrangearray: Option<&[PATCH_RETAIN_RANGE]>) -> i32 {
    windows_core::link!("mspatchc.dll" "system" fn NormalizeFileForPatchSignature(filebuffer : *mut core::ffi::c_void, filesize : u32, optionflags : u32, optiondata : *const PATCH_OPTION_DATA, newfilecoffbase : u32, newfilecofftime : u32, ignorerangecount : u32, ignorerangearray : *const PATCH_IGNORE_RANGE, retainrangecount : u32, retainrangearray : *const PATCH_RETAIN_RANGE) -> i32);
    unsafe { NormalizeFileForPatchSignature(filebuffer as _, filesize, optionflags, optiondata.unwrap_or(core::mem::zeroed()) as _, newfilecoffbase, newfilecofftime, ignorerangearray.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ignorerangearray.map_or(core::ptr::null(), |slice| slice.as_ptr())), retainrangearray.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(retainrangearray.map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[inline]
pub unsafe fn TestApplyPatchToFileA<P0, P1>(patchfilename: P0, oldfilename: P1, applyoptionflags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mspatcha.dll" "system" fn TestApplyPatchToFileA(patchfilename : windows_core::PCSTR, oldfilename : windows_core::PCSTR, applyoptionflags : u32) -> windows_core::BOOL);
    unsafe { TestApplyPatchToFileA(patchfilename.param().abi(), oldfilename.param().abi(), applyoptionflags) }
}
#[inline]
pub unsafe fn TestApplyPatchToFileByBuffers(patchfilebuffer: &[u8], oldfilebuffer: Option<&[u8]>, newfilesize: Option<*mut u32>, applyoptionflags: u32) -> windows_core::BOOL {
    windows_core::link!("mspatcha.dll" "system" fn TestApplyPatchToFileByBuffers(patchfilebuffer : *const u8, patchfilesize : u32, oldfilebuffer : *const u8, oldfilesize : u32, newfilesize : *mut u32, applyoptionflags : u32) -> windows_core::BOOL);
    unsafe { TestApplyPatchToFileByBuffers(core::mem::transmute(patchfilebuffer.as_ptr()), patchfilebuffer.len().try_into().unwrap(), core::mem::transmute(oldfilebuffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), oldfilebuffer.map_or(0, |slice| slice.len().try_into().unwrap()), newfilesize.unwrap_or(core::mem::zeroed()) as _, applyoptionflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn TestApplyPatchToFileByHandles(patchfilehandle: super::winnt::HANDLE, oldfilehandle: super::winnt::HANDLE, applyoptionflags: u32) -> windows_core::BOOL {
    windows_core::link!("mspatcha.dll" "system" fn TestApplyPatchToFileByHandles(patchfilehandle : super::winnt::HANDLE, oldfilehandle : super::winnt::HANDLE, applyoptionflags : u32) -> windows_core::BOOL);
    unsafe { TestApplyPatchToFileByHandles(patchfilehandle, oldfilehandle, applyoptionflags) }
}
#[inline]
pub unsafe fn TestApplyPatchToFileW<P0, P1>(patchfilename: P0, oldfilename: P1, applyoptionflags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mspatcha.dll" "system" fn TestApplyPatchToFileW(patchfilename : windows_core::PCWSTR, oldfilename : windows_core::PCWSTR, applyoptionflags : u32) -> windows_core::BOOL);
    unsafe { TestApplyPatchToFileW(patchfilename.param().abi(), oldfilename.param().abi(), applyoptionflags) }
}
pub const APPLY_OPTION_FAIL_IF_CLOSE: u32 = 2;
pub const APPLY_OPTION_FAIL_IF_EXACT: u32 = 1;
pub const APPLY_OPTION_TEST_ONLY: u32 = 4;
pub const APPLY_OPTION_VALID_FLAGS: u32 = 7;
pub const ERROR_PATCH_BIGGER_THAN_COMPRESSED: u32 = 3222155525;
pub const ERROR_PATCH_CORRUPT: u32 = 3222159618;
pub const ERROR_PATCH_DECODE_FAILURE: u32 = 3222159617;
pub const ERROR_PATCH_ENCODE_FAILURE: u32 = 3222155521;
pub const ERROR_PATCH_IMAGEHLP_FAILURE: u32 = 3222155526;
pub const ERROR_PATCH_INVALID_OPTIONS: u32 = 3222155522;
pub const ERROR_PATCH_NEWER_FORMAT: u32 = 3222159619;
pub const ERROR_PATCH_NOT_AVAILABLE: u32 = 3222159622;
pub const ERROR_PATCH_NOT_NECESSARY: u32 = 3222159621;
pub const ERROR_PATCH_RETAIN_RANGES_DIFFER: u32 = 3222155524;
pub const ERROR_PATCH_SAME_FILE: u32 = 3222155523;
pub const ERROR_PATCH_WRONG_FILE: u32 = 3222159620;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PATCH_IGNORE_RANGE {
    pub OffsetInOldFile: u32,
    pub LengthInBytes: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PATCH_INTERLEAVE_MAP {
    pub CountRanges: u32,
    pub Range: [PATCH_INTERLEAVE_MAP_0; 1],
}
impl Default for PATCH_INTERLEAVE_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PATCH_INTERLEAVE_MAP_0 {
    pub OldOffset: u32,
    pub OldLength: u32,
    pub NewLength: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct PATCH_OLD_FILE_INFO {
    pub SizeOfThisStruct: u32,
    pub Anonymous: PATCH_OLD_FILE_INFO_0,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: PPATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: PPATCH_RETAIN_RANGE,
}
#[cfg(feature = "winnt")]
impl Default for PATCH_OLD_FILE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union PATCH_OLD_FILE_INFO_0 {
    pub OldFileNameA: windows_core::PCSTR,
    pub OldFileNameW: windows_core::PCWSTR,
    pub OldFileHandle: super::winnt::HANDLE,
}
#[cfg(feature = "winnt")]
impl Default for PATCH_OLD_FILE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PATCH_OLD_FILE_INFO_A {
    pub SizeOfThisStruct: u32,
    pub OldFileName: windows_core::PCSTR,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: PPATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: PPATCH_RETAIN_RANGE,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PATCH_OLD_FILE_INFO_H {
    pub SizeOfThisStruct: u32,
    pub OldFileHandle: super::winnt::HANDLE,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: PPATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: PPATCH_RETAIN_RANGE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PATCH_OLD_FILE_INFO_W {
    pub SizeOfThisStruct: u32,
    pub OldFileName: windows_core::PCWSTR,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: PPATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: PPATCH_RETAIN_RANGE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PATCH_OPTION_DATA {
    pub SizeOfThisStruct: u32,
    pub SymbolOptionFlags: u32,
    pub NewFileSymbolPath: windows_core::PCSTR,
    pub OldFileSymbolPathArray: *mut windows_core::PCSTR,
    pub ExtendedOptionFlags: u32,
    pub SymLoadCallback: PPATCH_SYMLOAD_CALLBACK,
    pub SymLoadContext: *mut core::ffi::c_void,
    pub InterleaveMapArray: *mut PPATCH_INTERLEAVE_MAP,
    pub MaxLzxWindowSize: u32,
}
impl Default for PATCH_OPTION_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PATCH_OPTION_FAIL_IF_BIGGER: u32 = 1048576;
pub const PATCH_OPTION_FAIL_IF_SAME_FILE: u32 = 524288;
pub const PATCH_OPTION_INTERLEAVE_FILES: u32 = 1073741824;
pub const PATCH_OPTION_NO_BINDFIX: u32 = 65536;
pub const PATCH_OPTION_NO_CHECKSUM: u32 = 2097152;
pub const PATCH_OPTION_NO_LOCKFIX: u32 = 131072;
pub const PATCH_OPTION_NO_REBASE: u32 = 262144;
pub const PATCH_OPTION_NO_RESTIMEFIX: u32 = 4194304;
pub const PATCH_OPTION_NO_TIMESTAMP: u32 = 8388608;
pub const PATCH_OPTION_RESERVED1: u32 = 2147483648;
pub const PATCH_OPTION_SIGNATURE_MD5: u32 = 16777216;
pub const PATCH_OPTION_USE_BEST: u32 = 0;
pub const PATCH_OPTION_USE_LZX_A: u32 = 1;
pub const PATCH_OPTION_USE_LZX_B: u32 = 2;
pub const PATCH_OPTION_USE_LZX_BEST: u32 = 3;
pub const PATCH_OPTION_USE_LZX_LARGE: u32 = 4;
pub const PATCH_OPTION_VALID_FLAGS: u32 = 3237937159;
pub type PATCH_PROGRESS_CALLBACK = Option<unsafe extern "system" fn(callbackcontext: *mut core::ffi::c_void, currentposition: u32, maximumposition: u32) -> windows_core::BOOL>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PATCH_RETAIN_RANGE {
    pub OffsetInOldFile: u32,
    pub LengthInBytes: u32,
    pub OffsetInNewFile: u32,
}
pub const PATCH_SYMBOL_NO_FAILURES: u32 = 2;
pub const PATCH_SYMBOL_NO_IMAGEHLP: u32 = 1;
pub const PATCH_SYMBOL_RESERVED1: u32 = 2147483648;
pub const PATCH_SYMBOL_UNDECORATED_TOO: u32 = 4;
pub type PATCH_SYMLOAD_CALLBACK = Option<unsafe extern "system" fn(whichfile: u32, symbolfilename: windows_core::PCSTR, symtype: u32, symbolfilechecksum: u32, symbolfiletimedate: u32, imagefilechecksum: u32, imagefiletimedate: u32, callbackcontext: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub const PATCH_TRANSFORM_PE_IRELOC_2: u32 = 512;
pub const PATCH_TRANSFORM_PE_RESOURCE_2: u32 = 256;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPATCH_IGNORE_RANGE(pub *mut PATCH_IGNORE_RANGE);
impl PPATCH_IGNORE_RANGE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPATCH_IGNORE_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPATCH_INTERLEAVE_MAP(pub *mut PATCH_INTERLEAVE_MAP);
impl PPATCH_INTERLEAVE_MAP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPATCH_INTERLEAVE_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPATCH_OLD_FILE_INFO(pub *mut PATCH_OLD_FILE_INFO);
#[cfg(feature = "winnt")]
impl PPATCH_OLD_FILE_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PPATCH_OLD_FILE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPATCH_OLD_FILE_INFO_A(pub *mut PATCH_OLD_FILE_INFO_A);
impl PPATCH_OLD_FILE_INFO_A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPATCH_OLD_FILE_INFO_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPATCH_OLD_FILE_INFO_H(pub *mut PATCH_OLD_FILE_INFO_H);
#[cfg(feature = "winnt")]
impl PPATCH_OLD_FILE_INFO_H {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PPATCH_OLD_FILE_INFO_H {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPATCH_OLD_FILE_INFO_W(pub *mut PATCH_OLD_FILE_INFO_W);
impl PPATCH_OLD_FILE_INFO_W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPATCH_OLD_FILE_INFO_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPATCH_OPTION_DATA(pub *mut PATCH_OPTION_DATA);
impl PPATCH_OPTION_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPATCH_OPTION_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PPATCH_PROGRESS_CALLBACK = Option<unsafe extern "system" fn(callbackcontext: *mut core::ffi::c_void, currentposition: u32, maximumposition: u32) -> windows_core::BOOL>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPATCH_RETAIN_RANGE(pub *mut PATCH_RETAIN_RANGE);
impl PPATCH_RETAIN_RANGE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPATCH_RETAIN_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PPATCH_SYMLOAD_CALLBACK = Option<unsafe extern "system" fn(whichfile: u32, symbolfilename: windows_core::PCSTR, symtype: u32, symbolfilechecksum: u32, symbolfiletimedate: u32, imagefilechecksum: u32, imagefiletimedate: u32, callbackcontext: *mut core::ffi::c_void) -> windows_core::BOOL>;
