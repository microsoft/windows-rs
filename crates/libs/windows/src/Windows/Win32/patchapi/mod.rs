#[cfg(feature = "shared")]
#[inline]
pub unsafe fn ApplyPatchToFileA<P0, P1, P2>(patchfilename: P0, oldfilename: P1, newfilename: P2, applyoptionflags: super::ULONG) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mspatcha.dll" "system" fn ApplyPatchToFileA(patchfilename : windows_core::PCSTR, oldfilename : windows_core::PCSTR, newfilename : windows_core::PCSTR, applyoptionflags : super::ULONG) -> windows_core::BOOL);
    unsafe { ApplyPatchToFileA(patchfilename.param().abi(), oldfilename.param().abi(), newfilename.param().abi(), applyoptionflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ApplyPatchToFileByBuffers(patchfilemapped: &[u8], oldfilemapped: Option<&[u8]>, newfilebuffer: *mut super::PBYTE, newfilebuffersize: super::ULONG, newfileactualsize: Option<*mut super::ULONG>, newfiletime: Option<*mut super::FILETIME>, applyoptionflags: super::ULONG, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: Option<super::PVOID>) -> windows_core::BOOL {
    windows_core::link!("mspatcha.dll" "system" fn ApplyPatchToFileByBuffers(patchfilemapped : super::PBYTE, patchfilesize : super::ULONG, oldfilemapped : super::PBYTE, oldfilesize : super::ULONG, newfilebuffer : *mut super::PBYTE, newfilebuffersize : super::ULONG, newfileactualsize : *mut super::ULONG, newfiletime : *mut super::FILETIME, applyoptionflags : super::ULONG, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : super::PVOID) -> windows_core::BOOL);
    unsafe { ApplyPatchToFileByBuffers(core::mem::transmute(patchfilemapped.as_ptr()), patchfilemapped.len().try_into().unwrap(), core::mem::transmute(oldfilemapped.map_or(core::ptr::null(), |slice| slice.as_ptr())), oldfilemapped.map_or(0, |slice| slice.len().try_into().unwrap()), newfilebuffer as _, newfilebuffersize, newfileactualsize.unwrap_or(core::mem::zeroed()) as _, newfiletime.unwrap_or(core::mem::zeroed()) as _, applyoptionflags, progresscallback, callbackcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ApplyPatchToFileByHandles(patchfilehandle: super::HANDLE, oldfilehandle: Option<super::HANDLE>, newfilehandle: super::HANDLE, applyoptionflags: super::ULONG) -> windows_core::BOOL {
    windows_core::link!("mspatcha.dll" "system" fn ApplyPatchToFileByHandles(patchfilehandle : super::HANDLE, oldfilehandle : super::HANDLE, newfilehandle : super::HANDLE, applyoptionflags : super::ULONG) -> windows_core::BOOL);
    unsafe { ApplyPatchToFileByHandles(patchfilehandle, oldfilehandle.unwrap_or(core::mem::zeroed()) as _, newfilehandle, applyoptionflags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ApplyPatchToFileByHandlesEx(patchfilehandle: super::HANDLE, oldfilehandle: Option<super::HANDLE>, newfilehandle: super::HANDLE, applyoptionflags: super::ULONG, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: Option<super::PVOID>) -> windows_core::BOOL {
    windows_core::link!("mspatcha.dll" "system" fn ApplyPatchToFileByHandlesEx(patchfilehandle : super::HANDLE, oldfilehandle : super::HANDLE, newfilehandle : super::HANDLE, applyoptionflags : super::ULONG, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : super::PVOID) -> windows_core::BOOL);
    unsafe { ApplyPatchToFileByHandlesEx(patchfilehandle, oldfilehandle.unwrap_or(core::mem::zeroed()) as _, newfilehandle, applyoptionflags, progresscallback, callbackcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ApplyPatchToFileExA<P0, P1, P2>(patchfilename: P0, oldfilename: P1, newfilename: P2, applyoptionflags: super::ULONG, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: Option<super::PVOID>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mspatcha.dll" "system" fn ApplyPatchToFileExA(patchfilename : windows_core::PCSTR, oldfilename : windows_core::PCSTR, newfilename : windows_core::PCSTR, applyoptionflags : super::ULONG, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : super::PVOID) -> windows_core::BOOL);
    unsafe { ApplyPatchToFileExA(patchfilename.param().abi(), oldfilename.param().abi(), newfilename.param().abi(), applyoptionflags, progresscallback, callbackcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ApplyPatchToFileExW<P0, P1, P2>(patchfilename: P0, oldfilename: P1, newfilename: P2, applyoptionflags: super::ULONG, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: Option<super::PVOID>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mspatcha.dll" "system" fn ApplyPatchToFileExW(patchfilename : windows_core::PCWSTR, oldfilename : windows_core::PCWSTR, newfilename : windows_core::PCWSTR, applyoptionflags : super::ULONG, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : super::PVOID) -> windows_core::BOOL);
    unsafe { ApplyPatchToFileExW(patchfilename.param().abi(), oldfilename.param().abi(), newfilename.param().abi(), applyoptionflags, progresscallback, callbackcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn ApplyPatchToFileW<P0, P1, P2>(patchfilename: P0, oldfilename: P1, newfilename: P2, applyoptionflags: super::ULONG) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mspatcha.dll" "system" fn ApplyPatchToFileW(patchfilename : windows_core::PCWSTR, oldfilename : windows_core::PCWSTR, newfilename : windows_core::PCWSTR, applyoptionflags : super::ULONG) -> windows_core::BOOL);
    unsafe { ApplyPatchToFileW(patchfilename.param().abi(), oldfilename.param().abi(), newfilename.param().abi(), applyoptionflags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreatePatchFileA<P0, P1, P2>(oldfilename: P0, newfilename: P1, patchfilename: P2, optionflags: super::ULONG, optiondata: Option<PPATCH_OPTION_DATA>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mspatchc.dll" "system" fn CreatePatchFileA(oldfilename : windows_core::PCSTR, newfilename : windows_core::PCSTR, patchfilename : windows_core::PCSTR, optionflags : super::ULONG, optiondata : PPATCH_OPTION_DATA) -> windows_core::BOOL);
    unsafe { CreatePatchFileA(oldfilename.param().abi(), newfilename.param().abi(), patchfilename.param().abi(), optionflags, optiondata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreatePatchFileByHandles(oldfilehandle: Option<super::HANDLE>, newfilehandle: super::HANDLE, patchfilehandle: super::HANDLE, optionflags: super::ULONG, optiondata: Option<PPATCH_OPTION_DATA>) -> windows_core::BOOL {
    windows_core::link!("mspatchc.dll" "system" fn CreatePatchFileByHandles(oldfilehandle : super::HANDLE, newfilehandle : super::HANDLE, patchfilehandle : super::HANDLE, optionflags : super::ULONG, optiondata : PPATCH_OPTION_DATA) -> windows_core::BOOL);
    unsafe { CreatePatchFileByHandles(oldfilehandle.unwrap_or(core::mem::zeroed()) as _, newfilehandle, patchfilehandle, optionflags, optiondata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreatePatchFileByHandlesEx(oldfileinfoarray: &[PATCH_OLD_FILE_INFO_H], newfilehandle: super::HANDLE, patchfilehandle: super::HANDLE, optionflags: super::ULONG, optiondata: Option<PPATCH_OPTION_DATA>, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: Option<super::PVOID>) -> windows_core::BOOL {
    windows_core::link!("mspatchc.dll" "system" fn CreatePatchFileByHandlesEx(oldfilecount : super::ULONG, oldfileinfoarray : PPATCH_OLD_FILE_INFO_H, newfilehandle : super::HANDLE, patchfilehandle : super::HANDLE, optionflags : super::ULONG, optiondata : PPATCH_OPTION_DATA, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : super::PVOID) -> windows_core::BOOL);
    unsafe { CreatePatchFileByHandlesEx(oldfileinfoarray.len().try_into().unwrap(), core::mem::transmute(oldfileinfoarray.as_ptr()), newfilehandle, patchfilehandle, optionflags, optiondata.unwrap_or(core::mem::zeroed()) as _, progresscallback, callbackcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreatePatchFileExA<P2, P3>(oldfileinfoarray: &[PATCH_OLD_FILE_INFO_A], newfilename: P2, patchfilename: P3, optionflags: super::ULONG, optiondata: Option<PPATCH_OPTION_DATA>, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: Option<super::PVOID>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mspatchc.dll" "system" fn CreatePatchFileExA(oldfilecount : super::ULONG, oldfileinfoarray : PPATCH_OLD_FILE_INFO_A, newfilename : windows_core::PCSTR, patchfilename : windows_core::PCSTR, optionflags : super::ULONG, optiondata : PPATCH_OPTION_DATA, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : super::PVOID) -> windows_core::BOOL);
    unsafe { CreatePatchFileExA(oldfileinfoarray.len().try_into().unwrap(), core::mem::transmute(oldfileinfoarray.as_ptr()), newfilename.param().abi(), patchfilename.param().abi(), optionflags, optiondata.unwrap_or(core::mem::zeroed()) as _, progresscallback, callbackcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreatePatchFileExW<P2, P3>(oldfileinfoarray: &[PATCH_OLD_FILE_INFO_W], newfilename: P2, patchfilename: P3, optionflags: super::ULONG, optiondata: Option<PPATCH_OPTION_DATA>, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: Option<super::PVOID>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mspatchc.dll" "system" fn CreatePatchFileExW(oldfilecount : super::ULONG, oldfileinfoarray : PPATCH_OLD_FILE_INFO_W, newfilename : windows_core::PCWSTR, patchfilename : windows_core::PCWSTR, optionflags : super::ULONG, optiondata : PPATCH_OPTION_DATA, progresscallback : PPATCH_PROGRESS_CALLBACK, callbackcontext : super::PVOID) -> windows_core::BOOL);
    unsafe { CreatePatchFileExW(oldfileinfoarray.len().try_into().unwrap(), core::mem::transmute(oldfileinfoarray.as_ptr()), newfilename.param().abi(), patchfilename.param().abi(), optionflags, optiondata.unwrap_or(core::mem::zeroed()) as _, progresscallback, callbackcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreatePatchFileW<P0, P1, P2>(oldfilename: P0, newfilename: P1, patchfilename: P2, optionflags: super::ULONG, optiondata: Option<PPATCH_OPTION_DATA>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mspatchc.dll" "system" fn CreatePatchFileW(oldfilename : windows_core::PCWSTR, newfilename : windows_core::PCWSTR, patchfilename : windows_core::PCWSTR, optionflags : super::ULONG, optiondata : PPATCH_OPTION_DATA) -> windows_core::BOOL);
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
pub unsafe fn ExtractPatchHeaderToFileByHandles(patchfilehandle: super::HANDLE, patchheaderfilehandle: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("mspatchc.dll" "system" fn ExtractPatchHeaderToFileByHandles(patchfilehandle : super::HANDLE, patchheaderfilehandle : super::HANDLE) -> windows_core::BOOL);
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
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetFilePatchSignatureA<P0>(filename: P0, optionflags: super::ULONG, optiondata: Option<super::PVOID>, ignorerangearray: Option<&[PATCH_IGNORE_RANGE]>, retainrangearray: Option<&[PATCH_RETAIN_RANGE]>, signaturebuffersize: super::ULONG, signaturebuffer: windows_core::PSTR) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mspatchc.dll" "system" fn GetFilePatchSignatureA(filename : windows_core::PCSTR, optionflags : super::ULONG, optiondata : super::PVOID, ignorerangecount : super::ULONG, ignorerangearray : PPATCH_IGNORE_RANGE, retainrangecount : super::ULONG, retainrangearray : PPATCH_RETAIN_RANGE, signaturebuffersize : super::ULONG, signaturebuffer : windows_core::PSTR) -> windows_core::BOOL);
    unsafe { GetFilePatchSignatureA(filename.param().abi(), optionflags, optiondata.unwrap_or(core::mem::zeroed()) as _, ignorerangearray.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ignorerangearray.map_or(core::ptr::null(), |slice| slice.as_ptr())), retainrangearray.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(retainrangearray.map_or(core::ptr::null(), |slice| slice.as_ptr())), signaturebuffersize, signaturebuffer) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetFilePatchSignatureByBuffer(filebufferwritable: &mut [u8], optionflags: super::ULONG, optiondata: Option<super::PVOID>, ignorerangearray: Option<&[PATCH_IGNORE_RANGE]>, retainrangearray: Option<&[PATCH_RETAIN_RANGE]>, signaturebuffersize: super::ULONG, signaturebuffer: windows_core::PSTR) -> windows_core::BOOL {
    windows_core::link!("mspatchc.dll" "system" fn GetFilePatchSignatureByBuffer(filebufferwritable : super::PBYTE, filesize : super::ULONG, optionflags : super::ULONG, optiondata : super::PVOID, ignorerangecount : super::ULONG, ignorerangearray : PPATCH_IGNORE_RANGE, retainrangecount : super::ULONG, retainrangearray : PPATCH_RETAIN_RANGE, signaturebuffersize : super::ULONG, signaturebuffer : windows_core::PSTR) -> windows_core::BOOL);
    unsafe {
        GetFilePatchSignatureByBuffer(
            core::mem::transmute(filebufferwritable.as_mut_ptr()),
            filebufferwritable.len().try_into().unwrap(),
            optionflags,
            optiondata.unwrap_or(core::mem::zeroed()) as _,
            ignorerangearray.map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(ignorerangearray.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            retainrangearray.map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(retainrangearray.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            signaturebuffersize,
            signaturebuffer,
        )
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetFilePatchSignatureByHandle(filehandle: super::HANDLE, optionflags: super::ULONG, optiondata: Option<super::PVOID>, ignorerangearray: Option<&[PATCH_IGNORE_RANGE]>, retainrangearray: Option<&[PATCH_RETAIN_RANGE]>, signaturebuffersize: super::ULONG, signaturebuffer: windows_core::PSTR) -> windows_core::BOOL {
    windows_core::link!("mspatchc.dll" "system" fn GetFilePatchSignatureByHandle(filehandle : super::HANDLE, optionflags : super::ULONG, optiondata : super::PVOID, ignorerangecount : super::ULONG, ignorerangearray : PPATCH_IGNORE_RANGE, retainrangecount : super::ULONG, retainrangearray : PPATCH_RETAIN_RANGE, signaturebuffersize : super::ULONG, signaturebuffer : windows_core::PSTR) -> windows_core::BOOL);
    unsafe { GetFilePatchSignatureByHandle(filehandle, optionflags, optiondata.unwrap_or(core::mem::zeroed()) as _, ignorerangearray.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ignorerangearray.map_or(core::ptr::null(), |slice| slice.as_ptr())), retainrangearray.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(retainrangearray.map_or(core::ptr::null(), |slice| slice.as_ptr())), signaturebuffersize, signaturebuffer) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetFilePatchSignatureW<P0>(filename: P0, optionflags: super::ULONG, optiondata: Option<super::PVOID>, ignorerangearray: Option<&[PATCH_IGNORE_RANGE]>, retainrangearray: Option<&[PATCH_RETAIN_RANGE]>, signaturebuffersize: super::ULONG, signaturebuffer: windows_core::PWSTR) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mspatchc.dll" "system" fn GetFilePatchSignatureW(filename : windows_core::PCWSTR, optionflags : super::ULONG, optiondata : super::PVOID, ignorerangecount : super::ULONG, ignorerangearray : PPATCH_IGNORE_RANGE, retainrangecount : super::ULONG, retainrangearray : PPATCH_RETAIN_RANGE, signaturebuffersize : super::ULONG, signaturebuffer : windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { GetFilePatchSignatureW(filename.param().abi(), optionflags, optiondata.unwrap_or(core::mem::zeroed()) as _, ignorerangearray.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ignorerangearray.map_or(core::ptr::null(), |slice| slice.as_ptr())), retainrangearray.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(retainrangearray.map_or(core::ptr::null(), |slice| slice.as_ptr())), signaturebuffersize, signaturebuffer) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn NormalizeFileForPatchSignature(filebuffer: super::PVOID, filesize: super::ULONG, optionflags: super::ULONG, optiondata: Option<*const PATCH_OPTION_DATA>, newfilecoffbase: super::ULONG, newfilecofftime: super::ULONG, ignorerangearray: Option<&[PATCH_IGNORE_RANGE]>, retainrangearray: Option<&[PATCH_RETAIN_RANGE]>) -> super::INT {
    windows_core::link!("mspatchc.dll" "system" fn NormalizeFileForPatchSignature(filebuffer : super::PVOID, filesize : super::ULONG, optionflags : super::ULONG, optiondata : *const PATCH_OPTION_DATA, newfilecoffbase : super::ULONG, newfilecofftime : super::ULONG, ignorerangecount : super::ULONG, ignorerangearray : PPATCH_IGNORE_RANGE, retainrangecount : super::ULONG, retainrangearray : PPATCH_RETAIN_RANGE) -> super::INT);
    unsafe { NormalizeFileForPatchSignature(filebuffer as _, filesize, optionflags, optiondata.unwrap_or(core::mem::zeroed()) as _, newfilecoffbase, newfilecofftime, ignorerangearray.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ignorerangearray.map_or(core::ptr::null(), |slice| slice.as_ptr())), retainrangearray.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(retainrangearray.map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn TestApplyPatchToFileA<P0, P1>(patchfilename: P0, oldfilename: P1, applyoptionflags: super::ULONG) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mspatcha.dll" "system" fn TestApplyPatchToFileA(patchfilename : windows_core::PCSTR, oldfilename : windows_core::PCSTR, applyoptionflags : super::ULONG) -> windows_core::BOOL);
    unsafe { TestApplyPatchToFileA(patchfilename.param().abi(), oldfilename.param().abi(), applyoptionflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn TestApplyPatchToFileByBuffers(patchfilebuffer: &[u8], oldfilebuffer: Option<&[u8]>, newfilesize: Option<*mut super::ULONG>, applyoptionflags: super::ULONG) -> windows_core::BOOL {
    windows_core::link!("mspatcha.dll" "system" fn TestApplyPatchToFileByBuffers(patchfilebuffer : super::PBYTE, patchfilesize : super::ULONG, oldfilebuffer : super::PBYTE, oldfilesize : super::ULONG, newfilesize : *mut super::ULONG, applyoptionflags : super::ULONG) -> windows_core::BOOL);
    unsafe { TestApplyPatchToFileByBuffers(core::mem::transmute(patchfilebuffer.as_ptr()), patchfilebuffer.len().try_into().unwrap(), core::mem::transmute(oldfilebuffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), oldfilebuffer.map_or(0, |slice| slice.len().try_into().unwrap()), newfilesize.unwrap_or(core::mem::zeroed()) as _, applyoptionflags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TestApplyPatchToFileByHandles(patchfilehandle: super::HANDLE, oldfilehandle: super::HANDLE, applyoptionflags: super::ULONG) -> windows_core::BOOL {
    windows_core::link!("mspatcha.dll" "system" fn TestApplyPatchToFileByHandles(patchfilehandle : super::HANDLE, oldfilehandle : super::HANDLE, applyoptionflags : super::ULONG) -> windows_core::BOOL);
    unsafe { TestApplyPatchToFileByHandles(patchfilehandle, oldfilehandle, applyoptionflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn TestApplyPatchToFileW<P0, P1>(patchfilename: P0, oldfilename: P1, applyoptionflags: super::ULONG) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mspatcha.dll" "system" fn TestApplyPatchToFileW(patchfilename : windows_core::PCWSTR, oldfilename : windows_core::PCWSTR, applyoptionflags : super::ULONG) -> windows_core::BOOL);
    unsafe { TestApplyPatchToFileW(patchfilename.param().abi(), oldfilename.param().abi(), applyoptionflags) }
}
pub const APPLY_OPTION_FAIL_IF_CLOSE: i32 = 2;
pub const APPLY_OPTION_FAIL_IF_EXACT: i32 = 1;
pub const APPLY_OPTION_TEST_ONLY: i32 = 4;
pub const APPLY_OPTION_VALID_FLAGS: i32 = 7;
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PATCH_IGNORE_RANGE {
    pub OffsetInOldFile: super::ULONG,
    pub LengthInBytes: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PATCH_INTERLEAVE_MAP {
    pub CountRanges: super::ULONG,
    pub Range: [PATCH_INTERLEAVE_MAP_0; 1],
}
#[cfg(feature = "shared")]
impl Default for PATCH_INTERLEAVE_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PATCH_INTERLEAVE_MAP_0 {
    pub OldOffset: super::ULONG,
    pub OldLength: super::ULONG,
    pub NewLength: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct PATCH_OLD_FILE_INFO {
    pub SizeOfThisStruct: super::ULONG,
    pub Anonymous: PATCH_OLD_FILE_INFO_0,
    pub IgnoreRangeCount: super::ULONG,
    pub IgnoreRangeArray: PPATCH_IGNORE_RANGE,
    pub RetainRangeCount: super::ULONG,
    pub RetainRangeArray: PPATCH_RETAIN_RANGE,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PATCH_OLD_FILE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union PATCH_OLD_FILE_INFO_0 {
    pub OldFileNameA: windows_core::PCSTR,
    pub OldFileNameW: windows_core::PCWSTR,
    pub OldFileHandle: super::HANDLE,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PATCH_OLD_FILE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PATCH_OLD_FILE_INFO_A {
    pub SizeOfThisStruct: super::ULONG,
    pub OldFileName: windows_core::PCSTR,
    pub IgnoreRangeCount: super::ULONG,
    pub IgnoreRangeArray: PPATCH_IGNORE_RANGE,
    pub RetainRangeCount: super::ULONG,
    pub RetainRangeArray: PPATCH_RETAIN_RANGE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PATCH_OLD_FILE_INFO_H {
    pub SizeOfThisStruct: super::ULONG,
    pub OldFileHandle: super::HANDLE,
    pub IgnoreRangeCount: super::ULONG,
    pub IgnoreRangeArray: PPATCH_IGNORE_RANGE,
    pub RetainRangeCount: super::ULONG,
    pub RetainRangeArray: PPATCH_RETAIN_RANGE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PATCH_OLD_FILE_INFO_W {
    pub SizeOfThisStruct: super::ULONG,
    pub OldFileName: windows_core::PCWSTR,
    pub IgnoreRangeCount: super::ULONG,
    pub IgnoreRangeArray: PPATCH_IGNORE_RANGE,
    pub RetainRangeCount: super::ULONG,
    pub RetainRangeArray: PPATCH_RETAIN_RANGE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct PATCH_OPTION_DATA {
    pub SizeOfThisStruct: super::ULONG,
    pub SymbolOptionFlags: super::ULONG,
    pub NewFileSymbolPath: windows_core::PCSTR,
    pub OldFileSymbolPathArray: *mut windows_core::PCSTR,
    pub ExtendedOptionFlags: super::ULONG,
    pub SymLoadCallback: PPATCH_SYMLOAD_CALLBACK,
    pub SymLoadContext: super::PVOID,
    pub InterleaveMapArray: *mut PPATCH_INTERLEAVE_MAP,
    pub MaxLzxWindowSize: super::ULONG,
}
pub const PATCH_OPTION_FAIL_IF_BIGGER: i32 = 1048576;
pub const PATCH_OPTION_FAIL_IF_SAME_FILE: i32 = 524288;
pub const PATCH_OPTION_INTERLEAVE_FILES: i32 = 1073741824;
pub const PATCH_OPTION_NO_BINDFIX: i32 = 65536;
pub const PATCH_OPTION_NO_CHECKSUM: i32 = 2097152;
pub const PATCH_OPTION_NO_LOCKFIX: i32 = 131072;
pub const PATCH_OPTION_NO_REBASE: i32 = 262144;
pub const PATCH_OPTION_NO_RESTIMEFIX: i32 = 4194304;
pub const PATCH_OPTION_NO_TIMESTAMP: i32 = 8388608;
pub const PATCH_OPTION_RESERVED1: u32 = 2147483648;
pub const PATCH_OPTION_SIGNATURE_MD5: i32 = 16777216;
pub const PATCH_OPTION_USE_BEST: i32 = 0;
pub const PATCH_OPTION_USE_LZX_A: i32 = 1;
pub const PATCH_OPTION_USE_LZX_B: i32 = 2;
pub const PATCH_OPTION_USE_LZX_BEST: i32 = 3;
pub const PATCH_OPTION_USE_LZX_LARGE: i32 = 4;
pub const PATCH_OPTION_VALID_FLAGS: u32 = 3237937159;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PATCH_PROGRESS_CALLBACK = Option<unsafe extern "system" fn(callbackcontext: super::PVOID, currentposition: super::ULONG, maximumposition: super::ULONG) -> windows_core::BOOL>;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PATCH_RETAIN_RANGE {
    pub OffsetInOldFile: super::ULONG,
    pub LengthInBytes: super::ULONG,
    pub OffsetInNewFile: super::ULONG,
}
pub const PATCH_SYMBOL_NO_FAILURES: i32 = 2;
pub const PATCH_SYMBOL_NO_IMAGEHLP: i32 = 1;
pub const PATCH_SYMBOL_RESERVED1: u32 = 2147483648;
pub const PATCH_SYMBOL_UNDECORATED_TOO: i32 = 4;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PATCH_SYMLOAD_CALLBACK = Option<unsafe extern "system" fn(whichfile: super::ULONG, symbolfilename: windows_core::PCSTR, symtype: super::ULONG, symbolfilechecksum: super::ULONG, symbolfiletimedate: super::ULONG, imagefilechecksum: super::ULONG, imagefiletimedate: super::ULONG, callbackcontext: super::PVOID) -> windows_core::BOOL>;
pub const PATCH_TRANSFORM_PE_IRELOC_2: i32 = 512;
pub const PATCH_TRANSFORM_PE_RESOURCE_2: i32 = 256;
#[cfg(feature = "shared")]
pub type PPATCH_IGNORE_RANGE = *mut PATCH_IGNORE_RANGE;
#[cfg(feature = "shared")]
pub type PPATCH_INTERLEAVE_MAP = *mut PATCH_INTERLEAVE_MAP;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPATCH_OLD_FILE_INFO = *mut PATCH_OLD_FILE_INFO;
#[cfg(feature = "shared")]
pub type PPATCH_OLD_FILE_INFO_A = *mut PATCH_OLD_FILE_INFO_A;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPATCH_OLD_FILE_INFO_H = *mut PATCH_OLD_FILE_INFO_H;
#[cfg(feature = "shared")]
pub type PPATCH_OLD_FILE_INFO_W = *mut PATCH_OLD_FILE_INFO_W;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPATCH_OPTION_DATA = *mut PATCH_OPTION_DATA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPATCH_PROGRESS_CALLBACK = Option<unsafe extern "system" fn(callbackcontext: super::PVOID, currentposition: super::ULONG, maximumposition: super::ULONG) -> windows_core::BOOL>;
#[cfg(feature = "shared")]
pub type PPATCH_RETAIN_RANGE = *mut PATCH_RETAIN_RANGE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPATCH_SYMLOAD_CALLBACK = Option<unsafe extern "system" fn(whichfile: super::ULONG, symbolfilename: windows_core::PCSTR, symtype: super::ULONG, symbolfilechecksum: super::ULONG, symbolfiletimedate: super::ULONG, imagefilechecksum: super::ULONG, imagefiletimedate: super::ULONG, callbackcontext: super::PVOID) -> windows_core::BOOL>;
